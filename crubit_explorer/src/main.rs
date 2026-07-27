// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use axum::body::Body;
use axum::extract::State;
use axum::http::{header, StatusCode, Uri};
use axum::response::{IntoResponse, Response};
use axum::{routing, Router};
use std::error::Error;
use std::net::SocketAddr;
use std::path::{Component, Path, PathBuf};
use std::process::Command;
use tokio::net::TcpListener;

use runfiles::Runfiles;

const CC_BINDINGS_FROM_RS_RLOCATION: &str =
   "rules_crubit/cc_bindings_from_rs/cc_bindings_from_rs";

fn get_cc_bindings_from_rs_path() -> Result<PathBuf, Box<dyn Error>> {
    // Environment variable for location to cc_bindings_from_rs binary
    if let Ok(env_path) = std::env::var("CC_BINDINGS_FROM_RS")
        && let path = PathBuf::from(env_path)
        && path.exists()
    {
        return Ok(path);
    }

    // Check bazel runfiles tree
    if let Ok(r) = Runfiles::create()
        && let Some(path) = runfiles::rlocation!(r, CC_BINDINGS_FROM_RS_RLOCATION)
        && path.exists()
    {
        return Ok(path);
    }

    // Check if cc_bindings_from_rs is in the same directory as the executable
    // This is useful when crubit_explorer is run in a tarball or Docker container with
    // a specific directory structure
    if let Ok(mut exe_path) = std::env::current_exe() {
        exe_path.pop(); // Remove the executable name, leaving the directory
        let adjacent_path = exe_path.join("cc_bindings_from_rs");
        if adjacent_path.exists() {
            return Ok(adjacent_path);
        }
    }

    // Check system PATH
    if let Some(path) = find_in_path("cc_bindings_from_rs") {
        return Ok(path);
    }

    Err("cc_bindings_from_rs binary not found via CC_BINDINGS_FROM_RS env var, Bazel runfiles, adjacent to executable, or in system PATH".into())
}

#[cfg(test)]
fn new_cc_bindings_from_rs_command() -> Result<Command, Box<dyn Error>> {
    let binary_path = get_cc_bindings_from_rs_path()?;
    let mut cmd = Command::new(binary_path);

    let mut extra_lib_dirs = Vec::new();

    // Check Bazel runfiles
    if let Ok(r) = Runfiles::create()
        && let Ok(rustc_runfiles_env) = std::env::var("RUSTC_RUNFILES_PATH")
        && let Some(rustc_path) = runfiles::rlocation!(r, &rustc_runfiles_env)
    {
        let mut lib_dir = rustc_path;
        lib_dir.pop(); // pop rustc
        lib_dir.pop(); // pop bin
        let lib_dir = lib_dir.join("lib");
        if lib_dir.exists() {
            extra_lib_dirs.push(lib_dir);
        }
    }

    // Check adjacent lib directory to the current executable (useful in Docker/production tarball)
    if let Ok(mut exe_path) = std::env::current_exe() {
        exe_path.pop();
        let adjacent_lib = exe_path.join("lib");
        if adjacent_lib.exists() {
            extra_lib_dirs.push(adjacent_lib);
        }
    }

    if !extra_lib_dirs.is_empty() {
        const LIB_PATH_ENV: &str = cfg_select! {
            target_os = "macos" => "DYLD_LIBRARY_PATH",
            target_os = "windows" => "PATH",
            _ => "LD_LIBRARY_PATH",
        };

        let mut paths = extra_lib_dirs;
        if let Some(old_val) = std::env::var_os(LIB_PATH_ENV) {
            paths.extend(std::env::split_paths(&old_val));
        }

        let new_val = std::env::join_paths(paths)?;
        cmd.env(LIB_PATH_ENV, new_val);
    }

    Ok(cmd)
}

fn find_in_path(name: &str) -> Option<PathBuf> {
    let paths = std::env::var_os("PATH")?;
    std::env::split_paths(&paths).find_map(|dir| {
        let full_path = dir.join(name);
        full_path.exists().then_some(full_path)
    })
}

fn get_frontend_dist_path() -> Option<PathBuf> {
    if let Ok(r) = Runfiles::create() {
        if let Some(path) = runfiles::rlocation!(r, "crubit_explorer/frontend/dist/frontend") {
            if path.exists() {
                return Some(path);
            }
        }
    }

    if let Ok(mut exe_path) = std::env::current_exe() {
        exe_path.pop();
        let adjacent_path = exe_path.join("frontend/dist/frontend");
        if adjacent_path.exists() {
            return Some(adjacent_path);
        }
        let adjacent_path2 = exe_path.join("dist/frontend");
        if adjacent_path2.exists() {
            return Some(adjacent_path2);
        }
    }

    None
}

