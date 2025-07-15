// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/templates/type_alias_access_rule:type_alias_access_rule
// Features: experimental, infer_operator_lifetimes, supported, unsafe_types, wrapper

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![no_std]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

// Generated from: rs_bindings_from_cc/test/templates/type_alias_access_rule/type_alias_access_rule.h;l=10
// Error while generating bindings for class 'A':
// Class templates are not supported yet

/// Generated from: rs_bindings_from_cc/test/templates/type_alias_access_rule/type_alias_access_rule.h;l=13
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=B
pub struct B {
    /// Reason for representing this field as a blob of bytes:
    /// Can't generate bindings for A<B::PrivateMember> due to missing bindings for its dependency: Unsupported type 'struct B::PrivateMember': No generated bindings found for 'PrivateMember'
    pub(crate) a_: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for B {}
impl !Sync for B {}
unsafe impl ::cxx::ExternType for B {
    type Id = ::cxx::type_id!("B");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(forward_declare::symbol!("B"), crate::B);

/// Generated from: rs_bindings_from_cc/test/templates/type_alias_access_rule/type_alias_access_rule.h;l=13
impl Default for B {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN1BC1Ev(&raw mut tmp as *mut ::core::ffi::c_void);
            tmp.assume_init()
        }
    }
}

// Generated from: rs_bindings_from_cc/test/templates/type_alias_access_rule/type_alias_access_rule.h;l=11
// Error while generating bindings for class 'A<B::PrivateMember>':
// Can't generate bindings for A<B::PrivateMember> due to missing bindings for its dependency: Unsupported type 'struct B::PrivateMember': No generated bindings found for 'PrivateMember'

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN1BC1Ev(__this: *mut ::core::ffi::c_void);
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::B>() == 1);
    assert!(::core::mem::align_of::<crate::B>() == 1);
    static_assertions::assert_impl_all!(crate::B: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::B: Drop);
    assert!(::core::mem::offset_of!(crate::B, a_) == 0);
};
