// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:method_qualifiers_cc

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes, negative_impls, register_tool)]
#![allow(stable_features)]
#![no_std]
#![register_tool(__crubit)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code)]
#![deny(warnings)]

#[derive(Clone, Copy)]
#[repr(C)]
#[__crubit::annotate(cpp_type = "Noninline")]
pub struct Noninline {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for Noninline {}
impl !Sync for Noninline {}
forward_declare::unsafe_define!(forward_declare::symbol!("Noninline"), crate::Noninline);

// Error while generating bindings for item 'Noninline::Noninline':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347.
// Expected first constructor parameter to be a mutable reference, got: *mut crate::Noninline
// Missing lifetime for `__this` parameter type: *mut crate::Noninline

// Error while generating bindings for item 'Noninline::Noninline':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347.
// Expected first constructor parameter to be a mutable reference, got: *mut crate::Noninline
// Missing lifetime for `__this` parameter type: *mut crate::Noninline

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

#[derive(Clone, Copy)]
#[repr(C)]
#[__crubit::annotate(cpp_type = "Inline")]
pub struct Inline {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for Inline {}
impl !Sync for Inline {}
forward_declare::unsafe_define!(forward_declare::symbol!("Inline"), crate::Inline);

// Error while generating bindings for item 'Inline::Inline':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347.
// Expected first constructor parameter to be a mutable reference, got: *mut crate::Inline
// Missing lifetime for `__this` parameter type: *mut crate::Inline

// Error while generating bindings for item 'Inline::Inline':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347.
// Expected first constructor parameter to be a mutable reference, got: *mut crate::Inline
// Missing lifetime for `__this` parameter type: *mut crate::Inline

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

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        #[link_name = "_ZN9Noninline17UnqualifiedMethodEv"]
        pub(crate) unsafe fn __rust_thunk___ZN9Noninline17UnqualifiedMethodEv(
            __this: *mut crate::Noninline,
        );
        #[link_name = "_ZNR9Noninline12LvalueMethodEv"]
        pub(crate) unsafe fn __rust_thunk___ZNR9Noninline12LvalueMethodEv(
            __this: *mut crate::Noninline,
        );
        #[link_name = "_ZNKR9Noninline17LvalueMethodConstEv"]
        pub(crate) unsafe fn __rust_thunk___ZNKR9Noninline17LvalueMethodConstEv(
            __this: *const crate::Noninline,
        );
        #[link_name = "_ZNO9Noninline12RvalueMethodEv"]
        pub(crate) unsafe fn __rust_thunk___ZNO9Noninline12RvalueMethodEv(
            __this: *mut crate::Noninline,
        );
        #[link_name = "_ZNKO9Noninline17RvalueMethodConstEv"]
        pub(crate) unsafe fn __rust_thunk___ZNKO9Noninline17RvalueMethodConstEv(
            __this: *const crate::Noninline,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6Inline17UnqualifiedMethodEv(
            __this: *mut crate::Inline,
        );
        pub(crate) unsafe fn __rust_thunk___ZNR6Inline12LvalueMethodEv(__this: *mut crate::Inline);
        pub(crate) unsafe fn __rust_thunk___ZNKR6Inline17LvalueMethodConstEv(
            __this: *const crate::Inline,
        );
        pub(crate) unsafe fn __rust_thunk___ZNO6Inline12RvalueMethodEv(__this: *mut crate::Inline);
        pub(crate) unsafe fn __rust_thunk___ZNKO6Inline17RvalueMethodConstEv(
            __this: *const crate::Inline,
        );
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::Noninline>() == 1);
    assert!(::core::mem::align_of::<crate::Noninline>() == 1);
    static_assertions::assert_impl_all!(crate::Noninline: Clone);
    static_assertions::assert_impl_all!(crate::Noninline: Copy);
    static_assertions::assert_not_impl_any!(crate::Noninline: Drop);

    assert!(::core::mem::size_of::<crate::Inline>() == 1);
    assert!(::core::mem::align_of::<crate::Inline>() == 1);
    static_assertions::assert_impl_all!(crate::Inline: Clone);
    static_assertions::assert_impl_all!(crate::Inline: Copy);
    static_assertions::assert_not_impl_any!(crate::Inline: Drop);
};
