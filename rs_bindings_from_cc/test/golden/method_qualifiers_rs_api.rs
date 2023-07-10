// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:method_qualifiers_cc
// Features: experimental, supported

#![rustfmt::skip]
#![feature(custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![no_std]
#![allow(improper_ctypes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![deny(warnings)]

// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[::ctor::recursively_pinned]
#[repr(C)]
pub struct Noninline {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
forward_declare::unsafe_define!(forward_declare::symbol!("Noninline"), crate::Noninline);

// Error while generating bindings for item 'Noninline::Noninline':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// Error while generating bindings for item 'Noninline::Noninline':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// Error while generating bindings for item 'Noninline::Noninline':
// Parameter #0 is not supported: Unsupported type 'Noninline &&': Unsupported type: && without lifetime

// Error while generating bindings for item 'Noninline::operator=':
// `self` has no lifetime. Use lifetime annotations or `#pragma clang lifetime_elision` to create bindings for this function.

// Error while generating bindings for item 'Noninline::operator=':
// Parameter #0 is not supported: Unsupported type 'Noninline &&': Unsupported type: && without lifetime

impl Noninline {
    #[inline(always)]
    pub unsafe fn UnqualifiedMethod(__this: *mut Self) {
        crate::detail::__rust_thunk___ZN9Noninline17UnqualifiedMethodEv(__this)
    }
}

impl Noninline {
    #[inline(always)]
    pub unsafe fn LvalueMethod(__this: *mut Self) {
        crate::detail::__rust_thunk___ZNR9Noninline12LvalueMethodEv(__this)
    }
}

impl Noninline {
    #[inline(always)]
    pub unsafe fn LvalueMethodConst(__this: *const Self) {
        crate::detail::__rust_thunk___ZNKR9Noninline17LvalueMethodConstEv(__this)
    }
}

impl Noninline {
    #[inline(always)]
    pub unsafe fn RvalueMethod(__this: *mut Self) {
        crate::detail::__rust_thunk___ZNO9Noninline12RvalueMethodEv(__this)
    }
}

impl Noninline {
    #[inline(always)]
    pub unsafe fn RvalueMethodConst(__this: *const Self) {
        crate::detail::__rust_thunk___ZNKO9Noninline17RvalueMethodConstEv(__this)
    }
}

#[::ctor::recursively_pinned]
#[repr(C)]
pub struct Inline {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
forward_declare::unsafe_define!(forward_declare::symbol!("Inline"), crate::Inline);

// Error while generating bindings for item 'Inline::Inline':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// Error while generating bindings for item 'Inline::Inline':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// Error while generating bindings for item 'Inline::Inline':
// Parameter #0 is not supported: Unsupported type 'Inline &&': Unsupported type: && without lifetime

// Error while generating bindings for item 'Inline::operator=':
// `self` has no lifetime. Use lifetime annotations or `#pragma clang lifetime_elision` to create bindings for this function.

// Error while generating bindings for item 'Inline::operator=':
// Parameter #0 is not supported: Unsupported type 'Inline &&': Unsupported type: && without lifetime

impl Inline {
    #[inline(always)]
    pub unsafe fn UnqualifiedMethod(__this: *mut Self) {
        crate::detail::__rust_thunk___ZN6Inline17UnqualifiedMethodEv(__this)
    }
}

impl Inline {
    #[inline(always)]
    pub unsafe fn LvalueMethod(__this: *mut Self) {
        crate::detail::__rust_thunk___ZNR6Inline12LvalueMethodEv(__this)
    }
}

impl Inline {
    #[inline(always)]
    pub unsafe fn LvalueMethodConst(__this: *const Self) {
        crate::detail::__rust_thunk___ZNKR6Inline17LvalueMethodConstEv(__this)
    }
}

impl Inline {
    #[inline(always)]
    pub unsafe fn RvalueMethod(__this: *mut Self) {
        crate::detail::__rust_thunk___ZNO6Inline12RvalueMethodEv(__this)
    }
}

impl Inline {
    #[inline(always)]
    pub unsafe fn RvalueMethodConst(__this: *const Self) {
        crate::detail::__rust_thunk___ZNKO6Inline17RvalueMethodConstEv(__this)
    }
}

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_METHOD_QUALIFIERS_H_

mod detail {
    #[allow(unused_imports)]
    use super::*;
    extern "C" {
        #[link_name = "_ZN9Noninline17UnqualifiedMethodEv"]
        pub(crate) fn __rust_thunk___ZN9Noninline17UnqualifiedMethodEv(
            __this: *mut crate::Noninline,
        );
        #[link_name = "_ZNR9Noninline12LvalueMethodEv"]
        pub(crate) fn __rust_thunk___ZNR9Noninline12LvalueMethodEv(__this: *mut crate::Noninline);
        #[link_name = "_ZNKR9Noninline17LvalueMethodConstEv"]
        pub(crate) fn __rust_thunk___ZNKR9Noninline17LvalueMethodConstEv(
            __this: *const crate::Noninline,
        );
        #[link_name = "_ZNO9Noninline12RvalueMethodEv"]
        pub(crate) fn __rust_thunk___ZNO9Noninline12RvalueMethodEv(__this: *mut crate::Noninline);
        #[link_name = "_ZNKO9Noninline17RvalueMethodConstEv"]
        pub(crate) fn __rust_thunk___ZNKO9Noninline17RvalueMethodConstEv(
            __this: *const crate::Noninline,
        );
        pub(crate) fn __rust_thunk___ZN6Inline17UnqualifiedMethodEv(__this: *mut crate::Inline);
        pub(crate) fn __rust_thunk___ZNR6Inline12LvalueMethodEv(__this: *mut crate::Inline);
        pub(crate) fn __rust_thunk___ZNKR6Inline17LvalueMethodConstEv(__this: *const crate::Inline);
        pub(crate) fn __rust_thunk___ZNO6Inline12RvalueMethodEv(__this: *mut crate::Inline);
        pub(crate) fn __rust_thunk___ZNKO6Inline17RvalueMethodConstEv(__this: *const crate::Inline);
    }
}

const _: () = assert!(::core::mem::size_of::<Option<&i32>>() == ::core::mem::size_of::<&i32>());

const _: () = assert!(::core::mem::size_of::<crate::Noninline>() == 1);
const _: () = assert!(::core::mem::align_of::<crate::Noninline>() == 1);
const _: () = {
    static_assertions::assert_not_impl_any!(crate::Noninline:Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::Noninline:Drop);
};

const _: () = assert!(::core::mem::size_of::<crate::Inline>() == 1);
const _: () = assert!(::core::mem::align_of::<crate::Inline>() == 1);
const _: () = {
    static_assertions::assert_not_impl_any!(crate::Inline:Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::Inline:Drop);
};
