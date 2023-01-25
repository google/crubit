// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:no_elided_lifetimes_cc

#![rustfmt::skip]
#![feature(custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![deny(warnings)]

// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

/// rs_bindings_from_cc/test/golden/no_elided_lifetimes.h;l=8
#[inline(always)]
pub unsafe fn free_function(p1: *mut i32) -> *mut i32 {
    crate::detail::__rust_thunk___Z13free_functionRi(p1)
}

/// rs_bindings_from_cc/test/golden/no_elided_lifetimes.h;l=10
#[derive(Clone, Copy)]
#[repr(C)]
pub struct S {
    __non_field_data: [::std::mem::MaybeUninit<u8>; 1],
}
forward_declare::unsafe_define!(forward_declare::symbol!("S"), crate::S);

// rs_bindings_from_cc/test/golden/no_elided_lifetimes.h;l=10
// Error while generating bindings for item 'S::S':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// rs_bindings_from_cc/test/golden/no_elided_lifetimes.h;l=10
// Error while generating bindings for item 'S::S':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// rs_bindings_from_cc/test/golden/no_elided_lifetimes.h;l=10
// Error while generating bindings for item 'S::S':
// Parameter #0 is not supported: Unsupported type 'S &&': Unsupported type: && without lifetime

// rs_bindings_from_cc/test/golden/no_elided_lifetimes.h;l=10
// Error while generating bindings for item 'S::operator=':
// operator= for Unpin types is not yet supported.

// rs_bindings_from_cc/test/golden/no_elided_lifetimes.h;l=10
// Error while generating bindings for item 'S::operator=':
// Parameter #0 is not supported: Unsupported type 'S &&': Unsupported type: && without lifetime

impl S {
    /// rs_bindings_from_cc/test/golden/no_elided_lifetimes.h;l=11
    #[inline(always)]
    pub unsafe fn const_method(__this: *const Self, p1: *mut i32, p2: *mut i32) -> *mut i32 {
        crate::detail::__rust_thunk___ZNK1S12const_methodERiS0_(__this, p1, p2)
    }
}

impl S {
    /// rs_bindings_from_cc/test/golden/no_elided_lifetimes.h;l=12
    #[inline(always)]
    pub unsafe fn method(__this: *mut Self, p1: *mut i32, p2: *mut i32) -> *mut i32 {
        crate::detail::__rust_thunk___ZN1S6methodERiS0_(__this, p1, p2)
    }
}

/// rs_bindings_from_cc/test/golden/no_elided_lifetimes.h;l=15
#[::ctor::recursively_pinned(PinnedDrop)]
#[repr(C)]
pub struct TriviallyCopyableButNontriviallyDestructible {
    __non_field_data: [::std::mem::MaybeUninit<u8>; 1],
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("TriviallyCopyableButNontriviallyDestructible"),
    crate::TriviallyCopyableButNontriviallyDestructible
);

// rs_bindings_from_cc/test/golden/no_elided_lifetimes.h;l=15
// Error while generating bindings for item 'TriviallyCopyableButNontriviallyDestructible::operator=':
// `self` has no lifetime. Use lifetime annotations or `#pragma clang lifetime_elision` to create bindings for this function.

// rs_bindings_from_cc/test/golden/no_elided_lifetimes.h;l=16
// Error while generating bindings for item 'TriviallyCopyableButNontriviallyDestructible::TriviallyCopyableButNontriviallyDestructible':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

/// rs_bindings_from_cc/test/golden/no_elided_lifetimes.h;l=18
impl ::ctor::PinnedDrop for TriviallyCopyableButNontriviallyDestructible {
    #[inline(always)]
    unsafe fn pinned_drop<'a>(self: ::std::pin::Pin<&'a mut Self>) {
        crate::detail::__rust_thunk___ZN44TriviallyCopyableButNontriviallyDestructibleD1Ev(self)
    }
}

/// rs_bindings_from_cc/test/golden/no_elided_lifetimes.h;l=21
#[inline(always)]
pub unsafe fn take_pointer(p: *mut i32) {
    crate::detail::__rust_thunk___Z12take_pointerPi(p)
}

