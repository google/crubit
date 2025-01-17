// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// uses_rust_golden
// Features: experimental, supported

#![allow(improper_ctypes_definitions)]

#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_f1() -> i32 {
    ::uses_rust_golden::test_use_glob::f1()
}
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_f2() -> i32 {
    ::uses_rust_golden::test_use_glob::f2()
}
const _: () = assert!(::std::mem::size_of::<::uses_rust_golden::test_use_glob::X1>() == 4);
const _: () = assert!(::std::mem::align_of::<::uses_rust_golden::test_use_glob::X1>() == 4);
const _: () = assert!(::std::mem::size_of::<::uses_rust_golden::Bar>() == 4);
const _: () = assert!(::std::mem::align_of::<::uses_rust_golden::Bar>() == 4);
const _: () = assert!(::std::mem::size_of::<::uses_rust_golden::Foo>() == 8);
const _: () = assert!(::std::mem::align_of::<::uses_rust_golden::Foo>() == 4);
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_create(
    __ret_slot: &mut ::core::mem::MaybeUninit<::uses_rust_golden::Foo>,
) -> () {
    __ret_slot.write(::uses_rust_golden::Foo::create());
}
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_bar(
    __ret_slot: &mut ::core::mem::MaybeUninit<::uses_rust_golden::Bar>,
) -> () {
    __ret_slot.write(::uses_rust_golden::Foo::bar());
}
const _: () = assert!(::core::mem::offset_of!(::uses_rust_golden::Foo, bar) == 4);
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_g1() -> i32 {
    ::uses_rust_golden::g1()
}
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_g2() -> i32 {
    ::uses_rust_golden::g2()
}
const _: () = assert!(::std::mem::size_of::<::uses_rust_golden::InnerX>() == 4);
const _: () = assert!(::std::mem::align_of::<::uses_rust_golden::InnerX>() == 4);
const _: () = assert!(::core::mem::offset_of!(::uses_rust_golden::InnerX, field) == 0);
