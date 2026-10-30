// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/annotations:rust_layout_equivalent

#![rustfmt::skip]
#![feature(custom_inner_attributes)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![allow(deprecated)]
#![allow(unknown_lints, suspicious_runtime_symbol_definitions)]
#![deny(warnings)]
extern crate self as inline_cpp_generated_bindings;

pub mod crubit {
    pub mod test {
        // Type bindings for crubit::test::SameCrateCppType suppressed due to being mapped to an existing Rust type (crate::SameCrateRustType)

        #[inline(always)]
        pub fn AddOneToSameCrate(v: crate::SameCrateRustType) -> crate::SameCrateRustType {
            let mut v = ::core::mem::MaybeUninit::new(v);
            unsafe {
                let mut __crubit_return =
                    ::core::mem::MaybeUninit::<crate::SameCrateRustType>::uninit();
                crate::detail::__rust_thunk___ZN6crubit4test17AddOneToSameCrateENS0_16SameCrateCppTypeE(&raw mut __crubit_return as*mut::core::ffi::c_void,v.as_mut_ptr());
                __crubit_return.assume_init()
            }
        }

        // error: class `crubit::test::TaggedValue` could not be bound
        //   Class templates are not yet supported

        #[inline(always)]
        pub fn AddTag(v: crate::TaggedValue<::ffi_11::c_int, 10i64>) -> i64 {
            let mut v = ::core::mem::MaybeUninit::new(v);
            unsafe {
                crate::detail::__rust_thunk___ZN6crubit4test6AddTagENS0_11TaggedValueIiLl10EEE(
                    v.as_mut_ptr(),
                )
            }
        }

        // Type bindings for crubit::test::ExternalCppType suppressed due to being mapped to an existing Rust type (::rust_layout_equivalent_other_crate::ExternalRustType)

        #[inline(always)]
        pub fn DoubleExternal(
            v: ::rust_layout_equivalent_other_crate::ExternalRustType,
        ) -> ::rust_layout_equivalent_other_crate::ExternalRustType {
            let mut v = ::core::mem::MaybeUninit::new(v);
            unsafe {
                let mut __crubit_return = ::core::mem::MaybeUninit::<
                    ::rust_layout_equivalent_other_crate::ExternalRustType,
                >::uninit();
                crate::detail::__rust_thunk___ZN6crubit4test14DoubleExternalENS0_15ExternalCppTypeE(
                    &raw mut __crubit_return as *mut ::core::ffi::c_void,
                    v.as_mut_ptr(),
                );
                __crubit_return.assume_init()
            }
        }
    }
}

// namespace crubit::test

// Type bindings for crubit::test::TaggedValue<int, 10> suppressed due to being mapped to an existing Rust type (crate::TaggedValue<::ffi_11::c_int,10i64>)

#[path = "rs_bindings_from_cc/test/annotations/rust_layout_equivalent_rust_types.rs"]
mod __crubit_mod_0;
#[allow(unused_imports)]
pub use __crubit_mod_0::*;

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test17AddOneToSameCrateENS0_16SameCrateCppTypeE(
            __return: *mut ::core::ffi::c_void,
            v: *mut crate::SameCrateRustType,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test6AddTagENS0_11TaggedValueIiLl10EEE(
            v: *mut crate::TaggedValue<::ffi_11::c_int, 10i64>,
        ) -> i64;
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test14DoubleExternalENS0_15ExternalCppTypeE(
            __return: *mut ::core::ffi::c_void,
            v: *mut ::rust_layout_equivalent_other_crate::ExternalRustType,
        );
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::SameCrateRustType>() == 4);
    assert!(::core::mem::align_of::<crate::SameCrateRustType>() == 4);
    assert!(::core::mem::size_of::<::rust_layout_equivalent_other_crate::ExternalRustType>() == 4);
    assert!(::core::mem::align_of::<::rust_layout_equivalent_other_crate::ExternalRustType>() == 4);
    assert!(::core::mem::size_of::<crate::TaggedValue<::ffi_11::c_int, 10i64>>() == 4);
    assert!(::core::mem::align_of::<crate::TaggedValue<::ffi_11::c_int, 10i64>>() == 4);
};
