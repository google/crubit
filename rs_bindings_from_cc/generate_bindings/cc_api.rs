// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use arc_anyhow::Context;
use database::code_snippet::Bindings;
use error_report::{ErrorReport, ErrorReporting, FatalErrors, SourceLanguage};
use generate_bindings::generate_bindings as inner_generate_bindings;
use generate_bindings_rust_proto::{GenerateBindingsRequest, GenerateBindingsResponseMut};
use protobuf::TakeFrom;
use std::ffi::OsString;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::process;

pub fn generate_bindings(
    mut request: GenerateBindingsRequest,
    mut response: GenerateBindingsResponseMut<'_>,
) {
    assert!(request.as_view().has_ir_proto(), "Request should have provided a valid IR protobuf.");
    let mut ir_proto = ir_rust_proto::IRProto::new();
    ir_proto.take_from(request.as_mut().ir_proto_mut());

    let request_view = request.as_view();
    let ir = ir::proto_to_ir(ir_proto.as_view())
        .with_context(|| "Failed to deserialize IRProto".to_string())
        .unwrap();
    let crubit_support_path_format: &str = request_view
        .crubit_support_path_format()
        .to_str()
        .expect("crubit_support_path_format is not valid UTF-8");
    let crubit_support_versioned_path_format: &str = request_view
        .crubit_support_versioned_path_format()
        .to_str()
        .expect("crubit_support_versioned_path_format is not valid UTF-8");
    let clang_format_exe_path: OsString = request_view
        .clang_format_exe_path()
        .to_str()
        .expect("clang_format_exe_path is not valid UTF-8")
        .into();
    let rustfmt_exe_path: OsString = request_view
        .rustfmt_exe_path()
        .to_str()
        .expect("rustfmt_exe_path is not valid UTF-8")
        .into();
    let rustfmt_config_path: OsString = request_view
        .rustfmt_config_path()
        .to_str()
        .expect("rustfmt_config_path is not valid UTF-8")
        .into();
    let kythe_default_corpus: &str = request_view
        .kythe_default_corpus()
        .to_str()
        .expect("kythe_default_corpus is not valid UTF-8");
    let generate_error_report = request_view.generate_error_report();
    let is_golden_test = request_view.is_golden_test();
    let kythe_annotations = request_view.kythe_annotations();

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
        match inner_generate_bindings(
            &ir,
            crubit_support_path_format,
            crubit_support_versioned_path_format,
            &clang_format_exe_path,
            &rustfmt_exe_path,
            &rustfmt_config_path,
            errors,
            &fatal_errors,
            is_golden_test,
            kythe_annotations,
            kythe_default_corpus,
        ) {
            Ok(Bindings { rs_api, rs_api_impl }) => {
                response.set_rs_api(rs_api);
                response.set_rs_api_impl(rs_api_impl);
                if let Some(report) = error_report {
                    response.set_error_report(report.to_json_string());
                }
                response.set_fatal_errors(fatal_errors.take_string());
            }
            Err(e) => {
                let mut err_str = fatal_errors.take_string();
                if !err_str.is_empty() {
                    err_str.push_str("\n");
                }
                err_str.push_str(&format!("{:#}", e));
                response.set_fatal_errors(err_str);
            }
        }
    }))
    .unwrap_or_else(|_| process::abort())
}
