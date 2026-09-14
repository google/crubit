// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:user_of_base_class_cc

#![rustfmt::skip]
#![feature(cfi_encoding, custom_inner_attributes, impl_trait_in_assoc_type, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![allow(deprecated)]
#![allow(unknown_lints, suspicious_runtime_symbol_definitions)]
#![deny(warnings)]
/// The same as Derived from inheritance.h, but in a different build target.
///
/// This tests inheritance across library boundaries.
///
/// TODO(b/216195042): Correctly namespace base classes in generated Rust code.
#[::ctor::recursively_pinned]
#[cfi_encoding = "8Derived2"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=Derived2
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct Derived2 {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 20],
    pub derived_1: ::ffi_11::c_char,
}
impl !Send for Derived2 {}
impl !Sync for Derived2 {}
unsafe impl ::cxx::ExternType for Derived2 {
    type Id = ::cxx::type_id!("Derived2");
    type Kind = ::cxx::kind::Opaque;
}

impl ::ctor::CtorNew<()> for Derived2 {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk___ZN8Derived2C1Ev(
                    __crubit_dest as *mut ::core::ffi::c_void,
                );
            })
        }
    }
}

impl<'__param_0> ::ctor::CtorNew<&'__param_0 Self> for Derived2 {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__param_0>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: &'__param_0 Self) -> Self::CtorType {
        let mut __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk___ZN8Derived2C1ERKS_(
                    __crubit_dest as *mut ::core::ffi::c_void,
                    __param_0,
                );
            })
        }
    }
}
impl<'__param_0> ::ctor::CtorNew<(&'__param_0 Self,)> for Derived2 {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__param_0>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (&'__param_0 Self,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'__param_0 Self>>::ctor_new(arg)
    }
}

impl<'__unelided> ::ctor::CtorNew<::ctor::RvalueReference<'__unelided, Self>> for Derived2 {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__unelided>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'__unelided, Self>) -> Self::CtorType {
        let mut __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk___ZN8Derived2C1EOS_(
                    __crubit_dest as *mut ::core::ffi::c_void,
                    __param_0,
                );
            })
        }
    }
}
impl<'__unelided> ::ctor::CtorNew<(::ctor::RvalueReference<'__unelided, Self>,)> for Derived2 {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__unelided>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (::ctor::RvalueReference<'__unelided, Self>,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ctor::RvalueReference<'__unelided, Self>>>::ctor_new(arg)
    }
}

impl<'__param_0> ::ctor::Assign<&'__param_0 Self> for Derived2 {
    #[inline(always)]
    fn assign<'__this>(self: ::core::pin::Pin<&'__this mut Self>, __param_0: &'__param_0 Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN8Derived2aSERKS_(self, __param_0);
        }
    }
}

impl ::ctor::Assign<::ctor::RvalueReference<'_, Self>> for Derived2 {
    #[inline(always)]
    fn assign<'__this>(
        self: ::core::pin::Pin<&'__this mut Self>,
        __param_0: ::ctor::RvalueReference<'_, Self>,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN8Derived2aSEOS_(self, __param_0);
        }
    }
}

#[::ctor::recursively_pinned]
#[cfi_encoding = "15VirtualDerived2"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=VirtualDerived2
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct VirtualDerived2 {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 32],
}
impl !Send for VirtualDerived2 {}
impl !Sync for VirtualDerived2 {}
unsafe impl ::cxx::ExternType for VirtualDerived2 {
    type Id = ::cxx::type_id!("VirtualDerived2");
    type Kind = ::cxx::kind::Opaque;
}

impl ::ctor::CtorNew<()> for VirtualDerived2 {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk___ZN15VirtualDerived2C1Ev(
                    __crubit_dest as *mut ::core::ffi::c_void,
                );
            })
        }
    }
}

impl<'__param_0> ::ctor::CtorNew<&'__param_0 Self> for VirtualDerived2 {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__param_0>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: &'__param_0 Self) -> Self::CtorType {
        let mut __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk___ZN15VirtualDerived2C1ERKS_(
                    __crubit_dest as *mut ::core::ffi::c_void,
                    __param_0,
                );
            })
        }
    }
}
impl<'__param_0> ::ctor::CtorNew<(&'__param_0 Self,)> for VirtualDerived2 {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__param_0>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (&'__param_0 Self,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'__param_0 Self>>::ctor_new(arg)
    }
}

