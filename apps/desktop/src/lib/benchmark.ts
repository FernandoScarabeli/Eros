/** Native opt-in driver. Each stage runs in its own fixture/window. */
import { invoke } from '@tauri-apps/api/core';
import { resizeTerminal, writeTerminal } from './api';
import { markerCommand } from './benchmark-probe';
import type { Probe } from './benchmark-probe';

type Config = {
  scenario: string; terminals: number; idleSeconds: number; pressureSeconds: number;
  cycles: number; idleMode: 'terminal' | 'home-off' | 'home-on' | 'minimized';
};
type ProducerState = { tag: string; state: string; produced: number };
const delay = (ms: number) => new Promise<void>((resolve) => setTimeout(resolve, ms));
const paint = () => new Promise<void>((resolve) => requestAnimationFrame(() => requestAnimationFrame(() => resolve())));
class StageFailure extends Error {
  constructor(message: string, readonly classification: 'driver_failure' | 'product_failure' = 'driver_failure') { super(message); }
}

export async function configureBenchmark(): Promise<Config | null> {
  const config = await invoke<Config | null>('benchmark_config');
  if (config) window.__EROS_BENCHMARK__ = { probes: new Map(), instances: new Map(), animations: config.idleMode === 'home-on' ? 'on' : 'off', detailed: true };
  if (config && window.__EROS_BENCHMARK__?.animations === 'off') {
    const style = document.createElement('style');
    style.textContent = '*,*::before,*::after{animation:none!important;transition:none!important}';
    document.head.append(style);
  }
  return config;
}

