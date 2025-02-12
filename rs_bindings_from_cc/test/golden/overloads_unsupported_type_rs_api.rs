// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:overloads_unsupported_type_cc

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes, negative_impls, register_tool)]
#![allow(stable_features)]
#![no_std]
#![register_tool(__crubit)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code)]
#![deny(warnings)]

/// Tests that no bindings are generated when an overload set includes
/// any unsupported items.
///
/// See http://b/251045039
#[derive(Clone, Copy)]
#[repr(C)]
#[__crubit::annotate(cpp_type = "SomeClass")]
pub struct SomeClass {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for SomeClass {}
impl !Sync for SomeClass {}
forward_declare::unsafe_define!(forward_declare::symbol!("SomeClass"), crate::SomeClass);

// Error while generating bindings for item 'SomeClass::SomeClass':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347.
// Expected first constructor parameter to be a mutable reference, got: *mut crate::SomeClass
// Missing lifetime for `__this` parameter type: *mut crate::SomeClass

// Error while generating bindings for item 'SomeClass::SomeClass':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347.
// Expected first constructor parameter to be a mutable reference, got: *mut crate::SomeClass
// Missing lifetime for `__this` parameter type: *mut crate::SomeClass

// Error while generating bindings for item 'SomeClass::SomeClass':
// Parameter #0 is not supported: Unsupported type 'SomeClass &&': Unsupported type: && without lifetime

// Error while generating bindings for item 'SomeClass::operator=':
// `self` has no lifetime. Use lifetime annotations or `#pragma clang lifetime_elision` to create bindings for this function.

// Error while generating bindings for item 'SomeClass::operator=':
// Parameter #0 is not supported: Unsupported type 'SomeClass &&': Unsupported type: && without lifetime

#[inline(always)]
pub fn Overload() {
    unsafe { crate::detail::__rust_thunk___Z8Overloadv() }
}

// Error while generating bindings for item 'Overload':
// Parameter #0 is not supported: Unsupported type 'SomeClass &&': Unsupported type: && without lifetime

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___Z8Overloadv();
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::SomeClass>() == 1);
    assert!(::core::mem::align_of::<crate::SomeClass>() == 1);
    static_assertions::assert_impl_all!(crate::SomeClass: Clone);
    static_assertions::assert_impl_all!(crate::SomeClass: Copy);
    static_assertions::assert_not_impl_any!(crate::SomeClass: Drop);
};
