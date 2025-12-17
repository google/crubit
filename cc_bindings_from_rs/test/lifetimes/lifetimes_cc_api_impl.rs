// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// lifetimes_golden
// Features: non_unpin_ctor, std_unique_ptr, std_vector, supported

#![allow(unused_unsafe, deprecated, non_snake_case, unreachable_code)]
#![allow(improper_ctypes_definitions)]
#![deny(warnings)]

const _: () = assert!(::std::mem::size_of::<::lifetimes_golden::StructWithLifetime>() == 8);
const _: () = assert!(::std::mem::align_of::<::lifetimes_golden::StructWithLifetime>() == 8);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_from_uref(
    field_with_lifetime: &'static i32,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value =
            ::lifetimes_golden::StructWithLifetime::from_ref(field_with_lifetime);
        (__ret_ptr as *mut ::lifetimes_golden::StructWithLifetime<'static>)
            .write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_into_uref(
    __self: &'static mut ::core::mem::MaybeUninit<::lifetimes_golden::StructWithLifetime<'static>>,
) -> &'static i32 {
    unsafe {
        let __self = __self.assume_init_read();
        ::lifetimes_golden::StructWithLifetime::into_ref(__self)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_value(
    __self: &'static mut ::core::mem::MaybeUninit<::lifetimes_golden::StructWithLifetime<'static>>,
) -> i32 {
    unsafe {
        let __self = __self.assume_init_read();
        ::lifetimes_golden::StructWithLifetime::value(__self)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_borrow_ufrom_uself(
    __self: &'static ::lifetimes_golden::StructWithLifetime<'static>,
) -> &'static i32 {
    unsafe { ::lifetimes_golden::StructWithLifetime::borrow_from_self(__self) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_make_ustatic_u42(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::lifetimes_golden::StructWithLifetime::make_static_42();
        (__ret_ptr as *mut ::lifetimes_golden::StructWithLifetime<'static>)
            .write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_from_ustatic_uref(
    field_with_lifetime: &'static i32,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value =
            ::lifetimes_golden::StructWithLifetime::from_static_ref(field_with_lifetime);
        (__ret_ptr as *mut ::lifetimes_golden::StructWithLifetime<'static>)
            .write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_from_ustatic_uref_uwhere_ubound(
    field_with_lifetime: &'static i32,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = ::lifetimes_golden::StructWithLifetime::from_static_ref_where_bound(
            field_with_lifetime,
        );
        (__ret_ptr as *mut ::lifetimes_golden::StructWithLifetime<'static>)
            .write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_borrow_ufrom_ustatic_uself(
    __self: &'static ::lifetimes_golden::StructWithLifetime<'static>,
) -> &'static i32 {
    unsafe { ::lifetimes_golden::StructWithLifetime::borrow_from_static_self(__self) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_into_u_x00000026_x00000027a_x00000020i32(
    __self: &'static mut ::core::mem::MaybeUninit<::lifetimes_golden::StructWithLifetime<'static>>,
) -> &'static i32 {
    unsafe {
        let __self = __self.assume_init_read();
        <::lifetimes_golden::StructWithLifetime as ::core::convert::Into<&'static i32>>::into(
            __self,
        )
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_into_ui32(
    __self: &'static mut ::core::mem::MaybeUninit<::lifetimes_golden::StructWithLifetime<'static>>,
) -> i32 {
    unsafe {
        let __self = __self.assume_init_read();
        <::lifetimes_golden::StructWithLifetime as ::core::convert::Into<i32>>::into(__self)
    }
}
const _: () = assert!(
    ::core::mem::offset_of!(::lifetimes_golden::StructWithLifetime, field_with_lifetime) == 0
);
const _: () =
    assert!(::std::mem::size_of::<::lifetimes_golden::StructWithLifetimeAndDropGlue>() == 32);
const _: () =
    assert!(::std::mem::align_of::<::lifetimes_golden::StructWithLifetimeAndDropGlue>() == 8);
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_drop(
    __self: &'static mut ::core::mem::MaybeUninit<
        ::lifetimes_golden::StructWithLifetimeAndDropGlue,
    >,
) {
    unsafe { __self.assume_init_drop() };
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_make_ustatic_u42(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::lifetimes_golden::StructWithLifetimeAndDropGlue::make_static_42();
        (__ret_ptr as *mut ::lifetimes_golden::StructWithLifetimeAndDropGlue<'static>)
            .write(__rs_return_value);
    }
}
const _: () = assert!(
    ::core::mem::offset_of!(
        ::lifetimes_golden::StructWithLifetimeAndDropGlue,
        field_with_drop_glue
    ) == 0
);
const _: () = assert!(
    ::core::mem::offset_of!(::lifetimes_golden::StructWithLifetimeAndDropGlue, field_with_lifetime)
        == 24
);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_function_uwith_utrivial_uunnamed_ulifetime_uparam(
    __param_0: &'static i32,
) -> () {
    unsafe { ::lifetimes_golden::function_with_trivial_unnamed_lifetime_param(__param_0) }
}
