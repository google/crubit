// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use ir::ir_ffi_support::FfiStr;
use ir::ir_generated::{GenerateBindingsRequest, GenerateBindingsResponse, IRProto};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::process;

/// Generates bindings in-memory from a C++ `ffi::GenerateBindingsRequest` into a C++
/// `ffi::GenerateBindingsResponse`.
///
/// # Safety
///
/// * `request` must be a valid pointer to an initialized `GenerateBindingsRequest`.
/// * `response` must be a valid pointer to an initialized `GenerateBindingsResponse`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GenerateBindingsImpl(
    request: *mut GenerateBindingsRequest,
    response: *mut GenerateBindingsResponse,
) {
    catch_unwind(AssertUnwindSafe(|| unsafe {
        let request = std::mem::take(&mut *request);
        let response = &mut *response;
        cc_api::generate_bindings(request, response);
    }))
    .unwrap_or_else(|_| process::abort());
}

/// Formats an `IRProto` using its Rust `Debug` representation.
///
/// # Safety
///
/// * `ir` must be a valid pointer to an initialized `IRProto`.
/// * `out` must be a valid pointer to an initialized `FfiStr`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FormatIrDebugImpl(ir: *const IRProto, out: *mut FfiStr) {
    catch_unwind(AssertUnwindSafe(|| unsafe {
        *out = format!("{:#?}", *ir).into();
    }))
    .unwrap_or_else(|_| process::abort());
}
