#!/usr/bin/env python3
"""Bounded native PTY/IPC/hook test using CLI doubles; zero provider inference."""
import argparse
import json
import os
from pathlib import Path
import shlex
import socket
import subprocess
import tempfile
import time


def send(stream, request):
    stream.sendall(json.dumps(request).encode() + b"\n")


def reply(reader):
    result = json.loads(reader.readline())
    assert result["ok"], "Controlled service request failed"
    return result["data"]


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--binary", type=Path, default=Path("target/debug/eros-desktop"))
    args = parser.parse_args()
    binary = args.binary.resolve()
    fixture = Path(__file__).parent.resolve() / "fixtures/provider_status_cli.py"
    with tempfile.TemporaryDirectory(prefix="eros-status-native-") as temp:
        root = Path(temp)
        commands = root / "bin"
        commands.mkdir()
        for provider in ["codex", "claude"]:
            command = commands / provider
            command.write_text(f'#!/bin/sh\nexec python3 {shlex.quote(str(fixture))} "$@"\n')
            command.chmod(0o700)
        shell = commands / "test-shell"
        shell.write_text('#!/bin/sh\nexec /bin/bash --noprofile --norc "$@"\n')
        shell.chmod(0o700)
        env = dict(os.environ, EROS_DATA_DIR=str(root / "data"),
                   EROS_RUNTIME_DIR=str(root / "runtime"), SHELL=str(shell),
                   PATH=str(commands) + os.pathsep + os.environ["PATH"])
        service = subprocess.Popen([str(binary), "--eros-service"], env=env,
                                   stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
        socket_path = root / "runtime/service.sock"
        activity = None
        conversations = []
        try:
            deadline = time.monotonic() + 5
            while not socket_path.exists():
                assert time.monotonic() < deadline, "Controlled service failed to start"
                time.sleep(0.02)

            def request(value):
                with socket.socket(socket.AF_UNIX) as stream:
                    stream.settimeout(5)
                    stream.connect(str(socket_path))
                    send(stream, value)
                    return reply(stream.makefile("rb"))

            project = request({"op": "add_project", "path": str(root)})
            activity = socket.socket(socket.AF_UNIX)
            activity.settimeout(5)
            activity.connect(str(socket_path))
            send(activity, {"op": "subscribe_activity"})
            reader = activity.makefile("rb")
            reply(reader)

            def wait_status(conversation_id, expected):
                for _ in range(16):
                    event = json.loads(reader.readline())
                    if event["conversation_id"] == conversation_id and event["chat_status"] == expected:
                        return event
                raise AssertionError("Expected activity event was not delivered")

            def write(conversation_id, text):
                request({"op": "write", "conversation_id": conversation_id, "text": text + "\r"})

            for kind, project_id in [("codex", None), ("claude", project["id"])]:
                conversation_id = "fixture-" + kind
                request({"op": "create_conversation", "id": conversation_id,
                         "project_id": project_id, "name": "Controlled " + kind,
                         "cwd": str(root), "kind": kind})
                conversations.append(conversation_id)
                wait_status(conversation_id, "normal")
                write(conversation_id, "work")
                wait_status(conversation_id, "working")
                write(conversation_id, "decision")
                decision = wait_status(conversation_id, "awaiting_decision")
                assert request({"op": "mark_conversation_read", "conversation_id": conversation_id,
                                "activity_revision": decision["activity_revision"]})["chat_status"] == "awaiting_decision"
                write(conversation_id, "answer")
                wait_status(conversation_id, "working")
                write(conversation_id, "done")
                done = wait_status(conversation_id, "completed_unread")
                request({"op": "set_active_scope", "scope": "conversation:" + conversation_id})
                snapshot = request({"op": "snapshot"})
                chat = next(c for c in snapshot["conversations"] if c["id"] == conversation_id)
                assert chat["state"] == "running" and chat["chat_status"] == "completed_unread"
                assert chat["provider_thread_id"] == "fixture-" + conversation_id
                request({"op": "mark_conversation_read", "conversation_id": conversation_id,
                         "activity_revision": done["activity_revision"]})
                wait_status(conversation_id, "normal")
                write(conversation_id, "work")
                wait_status(conversation_id, "working")
                write(conversation_id, "cancel")
                wait_status(conversation_id, "normal")

            # A forged callback cannot update another chat.
            with socket.socket(socket.AF_UNIX) as stream:
                stream.settimeout(5)
                stream.connect(str(socket_path))
                send(stream, {"op": "provider_hook", "conversation_id": conversations[0],
                              "token": "wrong-token", "event": {"session_id": "fixture-session", "hook_event_name": "Stop"}})
                assert not json.loads(stream.makefile("rb").readline())["ok"]
            for conversation_id in conversations:
                request({"op": "stop", "conversation_id": conversation_id})
            print("PASS: controlled Codex/Claude CLI doubles → real hook binary → IPC → Rust/SQLite → global activity stream; decisions, completion/read, cancellation and callback rejection. Zero provider calls.")
        finally:
            for conversation_id in conversations:
                try:
                    request({"op": "stop", "conversation_id": conversation_id})
                except Exception:
                    pass
            if activity is not None:
                activity.close()
            service.terminate()
            try:
                service.wait(timeout=3)
            except subprocess.TimeoutExpired:
                service.kill()
                service.wait()


if __name__ == "__main__":
    main()
