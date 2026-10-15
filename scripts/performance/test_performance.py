import os
from pathlib import Path
import signal
import subprocess
import sys
import tempfile
import time
import unittest

from fixture import Fixture
from metrics import classify, cpu_percent, distribution, process_stat, resources
from run import derive, render, summary


class CalculationTests(unittest.TestCase):
    def test_nearest_rank_and_empty_are_not_zero(self):
        self.assertEqual(distribution(range(1, 21))['p95'], 19)
        self.assertEqual(distribution(range(1, 11))['p95'], 10)
        self.assertIsNone(distribution([])['p95'])
        self.assertEqual(distribution([1, 9])['median'], 5)

    def test_cpu_is_one_core_and_can_exceed_100(self):
        self.assertEqual(cpu_percent(1, 3, 1), 200)
        self.assertEqual(cpu_percent(2, 2.5, 2), 25)
        self.assertIsNone(cpu_percent(1, 2, 0))

    def test_owned_process_classification(self):
        parents = {11: 10, 12: 11, 21: 20, 22: 21, 99: 1}
        for pid, expected in [(10, 'service'), (12, 'controlled'), (20, 'ui_webkit'),
                              (22, 'ui_webkit'), (30, 'monitor'), (99, None)]:
            self.assertEqual(classify(pid, parents, 10, 20, 30), expected)
        self.assertIsNone(classify(1, {1: 2, 2: 1}, 10, 20, 30))

    def test_growth_and_cpu_keep_missing_application_metrics_unknown(self):
        rows = []
        for at, phase, cpu, pss in [(0, 'idle', 1, 100), (31, 'idle', 2, 100),
                                    (35, 'cycle-before', 0, 100), (70, 'settle', 0, 120)]:
            rows.append({'at_monotonic_s': at, 'phase': phase, 'groups': {
                group: {'cpu_percent': cpu, 'pss_kib': pss, 'fds': 3, 'threads': 2}
                for group in ('service', 'ui_webkit')}})
        result = derive(rows)
        self.assertEqual(result['idle_cpu_percent_one_core_mean'], 4)
        self.assertEqual(result['view_cycles_growth']['pss_kib'], 40)
        for row in rows:
            row['groups']['ui_webkit']['pss_kib'] = None
        self.assertIsNone(derive(rows)['peak_app_pss_kib'])
        self.assertIsNone(derive(rows)['view_cycles_growth']['pss_kib'])

    def test_proc_missing_is_unknown(self):
        self.assertTrue(all(v is None for v in resources(999999999).values()))
        self.assertGreater(process_stat(os.getpid())['start'], 0)

    def test_unavailable_report_does_not_claim_zero_latency(self):
        report = {'status': 'incomplete', 'workloads': [], 'summary': summary([])}
        text = render(report)
        self.assertIn('não medida / não verificado', text)
        self.assertNotIn('p95 0.000', text)


