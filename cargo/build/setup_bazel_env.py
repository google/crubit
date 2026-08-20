#!/usr/bin/env python3
# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""Locates prebuilt Bazel build artifacts and generates environment variables for Cargo build."""

import json
import os
import subprocess
import sys

from generate_proto_headers import generate_protobuf_headers

SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
REPO_ROOT = os.path.abspath(os.path.join(SCRIPT_DIR, "..", ".."))


def log(msg):
    print(f"--- {msg}", flush=True)


def fail(msg):
    print(f"Error: {msg}", file=sys.stderr)
    sys.exit(1)


def get_bzlmod_repo_mapping():
    """Returns mapping from repo name to its external directory name using bazel mod dump_repo_mapping."""
    try:
        raw_json = subprocess.check_output(
            ["bazelisk", "mod", "dump_repo_mapping", ""], text=True
        )
        return json.loads(raw_json)
    except Exception as e:
        log(f"Warning: Failed to dump repo mapping: {e}")
        return {}


def resolve_repo_dir(target_label):
    """Resolves the external repo directory for a target label using bazelisk query."""
    try:
        loc = subprocess.check_output(
            ["bazelisk", "query", target_label, "--output=location"],
            text=True,
        ).splitlines()[0]
        # Bazel location output is in the form: "<path>:<line>:<col>: rule ..."
        # On Windows, <path> includes a drive letter (e.g. C:\...), so split from the right.
        parts = loc.split(": rule ")[0].rsplit(":", 2)
        path = parts[0]
        norm_path = path.replace("\\", "/")
        external_idx = norm_path.find("/external/")
        if external_idx == -1:
            return None
        after_external = norm_path[external_idx + len("/external/") :]
        repo_name = after_external.split("/")[0]
        external_dir = path[: external_idx + len("/external")]
        return os.path.join(external_dir, repo_name)
    except Exception as e:
        print(
            f"Warning: Failed to resolve repo dir for {target_label}: {e}",
            file=sys.stderr,
        )
        return None


def find_llvm_generated_headers(bin_external_base, resolved_llvm_dirname):
    """Finds LLVM generated header directory inside bazel-bin."""
    candidate = os.path.join(bin_external_base, resolved_llvm_dirname)
    if os.path.isdir(candidate):
        log(f"Found LLVM generated headers at: {candidate}")
        return candidate
    return None


def find_hermetic_toolchain(external_dir):
    """Finds hermetic Clang/LLVM toolchain binaries and library directories."""
    if not os.path.isdir(external_dir):
        return None

    for d in os.listdir(external_dir):
        if "llvm_toolchain" in d:
            full_path = os.path.join(external_dir, d)
            if os.path.isdir(full_path):
                clang_path = os.path.join(full_path, "bin", "clang")
                clang_xx_path = os.path.join(full_path, "bin", "clang++")
                ar_path = os.path.join(full_path, "bin", "llvm-ar")
                if (
                    os.path.exists(clang_path)
                    and os.path.exists(clang_xx_path)
                    and os.path.exists(ar_path)
                ):
                    lib_base = os.path.join(full_path, "lib")
                    hermetic_lib_dir = None
                    if os.path.isdir(lib_base):
                        for sub in os.listdir(lib_base):
                            sub_path = os.path.join(lib_base, sub)
                            if os.path.isdir(sub_path) and (
                                "linux" in sub or "darwin" in sub or "apple" in sub
                            ):
                                hermetic_lib_dir = os.path.abspath(sub_path)
                                break
                        if not hermetic_lib_dir:
                            hermetic_lib_dir = os.path.abspath(lib_base)

                    return {
                        "clang": os.path.abspath(clang_path),
                        "clang_xx": os.path.abspath(clang_xx_path),
                        "llvm_ar": os.path.abspath(ar_path),
                        "lib_dir": hermetic_lib_dir,
                    }
    return None


