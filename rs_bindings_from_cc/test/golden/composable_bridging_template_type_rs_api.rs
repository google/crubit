// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:composable_bridging_template_type_cc

#![rustfmt::skip]
#![feature(cfi_encoding, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![allow(deprecated)]
#![allow(unknown_lints, suspicious_runtime_symbol_definitions)]
#![deny(warnings)]
// error: class `MyOption` could not be bound
//   Class templates are not yet supported

// error: class `Value` could not be bound
//   Class templates are not yet supported

#[inline(always)]
pub fn ReturnsValue(
) -> crate::MyOption<crate::zz_template_instantiations::__CcTemplateInst5ValueIiE> {
    unsafe {
        ::crubit_support::bridge::unstable_return!(@crate::MyOptionAbi(::crubit_support::bridge::transmute_abi::<crate::zz_template_instantiations::__CcTemplateInst5ValueIiE>()),crate::MyOptionAbi<::crubit_support::bridge::TransmuteAbi<crate::zz_template_instantiations::__CcTemplateInst5ValueIiE>>,|__crubit_return_abi_buffer|{ crate::detail::__rust_thunk___Z12ReturnsValuev(__crubit_return_abi_buffer,); })
    }
}

/// Class template instantiations that aren't named by a C++ type alias.
pub mod zz_template_instantiations {
    /// A basic templated type that does nothing fancy.
    #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
    #[cfi_encoding = "__CcTemplateInst5ValueIiE"]
    #[repr(C)]
    ///CRUBIT_ANNOTATE: cpp_type=Value < int >
    ///CRUBIT_ANNOTATE: cpp_move_constructible=
    pub struct __CcTemplateInst5ValueIiE {
        pub value: ::ffi_11::c_int,
    }
    impl !Send for __CcTemplateInst5ValueIiE {}
    impl !Sync for __CcTemplateInst5ValueIiE {}
    forward_declare::unsafe_define!(
        forward_declare::symbol!("Value < int >"),
        crate::zz_template_instantiations::__CcTemplateInst5ValueIiE
    );

    impl Default for __CcTemplateInst5ValueIiE {
        #[inline(always)]
        fn default() -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk__79c7b51c__ZN5ValueIiEC1Ev(&raw mut tmp as *mut _);
                tmp.assume_init()
            }
        }
    }
}
// Keeps the instantiations available at their old path.
#[doc(hidden)]
pub use self::zz_template_instantiations::*;

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___Z12ReturnsValuev(
            __return_abi_buffer: *mut ::core::ffi::c_uchar,
        );
        pub(crate) unsafe fn __rust_thunk__79c7b51c__ZN5ValueIiEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
    }
}

const _: () = {
    assert!(
        ::core::mem::size_of::<crate::zz_template_instantiations::__CcTemplateInst5ValueIiE>() == 4
    );
    assert!(
        ::core::mem::align_of::<crate::zz_template_instantiations::__CcTemplateInst5ValueIiE>()
            == 4
    );
    static_assertions::assert_impl_all!(crate::zz_template_instantiations::__CcTemplateInst5ValueIiE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::zz_template_instantiations::__CcTemplateInst5ValueIiE: Drop);
    assert!(
        ::core::mem::offset_of!(
            crate::zz_template_instantiations::__CcTemplateInst5ValueIiE,
            value
        ) == 0
    );
};
