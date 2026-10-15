"""Owned fixture lifecycle, bounded IPC and synthetic catalog."""
import ctypes
from contextlib import closing
import json
import os
from pathlib import Path
import signal
import shutil
import socket
import sqlite3
import subprocess
import tempfile
import time

from metrics import process_stat

MAX_FRAME = 2 * 1024 * 1024

class EnvironmentUnavailable(RuntimeError):
    pass

class IpcFailure(RuntimeError):
    def __init__(self, operation, reason, received_bytes):
        self.details = {'phase': operation, 'reason': reason,
                        'received_bytes': received_bytes, 'limit_bytes': MAX_FRAME}
        super().__init__(f'{operation}: {reason}; received {received_bytes} bytes; limit {MAX_FRAME}')


class Fixture:
    def __init__(self, binary):
        self.binary = str(Path(binary).resolve())
        # Linux subreaper keeps double-fork/PTY descendants owned after root failure.
        libc = ctypes.CDLL(None, use_errno=True)
        if libc.prctl(36, 1, 0, 0, 0) != 0:  # PR_SET_CHILD_SUBREAPER
            raise OSError(ctypes.get_errno(), 'Cannot enable fixture subreaper')
        self.existing_children = set()
        for entry in Path('/proc').iterdir():
            if entry.name.isdigit():
                try:
                    stat = process_stat(int(entry.name))
                    if stat['parent'] == os.getpid():
                        self.existing_children.add((int(entry.name), stat['start']))
                except (OSError, ValueError, IndexError):
                    pass
        self.root = Path(tempfile.mkdtemp(prefix='eros-perf-'))
        self.children = []
        self.identities = {}
        self.service = None
        self.ui = None
        self.socket = self.root/'runtime/service.sock'
        commands = self.root/'bin'
        commands.mkdir()
        for name in ('codex', 'claude'):
            p = commands/name
            p.write_text('#!/bin/sh\nif [ "$1" = login ]; then exit 1; fi\n'
                         'if [ "$EROS_BENCH_SHELL_CHATS" = 1 ]; then exec /bin/bash --noprofile --norc; fi\nexit 1\n')
            p.chmod(0o700)
        shell = commands/'shell'
        shell.write_text('#!/bin/sh\nexec /bin/bash --noprofile --norc "$@"\n')
        shell.chmod(0o700)
        home = self.root/'home'
        home.mkdir()
        self.env = {key: os.environ[key] for key in
                    ('DISPLAY', 'WAYLAND_DISPLAY', 'XDG_RUNTIME_DIR', 'DBUS_SESSION_BUS_ADDRESS', 'LANG') if key in os.environ}
        self.env.update(HOME=str(home), XDG_CONFIG_HOME=str(home/'config'),
                        EROS_DATA_DIR=str(self.root/'data'), EROS_RUNTIME_DIR=str(self.root/'runtime'),
                        SHELL=str(shell), PATH=str(commands)+':/usr/bin:/bin')

    def remember(self):
        """Remember PID+starttime before parents can exit/reparent children."""
        roots = {p.pid for p in self.children if p.poll() is None}
        stats = {}
        for entry in Path('/proc').iterdir():
            if entry.name.isdigit():
                try:
                    stats[int(entry.name)] = process_stat(int(entry.name))
                except (OSError, ValueError, IndexError):
                    pass
        roots |= {pid for pid, stat in stats.items() if stat['parent'] == os.getpid()
                  and (pid, stat['start']) not in self.existing_children}
        owned = roots | {pid for pid, start in self.identities.items()
                         if pid in stats and stats[pid]['start'] == start}
        changed = True
        while changed:
            new = {pid for pid, stat in stats.items() if stat['parent'] in owned}
            changed = bool(new-owned)
            owned |= new
        for pid in owned:
            if pid in stats:
                self.identities[pid] = stats[pid]['start']

    def spawn(self, args, env=None):
        child = subprocess.Popen(args, env=env or self.env, start_new_session=True,
                                 stdin=subprocess.DEVNULL, stdout=subprocess.DEVNULL,
                                 stderr=subprocess.DEVNULL)
        self.children.append(child)
        self.remember()
        return child

    def request(self, op, **kwargs):
        with socket.socket(socket.AF_UNIX) as stream:
            stream.settimeout(10)
            stream.connect(str(self.socket))
            stream.sendall(json.dumps(dict(op=op, **kwargs)).encode()+b'\n')
            with stream.makefile('rb') as reader:
                raw = reader.readline(MAX_FRAME+2)
                payload = raw[:-1] if raw.endswith(b'\n') else raw
                if len(payload) > MAX_FRAME:
                    raise IpcFailure(op, 'frame_exceeds_limit', len(payload))
                if not raw:
                    raise IpcFailure(op, 'empty_reply', 0)
                if not raw.endswith(b'\n'):
                    raise IpcFailure(op, 'incomplete_frame', len(payload))
                reply = json.loads(payload)
                if not reply['ok']:
                    raise RuntimeError(reply['error'])
                return reply['data']

    def start_service(self):
        probe_path = self.root/'socket-permission-probe'
        try:
            with socket.socket(socket.AF_UNIX) as probe:
                probe.bind(str(probe_path))
        except PermissionError as error:
            raise EnvironmentUnavailable(f'Unix socket bind denied: errno {error.errno}') from error
        finally:
            probe_path.unlink(missing_ok=True)
        started = time.monotonic()
        self.service = self.spawn([self.binary, '--eros-service'])
        deadline = started+10
        while time.monotonic() < deadline:
            if self.service.poll() is not None:
                raise RuntimeError(f'Isolated service exited ({self.service.returncode}); Unix socket/PTY permissions required')
            if self.socket.exists():
                try:
                    self.request('set_active_scope', scope='chats')
                    return (time.monotonic()-started)*1000
                except (OSError, ValueError):
                    pass
            time.sleep(.02)
        raise TimeoutError('Service startup exceeded 10 seconds')

    def seed(self, chats=10000, projects=100):
        # Service is stopped: no live data / concurrent store writes.
        with closing(sqlite3.connect(self.root/'data/eros.sqlite3')) as db, db:
            for index in range(projects):
                folder = self.root/f'project-{index:03d}'
                folder.mkdir(exist_ok=True)
                db.execute('INSERT INTO projects(id,name,path) VALUES (?,?,?)',
                           (f'p{index}', f'Synthetic {index:03d}', str(folder)))
            db.executemany('INSERT INTO conversations(id,project_id,name,cwd,kind,state) VALUES (?,?,?,?,?,?)',
                           [(f'c{i}', f'p{i%projects}', f'Synthetic chat {i:05d}', str(self.root), 'codex', 'stopped') for i in range(chats)])

    def create_shell_chats(self):
        self.env['EROS_BENCH_SHELL_CHATS'] = '1'
        # Service inherited env on startup: caller sets this flag before start_service.
        for i in range(8):
            self.request('create_conversation', id=f'bench-chat-{i}', project_id=None,
                         name=f'Synthetic shell chat {i}', cwd=str(self.root), kind='codex')
        self.request('set_active_scope', scope='conversation:bench-chat-0')

    def catalog_size(self):
        # Diagnostic serialization of persisted rows, not a substitute snapshot/transport.
        with closing(sqlite3.connect(self.root/'data/eros.sqlite3')) as db, db:
            db.row_factory = sqlite3.Row
            chats = [dict(row) for row in db.execute(
                'SELECT id,project_id,name,cwd,kind,state,provider_thread_id,pinned,chat_status,activity_revision FROM conversations')]
            for chat in chats:
                chat['pinned'] = bool(chat['pinned'])
            projects = db.execute('SELECT count(*) FROM projects').fetchone()[0]
        return {'projects': projects, 'chats': len(chats),
                'conversations_json_bytes': len(json.dumps(chats, separators=(',', ':'), ensure_ascii=False).encode()),
                'limit_bytes': MAX_FRAME, 'size_kind': 'conversation-array-only; full snapshot is larger'}

    def stop_child(self, child):
        self.remember()
        if child.poll() is None:
            os.killpg(child.pid, signal.SIGTERM)
            try:
                child.wait(timeout=3)
            except subprocess.TimeoutExpired:
                os.killpg(child.pid, signal.SIGKILL)
                child.wait(timeout=3)

    def cleanup(self):
        self.remember()
        # PTYs create their own sessions, so root process groups alone are insufficient.
        for sig in (signal.SIGTERM, signal.SIGKILL):
            self.remember()  # Adopt children created/reparented while TERM handlers ran.
            for pid, start in reversed(list(self.identities.items())):
                try:
                    if process_stat(pid)['start'] == start:
                        os.kill(pid, sig)
                except (ProcessLookupError, FileNotFoundError):
                    pass
            if sig == signal.SIGTERM:
                time.sleep(.2)
        for child in self.children:
            child.wait(timeout=3)
        reaped = set()
        deadline = time.monotonic()+3
        while time.monotonic() < deadline:
            self.remember()
            for pid, start in self.identities.items():
                if pid in reaped:
                    continue
                try:
                    if process_stat(pid)['start'] != start:
                        reaped.add(pid)
                        continue
                    os.kill(pid, signal.SIGKILL)
                    waited, _ = os.waitpid(pid, os.WNOHANG)
                    if waited:
                        reaped.add(pid)
                except (ProcessLookupError, FileNotFoundError):
                    reaped.add(pid)
                except ChildProcessError:
                    # Wait for final adoption if a parent died during the previous sweep.
                    pass
            if len(reaped) == len(self.identities):
                break
            time.sleep(.01)
        survivors = []
        for pid, start in self.identities.items():
            try:
                stat = process_stat(pid)
                # Reaped by init can lag: zombies consume no runtime resources.
                raw = Path(f'/proc/{pid}/stat').read_text().split(')')[-1].split()
                if stat['start'] == start and raw[0] != 'Z':
                    survivors.append(pid)
            except OSError:
                pass
        if survivors:
            raise RuntimeError('Fixture processes survived cleanup; temporary directory retained')
        shutil.rmtree(self.root)
        return {'live_survivors': len(survivors), 'owned_processes': len(self.identities)}