def merge_archives(llvm_ar_path, output_archive, input_dirs):
    """Merges all .a static library files and .o/.pic.o object files in input_dirs into output_archive using llvm-ar MRI script."""
    input_archives = []
    input_objects = []
    for d in input_dirs:
        if os.path.exists(d):
            for root, _, files in os.walk(d):
                for f in files:
                    if f.endswith(".a"):
                        input_archives.append(os.path.abspath(os.path.join(root, f)))
                    elif f.endswith(".pic.o") or f.endswith(".o"):
                        input_objects.append(os.path.abspath(os.path.join(root, f)))

    input_archives.sort()
    input_objects.sort()
    all_inputs = input_archives + input_objects

    if not all_inputs:
        fail(
            f"No static archives (.a) or object files (.o) found in input directories: {input_dirs}"
        )

    manifest_path = output_archive + ".inputs"
    up_to_date = False
    if os.path.exists(output_archive) and os.path.exists(manifest_path):
        try:
            with open(manifest_path, "r") as f:
                saved_inputs = f.read().splitlines()
            if saved_inputs == all_inputs:
                out_mtime = os.path.getmtime(output_archive)
                if all(os.path.getmtime(inp) <= out_mtime for inp in all_inputs):
                    up_to_date = True
        except Exception:
            pass

    if up_to_date:
        log(f"{output_archive} is up-to-date; skipping merge.")
        return

    if os.path.exists(output_archive):
        os.remove(output_archive)

    mri_lines = [f"CREATE {output_archive}"]
    for lib in input_archives:
        mri_lines.append(f"ADDLIB {lib}")
    for obj in input_objects:
        mri_lines.append(f"ADDMOD {obj}")
    mri_lines.append("SAVE")
    mri_lines.append("END")

    mri_script = "\n".join(mri_lines) + "\n"

    log(
        f"Merging {len(input_archives)} archives and {len(input_objects)} objects into {output_archive}..."
    )
    try:
        process = subprocess.Popen(
            [llvm_ar_path, "-M"],
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
        )
        stdout, stderr = process.communicate(input=mri_script)
        if process.returncode != 0:
            fail(
                f"llvm-ar failed with exit code {process.returncode}\nStderr: {stderr}"
            )
        with open(manifest_path, "w") as f:
            f.write("\n".join(all_inputs) + "\n")
        log("Merge successful!")
    except Exception as e:
        fail(f"Failed to run llvm-ar: {e}")


def write_env_config(bazel_outputs_dir, env_vars):
    """Writes bazel-env.sh and updates GITHUB_ENV if running in GitHub Actions."""
    env_sh_path = os.path.join(bazel_outputs_dir, "bazel-env.sh")
    with open(env_sh_path, "w") as f:
        f.write("# Generated by setup_bazel_env.py\n")
        for k, v in env_vars.items():
            f.write(f'export {k}="{v}"\n')

    log(f"Saved Bazel environment setup to {env_sh_path}")

    github_env_path = os.environ.get("GITHUB_ENV")
    if github_env_path:
        with open(github_env_path, "a") as f:
            for k, v in env_vars.items():
                f.write(f"{k}={v}\n")
        log(
            f"Saved environment variables directly to GitHub Actions environment: {github_env_path}"
        )


