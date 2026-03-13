// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// no_std_golden
// Features: supported, types

#![allow(unused_unsafe, deprecated, non_snake_case, unreachable_code)]
#![allow(improper_ctypes_definitions)]
#![deny(warnings)]

const _: () = assert!(::std::mem::size_of::<::no_std_golden::NoStdStruct>() == 24);
const _: () = assert!(::std::mem::align_of::<::no_std_golden::NoStdStruct>() == 8);
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_drop(
    __self: &'static mut ::core::mem::MaybeUninit<::no_std_golden::NoStdStruct>,
) {
    unsafe { __self.assume_init_drop() };
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_new(x: i32, y: f32, __ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::no_std_golden::NoStdStruct::new(x, y);
        (__ret_ptr as *mut ::no_std_golden::NoStdStruct).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_display(
    __self: &'static ::no_std_golden::NoStdStruct,
) -> &'static str {
    unsafe { ::no_std_golden::NoStdStruct::display(__self) }
}
const _: () = assert!(::core::mem::offset_of!(::no_std_golden::NoStdStruct, test) == 0);
