// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:non_member_operator_cc
// Features: experimental, supported

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes, negative_impls, register_tool)]
#![allow(stable_features)]
#![no_std]
#![register_tool(__crubit)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code)]
#![deny(warnings)]

pub mod ns {
    #[derive(Clone, Copy)]
    #[repr(C)]
    #[__crubit::annotate(cpp_type = "ns :: X")]
    pub struct X {
        pub f: ::core::ffi::c_int,
    }
    impl !Send for X {}
    impl !Sync for X {}
    forward_declare::unsafe_define!(forward_declare::symbol!("ns :: X"), crate::ns::X);

    // Error while generating bindings for item 'X::X':
    // Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347.
    // Expected first constructor parameter to be a mutable reference, got: *mut crate::ns::X
    // Missing lifetime for `__this` parameter type: *mut crate::ns::X

    // Error while generating bindings for item 'X::X':
    // Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347.
    // Expected first constructor parameter to be a mutable reference, got: *mut crate::ns::X
    // Missing lifetime for `__this` parameter type: *mut crate::ns::X

    // Error while generating bindings for item 'ns::X::X':
    // Parameter #0 is not supported: Unsupported type 'X &&': Unsupported type: && without lifetime

    // Error while generating bindings for item 'X::operator=':
    // `self` has no lifetime. Use lifetime annotations or `#pragma clang lifetime_elision` to create bindings for this function.

    // Error while generating bindings for item 'ns::X::operator=':
    // Parameter #0 is not supported: Unsupported type 'X &&': Unsupported type: && without lifetime
}

// namespace ns

impl PartialEq for crate::ns::X {
    #[inline(always)]
    fn eq(&self, b: &Self) -> bool {
        unsafe { crate::detail::__rust_thunk___ZeqN2ns1XES0_(&mut self.clone(), &mut b.clone()) }
    }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZeqN2ns1XES0_(
            a: &mut crate::ns::X,
            b: &mut crate::ns::X,
        ) -> bool;
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::ns::X>() == 4);
    assert!(::core::mem::align_of::<crate::ns::X>() == 4);
    static_assertions::assert_impl_all!(crate::ns::X: Clone);
    static_assertions::assert_impl_all!(crate::ns::X: Copy);
    static_assertions::assert_not_impl_any!(crate::ns::X: Drop);
    assert!(::core::mem::offset_of!(crate::ns::X, f) == 0);
};
