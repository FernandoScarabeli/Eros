#!/usr/bin/env python3
"""Check public repo boundaries and local Markdown links; no third-party packages."""
from pathlib import Path
import re
import subprocess
import sys
from urllib.parse import unquote, urlsplit


ROOT = Path(__file__).resolve().parent.parent
REQUIRED = [
    "README.md", "README.pt-BR.md", "LICENSE", "CONTRIBUTING.md",
    "CODE_OF_CONDUCT.md", "SECURITY.md", "ROADMAP.md", "AGENTS.md",
    ".gitignore", ".editorconfig", ".gitattributes", "docs/product.md",
    "docs/architecture.md", "docs/publishing.md", ".github/workflows/repository.yml",
    ".github/pull_request_template.md", ".github/ISSUE_TEMPLATE/bug_report.yml",
    ".github/ISSUE_TEMPLATE/feature_request.yml",
]
FORBIDDEN_ROOTS = {"legado", ".agents", ".codex", ".aws", ".runtime", ".data"}


def git(*args):
    return subprocess.run(["git", "-C", str(ROOT), *args], check=True, capture_output=True, text=True).stdout


def main():
    failures = []
    for name in REQUIRED:
        if not (ROOT / name).is_file():
            failures.append(f"Missing required file: {name}")
    try:
        tracked = git("ls-files", "-z").split("\0")
        candidates = git("ls-files", "--cached", "--others", "--exclude-standard", "-z").split("\0")
        for name in tracked:
            if not name:
                continue
            path = Path(name)
            if path.parts[0] in FORBIDDEN_ROOTS or path.name == "auth.json" or (path.name.startswith(".env") and not path.name.endswith(".example")):
                failures.append(f"Local/private path is tracked: {name}")
        for name in ["legado/", "legado/nexo/README.md", "legado/terminal-lab/README.md", ".codex/auth.json", ".aws/credentials", ".env.local"]:
            ignored = subprocess.run(["git", "-C", str(ROOT), "check-ignore", "--no-index", "-q", name]).returncode
            if ignored != 0:
                failures.append(f"Expected Git ignore rule for: {name}")
    except subprocess.CalledProcessError:
        print("Run this check inside the initialized Eros Git repository.", file=sys.stderr)
        return 1

    documents = sorted({name for name in candidates if name.endswith(".md")})
    for name in documents:
        source = ROOT / name
        if source.is_symlink():
            failures.append(f"Documentation must not be a symlink: {name}")
            continue
        body = source.read_text(encoding="utf-8")
        if re.search(r"(?<!\S)/(?:home|Users)/[^/\s]+", body):
            failures.append(f"Private machine path in public documentation: {name}")
        # These docs use ordinary inline Markdown links; remote links aren't fetched.
        for target in re.findall(r"\]\(([^)]+)\)", body):
            target = target.strip().strip("<>")
            parsed = urlsplit(target)
            if parsed.scheme or parsed.netloc or not parsed.path:
                continue
            destination = (source.parent / unquote(parsed.path)).resolve()
            if not destination.is_relative_to(ROOT) or destination.relative_to(ROOT).parts[:1] == ("legado",):
                failures.append(f"Public doc depends on an external/local archive path: {name} -> {target}")
            elif not destination.exists():
                failures.append(f"Broken local link: {name} -> {target}")

    if failures:
        print("\n".join(failures), file=sys.stderr)
        return 1
    print(f"Repository foundation passed: {len(REQUIRED)} required files, {len(documents)} Markdown documents, local/private paths excluded.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
