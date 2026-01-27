// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/templates/type_alias:type_alias
// Features: assume_lifetimes, custom_ffi_types, experimental, non_unpin_ctor, std_unique_ptr, std_vector, supported, wrapper

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![no_std]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

// Generated from: rs_bindings_from_cc/test/templates/type_alias/type_alias.h;l=10
// Error while generating bindings for class 'MyTemplate':
// Class templates are not supported yet

/// Generated from: rs_bindings_from_cc/test/templates/type_alias/type_alias.h;l=25
pub type MyTypeAlias = crate::__CcTemplateInst10MyTemplateIiE;

/// Generated from: rs_bindings_from_cc/test/templates/type_alias/type_alias.h;l=26
pub type OtherTypeAliasInSameTarget = crate::__CcTemplateInst10MyTemplateIiE;

/// Generated from: rs_bindings_from_cc/test/templates/type_alias/type_alias.h;l=11
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C, align(4))]
///CRUBIT_ANNOTATE: cpp_type=MyTemplate < int >
pub struct __CcTemplateInst10MyTemplateIiE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) value_: [::core::mem::MaybeUninit<u8>; 4],
}
impl !Send for __CcTemplateInst10MyTemplateIiE {}
impl !Sync for __CcTemplateInst10MyTemplateIiE {}
forward_declare::unsafe_define!(
    forward_declare::symbol!("MyTemplate < int >"),
    crate::__CcTemplateInst10MyTemplateIiE
);
impl __CcTemplateInst10MyTemplateIiE {
    /// Generated from: rs_bindings_from_cc/test/templates/type_alias/type_alias.h;l=13
    #[inline(always)]
    pub fn Create(value: ::ffi_11::c_int) -> crate::__CcTemplateInst10MyTemplateIiE {
        unsafe {
            let mut __return = ::core::mem::MaybeUninit::<Self>::uninit();
            crate::detail::__rust_thunk___ZN10MyTemplateIiE6CreateEi__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2ftemplates_2ftype_5falias_3atype_5falias(&raw mut __return as*mut::core::ffi::c_void,value);
            __return.assume_init()
        }
    }
    /// Generated from: rs_bindings_from_cc/test/templates/type_alias/type_alias.h;l=19
    #[inline(always)]
    pub fn value<'a>(&'a self) -> &'a ::ffi_11::c_int {
        unsafe {
            crate::detail::__rust_thunk___ZNK10MyTemplateIiE5valueEv__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2ftemplates_2ftype_5falias_3atype_5falias(self)
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/templates/type_alias/type_alias.h;l=11
impl Default for __CcTemplateInst10MyTemplateIiE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN10MyTemplateIiEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2ftemplates_2ftype_5falias_3atype_5falias(&raw mut tmp as*mut _);
            tmp.assume_init()
        }
    }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN10MyTemplateIiEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2ftemplates_2ftype_5falias_3atype_5falias(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN10MyTemplateIiE6CreateEi__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2ftemplates_2ftype_5falias_3atype_5falias(
            __return: *mut ::core::ffi::c_void,
            value: ::ffi_11::c_int,
        );
        pub(crate) unsafe fn __rust_thunk___ZNK10MyTemplateIiE5valueEv__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2ftemplates_2ftype_5falias_3atype_5falias<
            'a,
        >(
            __this: &'a crate::__CcTemplateInst10MyTemplateIiE,
        ) -> &'a ::ffi_11::c_int;
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::__CcTemplateInst10MyTemplateIiE>() == 4);
    assert!(::core::mem::align_of::<crate::__CcTemplateInst10MyTemplateIiE>() == 4);
    static_assertions::assert_impl_all!(crate::__CcTemplateInst10MyTemplateIiE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInst10MyTemplateIiE: Drop);
    assert!(::core::mem::offset_of!(crate::__CcTemplateInst10MyTemplateIiE, value_) == 0);
};
