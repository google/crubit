// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //common/test/bidirectional_deps:str_ref_cc_lib

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
    /// The result borrows from `s`: `HoldStr<'s>(s: &'s str) -> LeafRsStr<'s>`.
    #[inline(always)]
    pub fn HoldStr<'s>(s: &'s str) -> ::leaf_rs_lib::LeafRsStr<'s> {
        let mut s = ::core::mem::MaybeUninit::new(s);
        unsafe {
            let mut __crubit_return =
                ::core::mem::MaybeUninit::<::leaf_rs_lib::LeafRsStr<'s>>::uninit();
            crate::detail::__rust_thunk___ZN6crubit7HoldStrEN2rs6StrRefE(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                s.as_mut_ptr(),
            );
            __crubit_return.assume_init()
        }
    }

    /// Two inputs with lifetimes: the result's lifetime is ambiguous, so it is
    /// wrapped in `Unsafe`.
    ///
    /// # Safety
    ///
    /// The return value borrows data whose lifetime Crubit does not know. The caller must ensure that the borrowed data outlives the return value: unwrap it (`::crubit_support::Unsafe::into_inner`) or drop it while that data is still alive.
    #[inline(always)]
    pub unsafe fn HoldStrAmbiguous<'s, 'x>(
        s: &'s str,
        x: &'x ::ffi_11::c_uchar,
    ) -> ::crubit_support::Unsafe<::leaf_rs_lib::LeafRsStr<'static>> {
        let mut s = ::core::mem::MaybeUninit::new(s);
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                ::crubit_support::Unsafe<::leaf_rs_lib::LeafRsStr<'static>>,
            >::uninit();
            crate::detail::__rust_thunk___ZN6crubit16HoldStrAmbiguousEN2rs6StrRefERKh(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                s.as_mut_ptr(),
                x,
            );
            __crubit_return.assume_init()
        }
    }

    #[inline(always)]
    pub fn FirstByte<'h>(h: ::leaf_rs_lib::LeafRsStr<'h>) -> ::ffi_11::c_uchar {
        let mut h = ::core::mem::MaybeUninit::new(h);
        unsafe {
            crate::detail::__rust_thunk___ZN6crubit9FirstByteEN11leaf_rs_lib9LeafRsStrE(
                h.as_mut_ptr(),
            )
        }
    }

    /// `Echo<'s>(s: &'s str) -> &'s str`.
    #[inline(always)]
    pub fn Echo<'s>(s: &'s str) -> &'s str {
        let mut s = ::core::mem::MaybeUninit::new(s);
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<&'s str>::uninit();
            crate::detail::__rust_thunk___ZN6crubit4EchoEN2rs6StrRefE(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                s.as_mut_ptr(),
            );
            __crubit_return.assume_init()
        }
    }

    /// Two `rs::StrRef` inputs: ambiguous.
    ///
    /// # Safety
    ///
    /// The return value borrows data whose lifetime Crubit does not know. The caller must ensure that the borrowed data outlives the return value: unwrap it (`::crubit_support::Unsafe::into_inner`) or drop it while that data is still alive.
    #[inline(always)]
    pub unsafe fn Pick<'a, 'b>(a: &'a str, b: &'b str) -> ::crubit_support::Unsafe<&'static str> {
        let mut a = ::core::mem::MaybeUninit::new(a);
        let mut b = ::core::mem::MaybeUninit::new(b);
        unsafe {
            let mut __crubit_return =
                ::core::mem::MaybeUninit::<::crubit_support::Unsafe<&'static str>>::uninit();
            crate::detail::__rust_thunk___ZN6crubit4PickEN2rs6StrRefES1_(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                a.as_mut_ptr(),
                b.as_mut_ptr(),
            );
            __crubit_return.assume_init()
        }
    }

    /// A callback taking `rs::StrRef`: the argument only lives for the call, so the
    /// Rust parameter type is `dyn Fn(&str)` (any lifetime), not `Unsafe<..>`.
    #[inline(always)]
    pub fn CallWithStr(
        f: ::alloc::boxed::Box<
            dyn ::core::ops::Fn(&'_ str) -> ::ffi_11::c_uchar
                + ::core::marker::Send
                + ::core::marker::Sync
                + 'static,
        >,
    ) -> ::ffi_11::c_uchar {
        unsafe {
            crate::detail::__rust_thunk___ZN6crubit11CallWithStrEN4absl12AnyInvocableIKFhN2rs6StrRefEEEE(::crubit_support::bridge::unstable_encode!(@::any_invocable::AnyInvocableAbi::<dyn::core::ops::Fn(&'_ str)->::ffi_11::c_uchar+::core::marker::Send+::core::marker::Sync+'static>::new(::alloc::boxed::Box::new(|_: &'_ str|->::ffi_11::c_uchar{ ::core::panic!("moved-from value") }),|raw_any_invocable: ::cc_std::std::unique_ptr<::any_invocable::RawAnyInvocable>|->::alloc::boxed::Box<dyn::core::ops::Fn(&'_ str)->::ffi_11::c_uchar+::core::marker::Send+::core::marker::Sync+'static>{ ::alloc::boxed::Box::new(move|param_0: &'_ str|->::ffi_11::c_uchar{ unsafe{ crate::detail::__crubit_invoke_any_invocable___CcTemplateInstN4absl12AnyInvocableIKFhN2rs6StrRefEEEE__2f_2fthird_5fparty_2fcrubit_2fcommon_2ftest_2fbidirectional_5fdeps_3astr_5fref_5fcc_5flib(::cc_std::std::unique_ptr::as_ptr(&raw_any_invocable)as*mut _,::core::mem::MaybeUninit::new(param_0).as_mut_ptr()) } }) },),::any_invocable::AnyInvocableAbi<dyn::core::ops::Fn(&'_ str)->::ffi_11::c_uchar+::core::marker::Send+::core::marker::Sync+'static>,f).as_ptr()as*const u8)
        }
    }
}

