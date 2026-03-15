// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/types_only:types_only
// Features: types

#![rustfmt::skip]
#![feature(custom_inner_attributes, impl_trait_in_assoc_type, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![deny(warnings)]

/// Generated from: rs_bindings_from_cc/test/types_only/types_only.h;l=11
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=Copyable
pub struct Copyable {
    pub field: ::ffi_11::c_int,
}
impl !Send for Copyable {}
impl !Sync for Copyable {}
unsafe impl ::cxx::ExternType for Copyable {
    type Id = ::cxx::type_id!("Copyable");
    type Kind = ::cxx::kind::Trivial;
}

/// Generated from: rs_bindings_from_cc/test/types_only/types_only.h;l=11
impl Default for Copyable {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN8CopyableC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/types_only/types_only.h;l=17
#[::ctor::recursively_pinned(PinnedDrop)]
#[repr(C, align(4))]
///CRUBIT_ANNOTATE: cpp_type=Cloneable
pub struct Cloneable {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) field_: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 4],
}
impl !Send for Cloneable {}
impl !Sync for Cloneable {}
unsafe impl ::cxx::ExternType for Cloneable {
    type Id = ::cxx::type_id!("Cloneable");
    type Kind = ::cxx::kind::Opaque;
}

/// Generated from: rs_bindings_from_cc/test/types_only/types_only.h;l=19
impl ::ctor::CtorNew<::ffi_11::c_int> for Cloneable {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ffi_11::c_int) -> Self::CtorType {
        let mut field = args;
        unsafe {
            ::ctor::FnCtor::new(move |dest: *mut Self| {
                crate::detail::__rust_thunk___ZN9CloneableC1Ei(
                    dest as *mut ::core::ffi::c_void,
                    field,
                );
            })
        }
    }
}
impl ::ctor::CtorNew<(::ffi_11::c_int,)> for Cloneable {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (::ffi_11::c_int,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ffi_11::c_int>>::ctor_new(arg)
    }
}

/// Generated from: rs_bindings_from_cc/test/types_only/types_only.h;l=20
impl<'__unelided> ::ctor::CtorNew<&'__unelided Self> for Cloneable {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__unelided>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: &'__unelided Self) -> Self::CtorType {
        let mut __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(move |dest: *mut Self| {
                crate::detail::__rust_thunk___ZN9CloneableC1ERKS_(
                    dest as *mut ::core::ffi::c_void,
                    __param_0,
                );
            })
        }
    }
}
impl<'__unelided> ::ctor::CtorNew<(&'__unelided Self,)> for Cloneable {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__unelided>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (&'__unelided Self,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'__unelided Self>>::ctor_new(arg)
    }
}

/// Generated from: rs_bindings_from_cc/test/types_only/types_only.h;l=21
impl<'__unelided> ::ctor::CtorNew<::ctor::RvalueReference<'__unelided, Self>> for Cloneable {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__unelided>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'__unelided, Self>) -> Self::CtorType {
        let mut __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(move |dest: *mut Self| {
                crate::detail::__rust_thunk___ZN9CloneableC1EOS_(
                    dest as *mut ::core::ffi::c_void,
                    __param_0,
                );
            })
        }
    }
}
impl<'__unelided> ::ctor::CtorNew<(::ctor::RvalueReference<'__unelided, Self>,)> for Cloneable {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__unelided>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (::ctor::RvalueReference<'__unelided, Self>,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ctor::RvalueReference<'__unelided, Self>>>::ctor_new(arg)
    }
}

/// Generated from: rs_bindings_from_cc/test/types_only/types_only.h;l=22
impl ::ctor::Assign<&Self> for Cloneable {
    #[inline(always)]
    fn assign(self: ::core::pin::Pin<&mut Self>, __param_0: &Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN9CloneableaSERKS_(self, __param_0);
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/types_only/types_only.h;l=23
impl ::ctor::Assign<::ctor::RvalueReference<'_, Self>> for Cloneable {
    #[inline(always)]
    fn assign(self: ::core::pin::Pin<&mut Self>, __param_0: ::ctor::RvalueReference<'_, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN9CloneableaSEOS_(self, __param_0);
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/types_only/types_only.h;l=24
impl ::ctor::PinnedDrop for Cloneable {
    #[inline(always)]
    unsafe fn pinned_drop<'a>(self: ::core::pin::Pin<&'a mut Self>) {
        crate::detail::__rust_thunk___ZN9CloneableD1Ev(self)
    }
}

/// Generated from: rs_bindings_from_cc/test/types_only/types_only.h;l=32
#[::ctor::recursively_pinned(PinnedDrop)]
#[repr(C, align(4))]
///CRUBIT_ANNOTATE: cpp_type=Movable
pub struct Movable {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) field_: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 4],
}
impl !Send for Movable {}
impl !Sync for Movable {}
unsafe impl ::cxx::ExternType for Movable {
    type Id = ::cxx::type_id!("Movable");
    type Kind = ::cxx::kind::Opaque;
}

