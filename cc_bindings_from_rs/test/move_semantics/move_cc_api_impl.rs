// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// move_golden
// Features: non_unpin_ctor, std_unique_ptr, std_vector, supported

#![allow(unused_unsafe, deprecated, non_snake_case, unreachable_code)]
#![allow(improper_ctypes_definitions)]
#![deny(warnings)]

const _: () = assert!(::std::mem::size_of::<::move_golden::Copyable>() == 1);
const _: () = assert!(::std::mem::align_of::<::move_golden::Copyable>() == 1);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_default(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = <::move_golden::Copyable as ::core::default::Default>::default();
        (__ret_ptr as *mut ::move_golden::Copyable).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_from_ubyte(byte: u8, __ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::move_golden::Copyable::from_byte(byte);
        (__ret_ptr as *mut ::move_golden::Copyable).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_consume_uself(
    __self: &'static mut ::core::mem::MaybeUninit<::move_golden::Copyable>,
) -> u8 {
    unsafe {
        let __self = __self.assume_init_read();
        ::move_golden::Copyable::consume_self(__self)
    }
}
const _: () = assert!(::core::mem::offset_of!(::move_golden::Copyable, field) == 0);
const _: () = assert!(::std::mem::size_of::<::move_golden::Foo>() == 8);
const _: () = assert!(::std::mem::align_of::<::move_golden::Foo>() == 8);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_default(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = <::move_golden::Foo as ::core::default::Default>::default();
        (__ret_ptr as *mut ::move_golden::Foo).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_drop(
    __self: &'static mut ::core::mem::MaybeUninit<::move_golden::Foo>,
) {
    unsafe { __self.assume_init_drop() };
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_from_ubyte(byte: u8, __ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::move_golden::Foo::from_byte(byte);
        (__ret_ptr as *mut ::move_golden::Foo).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_read_ubyte(__self: &'static ::move_golden::Foo) -> u8 {
    unsafe { ::move_golden::Foo::read_byte(__self) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_into_ubyte(
    __self: &'static mut ::core::mem::MaybeUninit<::move_golden::Foo>,
) -> u8 {
    unsafe {
        let __self = __self.assume_init_read();
        ::move_golden::Foo::into_byte(__self)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_consume_ufoo(
    _foo: &'static mut ::core::mem::MaybeUninit<::move_golden::Foo>,
) -> () {
    unsafe {
        let _foo = _foo.assume_init_read();
        ::move_golden::consume_foo(_foo)
    }
}
