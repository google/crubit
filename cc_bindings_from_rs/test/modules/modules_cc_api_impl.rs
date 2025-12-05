// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// modules_golden
// Features: custom_ffi_types, non_unpin_ctor, std_unique_ptr, std_vector, supported

#![allow(unused_unsafe, deprecated, non_snake_case, unreachable_code)]
#![allow(improper_ctypes_definitions)]
#![deny(warnings)]

#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_add_ui32(x: i32, y: i32) -> i32 {
    unsafe { ::modules_golden::basic_module::add_i32(x, y) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_add_ui32(x: i32, y: i32) -> i32 {
    unsafe { ::modules_golden::deprecated_module::add_i32(x, y) }
}
const _: () =
    assert!(::std::mem::size_of::<::modules_golden::impl_in_separate_private_module::Foo>() == 4);
const _: () =
    assert!(::std::mem::align_of::<::modules_golden::impl_in_separate_private_module::Foo>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_create(i: i32, __ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::modules_golden::impl_in_separate_private_module::Foo::create(i);
        (__ret_ptr as *mut ::modules_golden::impl_in_separate_private_module::Foo)
            .write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_into_ui32(
    s: &'static mut ::core::mem::MaybeUninit<
        ::modules_golden::impl_in_separate_private_module::Foo,
    >,
) -> i32 {
    unsafe {
        let s = s.assume_init_read();
        ::modules_golden::impl_in_separate_private_module::Foo::into_i32(s)
    }
}
const _: () = assert!(
    ::core::mem::offset_of!(::modules_golden::impl_in_separate_private_module::Foo, 0) == 0
);
