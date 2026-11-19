#!/usr/bin/env python3
# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""Locates prebuilt Bazel build artifacts and generates Cargo or CMake LLVM setup."""

import argparse
import functools
import json
import os
import pathlib
import re
import shlex
import stat
import subprocess
import sys
from typing import NoReturn

import get_llvm_commit_date

Path = pathlib.Path

SCRIPT_DIR = Path(__file__).resolve().parent
REPO_ROOT = SCRIPT_DIR.parents[1]


def log(msg):
  print(f"--- {msg}", flush=True)


def warn(msg):
  print(f"Warning: {msg}", file=sys.stderr)


def fail(msg: str) -> NoReturn:
  print(f"Error: {msg}", file=sys.stderr)
  sys.exit(1)


@functools.cache
def get_bazel_info():
  """Returns Bazel info mapping for output_base and bazel-bin."""
  try:
    output = subprocess.check_output(
        ["bazelisk", "info", "output_base", "bazel-bin"], text=True
    )
    return dict(
        line.split(": ", 1) for line in output.splitlines() if ": " in line
    )
  except (subprocess.SubprocessError, OSError) as e:
    fail(f"Failed to get Bazel info: {e}")


def get_bazel_external_dir():
  return Path(get_bazel_info()["output_base"]) / "external"


def get_bazel_bin_external_dir():
  return Path(get_bazel_info()["bazel-bin"]) / "external"


@functools.cache
def get_bzlmod_repo_mapping():
  """Returns mapping from repo name to its external directory name using bazel mod dump_repo_mapping."""
  try:
    raw_json = subprocess.check_output(
        ["bazelisk", "mod", "dump_repo_mapping", ""], text=True
    )
    return json.loads(raw_json)
  except (subprocess.SubprocessError, OSError, json.JSONDecodeError) as e:
    warn(f"Failed to dump repo mapping: {e}")
    return {}


def query_repo_src_dir_fallback(target_label):
  """Fallback that resolves the external repo directory for a target label using bazelisk query."""
  try:
    output = subprocess.check_output(
        ["bazelisk", "query", target_label, "--output=location"],
        text=True,
    )
    match = re.search(r"[/\\]external[/\\]([^/\\]+)", output)
    if match:
      return get_bazel_external_dir() / match.group(1)
    return None
  except (subprocess.SubprocessError, OSError) as e:
    warn(f"Failed to resolve repo dir for {target_label}: {e}")
    return None


def resolve_external_repo(repo_name, query_target, prereq_hint):
  """Resolves source and bazel-bin directories for an external repository."""
  dir_name = get_bzlmod_repo_mapping().get(repo_name)
  src_dir = (
      get_bazel_external_dir() / dir_name
      if dir_name
      else query_repo_src_dir_fallback(query_target)
  )
  if not src_dir or not src_dir.is_dir():
    fail(
        f"Could not find {repo_name} directory. Did you run 'bazelisk build"
        f" {prereq_hint}' first?"
    )
  log(f"Found {repo_name} source at: {src_dir}")

  bin_dir = get_bazel_bin_external_dir() / src_dir.name
  if not bin_dir.is_dir():
    fail(
        f"{repo_name} build output not found at {bin_dir}. Did you run"
        f" 'bazelisk build {prereq_hint}' first?"
    )
  return src_dir.resolve(), bin_dir.resolve()


def get_llvm_dev_date(llvm_src_dir: Path | None = None) -> str:
  """Retrieves the LLVM dev date from the environment, extracted repo, or commit."""
  commit = get_llvm_commit_date.get_llvm_commit()
  dev_date = get_llvm_commit_date.get_commit_date(commit, llvm_src_dir)
  log(f"Computed LLVM dev date: {dev_date}")
  return dev_date


def find_hermetic_toolchain():
  """Finds hermetic Clang/LLVM toolchain binaries and library directories."""
  external_dir = get_bazel_external_dir()
  if not external_dir.is_dir():
    return None

  for d in sorted(external_dir.glob("*llvm_toolchain*")):
    clang = d / "bin" / "clang"
    clang_xx = d / "bin" / "clang++"
    llvm_ar = d / "bin" / "llvm-ar"
    if clang.is_file() and clang_xx.is_file() and llvm_ar.is_file():
      lib_base = d / "lib"
      if lib_base.is_dir():
        target_libs = [
            sub
            for sub in lib_base.iterdir()
            if sub.is_dir()
            and any(k in sub.name for k in ("linux", "darwin", "apple"))
        ]
      else:
        target_libs = []
      lib_dir = target_libs[0] if target_libs else lib_base

      return {
          "clang": str(clang.absolute()),
          "clang_xx": str(clang_xx.absolute()),
          "llvm_ar": str(llvm_ar.absolute()),
          "lib_dir": str(lib_dir.absolute()) if lib_dir.is_dir() else None,
      }
  return None


