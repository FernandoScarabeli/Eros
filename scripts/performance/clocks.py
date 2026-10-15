"""Epoch/monotonic correlation with conservative uncertainty and jump rejection."""
import math
import time

TOLERANCE_MS = 5.0
MAX_BRACKET_MS = 1.0


def correlate(epoch=time.time, monotonic=time.monotonic):
    before = monotonic()
    wall = epoch()
    after = monotonic()
    return {'offset_ms': (wall-(before+after)/2)*1000,
            'uncertainty_ms': (after-before)*500,
            'bracket_ms': (after-before)*1000}


def clock_valid(before, after):
    if any(not math.isfinite(c[k]) for c in (before, after)
           for k in ('offset_ms', 'uncertainty_ms', 'bracket_ms')):
        return False
    return (all(0 <= c['uncertainty_ms'] <= MAX_BRACKET_MS/2 for c in (before, after))
            and 0 <= before['bracket_ms'] <= MAX_BRACKET_MS
            and 0 <= after['bracket_ms'] <= MAX_BRACKET_MS
            and abs(after['offset_ms']-before['offset_ms']) <= TOLERANCE_MS)


def startup_sample(start_epoch_ms, ready_epoch_ms, before, after):
    latency = ready_epoch_ms-start_epoch_ms
    valid = clock_valid(before, after) and math.isfinite(latency) and latency >= 0
    return {'value_ms': latency if valid else None, 'valid': valid,
            'start_epoch_ms': start_epoch_ms, 'ready_epoch_ms': ready_epoch_ms,
            'observed_delta_ms': latency if math.isfinite(latency) else None,
            'reason': None if valid else 'epoch/monotonic jump, imprecise correlation or invalid latency',
            'before': before, 'after': after}
