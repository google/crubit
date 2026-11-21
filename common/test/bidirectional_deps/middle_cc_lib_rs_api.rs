// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //common/test/bidirectional_deps:middle_cc_lib

#![rustfmt::skip]
#![feature(custom_inner_attributes)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![allow(deprecated)]
#![allow(unknown_lints, suspicious_runtime_symbol_definitions)]
#![deny(warnings)]
extern crate alloc;

pub mod crubit {
    #[inline(always)]
    pub fn Wrap(x: ::ffi_11::c_uchar) -> ::leaf_rs_lib::LeafRsType {
        unsafe {
            let mut __crubit_return =
                ::core::mem::MaybeUninit::<::leaf_rs_lib::LeafRsType>::uninit();
            crate::detail::__rust_thunk___ZN6crubit4WrapEh(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                x,
            );
            __crubit_return.assume_init()
        }
    }

    #[inline(always)]
    pub fn Unwrap(x: ::leaf_rs_lib::LeafRsType) -> ::ffi_11::c_uchar {
        let mut x = ::core::mem::MaybeUninit::new(x);
        unsafe {
            crate::detail::__rust_thunk___ZN6crubit6UnwrapEN11leaf_rs_lib10LeafRsTypeE(
                x.as_mut_ptr(),
            )
        }
    }

    #[inline(always)]
    pub fn WrapEnum(x: ::ffi_11::c_uchar) -> ::leaf_rs_lib::LeafRsEnum {
        unsafe {
            let mut __crubit_return =
                ::core::mem::MaybeUninit::<::leaf_rs_lib::LeafRsEnum>::uninit();
            crate::detail::__rust_thunk___ZN6crubit8WrapEnumEh(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                x,
            );
            __crubit_return.assume_init()
        }
    }

    #[inline(always)]
    pub fn UnwrapEnum(x: ::leaf_rs_lib::LeafRsEnum) -> ::ffi_11::c_uchar {
        let mut x = ::core::mem::MaybeUninit::new(x);
        unsafe {
            crate::detail::__rust_thunk___ZN6crubit10UnwrapEnumEN11leaf_rs_lib10LeafRsEnumE(
                x.as_mut_ptr(),
            )
        }
    }