def merge_archives(llvm_ar_path, output_archive, input_dirs):
  """Merges all .a static library files in input_dirs into output_archive using llvm-ar MRI script."""
  input_archives = []
  for d in input_dirs:
    dir_path = Path(d)
    if dir_path.is_dir():
      for p in dir_path.rglob("*.a"):
        input_archives.append(str(p.resolve()))
  input_archives.sort()

  if not input_archives:
    fail(
        f"No static archives (.a) found in input directories: {input_dirs}. "
        "If using remote execution/caching, ensure Bazel was run with "
        "--remote_download_outputs=all or "
        "--remote_download_regex='.*\\.(a|h|inc|def)$'."
    )

  out_path = Path(output_archive)
  manifest_path = Path(f"{output_archive}.inputs")

  if out_path.is_file() and manifest_path.is_file():
    try:
      if (
          manifest_path.read_text(encoding="utf-8").splitlines()
          == input_archives
      ):
        out_mtime = out_path.stat().st_mtime
        if all(
            Path(inp).stat().st_mtime <= out_mtime for inp in input_archives
        ):
          log(f"{output_archive} is up-to-date; skipping merge.")
          return
    except OSError:
      pass

  out_path.unlink(missing_ok=True)

  mri_script = "\n".join(
      [f"CREATE {output_archive}"]
      + [f"ADDLIB {lib}" for lib in input_archives]
      + ["SAVE", "END", ""]
  )

  log(f"Merging {len(input_archives)} archives into {output_archive}...")
  try:
    res = subprocess.run(
        [llvm_ar_path, "-M"],
        input=mri_script,
        text=True,
        capture_output=True,
        check=False,
    )
    if res.returncode != 0:
      fail(
          f"llvm-ar failed with exit code {res.returncode}\nStderr:"
          f" {res.stderr}"
      )
    manifest_path.write_text("\n".join(input_archives) + "\n", encoding="utf-8")
    log("Merge successful!")
  except (subprocess.SubprocessError, OSError) as e:
    fail(f"Failed to run llvm-ar: {e}")


def get_llvm_ar_version(llvm_ar_path: str) -> str:
  """Extracts the LLVM version string reported by llvm-ar --version."""
  try:
    output = subprocess.check_output([llvm_ar_path, "--version"], text=True)
  except (subprocess.SubprocessError, OSError) as e:
    fail(f"Failed to run {llvm_ar_path} --version: {e}")
  match = re.search(r"LLVM version ([^ \t\r\n]+)", output)
  if not match:
    fail(
        f"Could not parse LLVM version from {llvm_ar_path} --version:\n{output}"
    )
  return match.group(1)


def get_hermetic_link_args(toolchain: dict[str, str | None]) -> list[str]:
  """Returns linker flags needed to link against Bazel's hermetic libc++ LLVM."""
  args = []
  lib_dir = toolchain["lib_dir"]
  if lib_dir:
    log(f"Found hermetic Lib dir: {lib_dir}")
    args.append(f"-L{lib_dir}")
  args.extend([
      "-stdlib=libc++",
      "-lc++",
      "-lc++abi",
      "-lunwind",
      "-lzstd",
  ])
  if lib_dir:
    args.extend([
        f"-Wl,-rpath,{lib_dir}",
        "-Wl,--disable-new-dtags",
    ])
  return args


_LLVM_CONFIG_SHIM_TEMPLATE = r"""#!/bin/sh
# Generated by cargo/build/setup_bazel_env.py for cmake/rs_bindings_from_cc.
set -eu

case "$*" in
  --version) printf '%s\n' {version} ;;
  --bindir) printf '%s\n' {bindir} ;;
  --includedir) printf '%s\n' {includedir} ;;
  --libdir) printf '%s\n' {libdir} ;;
  --has-rtti) printf '%s\n' 'NO' ;;
  --cxxflags) printf '%s\n' '-stdlib=libc++' ;;
  '--system-libs --link-static'|'--link-static --system-libs'|'--system-libs')
    printf '%s\n' {system_libs} ;;
  *)
    printf 'Unsupported llvm-config arguments: %s\n' "$*" >&2
    exit 1 ;;
esac
"""


