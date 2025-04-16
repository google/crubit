// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:uses_not_crubit_exposed_cc

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![no_std]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code)]
#![deny(warnings)]

// Error while generating bindings for item 'UseNotCrubitExposed':
// Failed to format type of parameter 0: Can't generate bindings for NotCrubitExposed, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:not_crubit_exposed needs [//features:supported] for NotCrubitExposed

#[derive(Clone, Copy)]
#[repr(C, align(4))]
///CRUBIT_ANNOTATE: cpp_type=CannotUpcastInCrubit
pub struct CannotUpcastInCrubit {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 4],
}
impl !Send for CannotUpcastInCrubit {}
impl !Sync for CannotUpcastInCrubit {}
forward_declare::unsafe_define!(
    forward_declare::symbol!("CannotUpcastInCrubit"),
    crate::CannotUpcastInCrubit
);

// Error while generating bindings for item 'CannotUpcastInCrubit::CannotUpcastInCrubit':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347.
// Expected first constructor parameter to be a mutable reference, got: *mut crate::CannotUpcastInCrubit
// Missing lifetime for `__this` parameter type: *mut crate::CannotUpcastInCrubit

// Error while generating bindings for item 'CannotUpcastInCrubit::CannotUpcastInCrubit':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347.
// Expected first constructor parameter to be a mutable reference, got: *mut crate::CannotUpcastInCrubit
// Missing lifetime for `__this` parameter type: *mut crate::CannotUpcastInCrubit

// Error while generating bindings for item 'CannotUpcastInCrubit::CannotUpcastInCrubit':
// Parameter #0 is not supported: Unsupported type 'CannotUpcastInCrubit &&': Unsupported type: && without lifetime

// Error while generating bindings for item 'CannotUpcastInCrubit::operator=':
// `self` has no lifetime. Use lifetime annotations or `#pragma clang lifetime_elision` to create bindings for this function.

// Error while generating bindings for item 'CannotUpcastInCrubit::operator=':
// Parameter #0 is not supported: Unsupported type 'CannotUpcastInCrubit &&': Unsupported type: && without lifetime

// 'CannotUpcastInCrubit' cannot be upcasted to 'NotCrubitExposed' because the base type doesn't have Crubit bindings.

const _: () = {
    assert!(::core::mem::size_of::<crate::CannotUpcastInCrubit>() == 4);
    assert!(::core::mem::align_of::<crate::CannotUpcastInCrubit>() == 4);
    static_assertions::assert_impl_all!(crate::CannotUpcastInCrubit: Clone);
    static_assertions::assert_impl_all!(crate::CannotUpcastInCrubit: Copy);
    static_assertions::assert_not_impl_any!(crate::CannotUpcastInCrubit: Drop);
};
