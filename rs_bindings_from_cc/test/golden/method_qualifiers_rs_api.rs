// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:method_qualifiers_cc

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![no_std]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

#[derive(Clone, Copy)]
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
forward_declare::unsafe_define!(forward_declare::symbol!("Noninline"), crate::Noninline);

impl Default for Noninline {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN9NoninlineC1Ev(
                &raw mut tmp as *mut ::core::ffi::c_void,
            );
            tmp.assume_init()
        }
    }
}

impl From<::ctor::RvalueReference<'_, Self>> for Noninline {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'_, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN9NoninlineC1EOS_(
                &raw mut tmp as *mut ::core::ffi::c_void,
                __param_0,
            );
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<::ctor::RvalueReference<'_, Self>> for Noninline {
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'_, Self>) -> Self::CtorType {
        <Self as From<::ctor::RvalueReference<'_, Self>>>::from(args)
    }
}

impl ::ctor::UnpinAssign<&Self> for Noninline {
    #[inline(always)]
    fn unpin_assign(&mut self, __param_0: &Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN9NoninlineaSERKS_(self, __param_0);
        }
    }
}

impl ::ctor::UnpinAssign<::ctor::RvalueReference<'_, Self>> for Noninline {
    #[inline(always)]
    fn unpin_assign(&mut self, __param_0: ::ctor::RvalueReference<'_, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN9NoninlineaSEOS_(self, __param_0);
        }
    }
}

impl Noninline {
    #[inline(always)]
    pub unsafe fn UnqualifiedMethod(__this: *mut Self) {
        crate::detail::__rust_thunk___ZN9Noninline17UnqualifiedMethodEv(__this)
    }
}

impl Noninline {
    #[inline(always)]
    pub unsafe fn LvalueMethod(__this: *mut Self) {
        crate::detail::__rust_thunk___ZNR9Noninline12LvalueMethodEv(__this)
    }
}

impl Noninline {
    #[inline(always)]
    pub unsafe fn LvalueMethodConst(__this: *const Self) {
        crate::detail::__rust_thunk___ZNKR9Noninline17LvalueMethodConstEv(__this)
    }
}

impl Noninline {
    #[inline(always)]
    pub unsafe fn RvalueMethod(__this: *mut Self) {
        crate::detail::__rust_thunk___ZNO9Noninline12RvalueMethodEv(__this)
    }
}

impl Noninline {
    #[inline(always)]
    pub unsafe fn RvalueMethodConst(__this: *const Self) {
        crate::detail::__rust_thunk___ZNKO9Noninline17RvalueMethodConstEv(__this)
    }
}

#[derive(Clone, Copy)]
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
forward_declare::unsafe_define!(forward_declare::symbol!("Inline"), crate::Inline);

impl Default for Inline {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN6InlineC1Ev(&raw mut tmp as *mut ::core::ffi::c_void);
            tmp.assume_init()
        }
    }
}

impl From<::ctor::RvalueReference<'_, Self>> for Inline {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'_, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN6InlineC1EOS_(
                &raw mut tmp as *mut ::core::ffi::c_void,
                __param_0,
            );
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<::ctor::RvalueReference<'_, Self>> for Inline {
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'_, Self>) -> Self::CtorType {
        <Self as From<::ctor::RvalueReference<'_, Self>>>::from(args)
    }
}

impl ::ctor::UnpinAssign<&Self> for Inline {
    #[inline(always)]
    fn unpin_assign(&mut self, __param_0: &Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN6InlineaSERKS_(self, __param_0);
        }
    }
}

impl ::ctor::UnpinAssign<::ctor::RvalueReference<'_, Self>> for Inline {
    #[inline(always)]
    fn unpin_assign(&mut self, __param_0: ::ctor::RvalueReference<'_, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN6InlineaSEOS_(self, __param_0);
        }
    }
}

impl Inline {
    #[inline(always)]
    pub unsafe fn UnqualifiedMethod(__this: *mut Self) {
        crate::detail::__rust_thunk___ZN6Inline17UnqualifiedMethodEv(__this)
    }
}

impl Inline {
    #[inline(always)]
    pub unsafe fn LvalueMethod(__this: *mut Self) {
        crate::detail::__rust_thunk___ZNR6Inline12LvalueMethodEv(__this)
    }
}

impl Inline {
    #[inline(always)]
    pub unsafe fn LvalueMethodConst(__this: *const Self) {
        crate::detail::__rust_thunk___ZNKR6Inline17LvalueMethodConstEv(__this)
    }
}

impl Inline {
    #[inline(always)]
    pub unsafe fn RvalueMethod(__this: *mut Self) {
        crate::detail::__rust_thunk___ZNO6Inline12RvalueMethodEv(__this)
    }
}

impl Inline {
    #[inline(always)]
    pub unsafe fn RvalueMethodConst(__this: *const Self) {
        crate::detail::__rust_thunk___ZNKO6Inline17RvalueMethodConstEv(__this)
    }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN9NoninlineC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN9NoninlineC1EOS_(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'_, crate::Noninline>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN9NoninlineaSERKS_<'__return_lifetime>(
            __this: &mut crate::Noninline,
            __param_0: &crate::Noninline,
        ) -> &'__return_lifetime mut crate::Noninline;
        pub(crate) unsafe fn __rust_thunk___ZN9NoninlineaSEOS_<'__return_lifetime>(
            __this: &mut crate::Noninline,
            __param_0: ::ctor::RvalueReference<'_, crate::Noninline>,
        ) -> &'__return_lifetime mut crate::Noninline;
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
        pub(crate) unsafe fn __rust_thunk___ZN6InlineC1EOS_(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'_, crate::Inline>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6InlineaSERKS_<'__return_lifetime>(
            __this: &mut crate::Inline,
            __param_0: &crate::Inline,
        ) -> &'__return_lifetime mut crate::Inline;
        pub(crate) unsafe fn __rust_thunk___ZN6InlineaSEOS_<'__return_lifetime>(
            __this: &mut crate::Inline,
            __param_0: ::ctor::RvalueReference<'_, crate::Inline>,
        ) -> &'__return_lifetime mut crate::Inline;
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
