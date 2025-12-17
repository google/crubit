// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// rs_default_golden
// Features: non_unpin_ctor, std_unique_ptr, std_vector, supported

#![allow(unused_unsafe, deprecated, non_snake_case, unreachable_code)]
#![allow(improper_ctypes_definitions)]
#![deny(warnings)]

const _: () = assert!(
    ::std::mem::size_of::<::rs_default_golden::field_with_no_default::StructWithoutDefault>() == 4
);
const _: () = assert!(
    ::std::mem::align_of::<::rs_default_golden::field_with_no_default::StructWithoutDefault>() == 4
);
const _: () = assert!(::std::mem::size_of::<::rs_default_golden::no_impl::SomeStruct>() == 4);
const _: () = assert!(::std::mem::align_of::<::rs_default_golden::no_impl::SomeStruct>() == 4);
const _: () = assert!(::std::mem::size_of::<::rs_default_golden::derived_impl::SomeStruct>() == 4);
const _: () = assert!(::std::mem::align_of::<::rs_default_golden::derived_impl::SomeStruct>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_default(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value =
            <::rs_default_golden::derived_impl::SomeStruct as ::core::default::Default>::default();
        (__ret_ptr as *mut ::rs_default_golden::derived_impl::SomeStruct).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_extract_uint(
    s: &'static mut ::core::mem::MaybeUninit<::rs_default_golden::derived_impl::SomeStruct>,
) -> i32 {
    unsafe {
        let s = s.assume_init_read();
        ::rs_default_golden::derived_impl::SomeStruct::extract_int(s)
    }
}
const _: () =
    assert!(::std::mem::size_of::<::rs_default_golden::transparent_struct::SomeStruct>() == 4);
const _: () =
    assert!(::std::mem::align_of::<::rs_default_golden::transparent_struct::SomeStruct>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_default() -> ::rs_default_golden::transparent_struct::SomeStruct
{
    unsafe {
        <::rs_default_golden::transparent_struct::SomeStruct as ::core::default::Default>::default()
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_extract_uint(
    __self: &'static ::rs_default_golden::transparent_struct::SomeStruct,
) -> i32 {
    unsafe { ::rs_default_golden::transparent_struct::SomeStruct::extract_int(__self) }
}
const _: () = assert!(
    ::std::mem::size_of::<::rs_default_golden::field_with_no_default::StructWithFieldWithNoDefault>(
    ) == 4
);
const _: () = assert!(
    ::std::mem::align_of::<::rs_default_golden::field_with_no_default::StructWithFieldWithNoDefault>(
    ) == 4
);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_default(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value=<::rs_default_golden::field_with_no_default::StructWithFieldWithNoDefault as::core::default::Default>::default();
        (__ret_ptr
            as *mut ::rs_default_golden::field_with_no_default::StructWithFieldWithNoDefault)
            .write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_extract_uint(
    s: &'static mut ::core::mem::MaybeUninit<
        ::rs_default_golden::field_with_no_default::StructWithFieldWithNoDefault,
    >,
) -> i32 {
    unsafe {
        let s = s.assume_init_read();
        ::rs_default_golden::field_with_no_default::StructWithFieldWithNoDefault::extract_int(s)
    }
}
const _: () = assert!(::std::mem::size_of::<::rs_default_golden::explicit_impl::SomeStruct>() == 4);
const _: () =
    assert!(::std::mem::align_of::<::rs_default_golden::explicit_impl::SomeStruct>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_default(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value =
            <::rs_default_golden::explicit_impl::SomeStruct as ::core::default::Default>::default();
        (__ret_ptr as *mut ::rs_default_golden::explicit_impl::SomeStruct).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_extract_uint(
    s: &'static mut ::core::mem::MaybeUninit<::rs_default_golden::explicit_impl::SomeStruct>,
) -> i32 {
    unsafe {
        let s = s.assume_init_read();
        ::rs_default_golden::explicit_impl::SomeStruct::extract_int(s)
    }
}
