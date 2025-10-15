// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:coroutines_cc

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes)]
#![allow(stable_features)]
#![no_std]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

// This is a fake version of c9::Co (http://<internal link>.h), constructed specifically
// for golden tests to avoid generating everything that //util/c9/co generates.

pub mod c9 {
    /// (Sometimes) change threads, then set the supplied bool and finish.
    #[inline(always)]
    pub unsafe fn SetBool(b: *mut bool) -> ::co::Co<'static, ()> {
        let mut __co_vtable_slot = ::co_vtable::c9::internal::rust::CoVtable {
            addr: ::core::ptr::null_mut(),
            start_coroutine: None,
            destroy_at_initial_suspend: None,
        };
        crate::detail::__rust_thunk___ZN2c97SetBoolERb(&raw mut __co_vtable_slot, b);
        ::co::Co::from_raw(__co_vtable_slot, ::co_lib::internal::consume_void)
    }

    /// Return 17, sometimes changing threads first.
    #[inline(always)]
    pub fn ReturnInt() -> ::co::Co<'static, ::core::ffi::c_int> {
        unsafe {
            let mut __co_vtable_slot = ::co_vtable::c9::internal::rust::CoVtable {
                addr: ::core::ptr::null_mut(),
                start_coroutine: None,
                destroy_at_initial_suspend: None,
            };
            crate::detail::__rust_thunk___ZN2c99ReturnIntEv(&raw mut __co_vtable_slot);
            ::co::Co::from_raw(
                __co_vtable_slot,
                ::co_lib::internal::consume_result::<
                    ::bridge_rust::TransmuteAbi<::core::ffi::c_int>,
                    {
                        <::bridge_rust::TransmuteAbi<::core::ffi::c_int>as::bridge_rust::CrubitAbi>::SIZE
                    },
                >,
            )
        }
    }
}

// namespace c9

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN2c97SetBoolERb(
            __return_co_vtable: *mut ::co_vtable::c9::internal::rust::CoVtable,
            b: *mut bool,
        );
        pub(crate) unsafe fn __rust_thunk___ZN2c99ReturnIntEv(
            __return_co_vtable: *mut ::co_vtable::c9::internal::rust::CoVtable,
        );
    }
}
