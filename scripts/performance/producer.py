#!/usr/bin/env python3
"""Paced synthetic payload, independent deadline, lifecycle records and markers."""
import argparse
import json
import os
from pathlib import Path
import re
import signal
import time


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--rate', type=int, required=True)
    parser.add_argument('--seconds', type=int, required=True)
    parser.add_argument('--tag', default='test')
    parser.add_argument('--started', type=int)
    parser.add_argument('--stopped', type=int)
    args = parser.parse_args()
    if not (1 <= args.rate <= 1048576 and 1 <= args.seconds <= 60):
        parser.error('rate <= 1 MiB/s, duration <= 60 seconds required')
    if not re.fullmatch(r'[a-z0-9-]{1,32}', args.tag):
        parser.error('invalid synthetic tag')
    if any(n is not None and not 1 <= n < 10**16 for n in (args.started, args.stopped)):
        parser.error('invalid synthetic marker id')
    def deadline(*_):
        raise TimeoutError('Producer deadline')
    signal.signal(signal.SIGALRM, deadline)
    signal.alarm(args.seconds + 3)
    block = b'x' * 4095 + b'\n'
    total, maximum = 0, args.rate * args.seconds
    start = time.monotonic()
    root = Path(os.environ['EROS_DATA_DIR']) if 'EROS_DATA_DIR' in os.environ else None

    def record(state):
        if root:
            path = root/f'producer-{args.tag}.json'
            temporary = path.with_suffix('.tmp')
            temporary.write_text(json.dumps({'tag': args.tag, 'state': state, 'produced': total}))
            temporary.replace(path)

    def marker(number):
        if number is not None:
            os.write(1, f'EROS_MARK_{number}_END\n'.encode())

    state = 'running'
    last_update = start
    try:
        while total < maximum:
            size = min(len(block), maximum-total)
            total += os.write(1, block[:size])
            if total == size:
                record('running')
                marker(args.started)  # Only after the first actual payload write.
            if time.monotonic()-last_update >= .5:
                record('running')
                last_update = time.monotonic()
            time.sleep(max(0, start + total/args.rate-time.monotonic()))
        state = 'complete'
    except KeyboardInterrupt:
        state = 'interrupted'
    except (TimeoutError, BrokenPipeError):
        state = 'deadline_or_disconnect'
    finally:
        record(state)
        if root:
            (root/f'producer-{os.getpid()}.count').write_text(str(total))
        signal.alarm(2)  # Even a blocked stop-marker write must terminate.
        try:
            marker(args.stopped)
        except (TimeoutError, BrokenPipeError):
            pass
        signal.alarm(0)
    return 0 if state == 'complete' else 130 if state == 'interrupted' else 2


if __name__ == '__main__':
    raise SystemExit(main())
