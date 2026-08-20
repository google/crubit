// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use crate::paths;

use std::path::{Path, PathBuf};

/// Returns a list of include paths for protobuf headers.
pub fn collect_protobuf_includes() -> Vec<PathBuf> {
    paths::get_env_paths("PROTOBUF_INCLUDE_PATH")
}

/// Returns search paths to protobuf static libraries.
pub fn collect_protobuf_lib_dirs() -> Vec<PathBuf> {
    paths::get_env_paths("PROTOBUF_LIB_STATIC_PATH")
}

/// Locates pre-generated .pb.cc C++ source files corresponding to `proto_sources`
/// within the directories specified by `PROTOBUF_INCLUDE_PATH`.
pub fn collect_generated_proto_sources<P: AsRef<Path>>(
    proto_sources: &[P],
    proto_include_dirs: &[PathBuf],
) -> Vec<PathBuf> {
    let mut generated_cc_files = Vec::new();
    for proto in proto_sources {
        let proto_path = proto.as_ref();
        let stem = proto_path
            .file_stem()
            .expect("proto path should have a file name");
        let mut pb_cc_filename = stem.to_owned();
        pb_cc_filename.push(".pb.cc");
        let rel_pb_cc = proto_path
            .parent()
            .unwrap_or(Path::new(""))
            .join(pb_cc_filename);

        let mut found = None;
        for inc_dir in proto_include_dirs {
            let candidate = inc_dir.join(&rel_pb_cc);
            if candidate.exists() {
                found = Some(candidate);
                break;
            }
        }

        match found {
            Some(path) => {
                println!("cargo::rerun-if-changed={}", path.display());
                generated_cc_files.push(path);
            }
            None => {
                panic!(
                    "\n\nERROR: Pre-generated protobuf C++ source '{}' not found in any PROTOBUF_INCLUDE_PATH ({:?}).\n\
                     Did you run 'python3 cargo/build/generate_proto_headers.py --out_dir=...' first?\n\n",
                    rel_pb_cc.display(),
                    proto_include_dirs
                );
            }
        }
    }
    generated_cc_files
}
