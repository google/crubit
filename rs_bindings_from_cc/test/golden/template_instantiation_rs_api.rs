// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:template_instantiation_cc

#![rustfmt::skip]
#![feature(cfi_encoding, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![allow(deprecated)]
#![allow(unknown_lints, suspicious_runtime_symbol_definitions)]
#![deny(warnings)]
// error: class `TS` could not be bound
//   Class templates are not yet supported

#[inline(always)]
pub fn RTS() -> crate::__CcTemplateInst2TSIiE {
    unsafe {
        let mut __crubit_return =
            ::core::mem::MaybeUninit::<crate::__CcTemplateInst2TSIiE>::uninit();
        crate::detail::__rust_thunk___Z3RTSv(&raw mut __crubit_return as *mut ::core::ffi::c_void);
        __crubit_return.assume_init()
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "11NormalClass"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=NormalClass
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct NormalClass {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for NormalClass {}
impl !Sync for NormalClass {}
unsafe impl ::cxx::ExternType for NormalClass {
    type Id = ::cxx::type_id!("NormalClass");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for NormalClass {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN11NormalClassC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

// error: function `operator==` could not be bound
//   Function templates are not yet supported

// error: class `TemplateWithStaticMembers` could not be bound
//   Class templates are not yet supported

pub type TemplateWithStaticMembersInt = crate::__CcTemplateInst25TemplateWithStaticMembersIiE;

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInst2TSIiE"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=TS < int >
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInst2TSIiE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for __CcTemplateInst2TSIiE {}
impl !Sync for __CcTemplateInst2TSIiE {}

impl Default for __CcTemplateInst2TSIiE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__20ba560a__ZN2TSIiEC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

/// Static data members of class template specializations are not supported
/// yet, because their initializers and definitions are instantiated lazily.
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInst25TemplateWithStaticMembersIiE"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=TemplateWithStaticMembers < int >
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInst25TemplateWithStaticMembersIiE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for __CcTemplateInst25TemplateWithStaticMembersIiE {}
impl !Sync for __CcTemplateInst25TemplateWithStaticMembersIiE {}

impl Default for __CcTemplateInst25TemplateWithStaticMembersIiE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__ae801879__ZN25TemplateWithStaticMembersIiEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

// error: global variable `TemplateWithStaticMembers<int>::kValue` could not be bound
//   static data members of class template specializations are not supported

// error: global variable `TemplateWithStaticMembers<int>::value` could not be bound
//   static data members of class template specializations are not supported

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___Z3RTSv(__return: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN11NormalClassC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk__20ba560a__ZN2TSIiEC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk__ae801879__ZN25TemplateWithStaticMembersIiEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::NormalClass>() == 1);
    assert!(::core::mem::align_of::<crate::NormalClass>() == 1);
    static_assertions::assert_impl_all!(crate::NormalClass: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::NormalClass: Drop);

    assert!(::core::mem::size_of::<crate::__CcTemplateInst2TSIiE>() == 1);
    assert!(::core::mem::align_of::<crate::__CcTemplateInst2TSIiE>() == 1);
    static_assertions::assert_impl_all!(crate::__CcTemplateInst2TSIiE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInst2TSIiE: Drop);

    assert!(::core::mem::size_of::<crate::__CcTemplateInst25TemplateWithStaticMembersIiE>() == 1);
    assert!(::core::mem::align_of::<crate::__CcTemplateInst25TemplateWithStaticMembersIiE>() == 1);
    static_assertions::assert_impl_all!(crate::__CcTemplateInst25TemplateWithStaticMembersIiE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInst25TemplateWithStaticMembersIiE: Drop);
};
