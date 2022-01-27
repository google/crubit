#![rustfmt::skip]
// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#![feature(custom_inner_attributes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub type __builtin_ms_va_list = *mut u8;

// rs_bindings_from_cc/test/golden/lifetimes.h;l=4
// Error while generating bindings for item 'AddHook':
// Parameter type 'void (*)(void)' is not supported

// rs_bindings_from_cc/test/golden/lifetimes.h;l=6
// Error while generating bindings for item 'FunctionPointer':
// Unsupported type 'void (*)(void)'

// rs_bindings_from_cc/test/golden/lifetimes.h;l=7
// Error while generating bindings for item 'AddHookWithTypedef':
// Parameter type 'FunctionPointer' is not supported

// rs_bindings_from_cc/test/golden/lifetimes.h;l=9
// Error while generating bindings for item 'AddAnotherHook':
// Parameter type 'void (&)(void)' is not supported

// rs_bindings_from_cc/test/golden/lifetimes.h;l=11
// Error while generating bindings for item 'FunctionReference':
// Unsupported type 'void (&)(void)'

// rs_bindings_from_cc/test/golden/lifetimes.h;l=12
// Error while generating bindings for item 'AddAnotherHookWithTypedef':
// Parameter type 'FunctionReference' is not supported

#[inline(always)]
pub unsafe fn ConsumeArray(pair: *mut i32) {
    crate::detail::__rust_thunk___Z12ConsumeArrayPi(pair)
}

// rs_bindings_from_cc/test/golden/lifetimes.h;l=16
// Error while generating bindings for item 'Arr':
// Unsupported type 'int[2]'

#[inline(always)]
pub unsafe fn ConsumeArrayWithTypedef(__param_0: *mut i32) {
    crate::detail::__rust_thunk___Z23ConsumeArrayWithTypedefPi(__param_0)
}

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_LIFETIMES_H_

mod detail {
    #[allow(unused_imports)]
    use super::*;
    extern "C" {
        #[link_name = "_Z12ConsumeArrayPi"]
        pub(crate) fn __rust_thunk___Z12ConsumeArrayPi(pair: *mut i32);
        #[link_name = "_Z23ConsumeArrayWithTypedefPi"]
        pub(crate) fn __rust_thunk___Z23ConsumeArrayWithTypedefPi(__param_0: *mut i32);
    }
}

const _: () = assert!(std::mem::size_of::<Option<&i32>>() == std::mem::size_of::<&i32>());