def write_cmake_llvm_root(
    cmake_llvm_root: Path,
    *,
    llvm_version: str,
    toolchain: dict[str, str | None],
    clang_include_paths: list[Path],
    lib_dir: Path,
    link_args: list[str],
    llvm_dev_date: str,
) -> None:
  """Stages a CRUBIT_LLVM_ROOT directory for cmake/rs_bindings_from_cc.

  cmake/rs_bindings_from_cc inspects prebuilt LLVM installs strictly through
  `<root>/bin/llvm-config`, whereas Bazel's `@llvm-project` build has no real
  `llvm-config` binary (its target is only a `testonly` `lit` stub) and splits
  its outputs across multiple directories (hermetic toolchain tools under
  `external/*llvm_toolchain*/bin`, headers across four source/generated trees,
  and the merged `libclang_monolithic.a` archive in `target/bazel_outputs`).

  Generates two files so CMake can exercise its normal `CRUBIT_LLVM_ROOT` path
  without Bazel-specific branches:
    * `bin/llvm-config`: a minimal `sh` shim reporting necessary flags.
    * `cmake-vars.sh`: exports crubit environment variables so callers can pass
    them to `cmake -D...` without leaking Cargo variables into `GITHUB_ENV`.

  Args:
    cmake_llvm_root: Target directory to stage `bin/llvm-config` and
      `cmake-vars.sh` into.
    llvm_version: LLVM version string reported by hermetic `llvm-ar --version`.
    toolchain: Hermetic toolchain paths dictionary from
      `find_hermetic_toolchain()`.
    clang_include_paths: List of LLVM/Clang source and generated header
      directories.
    lib_dir: Directory containing the merged `libclang_monolithic.a` archive.
    link_args: Linker flags for linking against Bazel's hermetic `libc++` LLVM.
    llvm_dev_date: LLVM commit date in `YYYYMMDD` format.
  """
  bin_dir = cmake_llvm_root / "bin"
  bin_dir.mkdir(parents=True, exist_ok=True)

  clang_path = toolchain["clang"]
  clang_xx_path = toolchain["clang_xx"]
  assert clang_path is not None and clang_xx_path is not None
  toolchain_bin_dir = str(Path(clang_path).parent)
  include_arg = ",".join(str(p) for p in clang_include_paths)
  system_libs = shlex.join(link_args)

  llvm_config_path = bin_dir / "llvm-config"
  llvm_config_script = _LLVM_CONFIG_SHIM_TEMPLATE.format(
      version=shlex.quote(llvm_version),
      bindir=shlex.quote(toolchain_bin_dir),
      includedir=shlex.quote(include_arg),
      libdir=shlex.quote(str(lib_dir)),
      system_libs=shlex.quote(system_libs),
  )
  llvm_config_path.write_text(llvm_config_script, encoding="utf-8")
  llvm_config_path.chmod(
      llvm_config_path.stat().st_mode
      | stat.S_IXUSR
      | stat.S_IXGRP
      | stat.S_IXOTH
  )
  log(f"Wrote CMake llvm-config shim to {llvm_config_path}")

  cmake_vars_path = cmake_llvm_root / "cmake-vars.sh"
  cmake_vars = {
      "CRUBIT_LLVM_ROOT": str(cmake_llvm_root),
      "CRUBIT_LLVM_DEV_DATE": llvm_dev_date,
      "CRUBIT_CMAKE_C_COMPILER": clang_path,
      "CRUBIT_CMAKE_CXX_COMPILER": clang_xx_path,
  }
  lines = ["# Generated by setup_bazel_env.py"] + [
      f"export {k}={shlex.quote(v)}" for k, v in cmake_vars.items()
  ]
  cmake_vars_path.write_text("\n".join(lines) + "\n", encoding="utf-8")
  log(f"Saved CMake variables to {cmake_vars_path}")


def write_env_config(bazel_outputs_dir, env_vars):
  """Writes bazel-env.sh for sourcing in Cargo build steps."""
  env_sh_path = Path(bazel_outputs_dir) / "bazel-env.sh"
  lines = ["# Generated by setup_bazel_env.py"] + [
      f"export {k}={shlex.quote(v)}" for k, v in env_vars.items()
  ]
  env_sh_path.write_text("\n".join(lines) + "\n", encoding="utf-8")

  log(f"Saved Bazel environment setup to {env_sh_path}")


