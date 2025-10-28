// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/namespace/inline:inline
// Features: non_unpin_ctor, std_unique_ptr, std_vector, supported

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![no_std]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

pub mod foo {
    // namespace inline1

    // Test coverage for the case where additional declarations appear in `inline1`,
    // but without `inline namespace /*...*/`, just with `namespace inline1`.

    pub mod inline1 {
        /// Generated from: rs_bindings_from_cc/test/namespace/inline/inline.h;l=11
        #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
        #[repr(C)]
        ///CRUBIT_ANNOTATE: cpp_type=foo :: inline1 :: MyStruct
        pub struct MyStruct {
            pub value: ::core::ffi::c_int,
        }
        impl !Send for MyStruct {}
        impl !Sync for MyStruct {}
        unsafe impl ::cxx::ExternType for MyStruct {
            type Id = ::cxx::type_id!("foo :: inline1 :: MyStruct");
            type Kind = ::cxx::kind::Trivial;
        }

        // Generated from: rs_bindings_from_cc/test/namespace/inline/inline.h;l=11
        // Error while generating bindings for constructor 'MyStruct::MyStruct':
        // Default constructors do yet receive bindings. See b/452726517.
        // Expected first constructor parameter to be a mutable reference, got: *mut crate::foo::inline1::MyStruct
        // Expected first reference parameter `__this` to have a lifetime, found *mut crate::foo::inline1::MyStruct

        // Generated from: rs_bindings_from_cc/test/namespace/inline/inline.h;l=11
        // Error while generating bindings for constructor 'MyStruct::MyStruct':
        // Move and copy constructors do yet receive bindings. See b/452726517.
        // Expected first constructor parameter to be a mutable reference, got: *mut crate::foo::inline1::MyStruct
        // Expected first reference parameter `__this` to have a lifetime, found *mut crate::foo::inline1::MyStruct

        // Generated from: rs_bindings_from_cc/test/namespace/inline/inline.h;l=11
        // Error while generating bindings for constructor 'MyStruct::MyStruct':
        // Move and copy constructors do yet receive bindings. See b/452726517.
        // Expected first constructor parameter to be a mutable reference, got: *mut crate::foo::inline1::MyStruct
        // Expected first reference parameter `__this` to have a lifetime, found *mut crate::foo::inline1::MyStruct

        // Generated from: rs_bindings_from_cc/test/namespace/inline/inline.h;l=11
        // Error while generating bindings for function 'MyStruct::operator=':
        // `self` has no lifetime. Use lifetime annotations or `#pragma clang lifetime_elision` to create bindings for this function.

        // Generated from: rs_bindings_from_cc/test/namespace/inline/inline.h;l=11
        // Error while generating bindings for function 'MyStruct::operator=':
        // `self` has no lifetime. Use lifetime annotations or `#pragma clang lifetime_elision` to create bindings for this function.

        /// Generated from: rs_bindings_from_cc/test/namespace/inline/inline.h;l=15
        #[inline(always)]
        pub unsafe fn GetStructValue1(
            s: *const crate::foo::inline1::MyStruct,
        ) -> ::core::ffi::c_int {
            crate::detail::__rust_thunk___ZN3foo7inline115GetStructValue1EPKNS0_8MyStructE(s)
        }

        /// Generated from: rs_bindings_from_cc/test/namespace/inline/inline.h;l=17
        #[inline(always)]
        pub unsafe fn GetStructValue2(
            s: *const crate::foo::inline1::MyStruct,
        ) -> ::core::ffi::c_int {
            crate::detail::__rust_thunk___ZN3foo7inline115GetStructValue2EPKNS0_8MyStructE(s)
        }

        /// Generated from: rs_bindings_from_cc/test/namespace/inline/inline.h;l=26
        #[inline(always)]
        pub unsafe fn GetStructValue3(
            s: *const crate::foo::inline1::MyStruct,
        ) -> ::core::ffi::c_int {
            crate::detail::__rust_thunk___ZN3foo7inline115GetStructValue3EPKNS0_8MyStructE(s)
        }

        /// Generated from: rs_bindings_from_cc/test/namespace/inline/inline.h;l=27
        #[inline(always)]
        pub unsafe fn GetStructValue4(
            s: *const crate::foo::inline1::MyStruct,
        ) -> ::core::ffi::c_int {
            crate::detail::__rust_thunk___ZN3foo7inline115GetStructValue4EPKNS0_8MyStructE(s)
        }
    }
    #[allow(unused_imports)]
    pub use inline1::*;

    // namespace inline1
}

// namespace foo

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN3foo7inline115GetStructValue1EPKNS0_8MyStructE(
            s: *const crate::foo::inline1::MyStruct,
        ) -> ::core::ffi::c_int;
        pub(crate) unsafe fn __rust_thunk___ZN3foo7inline115GetStructValue2EPKNS0_8MyStructE(
            s: *const crate::foo::inline1::MyStruct,
        ) -> ::core::ffi::c_int;
        pub(crate) unsafe fn __rust_thunk___ZN3foo7inline115GetStructValue3EPKNS0_8MyStructE(
            s: *const crate::foo::inline1::MyStruct,
        ) -> ::core::ffi::c_int;
        pub(crate) unsafe fn __rust_thunk___ZN3foo7inline115GetStructValue4EPKNS0_8MyStructE(
            s: *const crate::foo::inline1::MyStruct,
        ) -> ::core::ffi::c_int;
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::foo::inline1::MyStruct>() == 4);
    assert!(::core::mem::align_of::<crate::foo::inline1::MyStruct>() == 4);
    static_assertions::assert_impl_all!(crate::foo::inline1::MyStruct: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::foo::inline1::MyStruct: Drop);
    assert!(::core::mem::offset_of!(crate::foo::inline1::MyStruct, value) == 0);
};
