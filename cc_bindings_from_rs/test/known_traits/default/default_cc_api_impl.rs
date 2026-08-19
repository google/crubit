// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// rs_default_golden

#![allow(unused_unsafe, deprecated, non_snake_case, unreachable_code)]
#![allow(improper_ctypes_definitions)]
#![deny(warnings)]

extern crate alloc;
extern crate core;
const _: () = assert!(::std::mem::size_of::<::rs_default_golden::derived_impl::SomeStruct>() == 4);
const _: () = assert!(::std::mem::align_of::<::rs_default_golden::derived_impl::SomeStruct>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Default_udefault_urs_udefault_ugolden_x0000003a_x0000003aderived_uimpl_x0000003a_x0000003aSomeStruct(
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value =
            <::rs_default_golden::derived_impl::SomeStruct as ::core::default::Default>::default();
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_extract_uint(
    s: *mut ::rs_default_golden::derived_impl::SomeStruct,
) -> i32 {
    unsafe {
        let s = s.read();
        ::rs_default_golden::derived_impl::SomeStruct::extract_int(s)
    }
}
const _: () =
    assert!(::core::mem::offset_of!(::rs_default_golden::derived_impl::SomeStruct, field) == 0);
const _: () = assert!(::std::mem::size_of::<::rs_default_golden::explicit_impl::SomeStruct>() == 4);
const _: () =
    assert!(::std::mem::align_of::<::rs_default_golden::explicit_impl::SomeStruct>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Default_udefault_urs_udefault_ugolden_x0000003a_x0000003aexplicit_uimpl_x0000003a_x0000003aSomeStruct(
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value =
            <::rs_default_golden::explicit_impl::SomeStruct as ::core::default::Default>::default();
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_extract_uint(
    s: *mut ::rs_default_golden::explicit_impl::SomeStruct,
) -> i32 {
    unsafe {
        let s = s.read();
        ::rs_default_golden::explicit_impl::SomeStruct::extract_int(s)
    }
}
const _: () =
    assert!(::core::mem::offset_of!(::rs_default_golden::explicit_impl::SomeStruct, field) == 0);
const _: () = assert!(
    ::std::mem::size_of::<::rs_default_golden::field_with_no_default::StructWithFieldWithNoDefault>(
    ) == 4
);
const _: () = assert!(
    ::std::mem::align_of::<::rs_default_golden::field_with_no_default::StructWithFieldWithNoDefault>(
    ) == 4
);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Default_udefault_urs_udefault_ugolden_x0000003a_x0000003afield_uwith_uno_udefault_x0000003a_x0000003aStructWithFieldWithNoDefault(
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value=<::rs_default_golden::field_with_no_default::StructWithFieldWithNoDefault as::core::default::Default>::default();
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_extract_uint(
    s: *mut ::rs_default_golden::field_with_no_default::StructWithFieldWithNoDefault,
) -> i32 {
    unsafe {
        let s = s.read();
        ::rs_default_golden::field_with_no_default::StructWithFieldWithNoDefault::extract_int(s)
    }
}
const _: () = assert!(
    ::core::mem::offset_of!(
        ::rs_default_golden::field_with_no_default::StructWithFieldWithNoDefault,
        field
    ) == 0
);
const _: () = assert!(
    ::std::mem::size_of::<::rs_default_golden::field_with_no_default::StructWithoutDefault>() == 4
);
const _: () = assert!(
    ::std::mem::align_of::<::rs_default_golden::field_with_no_default::StructWithoutDefault>() == 4
);
const _: () = assert!(::std::mem::size_of::<::rs_default_golden::no_impl::SomeStruct>() == 4);
const _: () = assert!(::std::mem::align_of::<::rs_default_golden::no_impl::SomeStruct>() == 4);
const _: () =
    assert!(::core::mem::offset_of!(::rs_default_golden::no_impl::SomeStruct, field) == 0);
const _: () =
    assert!(::std::mem::size_of::<::rs_default_golden::transparent_struct::SomeStruct>() == 4);
const _: () =
    assert!(::std::mem::align_of::<::rs_default_golden::transparent_struct::SomeStruct>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_extract_uint(
    __self: &'static ::rs_default_golden::transparent_struct::SomeStruct,
) -> i32 {
    unsafe { ::rs_default_golden::transparent_struct::SomeStruct::extract_int(__self) }
}
const _: () =
    assert!(::core::mem::offset_of!(::rs_default_golden::transparent_struct::SomeStruct, 0) == 0);
