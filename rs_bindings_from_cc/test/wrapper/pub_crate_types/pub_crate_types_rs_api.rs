// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/wrapper/pub_crate_types:pub_crate_types
// Features: supported, wrapper

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![no_std]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code)]
#![deny(warnings)]

forward_declare::forward_declare!(pub(crate)ForwardDeclared = forward_declare::symbol!("ForwardDeclared"));

/// Generated from: rs_bindings_from_cc/test/wrapper/pub_crate_types/pub_crate_types.h;l=12
pub(crate) type ForwardDeclaredAlias = crate::ForwardDeclared;

/// Generated from: rs_bindings_from_cc/test/wrapper/pub_crate_types/pub_crate_types.h;l=14
#[derive(Clone, Copy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=CompoundDataType
pub struct CompoundDataType {
    pub(crate) forward_declared: *mut crate::ForwardDeclared,
}
impl !Send for CompoundDataType {}
impl !Sync for CompoundDataType {}

// Generated from: rs_bindings_from_cc/test/wrapper/pub_crate_types/pub_crate_types.h;l=14
// Error while generating bindings for item 'CompoundDataType::CompoundDataType':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347.
// Expected first constructor parameter to be a mutable reference, got: *mut crate::CompoundDataType
// Missing lifetime for `__this` parameter type: *mut crate::CompoundDataType

// Generated from: rs_bindings_from_cc/test/wrapper/pub_crate_types/pub_crate_types.h;l=14
// Error while generating bindings for item 'CompoundDataType::CompoundDataType':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347.
// Expected first constructor parameter to be a mutable reference, got: *mut crate::CompoundDataType
// Missing lifetime for `__this` parameter type: *mut crate::CompoundDataType

// Generated from: rs_bindings_from_cc/test/wrapper/pub_crate_types/pub_crate_types.h;l=14
// Error while generating bindings for item 'CompoundDataType::CompoundDataType':
// Parameter #0 is not supported: Unsupported type 'CompoundDataType &&': Unsupported type: && without lifetime

// Generated from: rs_bindings_from_cc/test/wrapper/pub_crate_types/pub_crate_types.h;l=14
// Error while generating bindings for item 'CompoundDataType::operator=':
// `self` has no lifetime. Use lifetime annotations or `#pragma clang lifetime_elision` to create bindings for this function.

// Generated from: rs_bindings_from_cc/test/wrapper/pub_crate_types/pub_crate_types.h;l=14
// Error while generating bindings for item 'CompoundDataType::operator=':
// Parameter #0 is not supported: Unsupported type 'CompoundDataType &&': Unsupported type: && without lifetime

/// Generated from: rs_bindings_from_cc/test/wrapper/pub_crate_types/pub_crate_types.h;l=18
#[inline(always)]
pub(crate) fn CreateForwardDeclared() -> *mut crate::ForwardDeclared {
    unsafe { crate::detail::__rust_thunk___Z21CreateForwardDeclaredv() }
}

extern "C" {
    pub(crate) static mut ForwardDeclaredConstant: *mut crate::ForwardDeclared;
}

/// Generated from: rs_bindings_from_cc/test/wrapper/pub_crate_types/pub_crate_types.h;l=22
#[inline(always)]
pub fn ConsumeCompoundDataType(mut container: crate::CompoundDataType) -> ::core::ffi::c_int {
    unsafe {
        crate::detail::__rust_thunk___Z23ConsumeCompoundDataType16CompoundDataType(&mut container)
    }
}

// Generated from: rs_bindings_from_cc/test/wrapper/pub_crate_types/pub_crate_types.h;l=25
// Error while generating bindings for item 'OtherPubCrateTypes':
// Can't generate bindings for OtherPubCrateTypes, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/wrapper/pub_crate_types:pub_crate_types needs [//features:experimental] for *mut other_pub_crate_types::ForwardDeclared2 (*mut other_pub_crate_types::ForwardDeclared2 is `pub(crate)` in //rs_bindings_from_cc/test/wrapper/pub_crate_types:other_pub_crate_types)

// Don't uncomment this: a `pair` include starts polluting the golden test with
// a lot of implementation details.
// But this function would produce a different error from the first,
// because it sees the types earlier.
// inline void MixedPubCrateTypes(std::pair<ForwardDeclared*,
// ForwardDeclared2*>) {}

#[path = "rs_bindings_from_cc/test/wrapper/pub_crate_types/pub_crate_types_extra.rs"]
mod __crubit_mod_0;
#[allow(unused_imports)]
pub use __crubit_mod_0::*;

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        #[link_name = "_Z21CreateForwardDeclaredv"]
        pub(crate) unsafe fn __rust_thunk___Z21CreateForwardDeclaredv(
        ) -> *mut crate::ForwardDeclared;
        pub(crate) unsafe fn __rust_thunk___Z23ConsumeCompoundDataType16CompoundDataType(
            container: &mut crate::CompoundDataType,
        ) -> ::core::ffi::c_int;
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::CompoundDataType>() == 8);
    assert!(::core::mem::align_of::<crate::CompoundDataType>() == 8);
    static_assertions::assert_impl_all!(crate::CompoundDataType: Clone);
    static_assertions::assert_impl_all!(crate::CompoundDataType: Copy);
    static_assertions::assert_not_impl_any!(crate::CompoundDataType: Drop);
    assert!(::core::mem::offset_of!(crate::CompoundDataType, forward_declared) == 0);
};
