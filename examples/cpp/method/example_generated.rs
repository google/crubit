// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //examples/cpp/method:example_lib

#![rustfmt::skip]
#![feature(custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![allow(deprecated)]
#![deny(warnings)]

pub mod foo {
    #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
    #[repr(C)]
    ///CRUBIT_ANNOTATE: cpp_type=foo :: Bar
    pub struct Bar {
        pub x: ::ffi_11::c_int,
    }
    impl !Send for Bar {}
    impl !Sync for Bar {}
    unsafe impl ::cxx::ExternType for Bar {
        type Id = ::cxx::type_id!("foo :: Bar");
        type Kind = ::cxx::kind::Trivial;
    }
    impl Bar {
        /// # Safety
        ///
        /// The caller must ensure that the following unsafe arguments are not misused by the function:
        /// * `__this`: raw pointer
        #[inline(always)]
        pub unsafe fn MyMethod(__this: *mut Self) {
            unsafe { self::bar::MyMethod(__this) }
        }
    }

    impl Default for Bar {
        #[inline(always)]
        fn default() -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk___ZN3foo3BarC1Ev(&raw mut tmp as *mut _);
                tmp.assume_init()
            }
        }
    }

    pub mod bar {
        /// # Safety
        ///
        /// The caller must ensure that the following unsafe arguments are not misused by the function:
        /// * `__this`: raw pointer
        #[inline(always)]
        pub(crate) unsafe fn MyMethod(__this: *mut crate::foo::Bar) {
            unsafe { crate::detail::__rust_thunk___ZN3foo3Bar8MyMethodEv(__this) }
        }
    }
}

// namespace foo

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN3foo3BarC1Ev(__this: *mut ::core::ffi::c_void);
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
