// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:definition_of_forward_declaration_cc

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![no_std]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=ForwardDeclaredStruct
pub struct ForwardDeclaredStruct {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for ForwardDeclaredStruct {}
impl !Sync for ForwardDeclaredStruct {}
unsafe impl ::cxx::ExternType for ForwardDeclaredStruct {
    type Id = ::cxx::type_id!("ForwardDeclaredStruct");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("ForwardDeclaredStruct"),
    crate::ForwardDeclaredStruct
);

impl Default for ForwardDeclaredStruct {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN21ForwardDeclaredStructC1Ev(
                &raw mut tmp as *mut ::core::ffi::c_void,
            );
            tmp.assume_init()
        }
    }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN21ForwardDeclaredStructC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::ForwardDeclaredStruct>() == 1);
    assert!(::core::mem::align_of::<crate::ForwardDeclaredStruct>() == 1);
    static_assertions::assert_impl_all!(crate::ForwardDeclaredStruct: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::ForwardDeclaredStruct: Drop);
};
