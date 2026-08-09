// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use crate::paths;

use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Returns a list of include paths for protobuf headers.
pub fn collect_protobuf_includes() -> Vec<PathBuf> {
    if let Some(env_paths) = paths::print_env_to_string("PROTOBUF_INCLUDE_PATH") {
        env_paths.split(',').map(|s| Path::new(s).to_owned()).collect()
    } else {
        vec![]
    }
}

/// Returns search paths to protobuf static libraries.
pub fn collect_protobuf_lib_dirs() -> Vec<PathBuf> {
    paths::print_env_to_string("PROTOBUF_LIB_STATIC_PATH")
        .unwrap_or_default()
        .split(',')
        .filter(|s| !s.is_empty())
        .map(PathBuf::from)
        .collect()
}

/// Compiles `.proto` files to C++ source/headers using `protoc`, and returns paths to all
/// generated .pb.cc C++ source files inside the out-directory.
pub fn compile_protos<P1: AsRef<Path>, P2: AsRef<Path>>(
    path_to_src_root: P1,
    proto_sources: &[P2],
    out_dir: &Path,
) -> std::io::Result<Vec<PathBuf>> {
    if proto_sources.is_empty() {
        return Ok(vec![]);
    }

    let protoc_bin = paths::print_env_to_string("PROTOC").unwrap_or_else(|| "protoc".to_string());
    let mut cmd = Command::new(&protoc_bin);
    cmd.arg(format!("--cpp_out={}", out_dir.display()));
    cmd.arg("-I").arg(path_to_src_root.as_ref());

    let mut generated_cc_files = Vec::new();
    for proto in proto_sources {
        let proto_full_path = path_to_src_root.as_ref().join(proto.as_ref());
        if proto_full_path.exists() {
            cmd.arg(&proto_full_path);

            let stem = proto_full_path.file_stem().expect("proto path should have a file name");
            let mut pb_cc = stem.to_owned();
            pb_cc.push(".pb.cc");
            let parent = proto.as_ref().parent().unwrap_or(Path::new(""));
            generated_cc_files.push(out_dir.join(parent).join(pb_cc));
        }
    }

    if !generated_cc_files.is_empty() {
        let output = cmd.output()?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);
            if !stderr.is_empty() {
                println!("cargo:warning=protoc stderr: {}", stderr.trim());
            }
            if !stdout.is_empty() {
                println!("cargo:warning=protoc stdout: {}", stdout.trim());
            }
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "protoc failed for files {:?}: {}",
                    proto_sources.iter().map(|p| p.as_ref()).collect::<Vec<_>>(),
                    stderr
                ),
            ));
        }
    }

    Ok(generated_cc_files)
}
