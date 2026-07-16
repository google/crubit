// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// return_position_impl_trait_golden

#![allow(unused_unsafe, deprecated, non_snake_case, unreachable_code)]
#![allow(improper_ctypes_definitions)]
#![deny(warnings)]

extern crate alloc;
extern crate core;
const _: () =
    assert!(::std::mem::size_of::<::return_position_impl_trait_golden::ArcWrapper>() == 8);
const _: () =
    assert!(::std::mem::align_of::<::return_position_impl_trait_golden::ArcWrapper>() == 8);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Default_udefault_ureturn_uposition_uimpl_utrait_ugolden_x0000003a_x0000003aArcWrapper(
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value =
            <::return_position_impl_trait_golden::ArcWrapper as ::core::default::Default>::default(
            );
        (__ret_ptr as *mut ::return_position_impl_trait_golden::ArcWrapper)
            .write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_Drop_udrop_ureturn_uposition_uimpl_utrait_ugolden_x0000003a_x0000003aArcWrapper(
    __self: *mut ::return_position_impl_trait_golden::ArcWrapper,
) {
    unsafe { ::core::ptr::drop_in_place(__self) };
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Clone_uclone_ureturn_uposition_uimpl_utrait_ugolden_x0000003a_x0000003aArcWrapper(
    __self: &'static ::return_position_impl_trait_golden::ArcWrapper,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value =
            <::return_position_impl_trait_golden::ArcWrapper as ::core::clone::Clone>::clone(
                __self,
            );
        (__ret_ptr as *mut ::return_position_impl_trait_golden::ArcWrapper)
            .write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Clone_uclone_ufrom_ureturn_uposition_uimpl_utrait_ugolden_x0000003a_x0000003aArcWrapper(
    __self: &'static mut ::return_position_impl_trait_golden::ArcWrapper,
    source: &'static ::return_position_impl_trait_golden::ArcWrapper,
) -> () {
    unsafe {
        <::return_position_impl_trait_golden::ArcWrapper as ::core::clone::Clone>::clone_from(
            __self, source,
        )
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_refcount(
    __self: &'static ::return_position_impl_trait_golden::ArcWrapper,
) -> usize {
    unsafe { ::return_position_impl_trait_golden::ArcWrapper::refcount(__self) }
}
