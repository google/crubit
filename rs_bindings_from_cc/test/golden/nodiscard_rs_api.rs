// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:nodiscard_cc

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![no_std]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

#[derive(Clone, Copy)]
#[must_use]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=NoDiscard
pub struct NoDiscard {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for NoDiscard {}
impl !Sync for NoDiscard {}
forward_declare::unsafe_define!(forward_declare::symbol!("NoDiscard"), crate::NoDiscard);

impl Default for NoDiscard {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN9NoDiscardC1Ev(
                &raw mut tmp as *mut ::core::ffi::c_void,
            );
            tmp.assume_init()
        }
    }
}

impl From<::ctor::RvalueReference<'_, Self>> for NoDiscard {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'_, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN9NoDiscardC1EOS_(
                &raw mut tmp as *mut ::core::ffi::c_void,
                __param_0,
            );
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<::ctor::RvalueReference<'_, Self>> for NoDiscard {
    type CtorType = Self;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'_, Self>) -> Self::CtorType {
        <Self as From<::ctor::RvalueReference<'_, Self>>>::from(args)
    }
}

impl ::ctor::UnpinAssign<&Self> for NoDiscard {
    #[inline(always)]
    fn unpin_assign(&mut self, __param_0: &Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN9NoDiscardaSERKS_(self, __param_0);
        }
    }
}

impl ::ctor::UnpinAssign<::ctor::RvalueReference<'_, Self>> for NoDiscard {
    #[inline(always)]
    fn unpin_assign(&mut self, __param_0: ::ctor::RvalueReference<'_, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN9NoDiscardaSEOS_(self, __param_0);
        }
    }
}

#[derive(Clone, Copy)]
#[must_use = "You really should use this"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=NoDiscardWithMessage
pub struct NoDiscardWithMessage {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for NoDiscardWithMessage {}
impl !Sync for NoDiscardWithMessage {}
forward_declare::unsafe_define!(
    forward_declare::symbol!("NoDiscardWithMessage"),
    crate::NoDiscardWithMessage
);

impl Default for NoDiscardWithMessage {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN20NoDiscardWithMessageC1Ev(
                &raw mut tmp as *mut ::core::ffi::c_void,
            );
            tmp.assume_init()
        }
    }
}

impl From<::ctor::RvalueReference<'_, Self>> for NoDiscardWithMessage {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'_, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN20NoDiscardWithMessageC1EOS_(
                &raw mut tmp as *mut ::core::ffi::c_void,
                __param_0,
            );
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<::ctor::RvalueReference<'_, Self>> for NoDiscardWithMessage {
    type CtorType = Self;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'_, Self>) -> Self::CtorType {
        <Self as From<::ctor::RvalueReference<'_, Self>>>::from(args)
    }
}

impl ::ctor::UnpinAssign<&Self> for NoDiscardWithMessage {
    #[inline(always)]
    fn unpin_assign(&mut self, __param_0: &Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN20NoDiscardWithMessageaSERKS_(self, __param_0);
        }
    }
}

impl ::ctor::UnpinAssign<::ctor::RvalueReference<'_, Self>> for NoDiscardWithMessage {
    #[inline(always)]
    fn unpin_assign(&mut self, __param_0: ::ctor::RvalueReference<'_, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN20NoDiscardWithMessageaSEOS_(self, __param_0);
        }
    }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN9NoDiscardC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN9NoDiscardC1EOS_(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'_, crate::NoDiscard>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN9NoDiscardaSERKS_<'__return_lifetime>(
            __this: &mut crate::NoDiscard,
            __param_0: &crate::NoDiscard,
        ) -> &'__return_lifetime mut crate::NoDiscard;
        pub(crate) unsafe fn __rust_thunk___ZN9NoDiscardaSEOS_<'__return_lifetime>(
            __this: &mut crate::NoDiscard,
            __param_0: ::ctor::RvalueReference<'_, crate::NoDiscard>,
        ) -> &'__return_lifetime mut crate::NoDiscard;
        pub(crate) unsafe fn __rust_thunk___ZN20NoDiscardWithMessageC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN20NoDiscardWithMessageC1EOS_(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'_, crate::NoDiscardWithMessage>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN20NoDiscardWithMessageaSERKS_<'__return_lifetime>(
            __this: &mut crate::NoDiscardWithMessage,
            __param_0: &crate::NoDiscardWithMessage,
        ) -> &'__return_lifetime mut crate::NoDiscardWithMessage;
        pub(crate) unsafe fn __rust_thunk___ZN20NoDiscardWithMessageaSEOS_<'__return_lifetime>(
            __this: &mut crate::NoDiscardWithMessage,
            __param_0: ::ctor::RvalueReference<'_, crate::NoDiscardWithMessage>,
        ) -> &'__return_lifetime mut crate::NoDiscardWithMessage;
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::NoDiscard>() == 1);
    assert!(::core::mem::align_of::<crate::NoDiscard>() == 1);
    static_assertions::assert_impl_all!(crate::NoDiscard: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::NoDiscard: Drop);

    assert!(::core::mem::size_of::<crate::NoDiscardWithMessage>() == 1);
    assert!(::core::mem::align_of::<crate::NoDiscardWithMessage>() == 1);
    static_assertions::assert_impl_all!(crate::NoDiscardWithMessage: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::NoDiscardWithMessage: Drop);
};