def main():
    os.chdir(REPO_ROOT)

    # 1. Discover Bazel output directories
    try:
        output_base = subprocess.check_output(
            ["bazelisk", "info", "output_base"], text=True
        ).strip()
        external_dir = os.path.abspath(os.path.join(output_base, "external"))
    except Exception as e:
        fail(f"Failed to get Bazel output_base: {e}")
    log(f"Using Bazel external dir: {external_dir}")

    try:
        bazel_bin = subprocess.check_output(
            ["bazelisk", "info", "bazel-bin"], text=True
        ).strip()
        bin_external_base = os.path.abspath(os.path.join(bazel_bin, "external"))
    except Exception as e:
        fail(f"Failed to get Bazel bazel-bin: {e}")
    log(f"Using Bazel bin external dir: {bin_external_base}")

    bazel_outputs_dir = os.path.abspath(
        os.path.join(REPO_ROOT, "target", "bazel_outputs")
    )
    os.makedirs(bazel_outputs_dir, exist_ok=True)

    repo_mapping = get_bzlmod_repo_mapping()

    # 2. LLVM/Clang paths
    llvm_dir_name = repo_mapping.get("llvm-project")
    llvm_proj_dir = (
        os.path.join(external_dir, llvm_dir_name)
        if llvm_dir_name
        else resolve_repo_dir("@llvm-project//llvm:Support")
    )
    if not llvm_proj_dir or not os.path.isdir(llvm_proj_dir):
        fail(
            "Could not find llvm directory. Did you run 'bazelisk build"
            " rs_bindings_from_cc:rs_bindings_from_cc_main' first?"
        )
    resolved_llvm_dirname = os.path.basename(llvm_proj_dir)
    log(f"Found LLVM project source at: {llvm_proj_dir}")

    gen_llvm_proj_dir = find_llvm_generated_headers(
        bin_external_base, resolved_llvm_dirname
    )
    if not gen_llvm_proj_dir:
        fail(
            "Could not find LLVM generated headers in bazel-bin. Did you run"
            " 'bazelisk build rs_bindings_from_cc:rs_bindings_from_cc_main' first?"
        )

    clang_include_paths = [
        os.path.abspath(os.path.join(llvm_proj_dir, "clang", "include")),
        os.path.abspath(os.path.join(llvm_proj_dir, "llvm", "include")),
        os.path.abspath(os.path.join(gen_llvm_proj_dir, "clang", "include")),
        os.path.abspath(os.path.join(gen_llvm_proj_dir, "llvm", "include")),
    ]

    bin_llvm_proj_dir = os.path.join(bin_external_base, resolved_llvm_dirname)
    if not os.path.isdir(bin_llvm_proj_dir):
        fail(
            f"Clang/LLVM build output not found at {bin_llvm_proj_dir}. Did you"
            " run 'bazelisk build rs_bindings_from_cc:rs_bindings_from_cc_main'"
            " first?"
        )

    clang_lib_paths = [
        os.path.abspath(os.path.join(bin_llvm_proj_dir, "clang")),
        os.path.abspath(os.path.join(bin_llvm_proj_dir, "llvm")),
    ]

    # 3. Abseil paths
    absl_dir_name = repo_mapping.get("abseil-cpp")
    absl_src_dir = (
        os.path.join(external_dir, absl_dir_name)
        if absl_dir_name
        else resolve_repo_dir("@abseil-cpp//absl/base:base")
    )
    if not absl_src_dir or not os.path.isdir(absl_src_dir):
        fail(
            "Could not find abseil-cpp directory. Did you run 'bazelisk build rs_bindings_from_cc:rs_bindings_from_cc_main' first?"
        )
    log(f"Found Abseil source at: {absl_src_dir}")
    resolved_absl_dirname = os.path.basename(absl_src_dir)
    absl_include_paths = [os.path.abspath(absl_src_dir)]

    bin_absl_dir = os.path.join(bin_external_base, resolved_absl_dirname)
    if not os.path.isdir(bin_absl_dir):
        bin_absl_dir = os.path.join(bin_external_base, "abseil-cpp+")
    if not os.path.isdir(bin_absl_dir):
        fail(
            f"Abseil build output not found at {bin_absl_dir}. Did you run 'bazelisk build rs_bindings_from_cc:rs_bindings_from_cc_main' first?"
        )
    absl_lib_base = os.path.join(bin_absl_dir, "absl")

    # 4. Protobuf paths
    protobuf_dir_name = repo_mapping.get("protobuf")
    protobuf_dir = (
        os.path.join(external_dir, protobuf_dir_name)
        if protobuf_dir_name
        else resolve_repo_dir("@protobuf//:protobuf")
    )
    if not protobuf_dir or not os.path.isdir(protobuf_dir):
        fail(
            "Could not find protobuf directory. Did you run 'bazelisk build @protobuf//:protoc' first?"
        )
    resolved_protobuf_dirname = os.path.basename(protobuf_dir)
    bin_protobuf_dir = os.path.join(bin_external_base, resolved_protobuf_dirname)
    if not os.path.isdir(bin_protobuf_dir):
        bin_protobuf_dir = os.path.join(bin_external_base, "protobuf+")
    if not os.path.isdir(bin_protobuf_dir):
        fail(
            f"Protobuf build output not found at {bin_protobuf_dir}. Did you run 'bazelisk build @protobuf//:protoc' first?"
        )
    protobuf_lib_base = bin_protobuf_dir
    candidate_protoc = os.path.abspath(os.path.join(bin_protobuf_dir, "protoc"))

    if not os.path.exists(candidate_protoc):
        fail(
            f"protoc not found at expected path: {candidate_protoc}. Did you run 'bazelisk build @protobuf//:protoc' first?"
        )
    protoc_path = candidate_protoc

    # 5. Pre-generate proto headers and configure protoc wrapper
    proto_include_dir = os.path.join(bazel_outputs_dir, "include")
    log(f"Pre-generating C++ Protobuf headers into {proto_include_dir}...")
    generate_protobuf_headers(protoc_path, proto_include_dir, REPO_ROOT)

    # Protobuf version in Bazel (36.0.bcr.1) produces protoc 36.0, while Cargo
    # pins protobuf-codegen 4.35.0-release. The wrapper reports version 35.0 to
    # satisfy protobuf-codegen, and adjusts the gencode version assertion in generated
    # Rust files so they compile cleanly against the protobuf-4.35.0 crate.
    protoc_wrapper_dir = os.path.join(bazel_outputs_dir, "bin")
    os.makedirs(protoc_wrapper_dir, exist_ok=True)
    protoc_wrapper_path = os.path.join(protoc_wrapper_dir, "protoc")
    with open(protoc_wrapper_path, "w") as f:
        f.write(
            f"""#!/bin/sh
REAL_PROTOC="{protoc_path}"
if [ "$1" = "--version" ]; then
    echo "libprotoc 35.0"
    exit 0
fi
"$REAL_PROTOC" "$@"
ret=$?
if [ $ret -eq 0 ]; then
    for arg in "$@"; do
        case "$arg" in
            --rust_out=*)
                out_dir="${{arg#--rust_out=}}"
                find "$out_dir" -name "*.rs" -exec sed -i 's/"0\\.36\\.[0-9]*-release"/"4.35.0-release"/g' {{}} + 2>/dev/null
                ;;
        esac
    done
fi
exit $ret
"""
        )
    os.chmod(protoc_wrapper_path, 0o755)

    # 6. Hermetic Toolchain
    toolchain = find_hermetic_toolchain(external_dir)
    if not toolchain:
        fail("Hermetic toolchain or llvm-ar not found in toolchains_llvm.")

    # 7. Merge static libraries into monolithic archives
    absl_monolithic_path = os.path.join(bazel_outputs_dir, "libabsl_monolithic.a")
    merge_archives(toolchain["llvm_ar"], absl_monolithic_path, [absl_lib_base])

    clang_monolithic_path = os.path.join(bazel_outputs_dir, "libclang_monolithic.a")
    merge_archives(toolchain["llvm_ar"], clang_monolithic_path, clang_lib_paths)

    protobuf_monolithic_dir = os.path.join(bazel_outputs_dir, "protobuf")
    os.makedirs(protobuf_monolithic_dir, exist_ok=True)
    protobuf_monolithic_path = os.path.join(protobuf_monolithic_dir, "libprotobuf.a")
    merge_archives(toolchain["llvm_ar"], protobuf_monolithic_path, [protobuf_lib_base])

    # 8. Environment variables
    env_vars = {
        "CLANG_INCLUDE_PATH": ",".join(clang_include_paths),
        "CLANG_LIB_STATIC_PATH": bazel_outputs_dir,
        "ABSL_INCLUDE_PATH": ",".join(absl_include_paths),
        "ABSL_LIB_STATIC_PATH": bazel_outputs_dir,
        "PROTOC": protoc_wrapper_path,
        "PROTOBUF_INCLUDE_PATH": ",".join(
            [
                proto_include_dir,
                os.path.abspath(os.path.join(protobuf_dir, "src")),
                os.path.abspath(
                    os.path.join(protobuf_dir, "third_party", "utf8_range")
                ),
            ]
        ),
        "PROTOBUF_LIB_STATIC_PATH": os.path.join(bazel_outputs_dir, "protobuf"),
    }

    if toolchain:
        log(f"Found hermetic Clang: {toolchain['clang']}")
        env_vars["CC"] = toolchain["clang"]
        env_vars["CXX"] = toolchain["clang_xx"]
        env_vars["CXXFLAGS"] = "-stdlib=libc++"

        hermetic_clang_dir = os.path.dirname(toolchain["clang"])
        rustflags = [
            f"-C linker={toolchain['clang']}",
            f"-C link-arg=-fuse-ld={os.path.join(hermetic_clang_dir, 'ld.lld')}",
            "-C link-arg=-stdlib=libc++",
            "-C link-arg=-lc++",
            "-C link-arg=-lc++abi",
            "-C link-arg=-lunwind",
            "-C link-arg=-lzstd",
        ]
        if toolchain["lib_dir"]:
            log(f"Found hermetic Lib dir: {toolchain['lib_dir']}")
            rustflags.append(f"-C link-arg=-L{toolchain['lib_dir']}")
            rustflags.append(f"-C link-arg=-Wl,-rpath,{toolchain['lib_dir']}")
            rustflags.append("-C link-arg=-Wl,--disable-new-dtags")

        env_vars["RUSTFLAGS"] = " ".join(rustflags)

    write_env_config(bazel_outputs_dir, env_vars)

    print("\nTo configure your environment, run:")
    print(
        f"source {os.path.relpath(os.path.join(bazel_outputs_dir, 'bazel-env.sh'), REPO_ROOT)}"
    )


if __name__ == "__main__":
    main()
