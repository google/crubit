// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use ffi_types::{FfiU8Slice, FfiU8SliceBox};
use generate_bindings_rust_proto::{GenerateBindingsRequest, GenerateBindingsResponse};
use protobuf::{Parse, Serialize};

/// Deserializes `GenerateBindingsRequest` and generates bindings source code.
///
/// This function aborts on error.
///
/// # Safety
///
/// Expectations:
///    * `serialized_request` should be a FfiU8Slice for a valid array of bytes
///      representing a serialized `GenerateBindingsRequest` protobuf message.
///
/// Ownership:
///    * function doesn't take ownership of the input param: `serialized_request`
///    * function passes ownership of the returned value (a serialized
///      `GenerateBindingsResponse` protobuf message) to the caller
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GenerateBindingsImplFromSerializedProto(
    serialized_request: FfiU8Slice,
) -> FfiU8SliceBox {
    let request_slice = serialized_request.as_slice();
    let request = GenerateBindingsRequest::parse(request_slice)
        .expect("serialized_request should be a valid, serialized GenerateBindingsRequest proto");
    let mut response = GenerateBindingsResponse::new();
    cc_api::generate_bindings(request.as_view(), response.as_mut());
    let response_bytes =
        response.serialize().expect("GenerateBindingsResponse serialization should never fail");
    FfiU8SliceBox::from_boxed_slice(response_bytes.into_boxed_slice())
}
