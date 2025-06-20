// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:uses_not_crubit_exposed_cc

#![rustfmt::skip]
#![feature(
    allocator_api,
    cfg_sanitize,
    custom_inner_attributes,
    impl_trait_in_assoc_type,
    negative_impls
)]
#![allow(stable_features)]
#![no_std]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

#[diagnostic::on_unimplemented(
    message = "binding generation for function failed\nCannot use an error type by value: Can't generate bindings for NotCrubitExposed, because of missing required features (<internal link>):\n//rs_bindings_from_cc/test/golden:not_crubit_exposed needs [//features:supported] for NotCrubitExposed"
)]
pub trait BindingFailedFor_Z19UseNotCrubitExposed16NotCrubitExposed {}
#[inline(always)]
pub(crate) unsafe fn UseNotCrubitExposed<'error>(
    not_crubit_exposed: impl ::ctor::Ctor<
        Output = ::forward_declare::Incomplete<::forward_declare::symbol!("NotCrubitExposed"), ()>,
    >,
) where
    &'error (): BindingFailedFor_Z19UseNotCrubitExposed16NotCrubitExposed,
{
    #![allow(unused_variables)]
    unreachable!(
        "This impl can never be instantiated. \
                    If this message appears at runtime, please report a <internal link>."
    )
}

#[derive(Clone, Copy)]
#[repr(C, align(4))]
///CRUBIT_ANNOTATE: cpp_type=CannotUpcastInCrubit
pub struct CannotUpcastInCrubit {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 4],
}
impl !Send for CannotUpcastInCrubit {}
impl !Sync for CannotUpcastInCrubit {}
forward_declare::unsafe_define!(
    forward_declare::symbol!("CannotUpcastInCrubit"),
    crate::CannotUpcastInCrubit
);

impl Default for CannotUpcastInCrubit {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN20CannotUpcastInCrubitC1Ev(
                &raw mut tmp as *mut ::core::ffi::c_void,
            );
            tmp.assume_init()
        }
    }
}

impl From<::ctor::RvalueReference<'_, Self>> for CannotUpcastInCrubit {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'_, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN20CannotUpcastInCrubitC1EOS_(
                &raw mut tmp as *mut ::core::ffi::c_void,
                __param_0,
            );
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<::ctor::RvalueReference<'_, Self>> for CannotUpcastInCrubit {
    type CtorType = Self;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'_, Self>) -> Self::CtorType {
        <Self as From<::ctor::RvalueReference<'_, Self>>>::from(args)
    }
}

impl ::ctor::UnpinAssign<&Self> for CannotUpcastInCrubit {
    #[inline(always)]
    fn unpin_assign(&mut self, __param_0: &Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN20CannotUpcastInCrubitaSERKS_(self, __param_0);
        }
    }
}

impl ::ctor::UnpinAssign<::ctor::RvalueReference<'_, Self>> for CannotUpcastInCrubit {
    #[inline(always)]
    fn unpin_assign(&mut self, __param_0: ::ctor::RvalueReference<'_, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN20CannotUpcastInCrubitaSEOS_(self, __param_0);
        }
    }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN20CannotUpcastInCrubitC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN20CannotUpcastInCrubitC1EOS_(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'_, crate::CannotUpcastInCrubit>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN20CannotUpcastInCrubitaSERKS_<'__return_lifetime>(
            __this: &mut crate::CannotUpcastInCrubit,
            __param_0: &crate::CannotUpcastInCrubit,
        ) -> &'__return_lifetime mut crate::CannotUpcastInCrubit;
        pub(crate) unsafe fn __rust_thunk___ZN20CannotUpcastInCrubitaSEOS_<'__return_lifetime>(
            __this: &mut crate::CannotUpcastInCrubit,
            __param_0: ::ctor::RvalueReference<'_, crate::CannotUpcastInCrubit>,
        ) -> &'__return_lifetime mut crate::CannotUpcastInCrubit;
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::CannotUpcastInCrubit>() == 4);
    assert!(::core::mem::align_of::<crate::CannotUpcastInCrubit>() == 4);
    static_assertions::assert_impl_all!(crate::CannotUpcastInCrubit: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::CannotUpcastInCrubit: Drop);
};
