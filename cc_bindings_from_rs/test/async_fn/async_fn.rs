// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use std::ffi::c_void;
use std::future;
use std::rc::Rc;
use std::task::Poll;

pub async fn add(x: i32, y: i32) -> i32 {
    x + y
}

// 1. A return type which implements `Drop`
#[derive(Default)]
pub struct StructWithDrop {
    pub field: i32,
}

impl Drop for StructWithDrop {
    fn drop(&mut self) {}
}

pub async fn return_struct_with_drop(x: i32) -> StructWithDrop {
    StructWithDrop { field: x }
}

// 2. A unit output type
pub async fn do_nothing() {}

// 3. await something which is not immediately ready
pub async fn pend_5_times() -> i32 {
    let mut count = 5;
    future::poll_fn(move |cx| {
        if count == 0 {
            return Poll::Ready(42);
        }
        count -= 1;
        cx.waker().wake_by_ref();
        Poll::Pending
    })
    .await
}

// 4. Bridged return types (cpp_convertible)
// Crubit currently does not support async functions returning bridged types that require conversion thunks.
#[crubit_annotate::cpp_convertible(
    cpp_type = "crubit::test::AsyncFnCppConvertible",
    include_path = "cc_bindings_from_rs/test/async_fn/async_fn_existing_cpp_types.h",
    rust_to_cpp_converter = "convert_rust_to_cpp_async_fn",
    cpp_to_rust_converter = "convert_cpp_to_rust_async_fn"
)]
#[repr(transparent)]
pub struct AsyncFnRustConvertible {
    pub x: i32,
}

pub async fn return_bridged_convertible(x: i32) -> AsyncFnRustConvertible {
    AsyncFnRustConvertible { x }
}

/// # Safety
///
/// `rs_in` must be a valid pointer to an `AsyncFnRustConvertible` that will not be used again.
/// `cpp_out` must be a valid pointer to uninitialized memory suitable for writing a
/// `AsyncFnCppConvertible`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn convert_rust_to_cpp_async_fn(
    _rs_in: *const c_void,
    _cpp_out: *mut c_void,
) {
    unimplemented!()
}

/// # Safety
///
/// `cpp_in` must be a valid pointer to a `AsyncFnCppConvertible` that will not be used again.
/// `rs_out` must be a valid pointer to uninitialized memory suitable for writing a
/// `AsyncFnRustConvertible`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn convert_cpp_to_rust_async_fn(
    _cpp_in: *const c_void,
    _rs_out: *mut c_void,
) {
    unimplemented!()
}

// 4b. Types that have existing C++ types (cpp_layout_equivalent)
// These are layout-compatible with an existing C++ type, so no conversion thunks are needed.
// Works perfectly for async functions.
#[crubit_annotate::cpp_layout_equivalent(
    cpp_type = "crubit::test::AsyncFnCppLayoutEquivalent",
    include_path = "cc_bindings_from_rs/test/async_fn/async_fn_existing_cpp_types.h"
)]
#[repr(C)]
pub struct AsyncFnRustLayoutEquivalent {
    pub x: i32,
}

pub async fn return_cpp_layout_equivalent(x: i32) -> AsyncFnRustLayoutEquivalent {
    AsyncFnRustLayoutEquivalent { x }
}

// 5. return types which are not C++-movable
// Implements Drop but not Default, so C++ move operations are unavailable.
// Works for non-async fn (via crubit::Slot), but is correctly rejected for async fn since the
// future outputs by value.
#[repr(transparent)]
pub struct NotCppMovable(pub i32);

impl Drop for NotCppMovable {
    fn drop(&mut self) {}
}

pub async fn return_unmovable(x: i32) -> NotCppMovable {
    NotCppMovable(x)
}

// 6. non-`async fn` which returns an `impl Future`
pub fn return_impl_future(x: i32) -> impl std::future::Future<Output = i32> {
    std::future::ready(x)
}

// 7. non-`async fn` which returns a `Box<dyn Future>`
pub fn return_box_dyn_future(x: i32) -> Box<dyn std::future::Future<Output = i32>> {
    Box::new(future::ready(x))
}

// 8. a non-Send return type
pub async fn non_send_return() -> i32 {
    let rc = Rc::new(42);
    future::pending::<()>().await;
    *rc
}
