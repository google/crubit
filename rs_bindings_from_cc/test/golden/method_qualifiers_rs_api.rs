// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:method_qualifiers_cc

#![rustfmt::skip]
#![feature(cfi_encoding, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![allow(deprecated)]
#![allow(unknown_lints, suspicious_runtime_symbol_definitions)]
#![deny(warnings)]
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "9Noninline"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=Noninline
pub struct Noninline {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for Noninline {}
impl !Sync for Noninline {}
unsafe impl ::cxx::ExternType for Noninline {
    type Id = ::cxx::type_id!("Noninline");
    type Kind = ::cxx::kind::Trivial;
}
impl Noninline {
    #[inline(always)]
    pub fn UnqualifiedMethod<'__this>(&'__this mut self) {
        unsafe { self::noninline::UnqualifiedMethod(self) }
    }
    #[inline(always)]
    pub fn LvalueMethod<'__this>(&'__this mut self) {
        unsafe { self::noninline::LvalueMethod(self) }
    }
    #[inline(always)]
    pub fn LvalueMethodConst<'__this>(&'__this self) {
        unsafe { self::noninline::LvalueMethodConst(self) }
    }
    #[inline(always)]
    pub fn RvalueMethod<'__this>(&'__this mut self) {
        unsafe { self::noninline::RvalueMethod(self) }
    }
    #[inline(always)]
    pub fn RvalueMethodConst<'__this>(&'__this self) {
        unsafe { self::noninline::RvalueMethodConst(self) }
    }
}

impl Default for Noninline {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN9NoninlineC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

pub mod noninline {
    #[inline(always)]
    pub(crate) fn UnqualifiedMethod<'__this>(__this: &'__this mut crate::Noninline) {
        unsafe { crate::detail::__rust_thunk___ZN9Noninline17UnqualifiedMethodEv(__this) }
    }
    #[inline(always)]
    pub(crate) fn LvalueMethod<'__this>(__this: &'__this mut crate::Noninline) {
        unsafe { crate::detail::__rust_thunk___ZNR9Noninline12LvalueMethodEv(__this) }
    }
    #[inline(always)]
    pub(crate) fn LvalueMethodConst<'__this>(__this: &'__this crate::Noninline) {
        unsafe { crate::detail::__rust_thunk___ZNKR9Noninline17LvalueMethodConstEv(__this) }
    }
    #[inline(always)]
    pub(crate) fn RvalueMethod<'__this>(__this: &'__this mut crate::Noninline) {
        unsafe { crate::detail::__rust_thunk___ZNO9Noninline12RvalueMethodEv(__this) }
    }
    #[inline(always)]
    pub(crate) fn RvalueMethodConst<'__this>(__this: &'__this crate::Noninline) {
        unsafe { crate::detail::__rust_thunk___ZNKO9Noninline17RvalueMethodConstEv(__this) }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "6Inline"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=Inline
pub struct Inline {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for Inline {}
impl !Sync for Inline {}
unsafe impl ::cxx::ExternType for Inline {
    type Id = ::cxx::type_id!("Inline");
    type Kind = ::cxx::kind::Trivial;
}
impl Inline {
    #[inline(always)]
    pub fn UnqualifiedMethod<'__this>(&'__this mut self) {
        unsafe { self::inline::UnqualifiedMethod(self) }
    }
    #[inline(always)]
    pub fn LvalueMethod<'__this>(&'__this mut self) {
        unsafe { self::inline::LvalueMethod(self) }
    }
    #[inline(always)]
    pub fn LvalueMethodConst<'__this>(&'__this self) {
        unsafe { self::inline::LvalueMethodConst(self) }
    }
    #[inline(always)]
    pub fn RvalueMethod<'__this>(&'__this mut self) {
        unsafe { self::inline::RvalueMethod(self) }
    }
    #[inline(always)]
    pub fn RvalueMethodConst<'__this>(&'__this self) {
        unsafe { self::inline::RvalueMethodConst(self) }
    }
}

