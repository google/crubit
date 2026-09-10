#!/usr/bin/env python3
# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""Queries Gitiles for an LLVM commit timestamp and outputs it in YYYYmmDD format (UTC)."""

import base64
import datetime
import pathlib
import re
import sys
import urllib.error
import urllib.request


def get_commit_date_from_gitiles(commit: str) -> str:
  """Fetches the commit date from Gitiles and returns YYYYmmDD in UTC."""
  url = f"https://llvm.googlesource.com/llvm-project/+/{commit}?format=TEXT"
  req = urllib.request.Request(
      url, headers={"User-Agent": "Crubit-LLVM-Date/1.0"}
  )
  try:
    with urllib.request.urlopen(req) as resp:
      data = base64.b64decode(resp.read()).decode("utf-8", errors="replace")
  except urllib.error.URLError as e:
    print(f"Error querying Gitiles for commit {commit}: {e}", file=sys.stderr)
    sys.exit(1)

  match = re.search(r"^committer .*? (\d+) [+-]\d+", data, re.MULTILINE)
  if not match:
    print(
        "Error: Could not parse committer line from Gitiles response for"
        f" {commit}",
        file=sys.stderr,
    )
    sys.exit(1)

  timestamp = int(match.group(1))
  utc_dt = datetime.datetime.fromtimestamp(timestamp, datetime.timezone.utc)
  return utc_dt.strftime("%Y%m%d")


def get_llvm_commit_from_module_bazel() -> str:
  """Extracts the LLVM commit from MODULE.bazel."""
  module_bazel_path = (
      pathlib.Path(__file__).resolve().parents[2] / "MODULE.bazel"
  )
  content = module_bazel_path.read_text(encoding="utf-8")
  match = re.search(
      r'llvm_ext\.configure\s*\(\s*commit\s*=\s*["\']([0-9a-fA-F]+)["\']',
      content,
  )
  if not match:
    print(
        "Error: Could not find llvm_ext.configure(commit = ...) in"
        f" {module_bazel_path}",
        file=sys.stderr,
    )
    sys.exit(1)
  return match.group(1)


def main():
  if len(sys.argv) != 2:
    print(f"Usage: {sys.argv[0]} <commit_hash>", file=sys.stderr)
    sys.exit(1)

  commit = sys.argv[1].strip()
  dev_date = get_commit_date_from_gitiles(commit)
  print(dev_date)


if __name__ == "__main__":
  main()
