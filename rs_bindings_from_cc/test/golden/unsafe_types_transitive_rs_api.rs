// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:unsafe_types_transitive_cc

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
///CRUBIT_ANNOTATE: cpp_type=PublicPointer
pub struct PublicPointer {
    pub p: *mut ::core::ffi::c_int,
}
impl !Send for PublicPointer {}
impl !Sync for PublicPointer {}
forward_declare::unsafe_define!(forward_declare::symbol!("PublicPointer"), crate::PublicPointer);

// Error while generating bindings for item 'PublicPointer::PublicPointer':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347.
// Expected first constructor parameter to be a mutable reference, got: *mut crate::PublicPointer
// Missing lifetime for `__this` parameter type: *mut crate::PublicPointer

// Error while generating bindings for item 'PublicPointer::PublicPointer':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347.
// Expected first constructor parameter to be a mutable reference, got: *mut crate::PublicPointer
// Missing lifetime for `__this` parameter type: *mut crate::PublicPointer

// Error while generating bindings for item 'PublicPointer::PublicPointer':
// Parameter #0 is not supported: Unsupported type 'PublicPointer &&': Unsupported type: && without lifetime

// Error while generating bindings for item 'PublicPointer::operator=':
// `self` has no lifetime. Use lifetime annotations or `#pragma clang lifetime_elision` to create bindings for this function.

// Error while generating bindings for item 'PublicPointer::operator=':
// Parameter #0 is not supported: Unsupported type 'PublicPointer &&': Unsupported type: && without lifetime

#[derive(Clone, Copy)]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=PrivatePointer
pub struct PrivatePointer {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) p_: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for PrivatePointer {}
impl !Sync for PrivatePointer {}
forward_declare::unsafe_define!(forward_declare::symbol!("PrivatePointer"), crate::PrivatePointer);

// Error while generating bindings for item 'PrivatePointer::PrivatePointer':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347.
// Expected first constructor parameter to be a mutable reference, got: *mut crate::PrivatePointer
// Missing lifetime for `__this` parameter type: *mut crate::PrivatePointer

// Error while generating bindings for item 'PrivatePointer::PrivatePointer':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347.
// Expected first constructor parameter to be a mutable reference, got: *mut crate::PrivatePointer
// Missing lifetime for `__this` parameter type: *mut crate::PrivatePointer

// Error while generating bindings for item 'PrivatePointer::PrivatePointer':
// Parameter #0 is not supported: Unsupported type 'PrivatePointer &&': Unsupported type: && without lifetime

// Error while generating bindings for item 'PrivatePointer::operator=':
// `self` has no lifetime. Use lifetime annotations or `#pragma clang lifetime_elision` to create bindings for this function.

// Error while generating bindings for item 'PrivatePointer::operator=':
// Parameter #0 is not supported: Unsupported type 'PrivatePointer &&': Unsupported type: && without lifetime

#[inline(always)]
pub fn DerefPrivatePointer(mut p: crate::PrivatePointer) -> ::core::ffi::c_int {
    unsafe { crate::detail::__rust_thunk___Z19DerefPrivatePointer14PrivatePointer(&mut p) }
}

#[derive(Clone, Copy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=TransitivePublicPointer
pub struct TransitivePublicPointer {
    pub r#pub: crate::PublicPointer,
    pub r#priv: crate::PrivatePointer,
}
impl !Send for TransitivePublicPointer {}
impl !Sync for TransitivePublicPointer {}
forward_declare::unsafe_define!(
    forward_declare::symbol!("TransitivePublicPointer"),
    crate::TransitivePublicPointer
);

// Error while generating bindings for item 'TransitivePublicPointer::TransitivePublicPointer':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347.
// Expected first constructor parameter to be a mutable reference, got: *mut crate::TransitivePublicPointer
// Missing lifetime for `__this` parameter type: *mut crate::TransitivePublicPointer

// Error while generating bindings for item 'TransitivePublicPointer::TransitivePublicPointer':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347.
// Expected first constructor parameter to be a mutable reference, got: *mut crate::TransitivePublicPointer
// Missing lifetime for `__this` parameter type: *mut crate::TransitivePublicPointer

// Error while generating bindings for item 'TransitivePublicPointer::TransitivePublicPointer':
// Parameter #0 is not supported: Unsupported type 'TransitivePublicPointer &&': Unsupported type: && without lifetime

// Error while generating bindings for item 'TransitivePublicPointer::operator=':
// `self` has no lifetime. Use lifetime annotations or `#pragma clang lifetime_elision` to create bindings for this function.

// Error while generating bindings for item 'TransitivePublicPointer::operator=':
// Parameter #0 is not supported: Unsupported type 'TransitivePublicPointer &&': Unsupported type: && without lifetime

#[derive(Clone, Copy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=Union
pub union Union {
    pub i: ::core::ffi::c_int,
    pub f: f32,
}
impl !Send for Union {}
impl !Sync for Union {}
forward_declare::unsafe_define!(forward_declare::symbol!("Union"), crate::Union);

