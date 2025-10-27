// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// aliasing_references_golden
// Features: do_not_hardcode_status_bridge, std_unique_ptr, std_vector, supported

#![allow(unused_unsafe, deprecated, non_snake_case, unreachable_code)]
#![allow(improper_ctypes_definitions)]
#![deny(warnings)]

#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_mut_urefs(
    __param_0: &'static mut i32,
    __param_1: &'static mut i32,
) -> () {
    unsafe { ::aliasing_references_golden::mut_refs(__param_0, __param_1) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_mut_uref_uand_ushared_urefs(
    __param_0: &'static mut i32,
    __param_1: &'static i32,
    __param_2: &'static i32,
) -> () {
    unsafe {
        ::aliasing_references_golden::mut_ref_and_shared_refs(__param_0, __param_1, __param_2)
    }
}
const _: () = assert!(::std::mem::size_of::<::aliasing_references_golden::SomeStruct>() == 4);
const _: () = assert!(::std::mem::align_of::<::aliasing_references_golden::SomeStruct>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_default(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value=<::aliasing_references_golden::SomeStruct as::core::prelude::rust_2015::Default>::default();
        (__ret_ptr as *mut ::aliasing_references_golden::SomeStruct).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_mut_uself_uand_umut_uref(
    __self: &'static mut ::aliasing_references_golden::SomeStruct,
    __param_1: &'static mut i32,
) -> () {
    unsafe { ::aliasing_references_golden::SomeStruct::mut_self_and_mut_ref(__self, __param_1) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_mut_uself_uand_ushared_uref(
    __self: &'static mut ::aliasing_references_golden::SomeStruct,
    __param_1: &'static i32,
) -> () {
    unsafe { ::aliasing_references_golden::SomeStruct::mut_self_and_shared_ref(__self, __param_1) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_shared_uself_uand_umut_uref(
    __self: &'static ::aliasing_references_golden::SomeStruct,
    __param_1: &'static mut i32,
) -> () {
    unsafe { ::aliasing_references_golden::SomeStruct::shared_self_and_mut_ref(__self, __param_1) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_shared_uself_uand_ushared_uref_uallows_ualias(
    __self: &'static ::aliasing_references_golden::SomeStruct,
    __param_1: &'static i32,
) -> () {
    unsafe {
        ::aliasing_references_golden::SomeStruct::shared_self_and_shared_ref_allows_alias(
            __self, __param_1,
        )
    }
}
const _: () =
    assert!(::core::mem::offset_of!(::aliasing_references_golden::SomeStruct, field) == 0);
const _: () = assert!(::std::mem::size_of::<::aliasing_references_golden::NonFreezeType>() == 4);
const _: () = assert!(::std::mem::align_of::<::aliasing_references_golden::NonFreezeType>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_default(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value=<::aliasing_references_golden::NonFreezeType as::core::prelude::rust_2015::Default>::default();
        (__ret_ptr as *mut ::aliasing_references_golden::NonFreezeType).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_as_umut_uunchecked(
    __self: &'static ::aliasing_references_golden::NonFreezeType,
) -> &'static mut i32 {
    unsafe { ::aliasing_references_golden::NonFreezeType::as_mut_unchecked(__self) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_shared_uself_umut_uref_uallows_ualias(
    __self: &'static ::aliasing_references_golden::NonFreezeType,
    __param_1: &'static mut i32,
) -> () {
    unsafe {
        ::aliasing_references_golden::NonFreezeType::shared_self_mut_ref_allows_alias(
            __self, __param_1,
        )
    }
}
