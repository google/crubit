// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:displayables_cc

#![rustfmt::skip]
#![feature(cfi_encoding, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![allow(deprecated)]
#![deny(warnings)]

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=DisplayableStruct
pub struct DisplayableStruct {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for DisplayableStruct {}
impl !Sync for DisplayableStruct {}
unsafe impl ::cxx::ExternType for DisplayableStruct {
    type Id = ::cxx::type_id!("DisplayableStruct");
    type Kind = ::cxx::kind::Trivial;
}
impl ::core::fmt::Display for DisplayableStruct {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut f = ::lossy_formatter::LossyFormatter::new(f);
        if unsafe {
            crate::detail::__crubit_fmt__17DisplayableStruct___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3adisplayables_5fcc(self,&mut f)
        } {
            ::core::result::Result::Ok(())
        } else {
            ::core::result::Result::Err(::core::fmt::Error)
        }
    }
}

impl Default for DisplayableStruct {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN17DisplayableStructC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
#[cfi_encoding = "15DisplayableEnum"]
///CRUBIT_ANNOTATE: cpp_type=DisplayableEnum
pub struct DisplayableEnum(::ffi_11::c_int);
impl DisplayableEnum {
    pub const kKnown: DisplayableEnum = DisplayableEnum(::ffi_11::new_c_int(1));
}
impl From<::ffi_11::c_int> for DisplayableEnum {
    fn from(value: ::ffi_11::c_int) -> DisplayableEnum {
        DisplayableEnum(value)
    }
}
impl From<DisplayableEnum> for ::ffi_11::c_int {
    fn from(value: DisplayableEnum) -> ::ffi_11::c_int {
        value.0
    }
}
impl ::core::fmt::Display for DisplayableEnum {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut f = ::lossy_formatter::LossyFormatter::new(f);
        if unsafe {
            crate::detail::__crubit_fmt__DisplayableEnum___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3adisplayables_5fcc(self,&mut f)
        } {
            ::core::result::Result::Ok(())
        } else {
            ::core::result::Result::Err(::core::fmt::Error)
        }
    }
}

// error: function `AbslStringify` could not be bound
//   Function templates are not yet supported

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN17DisplayableStructC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __crubit_fmt__17DisplayableStruct___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3adisplayables_5fcc(
            value: &crate::DisplayableStruct,
            formatter: &mut ::lossy_formatter::LossyFormatter,
        ) -> bool;
        pub(crate) unsafe fn __crubit_fmt__DisplayableEnum___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3adisplayables_5fcc(
            value: &crate::DisplayableEnum,
            formatter: &mut ::lossy_formatter::LossyFormatter,
        ) -> bool;
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::DisplayableStruct>() == 1);
    assert!(::core::mem::align_of::<crate::DisplayableStruct>() == 1);
    static_assertions::assert_impl_all!(crate::DisplayableStruct: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::DisplayableStruct: Drop);
};
