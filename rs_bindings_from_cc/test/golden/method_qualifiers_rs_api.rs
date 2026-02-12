// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:method_qualifiers_cc

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
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
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub unsafe fn UnqualifiedMethod(__this: *mut Self) {
        crate::detail::__rust_thunk___ZN9Noninline17UnqualifiedMethodEv(__this)
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub unsafe fn LvalueMethod(__this: *mut Self) {
        crate::detail::__rust_thunk___ZNR9Noninline12LvalueMethodEv(__this)
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub unsafe fn LvalueMethodConst(__this: *const Self) {
        crate::detail::__rust_thunk___ZNKR9Noninline17LvalueMethodConstEv(__this)
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub unsafe fn RvalueMethod(__this: *mut Self) {
        crate::detail::__rust_thunk___ZNO9Noninline12RvalueMethodEv(__this)
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub unsafe fn RvalueMethodConst(__this: *const Self) {
        crate::detail::__rust_thunk___ZNKO9Noninline17RvalueMethodConstEv(__this)
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

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
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
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub unsafe fn UnqualifiedMethod(__this: *mut Self) {
        crate::detail::__rust_thunk___ZN6Inline17UnqualifiedMethodEv(__this)
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub unsafe fn LvalueMethod(__this: *mut Self) {
        crate::detail::__rust_thunk___ZNR6Inline12LvalueMethodEv(__this)
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub unsafe fn LvalueMethodConst(__this: *const Self) {
        crate::detail::__rust_thunk___ZNKR6Inline17LvalueMethodConstEv(__this)
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub unsafe fn RvalueMethod(__this: *mut Self) {
        crate::detail::__rust_thunk___ZNO6Inline12RvalueMethodEv(__this)
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub unsafe fn RvalueMethodConst(__this: *const Self) {
        crate::detail::__rust_thunk___ZNKO6Inline17RvalueMethodConstEv(__this)
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

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN9NoninlineC1Ev(__this: *mut ::core::ffi::c_void);
        #[link_name = "_ZN9Noninline17UnqualifiedMethodEv"]
        pub(crate) unsafe fn __rust_thunk___ZN9Noninline17UnqualifiedMethodEv(
            __this: *mut crate::Noninline,
        );
        #[link_name = "_ZNR9Noninline12LvalueMethodEv"]
        pub(crate) unsafe fn __rust_thunk___ZNR9Noninline12LvalueMethodEv(
            __this: *mut crate::Noninline,
        );
        #[link_name = "_ZNKR9Noninline17LvalueMethodConstEv"]
        pub(crate) unsafe fn __rust_thunk___ZNKR9Noninline17LvalueMethodConstEv(
            __this: *const crate::Noninline,
        );
        #[link_name = "_ZNO9Noninline12RvalueMethodEv"]
        pub(crate) unsafe fn __rust_thunk___ZNO9Noninline12RvalueMethodEv(
            __this: *mut crate::Noninline,
        );
        #[link_name = "_ZNKO9Noninline17RvalueMethodConstEv"]
        pub(crate) unsafe fn __rust_thunk___ZNKO9Noninline17RvalueMethodConstEv(
            __this: *const crate::Noninline,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6InlineC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN6Inline17UnqualifiedMethodEv(
            __this: *mut crate::Inline,
        );
        pub(crate) unsafe fn __rust_thunk___ZNR6Inline12LvalueMethodEv(__this: *mut crate::Inline);
        pub(crate) unsafe fn __rust_thunk___ZNKR6Inline17LvalueMethodConstEv(
            __this: *const crate::Inline,
        );
        pub(crate) unsafe fn __rust_thunk___ZNO6Inline12RvalueMethodEv(__this: *mut crate::Inline);
        pub(crate) unsafe fn __rust_thunk___ZNKO6Inline17RvalueMethodConstEv(
            __this: *const crate::Inline,
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
