import assert from 'node:assert/strict';
import test from 'node:test';
import { readFileSync } from 'node:fs';
import ts from 'typescript';
const source = readFileSync(new URL('../src/lib/benchmark-probe.ts', import.meta.url), 'utf8');
const emitted = ts.transpileModule(source, { compilerOptions: { module: ts.ModuleKind.ESNext, target: ts.ScriptTarget.ES2022 } }).outputText;
const { MarkerObserver, markerCommand, registerProbe, removeProbe } = await import(`data:text/javascript;base64,${Buffer.from(emitted).toString('base64')}`);

for (const position of ['beginning', 'middle', 'end']) {
  test(`32 KiB chunk marker at ${position}`, () => {
    const marker = 'EROS_MARK_12345_END';
    const observer = new MarkerObserver();
    observer.arm(marker);
    const start = position === 'beginning' ? 0 : position === 'middle' ? 16000 : 32768 - marker.length;
    const text = 'x'.repeat(start) + marker + 'x'.repeat(32768 - start - marker.length);
    const callback = observer.scan(text);
    assert.equal(observer.has(marker), false, 'receipt is not parsing/render acknowledgement');
    assert.ok(observer.suffix.length <= 29);
    callback();
    assert.equal(observer.has(marker), true);
    assert.ok(observer.take(marker) >= 0);
    assert.equal(observer.tickets.size, 0);
  });
}

test('all marker boundaries across chunks', () => {
  const marker = 'EROS_MARK_1234567890123456_END';
  for (let boundary = 1; boundary < marker.length; boundary++) {
    const observer = new MarkerObserver();
    observer.arm(marker);
    observer.scan('x'.repeat(32768) + marker.slice(0, boundary))();
    assert.equal(observer.has(marker), false);
    const done = observer.scan(marker.slice(boundary) + 'x'.repeat(32768));
    assert.equal(observer.has(marker), false);
    done();
    assert.equal(observer.has(marker), true);
  }
});

test('multiple markers, delayed callbacks and duplicate suffix do not overwrite timestamps', () => {
  const observer = new MarkerObserver();
  const a = 'EROS_MARK_1_END', b = 'EROS_MARK_2_END';
  observer.arm(a); observer.arm(b);
  const late = observer.scan(a + 'x'.repeat(100) + b);
  observer.scan(b + 'noise')();
  assert.equal(observer.has(a), false);
  assert.equal(observer.has(b), false);
  late();
  const first = observer.take(a);
  observer.arm(a);
  late();
  assert.equal(observer.has(a), false, 'late callback must not acknowledge a replacement ticket');
  assert.ok(observer.take(b) >= first);
});

test('an old complete marker retained in suffix cannot satisfy a rearmed ticket', () => {
  const observer = new MarkerObserver();
  const marker = 'EROS_MARK_9_END';
  observer.arm(marker); observer.scan(marker)(); observer.take(marker);
  observer.arm(marker); observer.scan('unrelated output')();
  assert.equal(observer.has(marker), false);
});

test('command echo cannot satisfy either handshake or echo probe', () => {
  const marker = 'EROS_MARK_7_END';
  for (const disableEcho of [true, false]) {
    const observer = new MarkerObserver();
    observer.arm(marker);
    const command = markerCommand(marker, disableEcho);
    assert.equal(command.includes(marker), false);
    if (disableEcho) assert.ok(command.startsWith('stty -echo && '));
    observer.scan(command)();
    assert.equal(observer.has(marker), false);
    observer.scan(marker + '\r\n')();
    assert.equal(observer.has(marker), true);
  }
});

test('pending marker budget, retained suffix and disposal stay bounded', () => {
  const observer = new MarkerObserver();
  for (let i = 0; i < 64; i++) observer.arm(`EROS_MARK_${i}_END`);
  assert.throws(() => observer.arm('EROS_MARK_65_END'), /budget/);
  observer.scan('x'.repeat(2 * 1024 * 1024))();
  assert.ok(observer.suffix.length <= 29);
  const pending = observer.scan('EROS_MARK_0_END');
  observer.dispose(); pending();
  assert.equal(observer.tickets.size, 0);
  assert.equal(observer.suffix, '');
});

test('immutable old-instance cleanup cannot remove replacement view probe', () => {
  const old = { id: 'session', viewId: 'old', observer: new MarkerObserver() };
  const replacement = { id: 'session', viewId: 'new', observer: new MarkerObserver() };
  const probes = new Map([['session', replacement]]);
  removeProbe(probes, old);
  assert.equal(probes.get('session'), replacement);
  assert.equal(replacement.observer.active, true);
  removeProbe(probes, replacement);
  assert.equal(probes.size, 0);
});

test('displaced subscription instance remains auditable until its own cleanup', () => {
  const benchmark = { probes: new Map(), instances: new Map(), animations: 'off', detailed: true };
  const old = { id: 'session-2', viewId: 'old', observer: new MarkerObserver() };
  const replacement = { id: 'session-2', viewId: 'new', observer: new MarkerObserver() };
  registerProbe(benchmark, old); registerProbe(benchmark, replacement);
  old.bindingMismatch = { session: 'session-3', panel: 'panel-3' };
  assert.equal(benchmark.probes.get('session-2'), replacement);
  assert.equal([...benchmark.instances.values()].filter(p => p.bindingMismatch).length, 1);
  removeProbe(benchmark.probes, old, benchmark.instances);
  assert.equal(benchmark.instances.size, 1);
  assert.equal(benchmark.probes.get('session-2'), replacement);
  for (let i = 1; i < 32; i++) registerProbe(benchmark, { id: String(i), viewId: String(i), observer: new MarkerObserver() });
  assert.throws(() => registerProbe(benchmark, { id: 'overflow', viewId: 'overflow', observer: new MarkerObserver() }), /budget/);
});
