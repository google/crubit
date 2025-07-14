// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //examples/cpp/method:example_lib
// Features: supported, unsafe_types

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![no_std]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

pub mod foo {
    /// Generated from: examples/cpp/method/example.h;l=12
    #[derive(Clone, Copy)]
    #[repr(C)]
    ///CRUBIT_ANNOTATE: cpp_type=foo :: Bar
    pub struct Bar {
        pub x: ::core::ffi::c_int,
    }
    impl !Send for Bar {}
    impl !Sync for Bar {}
    unsafe impl ::cxx::ExternType for Bar {
        type Id = ::cxx::type_id!("foo :: Bar");
        type Kind = ::cxx::kind::Trivial;
    }

    // Generated from: examples/cpp/method/example.h;l=12
    // Error while generating bindings for function 'Bar::Bar':
    // Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347.
    // Expected first constructor parameter to be a mutable reference, got: *mut crate::foo::Bar
    // Expected first parameter to be a `__this` reference, found *mut crate::foo::Bar

    // Generated from: examples/cpp/method/example.h;l=12
    // Error while generating bindings for function 'Bar::Bar':
    // Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347.
    // Expected first constructor parameter to be a mutable reference, got: *mut crate::foo::Bar
    // Expected first parameter to be a `__this` reference, found *mut crate::foo::Bar

    // Generated from: examples/cpp/method/example.h;l=12
    // Error while generating bindings for function 'Bar::Bar':
    // Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347.
    // Expected first constructor parameter to be a mutable reference, got: *mut crate::foo::Bar
    // Expected first parameter to be a `__this` reference, found *mut crate::foo::Bar

    // Generated from: examples/cpp/method/example.h;l=12
    // Error while generating bindings for function 'Bar::operator=':
    // `self` has no lifetime. Use lifetime annotations or `#pragma clang lifetime_elision` to create bindings for this function.

    // Generated from: examples/cpp/method/example.h;l=12
    // Error while generating bindings for function 'Bar::operator=':
    // `self` has no lifetime. Use lifetime annotations or `#pragma clang lifetime_elision` to create bindings for this function.

    impl Bar {
        /// Generated from: examples/cpp/method/example.h;l=14
        #[inline(always)]
        pub unsafe fn MyMethod(__this: *mut Self) {
            crate::detail::__rust_thunk___ZN3foo3Bar8MyMethodEv(__this)
        }
    }
}

// namespace foo

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN3foo3Bar8MyMethodEv(__this: *mut crate::foo::Bar);
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::foo::Bar>() == 4);
    assert!(::core::mem::align_of::<crate::foo::Bar>() == 4);
    static_assertions::assert_impl_all!(crate::foo::Bar: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::foo::Bar: Drop);
    assert!(::core::mem::offset_of!(crate::foo::Bar, x) == 0);
};