// Error while generating bindings for item 'Union::Union':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347.
// Expected first constructor parameter to be a mutable reference, got: *mut crate::Union
// Missing lifetime for `__this` parameter type: *mut crate::Union

// Error while generating bindings for item 'Union::Union':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347.
// Expected first constructor parameter to be a mutable reference, got: *mut crate::Union
// Missing lifetime for `__this` parameter type: *mut crate::Union

// Error while generating bindings for item 'Union::Union':
// Parameter #0 is not supported: Unsupported type 'Union &&': Unsupported type: && without lifetime

// Error while generating bindings for item 'Union::operator=':
// `self` has no lifetime. Use lifetime annotations or `#pragma clang lifetime_elision` to create bindings for this function.

// Error while generating bindings for item 'Union::operator=':
// Parameter #0 is not supported: Unsupported type 'Union &&': Unsupported type: && without lifetime

#[inline(always)]
pub unsafe fn DerefPointer(p: *mut ::core::ffi::c_int) -> ::core::ffi::c_int {
    crate::detail::__rust_thunk___Z12DerefPointerPi(p)
}

#[inline(always)]
pub unsafe fn DerefPublicPointer(mut p: crate::PublicPointer) -> ::core::ffi::c_int {
    crate::detail::__rust_thunk___Z18DerefPublicPointer13PublicPointer(&mut p)
}

#[inline(always)]
pub unsafe fn DerefTransitivePublicPointer(
    mut p: crate::TransitivePublicPointer,
) -> ::core::ffi::c_int {
    crate::detail::__rust_thunk___Z28DerefTransitivePublicPointer23TransitivePublicPointer(&mut p)
}

#[inline(always)]
pub unsafe fn ReadUnion(mut u: crate::Union) -> ::core::ffi::c_int {
    crate::detail::__rust_thunk___Z9ReadUnion5Union(&mut u)
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___Z19DerefPrivatePointer14PrivatePointer(
            p: &mut crate::PrivatePointer,
        ) -> ::core::ffi::c_int;
        #[link_name = "_Z12DerefPointerPi"]
        pub(crate) unsafe fn __rust_thunk___Z12DerefPointerPi(
            p: *mut ::core::ffi::c_int,
        ) -> ::core::ffi::c_int;
        pub(crate) unsafe fn __rust_thunk___Z18DerefPublicPointer13PublicPointer(
            p: &mut crate::PublicPointer,
        ) -> ::core::ffi::c_int;
        pub(crate) unsafe fn __rust_thunk___Z28DerefTransitivePublicPointer23TransitivePublicPointer(
            p: &mut crate::TransitivePublicPointer,
        ) -> ::core::ffi::c_int;
        pub(crate) unsafe fn __rust_thunk___Z9ReadUnion5Union(
            u: &mut crate::Union,
        ) -> ::core::ffi::c_int;
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::PublicPointer>() == 8);
    assert!(::core::mem::align_of::<crate::PublicPointer>() == 8);
    static_assertions::assert_impl_all!(crate::PublicPointer: Clone);
    static_assertions::assert_impl_all!(crate::PublicPointer: Copy);
    static_assertions::assert_not_impl_any!(crate::PublicPointer: Drop);
    assert!(::core::mem::offset_of!(crate::PublicPointer, p) == 0);

    assert!(::core::mem::size_of::<crate::PrivatePointer>() == 8);
    assert!(::core::mem::align_of::<crate::PrivatePointer>() == 8);
    static_assertions::assert_impl_all!(crate::PrivatePointer: Clone);
    static_assertions::assert_impl_all!(crate::PrivatePointer: Copy);
    static_assertions::assert_not_impl_any!(crate::PrivatePointer: Drop);
    assert!(::core::mem::offset_of!(crate::PrivatePointer, p_) == 0);

    assert!(::core::mem::size_of::<crate::TransitivePublicPointer>() == 16);
    assert!(::core::mem::align_of::<crate::TransitivePublicPointer>() == 8);
    static_assertions::assert_impl_all!(crate::TransitivePublicPointer: Clone);
    static_assertions::assert_impl_all!(crate::TransitivePublicPointer: Copy);
    static_assertions::assert_not_impl_any!(crate::TransitivePublicPointer: Drop);
    assert!(::core::mem::offset_of!(crate::TransitivePublicPointer, r#pub) == 0);
    assert!(::core::mem::offset_of!(crate::TransitivePublicPointer, r#priv) == 8);

    assert!(::core::mem::size_of::<crate::Union>() == 4);
    assert!(::core::mem::align_of::<crate::Union>() == 4);
    static_assertions::assert_impl_all!(crate::Union: Clone);
    static_assertions::assert_impl_all!(crate::Union: Copy);
    static_assertions::assert_not_impl_any!(crate::Union: Drop);
    assert!(::core::mem::offset_of!(crate::Union, i) == 0);
    assert!(::core::mem::offset_of!(crate::Union, f) == 0);
    static_assertions::assert_impl_all!(::core::ffi::c_int: Copy);
    static_assertions::assert_impl_all!(f32: Copy);
};