    #[inline(always)]
    pub fn MakeStaticLifetime() -> ::leaf_rs_lib::LeafRsLifetime<'static> {
        unsafe {
            let mut __crubit_return =
                ::core::mem::MaybeUninit::<::leaf_rs_lib::LeafRsLifetime<'static>>::uninit();
            crate::detail::__rust_thunk___ZN6crubit18MakeStaticLifetimeEv(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
            );
            __crubit_return.assume_init()
        }
    }

    #[inline(always)]
    pub fn MakeUnannotatedLifetime<'lt>() -> ::leaf_rs_lib::LeafRsLifetime<'lt> {
        unsafe {
            let mut __crubit_return =
                ::core::mem::MaybeUninit::<::leaf_rs_lib::LeafRsLifetime<'lt>>::uninit();
            crate::detail::__rust_thunk___ZN6crubit23MakeUnannotatedLifetimeEv(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
            );
            __crubit_return.assume_init()
        }
    }

    /// The functions below take `const unsigned char&` on purpose: the result
    /// borrows from the argument, and the reference parameters are what give it a
    /// lifetime (two of them make it ambiguous).
    ///
    /// # Safety
    ///
    /// The return value borrows data whose lifetime Crubit does not know. The caller must ensure that the borrowed data outlives the return value: unwrap it (`::crubit_support::Unsafe::into_inner`) or drop it while that data is still alive.
    #[inline(always)]
    pub unsafe fn MakeAmbiguousLifetime<'x, 'y>(
        x: &'x ::ffi_11::c_uchar,
        y: &'y ::ffi_11::c_uchar,
    ) -> ::crubit_support::Unsafe<::leaf_rs_lib::LeafRsLifetime<'static>> {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                ::crubit_support::Unsafe<::leaf_rs_lib::LeafRsLifetime<'static>>,
            >::uninit();
            crate::detail::__rust_thunk___ZN6crubit21MakeAmbiguousLifetimeERKhS1_(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                x,
                y,
            );
            __crubit_return.assume_init()
        }
    }

    #[inline(always)]
    pub fn WrapLifetime<'a>(x: &'a ::ffi_11::c_uchar) -> ::leaf_rs_lib::LeafRsLifetime<'a> {
        unsafe {
            let mut __crubit_return =
                ::core::mem::MaybeUninit::<::leaf_rs_lib::LeafRsLifetime<'a>>::uninit();
            crate::detail::__rust_thunk___ZN6crubit12WrapLifetimeERKh(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                x,
            );
            __crubit_return.assume_init()
        }
    }

    #[inline(always)]
    pub fn WrapLifetimeElided<'x>(x: &'x ::ffi_11::c_uchar) -> ::leaf_rs_lib::LeafRsLifetime<'x> {
        unsafe {
            let mut __crubit_return =
                ::core::mem::MaybeUninit::<::leaf_rs_lib::LeafRsLifetime<'x>>::uninit();
            crate::detail::__rust_thunk___ZN6crubit18WrapLifetimeElidedERKh(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                x,
            );
            __crubit_return.assume_init()
        }
    }

    #[inline(always)]
    pub fn UnwrapLifetime<'x>(x: ::leaf_rs_lib::LeafRsLifetime<'x>) -> ::ffi_11::c_uchar {
        let mut x = ::core::mem::MaybeUninit::new(x);
        unsafe {
            crate::detail::__rust_thunk___ZN6crubit14UnwrapLifetimeEN11leaf_rs_lib14LeafRsLifetimeE(
                x.as_mut_ptr(),
            )
        }
    }

    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `x`: type :: leaf_rs_lib :: LeafRsLifetime < { 'a } > has 1 lifetime parameter, but 0 were provided; callers must ensure that arguments have the appropriate lifetime
    #[inline(always)]
    pub unsafe fn UnwrapUnknownLifetime(
        x: ::crubit_support::Unsafe<::leaf_rs_lib::LeafRsLifetime<'static>>,
    ) -> ::ffi_11::c_uchar {
        let mut x = ::core::mem::MaybeUninit::new(x);
        unsafe {
            crate::detail::__rust_thunk___ZN6crubit21UnwrapUnknownLifetimeEN11leaf_rs_lib14LeafRsLifetimeE(x.as_mut_ptr())
        }
    }

    #[inline(always)]
    pub fn WrapLifetimeAlias<'a>(x: &'a ::ffi_11::c_uchar) -> ::leaf_rs_lib::LeafRsLifetime<'a> {
        unsafe {
            let mut __crubit_return =
                ::core::mem::MaybeUninit::<::leaf_rs_lib::LeafRsLifetime<'a>>::uninit();
            crate::detail::__rust_thunk___ZN6crubit17WrapLifetimeAliasERKh(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                x,
            );
            __crubit_return.assume_init()
        }
    }

    #[inline(always)]
    pub fn MakeStaticLifetimeAlias() -> ::leaf_rs_lib::StaticLeafRsLifetime {
        unsafe {
            let mut __crubit_return =
                ::core::mem::MaybeUninit::<::leaf_rs_lib::StaticLeafRsLifetime>::uninit();
            crate::detail::__rust_thunk___ZN6crubit23MakeStaticLifetimeAliasEv(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
            );
            __crubit_return.assume_init()
        }
    }

    /// `LeafRsBounded<'a, 'b: 'a, 'c>` has three lifetime parameters, two of them
    /// related by an outlives bound. C++ cannot state the bound; the Rust signatures
    /// below must still satisfy it.
    #[inline(always)]
    pub fn WrapBounded<'a, 'c>(
        x: &'a ::ffi_11::c_uchar,
        z: &'c ::ffi_11::c_uchar,
    ) -> ::leaf_rs_lib::LeafRsBounded<'a, 'a, 'c> {
        unsafe {
            let mut __crubit_return =
                ::core::mem::MaybeUninit::<::leaf_rs_lib::LeafRsBounded<'a, 'a, 'c>>::uninit();
            crate::detail::__rust_thunk___ZN6crubit11WrapBoundedERKhS1_(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                x,
                z,
            );
            __crubit_return.assume_init()
        }
    }

    #[inline(always)]
    pub fn WrapBoundedDistinct<'a, 'b, 'c>(
        x: &'a ::ffi_11::c_uchar,
        y: &'b ::ffi_11::c_uchar,
        z: &'c ::ffi_11::c_uchar,
    ) -> ::leaf_rs_lib::LeafRsBounded<'a, 'b, 'c> {
        unsafe {
            let mut __crubit_return =
                ::core::mem::MaybeUninit::<::leaf_rs_lib::LeafRsBounded<'a, 'b, 'c>>::uninit();
            crate::detail::__rust_thunk___ZN6crubit19WrapBoundedDistinctERKhS1_S1_(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                x,
                y,
                z,
            );
            __crubit_return.assume_init()
        }
    }

    #[inline(always)]
    pub fn WrapBoundedElided<'x>(
        x: &'x ::ffi_11::c_uchar,
    ) -> ::leaf_rs_lib::LeafRsBounded<'x, 'x, 'x> {
        unsafe {
            let mut __crubit_return =
                ::core::mem::MaybeUninit::<::leaf_rs_lib::LeafRsBounded<'x, 'x, 'x>>::uninit();
            crate::detail::__rust_thunk___ZN6crubit17WrapBoundedElidedERKh(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                x,
            );
            __crubit_return.assume_init()
        }
    }

    /// # Safety
    ///
    /// The return value borrows data whose lifetime Crubit does not know. The caller must ensure that the borrowed data outlives the return value: unwrap it (`::crubit_support::Unsafe::into_inner`) or drop it while that data is still alive.
    #[inline(always)]
    pub unsafe fn MakeBoundedAmbiguous<'x, 'z>(
        x: &'x ::ffi_11::c_uchar,
        z: &'z ::ffi_11::c_uchar,
    ) -> ::crubit_support::Unsafe<::leaf_rs_lib::LeafRsBounded<'static, 'static, 'static>> {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                ::crubit_support::Unsafe<::leaf_rs_lib::LeafRsBounded<'static, 'static, 'static>>,
            >::uninit();
            crate::detail::__rust_thunk___ZN6crubit20MakeBoundedAmbiguousERKhS1_(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                x,
                z,
            );
            __crubit_return.assume_init()
        }
    }

    #[inline(always)]
    pub fn UnwrapBounded<'x, 'x_0, 'x_1>(
        x: ::leaf_rs_lib::LeafRsBounded<'x, 'x_0, 'x_1>,
    ) -> ::ffi_11::c_uchar {
        let mut x = ::core::mem::MaybeUninit::new(x);
        unsafe {
            crate::detail::__rust_thunk___ZN6crubit13UnwrapBoundedEN11leaf_rs_lib13LeafRsBoundedE(
                x.as_mut_ptr(),
            )
        }
    }

    /// `LeafRsStaticBound<'a: 'static>` only admits `'static`.
    #[inline(always)]
    pub fn WrapStaticBound(
        x: &'static ::ffi_11::c_uchar,
    ) -> ::leaf_rs_lib::LeafRsStaticBound<'static> {
        unsafe {
            let mut __crubit_return =
                ::core::mem::MaybeUninit::<::leaf_rs_lib::LeafRsStaticBound<'static>>::uninit();
            crate::detail::__rust_thunk___ZN6crubit15WrapStaticBoundERKh(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                x,
            );
            __crubit_return.assume_init()
        }
    }

    #[inline(always)]
    pub fn WrapStaticBoundElided<'x>(
        x: &'x ::ffi_11::c_uchar,
    ) -> ::leaf_rs_lib::LeafRsStaticBound<'x> {
        unsafe {
            let mut __crubit_return =
                ::core::mem::MaybeUninit::<::leaf_rs_lib::LeafRsStaticBound<'x>>::uninit();
            crate::detail::__rust_thunk___ZN6crubit21WrapStaticBoundElidedERKh(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                x,
            );
            __crubit_return.assume_init()
        }
    }

    #[inline(always)]
    pub fn UnwrapStaticBound<'x>(x: ::leaf_rs_lib::LeafRsStaticBound<'x>) -> ::ffi_11::c_uchar {
        let mut x = ::core::mem::MaybeUninit::new(x);
        unsafe {
            crate::detail::__rust_thunk___ZN6crubit17UnwrapStaticBoundEN11leaf_rs_lib17LeafRsStaticBoundE(x.as_mut_ptr())
        }
    }

    /// A callback whose parameter type has a lifetime parameter. The argument only
    /// lives for the call, so the Rust parameter type takes any lifetime
    /// (`dyn Fn(LeafRsLifetime<'_>)`) rather than being wrapped in `Unsafe`.
    #[inline(always)]
    pub fn CallWithLifetime(
        f: ::alloc::boxed::Box<
            dyn ::core::ops::Fn(::leaf_rs_lib::LeafRsLifetime<'_>) -> ::ffi_11::c_uchar
                + ::core::marker::Send
                + ::core::marker::Sync
                + 'static,
        >,
    ) -> ::ffi_11::c_uchar {
        unsafe {
            crate::detail::__rust_thunk___ZN6crubit16CallWithLifetimeEN4absl12AnyInvocableIKFhN11leaf_rs_lib14LeafRsLifetimeEEEE(::crubit_support::bridge::unstable_encode!(@::any_invocable::AnyInvocableAbi::<dyn::core::ops::Fn(::leaf_rs_lib::LeafRsLifetime<'_>)->::ffi_11::c_uchar+::core::marker::Send+::core::marker::Sync+'static>::new(::alloc::boxed::Box::new(|_: ::leaf_rs_lib::LeafRsLifetime<'_>|->::ffi_11::c_uchar{ ::core::panic!("moved-from value") }),|raw_any_invocable: ::cc_std::std::unique_ptr<::any_invocable::RawAnyInvocable>|->::alloc::boxed::Box<dyn::core::ops::Fn(::leaf_rs_lib::LeafRsLifetime<'_>)->::ffi_11::c_uchar+::core::marker::Send+::core::marker::Sync+'static>{ ::alloc::boxed::Box::new(move|param_0: ::leaf_rs_lib::LeafRsLifetime<'_>|->::ffi_11::c_uchar{ unsafe{ crate::detail::__crubit_invoke_any_invocable___CcTemplateInstN4absl12AnyInvocableIKFhN11leaf_rs_lib14LeafRsLifetimeEEEE__2f_2fthird_5fparty_2fcrubit_2fcommon_2ftest_2fbidirectional_5fdeps_3amiddle_5fcc_5flib(::cc_std::std::unique_ptr::as_ptr(&raw_any_invocable)as*mut _,::core::mem::MaybeUninit::new(param_0).as_mut_ptr()) } }) },),::any_invocable::AnyInvocableAbi<dyn::core::ops::Fn(::leaf_rs_lib::LeafRsLifetime<'_>)->::ffi_11::c_uchar+::core::marker::Send+::core::marker::Sync+'static>,f).as_ptr()as*const u8)
        }
    }
}

