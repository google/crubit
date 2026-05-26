// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use database::code_snippet::Bindings;
use error_report::{ErrorReport, ErrorReporting, FatalErrors, SourceLanguage};
use ffi_types::Environment;
use generate_bindings::generate_bindings as inner_generate_bindings;
use generate_bindings_rust_proto::{GenerateBindingsRequest, GenerateBindingsResponse};
use std::ffi::OsString;
use std::panic::catch_unwind;
use std::process;

pub fn generate_bindings(request: &GenerateBindingsRequest) -> GenerateBindingsResponse {
    let json: &[u8] = request.json().as_bytes();
    let crubit_support_path_format: &str =
        std::str::from_utf8(request.crubit_support_path_format().as_ref()).unwrap();
    let clang_format_exe_path: OsString =
        std::str::from_utf8(request.clang_format_exe_path().as_ref()).unwrap().into();
    let rustfmt_exe_path: OsString =
        std::str::from_utf8(request.rustfmt_exe_path().as_ref()).unwrap().into();
    let rustfmt_config_path: OsString =
        std::str::from_utf8(request.rustfmt_config_path().as_ref()).unwrap().into();
    let kythe_default_corpus: &str =
        std::str::from_utf8(request.kythe_default_corpus().as_ref()).unwrap();
    let generate_error_report = request.generate_error_report();
    let environment = if request.skip_source_location_in_doc_comments() {
        Environment::GoldenTest
    } else {
        Environment::Production
    };
    let kythe_annotations = request.kythe_annotations();

    catch_unwind(|| {
        let mut error_report: Option<ErrorReport> = None;
        let errors: &dyn ErrorReporting = if generate_error_report {
            error_report.insert(ErrorReport::new(SourceLanguage::Cpp))
        } else {
            &error_report::IgnoreErrors
        };
        let fatal_errors = FatalErrors::new();
        let Bindings { rs_api, rs_api_impl } = inner_generate_bindings(
            json,
            crubit_support_path_format,
            &clang_format_exe_path,
            &rustfmt_exe_path,
            &rustfmt_config_path,
            errors,
            &fatal_errors,
            environment,
            kythe_annotations,
            kythe_default_corpus,
        )
        .unwrap();

        let mut response = GenerateBindingsResponse::new();
        response.set_rs_api(rs_api);
        response.set_rs_api_impl(rs_api_impl);
        if let Some(report) = error_report {
            response.set_error_report(report.to_json_string());
        }
        response.set_fatal_errors(fatal_errors.take_string());

        response
    })
    .unwrap_or_else(|_| process::abort())
}