fn app(frontend_path: Option<PathBuf>) -> Router {
    if let Some(path) = frontend_path {
        println!("Serving frontend from {}", path.display());
        Router::new().fallback(serve_frontend).with_state(path)
    } else {
        println!("Frontend not found, serving Hello World at root");
        Router::new().route("/", routing::get(|| async { "Hello, World!" }))
    }
}

async fn serve_frontend(uri: Uri, State(frontend_path): State<PathBuf>) -> impl IntoResponse {
    let path = uri.path();
    let relative_path = path.trim_start_matches('/');

    if relative_path.is_empty() {
        return serve_file(&frontend_path.join("index.html")).await;
    }

    // Don't allow path traversal
    let has_parent_dir =
        Path::new(relative_path).components().any(|c| matches!(c, Component::ParentDir));

    if has_parent_dir {
        return (StatusCode::FORBIDDEN, "Forbidden").into_response();
    }

    let file_path = frontend_path.join(relative_path);
    if file_path.exists() && file_path.is_file() {
        return serve_file(&file_path).await;
    }

    serve_file(&frontend_path.join("index.html")).await
}

async fn serve_file(path: &Path) -> Response {
    match tokio::fs::read(path).await {
        Ok(content) => {
            let mime = mime_guess_fallback(path);
            Response::builder()
                .header(header::CONTENT_TYPE, mime)
                .body(Body::from(content))
                .unwrap_or_else(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())
        }
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}

fn mime_guess_fallback(path: &Path) -> &'static str {
    match path.extension().and_then(|s| s.to_str()) {
        Some("html") => "text/html",
        Some("js") => "application/javascript",
        Some("css") => "text/css",
        Some("svg") => "image/svg+xml",
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("wasm") => "application/wasm",
        _ => "application/octet-stream",
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    match get_cc_bindings_from_rs_path() {
        Ok(path) => println!("cc_bindings_from_rs found at: {}", path.display()),
        Err(err) => eprintln!("Error locating cc_bindings_from_rs: {}", err),
    }

    let frontend_path = get_frontend_dist_path();
    let app = app(frontend_path);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Listening on http://{}", addr);

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use googletest::prelude::*;
    use tower::ServiceExt;

    #[gtest]
    #[tokio::test]
    async fn hello_world() {
        let app = app(None);

        let response =
            app.oneshot(Request::builder().uri("/").body(Body::empty()).unwrap()).await.unwrap();

        expect_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        expect_eq!(&body[..], b"Hello, World!");
    }

    #[gtest]
    #[tokio::test]
    async fn test_frontend_serving() {
        let path = get_frontend_dist_path();
        if let Some(frontend_path) = path {
            let app = app(Some(frontend_path));
            let response = app
                .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
                .await
                .unwrap();

            expect_eq!(response.status(), StatusCode::OK);
            let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
            let html_str = String::from_utf8(body.to_vec()).unwrap();
            expect_that!(html_str, contains_substring("<app-root>"));
        } else {
            println!("Frontend dist path not found, skipping test.");
        }
    }

    #[gtest]
    #[tokio::test]
    async fn test_cc_bindings_from_rs_help() {
        let mut cmd = new_cc_bindings_from_rs_command()
            .expect("Failed to create cc_bindings_from_rs command");
        let output = cmd.arg("--help").output().expect("Failed to execute cc_bindings_from_rs");

        expect_true!(output.status.success());
        let stdout = String::from_utf8(output.stdout).unwrap();
        expect_that!(stdout, contains_substring("Generates C++ bindings for a Rust crate"));
    }

    #[gtest]
    #[tokio::test]
    async fn test_generate_bindings() {
        let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
        let rs_input = temp_dir.path().join("input.rs");
        let h_out = temp_dir.path().join("output.h");
        let rs_out = temp_dir.path().join("output.rs");

        std::fs::write(&rs_input, b"#[no_mangle] pub extern \"C\" fn foo() {}")
            .expect("Failed to write input file");

        let mut cmd = new_cc_bindings_from_rs_command()
            .expect("Failed to create cc_bindings_from_rs command");
        cmd.arg(format!("--h-out={}", h_out.display()))
            .arg(format!("--rs-out={}", rs_out.display()))
            .arg("--crubit-support-path-format=<crubit/support/{header}>")
            .arg("--")
            .arg(&rs_input)
            .arg("--crate-type=lib");

        let output = cmd.output().expect("Failed to execute cc_bindings_from_rs");

        let stderr = String::from_utf8_lossy(&output.stderr);

        expect_true!(output.status.success(), "Command failed with stderr: {}", stderr);
        expect_true!(h_out.exists());
        expect_true!(rs_out.exists());

        let h_content = std::fs::read_to_string(&h_out).expect("Failed to read h_out");
        let rs_content = std::fs::read_to_string(&rs_out).expect("Failed to read rs_out");

        expect_false!(h_content.is_empty());
        expect_false!(rs_content.is_empty());
        expect_that!(h_content, contains_substring("foo"));
    }
}