// namespace crubit

pub type LeafRsTypeAlias2 = ::leaf_rs_lib::LeafRsType;

pub type LeafRsLifetimeAlias2<'__alias0> = ::leaf_rs_lib::LeafRsLifetime<'__alias0>;

/// Class template instantiations that aren't named by a C++ type alias.
pub mod zz_template_instantiations {
    // error: class `absl::internal_any_invocable::CoreImpl<false, unsigned char, struct leaf_rs_lib::LeafRsLifetime>` could not be bound
    //   crubit.rs/errors/unknown_target: the type is defined in
    //     third_party/absl/functional/internal/any_invocable.h
    //   which is not a public header of any Crubit-enabled target that
    //     //common/test/bidirectional_deps:middle_cc_lib
    //   depends on directly. Either enable Crubit on the library that provides the header,
    //   add that library as a direct dependency (and `#include` the header directly),
    //   or move the type to a public header.
    //   template instantiation is not yet supported
}
mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4WrapEh(
            __return: *mut ::core::ffi::c_void,
            x: ::ffi_11::c_uchar,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6crubit6UnwrapEN11leaf_rs_lib10LeafRsTypeE(
            x: *mut ::leaf_rs_lib::LeafRsType,
        ) -> ::ffi_11::c_uchar;
        pub(crate) unsafe fn __rust_thunk___ZN6crubit8WrapEnumEh(
            __return: *mut ::core::ffi::c_void,
            x: ::ffi_11::c_uchar,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6crubit10UnwrapEnumEN11leaf_rs_lib10LeafRsEnumE(
            x: *mut ::leaf_rs_lib::LeafRsEnum,
        ) -> ::ffi_11::c_uchar;
        pub(crate) unsafe fn __rust_thunk___ZN6crubit18MakeStaticLifetimeEv(
            __return: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6crubit23MakeUnannotatedLifetimeEv<'lt>(
            __return: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6crubit21MakeAmbiguousLifetimeERKhS1_<'x, 'y>(
            __return: *mut ::core::ffi::c_void,
            x: &'x ::ffi_11::c_uchar,
            y: &'y ::ffi_11::c_uchar,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6crubit12WrapLifetimeERKh<'a>(
            __return: *mut ::core::ffi::c_void,
            x: &'a ::ffi_11::c_uchar,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6crubit18WrapLifetimeElidedERKh<'x>(
            __return: *mut ::core::ffi::c_void,
            x: &'x ::ffi_11::c_uchar,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6crubit14UnwrapLifetimeEN11leaf_rs_lib14LeafRsLifetimeE<
            'x,
        >(
            x: *mut ::leaf_rs_lib::LeafRsLifetime<'x>,
        ) -> ::ffi_11::c_uchar;
        pub(crate) unsafe fn __rust_thunk___ZN6crubit21UnwrapUnknownLifetimeEN11leaf_rs_lib14LeafRsLifetimeE(
            x: *mut ::crubit_support::Unsafe<::leaf_rs_lib::LeafRsLifetime<'static>>,
        ) -> ::ffi_11::c_uchar;
        pub(crate) unsafe fn __rust_thunk___ZN6crubit17WrapLifetimeAliasERKh<'a>(
            __return: *mut ::core::ffi::c_void,
            x: &'a ::ffi_11::c_uchar,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6crubit23MakeStaticLifetimeAliasEv(
            __return: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6crubit11WrapBoundedERKhS1_<'a, 'c>(
            __return: *mut ::core::ffi::c_void,
            x: &'a ::ffi_11::c_uchar,
            z: &'c ::ffi_11::c_uchar,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6crubit19WrapBoundedDistinctERKhS1_S1_<'a, 'b, 'c>(
            __return: *mut ::core::ffi::c_void,
            x: &'a ::ffi_11::c_uchar,
            y: &'b ::ffi_11::c_uchar,
            z: &'c ::ffi_11::c_uchar,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6crubit17WrapBoundedElidedERKh<'x>(
            __return: *mut ::core::ffi::c_void,
            x: &'x ::ffi_11::c_uchar,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6crubit20MakeBoundedAmbiguousERKhS1_<'x, 'z>(
            __return: *mut ::core::ffi::c_void,
            x: &'x ::ffi_11::c_uchar,
            z: &'z ::ffi_11::c_uchar,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6crubit13UnwrapBoundedEN11leaf_rs_lib13LeafRsBoundedE<
            'x,
            'x_0,
            'x_1,
        >(
            x: *mut ::leaf_rs_lib::LeafRsBounded<'x, 'x_0, 'x_1>,
        ) -> ::ffi_11::c_uchar;
        pub(crate) unsafe fn __rust_thunk___ZN6crubit15WrapStaticBoundERKh(
            __return: *mut ::core::ffi::c_void,
            x: &'static ::ffi_11::c_uchar,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6crubit21WrapStaticBoundElidedERKh<'x>(
            __return: *mut ::core::ffi::c_void,
            x: &'x ::ffi_11::c_uchar,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6crubit17UnwrapStaticBoundEN11leaf_rs_lib17LeafRsStaticBoundE<
            'x,
        >(
            x: *mut ::leaf_rs_lib::LeafRsStaticBound<'x>,
        ) -> ::ffi_11::c_uchar;
        pub(crate) unsafe fn __rust_thunk___ZN6crubit16CallWithLifetimeEN4absl12AnyInvocableIKFhN11leaf_rs_lib14LeafRsLifetimeEEEE(
            f: *const ::core::ffi::c_uchar,
        ) -> ::ffi_11::c_uchar;
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __crubit_invoker___CcTemplateInstN4absl12AnyInvocableIKFhN11leaf_rs_lib14LeafRsLifetimeEEEE__2f_2fthird_5fparty_2fcrubit_2fcommon_2ftest_2fbidirectional_5fdeps_3amiddle_5fcc_5flib(
        f: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn(::leaf_rs_lib::LeafRsLifetime<'_>) -> ::ffi_11::c_uchar
                + ::core::marker::Send
                + ::core::marker::Sync
                + 'static,
        >,
        param_0: *mut ::leaf_rs_lib::LeafRsLifetime<'_>,
    ) -> ::ffi_11::c_uchar {
        let param_0 = unsafe { ::core::ptr::read(param_0) };
        (unsafe { &*f })(param_0)
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __crubit_manager___CcTemplateInstN4absl12AnyInvocableIKFhN11leaf_rs_lib14LeafRsLifetimeEEEE__2f_2fthird_5fparty_2fcrubit_2fcommon_2ftest_2fbidirectional_5fdeps_3amiddle_5fcc_5flib(
        operation: ::dyn_callable_rs::FunctionToCall,
        from: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn(::leaf_rs_lib::LeafRsLifetime<'_>) -> ::ffi_11::c_uchar
                + ::core::marker::Send
                + ::core::marker::Sync
                + 'static,
        >,
        to: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn(::leaf_rs_lib::LeafRsLifetime<'_>) -> ::ffi_11::c_uchar
                + ::core::marker::Send
                + ::core::marker::Sync
                + 'static,
        >,
    ) {
        unsafe {
            ::dyn_callable_rs::manager(operation, from, to);
        }
    }
    unsafe extern "C" {
        pub(crate) unsafe fn __crubit_invoke_any_invocable___CcTemplateInstN4absl12AnyInvocableIKFhN11leaf_rs_lib14LeafRsLifetimeEEEE__2f_2fthird_5fparty_2fcrubit_2fcommon_2ftest_2fbidirectional_5fdeps_3amiddle_5fcc_5flib(
            f: *mut ::any_invocable::RawAnyInvocable,
            param_0: *mut ::leaf_rs_lib::LeafRsLifetime<'_>,
        ) -> ::ffi_11::c_uchar;
    }
}
