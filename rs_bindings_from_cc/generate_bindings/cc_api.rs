// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use database::code_snippet::{Bindings, FfiBindings};
use database::db::FatalErrors;
use error_report::{ErrorReport, ErrorReporting};
use ffi_types::{FfiU8Slice, FfiU8SliceBox, SourceLocationDocComment};
use generate_bindings::generate_bindings;
use std::ffi::OsString;
use std::panic::catch_unwind;
use std::process;

/// Deserializes IR from `json` and generates bindings source code.
///
/// This function aborts on error.
///
/// # Safety
///
/// Expectations:
///    * `json` should be a FfiU8Slice for a valid array of bytes with the given
///      size.
///    * `crubit_support_path_format` should be a FfiU8Slice for a valid array
///      of bytes representing an UTF8-encoded string
///    * `rustfmt_exe_path` and `rustfmt_config_path` should both be a
///      FfiU8Slice for a valid array of bytes representing an UTF8-encoded
///      string (without the UTF-8 requirement, it seems that Rust doesn't offer
///      a way to convert to OsString on Windows)
///    * `json`, `crubit_support_path_format`, `rustfmt_exe_path`, and
///      `rustfmt_config_path` shouldn't change during the call.
///
/// Ownership:
///    * function doesn't take ownership of (in other words it borrows) the
///      input params: `json`, `crubit_support_path_format`, `rustfmt_exe_path`,
///      and `rustfmt_config_path`
///    * function passes ownership of the returned value to the caller
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GenerateBindingsImpl(
    json: FfiU8Slice,
    crubit_support_path_format: FfiU8Slice,
    clang_format_exe_path: FfiU8Slice,
    rustfmt_exe_path: FfiU8Slice,
    rustfmt_config_path: FfiU8Slice,
    generate_error_report: bool,
    generate_source_loc_doc_comment: SourceLocationDocComment,
) -> FfiBindings {
    let json: &[u8] = json.as_slice();
    let crubit_support_path_format: &str =
        std::str::from_utf8(crubit_support_path_format.as_slice()).unwrap();
    let clang_format_exe_path: OsString =
        std::str::from_utf8(clang_format_exe_path.as_slice()).unwrap().into();
    let rustfmt_exe_path: OsString =
        std::str::from_utf8(rustfmt_exe_path.as_slice()).unwrap().into();
    let rustfmt_config_path: OsString =
        std::str::from_utf8(rustfmt_config_path.as_slice()).unwrap().into();
    catch_unwind(|| {
        let error_report: Option<ErrorReport>;
        let errors: &dyn ErrorReporting = if generate_error_report {
            error_report = Some(ErrorReport::new());
            error_report.as_ref().unwrap()
        } else {
            error_report = None;
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
            generate_source_loc_doc_comment,
        )
        .unwrap();
        FfiBindings {
            rs_api: FfiU8SliceBox::from_boxed_slice(rs_api.into_bytes().into_boxed_slice()),
            rs_api_impl: FfiU8SliceBox::from_boxed_slice(
                rs_api_impl.into_bytes().into_boxed_slice(),
            ),
            error_report: FfiU8SliceBox::from_boxed_slice(
                error_report
                    .map(|s| s.to_json_string().into_bytes().into_boxed_slice())
                    .unwrap_or_else(|| Box::new([])),
            ),
            fatal_errors: FfiU8SliceBox::from_boxed_slice(
                fatal_errors.take_string().into_bytes().into_boxed_slice(),
            ),
        }
    })
    .unwrap_or_else(|_| process::abort())
}
