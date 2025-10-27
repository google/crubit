// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// into_golden
// Features: do_not_hardcode_status_bridge, std_unique_ptr, std_vector, supported

#![allow(unused_unsafe, deprecated, non_snake_case, unreachable_code)]
#![allow(improper_ctypes_definitions)]
#![deny(warnings)]

const _: () = assert!(::std::mem::size_of::<::into_golden::Convert>() == 4);
const _: () = assert!(::std::mem::align_of::<::into_golden::Convert>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_into_ui32(
    __self: &'static mut ::core::mem::MaybeUninit<::into_golden::Convert>,
) -> i32 {
    unsafe {
        let __self = __self.assume_init_read();
        <::into_golden::Convert as ::core::prelude::rust_2015::Into<i32>>::into(__self)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_into_ui64(
    __self: &'static mut ::core::mem::MaybeUninit<::into_golden::Convert>,
) -> i64 {
    unsafe {
        let __self = __self.assume_init_read();
        <::into_golden::Convert as ::core::prelude::rust_2015::Into<i64>>::into(__self)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_into_u_x00000026_x00000027static_x00000020str(
    __self: &'static mut ::core::mem::MaybeUninit<::into_golden::Convert>,
) -> &'static str {
    unsafe {
        let __self = __self.assume_init_read();
        <::into_golden::Convert as ::core::prelude::rust_2015::Into<&'static str>>::into(__self)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_into_ui16(
    __self: &'static mut ::core::mem::MaybeUninit<::into_golden::Convert>,
) -> i16 {
    unsafe {
        let __self = __self.assume_init_read();
        <::into_golden::Convert as ::core::prelude::rust_2015::Into<i16>>::into(__self)
    }
}
const _: () = assert!(::core::mem::offset_of!(::into_golden::Convert, 0) == 0);
const _: () = assert!(::std::mem::size_of::<::into_golden::ConvertRef>() == 16);
const _: () = assert!(::std::mem::align_of::<::into_golden::ConvertRef>() == 8);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_create(
    s: &'static str,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = ::into_golden::ConvertRef::create(s);
        (__ret_ptr as *mut ::into_golden::ConvertRef<'static>).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_transmigrate(
    __self: &'static mut ::core::mem::MaybeUninit<::into_golden::ConvertRef<'static>>,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __self = __self.assume_init_read();
        let __rs_return_value = ::into_golden::ConvertRef::transmigrate(__self);
        (__ret_ptr as *mut ::into_golden::Convert).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_into_u_x00000026_x00000027a_x00000020str(
    __self: &'static mut ::core::mem::MaybeUninit<::into_golden::ConvertRef<'static>>,
) -> &'static str {
    unsafe {
        let __self = __self.assume_init_read();
        <::into_golden::ConvertRef as ::core::prelude::rust_2015::Into<&'static str>>::into(__self)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_into_uConvert(
    __self: &'static mut ::core::mem::MaybeUninit<::into_golden::ConvertRef<'static>>,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __self = __self.assume_init_read();
        let __rs_return_value = <::into_golden::ConvertRef as ::core::prelude::rust_2015::Into<
            ::into_golden::Convert,
        >>::into(__self);
        (__ret_ptr as *mut ::into_golden::Convert).write(__rs_return_value);
    }
}
const _: () = assert!(::std::mem::size_of::<::into_golden::NotFfiSafe>() == 8);
const _: () = assert!(::std::mem::align_of::<::into_golden::NotFfiSafe>() == 8);
const _: () = assert!(::std::mem::size_of::<::into_golden::ConvertModule>() == 4);
const _: () = assert!(::std::mem::align_of::<::into_golden::ConvertModule>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_into_ui32(
    __self: &'static mut ::core::mem::MaybeUninit<::into_golden::ConvertModule>,
) -> i32 {
    unsafe {
        let __self = __self.assume_init_read();
        <::into_golden::ConvertModule as ::core::prelude::rust_2015::Into<i32>>::into(__self)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_into_ui64(
    __self: &'static mut ::core::mem::MaybeUninit<::into_golden::ConvertModule>,
) -> i64 {
    unsafe {
        let __self = __self.assume_init_read();
        <::into_golden::ConvertModule as ::core::prelude::rust_2015::Into<i64>>::into(__self)
    }
}
const _: () = assert!(::core::mem::offset_of!(::into_golden::ConvertModule, 0) == 0);