def parse_args():
  """Parses command-line arguments."""
  parser = argparse.ArgumentParser(
      description=(
          "Locates prebuilt Bazel build artifacts and generates environment"
          " scripts for Cargo (bazel-env.sh) and optionally CMake"
          " (bin/llvm-config and cmake-vars.sh)."
      )
  )
  parser.add_argument(
      "--cmake-llvm-root",
      type=Path,
      default=None,
      help=(
          "If set, also stage a CRUBIT_LLVM_ROOT directory (bin/llvm-config and"
          " cmake-vars.sh) reusing Bazel's LLVM outputs alongside bazel-env.sh."
      ),
  )
  return parser.parse_args()


def main():
  args = parse_args()
  cmake_llvm_root = (
      args.cmake_llvm_root.resolve() if args.cmake_llvm_root else None
  )

  os.chdir(REPO_ROOT)

  # 1. Discover Bazel output directories
  external_dir = get_bazel_external_dir()
  bin_external_base = get_bazel_bin_external_dir()
  log(f"Using Bazel external dir: {external_dir}")
  log(f"Using Bazel bin external dir: {bin_external_base}")

  bazel_outputs_dir = (REPO_ROOT / "target" / "bazel_outputs").resolve()
  bazel_outputs_dir.mkdir(parents=True, exist_ok=True)

  # 2. LLVM/Clang paths
  llvm_src_dir, llvm_bin_dir = resolve_external_repo(
      "llvm-project",
      "@llvm-project//llvm:Support",
      "rs_bindings_from_cc:rs_bindings_from_cc_main",
  )
  clang_include_paths = [
      llvm_src_dir / "clang" / "include",
      llvm_src_dir / "llvm" / "include",
      llvm_bin_dir / "clang" / "include",
      llvm_bin_dir / "llvm" / "include",
  ]
  clang_lib_paths = [
      llvm_bin_dir / "clang",
      llvm_bin_dir / "llvm",
  ]

  # 3. Hermetic Toolchain
  toolchain = find_hermetic_toolchain()
  if not toolchain:
    fail("Hermetic toolchain or llvm-ar not found in toolchains_llvm.")

  # 4. Merge static libraries into monolithic archives
  archives_to_merge = [
      ("libclang_monolithic.a", clang_lib_paths),
  ]
  for name, input_dirs in archives_to_merge:
    merge_archives(
        toolchain["llvm_ar"],
        bazel_outputs_dir / name,
        input_dirs,
    )

  log(f"Found hermetic Clang: {toolchain['clang']}")
  llvm_dev_date = get_llvm_dev_date(llvm_src_dir)
  link_args = get_hermetic_link_args(toolchain)

  if cmake_llvm_root is not None:
    llvm_ar_version = get_llvm_ar_version(toolchain["llvm_ar"])
    write_cmake_llvm_root(
        cmake_llvm_root,
        llvm_version=llvm_ar_version,
        toolchain=toolchain,
        clang_include_paths=clang_include_paths,
        lib_dir=bazel_outputs_dir,
        link_args=link_args,
        llvm_dev_date=llvm_dev_date,
    )

  # 5. Environment variables
  env_vars = {
      "CLANG_INCLUDE_PATH": ",".join(str(p) for p in clang_include_paths),
      "CLANG_LIB_STATIC_PATH": str(bazel_outputs_dir),
      "CC": toolchain["clang"],
      "CXX": toolchain["clang_xx"],
      "CXXFLAGS": f"-stdlib=libc++ -DCRUBIT_LLVM_DEV_DATE={llvm_dev_date}",
  }

  hermetic_clang_dir = Path(toolchain["clang"]).parent
  rustflags = [
      f"-C linker={toolchain['clang']}",
      f"-C link-arg=-fuse-ld={hermetic_clang_dir / 'ld.lld'}",
      *(f"-C link-arg={arg}" for arg in link_args),
  ]

  env_vars["RUSTFLAGS"] = " ".join(rustflags)

  write_env_config(bazel_outputs_dir, env_vars)

  env_sh_rel = (bazel_outputs_dir / "bazel-env.sh").relative_to(REPO_ROOT)
  print(f"\nTo configure your environment, run:\nsource {env_sh_rel}")


if __name__ == "__main__":
  main()
