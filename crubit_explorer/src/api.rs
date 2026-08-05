// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use axum::extract::rejection::JsonRejection;
use axum::extract::Json;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use runfiles::Runfiles;
use serde::{Deserialize, Serialize};
use std::env;
use std::fmt::Display;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use tempfile::{Builder, TempDir};

use crate::resource_locator;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CrubitBuildRequest {
    pub plugin_name: String,
    #[serde(default)]
    pub enable_codegen_tracing: bool,
    #[serde(default)]
    pub plugin_flags: Vec<String>,
    pub input: FileSet,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FileSet {
    pub files: Vec<File>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct File {
    pub name: String,
    pub contents_b64: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorDetails {
    pub text: String,
    pub reason: String,
}

impl ErrorDetails {
    pub fn new(text: impl Into<String>, reason: impl Into<String>) -> Self {
        Self { text: text.into(), reason: reason.into() }
    }
}

trait MapErrToDetailsExt<T> {
    fn map_err_to_details(self, text: impl Into<String>) -> Result<T, ErrorDetails>;
}

impl<T, E: Display> MapErrToDetailsExt<T> for Result<T, E> {
    fn map_err_to_details(self, text: impl Into<String>) -> Result<T, ErrorDetails> {
        self.map_err(|e| ErrorDetails::new(text, e.to_string()))
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CrubitBuildResponse {
    Success { output: FileSet },
    Error { error: ErrorDetails },
}

fn json_error(e: impl Display, text: impl Into<String>) -> Json<CrubitBuildResponse> {
    Json(CrubitBuildResponse::Error { error: ErrorDetails::new(text, e.to_string()) })
}

pub async fn compile_handler(
    payload: Result<Json<CrubitBuildRequest>, JsonRejection>,
) -> Json<CrubitBuildResponse> {
    let payload = match payload {
        Ok(Json(p)) => p,
        Err(e) => {
            return json_error(e, "Invalid request format");
        }
    };

    let result = tokio::task::spawn_blocking(move || execute_compile(payload)).await;

    match result {
        Ok(Ok(output)) => Json(CrubitBuildResponse::Success { output }),
        Ok(Err(error)) => Json(CrubitBuildResponse::Error { error }),
        Err(e) => json_error(e, "Internal server error"),
    }
}

fn execute_compile(mut payload: CrubitBuildRequest) -> Result<FileSet, ErrorDetails> {
    let env = prepare_input_environment(&mut payload)?;
    run_compiler_command(&payload, &env)?;
    collect_output_files(&env)
}

struct CompileInput {
    _temp_dir: TempDir,
    input_path: PathBuf,
    h_out_path: PathBuf,
    rs_out_path: PathBuf,
    h_out_name: String,
    rs_out_name: String,
}

fn prepare_input_environment(
    payload: &mut CrubitBuildRequest,
) -> Result<CompileInput, ErrorDetails> {
    if payload.plugin_name != "cc_bindings_from_rs" {
        return Err(ErrorDetails::new(
            "Compilation failed",
            format!("Only cc_bindings_from_rs is supported, got: {}", payload.plugin_name),
        ));
    }

    if payload.input.files.is_empty() {
        return Err(ErrorDetails::new("No input files", "Empty input files"));
    }

    let temp_dir = Builder::new()
        .prefix("crubit_")
        .tempdir()
        .map_err_to_details("Failed to create temp dir")?;

    let input_file = payload.input.files.remove(0);
    let File { name, contents_b64 } = input_file;

    let decoded_content =
        BASE64_STANDARD.decode(&contents_b64).map_err_to_details("Base64 decode failed")?;

    let base_name = Path::new(&name).file_name().and_then(|n| n.to_str()).unwrap_or("test.rs");

    let file_name = if base_name.ends_with(".rs") {
        base_name.to_string()
    } else {
        format!("{}.rs", base_name)
    };

    let input_path = temp_dir.path().join(&file_name);
    fs::write(&input_path, decoded_content).map_err_to_details("Write failed")?;

    let stem = file_name.trim_end_matches(".rs");
    let h_out_name = format!("{}.h", stem);
    let rs_out_name = format!("{}_impl.rs", stem);

    let h_out_path = temp_dir.path().join(&h_out_name);
    let rs_out_path = temp_dir.path().join(&rs_out_name);

    Ok(CompileInput {
        _temp_dir: temp_dir,
        input_path,
        h_out_path,
        rs_out_path,
        h_out_name,
        rs_out_name,
    })
}

fn run_compiler_command(
    payload: &CrubitBuildRequest,
    env: &CompileInput,
) -> Result<Output, ErrorDetails> {
    let mut cmd = crate::new_cc_bindings_from_rs_command()
        .map_err_to_details("Failed to locate cc_bindings_from_rs command")?;

    cmd.arg(format!("--h-out={}", env.h_out_path.display()))
        .arg(format!("--rs-out={}", env.rs_out_path.display()))
        .arg("--crubit-support-path-format=<crubit/support/{header}>");

    if let Some(clang_format_path) = resource_locator::get_clang_format_path() {
        cmd.arg(format!("--clang-format-exe-path={}", clang_format_path.display()));
    }

    if payload.enable_codegen_tracing {
        cmd.arg("--enable-codegen-tracing");
    }

    for flag in &payload.plugin_flags {
        cmd.arg(flag);
    }

    cmd.arg("--").arg(&env.input_path).arg("--crate-type=lib");

    resolve_sysroot_and_target_flags(&mut cmd, env._temp_dir.path());

    let output = cmd.output().map_err_to_details("Error executing command")?;

    if !output.status.success() {
        return Err(ErrorDetails::new(
            "Bindings generation failed",
            String::from_utf8_lossy(&output.stderr),
        ));
    }

    Ok(output)
}

fn resolve_sysroot_and_target_flags(cmd: &mut Command, temp_dir_path: &Path) {
    if let Some(sysroot) = find_sysroot_from_env() {
        cmd.arg(format!("--sysroot={}", sysroot.display()));
        return;
    }

    if configure_from_runfiles(cmd, temp_dir_path) {
        return;
    }

    if let Some(sysroot) = find_sysroot_from_exe_dir().or_else(find_sysroot_from_rustc) {
        cmd.arg(format!("--sysroot={}", sysroot.display()));
    }
}

fn find_sysroot_from_env() -> Option<PathBuf> {
    let sysroot_env = env::var("SYSROOT").or_else(|_| env::var("RUSTC_SYSROOT")).ok()?;
    let p = PathBuf::from(&sysroot_env);
    p.exists().then_some(p)
}

fn configure_from_runfiles(cmd: &mut Command, temp_dir_path: &Path) -> bool {
    let Ok(r) = Runfiles::create() else { return false };
    let mut found_sysroot = false;

    if !found_sysroot && let Ok(rustc_rf) = env::var("RUSTC_RUNFILES_PATH") {
        if let Some(rustc_path) = runfiles::rlocation!(r, &rustc_rf) {
            if let Some(sysroot_path) = rustc_path.parent().and_then(|p| p.parent()) {
                if sysroot_path.exists() {
                    cmd.arg(format!("--sysroot={}", sysroot_path.display()));
                    found_sysroot = true;
                }
            }
        }
    }

    found_sysroot
}

fn find_sysroot_from_exe_dir() -> Option<PathBuf> {
    let exe_path = env::current_exe().ok()?;
    let exe_dir = exe_path.parent()?;

    let candidates =
        [exe_dir.join("sysroot"), exe_dir.join("rustlib"), exe_dir.join("lib/rustlib")];
    for candidate in &candidates {
        if candidate.exists() {
            let sysroot = if candidate.ends_with("lib/rustlib") || candidate.ends_with("rustlib") {
                candidate.parent().and_then(|p| p.parent()).unwrap_or(candidate)
            } else {
                candidate
            };
            return Some(sysroot.to_path_buf());
        }
    }

    if let Some(parent_dir) = exe_dir.parent() {
        if parent_dir.join("lib/rustlib").exists() {
            return Some(parent_dir.to_path_buf());
        }
    }

    None
}

fn find_sysroot_from_rustc() -> Option<PathBuf> {
    let output = Command::new("rustc").arg("--print").arg("sysroot").output().ok()?;
    if !output.status.success() {
        return None;
    }
    let sysroot_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let sysroot_path = PathBuf::from(&sysroot_str);
    sysroot_path.exists().then_some(sysroot_path)
}

fn collect_output_files(env: &CompileInput) -> Result<FileSet, ErrorDetails> {
    let mut output_files = Vec::new();

    if env.h_out_path.exists() {
        let h_content = fs::read(&env.h_out_path)
            .map_err_to_details("Failed to read generated C++ header file")?;
        output_files.push(File {
            name: env.h_out_name.clone(),
            contents_b64: BASE64_STANDARD.encode(&h_content),
        });
    }

    if env.rs_out_path.exists() {
        let rs_content = fs::read(&env.rs_out_path)
            .map_err_to_details("Failed to read generated Rust implementation file")?;
        output_files.push(File {
            name: env.rs_out_name.clone(),
            contents_b64: BASE64_STANDARD.encode(&rs_content),
        });
    }

    if output_files.is_empty() {
        return Err(ErrorDetails::new(
            "No output files generated",
            "Crubit ran but produced no output files",
        ));
    }

    Ok(FileSet { files: output_files })
}
