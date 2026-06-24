// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// async_fn_golden

#![allow(unused_unsafe, deprecated, non_snake_case, unreachable_code)]
#![allow(improper_ctypes_definitions)]
#![deny(warnings)]

extern crate alloc;
extern crate core;
const _: () = assert!(::std::mem::size_of::<::async_fn_golden::NotCppMovable>() == 4);
const _: () = assert!(::std::mem::align_of::<::async_fn_golden::NotCppMovable>() == 4);
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_drop(
    __self: &'static mut ::core::mem::MaybeUninit<::async_fn_golden::NotCppMovable>,
) {
    unsafe { __self.assume_init_drop() };
}
const _: () = assert!(::core::mem::offset_of!(::async_fn_golden::NotCppMovable, 0) == 0);
const _: () = assert!(::std::mem::size_of::<::async_fn_golden::StructWithDrop>() == 4);
const _: () = assert!(::std::mem::align_of::<::async_fn_golden::StructWithDrop>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_default(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value =
            <::async_fn_golden::StructWithDrop as ::core::default::Default>::default();
        (__ret_ptr as *mut ::async_fn_golden::StructWithDrop).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_drop(
    __self: &'static mut ::core::mem::MaybeUninit<::async_fn_golden::StructWithDrop>,
) {
    unsafe { __self.assume_init_drop() };
}
const _: () = assert!(::core::mem::offset_of!(::async_fn_golden::StructWithDrop, field) == 0);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_add(
    x: i32,
    y: i32,
    __ret_ptr: *mut ::dyn_erased_future::DynErasedFuture<'_>,
) -> () {
    unsafe {
        ::core::ptr::write(
            __ret_ptr,
            ::dyn_erased_future::DynErasedFuture::new(::async_fn_golden::add(x, y)),
        );
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_do_unothing(
    __ret_ptr: *mut ::dyn_erased_future::DynErasedFuture<'_>,
) -> () {
    unsafe {
        ::core::ptr::write(
            __ret_ptr,
            ::dyn_erased_future::DynErasedFuture::new(::async_fn_golden::do_nothing()),
        );
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_pend_u5_utimes(
    __ret_ptr: *mut ::dyn_erased_future::DynErasedFuture<'_>,
) -> () {
    unsafe {
        ::core::ptr::write(
            __ret_ptr,
            ::dyn_erased_future::DynErasedFuture::new(::async_fn_golden::pend_5_times()),
        );
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_return_ucpp_ulayout_uequivalent(
    x: i32,
    __ret_ptr: *mut ::dyn_erased_future::DynErasedFuture<'_>,
) -> () {
    unsafe {
        ::core::ptr::write(
            __ret_ptr,
            ::dyn_erased_future::DynErasedFuture::new(
                ::async_fn_golden::return_cpp_layout_equivalent(x),
            ),
        );
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_return_ustruct_uwith_udrop(
    x: i32,
    __ret_ptr: *mut ::dyn_erased_future::DynErasedFuture<'_>,
) -> () {
    unsafe {
        ::core::ptr::write(
            __ret_ptr,
            ::dyn_erased_future::DynErasedFuture::new(::async_fn_golden::return_struct_with_drop(
                x,
            )),
        );
    }
}
