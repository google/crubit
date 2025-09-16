// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// uses_rust_golden
// Features: do_not_hardcode_status_bridge, experimental, infer_operator_lifetimes, std_unique_ptr, std_vector, supported, unsafe_types, wrapper

#![allow(unused_unsafe)]
#![allow(improper_ctypes_definitions)]

#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_f1() -> i32 {
    unsafe { ::uses_rust_golden::test_use_glob::f1() }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_f2() -> i32 {
    unsafe { ::uses_rust_golden::test_use_glob::f2() }
}
const _: () = assert!(::std::mem::size_of::<::uses_rust_golden::test_use_glob::X1>() == 4);
const _: () = assert!(::std::mem::align_of::<::uses_rust_golden::test_use_glob::X1>() == 4);
const _: () = assert!(::std::mem::size_of::<::uses_rust_golden::Bar>() == 4);
const _: () = assert!(::std::mem::align_of::<::uses_rust_golden::Bar>() == 4);
const _: () = assert!(::std::mem::size_of::<::uses_rust_golden::Foo>() == 8);
const _: () = assert!(::std::mem::align_of::<::uses_rust_golden::Foo>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_create(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::uses_rust_golden::Foo::create();
        (__ret_ptr as *mut ::uses_rust_golden::Foo).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_bar(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::uses_rust_golden::Foo::bar();
        (__ret_ptr as *mut ::uses_rust_golden::Bar).write(__rs_return_value);
    }
}
const _: () = assert!(::core::mem::offset_of!(::uses_rust_golden::Foo, bar) == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_g1() -> i32 {
    unsafe { ::uses_rust_golden::g1() }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_g2() -> i32 {
    unsafe { ::uses_rust_golden::g2() }
}
const _: () = assert!(::std::mem::size_of::<::uses_rust_golden::InnerX>() == 4);
const _: () = assert!(::std::mem::align_of::<::uses_rust_golden::InnerX>() == 4);
const _: () = assert!(::core::mem::offset_of!(::uses_rust_golden::InnerX, field) == 0);
