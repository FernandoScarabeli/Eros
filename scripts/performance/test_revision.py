import io
from contextlib import closing
import json
import math
import os
import signal
import sqlite3
import subprocess
import sys
import time
from pathlib import Path
import tempfile
from types import SimpleNamespace
import unittest
from unittest.mock import patch

from clocks import correlate, clock_valid, startup_sample
from fixture import Fixture, IpcFailure, MAX_FRAME
from metrics import distribution, cpu_percent
from run import render, summary, UiFailure, wait_ui


def clock(offset=1000, bracket=.2):
    return {'offset_ms': offset, 'uncertainty_ms': bracket/2, 'bracket_ms': bracket}


class ClockTests(unittest.TestCase):
    def test_injected_bracketing_precision(self):
        ticks = iter([10.0, 10.0004])
        result = correlate(epoch=lambda: 1000.0, monotonic=lambda: next(ticks))
        self.assertAlmostEqual(result['uncertainty_ms'], .2, places=6)
        self.assertAlmostEqual(result['offset_ms'], 989999.8, places=6)

    def test_forward_and_backward_jumps_rejected(self):
        for offset in (1006, 994):
            self.assertFalse(clock_valid(clock(), clock(offset)))
            result = startup_sample(100, 200, clock(), clock(offset))
            self.assertFalse(result['valid'])
            self.assertIsNone(result['value_ms'])

    def test_tolerance_and_bracket_limits(self):
        self.assertTrue(clock_valid(clock(), clock(1005, 1)))
        self.assertFalse(clock_valid(clock(), clock(1000, 1.001)))
        self.assertFalse(clock_valid(clock(), clock(math.nan)))
        self.assertFalse(clock_valid(clock(), clock(1000, -1)))

    def test_negative_nonfinite_latency_never_aggregated(self):
        for ready in (99, math.nan, math.inf):
            result = startup_sample(100, ready, clock(), clock())
            self.assertIsNone(result['value_ms'])
        self.assertEqual(distribution([None, math.nan, math.inf])['n'], 0)

    def test_summary_trusts_valid_samples_not_stale_cached_values(self):
        record = {'scenario': 'W0', 'startupEmpty': {
            'coldMs': [-1, 100, 200], 'warmMs': [], 'samples': [
                {'mode': 'cold', 'valid': False, 'value_ms': -1},
                {'mode': 'cold', 'valid': False, 'value_ms': 100},
                {'mode': 'cold', 'valid': True, 'value_ms': 200}]}}
        result = summary([record])['W0.startupEmpty.coldMs']
        self.assertEqual(result['n'], 1)
        self.assertEqual(result['p95'], 200)

    def test_bad_epoch_does_not_invalidate_monotonic_cpu(self):
        self.assertFalse(clock_valid(clock(), clock(2000)))
        self.assertEqual(cpu_percent(1, 1.5, 2), 25)

    def test_invalid_epoch_blocks_startup_comparison_only(self):
        environment = {'binary_sha256': 'same', 'source_sha256': {}, 'configuration': {},
                       'cpu_model': 'same', 'kernel': 'same', 'versions': {}}
        def report(value):
            return {'environment': environment, 'status': 'complete', 'workloads': [
                {'scenario': 'W0', 'status': 'complete', 'duration_s': 1, 'samples': [],
                 'cleanup': {'live_survivors': 0}, 'startupEmpty': {}, 'clock': {'comparison_valid': False}}],
                'summary': {'W0.startupEmpty.warmMs': distribution([value]),
                            'W1.1.echoApproxMs': distribution([value])}}
        text = render(report(90), report(100))
        self.assertNotIn('- W0.startupEmpty.warmMs: variação', text)
        self.assertIn('- W1.1.echoApproxMs: variação', text)


class FakeStream:
    def __init__(self, raw): self.raw = raw
    def __enter__(self): return self
    def __exit__(self, *_): pass
    def connect(self, _): pass
    def settimeout(self, _): pass
    def sendall(self, _): pass
    def makefile(self, _): return io.BytesIO(self.raw)


