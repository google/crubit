// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::matchers::eq;
use googletest::{expect_that, gtest};
#[gtest]
fn test_functions() {
    use functions::{
        call_cb, call_struct_fn_ptr, kFnPtrsAlign, kFnPtrsSize, kFnPtrs_field_fnOffset,
        kFnPtrs_typedef_fieldOffset, takes_fn_ptr, void_func, FnPtrs,
    };
    void_func();
    let cb: Option<extern "C" fn(core::ffi::c_int)> = None;
    takes_fn_ptr(cb);

    extern "C" fn my_cb(x: core::ffi::c_int) -> core::ffi::c_int {
        x + 1
    }
    extern "C" fn my_void_cb(_x: core::ffi::c_int) {}
    let res = call_cb(Some(my_cb), 41);
    expect_that!(res, eq(42));

    let mut f = FnPtrs { field_fn: None, typedef_field: None };
    f.field_fn = Some(my_void_cb);
    call_struct_fn_ptr(f, 41);

    expect_that!(core::mem::size_of::<FnPtrs>(), eq(unsafe { kFnPtrsSize }));
    expect_that!(core::mem::align_of::<FnPtrs>(), eq(unsafe { kFnPtrsAlign }));
    expect_that!(core::mem::offset_of!(FnPtrs, field_fn), eq(unsafe { kFnPtrs_field_fnOffset }));
    expect_that!(
        core::mem::offset_of!(FnPtrs, typedef_field),
        eq(unsafe { kFnPtrs_typedef_fieldOffset })
    );
}
