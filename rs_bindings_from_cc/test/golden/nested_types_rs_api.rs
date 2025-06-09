// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:nested_types_cc

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
///CRUBIT_ANNOTATE: cpp_type=Foo
pub struct Foo {
    pub foo: ::core::ffi::c_int,
}
impl !Send for Foo {}
impl !Sync for Foo {}
forward_declare::unsafe_define!(forward_declare::symbol!("Foo"), crate::Foo);

// Error while generating bindings for item 'Foo::Foo':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347.
// Expected first constructor parameter to be a mutable reference, got: *mut crate::Foo
// Missing lifetime for `__this` parameter type: *mut crate::Foo

// Error while generating bindings for item 'Foo::Foo':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347.
// Expected first constructor parameter to be a mutable reference, got: *mut crate::Foo
// Missing lifetime for `__this` parameter type: *mut crate::Foo

// Error while generating bindings for item 'Foo::Foo':
// Parameter #0 is not supported: Unsupported type 'Foo &&': Unsupported type: && without lifetime

// Error while generating bindings for item 'Foo::operator=':
// `self` has no lifetime. Use lifetime annotations or `#pragma clang lifetime_elision` to create bindings for this function.

// Error while generating bindings for item 'Foo::operator=':
// Parameter #0 is not supported: Unsupported type 'Foo &&': Unsupported type: && without lifetime

// Error while generating bindings for item 'Bar':
// Can't generate bindings for Bar, because it is unsupported: b/200067824: type definitions nested inside records are not yet supported

const _: () = {
    assert!(::core::mem::size_of::<crate::Foo>() == 4);
    assert!(::core::mem::align_of::<crate::Foo>() == 4);
    static_assertions::assert_impl_all!(crate::Foo: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::Foo: Drop);
    assert!(::core::mem::offset_of!(crate::Foo, foo) == 0);
};
