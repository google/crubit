#!/usr/bin/env python3
# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""Pre-generates C++ headers and sources for all Crubit Protobuf definitions.

Generating headers upfront before invoking Cargo avoids parallel build race
conditions.
For architectural details, see docs/overview/cargo_build_protobuf.md.
"""

import argparse
import os
import subprocess
import sys

SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
REPO_ROOT = os.path.abspath(os.path.join(SCRIPT_DIR, "..", ".."))

PROTO_FILES = [
    "rs_bindings_from_cc/ir.proto",
    "rs_bindings_from_cc/generate_bindings/generate_bindings.proto",
]


def fail(msg):
  """Prints an error message to stderr and exits with status 1."""
  print(f"Error: {msg}", file=sys.stderr)
  sys.exit(1)


def generate_protobuf_headers(protoc_bin, out_dir, repo_root=REPO_ROOT):
  """Invokes protoc to generate C++ headers and sources for PROTO_FILES."""
  os.makedirs(out_dir, exist_ok=True)
  proto_paths = [os.path.join(repo_root, p) for p in PROTO_FILES]
  missing = [p for p in proto_paths if not os.path.exists(p)]
  if missing:
    fail(f"Missing required proto files: {missing}")

  cmd = [
      protoc_bin,
      f"--cpp_out={out_dir}",
      f"-I={repo_root}",
  ] + proto_paths
  subprocess.check_call(cmd)
  print(f"Pre-generated Protobuf C++ headers in {out_dir}")


def main():
  parser = argparse.ArgumentParser(description=__doc__)
  parser.add_argument(
      "--protoc",
      default=os.environ.get("PROTOC", "protoc"),
      help="Path to protoc executable (defaults to PROTOC env var or 'protoc')",
  )
  parser.add_argument(
      "--out_dir",
      required=True,
      help="Directory where generated .pb.h and .pb.cc files will be written",
  )
  parser.add_argument(
      "--repo_root",
      default=REPO_ROOT,
      help="Root directory of Crubit repository",
  )
  args = parser.parse_args()
  generate_protobuf_headers(
      args.protoc,
      os.path.abspath(args.out_dir),
      os.path.abspath(args.repo_root),
  )


if __name__ == "__main__":
  main()
