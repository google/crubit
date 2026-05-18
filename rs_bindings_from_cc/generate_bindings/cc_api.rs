// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use database::code_snippet::Bindings;
use error_report::{ErrorReport, ErrorReporting, FatalErrors, SourceLanguage};
use generate_bindings::generate_bindings;
use generate_bindings_rust_proto::{GenerateBindingsRequestView, GenerateBindingsResponseMut};
use protobuf::{MessageMutInterop, MessageViewInterop};
use std::ffi::{c_void, OsString};
use std::panic::catch_unwind;
use std::process;
use std::sync::Once;

/// Deserializes IR from `request` and generates bindings source code
/// into the `response` object.
pub fn generate_bindings_impl(
    request: GenerateBindingsRequestView<'_>,
    mut response: GenerateBindingsResponseMut<'_>,
) {

    let json: &[u8] = request.json().as_bytes();
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
    let generate_error_report: bool = request.generate_error_report();
    let skip_source_location_in_doc_comments: bool = request.skip_source_location_in_doc_comments();
    let kythe_annotations: bool = request.kythe_annotations();

    let mut error_report: Option<ErrorReport> = None;
    let errors: &dyn ErrorReporting = if generate_error_report {
        error_report.insert(ErrorReport::new(SourceLanguage::Cpp))
    } else {
        &error_report::IgnoreErrors
    };
    let fatal_errors = FatalErrors::new();
    let Bindings { rs_api, rs_api_impl } = generate_bindings(
        json,
        crubit_support_path_format,
        &clang_format_exe_path,
        &rustfmt_exe_path,
        &rustfmt_config_path,
        errors,
        &fatal_errors,
        skip_source_location_in_doc_comments,
        kythe_annotations,
        kythe_default_corpus,
    )
    .unwrap();

    response.set_rs_api(rs_api);
    response.set_rs_api_impl(rs_api_impl);

    if let Some(err) = error_report.map(|s| s.to_json_string()) {
        response.set_error_report(err);
    }

    response.set_fatal_errors(fatal_errors.take_string());
}

/// C API for `generate_bindings_impl`.
///
/// # Safety
///
/// Expectations:
///     * `raw_request` must point to a valid C++ `GenerateBindingsRequest` object.
///     * `raw_response` must point to a valid C++ `GenerateBindingsResponse` object.
///
/// Ownership:
///     * The function does not take ownership of the `raw_request` and `raw_response` pointers.
///     * The caller is responsible for their memory.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GenerateBindingsImpl(
    raw_request: *const c_void,
    mut raw_response: *mut c_void,
) {
    catch_unwind(std::panic::AssertUnwindSafe(|| {
        let request =
            unsafe { GenerateBindingsRequestView::__unstable_wrap_raw_message(&raw_request) };
        let response = unsafe {
            GenerateBindingsResponseMut::__unstable_wrap_raw_message_mut(&mut raw_response)
        };
        generate_bindings_impl(request, response);
    }))
    .unwrap_or_else(|_| process::abort());
}
