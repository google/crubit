// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:unsafe_types_transitive_cc

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
///CRUBIT_ANNOTATE: cpp_type=PublicPointer
pub struct PublicPointer {
    pub p: *mut ::core::ffi::c_int,
}
impl !Send for PublicPointer {}
impl !Sync for PublicPointer {}
unsafe impl ::cxx::ExternType for PublicPointer {
    type Id = ::cxx::type_id!("PublicPointer");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(forward_declare::symbol!("PublicPointer"), crate::PublicPointer);

impl Default for PublicPointer {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN13PublicPointerC1Ev(
                &raw mut tmp as *mut ::core::ffi::c_void,
            );
            tmp.assume_init()
        }
    }
}

impl From<::ctor::RvalueReference<'_, Self>> for PublicPointer {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'_, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN13PublicPointerC1EOS_(
                &raw mut tmp as *mut ::core::ffi::c_void,
                __param_0,
            );
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<::ctor::RvalueReference<'_, Self>> for PublicPointer {
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'_, Self>) -> Self::CtorType {
        <Self as From<::ctor::RvalueReference<'_, Self>>>::from(args)
    }
}

impl ::ctor::UnpinAssign<&Self> for PublicPointer {
    #[inline(always)]
    fn unpin_assign(&mut self, __param_0: &Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN13PublicPointeraSERKS_(self, __param_0);
        }
    }
}

impl ::ctor::UnpinAssign<::ctor::RvalueReference<'_, Self>> for PublicPointer {
    #[inline(always)]
    fn unpin_assign(&mut self, __param_0: ::ctor::RvalueReference<'_, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN13PublicPointeraSEOS_(self, __param_0);
        }
    }
}

#[derive(Clone, Copy)]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=PrivatePointer
pub struct PrivatePointer {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) p_: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for PrivatePointer {}
impl !Sync for PrivatePointer {}
unsafe impl ::cxx::ExternType for PrivatePointer {
    type Id = ::cxx::type_id!("PrivatePointer");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(forward_declare::symbol!("PrivatePointer"), crate::PrivatePointer);

impl Default for PrivatePointer {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN14PrivatePointerC1Ev(
                &raw mut tmp as *mut ::core::ffi::c_void,
            );
            tmp.assume_init()
        }
    }
}

impl From<::ctor::RvalueReference<'_, Self>> for PrivatePointer {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'_, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN14PrivatePointerC1EOS_(
                &raw mut tmp as *mut ::core::ffi::c_void,
                __param_0,
            );
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<::ctor::RvalueReference<'_, Self>> for PrivatePointer {
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'_, Self>) -> Self::CtorType {
        <Self as From<::ctor::RvalueReference<'_, Self>>>::from(args)
    }
}

impl ::ctor::UnpinAssign<&Self> for PrivatePointer {
    #[inline(always)]
    fn unpin_assign(&mut self, __param_0: &Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN14PrivatePointeraSERKS_(self, __param_0);
        }
    }
}

impl ::ctor::UnpinAssign<::ctor::RvalueReference<'_, Self>> for PrivatePointer {
    #[inline(always)]
    fn unpin_assign(&mut self, __param_0: ::ctor::RvalueReference<'_, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN14PrivatePointeraSEOS_(self, __param_0);
        }
    }
}

#[inline(always)]
pub fn DerefPrivatePointer(mut p: crate::PrivatePointer) -> ::core::ffi::c_int {
    unsafe { crate::detail::__rust_thunk___Z19DerefPrivatePointer14PrivatePointer(&mut p) }
}

#[derive(Clone, Copy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=TransitivePublicPointer
pub struct TransitivePublicPointer {
    pub r#pub: crate::PublicPointer,
    pub r#priv: crate::PrivatePointer,
}
impl !Send for TransitivePublicPointer {}
impl !Sync for TransitivePublicPointer {}
unsafe impl ::cxx::ExternType for TransitivePublicPointer {
    type Id = ::cxx::type_id!("TransitivePublicPointer");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("TransitivePublicPointer"),
    crate::TransitivePublicPointer
);

impl Default for TransitivePublicPointer {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN23TransitivePublicPointerC1Ev(
                &raw mut tmp as *mut ::core::ffi::c_void,
            );
            tmp.assume_init()
        }
    }
}

