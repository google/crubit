// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:polymorphic_cc

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

#[::ctor::recursively_pinned(PinnedDrop)]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=PolymorphicBase
pub struct PolymorphicBase {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for PolymorphicBase {}
impl !Sync for PolymorphicBase {}
unsafe impl ::cxx::ExternType for PolymorphicBase {
    type Id = ::cxx::type_id!("PolymorphicBase");
    type Kind = ::cxx::kind::Opaque;
}

impl ::ctor::CtorNew<()> for PolymorphicBase {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(move |dest: *mut Self| {
                crate::detail::__rust_thunk___ZN15PolymorphicBaseC1Ev(
                    dest as *mut ::core::ffi::c_void,
                );
            })
        }
    }
}

// Error while generating bindings for constructor 'PolymorphicBase::PolymorphicBase':
// Can't generate bindings for PolymorphicBase::PolymorphicBase, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:polymorphic_cc needs [//features:experimental] for PolymorphicBase::PolymorphicBase (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'PolymorphicBase::operator=':
// Can't generate bindings for PolymorphicBase::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:polymorphic_cc needs [//features:experimental] for PolymorphicBase::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:polymorphic_cc needs [//features:experimental] for PolymorphicBase::operator= (the type of __param_0 (parameter #1): references are not supported)

impl ::ctor::PinnedDrop for PolymorphicBase {
    #[inline(always)]
    unsafe fn pinned_drop<'a>(self: ::core::pin::Pin<&'a mut Self>) {
        crate::detail::__rust_thunk___ZN15PolymorphicBaseD1Ev(self)
    }
}

#[::ctor::recursively_pinned(PinnedDrop)]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=PolymorphicBase2
pub struct PolymorphicBase2 {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for PolymorphicBase2 {}
impl !Sync for PolymorphicBase2 {}
unsafe impl ::cxx::ExternType for PolymorphicBase2 {
    type Id = ::cxx::type_id!("PolymorphicBase2");
    type Kind = ::cxx::kind::Opaque;
}
impl PolymorphicBase2 {
    #[inline(always)]
    pub fn Foo<'a>(self: ::core::pin::Pin<&'a mut Self>) {
        unsafe { crate::detail::__rust_thunk___ZN16PolymorphicBase23FooEv(self) }
    }
}

impl ::ctor::CtorNew<()> for PolymorphicBase2 {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(move |dest: *mut Self| {
                crate::detail::__rust_thunk___ZN16PolymorphicBase2C1Ev(
                    dest as *mut ::core::ffi::c_void,
                );
            })
        }
    }
}

// Error while generating bindings for constructor 'PolymorphicBase2::PolymorphicBase2':
// Can't generate bindings for PolymorphicBase2::PolymorphicBase2, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:polymorphic_cc needs [//features:experimental] for PolymorphicBase2::PolymorphicBase2 (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'PolymorphicBase2::operator=':
// Can't generate bindings for PolymorphicBase2::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:polymorphic_cc needs [//features:experimental] for PolymorphicBase2::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:polymorphic_cc needs [//features:experimental] for PolymorphicBase2::operator= (the type of __param_0 (parameter #1): references are not supported)

impl ::ctor::PinnedDrop for PolymorphicBase2 {
    #[inline(always)]
    unsafe fn pinned_drop<'a>(self: ::core::pin::Pin<&'a mut Self>) {
        crate::detail::__rust_thunk___ZN16PolymorphicBase2D1Ev(self)
    }
}

#[::ctor::recursively_pinned(PinnedDrop)]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=PolymorphicDerived
pub struct PolymorphicDerived {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 16],
}
impl !Send for PolymorphicDerived {}
impl !Sync for PolymorphicDerived {}
unsafe impl ::cxx::ExternType for PolymorphicDerived {
    type Id = ::cxx::type_id!("PolymorphicDerived");
    type Kind = ::cxx::kind::Opaque;
}

impl ::ctor::CtorNew<()> for PolymorphicDerived {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(move |dest: *mut Self| {
                crate::detail::__rust_thunk___ZN18PolymorphicDerivedC1Ev(
                    dest as *mut ::core::ffi::c_void,
                );
            })
        }
    }
}

// Error while generating bindings for constructor 'PolymorphicDerived::PolymorphicDerived':
// Can't generate bindings for PolymorphicDerived::PolymorphicDerived, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:polymorphic_cc needs [//features:experimental] for PolymorphicDerived::PolymorphicDerived (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for constructor 'PolymorphicDerived::PolymorphicDerived':
// Can't generate bindings for PolymorphicDerived::PolymorphicDerived, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:polymorphic_cc needs [//features:experimental] for PolymorphicDerived::PolymorphicDerived (the type of __param_0 (parameter #1): references are not supported)

impl ::ctor::PinnedDrop for PolymorphicDerived {
    #[inline(always)]
    unsafe fn pinned_drop<'a>(self: ::core::pin::Pin<&'a mut Self>) {
        crate::detail::__rust_thunk___ZN18PolymorphicDerivedD1Ev(self)
    }
}

// Error while generating bindings for function 'PolymorphicDerived::operator=':
// Can't generate bindings for PolymorphicDerived::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:polymorphic_cc needs [//features:experimental] for PolymorphicDerived::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:polymorphic_cc needs [//features:experimental] for PolymorphicDerived::operator= (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'PolymorphicDerived::operator=':
// Can't generate bindings for PolymorphicDerived::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:polymorphic_cc needs [//features:experimental] for PolymorphicDerived::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:polymorphic_cc needs [//features:experimental] for PolymorphicDerived::operator= (the type of __param_0 (parameter #1): references are not supported)

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN15PolymorphicBaseC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN15PolymorphicBaseD1Ev<'a>(
            __this: ::core::pin::Pin<&'a mut crate::PolymorphicBase>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN16PolymorphicBase2C1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN16PolymorphicBase23FooEv<'a>(
            __this: ::core::pin::Pin<&'a mut crate::PolymorphicBase2>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN16PolymorphicBase2D1Ev<'a>(
            __this: ::core::pin::Pin<&'a mut crate::PolymorphicBase2>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN18PolymorphicDerivedC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN18PolymorphicDerivedD1Ev<'a>(
            __this: ::core::pin::Pin<&'a mut crate::PolymorphicDerived>,
        );
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::PolymorphicBase>() == 8);
    assert!(::core::mem::align_of::<crate::PolymorphicBase>() == 8);
    static_assertions::assert_impl_all!(crate::PolymorphicBase: Drop);
    static_assertions::assert_not_impl_any!(crate::PolymorphicBase: Copy);

    assert!(::core::mem::size_of::<crate::PolymorphicBase2>() == 8);
    assert!(::core::mem::align_of::<crate::PolymorphicBase2>() == 8);
    static_assertions::assert_impl_all!(crate::PolymorphicBase2: Drop);
    static_assertions::assert_not_impl_any!(crate::PolymorphicBase2: Copy);

    assert!(::core::mem::size_of::<crate::PolymorphicDerived>() == 16);
    assert!(::core::mem::align_of::<crate::PolymorphicDerived>() == 8);
    static_assertions::assert_impl_all!(crate::PolymorphicDerived: Drop);
    static_assertions::assert_not_impl_any!(crate::PolymorphicDerived: Copy);
};
