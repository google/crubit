// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// must_bind_golden

#![allow(unused_unsafe, deprecated, non_snake_case, unreachable_code)]
#![allow(improper_ctypes_definitions)]
#![deny(warnings)]

extern crate alloc;
extern crate core;
const _: () = assert!(::core::mem::size_of::<::must_bind_golden::Original>() == 4);
const _: () = assert!(::core::mem::align_of::<::must_bind_golden::Original>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_new(__ret_ptr: *mut ::must_bind_golden::Original) -> () {
    unsafe {
        __ret_ptr.write(::must_bind_golden::Original::new());
    }
}
const _: () = assert!(::core::mem::offset_of!(::must_bind_golden::Original, x) == 0);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_bar() -> () {
    unsafe { ::must_bind_golden::bar() }
}
