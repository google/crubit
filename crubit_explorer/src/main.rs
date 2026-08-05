// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

pub mod api;
pub mod doxygen;
pub mod resource_locator;

use axum::body::Body;
use axum::extract::State;
use axum::http::{header, StatusCode, Uri};
use axum::response::{IntoResponse, Response};
use axum::{routing, Router};
use std::env;
use std::error::Error;
use std::net::SocketAddr;
use std::path::{Component, Path, PathBuf};
use std::process::Command;
use tokio::net::TcpListener;

use runfiles::Runfiles;

pub(crate) fn new_cc_bindings_from_rs_command() -> Result<Command, Box<dyn Error>> {
    let binary_path = resource_locator::get_cc_bindings_from_rs_path()?;
    let mut cmd = Command::new(binary_path);

    let mut extra_lib_dirs = Vec::new();

    // Check Bazel runfiles
    if let Ok(r) = Runfiles::create()
        && let Ok(rustc_runfiles_env) = env::var("RUSTC_RUNFILES_PATH")
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
    if let Ok(mut exe_path) = env::current_exe() {
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
        if let Some(old_val) = env::var_os(LIB_PATH_ENV) {
            paths.extend(env::split_paths(&old_val));
        }

        let new_val = env::join_paths(paths)?;
        cmd.env(LIB_PATH_ENV, new_val);
    }

    Ok(cmd)
}

fn app(frontend_path: Option<PathBuf>) -> Router {
    let mut app = Router::new()
        .route("/api/compile", routing::post(api::compile_handler).get(api::compile_handler))
        .route(
            "/api/doxygen",
            routing::post(doxygen::doxygen_handler).get(doxygen::doxygen_handler),
        )
        .route("/doxygen", routing::post(doxygen::doxygen_handler).get(doxygen::doxygen_handler));

    if let Some(path) = frontend_path {
        println!("Serving frontend from {}", path.display());
        let frontend_router = Router::new().fallback(serve_frontend).with_state(path);
        app = app.fallback_service(frontend_router);
    } else {
        println!("Frontend not found, serving Hello World at root");
        app = app.route("/", routing::get(|| async { "Hello, World!" }));
    }

    app
}

