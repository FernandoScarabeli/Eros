#!/usr/bin/env python3
"""Explicit opt-in native release performance harness; standard-library only."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import platform
import shutil
import signal
import subprocess
import sys
import time

from fixture import Fixture, EnvironmentUnavailable, IpcFailure
from failures import exercise
from metrics import Sampler, distribution
from clocks import correlate, clock_valid, startup_sample

REPO = Path(__file__).resolve().parents[2]


def command(args):
    try:
        result = subprocess.run(args, cwd=REPO, capture_output=True, text=True, timeout=15)
        return result.stdout.strip() if result.returncode == 0 else None
    except (OSError, subprocess.TimeoutExpired):
        return None


def metadata(binary, args):
    model = next((line.split(':', 1)[1].strip() for line in Path('/proc/cpuinfo').read_text().splitlines()
                  if line.startswith('model name')), None)
    memory = next(line for line in Path('/proc/meminfo').read_text().splitlines() if line.startswith('MemTotal:'))
    changes = command(['git', 'status', '--porcelain'])
    return {'commit': command(['git', 'rev-parse', 'HEAD']), 'changes': changes.splitlines() if changes else [],
            'diff_sha256': hashlib.sha256((command(['git', 'diff', '--no-ext-diff']) or '').encode()).hexdigest(),
            'binary_sha256': hashlib.sha256(binary.read_bytes()).hexdigest() if binary.exists() else None,
            'os': platform.freedesktop_os_release(), 'kernel': platform.release(), 'architecture': platform.machine(),
            'cpu_model': model, 'logical_cpus': os.cpu_count(), 'memory': memory,
            'versions': {name: command(cmd) for name, cmd in {
                'rustc': ['rustc', '--version'], 'cargo': ['cargo', '--version'], 'node': ['node', '--version'],
                'npm': ['npm', '--version'], 'webkitgtk': ['pkg-config', '--modversion', 'webkit2gtk-4.1'],
                'python': ['python3', '--version']}.items()},
            'frontend_packages': {name: json.loads((REPO/'apps/desktop/node_modules'/name/'package.json').read_text())['version']
                                  if (REPO/'apps/desktop/node_modules'/name/'package.json').exists() else None
                                  for name in ('@tauri-apps/cli', '@tauri-apps/api', '@xterm/xterm', 'svelte', 'vite')},
            'build': 'direct release executable, opt-in VITE_EROS_BENCHMARK=1; not installed package',
            'configuration': vars(args) | {'binary': str(args.binary), 'output': str(args.output),
                                            'compare': str(args.compare) if args.compare else None},
            'clock_offset_ms_before': (time.time()-time.monotonic())*1000,
            'source_sha256': {str(p.relative_to(REPO)): hashlib.sha256(p.read_bytes()).hexdigest()
                for p in [*Path(__file__).parent.glob('*.py'), REPO/'apps/desktop/src/lib/benchmark.ts', REPO/'apps/desktop/src/lib/benchmark-probe.ts', REPO/'apps/desktop/src/lib/HomeScreen.svelte', REPO/'apps/desktop/src/lib/TextType.svelte',
                          REPO/'apps/desktop/src/App.svelte', REPO/'apps/desktop/src/lib/TerminalPane.svelte', REPO/'apps/desktop/src/main.ts',
                          REPO/'apps/desktop/src-tauri/src/lib.rs']}}


def read_ui(fixture):
    path = fixture.root/'runtime/benchmark-ui.json'
    try:
        return json.loads(path.read_text())
    except FileNotFoundError:
        return {}


def launch_ui(fixture, scenario, args, terminals=1, idle_mode='terminal'):
    path = fixture.root/'runtime/benchmark-ui.json'
    path.unlink(missing_ok=True)
    config = {'scenario': scenario, 'idleSeconds': args.idle_seconds,
              'pressureSeconds': args.pressure_seconds, 'cycles': args.cycles,
              'terminals': terminals, 'idleMode': idle_mode}
    env = dict(fixture.env, EROS_BENCHMARK='1', EROS_BENCH_CONFIG=json.dumps(config))
    fixture.launch_clock = correlate()
    start = time.time()*1000
    fixture.ui = fixture.spawn([fixture.binary], env)
    return start


class UiFailure(RuntimeError):
    def __init__(self, message, classification, payload):
        self.classification, self.payload = classification, payload
        super().__init__(message)


def wait_ui(fixture, sampler, deadline, ready_only=False):
    while time.monotonic() < deadline:
        ui = read_ui(fixture)
        fixture.last_ui = ui
        fixture.remember()
        sampler.service = fixture.service.pid if fixture.service else None
        sampler.ui = fixture.ui.pid
        row = sampler.sample(ui.get('phase', 'startup'))
        row['clock'] = correlate()
        fixture.clock_samples.append(row['clock'])
        if ui.get('status') == 'failed':
            raise UiFailure(ui.get('error', 'Native UI workload failed'), ui.get('classification', 'driver_failure'), ui)
        if ready_only and 'readyEpochMs' in ui:
            return ui
        if ui.get('phase') == 'done':
            return ui
        if fixture.ui.poll() is not None:
            raise UiFailure(f'Native UI exited ({fixture.ui.returncode}); inspect graphical availability', 'native_exit_unclassified', ui)
        time.sleep(1)  # PSS at most once/second; latency never calculated from this polling interval.
    raise UiFailure(f"Native deadline: phase {fixture.last_ui.get('phase', 'startup')}, condition {fixture.last_ui.get('condition', 'missing UI report')}", 'driver_failure', fixture.last_ui)


def startup(fixture, args, sampler, result):
    for mode, count in (('cold', args.cold_samples), ('warm', args.warm_samples)):
        for _ in range(count):
            before = correlate()
            clock_start = len(fixture.clock_samples)
            if mode == 'cold':
                fixture.stop_child(fixture.service)
                start = time.time()*1000
                fixture.start_service()
            else:
                start = time.time()*1000
            launch_start = launch_ui(fixture, 'startup', args)
            if mode == 'warm':
                start = launch_start
            ui = wait_ui(fixture, sampler, time.monotonic()+30, ready_only=True)
            after = correlate()
            sample = startup_sample(start, ui['readyEpochMs'], before, after)
            if not all(clock_valid(before, c) for c in fixture.clock_samples[clock_start:]):
                sample.update(valid=False, value_ms=None, reason='Clock jump observed during startup')
            if ui.get('uiClock', {}).get('valid') is not True:
                sample.update(valid=False, value_ms=None, reason='UI epoch correlation not validated')
            sample['mode'] = mode
            result['samples'].append(sample)
            if sample['valid']:
                result[mode+'Ms'].append(sample['value_ms'])
            fixture.stop_child(fixture.ui)
    return result


def workload(scenario, binary, args, terminals=1, idle_mode='terminal', label=None):
    fixture = Fixture(binary)
    sampler = Sampler()
    fixture.clock_samples, fixture.last_ui = [], {}
    record = {'scenario': label or scenario, 'workload': scenario, 'terminals': terminals, 'idle_mode': idle_mode, 'status': 'running'}
    shutil.copyfile(Path(__file__).with_name('producer.py'), fixture.root/'producer.py')
    start = time.monotonic()
    try:
        if scenario == 'W4':
            # Corpus is available for future adapters, never claim files/Git UI implemented.
            corpus = fixture.root/'corpus'
            corpus.mkdir()
            for i in range(50000):
                (corpus/f'file-{i:05d}.txt').write_text('synthetic\n')
            for excluded in ('node_modules', 'target', '.git'):
                (corpus/excluded).mkdir()
            record.update(status='unavailable', generated_files=50000,
                          reason='Files/Git views and adapters not implemented (EROS-024/025/026/047)')
            return record
        if scenario == 'chat-switch':
            fixture.env['EROS_BENCH_SHELL_CHATS'] = '1'
        fixture.start_service()
        fixture.request('snapshot')  # Small catalog availability checked separately from liveness.
        if scenario == 'W3':
            for _ in range(5):
                sampler.service = fixture.service.pid
                sampler.sample('catalog-service-base')
                time.sleep(1)
            fixture.stop_child(fixture.service)
            fixture.seed()
            record['catalog'] = fixture.catalog_size()
            fixture.start_service()  # Control RPC remains usable even when snapshot fails.
            sampler.service = fixture.service.pid
            try:
                fixture.request('snapshot')
            except IpcFailure as error:
                record.update(status='product_failure', classification=('catalog_capacity' if record['catalog']['conversations_json_bytes'] > record['catalog']['limit_bytes'] or error.details['reason'] == 'frame_exceeds_limit' else 'catalog_delivery_failure'),
                    error=str(error), ipc=error.details, related_cards=['EROS-005'],
                    liveness_after_snapshot=fixture.request('set_active_scope', scope='chats') is None,
                    unavailable=['native catalog navigation/search/order/chat-switch: snapshot not delivered',
                                 'UI-inclusive equivalent-catalog PSS: large catalog cannot mount'])
                for _ in range(5):
                    sampler.sample('catalog-service-large')
                    time.sleep(1)
                return record
        if scenario == 'chat-switch':
            fixture.create_shell_chats()
        sampler.service = fixture.service.pid
        if scenario == 'W5':
            record.update(status='partial', failure_cases=exercise(fixture), reason='Supported bounded IPC failures; remaining cases explicitly unavailable')
            return record
        if scenario == 'W0' and label is None:
            record['startupEmpty'] = {'coldMs': [], 'warmMs': [], 'samples': []}
            startup(fixture, args, sampler, record['startupEmpty'])
            fixture.stop_child(fixture.service)
            fixture.seed(chats=0)
            fixture.start_service()
            record['startup100Projects'] = {'coldMs': [], 'warmMs': [], 'samples': []}
            startup(fixture, args, sampler, record['startup100Projects'])
        launch_ui(fixture, scenario, args, terminals, 'terminal' if idle_mode == 'empty-terminal' else idle_mode)
        timeout = 1900 if scenario == 'W6' else 600 + args.idle_seconds
        ui = wait_ui(fixture, sampler, time.monotonic()+timeout)
        record.update(status='complete', ui=ui)
        if scenario == 'W2':
            counts = [json.loads(p.read_text()) for p in (fixture.root/'data').glob('producer-load-*.json')]
            record['actualProducedBytes'] = sum(p['produced'] for p in counts) if counts else None
            record['producerCompletions'] = sum(p['state'] == 'complete' for p in counts)
            if record['producerCompletions'] != terminals:
                record.update(status='partial', reason='Not all bounded pressure producers reported completion')
        record['derived'] = derive(sampler.rows)
    except KeyboardInterrupt:
        record.update(status='interrupted')
    except EnvironmentUnavailable as error:
        record.update(status='unavailable', classification='environment_unavailable', error=str(error))
    except UiFailure as error:
        record.update(status=error.classification, classification=error.classification,
                      error=str(error), ui=error.payload)
    except (OSError, RuntimeError, ValueError, TimeoutError) as error:
        # No terminal content, argv, full environment or provider output in report.
        record.update(status='driver_failure', classification='unclassified_harness_or_ipc_failure', error=str(error))
    finally:
        handlers = {sig: signal.signal(sig, signal.SIG_IGN) for sig in (signal.SIGINT, signal.SIGTERM)}
        try:
            record.setdefault('ui', fixture.last_ui)
            record['clock'] = {'samples': fixture.clock_samples, 'tolerance_ms': 5,
                'comparison_valid': all(clock_valid(a, b) for a, b in zip(fixture.clock_samples, fixture.clock_samples[1:])) if fixture.clock_samples else None}
            record['derived'] = derive(sampler.rows)
            record['producerStates'] = []
            for path in (fixture.root/'data').glob('producer-*.json'):
                try:
                    record['producerStates'].append(json.loads(path.read_text()))
                except (OSError, ValueError):
                    record['producerStates'].append({'tag': path.stem, 'state': 'counter_unreadable', 'produced': None})
            record['duration_s'] = time.monotonic()-start
            record['samples'] = sampler.rows
            record['samplingOverhead'] = {'cpu_s': sampler.overhead_cpu_s, 'wall_s': sampler.overhead_wall_s,
                'calls': len(sampler.rows), 'cpu_percent_one_core': sampler.overhead_cpu_s/max(.001, record['duration_s'])*100}
        finally:
            try:
                record['cleanup'] = fixture.cleanup()
            except (OSError, RuntimeError, subprocess.TimeoutExpired) as error:
                record.update(status='cleanup_failure', cleanup={'live_survivors': None,
                    'error': str(error), 'retained_fixture': str(fixture.root)})
            finally:
                for sig, handler in handlers.items():
                    signal.signal(sig, handler)

    return record


def derive(rows):
    def app_sum(row, metric):
        values = [row['groups'][g][metric] for g in ('service', 'ui_webkit')]
        return sum(values) if all(v is not None for v in values) else None

    idle = [row for row in rows if row['phase'] == 'idle']
    steady = [row for row in idle if row['at_monotonic_s'] - idle[0]['at_monotonic_s'] >= 30] if idle else []
    cpu = [app_sum(row, 'cpu_percent') for row in steady]
    cpu = [v for v in cpu if v is not None]
    pss = [app_sum(row, 'pss_kib') for row in rows]
    before = [row for row in rows if row['phase'] == 'cycle-before']
    after = [row for row in rows if row['phase'] == 'settle'][-5:]
    growth = {}
    for metric in ('pss_kib', 'fds', 'threads'):
        old = [app_sum(row, metric) for row in before]
        new = [app_sum(row, metric) for row in after]
        old, new = [v for v in old if v is not None], [v for v in new if v is not None]
        growth[metric] = (sum(new)/len(new)-sum(old)/len(old)) if old and new else None
    return {'idle_cpu_percent_one_core_mean': sum(cpu)/len(cpu) if cpu else None,
            'idle_samples_after_30s': len(cpu),
            'peak_app_pss_kib': max((v for v in pss if v is not None), default=None),
            'view_cycles_growth': growth, 'background_cpu': None, 'rust_subscriptions': None,
            'catalog_service_only_pss': catalog_pss(rows)}


def catalog_pss(rows):
    means = {}
    for phase in ('catalog-service-base', 'catalog-service-large'):
        values = [row['groups']['service']['pss_kib'] for row in rows if row['phase'] == phase
                  and row['groups']['service']['pss_kib'] is not None]
        means[phase] = sum(values)/len(values) if values else None
    base, large = means.values()
    return {'base_kib': base, 'large_kib': large,
            'delta_kib': large-base if base is not None and large is not None else None,
            'scope': 'service-only; UI-inclusive catalog goal remains unmeasured if snapshot fails'}


def summary(workloads):
    result = {}
    for record in workloads:
        name = record['scenario']
        for catalog in ('startupEmpty', 'startup100Projects'):
            for mode in ('coldMs', 'warmMs'):
                if catalog in record:
                    data = record[catalog]
                    values = [s['value_ms'] for s in data['samples'] if s['valid'] and s['mode']+'Ms' == mode] if 'samples' in data else data[mode]
                    result[f'{name}.{catalog}.{mode}'] = distribution(values)
        measured = record.get('ui', {}).get('measurements', {})
        for key in ('echoApproxMs', 'echo1ShellApproxMs', 'navigationApproxMs', 'chatSwitchApproxMs', 'interruptApproxMs', 'echoUnderLoadApproxMs'):
            if key in measured:
                result[f'{name}.{key}'] = distribution([v for v in measured[key] if isinstance(v, (int, float)) and v >= 0])
        for pressure in record.get('ui', {}).get('pressure', []):
            result[f"{name}.echoUnderLoad{pressure['terminals']}TerminalsMs"] = distribution(pressure.get('echoUnderLoadApproxMs', []))
        for group in ('service', 'ui_webkit', 'controlled', 'monitor'):
            for key in ('cpu_percent', 'pss_kib', 'rss_kib', 'fds', 'threads'):
                values = [row['groups'][group][key] for row in record.get('samples', [])
                          if row['groups'][group][key] is not None]
                result[f'{name}.{group}.{key}'] = distribution(values)
    return result


GOALS = [
    ('Startup quente instalado / 20 amostras', '≤ 1500 ms', 'W0.startupEmpty.warmMs'),
    ('Startup frio / 10 amostras', '≤ 3000 ms', 'W0.startupEmpty.coldMs'),
    ('CPU idle, estabilização 30 s + janela 60 s', '≤ 1% core', None),
    ('CPU minimizada', '≤ 0,5% core', None), ('PSS UI+WebKit+serviço', '≤ 250 MiB', None),
    ('Catálogo / aumento PSS', '≤ 75 MiB', None),
    ('Navegação catálogo p95', '≤ 100 ms', 'W3.navigationApproxMs'),
    ('Troca de chat p95 (shells via fixture de chat)', '≤ 150 ms', 'W1.chat-switch.chatSwitchApproxMs'),
    ('Echo renderizado idle / carga', '≤ 50 / 100 ms', 'W1.1.echoApproxMs'),
    ('Ctrl+C p95', '≤ 500 ms', 'W2.1.interruptApproxMs'), ('Leaks 100 ciclos', '≤ max(10%,20 MiB)', None),
    ('Watchers / jobs', 'limitados; sem varredura periódica', None)]


def render(report, previous=None):
    lines = ['# Eros — medição opt-in', '', f"Status: **{report['status']}**. Build release direto; pacote instalado não medido.",
             '', '## Metodologia e limites', '',
             'Linux /proc: CPU = delta(utime+stime)/delta(monotonic) × 100; 100% = um core. '
             'PSS/RSS/FDs/threads agregados por árvore de raízes possuídas, no máximo 1 Hz. '
             'Processos curtos entre amostras podem ser perdidos; primeiro delta CPU e grupo ausente são null. '
             'Não são coletados argv/env/conteúdo pessoal. p95: nearest rank ceil(0,95×n), distribuição min/mediana/p95/max.',
             '', 'Startup: relógio epoch da UI (performance.timeOrigin+now) correlacionado com time.time do monitor; '
             'cache de disco normal. Pronto = controle habilitado + dois rAF, aproximação de workspace interativo. '
             'Echo = escrita solicitada → callback xterm + dois rAF; não é tempo de retorno IPC e não prova pixels apresentados. '
             'Erro de composição/scheduling não limitado; duas frames acrescentam tipicamente 0–2 períodos de refresh. '
             'Sem medição GPU/pixels, metas de renderização permanecem não verificadas.',
             '', 'Fila xterm: bytes recebidos menos bytes cujo callback de parsing terminou; não mede buffers internos WebKit/IPC/kernel. '
             'Bytes PTY incluem prompts/controle e transformação LF→CRLF; produção real vem de contador separado do produtor. '
             'Assinaturas Rust e filas internas: não medidas. Animações off/on usam Home equivalente com shell retido; minimização exige confirmação nativa. '
             'Overhead do sampler é medido por process_time e monotonic ao redor da coleta; custo JS de scan/callback é registrado por chunk; não limita custo total de rAF/composição/driver. Comparação pareada integral permanece pendente.',
             '', '## Metas propostas (sem alteração)', '', '| Medida | Meta | Resultado / limitação |', '| --- | --- | --- |']
    for label, target, key in GOALS:
        value = report['summary'].get(key, {}).get('p95') if key else None
        lines.append(f'| {label} | {target} | '+ (f'p95 {value:.3f}; diagnóstico, não verificado contra meta completa' if value is not None else 'não medida / não verificado')+' |')
    lines.extend(['', '## Workloads', ''])
    for item in report['workloads']:
        lines.append(f"- {item['scenario']}: {item['status']}; duração {item['duration_s']:.3f} s; amostras {len(item['samples'])}; cleanup {item['cleanup']['live_survivors']} sobreviventes.")
        if item.get('clock'):
            invalid = sum(not sample['valid'] for name in ('startupEmpty', 'startup100Projects') for sample in item.get(name, {}).get('samples', []))
            lines.append(f"  Clock: comparação de startup válida={item['clock']['comparison_valid']}; amostras rejeitadas={invalid}; tolerância de salto 5 ms, bracket máximo 1 ms. Métricas monotônicas independentes preservadas.")
        if item.get('ui'):
            lines.append('  UI parcial/final: '+json.dumps(item['ui'], ensure_ascii=False))
        if item.get('catalog'):
            lines.append('  Catálogo/IPC: '+json.dumps({k: item[k] for k in ('catalog', 'ipc', 'liveness_after_snapshot') if k in item}, ensure_ascii=False))
        if item.get('derived'):
            lines.append('  Recursos derivados (UI + serviço): '+json.dumps(item['derived'], ensure_ascii=False))
        if item.get('error') or item.get('reason'):
            lines.append('  '+item.get('error', item.get('reason')))
    calibration = report.get('monitorCalibration')
    if calibration:
        lines.extend(['', f"Calibração sem aplicativo: {len(calibration['samples'])} amostras; CPU da coleta {calibration['sampling_cpu_s']:.6f} s; parede da coleta {calibration['sampling_wall_s']:.6f} s em {calibration['duration_s']:.3f} s. Inclui monitor apenas; não é desempenho Eros."])
    lines.extend(['', '## Resumo medido', '', '| Métrica | n | min | mediana | p95 | max |', '| --- | --- | --- | --- | --- | --- |'])
    for key, value in report['summary'].items():
        if value['n']:
            lines.append('| '+key+' | '+' | '.join(str(round(value[k], 4)) if isinstance(value[k], float) else str(value[k]) for k in ('n','min','median','p95','max'))+' |')
    lines.extend(['', '## Comparação entre rodadas', ''])
    if previous:
        compatible = (previous['environment']['binary_sha256'] == report['environment']['binary_sha256']
                      and previous['environment']['source_sha256'] == report['environment']['source_sha256']
                      and previous['environment']['configuration'] | {'output': None, 'compare': None} == report['environment']['configuration'] | {'output': None, 'compare': None}
                      and all(previous['environment'][k] == report['environment'][k] for k in ('cpu_model','kernel','versions')))
        lines.append(f'Configuração/binário/ambiente comparáveis: {compatible}. Rodadas bloqueadas não constituem baseline.')
        for key, value in report['summary'].items():
            old = previous.get('summary', {}).get(key, {}).get('p95')
            startup_clock_ok = not ('.coldMs' in key or '.warmMs' in key) or all(w.get('clock', {}).get('comparison_valid') is True for r in (previous, report) for w in r['workloads'] if 'startupEmpty' in w)
            if compatible and startup_clock_ok and old and value['p95'] is not None:
                lines.append(f"- {key}: variação p95 {(value['p95']/old-1)*100:.2f}% (investigar >10%; repetir se próximo do ruído).")
    else:
        lines.append('Use --compare rodada-anterior.json no mesmo build/configuração/hardware.')
    lines.extend(['', 'Ambiente e amostras brutas: JSON adjacente. Valores ausentes são null, nunca zero.', ''])
    return '\n'.join(lines)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--binary', type=Path, default=REPO/'target/release/eros-desktop')
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--compare', type=Path)
    parser.add_argument('--workloads', default='W0,W1,W2,W3')
    parser.add_argument('--idle-seconds', type=int, default=90)
    parser.add_argument('--pressure-seconds', type=int, default=60)
    parser.add_argument('--cycles', type=int, default=100)
    parser.add_argument('--cold-samples', type=int, default=10)
    parser.add_argument('--warm-samples', type=int, default=20)
    args = parser.parse_args()
    if not args.binary.is_file():
        parser.error('Build release first; see scripts/performance/README.md')
    if not (1 <= args.pressure_seconds <= 60 and 1 <= args.idle_seconds <= 3600 and
            1 <= args.cycles <= 100 and 1 <= args.cold_samples <= 20 and 1 <= args.warm_samples <= 40):
        parser.error('Invalid bounded measurement parameters')
    workloads = args.workloads.split(',')
    if any(w not in [f'W{i}' for i in range(7)] for w in workloads):
        parser.error('Unknown workload')
    signal.signal(signal.SIGTERM, lambda *_: (_ for _ in ()).throw(KeyboardInterrupt()))
    binary = args.binary.resolve()
    report = {'schema_version': 2, 'environment': metadata(binary, args), 'workloads': [], 'status': 'running'}
    args.output.parent.mkdir(parents=True, exist_ok=True)
    exit_code = 0
    try:
        calibration = Sampler()
        calibration_start = time.monotonic()
        for _ in range(5):
            calibration.sample('sampler-calibration-no-application')
            time.sleep(1)
        report['monitorCalibration'] = {'duration_s': time.monotonic()-calibration_start,
            'sampling_cpu_s': calibration.overhead_cpu_s, 'sampling_wall_s': calibration.overhead_wall_s,
            'samples': calibration.rows}
        for scenario in workloads:
            stages = [(scenario, 1, 'terminal', None)]
            if scenario in ('W1', 'W2'):
                stages = [(scenario, n, 'terminal', f'{scenario}.{n}') for n in (1, 8)]
            if scenario == 'W1':
                stages.append(('chat-switch', 8, 'terminal', 'W1.chat-switch'))
            if scenario == 'W0':
                stages.extend(('W0', 1, mode, f'W0.{mode}') for mode in ('empty-terminal', 'home-off', 'home-on', 'minimized'))
            for native_scenario, terminals, idle_mode, label in stages:
                print(f'{label or scenario}: native isolated fixture', flush=True)
                report['workloads'].append(workload(native_scenario, binary, args, terminals, idle_mode, label))
                if report['workloads'][-1]['status'] == 'interrupted':
                    raise KeyboardInterrupt
    except KeyboardInterrupt:
        report['status'] = 'interrupted'
        exit_code = 130
    else:
        statuses = {w['status'] for w in report['workloads']}
        if statuses <= {'complete', 'product_failure'}:
            report['status'] = 'complete_with_product_failures' if 'product_failure' in statuses else 'complete'
        else:
            report['status'] = 'incomplete'
        exit_code = 0 if report['status'] == 'complete' else 2
    finally:
        report['environment']['clock_offset_ms_after'] = correlate()['offset_ms']
        report['summary'] = summary(report['workloads'])
        previous = json.loads(args.compare.read_text()) if args.compare else None
        args.output.write_text(json.dumps(report, indent=2)+'\n')
        args.output.with_suffix('.md').write_text(render(report, previous))
    print(f"{report['status']}: {args.output}")
    return exit_code


if __name__ == '__main__':
    sys.exit(main())
