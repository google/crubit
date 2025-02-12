// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:nodiscard_cc

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes, negative_impls, register_tool)]
#![allow(stable_features)]
#![no_std]
#![register_tool(__crubit)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code)]
#![deny(warnings)]

#[derive(Clone, Copy)]
#[must_use]
#[repr(C)]
#[__crubit::annotate(cpp_type = "NoDiscard")]
pub struct NoDiscard {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for NoDiscard {}
impl !Sync for NoDiscard {}
forward_declare::unsafe_define!(forward_declare::symbol!("NoDiscard"), crate::NoDiscard);

// Error while generating bindings for item 'NoDiscard::NoDiscard':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347.
// Expected first constructor parameter to be a mutable reference, got: *mut crate::NoDiscard
// Missing lifetime for `__this` parameter type: *mut crate::NoDiscard

// Error while generating bindings for item 'NoDiscard::NoDiscard':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347.
// Expected first constructor parameter to be a mutable reference, got: *mut crate::NoDiscard
// Missing lifetime for `__this` parameter type: *mut crate::NoDiscard

// Error while generating bindings for item 'NoDiscard::NoDiscard':
// Parameter #0 is not supported: Unsupported type 'NoDiscard &&': Unsupported type: && without lifetime

// Error while generating bindings for item 'NoDiscard::operator=':
// `self` has no lifetime. Use lifetime annotations or `#pragma clang lifetime_elision` to create bindings for this function.

// Error while generating bindings for item 'NoDiscard::operator=':
// Parameter #0 is not supported: Unsupported type 'NoDiscard &&': Unsupported type: && without lifetime

#[derive(Clone, Copy)]
#[must_use = "You really should use this"]
#[repr(C)]
#[__crubit::annotate(cpp_type = "NoDiscardWithMessage")]
pub struct NoDiscardWithMessage {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for NoDiscardWithMessage {}
impl !Sync for NoDiscardWithMessage {}
forward_declare::unsafe_define!(
    forward_declare::symbol!("NoDiscardWithMessage"),
    crate::NoDiscardWithMessage
);

// Error while generating bindings for item 'NoDiscardWithMessage::NoDiscardWithMessage':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347.
// Expected first constructor parameter to be a mutable reference, got: *mut crate::NoDiscardWithMessage
// Missing lifetime for `__this` parameter type: *mut crate::NoDiscardWithMessage

// Error while generating bindings for item 'NoDiscardWithMessage::NoDiscardWithMessage':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347.
// Expected first constructor parameter to be a mutable reference, got: *mut crate::NoDiscardWithMessage
// Missing lifetime for `__this` parameter type: *mut crate::NoDiscardWithMessage

// Error while generating bindings for item 'NoDiscardWithMessage::NoDiscardWithMessage':
// Parameter #0 is not supported: Unsupported type 'NoDiscardWithMessage &&': Unsupported type: && without lifetime

// Error while generating bindings for item 'NoDiscardWithMessage::operator=':
// `self` has no lifetime. Use lifetime annotations or `#pragma clang lifetime_elision` to create bindings for this function.

// Error while generating bindings for item 'NoDiscardWithMessage::operator=':
// Parameter #0 is not supported: Unsupported type 'NoDiscardWithMessage &&': Unsupported type: && without lifetime

const _: () = {
    assert!(::core::mem::size_of::<crate::NoDiscard>() == 1);
    assert!(::core::mem::align_of::<crate::NoDiscard>() == 1);
    static_assertions::assert_impl_all!(crate::NoDiscard: Clone);
    static_assertions::assert_impl_all!(crate::NoDiscard: Copy);
    static_assertions::assert_not_impl_any!(crate::NoDiscard: Drop);

    assert!(::core::mem::size_of::<crate::NoDiscardWithMessage>() == 1);
    assert!(::core::mem::align_of::<crate::NoDiscardWithMessage>() == 1);
    static_assertions::assert_impl_all!(crate::NoDiscardWithMessage: Clone);
    static_assertions::assert_impl_all!(crate::NoDiscardWithMessage: Copy);
    static_assertions::assert_not_impl_any!(crate::NoDiscardWithMessage: Drop);
};