async fn serve_frontend(uri: Uri, State(frontend_path): State<PathBuf>) -> impl IntoResponse {
    let path = uri.path();

    if path.starts_with("/api") {
        return (
            StatusCode::NOT_FOUND,
            axum::Json(serde_json::json!({
                "error": {
                    "text": "Not Found",
                    "reason": format!("API endpoint not found: {}", path)
                }
            })),
        )
            .into_response();
    }

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
    match resource_locator::get_cc_bindings_from_rs_path() {
        Ok(path) => println!("cc_bindings_from_rs found at: {}", path.display()),
        Err(err) => eprintln!("Error locating cc_bindings_from_rs: {}", err),
    }

    match resource_locator::get_clang_format_path() {
        Some(path) => println!("clang-format found at: {}", path.display()),
        None => println!("clang-format not found; generated code will be unformatted"),
    }

    match resource_locator::get_rustfmt_path() {
        Some(path) => println!("rustfmt found at: {}", path.display()),
        None => println!("rustfmt not found; generated Rust code will be unformatted"),
    }

    match resource_locator::get_doxygen_path() {
        Ok(path) => println!("doxygen found at: {}", path.display()),
        Err(err) => eprintln!("Error locating doxygen: {}", err),
    }

    let frontend_path = resource_locator::get_frontend_dist_path();
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
    use base64::prelude::BASE64_STANDARD;
    use base64::Engine;
    use googletest::prelude::*;
    use std::fs;
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
    async fn test_compile_handler_api() {
        let app = app(resource_locator::get_frontend_dist_path());

        let input_code = "pub struct TestStruct { pub x: i32 }";
        let payload = api::CrubitBuildRequest {
            plugin_name: "cc_bindings_from_rs".to_string(),
            enable_codegen_tracing: false,
            plugin_flags: vec![],
            input: api::FileSet {
                files: vec![api::File {
                    name: "test.rs".to_string(),
                    contents_b64: BASE64_STANDARD.encode(input_code),
                }],
            },
        };

        let req_body = serde_json::to_vec(&payload).unwrap();

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/compile")
                    .header("content-type", "application/json")
                    .body(Body::from(req_body))
                    .unwrap(),
            )
            .await
            .unwrap();

        expect_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let resp: api::CrubitBuildResponse = serde_json::from_slice(&body).unwrap();

        match resp {
            api::CrubitBuildResponse::Success { output } => {
                expect_true!(!output.files.is_empty());
                let h_file = output
                    .files
                    .iter()
                    .find(|f| f.name.ends_with(".h"))
                    .expect("Expected .h file in output");
                let decoded =
                    String::from_utf8(BASE64_STANDARD.decode(&h_file.contents_b64).unwrap())
                        .unwrap();
                expect_that!(decoded, contains_substring("#include <cstddef>"));
            }
            api::CrubitBuildResponse::Error { error } => {
                panic!("Expected success response, got error: {:?}", error);
            }
        }
    }

    #[gtest]
    #[tokio::test]
    async fn test_compile_handler_api_invalid_payload() {
        let app = app(resource_locator::get_frontend_dist_path());

        // Send invalid JSON body
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/compile")
                    .header("content-type", "application/json")
                    .body(Body::from("{invalid_json}"))
                    .unwrap(),
            )
            .await
            .unwrap();

        expect_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let resp: api::CrubitBuildResponse = serde_json::from_slice(&body).unwrap();

        match resp {
            api::CrubitBuildResponse::Error { error } => {
                expect_that!(error.text, contains_substring("Invalid request format"));
            }
            api::CrubitBuildResponse::Success { .. } => {
                panic!("Expected error response for invalid JSON payload");
            }
        }
    }

    #[gtest]
    #[tokio::test]
    async fn test_api_not_found() {
        let app = app(resource_locator::get_frontend_dist_path());

        let response = app
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/api/nonexistent")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        expect_eq!(response.status(), StatusCode::NOT_FOUND);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let json_val: serde_json::Value = serde_json::from_slice(&body).unwrap();
        expect_that!(json_val["error"]["text"].as_str().unwrap(), contains_substring("Not Found"));
    }

    #[gtest]
    #[tokio::test]
    async fn test_frontend_serving() {
        let path = resource_locator::get_frontend_dist_path();
        let Some(frontend_path) = path else {
            println!("Frontend dist path not found, skipping test.");
            return;
        };

        let app = app(Some(frontend_path));
        let response =
            app.oneshot(Request::builder().uri("/").body(Body::empty()).unwrap()).await.unwrap();

        expect_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let html_str = String::from_utf8(body.to_vec()).unwrap();
        expect_that!(html_str, contains_substring("<app-root>"));
    }

    #[gtest]
    #[tokio::test]
    async fn test_frontend_serving_with_query_params() {
        let path = resource_locator::get_frontend_dist_path();
        let Some(frontend_path) = path else {
            println!("Frontend dist path not found, skipping test.");
            return;
        };

        let app = app(Some(frontend_path));
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/?code=cHViIHN0cnVjdCBNeVN0cnVjdCB7fQ")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        expect_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let html_str = String::from_utf8(body.to_vec()).unwrap();
        expect_that!(html_str, contains_substring("<app-root>"));
    }

    #[gtest]
    #[tokio::test]
    async fn test_frontend_embed_route_serving() {
        let path = resource_locator::get_frontend_dist_path();
        let Some(frontend_path) = path else {
            println!("Frontend dist path not found, skipping test.");
            return;
        };

        let app = app(Some(frontend_path));
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/embed#code=cHViIHN0cnVjdCBNeVN0cnVjdCB7fQ&tool=cc_bindings_from_rs&editable=true&view=split")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        expect_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let html_str = std::str::from_utf8(&body).unwrap();
        expect_that!(html_str, contains_substring("<app-root>"));
    }

    #[gtest]
    #[tokio::test]
    async fn test_frontend_3rdpartylicenses_serving() {
        let path = resource_locator::get_frontend_dist_path();
        if let Some(frontend_path) = path {
            let app = app(Some(frontend_path));
            let response = app
                .oneshot(
                    Request::builder().uri("/3rdpartylicenses.txt").body(Body::empty()).unwrap(),
                )
                .await
                .unwrap();

            expect_eq!(response.status(), StatusCode::OK);
            let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
            let text = String::from_utf8(body.to_vec()).unwrap();
            expect_that!(text, contains_substring("MIT License"));
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

        fs::write(&rs_input, b"#[no_mangle] pub extern \"C\" fn foo() {}")
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

        let h_content = fs::read_to_string(&h_out).expect("Failed to read h_out");
        let rs_content = fs::read_to_string(&rs_out).expect("Failed to read rs_out");

        expect_false!(h_content.is_empty());
        expect_false!(rs_content.is_empty());
        expect_that!(h_content, contains_substring("foo"));
    }

    #[gtest]
    #[tokio::test]
    async fn test_get_clang_format_path() {
        let clang_format_path = resource_locator::get_clang_format_path();
        expect_true!(
            clang_format_path.is_some(),
            "clang-format should be found in test environment"
        );
        let path = clang_format_path.unwrap();
        expect_true!(path.exists(), "clang-format path {:?} must exist", path);
    }

    #[gtest]
    #[tokio::test]
    async fn test_doxygen_handler_api() {
        let app = app(None);

        let input_code = "class TestClass { public: void doSomething(); };";
        let payload = doxygen::DoxygenRequest {
            input: api::FileSet {
                files: vec![api::File {
                    name: "test.h".to_string(),
                    contents_b64: BASE64_STANDARD.encode(input_code),
                }],
            },
        };

        let req_body = serde_json::to_vec(&payload).unwrap();

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/doxygen")
                    .header("content-type", "application/json")
                    .body(Body::from(req_body))
                    .unwrap(),
            )
            .await
            .unwrap();

        expect_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let resp: doxygen::DoxygenResponse = serde_json::from_slice(&body).unwrap();

        expect_true!(resp.error.is_none(), "Expected no error, got: {:?}", resp.error);
        expect_true!(resp.xml_output.is_some());
        expect_true!(resp.file_symbols.is_some());

        let file_symbols = resp.file_symbols.unwrap();
        expect_true!(
            file_symbols.contains_key("test.h"),
            "file_symbols should contain test.h, keys are: {:?}",
            file_symbols.keys()
        );
        let symbols = &file_symbols["test.h"].symbols;
        let class_symbol = symbols.iter().find(|s| s.name == "TestClass");
        expect_true!(class_symbol.is_some(), "Expected TestClass in symbols: {:?}", symbols);
        expect_eq!(class_symbol.unwrap().kind, doxygen::SymbolKind::Class);

        let func_symbol = symbols.iter().find(|s| s.name == "TestClass::doSomething");
        expect_true!(
            func_symbol.is_some(),
            "Expected TestClass::doSomething in symbols: {:?}",
            symbols
        );
        expect_eq!(func_symbol.unwrap().kind, doxygen::SymbolKind::Function);
    }

    #[gtest]
    #[tokio::test]
    async fn test_doxygen_handler_crubit_internal_rust_type() {
        let app = app(None);

        let input_code = r#"
namespace input {
struct CRUBIT_INTERNAL_RUST_TYPE(":: input :: MyStruct") alignas(4) [[clang::trivial_abi]] MyStruct final {
  union { ::std::int32_t a; };
};
::input::MyStruct hello();
}
"#;
        let payload = doxygen::DoxygenRequest {
            input: api::FileSet {
                files: vec![api::File {
                    name: "bindings.h".to_string(),
                    contents_b64: BASE64_STANDARD.encode(input_code),
                }],
            },
        };

        let req_body = serde_json::to_vec(&payload).unwrap();

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/doxygen")
                    .header("content-type", "application/json")
                    .body(Body::from(req_body))
                    .unwrap(),
            )
            .await
            .unwrap();

        expect_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let resp: doxygen::DoxygenResponse = serde_json::from_slice(&body).unwrap();

        expect_true!(resp.error.is_none(), "Expected no error, got: {:?}", resp.error);
        let file_symbols = resp.file_symbols.unwrap();
        expect_true!(file_symbols.contains_key("bindings.h"));
        let symbols = &file_symbols["bindings.h"].symbols;
        let struct_symbol = symbols.iter().find(|s| s.name.contains("MyStruct"));
        expect_true!(struct_symbol.is_some(), "Expected MyStruct in symbols, got: {:?}", symbols);
    }
}
