// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:private_method_cc
// Features: experimental, supported

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes, negative_impls, register_tool)]
#![allow(stable_features)]
#![no_std]
#![register_tool(__crubit)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code)]
#![deny(warnings)]

// Error while generating bindings for item 'Ptr':
// Class templates are not supported yet

#[derive(Clone, Copy)]
#[repr(C)]
#[__crubit::annotate(cpp_type = "Outer")]
pub struct Outer {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for Outer {}
impl !Sync for Outer {}
forward_declare::unsafe_define!(forward_declare::symbol!("Outer"), crate::Outer);

// Error while generating bindings for item 'Outer::Outer':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// Error while generating bindings for item 'Outer::Outer':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// Error while generating bindings for item 'Outer::Outer':
// Parameter #0 is not supported: Unsupported type 'Outer &&': Unsupported type: && without lifetime

// Error while generating bindings for item 'Outer::operator=':
// `self` has no lifetime. Use lifetime annotations or `#pragma clang lifetime_elision` to create bindings for this function.

// Error while generating bindings for item 'Outer::operator=':
// Parameter #0 is not supported: Unsupported type 'Outer &&': Unsupported type: && without lifetime

const _: () = {
    assert!(::core::mem::size_of::<crate::Outer>() == 1);
    assert!(::core::mem::align_of::<crate::Outer>() == 1);
    static_assertions::assert_impl_all!(crate::Outer: Clone);
    static_assertions::assert_impl_all!(crate::Outer: Copy);
    static_assertions::assert_not_impl_any!(crate::Outer: Drop);
};
