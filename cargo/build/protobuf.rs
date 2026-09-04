// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use crate::paths;

use std::ffi::OsString;
use std::path::{Path, PathBuf};

/// Returns a list of include paths for protobuf headers.
pub fn collect_protobuf_includes() -> Vec<PathBuf> {
    paths::get_env_paths("PROTOBUF_INCLUDE_PATH")
}

/// Returns the paths to the protobuf libraries (to be used as a search path) and a
/// list of libraries to be linked.
pub fn collect_protobuf_libs() -> (Vec<PathBuf>, Vec<OsString>) {
    paths::collect_static_libs("PROTOBUF_LIB_STATIC_PATH", |name| {
        name.strip_prefix("lib").unwrap_or(name) == "protobuf"
    })
}

/// Locates pre-generated .pb.cc C++ source files corresponding to `proto_sources`
/// within the directories specified by `PROTOBUF_INCLUDE_PATH`.
///
/// Protobuf C++ headers are pre-generated upfront by `generate_proto_headers.py`
/// rather than compiled dynamically inside build.rs to avoid parallel build races.
/// For architectural details, see `docs/overview/cargo_build_protobuf.md`.
pub fn collect_generated_proto_sources<P: AsRef<Path>>(
    proto_sources: &[P],
    proto_include_dirs: &[PathBuf],
) -> Vec<PathBuf> {
    proto_sources
        .iter()
        .map(|proto| {
            let rel_pb_h = proto.as_ref().with_extension("pb.h");
            let h_path = proto_include_dirs
                .iter()
                .map(|inc_dir| inc_dir.join(&rel_pb_h))
                .find(|candidate| candidate.exists())
                .unwrap_or_else(|| {
                    panic!(
                        "\n\nERROR: Pre-generated protobuf C++ header '{}' not found in any PROTOBUF_INCLUDE_PATH ({:?}).\n\
                         Did you run 'python3 cargo/build/setup_bazel_env.py' first?\n\n",
                        rel_pb_h.display(),
                        proto_include_dirs
                    );
                });
            println!("cargo::rerun-if-changed={}", h_path.display());

            let rel_pb_cc = proto.as_ref().with_extension("pb.cc");
            proto_include_dirs
                .iter()
                .map(|inc_dir| inc_dir.join(&rel_pb_cc))
                .find(|candidate| candidate.exists())
                .unwrap_or_else(|| {
                    panic!(
                        "\n\nERROR: Pre-generated protobuf C++ source '{}' not found in any PROTOBUF_INCLUDE_PATH ({:?}).\n\
                         Did you run 'python3 cargo/build/setup_bazel_env.py' first?\n\n",
                        rel_pb_cc.display(),
                        proto_include_dirs
                    );
                })
        })
        .collect()
}
