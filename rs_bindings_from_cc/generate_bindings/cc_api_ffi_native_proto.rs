// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use generate_bindings_rust_proto::{GenerateBindingsRequest, GenerateBindingsResponseMut};
use protobuf::{MessageMutInterop, OwnedMessageInterop};

/// Generates bindings, transferring ownership of the request but writing the response in-place.
///
/// # Safety
///
/// * `raw_request` must be a valid pointer to a heap-allocated C++ `GenerateBindingsRequest` proto
///   message. The function takes ownership of this message and is responsible for destroying it.
/// * `raw_response` must be a valid pointer to an initialized C++ `GenerateBindingsResponse`
///   proto message.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GenerateBindingsImplFromNativeProto(
    raw_request: *mut std::ffi::c_void,
    mut raw_response: *mut std::ffi::c_void,
) {
    unsafe {
        let request =
            GenerateBindingsRequest::__unstable_take_ownership_of_raw_message(raw_request);
        let response =
            GenerateBindingsResponseMut::__unstable_wrap_raw_message_mut(&mut raw_response);
        cc_api::generate_bindings(request, response);
    }
}