impl From<::ctor::RvalueReference<'_, Self>> for TransitivePublicPointer {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'_, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN23TransitivePublicPointerC1EOS_(
                &raw mut tmp as *mut ::core::ffi::c_void,
                __param_0,
            );
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<::ctor::RvalueReference<'_, Self>> for TransitivePublicPointer {
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'_, Self>) -> Self::CtorType {
        <Self as From<::ctor::RvalueReference<'_, Self>>>::from(args)
    }
}

impl ::ctor::UnpinAssign<&Self> for TransitivePublicPointer {
    #[inline(always)]
    fn unpin_assign(&mut self, __param_0: &Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN23TransitivePublicPointeraSERKS_(self, __param_0);
        }
    }
}

impl ::ctor::UnpinAssign<::ctor::RvalueReference<'_, Self>> for TransitivePublicPointer {
    #[inline(always)]
    fn unpin_assign(&mut self, __param_0: ::ctor::RvalueReference<'_, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN23TransitivePublicPointeraSEOS_(self, __param_0);
        }
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=Union
pub union Union {
    pub i: ::core::ffi::c_int,
    pub f: f32,
}
impl !Send for Union {}
impl !Sync for Union {}
unsafe impl ::cxx::ExternType for Union {
    type Id = ::cxx::type_id!("Union");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(forward_declare::symbol!("Union"), crate::Union);

impl Default for Union {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN5UnionC1Ev(&raw mut tmp as *mut ::core::ffi::c_void);
            tmp.assume_init()
        }
    }
}

impl From<::ctor::RvalueReference<'_, Self>> for Union {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'_, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN5UnionC1EOS_(
                &raw mut tmp as *mut ::core::ffi::c_void,
                __param_0,
            );
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<::ctor::RvalueReference<'_, Self>> for Union {
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'_, Self>) -> Self::CtorType {
        <Self as From<::ctor::RvalueReference<'_, Self>>>::from(args)
    }
}

impl ::ctor::UnpinAssign<&Self> for Union {
    #[inline(always)]
    fn unpin_assign(&mut self, __param_0: &Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN5UnionaSERKS_(self, __param_0);
        }
    }
}

impl ::ctor::UnpinAssign<::ctor::RvalueReference<'_, Self>> for Union {
    #[inline(always)]
    fn unpin_assign(&mut self, __param_0: ::ctor::RvalueReference<'_, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN5UnionaSEOS_(self, __param_0);
        }
    }
}

#[inline(always)]
pub unsafe fn DerefPointer(p: *mut ::core::ffi::c_int) -> ::core::ffi::c_int {
    crate::detail::__rust_thunk___Z12DerefPointerPi(p)
}

#[inline(always)]
pub unsafe fn DerefPublicPointer(mut p: crate::PublicPointer) -> ::core::ffi::c_int {
    crate::detail::__rust_thunk___Z18DerefPublicPointer13PublicPointer(&mut p)
}

#[inline(always)]
pub unsafe fn DerefTransitivePublicPointer(
    mut p: crate::TransitivePublicPointer,
) -> ::core::ffi::c_int {
    crate::detail::__rust_thunk___Z28DerefTransitivePublicPointer23TransitivePublicPointer(&mut p)
}

