"""Bounded negative IPC cases against the owned native service only."""
import json
import socket
import time


def exercise(fixture):
    results = {}
    with socket.socket(socket.AF_UNIX) as client:
        client.settimeout(3)
        client.connect(str(fixture.socket))
        client.sendall(b'{"op":"snapshot"')
        # A partial frame must not prevent a second control connection.
        started = time.monotonic()
        fixture.request('snapshot')
        results['partial_frame_control_response_ms'] = (time.monotonic()-started)*1000
    with socket.socket(socket.AF_UNIX) as client:
        client.settimeout(3)
        client.connect(str(fixture.socket))
        client.sendall(b'{"op":"not_a_protocol_operation"}\n')
        with client.makefile('rb') as reader:
            reply = json.loads(reader.readline(65536))
            results['invalid_operation_rejected'] = reply['ok'] is False
    fixture.stop_child(fixture.service)
    results['service_restart_ms'] = fixture.start_service()
    results['unavailable'] = ['slow subscriber saturation', 'stalled provider (no real inference)',
                              'removed file / files view', 'read-only disk', 'incompatible client version']
    return results
