// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:bridge_type_cc

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes)]
#![allow(stable_features)]
#![no_std]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

#[inline(always)]
pub fn ReturnCppStruct() -> crate::RustStruct {
    unsafe {
        let mut __return = ::core::mem::MaybeUninit::<crate::RustStruct>::uninit();
        crate::detail::__rust_thunk___Z15ReturnCppStructv(
            &raw mut __return as *mut ::core::ffi::c_void,
        );
        __return.assume_init()
    }
}

#[inline(always)]
pub fn TakeCppStruct(mut __param_0: crate::RustStruct) {
    unsafe { crate::detail::__rust_thunk___Z13TakeCppStruct9CppStruct(&mut __param_0) }
}

// Error while generating bindings for function 'TakeCppStructByPtr':
// Can't generate bindings for TakeCppStructByPtr, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:bridge_type_cc needs [//features:wrapper] for TakeCppStructByPtr (the type of __param_0 (parameter #0): error: Bridging types are not supported as pointee/referent types.)

// Error while generating bindings for function 'ReturnCppStructByPtr':
// Can't generate bindings for ReturnCppStructByPtr, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:bridge_type_cc needs [//features:wrapper] for ReturnCppStructByPtr (return type: error: Bridging types are not supported as pointee/referent types.)

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___Z15ReturnCppStructv(__return: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___Z13TakeCppStruct9CppStruct(
            __param_0: &mut crate::RustStruct,
        );
    }
}
