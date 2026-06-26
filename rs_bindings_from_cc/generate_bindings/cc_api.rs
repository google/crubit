// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use arc_anyhow::Context;
use database::code_snippet::Bindings;
use error_report::{ErrorReport, ErrorReporting, FatalErrors, SourceLanguage};
use generate_bindings::generate_bindings as inner_generate_bindings;
use generate_bindings_rust_proto::{GenerateBindingsRequestView, GenerateBindingsResponseMut};
use ir::deserialize_ir;
use std::ffi::OsString;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::process;

pub fn generate_bindings(
    request: GenerateBindingsRequestView<'_>,
    mut response: GenerateBindingsResponseMut<'_>,
) {
    let ir = if request.has_ir_proto() {
        ir::proto_to_ir(request.ir_proto())
            .with_context(|| "Failed to deserialize IRProto".to_string())
            .unwrap()
    } else {
        let json: &[u8] = request.json().as_bytes();
        deserialize_ir(json)
            .with_context(|| {
                let ir_string = String::from_utf8_lossy(json);
                format!("Failed to deserialize IR:\n{}", ir_string)
            })
            .unwrap()
    };
    let crubit_support_path_format: &str = request
        .crubit_support_path_format()
        .to_str()
        .expect("crubit_support_path_format is not valid UTF-8");
    let clang_format_exe_path: OsString = request
        .clang_format_exe_path()
        .to_str()
        .expect("clang_format_exe_path is not valid UTF-8")
        .into();
    let rustfmt_exe_path: OsString =
        request.rustfmt_exe_path().to_str().expect("rustfmt_exe_path is not valid UTF-8").into();
    let rustfmt_config_path: OsString = request
        .rustfmt_config_path()
        .to_str()
        .expect("rustfmt_config_path is not valid UTF-8")
        .into();
    let kythe_default_corpus: &str =
        request.kythe_default_corpus().to_str().expect("kythe_default_corpus is not valid UTF-8");
    let generate_error_report = request.generate_error_report();
    let is_golden_test = request.is_golden_test();
    let kythe_annotations = request.kythe_annotations();

    // The `ir::IR` tree from the outer scope is not unwind safe. However, since we abort on
    // panic anyways, it is safe to bypass this check.
    catch_unwind(AssertUnwindSafe(|| {
        let mut error_report: Option<ErrorReport> = None;
        let errors: &dyn ErrorReporting = if generate_error_report {
            error_report.insert(ErrorReport::new(SourceLanguage::Cpp))
        } else {
            &error_report::IgnoreErrors
        };
        let fatal_errors = FatalErrors::new();
        let Bindings { rs_api, rs_api_impl } = inner_generate_bindings(
            &ir,
            crubit_support_path_format,
            &clang_format_exe_path,
            &rustfmt_exe_path,
            &rustfmt_config_path,
            errors,
            &fatal_errors,
            is_golden_test,
            kythe_annotations,
            kythe_default_corpus,
        )
        .unwrap();

        response.set_rs_api(rs_api);
        response.set_rs_api_impl(rs_api_impl);
        if let Some(report) = error_report {
            response.set_error_report(report.to_json_string());
        }
        response.set_fatal_errors(fatal_errors.take_string());
    }))
    .unwrap_or_else(|_| process::abort())
}
