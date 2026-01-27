// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// function_annotations_rust_golden
// Features: assume_lifetimes, custom_ffi_types, experimental, non_unpin_ctor, std_unique_ptr, std_vector, supported, unhardcode_c9_co, wrapper

#![allow(unused_unsafe, deprecated, non_snake_case, unreachable_code)]
#![allow(improper_ctypes_definitions)]
#![deny(warnings)]

#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_add_utwo_uintegers(x: i32, y: i32) -> i32 {
    unsafe { ::function_annotations_rust_golden::add_two_integers(x, y) }
}
