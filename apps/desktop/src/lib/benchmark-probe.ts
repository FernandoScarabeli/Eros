/** Bounded synthetic marker observer; no retained terminal content. */
const MAX_PENDING = 64;
const MAX_MARKER_LENGTH = 'EROS_MARK_'.length + 16 + '_END'.length;
type Ticket = { pending: boolean; at: number | null };
export class MarkerObserver {
  suffix = '';
  readonly tickets = new Map<string, Ticket>();
  active = true;

  arm(marker: string) {
    if (!/^EROS_MARK_[0-9]{1,16}_END$/.test(marker)) throw new Error('Invalid synthetic marker');
    if (this.tickets.has(marker) || this.tickets.size >= MAX_PENDING) throw new Error('Marker budget exceeded');
    this.tickets.set(marker, { pending: false, at: null });
  }

  scan(chunk: string): () => void {
    const retainedLength = this.suffix.length;
    const combined = this.suffix + chunk;
    const found: [string, Ticket][] = [];
    for (const match of combined.matchAll(/EROS_MARK_[0-9]{1,16}_END/g)) {
      if (match.index + match[0].length <= retainedLength) continue;
      const ticket = this.tickets.get(match[0]);
      if (ticket && !ticket.pending) {
        ticket.pending = true;
        found.push([match[0], ticket]);
      }
    }
    this.suffix = combined.slice(-(MAX_MARKER_LENGTH - 1));
    return () => {
      if (!this.active) return;
      const at = performance.now();
      for (const [marker, ticket] of found) {
        if (this.tickets.get(marker) === ticket && ticket.at === null) ticket.at = at;
      }
    };
  }

  has(marker: string) { return this.tickets.get(marker)?.at != null; }
  take(marker: string) {
    const at = this.tickets.get(marker)?.at;
    if (at == null) throw new Error('Marker not acknowledged');
    this.tickets.delete(marker);
    return at;
  }
  dispose() { this.active = false; this.tickets.clear(); this.suffix = ''; }
}

export type Probe = {
  readonly id: string; readonly viewId: string; readonly panelId: string;
  delivered: number; queued: number; peakQueued: number; ready: boolean;
  observer: MarkerObserver; bindingMismatch?: { session: string; panel: string };
  scanWallMs: number; callbackWallMs: number; scannedChunks: number;
};
export type Benchmark = { probes: Map<string, Probe>; instances: Map<string, Probe>; animations: 'off' | 'on'; detailed: boolean };
declare global { interface Window { __EROS_BENCHMARK__?: Benchmark } }

export function registerProbe(benchmark: Benchmark, probe: Probe) {
  if (benchmark.instances.size >= 32) throw new Error('Mounted probe budget exceeded');
  benchmark.instances.set(probe.viewId, probe);
  benchmark.probes.set(probe.id, probe);
}

export function removeProbe(registry: Map<string, Probe>, probe: Probe, instances?: Map<string, Probe>) {
  probe.observer.dispose();
  if (registry.get(probe.id) === probe) registry.delete(probe.id);
  if (instances?.get(probe.viewId) === probe) instances.delete(probe.viewId);
}

/** Command text never contains the complete marker, even while ECHO is on. */
export function markerCommand(marker: string, disableEcho = false) {
  if (!/^EROS_MARK_[0-9]{1,16}_END$/.test(marker)) throw new Error('Invalid synthetic marker');
  const id = marker.slice('EROS_MARK_'.length, -'_END'.length);
  return `${disableEcho ? 'stty -echo && ' : ''}printf 'EROS_%s_%s_END\\n' MARK ${id}\r`;
}
