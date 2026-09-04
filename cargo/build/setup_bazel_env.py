#!/usr/bin/env python3
# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""Locates prebuilt Bazel build artifacts and generates environment variables for Cargo build."""

import functools
import json
import os
import pathlib
import re
import shlex
import subprocess
import sys
from typing import NoReturn

import generate_proto_headers

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
        "If using remote caching, ensure Bazel was run with"
        " --remote_download_outputs=all."
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


def write_env_config(bazel_outputs_dir, env_vars):
  """Writes bazel-env.sh and updates GITHUB_ENV if running in GitHub Actions."""
  env_sh_path = Path(bazel_outputs_dir) / "bazel-env.sh"
  lines = ["# Generated by setup_bazel_env.py"] + [
      f"export {k}={shlex.quote(v)}" for k, v in env_vars.items()
  ]
  env_sh_path.write_text("\n".join(lines) + "\n", encoding="utf-8")

  log(f"Saved Bazel environment setup to {env_sh_path}")

  if github_env := os.environ.get("GITHUB_ENV"):
    with open(github_env, "a", encoding="utf-8") as f:
      f.writelines(f"{k}={v}\n" for k, v in env_vars.items())
    log(
        "Saved environment variables directly to GitHub Actions environment:"
        f" {github_env}"
    )


def main():
  os.chdir(REPO_ROOT)

  # 1. Discover Bazel output directories
  external_dir = get_bazel_external_dir()
  bin_external_base = get_bazel_bin_external_dir()
  log(f"Using Bazel external dir: {external_dir}")
  log(f"Using Bazel bin external dir: {bin_external_base}")

  bazel_outputs_dir = REPO_ROOT / "target" / "bazel_outputs"
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

  # 3. Abseil paths
  absl_src_dir, absl_bin_dir = resolve_external_repo(
      "abseil-cpp",
      "@abseil-cpp//absl/base:base",
      "rs_bindings_from_cc:rs_bindings_from_cc_main",
  )
  absl_include_paths = [absl_src_dir]
  absl_lib_base = absl_bin_dir / "absl"

  # 4. Protobuf paths
  protobuf_src_dir, protobuf_bin_dir = resolve_external_repo(
      "protobuf",
      "@protobuf//:protobuf",
      "@protobuf//:protoc",
  )
  protoc_path = protobuf_bin_dir / "protoc"
  if not protoc_path.is_file():
    fail(
        f"protoc not found at expected path: {protoc_path}. Did you run"
        " 'bazelisk build @protobuf//:protoc' first?"
    )

  # 5. Pre-generate proto headers upfront to avoid race conditions in parallel
  # Cargo builds. For details, see docs/overview/cargo_build_protobuf.md.
  proto_include_dir = bazel_outputs_dir / "include"
  log(f"Pre-generating C++ Protobuf headers into {proto_include_dir}...")
  generate_proto_headers.generate_protobuf_headers(
      str(protoc_path), str(proto_include_dir), str(REPO_ROOT)
  )

  # 6. Hermetic Toolchain
  toolchain = find_hermetic_toolchain()
  if not toolchain:
    fail("Hermetic toolchain or llvm-ar not found in toolchains_llvm.")

  # 7. Merge static libraries into monolithic archives
  archives_to_merge = [
      ("libabsl_monolithic.a", [absl_lib_base]),
      ("libclang_monolithic.a", clang_lib_paths),
      ("libprotobuf.a", [protobuf_bin_dir]),
  ]
  for name, input_dirs in archives_to_merge:
    merge_archives(
        toolchain["llvm_ar"],
        bazel_outputs_dir / name,
        input_dirs,
    )

  # 8. Environment variables
  env_vars = {
      "CLANG_INCLUDE_PATH": ",".join(str(p) for p in clang_include_paths),
      "CLANG_LIB_STATIC_PATH": str(bazel_outputs_dir),
      "ABSL_INCLUDE_PATH": ",".join(str(p) for p in absl_include_paths),
      "ABSL_LIB_STATIC_PATH": str(bazel_outputs_dir),
      # PROTOC is required by protobuf-codegen during Cargo build to generate
      # Rust Protobuf bindings (for ir_rust_proto and
      # generate_bindings_rust_proto).
      # For details, see docs/overview/cargo_build_protobuf.md.
      "PROTOC": str(protoc_path),
      "PROTOBUF_INCLUDE_PATH": ",".join(
          str(p)
          for p in [
              proto_include_dir,
              protobuf_src_dir / "src",
              protobuf_src_dir / "third_party" / "utf8_range",
          ]
      ),
      "PROTOBUF_LIB_STATIC_PATH": str(bazel_outputs_dir),
  }

  log(f"Found hermetic Clang: {toolchain['clang']}")
  env_vars["CC"] = toolchain["clang"]
  env_vars["CXX"] = toolchain["clang_xx"]
  env_vars["CXXFLAGS"] = "-stdlib=libc++"

  hermetic_clang_dir = Path(toolchain["clang"]).parent
  rustflags = [
      f"-C linker={toolchain['clang']}",
      f"-C link-arg=-fuse-ld={hermetic_clang_dir / 'ld.lld'}",
      "-C link-arg=-stdlib=libc++",
      "-C link-arg=-lc++",
      "-C link-arg=-lc++abi",
      "-C link-arg=-lunwind",
      "-C link-arg=-lzstd",
  ]
  if lib_dir := toolchain["lib_dir"]:
    log(f"Found hermetic Lib dir: {lib_dir}")
    rustflags.extend([
        f"-C link-arg=-L{lib_dir}",
        f"-C link-arg=-Wl,-rpath,{lib_dir}",
        "-C link-arg=-Wl,--disable-new-dtags",
    ])

  env_vars["RUSTFLAGS"] = " ".join(rustflags)

  write_env_config(bazel_outputs_dir, env_vars)

  env_sh_rel = (bazel_outputs_dir / "bazel-env.sh").relative_to(REPO_ROOT)
  print(f"\nTo configure your environment, run:\nsource {env_sh_rel}")


if __name__ == "__main__":
  main()
