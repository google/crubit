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

fn is_protobuf_lib(name: &str) -> bool {
    // Protobuf's CMake build prefixes these archives with `lib` on every platform,
    // including Windows, where `collect_static_libs` keeps the file stem as-is.
    matches!(name.strip_prefix("lib").unwrap_or(name), "protobuf" | "utf8_validity")
}

/// Returns the paths to the protobuf libraries (to be used as a search path) and a
/// list of libraries to be linked.
///
/// This is an allowlist rather than "everything in the directory", because a protobuf
/// install tree holds several archives that must not be linked together:
///
/// * `protobuf` is the library Crubit uses, and it has undefined references into
///   `utf8_validity`, so the two belong together.
/// * `utf8_range` is compiled from the same `utf8_range.c` as `utf8_validity` and
///   defines the same symbols, so linking both is a duplicate symbol error.
/// * `protobuf-lite` is a subset of `protobuf`, and would collide with it likewise.
/// * `protoc` and `upb` are not used by Crubit.
pub fn collect_protobuf_libs() -> (Vec<PathBuf>, Vec<OsString>) {
    paths::collect_static_libs("PROTOBUF_LIB_STATIC_PATH", is_protobuf_lib)
}

/// Unit tests for protobuf library allowlist matching.
///
/// Note: These tests use standard `#[test]` rather than `googletest` because
/// `crubit_build` is a build helper crate built via Cargo.
#[cfg(test)]
mod tests {
    use super::*;

    #[test] // allow_core_test (see mod tests doc comment)
    fn test_is_protobuf_lib() {
        assert!(is_protobuf_lib("protobuf"));
        assert!(is_protobuf_lib("libprotobuf"));
        assert!(is_protobuf_lib("utf8_validity"));
        assert!(is_protobuf_lib("libutf8_validity"));
        assert!(!is_protobuf_lib("utf8_range"));
        assert!(!is_protobuf_lib("libutf8_range"));
        assert!(!is_protobuf_lib("protobuf-lite"));
        assert!(!is_protobuf_lib("libprotobuf-lite"));
        assert!(!is_protobuf_lib("protoc"));
        assert!(!is_protobuf_lib("upb"));
    }
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
