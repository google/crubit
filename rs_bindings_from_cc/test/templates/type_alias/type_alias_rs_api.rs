// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/templates/type_alias:type_alias

#![rustfmt::skip]
#![feature(cfi_encoding, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![allow(deprecated)]
#![allow(unknown_lints, suspicious_runtime_symbol_definitions)]
#![deny(warnings)]
// error: class `MyTemplate` could not be bound
//   Class templates are not yet supported

pub type MyTypeAlias = crate::__CcTemplateInst10MyTemplateIiE;

pub type OtherTypeAliasInSameTarget = crate::__CcTemplateInst10MyTemplateIiE;

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInst10MyTemplateIiE"]
#[repr(C, align(4))]
///CRUBIT_ANNOTATE: cpp_type=:: MyTemplate < int >
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInst10MyTemplateIiE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) value_: [::core::mem::MaybeUninit<u8>; 4],
}
impl !Send for __CcTemplateInst10MyTemplateIiE {}
impl !Sync for __CcTemplateInst10MyTemplateIiE {}
impl ::core::fmt::Debug for __CcTemplateInst10MyTemplateIiE {
    fn fmt(&self, formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        formatter.debug_struct("__CcTemplateInst10MyTemplateIiE").finish_non_exhaustive()
    }
}
forward_declare::unsafe_define!(
    forward_declare::symbol!(":: MyTemplate < int >"),
    crate::__CcTemplateInst10MyTemplateIiE
);
impl __CcTemplateInst10MyTemplateIiE {
    #[inline(always)]
    pub fn Create(value: ::ffi_11::c_int) -> crate::__CcTemplateInst10MyTemplateIiE {
        unsafe { self::cc_template_inst10_my_template_ii_e::Create(value) }
    }
    #[inline(always)]
    pub fn value<'__this>(&'__this self) -> ::cref::CRef<'__this, ::ffi_11::c_int> {
        unsafe { self::cc_template_inst10_my_template_ii_e::value(self) }
    }
}

impl Default for __CcTemplateInst10MyTemplateIiE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__de4d35d3__ZN10MyTemplateIiEC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

pub mod cc_template_inst10_my_template_ii_e {
    #[inline(always)]
    pub(crate) fn Create(value: ::ffi_11::c_int) -> crate::__CcTemplateInst10MyTemplateIiE {
        unsafe {
            let mut __crubit_return =
                ::core::mem::MaybeUninit::<crate::__CcTemplateInst10MyTemplateIiE>::uninit();
            crate::detail::__rust_thunk__5a9d55c6__ZN10MyTemplateIiE6CreateEi(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                value,
            );
            __crubit_return.assume_init()
        }
    }
    #[inline(always)]
    pub(crate) fn value<'__this>(
        __this: &'__this crate::__CcTemplateInst10MyTemplateIiE,
    ) -> ::cref::CRef<'__this, ::ffi_11::c_int> {
        unsafe { crate::detail::__rust_thunk__758630aa__ZNK10MyTemplateIiE5valueEv(__this) }
    }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk__de4d35d3__ZN10MyTemplateIiEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__5a9d55c6__ZN10MyTemplateIiE6CreateEi(
            __return: *mut ::core::ffi::c_void,
            value: ::ffi_11::c_int,
        );
        pub(crate) unsafe fn __rust_thunk__758630aa__ZNK10MyTemplateIiE5valueEv<'__this>(
            __this: &'__this crate::__CcTemplateInst10MyTemplateIiE,
        ) -> ::cref::CRef<'__this, ::ffi_11::c_int>;
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::__CcTemplateInst10MyTemplateIiE>() == 4);
    assert!(::core::mem::align_of::<crate::__CcTemplateInst10MyTemplateIiE>() == 4);
    static_assertions::assert_impl_all!(crate::__CcTemplateInst10MyTemplateIiE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInst10MyTemplateIiE: Drop);
    assert!(::core::mem::offset_of!(crate::__CcTemplateInst10MyTemplateIiE, value_) == 0);
};
