// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// trait_definition_golden
// Features: custom_ffi_types, non_unpin_ctor, std_unique_ptr, std_vector, supported

#![allow(unused_unsafe, deprecated, non_snake_case, unreachable_code)]
#![allow(improper_ctypes_definitions)]
#![deny(warnings)]

const _: () = assert!(::std::mem::size_of::<::trait_definition_golden::MyStruct>() == 4);
const _: () = assert!(::std::mem::align_of::<::trait_definition_golden::MyStruct>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_MyTrait_udo_usomething(
    __self: &'static ::trait_definition_golden::MyStruct,
) -> i32 {
    unsafe {
        <::trait_definition_golden::MyStruct as ::trait_definition_golden::MyTrait>::do_something(
            __self,
        )
    }
}
