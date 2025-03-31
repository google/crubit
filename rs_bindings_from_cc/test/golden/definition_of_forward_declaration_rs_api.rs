// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:definition_of_forward_declaration_cc

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![no_std]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code)]
#![deny(warnings)]

#[derive(Clone, Copy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=ForwardDeclaredStruct
pub struct ForwardDeclaredStruct {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for ForwardDeclaredStruct {}
impl !Sync for ForwardDeclaredStruct {}
forward_declare::unsafe_define!(
    forward_declare::symbol!("ForwardDeclaredStruct"),
    crate::ForwardDeclaredStruct
);

// Error while generating bindings for item 'ForwardDeclaredStruct::ForwardDeclaredStruct':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347.
// Expected first constructor parameter to be a mutable reference, got: *mut crate::ForwardDeclaredStruct
// Missing lifetime for `__this` parameter type: *mut crate::ForwardDeclaredStruct

// Error while generating bindings for item 'ForwardDeclaredStruct::ForwardDeclaredStruct':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347.
// Expected first constructor parameter to be a mutable reference, got: *mut crate::ForwardDeclaredStruct
// Missing lifetime for `__this` parameter type: *mut crate::ForwardDeclaredStruct

// Error while generating bindings for item 'ForwardDeclaredStruct::ForwardDeclaredStruct':
// Parameter #0 is not supported: Unsupported type 'ForwardDeclaredStruct &&': Unsupported type: && without lifetime

// Error while generating bindings for item 'ForwardDeclaredStruct::operator=':
// `self` has no lifetime. Use lifetime annotations or `#pragma clang lifetime_elision` to create bindings for this function.

// Error while generating bindings for item 'ForwardDeclaredStruct::operator=':
// Parameter #0 is not supported: Unsupported type 'ForwardDeclaredStruct &&': Unsupported type: && without lifetime

const _: () = {
    assert!(::core::mem::size_of::<crate::ForwardDeclaredStruct>() == 1);
    assert!(::core::mem::align_of::<crate::ForwardDeclaredStruct>() == 1);
    static_assertions::assert_impl_all!(crate::ForwardDeclaredStruct: Clone);
    static_assertions::assert_impl_all!(crate::ForwardDeclaredStruct: Copy);
    static_assertions::assert_not_impl_any!(crate::ForwardDeclaredStruct: Drop);
};
