// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use core::ffi::c_void;
use core::mem::{align_of, offset_of, size_of};
use googletest::matchers::eq;
use googletest::{expect_that, gtest};

extern "C" fn my_callback(arg: *mut c_void) {
    unsafe {
        let val = &mut *(arg as *mut i32);
        *val += 1;
    }
}

#[gtest]
fn test_pointers() {
    use pointers::{get_func_pointers, invoke_callback, FuncPointers, Pointers};
    use pointers::{
        kFuncPointersAlign, kFuncPointersSize, kFuncPointers_addOffset,
        kFuncPointers_callbackOffset, kFuncPointers_callback_argOffset, kPointersAlign,
        kPointersSize, kPointers_p1Offset, kPointers_p2Offset, kPointers_p3Offset,
        kPointers_p4Offset, kPointers_p5Offset,
    };

    expect_that!(size_of::<Pointers>(), eq(unsafe { kPointersSize }));
    expect_that!(align_of::<Pointers>(), eq(unsafe { kPointersAlign }));
    expect_that!(offset_of!(Pointers, p1), eq(unsafe { kPointers_p1Offset }));
    expect_that!(offset_of!(Pointers, p2), eq(unsafe { kPointers_p2Offset }));
    expect_that!(offset_of!(Pointers, p3), eq(unsafe { kPointers_p3Offset }));
    expect_that!(offset_of!(Pointers, p4), eq(unsafe { kPointers_p4Offset }));
    expect_that!(offset_of!(Pointers, p5), eq(unsafe { kPointers_p5Offset }));

    expect_that!(size_of::<FuncPointers>(), eq(unsafe { kFuncPointersSize }));
    expect_that!(align_of::<FuncPointers>(), eq(unsafe { kFuncPointersAlign }));
    expect_that!(offset_of!(FuncPointers, add), eq(unsafe { kFuncPointers_addOffset }));
    expect_that!(offset_of!(FuncPointers, callback), eq(unsafe { kFuncPointers_callbackOffset }));
    expect_that!(
        offset_of!(FuncPointers, callback_arg),
        eq(unsafe { kFuncPointers_callback_argOffset })
    );

    let mut fp = get_func_pointers();

    // Call C function pointer from Rust
    if let Some(add_func) = fp.add {
        let result = add_func(10, 20);
        expect_that!(result, eq(30));
    } else {
        panic!("fp.add is null");
    }

    // Pass Rust extern "C" callback to C
    let mut val: i32 = 0;
    fp.callback = Some(my_callback);
    fp.callback_arg = &mut val as *mut i32 as *mut c_void;

    let res = unsafe { invoke_callback(&mut fp) };
    expect_that!(res, eq(1));
    expect_that!(val, eq(1));
}
