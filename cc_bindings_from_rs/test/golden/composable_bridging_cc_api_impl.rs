// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// composable_bridging_rust_golden
// Features: callables, custom_ffi_types, experimental, fmt, non_unpin_ctor, std_unique_ptr, std_vector, supported, wrapper

#![allow(unused_unsafe, deprecated, non_snake_case, unreachable_code)]
#![allow(improper_ctypes_definitions)]
#![deny(warnings)]

#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_make_umy_uoption_urust(
    __ret_ptr: *mut core::ffi::c_uchar,
) -> () {
    unsafe {
        let __rs_return_value = ::composable_bridging_rust_golden::make_my_option_rust();
        unsafe {
            ::bridge_rust::internal::encode(
                ::composable_bridging_rust_golden::MyOptionRustAbi(::bridge_rust::transmute_abi::<
                    i32,
                >()),
                __ret_ptr as *mut core::ffi::c_uchar,
                __rs_return_value,
            );
        }
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_maybe_uint_uslice(__ret_ptr: *mut core::ffi::c_uchar) -> () {
    unsafe {
        let __rs_return_value = ::composable_bridging_rust_golden::maybe_int_slice();
        unsafe {
            ::bridge_rust::internal::encode(
                ::bridge_rust::OptionAbi(::bridge_rust::transmute_abi::<*const [i32]>()),
                __ret_ptr as *mut core::ffi::c_uchar,
                __rs_return_value,
            );
        }
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_option_uadds_uone_uto_uref(
    x: *const core::ffi::c_uchar,
    __ret_ptr: *mut core::ffi::c_uchar,
) -> () {
    unsafe {
        let x = unsafe {
            ::bridge_rust::internal::decode(
                ::bridge_rust::OptionAbi(::bridge_rust::transmute_abi::<&'static mut i32>()),
                x,
            )
        };
        let __rs_return_value = ::composable_bridging_rust_golden::option_adds_one_to_ref(x);
        unsafe {
            ::bridge_rust::internal::encode(
                ::bridge_rust::OptionAbi(::bridge_rust::transmute_abi::<&'static mut i32>()),
                __ret_ptr as *mut core::ffi::c_uchar,
                __rs_return_value,
            );
        }
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_option_uincrements(
    x: *const core::ffi::c_uchar,
    __ret_ptr: *mut core::ffi::c_uchar,
) -> () {
    unsafe {
        let x = unsafe {
            ::bridge_rust::internal::decode(
                ::bridge_rust::OptionAbi(::bridge_rust::transmute_abi::<i32>()),
                x,
            )
        };
        let __rs_return_value = ::composable_bridging_rust_golden::option_increments(x);
        unsafe {
            ::bridge_rust::internal::encode(
                ::bridge_rust::OptionAbi(::bridge_rust::transmute_abi::<i32>()),
                __ret_ptr as *mut core::ffi::c_uchar,
                __rs_return_value,
            );
        }
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_option_uslice_uwithout_ufirst(
    x: *const core::ffi::c_uchar,
    __ret_ptr: *mut core::ffi::c_uchar,
) -> () {
    unsafe {
        let x = unsafe {
            ::bridge_rust::internal::decode(
                ::bridge_rust::OptionAbi(::bridge_rust::transmute_abi::<&'static [i32]>()),
                x,
            )
        };
        let __rs_return_value = ::composable_bridging_rust_golden::option_slice_without_first(x);
        unsafe {
            ::bridge_rust::internal::encode(
                ::bridge_rust::OptionAbi(::bridge_rust::transmute_abi::<&'static [i32]>()),
                __ret_ptr as *mut core::ffi::c_uchar,
                __rs_return_value,
            );
        }
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_returns_uno_uint(__ret_ptr: *mut core::ffi::c_uchar) -> () {
    unsafe {
        let __rs_return_value = ::composable_bridging_rust_golden::returns_no_int();
        unsafe {
            ::bridge_rust::internal::encode(
                ::bridge_rust::OptionAbi(::bridge_rust::transmute_abi::<i32>()),
                __ret_ptr as *mut core::ffi::c_uchar,
                __rs_return_value,
            );
        }
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_returns_usome_uint(__ret_ptr: *mut core::ffi::c_uchar) -> () {
    unsafe {
        let __rs_return_value = ::composable_bridging_rust_golden::returns_some_int();
        unsafe {
            ::bridge_rust::internal::encode(
                ::bridge_rust::OptionAbi(::bridge_rust::transmute_abi::<i32>()),
                __ret_ptr as *mut core::ffi::c_uchar,
                __rs_return_value,
            );
        }
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_unwrap_uor_uzero(x: *const core::ffi::c_uchar) -> i32 {
    unsafe {
        let x = unsafe {
            ::bridge_rust::internal::decode(
                ::bridge_rust::OptionAbi(::bridge_rust::transmute_abi::<i32>()),
                x,
            )
        };
        ::composable_bridging_rust_golden::unwrap_or_zero(x)
    }
}
