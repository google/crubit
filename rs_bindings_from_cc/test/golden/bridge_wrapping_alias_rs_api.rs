// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:bridge_wrapping_alias_cc

#![rustfmt::skip]
#![feature(cfi_encoding, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![allow(deprecated)]
#![allow(unknown_lints, suspicious_runtime_symbol_definitions)]
#![deny(warnings)]
// error: class `TemplateType` could not be bound
//   Class templates are not yet supported

pub type AliasToInst = crate::__CcTemplateInst12TemplateTypeIiE;

// error: class `Bridge` could not be bound
//   Class templates are not yet supported

// error: class `BridgeAbi` could not be bound
//   Class templates are not yet supported

/// `AliasToInst` is an alias to a template instantiation. With the
/// `template_instantiation` feature enabled (see the aspect hints in BUILD),
/// the instantiation is bound and the bridge type wrapping it can be bridged
/// too, as `Bridge<__CcTemplateInst12TemplateTypeIiE>`.
#[inline(always)]
pub fn bridge_alias_to_inst() -> crate::Bridge<crate::__CcTemplateInst12TemplateTypeIiE> {
    unsafe {
        ::crubit_support::bridge::unstable_return!(@crate::BridgeAbi(::crubit_support::bridge::transmute_abi::<crate::__CcTemplateInst12TemplateTypeIiE>()),crate::BridgeAbi<::crubit_support::bridge::TransmuteAbi<crate::__CcTemplateInst12TemplateTypeIiE>>,|__crubit_return_abi_buffer|{ crate::detail::__rust_thunk___Z20bridge_alias_to_instv(__crubit_return_abi_buffer,); })
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInst12TemplateTypeIiE"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=TemplateType < int >
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInst12TemplateTypeIiE {
    pub value: ::ffi_11::c_int,
}
impl !Send for __CcTemplateInst12TemplateTypeIiE {}
impl !Sync for __CcTemplateInst12TemplateTypeIiE {}

impl Default for __CcTemplateInst12TemplateTypeIiE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__2f8f40d6__ZN12TemplateTypeIiEC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

#[path = "rs_bindings_from_cc/test/golden/bridge_wrapping_alias_extra.rs"]
mod __crubit_mod_0;
#[allow(unused_imports)]
pub use __crubit_mod_0::*;

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___Z20bridge_alias_to_instv(
            __return_abi_buffer: *mut ::core::ffi::c_uchar,
        );
        pub(crate) unsafe fn __rust_thunk__2f8f40d6__ZN12TemplateTypeIiEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::__CcTemplateInst12TemplateTypeIiE>() == 4);
    assert!(::core::mem::align_of::<crate::__CcTemplateInst12TemplateTypeIiE>() == 4);
    static_assertions::assert_impl_all!(crate::__CcTemplateInst12TemplateTypeIiE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInst12TemplateTypeIiE: Drop);
    assert!(::core::mem::offset_of!(crate::__CcTemplateInst12TemplateTypeIiE, value) == 0);
};
