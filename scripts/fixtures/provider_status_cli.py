#!/usr/bin/env python3
"""Controlled CLI double. It never authenticates, calls a model or edits projects."""
import json
import os
import subprocess
import sys
import tomllib

if sys.argv[1:3] == ["login", "status"]:
    sys.exit(1)

hooks = {}
args = iter(sys.argv[1:])
for arg in args:
    if arg == "-c":
        hooks.update(tomllib.loads(next(args))["hooks"])
    elif arg == "--settings":
        hooks.update(json.loads(next(args))["hooks"])


def send(name, **extra):
    payload = {
        "session_id": "fixture-" + os.environ["EROS_HOOK_CONVERSATION"],
        "hook_event_name": name,
        "turn_id": "fixture-turn",
        "tool_name": "Bash",
        "tool_use_id": "fixture-call",
        **extra,
    }
    for group in hooks.get(name, []):
        for handler in group["hooks"]:
            result = subprocess.run(
                handler["command"], shell=True, input=json.dumps(payload),
                text=True, capture_output=True, timeout=3,
            )
            if result.returncode != 0 or json.loads(result.stdout) != {}:
                raise RuntimeError("Observer must not return a provider decision")


send("SessionStart", source="startup")
print("controlled provider ready", flush=True)
for line in sys.stdin:
    name = line.strip()
    if name == "work":
        send("UserPromptSubmit", prompt="fixture prompt excluded from IPC")
    elif name == "decision":
        send("PermissionRequest")
    elif name == "answer":
        send("PostToolUse", tool_response="fixture result excluded from IPC")
    elif name == "done":
        send("Stop", last_assistant_message="fixture answer excluded from IPC")
    elif name == "cancel":
        send("Interrupt" if "Interrupt" in hooks else "StopFailure")
    elif name == "quit":
        send("SessionEnd")
        break
    else:
        raise RuntimeError("Unexpected controlled test command")
    print("controlled event sent", flush=True)
