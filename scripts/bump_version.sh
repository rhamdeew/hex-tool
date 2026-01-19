#!/usr/bin/env bash
set -euo pipefail

default_version="0.1.4"

read -r -p "Enter new app version [${default_version}]: " version
version="${version:-$default_version}"

NEW_VERSION="${version}" python3 - <<'PY'
import json
import os
import sys

version = os.environ.get("NEW_VERSION")
if not version:
    print("NEW_VERSION is not set", file=sys.stderr)
    sys.exit(1)

def bump(path):
    with open(path, "r", encoding="utf-8") as f:
        data = json.load(f)
    data["version"] = version
    with open(path, "w", encoding="utf-8") as f:
        json.dump(data, f, indent=2)
        f.write("\n")

bump("package.json")
bump("src-tauri/tauri.conf.json")
PY
