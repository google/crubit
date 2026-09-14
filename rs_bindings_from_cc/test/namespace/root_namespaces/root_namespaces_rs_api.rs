// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/namespace/root_namespaces:root_namespaces_cc

#![rustfmt::skip]
#![feature(cfi_encoding, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![allow(deprecated)]
#![allow(unknown_lints, suspicious_runtime_symbol_definitions)]
#![deny(warnings)]
pub use crate::test_namespace::*;
pub mod test_namespace {
    #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
    #[cfi_encoding = "N14test_namespace3FooE"]
    #[repr(C)]
    ///CRUBIT_ANNOTATE: cpp_type=test_namespace :: Foo
    ///CRUBIT_ANNOTATE: cpp_move_constructible=
    pub struct Foo {
        pub x: ::ffi_11::c_int,
    }
    impl !Send for Foo {}
    impl !Sync for Foo {}
    unsafe impl ::cxx::ExternType for Foo {
        type Id = ::cxx::type_id!("test_namespace :: Foo");
        type Kind = ::cxx::kind::Trivial;
    }

    impl Default for Foo {
        #[inline(always)]
        fn default() -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk___ZN14test_namespace3FooC1Ev(&raw mut tmp as *mut _);
                tmp.assume_init()
            }
        }
    }

    #[inline(always)]
    pub fn bar() {
        unsafe { crate::detail::__rust_thunk___ZN14test_namespace3barEv() }
    }
}

// namespace test_namespace

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN14test_namespace3FooC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN14test_namespace3barEv();
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::test_namespace::Foo>() == 4);
    assert!(::core::mem::align_of::<crate::test_namespace::Foo>() == 4);
    static_assertions::assert_impl_all!(crate::test_namespace::Foo: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::test_namespace::Foo: Drop);
    assert!(::core::mem::offset_of!(crate::test_namespace::Foo, x) == 0);
};
