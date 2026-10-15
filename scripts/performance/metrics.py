"""Linux aggregate-only sampling. No argv, environment or terminal content retained."""
import math
import os
from pathlib import Path
import time

GROUPS = ('service', 'ui_webkit', 'controlled', 'monitor')


def distribution(values):
    values = sorted(v for v in values if isinstance(v, (int, float)) and math.isfinite(v))
    if not values:
        return {'n': 0, 'min': None, 'median': None, 'p95': None, 'max': None}
    return {'n': len(values), 'min': values[0],
            'median': (values[(len(values)-1)//2] + values[len(values)//2])/2,
            'p95': values[math.ceil(len(values)*.95)-1], 'max': values[-1]}


def cpu_percent(before, after, elapsed):
    return max(0, after-before) / elapsed * 100 if elapsed > 0 else None


def classify(pid, parents, service, ui, monitor):
    """Only owned roots and descendants; never select by process name."""
    if pid == monitor:
        return 'monitor'
    if pid == service:
        return 'service'
    seen = set()
    while pid and pid not in seen:
        seen.add(pid)
        if pid == ui:
            return 'ui_webkit'
        if pid == service:
            return 'controlled'
        pid = parents.get(pid, 0)
    return None


def process_stat(pid):
    raw = Path(f'/proc/{pid}/stat').read_text()
    fields = raw[raw.rfind(')')+2:].split()
    return {'parent': int(fields[1]), 'cpu_s': (int(fields[11])+int(fields[12]))/os.sysconf('SC_CLK_TCK'),
            'start': int(fields[19])}


def resources(pid):
    root = Path(f'/proc/{pid}')
    result = {'pss_kib': None, 'rss_kib': None, 'fds': None, 'threads': None}
    for filename in ('status', 'smaps_rollup'):
        try:
            for line in (root/filename).read_text().splitlines():
                key, _, value = line.partition(':')
                metric = {'Pss': 'pss_kib', 'Rss': 'rss_kib', 'VmRSS': 'rss_kib', 'Threads': 'threads'}.get(key)
                if metric:
                    result[metric] = int(value.split()[0])
        except (OSError, ValueError):
            pass
    try:
        result['fds'] = len(list((root/'fd').iterdir()))
    except OSError:
        pass
    return result


class Sampler:
    def __init__(self, service=None, ui=None):
        self.service, self.ui = service, ui
        self.previous = {}
        self.owners = {}
        self.rows = []
        self.overhead_cpu_s = 0
        self.overhead_wall_s = 0

    def sample(self, phase):
        wall, cpu = time.monotonic(), time.process_time()
        stats = {}
        for entry in Path('/proc').iterdir():
            if entry.name.isdigit():
                try:
                    stats[int(entry.name)] = process_stat(int(entry.name))
                except (OSError, ValueError, IndexError):
                    pass
        parents = {pid: stat['parent'] for pid, stat in stats.items()}
        grouped = {key: [] for key in GROUPS}
        for pid, stat in stats.items():
            identity = (pid, stat['start'])
            group = classify(pid, parents, self.service, self.ui, os.getpid()) or self.owners.get(identity)
            if group is None:
                continue
            values = resources(pid)
            self.owners[identity] = group
            previous = self.previous.get(identity)
            values['cpu_percent'] = cpu_percent(previous[1], stat['cpu_s'], wall-previous[0]) if previous else None
            self.previous[identity] = (wall, stat['cpu_s'])
            grouped[group].append(values)
        row = {'at_monotonic_s': wall, 'phase': phase, 'groups': {}}
        for group, values in grouped.items():
            # Missing/permission denied must propagate, including absent groups.
            row['groups'][group] = {'processes': len(values)}
            for key in ('cpu_percent', 'pss_kib', 'rss_kib', 'fds', 'threads'):
                row['groups'][group][key] = (sum(v[key] for v in values)
                    if values and all(v[key] is not None for v in values) else None)
        live = {(pid, stat['start']) for pid, stat in stats.items()}
        self.previous = {key: value for key, value in self.previous.items() if key in live}
        self.owners = {key: value for key, value in self.owners.items() if key in live}
        self.rows.append(row)
        self.overhead_cpu_s += time.process_time()-cpu
        self.overhead_wall_s += time.monotonic()-wall
        return row
