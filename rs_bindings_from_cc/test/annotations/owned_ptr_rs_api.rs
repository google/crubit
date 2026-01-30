// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/annotations:owned_ptr
// Features: custom_ffi_types, non_unpin_ctor, std_unique_ptr, std_vector, supported

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![no_std]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

/// An example of a C++ struct that supports ownership via the raw pointer.
///
/// The CRUBIT_OWNED_PTR_TYPE annotation is used to specify the Rust type that
/// will be used to represent the C++ struct when it is used in a position
/// annotated with CRUBIT_OWNED_PTR.
///
/// Generated from: rs_bindings_from_cc/test/annotations/owned_ptr.h;l=17
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=Thing
pub struct RawThing {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    pub value: i32,
}
impl !Send for RawThing {}
impl !Sync for RawThing {}
unsafe impl ::cxx::ExternType for RawThing {
    type Id = ::cxx::type_id!("Thing");
    type Kind = ::cxx::kind::Trivial;
}
impl RawThing {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    ///
    /// Generated from: rs_bindings_from_cc/test/annotations/owned_ptr.h;l=21
    #[inline(always)]
    pub unsafe fn Close(__this: *mut Self) {
        crate::detail::__rust_thunk___ZN5Thing5CloseEv(__this)
    }
}

// Generated due to CRUBIT_OWNED_POINTEE annotation.
#[repr(transparent)]
pub struct OwnedThing(::core::ptr::NonNull<RawThing>);

/// Generated from: rs_bindings_from_cc/test/annotations/owned_ptr.h;l=18
impl From<i32> for RawThing {
    #[inline(always)]
    fn from(value: i32) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN5ThingC1Ei(&raw mut tmp as *mut _, value);
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<i32> for RawThing {
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: i32) -> Self::CtorType {
        <Self as From<i32>>::from(args)
    }
}

// Generated from: nowhere/llvm/src/libcxx/include/__type_traits/integral_constant.h;l=21
// Error while generating bindings for struct 'std::integral_constant<bool, false>':
// Can't generate bindings for std::integral_constant<bool, false>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/annotations:owned_ptr needs [//features:wrapper] for std::integral_constant<bool, false> (crate::__CcTemplateInstNSt3__u17integral_constantIbLb0EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__type_traits/integral_constant.h;l=21
// Error while generating bindings for struct 'std::integral_constant<bool, true>':
// Can't generate bindings for std::integral_constant<bool, true>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/annotations:owned_ptr needs [//features:wrapper] for std::integral_constant<bool, true> (crate::__CcTemplateInstNSt3__u17integral_constantIbLb1EEE is a template instantiation)

#[path = "rs_bindings_from_cc/test/annotations/owned_ptr_rust_thing.rs"]
mod __crubit_mod_0;
#[allow(unused_imports)]
pub use __crubit_mod_0::*;

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN5ThingC1Ei(
            __this: *mut ::core::ffi::c_void,
            value: i32,
        );
        pub(crate) unsafe fn __rust_thunk___ZN5Thing5CloseEv(__this: *mut crate::RawThing);
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::RawThing>() == 4);
    assert!(::core::mem::align_of::<crate::RawThing>() == 4);
    static_assertions::assert_impl_all!(crate::RawThing: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::RawThing: Drop);
    assert!(::core::mem::offset_of!(crate::RawThing, value) == 0);
};