class IpcTests(unittest.TestCase):
    def request(self, raw):
        with patch('fixture.socket.socket', return_value=FakeStream(raw)):
            return Fixture.request(SimpleNamespace(socket=Path('/synthetic.sock')), 'snapshot')

    def test_empty_reply_records_size_limit_and_phase(self):
        with self.assertRaises(IpcFailure) as caught:
            self.request(b'')
        self.assertEqual(caught.exception.details, {'phase': 'snapshot', 'reason': 'empty_reply',
                                                  'received_bytes': 0, 'limit_bytes': MAX_FRAME})

    def test_exact_payload_limit_including_newline_handling(self):
        raw = b'{"ok":true,"data":null}'
        self.assertIsNone(self.request(raw + b' '*(MAX_FRAME-len(raw)) + b'\n'))
        with self.assertRaises(IpcFailure) as caught:
            self.request(raw + b' '*(MAX_FRAME+1-len(raw)) + b'\n')
        self.assertEqual(caught.exception.details['reason'], 'frame_exceeds_limit')

    def test_partial_frame_is_not_parsed_as_a_valid_reply(self):
        with self.assertRaises(IpcFailure) as caught:
            self.request(b'{"ok":true,"data":null}')
        self.assertEqual(caught.exception.details['reason'], 'incomplete_frame')

    def test_service_readiness_uses_small_control_not_snapshot(self):
        with tempfile.TemporaryDirectory(prefix='eros-perf-liveness-test-') as temp:
            root = Path(temp)
            sock = root/'service.sock'
            sock.touch()
            calls = []
            fixture = SimpleNamespace(root=root, socket=sock, binary='/fake',
                spawn=lambda _: SimpleNamespace(poll=lambda: None),
                request=lambda operation, **kw: calls.append((operation, kw)))
            class PermissionProbe(FakeStream):
                def bind(self, _): pass
            with patch('fixture.socket.socket', return_value=PermissionProbe(b'')):
                Fixture.start_service(fixture)
            self.assertEqual(calls, [('set_active_scope', {'scope': 'chats'})])

    def test_ui_failure_preserves_partial_payload_and_diagnostic(self):
        payload = {'status': 'failed', 'phase': 'done', 'failedPhase': 'mount-terminals',
                   'condition': 'mount-terminal-3', 'classification': 'product_failure',
                   'diagnostic': {'expectedProbes': 3, 'observedProbes': 2, 'readyProbes': 2},
                   'measurements': {'echoApproxMs': [25]}}
        fixture = SimpleNamespace(service=SimpleNamespace(pid=1), ui=SimpleNamespace(pid=2),
                                  remember=lambda: None, clock_samples=[])
        class Sampler:
            def sample(self, _): return {}
        with patch('run.read_ui', return_value=payload):
            with self.assertRaises(UiFailure) as caught:
                import time
                wait_ui(fixture, Sampler(), time.monotonic()+1)
        self.assertIs(caught.exception.payload, payload)
        self.assertEqual(caught.exception.classification, 'product_failure')
        self.assertEqual(fixture.last_ui['measurements']['echoApproxMs'], [25])


class FixtureDataTests(unittest.TestCase):
    def test_full_catalog_preserved_and_serialized_size_diagnosed(self):
        fixture = Fixture('/bin/false')
        try:
            (fixture.root/'data').mkdir()
            with closing(sqlite3.connect(fixture.root/'data/eros.sqlite3')) as db, db:
                db.executescript("""
                    CREATE TABLE projects(id TEXT, name TEXT, path TEXT);
                    CREATE TABLE conversations(id TEXT,project_id TEXT,name TEXT,cwd TEXT,
                        kind TEXT,state TEXT,provider_thread_id TEXT,pinned INTEGER DEFAULT 0,
                        chat_status TEXT DEFAULT 'normal',activity_revision INTEGER DEFAULT 0);
                """)
            fixture.seed()
            size = fixture.catalog_size()
            self.assertEqual(size['projects'], 100)
            self.assertEqual(size['chats'], 10000)
            self.assertGreater(size['conversations_json_bytes'], MAX_FRAME)
        finally:
            self.assertEqual(fixture.cleanup()['live_survivors'], 0)

    def test_producer_first_payload_and_sigint_finally_are_observable(self):
        with tempfile.TemporaryDirectory(prefix='eros-perf-producer-revision-test-') as temp:
            script = Path(__file__).with_name('producer.py')
            p = subprocess.Popen([sys.executable, str(script), '--rate', '4096', '--seconds', '60',
                                  '--tag', 'interrupt-test', '--started', '31', '--stopped', '32'],
                                 stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                                 env=dict(os.environ, EROS_DATA_DIR=temp))
            try:
                path = Path(temp)/'producer-interrupt-test.json'
                deadline = time.monotonic()+3
                while not path.exists() and time.monotonic() < deadline:
                    time.sleep(.01)
                first = json.loads(path.read_text())
                self.assertEqual(first['state'], 'running')
                self.assertGreater(first['produced'], 0)
                p.send_signal(signal.SIGINT)
                output, error = p.communicate(timeout=5)
                self.assertEqual(p.returncode, 130, error.decode())
                self.assertIn(b'EROS_MARK_31_END', output)
                self.assertIn(b'EROS_MARK_32_END', output)
                final = json.loads(path.read_text())
                self.assertEqual(final['state'], 'interrupted')
                self.assertLessEqual(final['produced'], 4096 * 60)
            finally:
                if p.poll() is None:
                    p.kill()
                    p.wait()


if __name__ == '__main__':
    unittest.main()
