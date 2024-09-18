// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// uses_rust
// Features: experimental, supported

#![allow(improper_ctypes_definitions)]

#[no_mangle]
extern "C" fn __crubit_thunk_f1() -> i32 {
    ::uses_rust::test_use_glob::f1()
}
#[no_mangle]
extern "C" fn __crubit_thunk_f2() -> i32 {
    ::uses_rust::test_use_glob::f2()
}
const _: () = assert!(::std::mem::size_of::<::uses_rust::test_use_glob::X1>() == 4);
const _: () = assert!(::std::mem::align_of::<::uses_rust::test_use_glob::X1>() == 4);
const _: () = assert!(::std::mem::size_of::<::uses_rust::Bar>() == 4);
const _: () = assert!(::std::mem::align_of::<::uses_rust::Bar>() == 4);
const _: () = assert!(::std::mem::size_of::<::uses_rust::Foo>() == 8);
const _: () = assert!(::std::mem::align_of::<::uses_rust::Foo>() == 4);
#[no_mangle]
extern "C" fn __crubit_thunk_create(
    __ret_slot: &mut ::core::mem::MaybeUninit<::uses_rust::Foo>,
) -> () {
    __ret_slot.write(::uses_rust::Foo::create());
}
#[no_mangle]
extern "C" fn __crubit_thunk_bar(
    __ret_slot: &mut ::core::mem::MaybeUninit<::uses_rust::Bar>,
) -> () {
    __ret_slot.write(::uses_rust::Foo::bar());
}
const _: () = assert!(::core::mem::offset_of!(::uses_rust::Foo, bar) == 4);
#[no_mangle]
extern "C" fn __crubit_thunk_g1() -> i32 {
    ::uses_rust::g1()
}
#[no_mangle]
extern "C" fn __crubit_thunk_g2() -> i32 {
    ::uses_rust::g2()
}
const _: () = assert!(::std::mem::size_of::<::uses_rust::InnerX>() == 4);
const _: () = assert!(::std::mem::align_of::<::uses_rust::InnerX>() == 4);
const _: () = assert!(::core::mem::offset_of!(::uses_rust::InnerX, field) == 0);