/// rs_bindings_from_cc/test/golden/no_elided_lifetimes.h;l=23
#[::ctor::recursively_pinned]
#[repr(C, align(4))]
pub struct WrappedValue {
    __non_field_data: [::std::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) value_: [::std::mem::MaybeUninit<u8>; 4],
}
forward_declare::unsafe_define!(forward_declare::symbol!("WrappedValue"), crate::WrappedValue);

// rs_bindings_from_cc/test/golden/no_elided_lifetimes.h;l=23
// Error while generating bindings for item 'WrappedValue::WrappedValue':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// rs_bindings_from_cc/test/golden/no_elided_lifetimes.h;l=23
// Error while generating bindings for item 'WrappedValue::WrappedValue':
// Parameter #0 is not supported: Unsupported type 'WrappedValue &&': Unsupported type: && without lifetime

// rs_bindings_from_cc/test/golden/no_elided_lifetimes.h;l=23
// Error while generating bindings for item 'WrappedValue::operator=':
// `self` has no lifetime. Use lifetime annotations or `#pragma clang lifetime_elision` to create bindings for this function.

// rs_bindings_from_cc/test/golden/no_elided_lifetimes.h;l=23
// Error while generating bindings for item 'WrappedValue::operator=':
// Parameter #0 is not supported: Unsupported type 'WrappedValue &&': Unsupported type: && without lifetime

// rs_bindings_from_cc/test/golden/no_elided_lifetimes.h;l=25
// Error while generating bindings for item 'WrappedValue::WrappedValue':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// rs_bindings_from_cc/test/golden/no_elided_lifetimes.h;l=27
// Error while generating bindings for item 'WrappedValue::operator+':
// Expected first parameter to be a record or reference

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_NO_ELIDED_LIFETIMES_H_

mod detail {
    #[allow(unused_imports)]
    use super::*;
    extern "C" {
        #[link_name = "_Z13free_functionRi"]
        pub(crate) fn __rust_thunk___Z13free_functionRi(p1: *mut i32) -> *mut i32;
        #[link_name = "_ZNK1S12const_methodERiS0_"]
        pub(crate) fn __rust_thunk___ZNK1S12const_methodERiS0_(
            __this: *const crate::S,
            p1: *mut i32,
            p2: *mut i32,
        ) -> *mut i32;
        #[link_name = "_ZN1S6methodERiS0_"]
        pub(crate) fn __rust_thunk___ZN1S6methodERiS0_(
            __this: *mut crate::S,
            p1: *mut i32,
            p2: *mut i32,
        ) -> *mut i32;
        #[link_name = "_ZN44TriviallyCopyableButNontriviallyDestructibleD1Ev"]
        pub(crate) fn __rust_thunk___ZN44TriviallyCopyableButNontriviallyDestructibleD1Ev<'a>(
            __this: ::std::pin::Pin<&'a mut crate::TriviallyCopyableButNontriviallyDestructible>,
        );
        #[link_name = "_Z12take_pointerPi"]
        pub(crate) fn __rust_thunk___Z12take_pointerPi(p: *mut i32);
    }
}

const _: () = assert!(::std::mem::size_of::<Option<&i32>>() == ::std::mem::size_of::<&i32>());

const _: () = assert!(::std::mem::size_of::<crate::S>() == 1);
const _: () = assert!(::std::mem::align_of::<crate::S>() == 1);
const _: () = {
    static_assertions::assert_impl_all!(crate::S: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::S: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::S: Drop);
};

const _: () =
    assert!(::std::mem::size_of::<crate::TriviallyCopyableButNontriviallyDestructible>() == 1);
const _: () =
    assert!(::std::mem::align_of::<crate::TriviallyCopyableButNontriviallyDestructible>() == 1);
const _: () = {
    static_assertions::assert_not_impl_any!(
        crate::TriviallyCopyableButNontriviallyDestructible: Copy
    );
};
const _: () = {
    static_assertions::assert_impl_all!(crate::TriviallyCopyableButNontriviallyDestructible: Drop);
};

const _: () = assert!(::std::mem::size_of::<crate::WrappedValue>() == 4);
const _: () = assert!(::std::mem::align_of::<crate::WrappedValue>() == 4);
const _: () = {
    static_assertions::assert_not_impl_any!(crate::WrappedValue: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::WrappedValue: Drop);
};
const _: () = assert!(memoffset::offset_of!(crate::WrappedValue, value_) == 0);
