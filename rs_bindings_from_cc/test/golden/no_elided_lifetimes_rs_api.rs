// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:no_elided_lifetimes_cc
// Features: experimental, supported

#![rustfmt::skip]
#![feature(custom_inner_attributes, negative_impls, register_tool)]
#![allow(stable_features)]
#![no_std]
#![register_tool(__crubit)]
#![allow(improper_ctypes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![deny(warnings)]

// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[inline(always)]
pub unsafe fn free_function(p1: *mut ::core::ffi::c_int) -> *mut ::core::ffi::c_int {
    crate::detail::__rust_thunk___Z13free_functionRi(p1)
}

#[derive(Clone, Copy)]
#[repr(C)]
#[__crubit::annotate(cc_type = "S")]
pub struct S {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
forward_declare::unsafe_define!(forward_declare::symbol!("S"), crate::S);

// Error while generating bindings for item 'S::S':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// Error while generating bindings for item 'S::S':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// Error while generating bindings for item 'S::S':
// Parameter #0 is not supported: Unsupported type 'S &&': Unsupported type: && without lifetime

// Error while generating bindings for item 'S::operator=':
// `self` has no lifetime. Use lifetime annotations or `#pragma clang lifetime_elision` to create bindings for this function.

// Error while generating bindings for item 'S::operator=':
// Parameter #0 is not supported: Unsupported type 'S &&': Unsupported type: && without lifetime

impl S {
    #[inline(always)]
    pub unsafe fn const_method(
        __this: *const Self,
        p1: *mut ::core::ffi::c_int,
        p2: *mut ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_int {
        crate::detail::__rust_thunk___ZNK1S12const_methodERiS0_(__this, p1, p2)
    }
}

impl S {
    #[inline(always)]
    pub unsafe fn method(
        __this: *mut Self,
        p1: *mut ::core::ffi::c_int,
        p2: *mut ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_int {
        crate::detail::__rust_thunk___ZN1S6methodERiS0_(__this, p1, p2)
    }
}

#[::ctor::recursively_pinned(PinnedDrop)]
#[repr(C)]
#[__crubit::annotate(cc_type = "TriviallyCopyableButNontriviallyDestructible")]
pub struct TriviallyCopyableButNontriviallyDestructible {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("TriviallyCopyableButNontriviallyDestructible"),
    crate::TriviallyCopyableButNontriviallyDestructible
);

// Error while generating bindings for item 'TriviallyCopyableButNontriviallyDestructible::operator=':
// `self` has no lifetime. Use lifetime annotations or `#pragma clang lifetime_elision` to create bindings for this function.

// Error while generating bindings for item 'TriviallyCopyableButNontriviallyDestructible::TriviallyCopyableButNontriviallyDestructible':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

impl ::ctor::PinnedDrop for TriviallyCopyableButNontriviallyDestructible {
    #[inline(always)]
    unsafe fn pinned_drop<'a>(self: ::core::pin::Pin<&'a mut Self>) {
        crate::detail::__rust_thunk___ZN44TriviallyCopyableButNontriviallyDestructibleD1Ev(self)
    }
}

#[inline(always)]
pub unsafe fn take_pointer(p: *mut ::core::ffi::c_int) {
    crate::detail::__rust_thunk___Z12take_pointerPi(p)
}

#[derive(Clone, Copy)]
#[repr(C, align(4))]
#[__crubit::annotate(cc_type = "WrappedValue")]
pub struct WrappedValue {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) value_: [::core::mem::MaybeUninit<u8>; 4],
}
forward_declare::unsafe_define!(forward_declare::symbol!("WrappedValue"), crate::WrappedValue);

// Error while generating bindings for item 'WrappedValue::WrappedValue':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// Error while generating bindings for item 'WrappedValue::WrappedValue':
// Parameter #0 is not supported: Unsupported type 'WrappedValue &&': Unsupported type: && without lifetime

// Error while generating bindings for item 'WrappedValue::operator=':
// `self` has no lifetime. Use lifetime annotations or `#pragma clang lifetime_elision` to create bindings for this function.

// Error while generating bindings for item 'WrappedValue::operator=':
// Parameter #0 is not supported: Unsupported type 'WrappedValue &&': Unsupported type: && without lifetime

// Error while generating bindings for item 'WrappedValue::WrappedValue':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// Error while generating bindings for item 'WrappedValue::operator+':
// Expected first parameter to be a record or reference

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_NO_ELIDED_LIFETIMES_H_

mod detail {
    #[allow(unused_imports)]
    use super::*;
    extern "C" {
        #[link_name = "_Z13free_functionRi"]
        pub(crate) fn __rust_thunk___Z13free_functionRi(
            p1: *mut ::core::ffi::c_int,
        ) -> *mut ::core::ffi::c_int;
        #[link_name = "_ZNK1S12const_methodERiS0_"]
        pub(crate) fn __rust_thunk___ZNK1S12const_methodERiS0_(
            __this: *const crate::S,
            p1: *mut ::core::ffi::c_int,
            p2: *mut ::core::ffi::c_int,
        ) -> *mut ::core::ffi::c_int;
        #[link_name = "_ZN1S6methodERiS0_"]
        pub(crate) fn __rust_thunk___ZN1S6methodERiS0_(
            __this: *mut crate::S,
            p1: *mut ::core::ffi::c_int,
            p2: *mut ::core::ffi::c_int,
        ) -> *mut ::core::ffi::c_int;
        #[link_name = "_ZN44TriviallyCopyableButNontriviallyDestructibleD1Ev"]
        pub(crate) fn __rust_thunk___ZN44TriviallyCopyableButNontriviallyDestructibleD1Ev<'a>(
            __this: ::core::pin::Pin<&'a mut crate::TriviallyCopyableButNontriviallyDestructible>,
        );
        #[link_name = "_Z12take_pointerPi"]
        pub(crate) fn __rust_thunk___Z12take_pointerPi(p: *mut ::core::ffi::c_int);
    }
}

const _: () = assert!(::core::mem::size_of::<Option<&i32>>() == ::core::mem::size_of::<&i32>());

const _: () = assert!(::core::mem::size_of::<crate::S>() == 1);
const _: () = assert!(::core::mem::align_of::<crate::S>() == 1);
const _: () = {
    static_assertions::assert_impl_all!(crate::S:Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::S:Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::S:Drop);
};

const _: () =
    assert!(::core::mem::size_of::<crate::TriviallyCopyableButNontriviallyDestructible>() == 1);
const _: () =
    assert!(::core::mem::align_of::<crate::TriviallyCopyableButNontriviallyDestructible>() == 1);
const _: () = {
    static_assertions::assert_not_impl_any!(crate::TriviallyCopyableButNontriviallyDestructible:Copy);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::TriviallyCopyableButNontriviallyDestructible:Drop);
};

const _: () = assert!(::core::mem::size_of::<crate::WrappedValue>() == 4);
const _: () = assert!(::core::mem::align_of::<crate::WrappedValue>() == 4);
const _: () = {
    static_assertions::assert_impl_all!(crate::WrappedValue:Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::WrappedValue:Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::WrappedValue:Drop);
};
const _: () = assert!(memoffset::offset_of!(crate::WrappedValue, value_) == 0);
