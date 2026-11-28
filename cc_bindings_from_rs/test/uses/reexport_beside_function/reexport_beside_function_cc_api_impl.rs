// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// reexport_beside_function_golden

#![allow(unused_unsafe, deprecated, non_snake_case, unreachable_code)]
#![allow(improper_ctypes_definitions)]
#![deny(warnings)]

extern crate alloc;
extern crate core;
const _: () = assert!(::core::mem::size_of::<::reexport_beside_function_golden::Braced>() == 4);
const _: () = assert!(::core::mem::align_of::<::reexport_beside_function_golden::Braced>() == 4);
const _: () =
    assert!(::core::mem::offset_of!(::reexport_beside_function_golden::Braced, value) == 0);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Braced() -> i32 {
    unsafe { ::reexport_beside_function_golden::Braced() }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Local() -> i32 {
    unsafe { ::reexport_beside_function_golden::Local() }
}
