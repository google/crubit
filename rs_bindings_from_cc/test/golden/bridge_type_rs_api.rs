// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:bridge_type_cc
// Features: experimental, supported

#![rustfmt::skip]
#![feature(custom_inner_attributes)]
#![allow(stable_features)]
#![no_std]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code)]
#![deny(warnings)]

#[inline(always)]
pub fn ReturnCppStruct() -> RustStruct {
    unsafe {
        let mut __return = ::core::mem::MaybeUninit::<RustStruct>::uninit();
        crate::detail::__rust_thunk___Z15ReturnCppStructv(&mut __return);
        __return.assume_init()
    }
}

#[inline(always)]
pub fn TakeCppStruct(mut __param_0: RustStruct) {
    unsafe { crate::detail::__rust_thunk___Z13TakeCppStruct9CppStruct(&mut __param_0) }
}

// Error while generating bindings for item 'TakeCppStructByPtr':
// Failed to format type of parameter 0: Bridging types are not supported as pointee/referent types.

// Error while generating bindings for item 'ReturnCppStructByPtr':
// Failed to format return type: Bridging types are not supported as pointee/referent types.

mod detail {
    #[allow(unused_imports)]
    use super::*;
    extern "C" {
        pub(crate) fn __rust_thunk___Z15ReturnCppStructv(
            __return: &mut ::core::mem::MaybeUninit<RustStruct>,
        );
        pub(crate) fn __rust_thunk___Z13TakeCppStruct9CppStruct(__param_0: &mut RustStruct);
    }
}