impl<'__unelided> ::ctor::CtorNew<::ctor::RvalueReference<'__unelided, Self>> for VirtualDerived2 {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__unelided>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'__unelided, Self>) -> Self::CtorType {
        let mut __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk___ZN15VirtualDerived2C1EOS_(
                    __crubit_dest as *mut ::core::ffi::c_void,
                    __param_0,
                );
            })
        }
    }
}
impl<'__unelided> ::ctor::CtorNew<(::ctor::RvalueReference<'__unelided, Self>,)>
    for VirtualDerived2
{
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__unelided>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (::ctor::RvalueReference<'__unelided, Self>,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ctor::RvalueReference<'__unelided, Self>>>::ctor_new(arg)
    }
}

impl<'__param_0> ::ctor::Assign<&'__param_0 Self> for VirtualDerived2 {
    #[inline(always)]
    fn assign<'__this>(self: ::core::pin::Pin<&'__this mut Self>, __param_0: &'__param_0 Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN15VirtualDerived2aSERKS_(self, __param_0);
        }
    }
}

impl ::ctor::Assign<::ctor::RvalueReference<'_, Self>> for VirtualDerived2 {
    #[inline(always)]
    fn assign<'__this>(
        self: ::core::pin::Pin<&'__this mut Self>,
        __param_0: ::ctor::RvalueReference<'_, Self>,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN15VirtualDerived2aSEOS_(self, __param_0);
        }
    }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN8Derived2C1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN8Derived2C1ERKS_<'__param_0>(
            __this: *mut ::core::ffi::c_void,
            __param_0: &'__param_0 crate::Derived2,
        );
        pub(crate) unsafe fn __rust_thunk___ZN8Derived2C1EOS_<'__unelided>(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'__unelided, crate::Derived2>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN8Derived2aSERKS_<'__param_0, '__this>(
            __this: ::core::pin::Pin<&'__this mut crate::Derived2>,
            __param_0: &'__param_0 crate::Derived2,
        ) -> ::core::pin::Pin<&'__this mut crate::Derived2>;
        pub(crate) unsafe fn __rust_thunk___ZN8Derived2aSEOS_<'__this>(
            __this: ::core::pin::Pin<&'__this mut crate::Derived2>,
            __param_0: ::ctor::RvalueReference<'_, crate::Derived2>,
        ) -> ::core::pin::Pin<&'__this mut crate::Derived2>;
        pub(crate) unsafe fn __rust_thunk___ZN15VirtualDerived2C1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN15VirtualDerived2C1ERKS_<'__param_0>(
            __this: *mut ::core::ffi::c_void,
            __param_0: &'__param_0 crate::VirtualDerived2,
        );
        pub(crate) unsafe fn __rust_thunk___ZN15VirtualDerived2C1EOS_<'__unelided>(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'__unelided, crate::VirtualDerived2>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN15VirtualDerived2aSERKS_<'__param_0, '__this>(
            __this: ::core::pin::Pin<&'__this mut crate::VirtualDerived2>,
            __param_0: &'__param_0 crate::VirtualDerived2,
        ) -> ::core::pin::Pin<&'__this mut crate::VirtualDerived2>;
        pub(crate) unsafe fn __rust_thunk___ZN15VirtualDerived2aSEOS_<'__this>(
            __this: ::core::pin::Pin<&'__this mut crate::VirtualDerived2>,
            __param_0: ::ctor::RvalueReference<'_, crate::VirtualDerived2>,
        ) -> ::core::pin::Pin<&'__this mut crate::VirtualDerived2>;
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::Derived2>() == 24);
    assert!(::core::mem::align_of::<crate::Derived2>() == 8);
    static_assertions::assert_not_impl_any!(crate::Derived2: Copy,Drop);
    assert!(::core::mem::offset_of!(crate::Derived2, derived_1) == 20);
    assert!(::core::mem::size_of::<crate::VirtualDerived2>() == 32);
    assert!(::core::mem::align_of::<crate::VirtualDerived2>() == 8);
    static_assertions::assert_not_impl_any!(crate::VirtualDerived2: Copy,Drop);
};
