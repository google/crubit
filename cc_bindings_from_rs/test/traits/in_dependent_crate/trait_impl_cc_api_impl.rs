// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// trait_impl_golden
// Features: custom_ffi_types, non_unpin_ctor, std_unique_ptr, std_vector, supported

#![allow(unused_unsafe, deprecated, non_snake_case, unreachable_code)]
#![allow(improper_ctypes_definitions)]
#![deny(warnings)]

const _: () = assert!(::std::mem::size_of::<::trait_impl_golden::MyStruct>() == 4);
const _: () = assert!(::std::mem::align_of::<::trait_impl_golden::MyStruct>() == 4);
const _: () = assert!(::core::mem::offset_of!(::trait_impl_golden::MyStruct, x) == 0);
const _: () = assert!(::std::mem::size_of::<::trait_impl_golden::NotImplemented>() == 24);
const _: () = assert!(::std::mem::align_of::<::trait_impl_golden::NotImplemented>() == 8);
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_drop(
    __self: &'static mut ::core::mem::MaybeUninit<::trait_impl_golden::NotImplemented>,
) {
    unsafe { __self.assume_init_drop() };
}
const _: () = assert!(::core::mem::offset_of!(::trait_impl_golden::NotImplemented, foo) == 0);