#[inline(always)]
pub unsafe fn ReadUnion(mut u: crate::Union) -> ::core::ffi::c_int {
    crate::detail::__rust_thunk___Z9ReadUnion5Union(&mut u)
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN13PublicPointerC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN13PublicPointerC1EOS_(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'_, crate::PublicPointer>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN13PublicPointeraSERKS_<'__return_lifetime>(
            __this: &mut crate::PublicPointer,
            __param_0: &crate::PublicPointer,
        ) -> &'__return_lifetime mut crate::PublicPointer;
        pub(crate) unsafe fn __rust_thunk___ZN13PublicPointeraSEOS_<'__return_lifetime>(
            __this: &mut crate::PublicPointer,
            __param_0: ::ctor::RvalueReference<'_, crate::PublicPointer>,
        ) -> &'__return_lifetime mut crate::PublicPointer;
        pub(crate) unsafe fn __rust_thunk___ZN14PrivatePointerC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN14PrivatePointerC1EOS_(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'_, crate::PrivatePointer>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN14PrivatePointeraSERKS_<'__return_lifetime>(
            __this: &mut crate::PrivatePointer,
            __param_0: &crate::PrivatePointer,
        ) -> &'__return_lifetime mut crate::PrivatePointer;
        pub(crate) unsafe fn __rust_thunk___ZN14PrivatePointeraSEOS_<'__return_lifetime>(
            __this: &mut crate::PrivatePointer,
            __param_0: ::ctor::RvalueReference<'_, crate::PrivatePointer>,
        ) -> &'__return_lifetime mut crate::PrivatePointer;
        pub(crate) unsafe fn __rust_thunk___Z19DerefPrivatePointer14PrivatePointer(
            p: &mut crate::PrivatePointer,
        ) -> ::core::ffi::c_int;
        pub(crate) unsafe fn __rust_thunk___ZN23TransitivePublicPointerC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN23TransitivePublicPointerC1EOS_(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'_, crate::TransitivePublicPointer>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN23TransitivePublicPointeraSERKS_<'__return_lifetime>(
            __this: &mut crate::TransitivePublicPointer,
            __param_0: &crate::TransitivePublicPointer,
        ) -> &'__return_lifetime mut crate::TransitivePublicPointer;
        pub(crate) unsafe fn __rust_thunk___ZN23TransitivePublicPointeraSEOS_<'__return_lifetime>(
            __this: &mut crate::TransitivePublicPointer,
            __param_0: ::ctor::RvalueReference<'_, crate::TransitivePublicPointer>,
        ) -> &'__return_lifetime mut crate::TransitivePublicPointer;
        pub(crate) unsafe fn __rust_thunk___ZN5UnionC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN5UnionC1EOS_(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'_, crate::Union>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN5UnionaSERKS_<'__return_lifetime>(
            __this: &mut crate::Union,
            __param_0: &crate::Union,
        ) -> &'__return_lifetime mut crate::Union;
        pub(crate) unsafe fn __rust_thunk___ZN5UnionaSEOS_<'__return_lifetime>(
            __this: &mut crate::Union,
            __param_0: ::ctor::RvalueReference<'_, crate::Union>,
        ) -> &'__return_lifetime mut crate::Union;
        #[link_name = "_Z12DerefPointerPi"]
        pub(crate) unsafe fn __rust_thunk___Z12DerefPointerPi(
            p: *mut ::core::ffi::c_int,
        ) -> ::core::ffi::c_int;
        pub(crate) unsafe fn __rust_thunk___Z18DerefPublicPointer13PublicPointer(
            p: &mut crate::PublicPointer,
        ) -> ::core::ffi::c_int;
        pub(crate) unsafe fn __rust_thunk___Z28DerefTransitivePublicPointer23TransitivePublicPointer(
            p: &mut crate::TransitivePublicPointer,
        ) -> ::core::ffi::c_int;
        pub(crate) unsafe fn __rust_thunk___Z9ReadUnion5Union(
            u: &mut crate::Union,
        ) -> ::core::ffi::c_int;
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::PublicPointer>() == 8);
    assert!(::core::mem::align_of::<crate::PublicPointer>() == 8);
    static_assertions::assert_impl_all!(crate::PublicPointer: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::PublicPointer: Drop);
    assert!(::core::mem::offset_of!(crate::PublicPointer, p) == 0);
    assert!(::core::mem::size_of::<crate::PrivatePointer>() == 8);
    assert!(::core::mem::align_of::<crate::PrivatePointer>() == 8);
    static_assertions::assert_impl_all!(crate::PrivatePointer: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::PrivatePointer: Drop);
    assert!(::core::mem::offset_of!(crate::PrivatePointer, p_) == 0);
    assert!(::core::mem::size_of::<crate::TransitivePublicPointer>() == 16);
    assert!(::core::mem::align_of::<crate::TransitivePublicPointer>() == 8);
    static_assertions::assert_impl_all!(crate::TransitivePublicPointer: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::TransitivePublicPointer: Drop);
    assert!(::core::mem::offset_of!(crate::TransitivePublicPointer, r#pub) == 0);
    assert!(::core::mem::offset_of!(crate::TransitivePublicPointer, r#priv) == 8);
    assert!(::core::mem::size_of::<crate::Union>() == 4);
    assert!(::core::mem::align_of::<crate::Union>() == 4);
    static_assertions::assert_impl_all!(crate::Union: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::Union: Drop);
    assert!(::core::mem::offset_of!(crate::Union, i) == 0);
    assert!(::core::mem::offset_of!(crate::Union, f) == 0);
    static_assertions::assert_impl_all!(::core::ffi::c_int: Copy);
    static_assertions::assert_impl_all!(f32: Copy);
};