impl Default for Inline {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN6InlineC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

pub mod inline {
    #[inline(always)]
    pub(crate) fn UnqualifiedMethod<'__this>(__this: &'__this mut crate::Inline) {
        unsafe { crate::detail::__rust_thunk___ZN6Inline17UnqualifiedMethodEv(__this) }
    }
    #[inline(always)]
    pub(crate) fn LvalueMethod<'__this>(__this: &'__this mut crate::Inline) {
        unsafe { crate::detail::__rust_thunk___ZNR6Inline12LvalueMethodEv(__this) }
    }
    #[inline(always)]
    pub(crate) fn LvalueMethodConst<'__this>(__this: &'__this crate::Inline) {
        unsafe { crate::detail::__rust_thunk___ZNKR6Inline17LvalueMethodConstEv(__this) }
    }
    #[inline(always)]
    pub(crate) fn RvalueMethod<'__this>(__this: &'__this mut crate::Inline) {
        unsafe { crate::detail::__rust_thunk___ZNO6Inline12RvalueMethodEv(__this) }
    }
    #[inline(always)]
    pub(crate) fn RvalueMethodConst<'__this>(__this: &'__this crate::Inline) {
        unsafe { crate::detail::__rust_thunk___ZNKO6Inline17RvalueMethodConstEv(__this) }
    }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN9NoninlineC1Ev(__this: *mut ::core::ffi::c_void);
        #[link_name = "_ZN9Noninline17UnqualifiedMethodEv"]
        pub(crate) unsafe fn __rust_thunk___ZN9Noninline17UnqualifiedMethodEv<'__this>(
            __this: &'__this mut crate::Noninline,
        );
        #[link_name = "_ZNR9Noninline12LvalueMethodEv"]
        pub(crate) unsafe fn __rust_thunk___ZNR9Noninline12LvalueMethodEv<'__this>(
            __this: &'__this mut crate::Noninline,
        );
        #[link_name = "_ZNKR9Noninline17LvalueMethodConstEv"]
        pub(crate) unsafe fn __rust_thunk___ZNKR9Noninline17LvalueMethodConstEv<'__this>(
            __this: &'__this crate::Noninline,
        );
        #[link_name = "_ZNO9Noninline12RvalueMethodEv"]
        pub(crate) unsafe fn __rust_thunk___ZNO9Noninline12RvalueMethodEv<'__this>(
            __this: &'__this mut crate::Noninline,
        );
        #[link_name = "_ZNKO9Noninline17RvalueMethodConstEv"]
        pub(crate) unsafe fn __rust_thunk___ZNKO9Noninline17RvalueMethodConstEv<'__this>(
            __this: &'__this crate::Noninline,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6InlineC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN6Inline17UnqualifiedMethodEv<'__this>(
            __this: &'__this mut crate::Inline,
        );
        pub(crate) unsafe fn __rust_thunk___ZNR6Inline12LvalueMethodEv<'__this>(
            __this: &'__this mut crate::Inline,
        );
        pub(crate) unsafe fn __rust_thunk___ZNKR6Inline17LvalueMethodConstEv<'__this>(
            __this: &'__this crate::Inline,
        );
        pub(crate) unsafe fn __rust_thunk___ZNO6Inline12RvalueMethodEv<'__this>(
            __this: &'__this mut crate::Inline,
        );
        pub(crate) unsafe fn __rust_thunk___ZNKO6Inline17RvalueMethodConstEv<'__this>(
            __this: &'__this crate::Inline,
        );
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::Noninline>() == 1);
    assert!(::core::mem::align_of::<crate::Noninline>() == 1);
    static_assertions::assert_impl_all!(crate::Noninline: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::Noninline: Drop);

    assert!(::core::mem::size_of::<crate::Inline>() == 1);
    assert!(::core::mem::align_of::<crate::Inline>() == 1);
    static_assertions::assert_impl_all!(crate::Inline: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::Inline: Drop);
};
