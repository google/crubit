// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:user_of_imported_type_cc

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes, negative_impls, register_tool)]
#![allow(stable_features)]
#![no_std]
#![register_tool(__crubit)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code)]
#![deny(warnings)]

#[inline(always)]
pub fn UsesImportedType(mut t: trivial_type_cc::ns::Trivial) -> trivial_type_cc::ns::Trivial {
    unsafe {
        let mut __return = ::core::mem::MaybeUninit::<trivial_type_cc::ns::Trivial>::uninit();
        crate::detail::__rust_thunk___Z16UsesImportedTypeN2ns7TrivialE(
            &raw mut __return as *mut ::core::ffi::c_void,
            &mut t,
        );
        __return.assume_init()
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
#[__crubit::annotate(cpp_type = "UserOfImportedType")]
pub struct UserOfImportedType {
    pub trivial: *mut trivial_type_cc::ns::Trivial,
}
impl !Send for UserOfImportedType {}
impl !Sync for UserOfImportedType {}
forward_declare::unsafe_define!(
    forward_declare::symbol!("UserOfImportedType"),
    crate::UserOfImportedType
);

impl Default for UserOfImportedType {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN18UserOfImportedTypeC1Ev(
                &raw mut tmp as *mut ::core::ffi::c_void,
            );
            tmp.assume_init()
        }
    }
}

impl<'b> From<::ctor::RvalueReference<'b, Self>> for UserOfImportedType {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN18UserOfImportedTypeC1EOS_(
                &raw mut tmp as *mut ::core::ffi::c_void,
                __param_0,
            );
            tmp.assume_init()
        }
    }
}

impl<'b> ::ctor::UnpinAssign<&'b Self> for UserOfImportedType {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN18UserOfImportedTypeaSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::UnpinAssign<::ctor::RvalueReference<'b, Self>> for UserOfImportedType {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: ::ctor::RvalueReference<'b, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN18UserOfImportedTypeaSEOS_(self, __param_0);
        }
    }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___Z16UsesImportedTypeN2ns7TrivialE(
            __return: *mut ::core::ffi::c_void,
            t: &mut trivial_type_cc::ns::Trivial,
        );
        pub(crate) unsafe fn __rust_thunk___ZN18UserOfImportedTypeC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN18UserOfImportedTypeC1EOS_<'b>(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'b, crate::UserOfImportedType>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN18UserOfImportedTypeaSERKS_<'a, 'b>(
            __this: &'a mut crate::UserOfImportedType,
            __param_0: &'b crate::UserOfImportedType,
        ) -> &'a mut crate::UserOfImportedType;
        pub(crate) unsafe fn __rust_thunk___ZN18UserOfImportedTypeaSEOS_<'a, 'b>(
            __this: &'a mut crate::UserOfImportedType,
            __param_0: ::ctor::RvalueReference<'b, crate::UserOfImportedType>,
        ) -> &'a mut crate::UserOfImportedType;
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::UserOfImportedType>() == 8);
    assert!(::core::mem::align_of::<crate::UserOfImportedType>() == 8);
    static_assertions::assert_impl_all!(crate::UserOfImportedType: Clone);
    static_assertions::assert_impl_all!(crate::UserOfImportedType: Copy);
    static_assertions::assert_not_impl_any!(crate::UserOfImportedType: Drop);
    assert!(::core::mem::offset_of!(crate::UserOfImportedType, trivial) == 0);
};