/// Generated from: rs_bindings_from_cc/test/types_only/types_only.h;l=34
impl ::ctor::CtorNew<::ffi_11::c_int> for Movable {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ffi_11::c_int) -> Self::CtorType {
        let mut field = args;
        unsafe {
            ::ctor::FnCtor::new(move |dest: *mut Self| {
                crate::detail::__rust_thunk___ZN7MovableC1Ei(
                    dest as *mut ::core::ffi::c_void,
                    field,
                );
            })
        }
    }
}
impl ::ctor::CtorNew<(::ffi_11::c_int,)> for Movable {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (::ffi_11::c_int,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ffi_11::c_int>>::ctor_new(arg)
    }
}

/// Generated from: rs_bindings_from_cc/test/types_only/types_only.h;l=35
impl<'__unelided> ::ctor::CtorNew<::ctor::RvalueReference<'__unelided, Self>> for Movable {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__unelided>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'__unelided, Self>) -> Self::CtorType {
        let mut other = args;
        unsafe {
            ::ctor::FnCtor::new(move |dest: *mut Self| {
                crate::detail::__rust_thunk___ZN7MovableC1EOS_(
                    dest as *mut ::core::ffi::c_void,
                    other,
                );
            })
        }
    }
}
impl<'__unelided> ::ctor::CtorNew<(::ctor::RvalueReference<'__unelided, Self>,)> for Movable {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__unelided>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (::ctor::RvalueReference<'__unelided, Self>,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ctor::RvalueReference<'__unelided, Self>>>::ctor_new(arg)
    }
}

/// Generated from: rs_bindings_from_cc/test/types_only/types_only.h;l=36
impl ::ctor::Assign<::ctor::RvalueReference<'_, Self>> for Movable {
    #[inline(always)]
    fn assign(self: ::core::pin::Pin<&mut Self>, other: ::ctor::RvalueReference<'_, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN7MovableaSEOS_(self, other);
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/types_only/types_only.h;l=41
impl ::ctor::PinnedDrop for Movable {
    #[inline(always)]
    unsafe fn pinned_drop<'a>(self: ::core::pin::Pin<&'a mut Self>) {
        crate::detail::__rust_thunk___ZN7MovableD1Ev(self)
    }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN8CopyableC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN9CloneableC1Ei(
            __this: *mut ::core::ffi::c_void,
            field: ::ffi_11::c_int,
        );
        pub(crate) unsafe fn __rust_thunk___ZN9CloneableC1ERKS_<'__unelided>(
            __this: *mut ::core::ffi::c_void,
            __param_0: &'__unelided crate::Cloneable,
        );
        pub(crate) unsafe fn __rust_thunk___ZN9CloneableC1EOS_<'__unelided>(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'__unelided, crate::Cloneable>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN9CloneableaSERKS_<'__return_lifetime>(
            __this: ::core::pin::Pin<&mut crate::Cloneable>,
            __param_0: &crate::Cloneable,
        ) -> ::core::pin::Pin<&'__return_lifetime mut crate::Cloneable>;
        pub(crate) unsafe fn __rust_thunk___ZN9CloneableaSEOS_<'__return_lifetime>(
            __this: ::core::pin::Pin<&mut crate::Cloneable>,
            __param_0: ::ctor::RvalueReference<'_, crate::Cloneable>,
        ) -> ::core::pin::Pin<&'__return_lifetime mut crate::Cloneable>;
        pub(crate) unsafe fn __rust_thunk___ZN9CloneableD1Ev<'a>(
            __this: ::core::pin::Pin<&'a mut crate::Cloneable>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN7MovableC1Ei(
            __this: *mut ::core::ffi::c_void,
            field: ::ffi_11::c_int,
        );
        pub(crate) unsafe fn __rust_thunk___ZN7MovableC1EOS_<'__unelided>(
            __this: *mut ::core::ffi::c_void,
            other: ::ctor::RvalueReference<'__unelided, crate::Movable>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN7MovableaSEOS_<'__return_lifetime>(
            __this: ::core::pin::Pin<&mut crate::Movable>,
            other: ::ctor::RvalueReference<'_, crate::Movable>,
        ) -> ::core::pin::Pin<&'__return_lifetime mut crate::Movable>;
        pub(crate) unsafe fn __rust_thunk___ZN7MovableD1Ev<'a>(
            __this: ::core::pin::Pin<&'a mut crate::Movable>,
        );
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::Copyable>() == 4);
    assert!(::core::mem::align_of::<crate::Copyable>() == 4);
    static_assertions::assert_impl_all!(crate::Copyable: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::Copyable: Drop);
    assert!(::core::mem::offset_of!(crate::Copyable, field) == 0);
    assert!(::core::mem::size_of::<crate::Cloneable>() == 4);
    assert!(::core::mem::align_of::<crate::Cloneable>() == 4);
    static_assertions::assert_impl_all!(crate::Cloneable: Drop);
    static_assertions::assert_not_impl_any!(crate::Cloneable: Copy);
    assert!(::core::mem::offset_of!(crate::Cloneable, field_) == 0);
    assert!(::core::mem::size_of::<crate::Movable>() == 4);
    assert!(::core::mem::align_of::<crate::Movable>() == 4);
    static_assertions::assert_impl_all!(crate::Movable: Drop);
    static_assertions::assert_not_impl_any!(crate::Movable: Copy);
    assert!(::core::mem::offset_of!(crate::Movable, field_) == 0);
};