class LifecycleTests(unittest.TestCase):
    def test_detached_descendant_reclaimed_on_failure(self):
        fixture = Fixture('/bin/false')
        root = fixture.root
        marker = root/'child'
        child = fixture.spawn([sys.executable, '-c',
            'import subprocess,time,pathlib,sys; '
            'p=subprocess.Popen([sys.executable,"-c","import time; time.sleep(60)"],start_new_session=True); '
            'pathlib.Path(sys.argv[1]).write_text(str(p.pid)); time.sleep(60)', str(marker)], os.environ.copy())
        try:
            deadline = time.monotonic()+3
            while not marker.exists() and time.monotonic() < deadline:
                time.sleep(.02)
            self.assertTrue(marker.exists())
            descendant = int(marker.read_text())
            fixture.remember()
            self.assertIn(descendant, fixture.identities)
            raise RuntimeError('Controlled failure')
        except RuntimeError:
            result = fixture.cleanup()
        self.assertEqual(result['live_survivors'], 0)
        self.assertIsNotNone(child.poll())
        self.assertFalse(root.exists())

    def test_reparented_child_is_reaped_after_root_already_exited(self):
        fixture = Fixture('/bin/false')
        marker = fixture.root/'adopted'
        child_source = 'import os,pathlib,sys,time; pathlib.Path(sys.argv[1]).write_text(str(os.getpid())); time.sleep(60)'
        root_source = 'import subprocess,sys; subprocess.Popen([sys.executable,"-c",sys.argv[1],sys.argv[2]],start_new_session=True)'
        root = fixture.spawn([sys.executable, '-c', root_source, child_source, str(marker)], os.environ.copy())
        try:
            self.assertEqual(root.wait(timeout=3), 0)
            deadline = time.monotonic()+3
            while not marker.exists() and time.monotonic() < deadline:
                time.sleep(.02)
            self.assertTrue(marker.exists())
            adopted = int(marker.read_text())
            fixture.remember()
            self.assertIn(adopted, fixture.identities)
        finally:
            result = fixture.cleanup()
        self.assertEqual(result['live_survivors'], 0)
        self.assertFalse(Path(f'/proc/{adopted}').exists())

    def test_descendant_created_by_term_handler_is_also_reclaimed(self):
        fixture = Fixture('/bin/false')
        ready = fixture.root/'ready'
        marker = fixture.root/'late-child'
        source = """
import os,pathlib,signal,subprocess,sys,time
def stop(*_):
    p=subprocess.Popen(['/bin/sleep','60'],start_new_session=True)
    pathlib.Path(sys.argv[2]).write_text(str(p.pid))
    os._exit(0)
signal.signal(signal.SIGTERM,stop)
pathlib.Path(sys.argv[1]).write_text('ready')
time.sleep(60)
"""
        fixture.spawn([sys.executable, '-c', source, str(ready), str(marker)], os.environ.copy())
        deadline = time.monotonic()+3
        while not ready.exists() and time.monotonic() < deadline:
            time.sleep(.01)
        self.assertTrue(ready.exists())
        result = fixture.cleanup()
        self.assertEqual(result['live_survivors'], 0)
        self.assertGreaterEqual(result['owned_processes'], 2)

    def test_sigterm_interrupt_runs_cleanup(self):
        with tempfile.TemporaryDirectory(prefix='eros-perf-test-') as temp:
            marker = Path(temp)/'ready'
            source = '''
import json,os,signal,sys,time
from pathlib import Path
from fixture import Fixture
f=Fixture('/bin/false')
signal.signal(signal.SIGTERM, lambda *_: (_ for _ in ()).throw(KeyboardInterrupt()))
try:
    p=f.spawn(['/bin/sleep','60'],os.environ.copy())
    Path(sys.argv[1]).write_text(json.dumps({'pid':p.pid,'root':str(f.root)}))
    time.sleep(60)
except KeyboardInterrupt:
    pass
finally:
    f.cleanup()
'''
            p = subprocess.Popen([sys.executable, '-c', source, str(marker)], cwd=Path(__file__).parent)
            try:
                deadline = time.monotonic()+3
                while not marker.exists() and time.monotonic() < deadline:
                    time.sleep(.02)
                self.assertTrue(marker.exists())
                import json
                owned = json.loads(marker.read_text())
                p.terminate()
                self.assertEqual(p.wait(timeout=5), 0)
                self.assertFalse(Path(owned['root']).exists())
                self.assertFalse(Path(f"/proc/{owned['pid']}").exists())
            finally:
                if p.poll() is None:
                    p.kill()
                    p.wait()

    def test_producer_actual_volume_and_deadline(self):
        with tempfile.TemporaryDirectory(prefix='eros-perf-producer-test-') as temp:
            with tempfile.TemporaryFile() as output:
                result = subprocess.run([sys.executable, str(Path(__file__).with_name('producer.py')),
                    '--rate', '4096', '--seconds', '1'], stdout=output,
                    env=dict(os.environ, EROS_DATA_DIR=temp), timeout=5)
                self.assertEqual(result.returncode, 0)
                self.assertEqual(output.tell(), 4096)
            counts = list(Path(temp).glob('producer-*.count'))
            self.assertEqual(len(counts), 1)
            self.assertEqual(int(counts[0].read_text()), 4096)


if __name__ == '__main__':
    unittest.main()