export async function runBenchmark(config: Config) {
  const state = window.__EROS_BENCHMARK__!;
  let sequence = 0;
  const result: Record<string, unknown> = { scenario: config.scenario, terminals: config.terminals, status: 'running', measurements: {} };
  const measurements = result.measurements as Record<string, unknown>;
  const diagnostics = () => ({
    expectedProbes: result.expectedProbes ?? null,
    uiError: (document.querySelector('.app-alert > span')?.textContent ?? '').replace(/\/tmp\/eros-perf-[^/\s]+/g, '<fixture>').slice(0, 512),
    observedProbes: state.probes.size,
    observedViews: state.instances.size,
    readyProbes: [...state.probes.values()].filter((p) => p.ready).length,
    probes: [...state.instances.values()].map((p) => ({ session: p.id, view: p.viewId, panel: p.panelId,
      ready: p.ready, bindingMismatch: p.bindingMismatch ?? null, delivered: p.delivered, queued: p.queued })),
    renderedSessions: [...document.querySelectorAll<HTMLElement>('[data-benchmark-session]')].map((e) => e.dataset.benchmarkSession),
  });
  let reportQueue = Promise.resolve();
  const report = async (phase: string) => {
    result.phase = phase;
    const value = structuredClone({ ...result, diagnostic: diagnostics() });
    reportQueue = reportQueue.catch(() => {}).then(() => invoke<void>('benchmark_report', { value }));
    await reportQueue;
  };
  async function until(condition: string, test: () => boolean, expected: number | null = null, timeout = 15000) {
    result.condition = condition;
    result.expectedProbes = expected;
    await report(String(result.phase ?? 'startup'));
    const deadline = performance.now() + timeout;
    while (!test()) {
      if ([...state.instances.values()].some((p) => p.bindingMismatch)) {
        throw new StageFailure('Mounted terminal identity changed without remount; EROS-005/006', 'product_failure');
      }
      if (performance.now() >= deadline) throw new StageFailure(`Timeout: ${condition}`);
      await delay(10);
    }
  }
  function click(selector: string) {
    const element = document.querySelector<HTMLButtonElement>(selector);
    if (!element) throw new StageFailure(`Missing control: ${selector}`);
    element.click();
  }
  const ready = () => !!document.querySelector('.app-shell[data-benchmark-ready="true"]');
  function marker() { return `EROS_MARK_${++sequence}_END`; }
  async function echo(probe: Probe, disableEcho = false) {
    const mark = marker();
    probe.observer.arm(mark);
    const start = performance.now();
    await writeTerminal(probe.id, markerCommand(mark, disableEcho));
    await until(disableEcho ? 'stty-success-output-handshake' : 'echo-output-parsed-plus-two-frames', () => probe.observer.has(mark));
    const latency = probe.observer.take(mark) - start;
    if (!Number.isFinite(latency) || latency < 0) throw new StageFailure('Invalid monotonic echo latency');
    return latency;
  }
  async function mountShells(count: number) {
    await report('mount-terminals');
    for (let i = 0; i < count; i++) {
      await until(`application-ready-before-terminal-${i + 1}`, ready);
      click('button[aria-label="Novo terminal"]');
      await until(`mount-terminal-${i + 1}: exact-count-and-all-subscriptions-ready`, () =>
        state.probes.size === i + 1 && state.instances.size === i + 1 && [...state.probes.values()].every((p) => p.ready), i + 1);
    }
    const probes = [...state.probes.values()];
    for (const probe of probes) await echo(probe, true);
    return probes;
  }
  const producerStates = () => invoke<ProducerState[]>('benchmark_producers');
  async function producerCommand(probe: Probe, tag: string, rate: number, background: boolean) {
    const started = marker(), stopped = marker();
    probe.observer.arm(started);
    if (!background) probe.observer.arm(stopped);
    const startedId = started.slice(10, -4), stoppedId = stopped.slice(10, -4);
    await writeTerminal(probe.id, `python3 "$HOME/../producer.py" --rate ${rate} --seconds ${config.pressureSeconds} --tag ${tag} --started ${startedId} --stopped ${stoppedId}${background ? ' &' : ''}\r`);
    await until(`producer-${tag}: first-payload-and-start-marker`, () => probe.observer.has(started));
    probe.observer.take(started);
    return stopped;
  }
  async function pressure(probes: Probe[]) {
    const rate = probes.length === 1 ? 1048576 : 262144;
    const samples: number[] = [];
    const sampleOffsetsMs: number[] = [];
    measurements.echoUnderLoadApproxMs = samples;
    measurements.echoUnderLoadOffsetsMs = sampleOffsetsMs;
    await report('pressure-start');
    const started = performance.now();
    await Promise.all(probes.map((p, i) => producerCommand(p, `load-${i}`, rate, true)));
    measurements.pressureConfirmedAtMs = performance.now() - started;
    await report('pressure');
    const n = config.pressureSeconds >= 20 ? 20 : 2;
    for (let i = 0; i < n; i++) {
      await delay(Math.max(0, config.pressureSeconds * 1000 * (i + .25) / n - (performance.now() - started)));
      const statuses = await producerStates();
      if (statuses.filter((p) => p.tag.startsWith('load-') && p.state === 'running' && p.produced > 0).length !== probes.length) {
        throw new StageFailure('Producer stopped before scheduled load echo; pressure sample invalid', 'product_failure');
      }
      sampleOffsetsMs.push(performance.now() - started);
      samples.push(await echo(probes[i % probes.length]));
      await report('pressure');
    }
    await delay(Math.max(0, (config.pressureSeconds + 2) * 1000 - (performance.now() - started)));
    const statuses = await producerStates();
    measurements.producers = statuses;
    if (statuses.filter((p) => p.tag.startsWith('load-') && p.state === 'complete').length !== probes.length) {
      throw new StageFailure('Bounded pressure producer did not finish normally', 'product_failure');
    }
    await until('xterm-parsing-queue-drained', () => probes.every((p) => p.queued === 0));
  }
  async function interrupt(probe: Probe) {
    const samples: number[] = [];
    measurements.interruptApproxMs = samples;
    await report('foreground-interrupt');
    for (let i = 0; i < 20; i++) {
      const tag = `interrupt-${i}`;
      const stopped = await producerCommand(probe, tag, 1048576, false);
      const recovered = marker();
      probe.observer.arm(recovered);
      const start = performance.now();
      await writeTerminal(probe.id, '\x03');
      // Wait for foreground finally before sending input: a SIGINT flush must not erase the recovery command.
      await until(`foreground-${i}: stop-marker-before-input`, () => probe.observer.has(stopped));
      await writeTerminal(probe.id, markerCommand(recovered));
      await until(`foreground-${i}: producer-finally-and-shell-input-recovery`, () =>
        probe.observer.has(stopped) && probe.observer.has(recovered));
      const stoppedAt = probe.observer.take(stopped), recoveredAt = probe.observer.take(recovered);
      const statuses = await producerStates();
      if (!statuses.some((p) => p.tag === tag && p.state === 'interrupted')) {
        throw new StageFailure('Foreground producer was not interrupted', 'product_failure');
      }
      samples.push(Math.max(stoppedAt, recoveredAt) - start);
      await report('foreground-interrupt');
    }
  }
  try {
    await until('initial-app-snapshot-ready', ready);
    await paint();
    result.readyEpochMs = performance.timeOrigin + performance.now();
    const uiOffsetMs = Date.now() - (performance.timeOrigin + performance.now());
    result.uiClock = { epochDeltaMs: uiOffsetMs, toleranceMs: 5, valid: Math.abs(uiOffsetMs) <= 5 };
    await report('ready');
    if (config.scenario === 'startup') return;
    if (config.scenario === 'W0') {
      await mountShells(1);
      if (config.idleMode.startsWith('home')) {
        click('button[aria-label="Eros, início"]');
        await until('home-mounted-for-animation-comparison', () => !!document.querySelector('.home-content'));
        measurements.animationCanvasMounted = !!document.querySelector('.terminal-atmosphere-canvas');
      }
      if (config.idleMode === 'minimized') {
        const minimized = await invoke<boolean>('benchmark_window', { minimized: true });
        if (!minimized) throw new StageFailure('Window manager did not confirm minimized state');
      }
      await report('idle');
      await delay(config.idleSeconds * 1000);
      await report('idle-complete');
      if (config.idleMode === 'minimized') await invoke('benchmark_window', { minimized: false });
    } else if (config.scenario === 'chat-switch') {
      const times: number[] = [];
      measurements.chatSwitchApproxMs = times;
      measurements.fixture = 'eight real local shells via fixture-only CLI double; no provider/authentication';
      for (let i = 0; i < 100; i++) {
        const id = `bench-chat-${(i + 1) % 8}`;
        const button = document.querySelector<HTMLButtonElement>(`button[title="Synthetic shell chat ${(i + 1) % 8}"]`);
        if (!button) throw new StageFailure('Fixture chat missing');
        const start = performance.now();
        button.click();
        await until(`chat-${i}: target-mounted-and-subscribed`, () => !!state.probes.get(id)?.ready, 1);
        const probe = state.probes.get(id)!;
        await echo(probe, true);
        times.push(performance.now() - start);
        await report('chat-switch');
      }
    } else if (config.scenario === 'W1' || config.scenario === 'W2' || config.scenario === 'W6') {
      const probes = await mountShells(config.terminals);
      const echoes: number[] = [];
      measurements.echoApproxMs = echoes;
      for (let i = 0; i < 20; i++) { echoes.push(await echo(probes[i % probes.length])); await report('idle-echo'); }
      if (config.scenario === 'W1') {
        for (const probe of probes) {
          await writeTerminal(probe.id, "printf '\\033[?1049h\\033[2J\\033[Hação 🌾 \\033[31mANSI\\033[0m\\n'; printf '\\033[?1049l'\r");
          await resizeTerminal(probe.id, 80, 24);
          await echo(probe);
          await resizeTerminal(probe.id, 120, 40);
          await echo(probe);
        }
      }
      if (config.scenario === 'W2') { await pressure(probes); if (probes.length === 1) await interrupt(probes[0]); }
      if (config.scenario === 'W6') {
        await report('long-duration');
        for (let i = 0; i < 180; i++) { await echo(probes[0]); await delay(10000); }
      }
      await report('cycle-before');
      await delay(2000);
      for (let i = 0; i < config.cycles; i++) {
        await until(`cycle-${i}: application-ready`, ready);
        click('button[aria-label="Novo terminal"]');
        await until(`cycle-${i}: view-mounted`, () => state.probes.size === probes.length + 1 && [...state.probes.values()].every((p) => p.ready), probes.length + 1);
        const buttons = document.querySelectorAll<HTMLButtonElement>('button[aria-label="Encerrar terminal e seus processos"]');
        buttons[buttons.length - 1].click();
        await until(`cycle-${i}: view-disposed`, () => state.probes.size === probes.length, probes.length);
        measurements.completedViewCycles = i + 1;
        await report('view-cycles');
      }
    } else if (config.scenario === 'W3') {
      const times: number[] = [];
      measurements.navigationApproxMs = times;
      for (let i = 0; i < 100; i++) {
        const buttons = document.querySelectorAll<HTMLButtonElement>('.project-entry');
        if (buttons.length !== 100) throw new StageFailure('Expected 100 persisted projects', 'product_failure');
        const start = performance.now();
        buttons[i].click();
        await until(`catalog-scope-${i}`, () => buttons[i].getAttribute('aria-current') === 'page');
        await paint(); times.push(performance.now() - start); await report('catalog-navigation');
      }
      const search = document.querySelector<HTMLInputElement>('input[type="search"]');
      if (!search) throw new StageFailure('Catalog search control missing');
      const start = performance.now();
      search.value = 'Synthetic chat 00001';
      search.dispatchEvent(new Event('input', { bubbles: true }));
      await until('catalog-search-filtered-results', () => document.querySelectorAll('.chat-entry').length === 1);
      await paint();
      measurements.searchApproxMs = performance.now() - start;
      search.value = ''; search.dispatchEvent(new Event('input', { bubbles: true }));
      await paint();
      measurements.sorting = { status: 'unavailable', reason: 'No user-facing sort control; EROS-023' };
    }
    measurements.instrumentation = [...state.probes.values()].map((p) => ({ session: p.id, chunks: p.scannedChunks,
      scanWallMs: p.scanWallMs, callbackWallMs: p.callbackWallMs, delivered: p.delivered, peakQueued: p.peakQueued }));
    result.status = 'complete';
    await report('settle');
    await delay(30000);
    await report('done');
  } catch (error) {
    result.status = 'failed';
    result.classification = error instanceof StageFailure ? error.classification : 'driver_failure';
    result.error = String(error);
    result.failedPhase = result.phase;
    measurements.instrumentation = [...state.probes.values()].map((p) => ({ session: p.id,
      scanWallMs: p.scanWallMs, callbackWallMs: p.callbackWallMs, chunks: p.scannedChunks, delivered: p.delivered, peakQueued: p.peakQueued }));
    await report('done');
  }
}
