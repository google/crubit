// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:overloads_cc
// Features: experimental, extern_c, supported

#![rustfmt::skip]
#![feature(custom_inner_attributes, register_tool)]
#![allow(stable_features)]
#![no_std]
#![register_tool(__crubit)]
#![allow(improper_ctypes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![deny(warnings)]

// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Error while generating bindings for item 'Overload':
// Cannot generate bindings for overloaded function

// Error while generating bindings for item 'Overload':
// Cannot generate bindings for overloaded function

// Error while generating bindings for item 'UncallableOverload':
// Cannot generate bindings for overloaded function

// Error while generating bindings for item 'UncallableOverload':
// Cannot generate bindings for overloaded function

// Error while generating bindings for item 'Sizeof':
// Class templates are not supported yet

// Error while generating bindings for item 'UncallableOverload':
// Function templates are not supported yet

#[inline(always)]
pub fn AlsoTemplateOverload() {
    unsafe { crate::detail::__rust_thunk___Z20AlsoTemplateOverloadv() }
}

// Error while generating bindings for item 'AlsoTemplateOverload':
// Function templates are not supported yet

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_OVERLOADS_H_

mod detail {
    #[allow(unused_imports)]
    use super::*;
    extern "C" {
        pub(crate) fn __rust_thunk___Z20AlsoTemplateOverloadv();
    }
}

const _: () = assert!(::core::mem::size_of::<Option<&i32>>() == ::core::mem::size_of::<&i32>());
