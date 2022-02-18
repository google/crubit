// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for C++ target
// //rs_bindings_from_cc/test/golden:lifetimes_cc
#![rustfmt::skip]
#![feature(custom_inner_attributes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub type __builtin_ms_va_list = *mut u8;

#[inline(always)]
pub fn AddHook(__param_0: Option<extern "C" fn()>) {
    unsafe { crate::detail::__rust_thunk___Z7AddHookPFvvE(__param_0) }
}

pub type FunctionPointer = Option<extern "C" fn()>;

#[inline(always)]
pub fn AddHookWithTypedef(hook: Option<extern "C" fn()>) {
    unsafe { crate::detail::__rust_thunk___Z18AddHookWithTypedefPFvvE(hook) }
}

// rs_bindings_from_cc/test/golden/lifetimes.h;l=9
// Error while generating bindings for item 'AddAnotherHook':
// Parameter #0 is not supported: Unsupported type 'void (&)(void)'

// rs_bindings_from_cc/test/golden/lifetimes.h;l=11
// Error while generating bindings for item 'FunctionReference':
// Unsupported type 'void (&)(void)'

// rs_bindings_from_cc/test/golden/lifetimes.h;l=12
// Error while generating bindings for item 'AddAnotherHookWithTypedef':
// Parameter #0 is not supported: Unsupported type 'FunctionReference'

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
        #[link_name = "_Z7AddHookPFvvE"]
        pub(crate) fn __rust_thunk___Z7AddHookPFvvE(__param_0: Option<extern "C" fn()>);
        #[link_name = "_Z18AddHookWithTypedefPFvvE"]
        pub(crate) fn __rust_thunk___Z18AddHookWithTypedefPFvvE(hook: Option<extern "C" fn()>);
        #[link_name = "_Z12ConsumeArrayPi"]
        pub(crate) fn __rust_thunk___Z12ConsumeArrayPi(pair: *mut i32);
        #[link_name = "_Z23ConsumeArrayWithTypedefPi"]
        pub(crate) fn __rust_thunk___Z23ConsumeArrayWithTypedefPi(__param_0: *mut i32);
    }
}

const _: () = assert!(std::mem::size_of::<Option<&i32>>() == std::mem::size_of::<&i32>());
