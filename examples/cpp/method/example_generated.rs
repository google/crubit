// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //examples/cpp/method:example_lib
// Features: infer_operator_lifetimes, supported, unsafe_types

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![no_std]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

pub mod foo {
    /// Generated from: examples/cpp/method/example.h;l=12
    #[derive(Clone, Copy)]
    #[repr(C)]
    ///CRUBIT_ANNOTATE: cpp_type=foo :: Bar
    pub struct Bar {
        pub x: ::core::ffi::c_int,
    }
    impl !Send for Bar {}
    impl !Sync for Bar {}
    unsafe impl ::cxx::ExternType for Bar {
        type Id = ::cxx::type_id!("foo :: Bar");
        type Kind = ::cxx::kind::Trivial;
    }

    /// Generated from: examples/cpp/method/example.h;l=12
    impl Default for Bar {
        #[inline(always)]
        fn default() -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk___ZN3foo3BarC1Ev(
                    &raw mut tmp as *mut ::core::ffi::c_void,
                );
                tmp.assume_init()
            }
        }
    }

    /// Generated from: examples/cpp/method/example.h;l=12
    impl From<::ctor::RvalueReference<'_, Self>> for Bar {
        #[inline(always)]
        fn from(__param_0: ::ctor::RvalueReference<'_, Self>) -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk___ZN3foo3BarC1EOS0_(
                    &raw mut tmp as *mut ::core::ffi::c_void,
                    __param_0,
                );
                tmp.assume_init()
            }
        }
    }
    impl ::ctor::CtorNew<::ctor::RvalueReference<'_, Self>> for Bar {
        type CtorType = Self;
        type Error = ::ctor::Infallible;
        #[inline(always)]
        fn ctor_new(args: ::ctor::RvalueReference<'_, Self>) -> Self::CtorType {
            <Self as From<::ctor::RvalueReference<'_, Self>>>::from(args)
        }
    }

    /// Generated from: examples/cpp/method/example.h;l=12
    impl ::ctor::UnpinAssign<&Self> for Bar {
        #[inline(always)]
        fn unpin_assign(&mut self, __param_0: &Self) {
            unsafe {
                crate::detail::__rust_thunk___ZN3foo3BaraSERKS0_(self, __param_0);
            }
        }
    }

    /// Generated from: examples/cpp/method/example.h;l=12
    impl ::ctor::UnpinAssign<::ctor::RvalueReference<'_, Self>> for Bar {
        #[inline(always)]
        fn unpin_assign(&mut self, __param_0: ::ctor::RvalueReference<'_, Self>) {
            unsafe {
                crate::detail::__rust_thunk___ZN3foo3BaraSEOS0_(self, __param_0);
            }
        }
    }

    impl Bar {
        /// Generated from: examples/cpp/method/example.h;l=14
        #[inline(always)]
        pub unsafe fn MyMethod(__this: *mut Self) {
            crate::detail::__rust_thunk___ZN3foo3Bar8MyMethodEv(__this)
        }
    }
}

// namespace foo

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN3foo3BarC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN3foo3BarC1EOS0_(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'_, crate::foo::Bar>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN3foo3BaraSERKS0_<'__return_lifetime>(
            __this: &mut crate::foo::Bar,
            __param_0: &crate::foo::Bar,
        ) -> &'__return_lifetime mut crate::foo::Bar;
        pub(crate) unsafe fn __rust_thunk___ZN3foo3BaraSEOS0_<'__return_lifetime>(
            __this: &mut crate::foo::Bar,
            __param_0: ::ctor::RvalueReference<'_, crate::foo::Bar>,
        ) -> &'__return_lifetime mut crate::foo::Bar;
        pub(crate) unsafe fn __rust_thunk___ZN3foo3Bar8MyMethodEv(__this: *mut crate::foo::Bar);
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::foo::Bar>() == 4);
    assert!(::core::mem::align_of::<crate::foo::Bar>() == 4);
    static_assertions::assert_impl_all!(crate::foo::Bar: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::foo::Bar: Drop);
    assert!(::core::mem::offset_of!(crate::foo::Bar, x) == 0);
};
