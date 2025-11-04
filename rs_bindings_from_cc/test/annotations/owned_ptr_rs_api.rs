// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/annotations:owned_ptr
// Features: non_unpin_ctor, std_unique_ptr, std_vector, supported

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
pub struct Thing {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    pub value: i32,
}
impl !Send for Thing {}
impl !Sync for Thing {}
unsafe impl ::cxx::ExternType for Thing {
    type Id = ::cxx::type_id!("Thing");
    type Kind = ::cxx::kind::Trivial;
}

// Generated from: rs_bindings_from_cc/test/annotations/owned_ptr.h;l=17
// Error while generating bindings for constructor 'Thing::Thing':
// Move and copy constructors do yet receive bindings. See b/452726517.
// Expected first constructor parameter to be a mutable reference, got: *mut crate::Thing
// Expected first reference parameter `__this` to have a lifetime, found *mut crate::Thing

// Generated from: rs_bindings_from_cc/test/annotations/owned_ptr.h;l=17
// Error while generating bindings for constructor 'Thing::Thing':
// Move and copy constructors do yet receive bindings. See b/452726517.
// Expected first constructor parameter to be a mutable reference, got: *mut crate::Thing
// Expected first reference parameter `__this` to have a lifetime, found *mut crate::Thing

// Generated from: rs_bindings_from_cc/test/annotations/owned_ptr.h;l=17
// Error while generating bindings for function 'Thing::operator=':
// `self` has no lifetime. Use lifetime annotations or `#pragma clang lifetime_elision` to create bindings for this function.

// Generated from: rs_bindings_from_cc/test/annotations/owned_ptr.h;l=17
// Error while generating bindings for function 'Thing::operator=':
// `self` has no lifetime. Use lifetime annotations or `#pragma clang lifetime_elision` to create bindings for this function.

// Generated from: rs_bindings_from_cc/test/annotations/owned_ptr.h;l=18
// Error while generating bindings for constructor 'Thing::Thing':
// Expected first constructor parameter to be a mutable reference, got: *mut crate::Thing
// Expected first reference parameter `__this` to have a lifetime, found *mut crate::Thing

impl Thing {
    /// Generated from: rs_bindings_from_cc/test/annotations/owned_ptr.h;l=21
    #[inline(always)]
    pub unsafe fn Close(__this: *mut Self) {
        crate::detail::__rust_thunk___ZN5Thing5CloseEv(__this)
    }
}

// Generated from: nowhere/llvm/src/libcxx/include/__type_traits/integral_constant.h;l=21
// Error while generating bindings for struct 'std::integral_constant<bool, false>':
// Can't generate bindings for std::integral_constant<bool, false>, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/annotations:owned_ptr needs [//features:wrapper] for std::integral_constant<bool, false> (crate::__CcTemplateInstNSt3__u17integral_constantIbLb0EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__type_traits/integral_constant.h;l=21
// Error while generating bindings for struct 'std::integral_constant<bool, true>':
// Can't generate bindings for std::integral_constant<bool, true>, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/annotations:owned_ptr needs [//features:wrapper] for std::integral_constant<bool, true> (crate::__CcTemplateInstNSt3__u17integral_constantIbLb1EEE is a template instantiation)

#[path = "rs_bindings_from_cc/test/annotations/owned_ptr_rust_thing.rs"]
mod __crubit_mod_0;
#[allow(unused_imports)]
pub use __crubit_mod_0::*;

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN5Thing5CloseEv(__this: *mut crate::Thing);
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::Thing>() == 4);
    assert!(::core::mem::align_of::<crate::Thing>() == 4);
    static_assertions::assert_impl_all!(crate::Thing: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::Thing: Drop);
    assert!(::core::mem::offset_of!(crate::Thing, value) == 0);
};