// namespace crubit

/// Class template instantiations that aren't named by a C++ type alias.
pub mod zz_template_instantiations {
    // error: class `absl::internal_any_invocable::CoreImpl<false, unsigned char, class rs::StrRef>` could not be bound
    //   crubit.rs/errors/unknown_target: the type is defined in
    //     third_party/absl/functional/internal/any_invocable.h
    //   which is not a public header of any Crubit-enabled target that
    //     //common/test/bidirectional_deps:str_ref_cc_lib
    //   depends on directly. Either enable Crubit on the library that provides the header,
    //   add that library as a direct dependency (and `#include` the header directly),
    //   or move the type to a public header.
    //   template instantiation is not yet supported
}
mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN6crubit7HoldStrEN2rs6StrRefE<'s>(
            __return: *mut ::core::ffi::c_void,
            s: *mut &'s str,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6crubit16HoldStrAmbiguousEN2rs6StrRefERKh<'s, 'x>(
            __return: *mut ::core::ffi::c_void,
            s: *mut &'s str,
            x: &'x ::ffi_11::c_uchar,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6crubit9FirstByteEN11leaf_rs_lib9LeafRsStrE<'h>(
            h: *mut ::leaf_rs_lib::LeafRsStr<'h>,
        ) -> ::ffi_11::c_uchar;
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4EchoEN2rs6StrRefE<'s>(
            __return: *mut ::core::ffi::c_void,
            s: *mut &'s str,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4PickEN2rs6StrRefES1_<'a, 'b>(
            __return: *mut ::core::ffi::c_void,
            a: *mut &'a str,
            b: *mut &'b str,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6crubit11CallWithStrEN4absl12AnyInvocableIKFhN2rs6StrRefEEEE(
            f: *const ::core::ffi::c_uchar,
        ) -> ::ffi_11::c_uchar;
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __crubit_invoker___CcTemplateInstN4absl12AnyInvocableIKFhN2rs6StrRefEEEE__2f_2fthird_5fparty_2fcrubit_2fcommon_2ftest_2fbidirectional_5fdeps_3astr_5fref_5fcc_5flib(
        f: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn(&'_ str) -> ::ffi_11::c_uchar
                + ::core::marker::Send
                + ::core::marker::Sync
                + 'static,
        >,
        param_0: *mut &'_ str,
    ) -> ::ffi_11::c_uchar {
        let param_0 = unsafe { ::core::ptr::read(param_0) };
        (unsafe { &*f })(param_0)
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __crubit_manager___CcTemplateInstN4absl12AnyInvocableIKFhN2rs6StrRefEEEE__2f_2fthird_5fparty_2fcrubit_2fcommon_2ftest_2fbidirectional_5fdeps_3astr_5fref_5fcc_5flib(
        operation: ::dyn_callable_rs::FunctionToCall,
        from: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn(&'_ str) -> ::ffi_11::c_uchar
                + ::core::marker::Send
                + ::core::marker::Sync
                + 'static,
        >,
        to: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn(&'_ str) -> ::ffi_11::c_uchar
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
        pub(crate) unsafe fn __crubit_invoke_any_invocable___CcTemplateInstN4absl12AnyInvocableIKFhN2rs6StrRefEEEE__2f_2fthird_5fparty_2fcrubit_2fcommon_2ftest_2fbidirectional_5fdeps_3astr_5fref_5fcc_5flib(
            f: *mut ::any_invocable::RawAnyInvocable,
            param_0: *mut &'_ str,
        ) -> ::ffi_11::c_uchar;
    }
}
