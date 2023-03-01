// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:overloads_cc

#![rustfmt::skip]
#![feature(custom_inner_attributes)]
#![allow(stable_features)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![deny(warnings)]

// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Generated from: rs_bindings_from_cc/test/golden/overloads.h;l=8
// Error while generating bindings for item 'Overload':
// Cannot generate bindings for overloaded function

// Generated from: rs_bindings_from_cc/test/golden/overloads.h;l=9
// Error while generating bindings for item 'Overload':
// Cannot generate bindings for overloaded function

// Generated from: rs_bindings_from_cc/test/golden/overloads.h;l=14
// Error while generating bindings for item 'UncallableOverload':
// Cannot generate bindings for overloaded function

// Generated from: rs_bindings_from_cc/test/golden/overloads.h;l=17
// Error while generating bindings for item 'UncallableOverload':
// Cannot generate bindings for overloaded function

// Generated from: rs_bindings_from_cc/test/golden/overloads.h;l=19
// Error while generating bindings for item 'Sizeof':
// Class templates are not supported yet

// Generated from: rs_bindings_from_cc/test/golden/overloads.h;l=28
// Error while generating bindings for item 'UncallableOverload':
// Function templates are not supported yet

/// Generated from: rs_bindings_from_cc/test/golden/overloads.h;l=31
#[inline(always)]
pub fn AlsoTemplateOverload() {
    unsafe { crate::detail::__rust_thunk___Z20AlsoTemplateOverloadv() }
}

// Generated from: rs_bindings_from_cc/test/golden/overloads.h;l=32
// Error while generating bindings for item 'AlsoTemplateOverload':
// Function templates are not supported yet

// THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_OVERLOADS_H_

mod detail {
    #[allow(unused_imports)]
    use super::*;
    extern "C" {
        pub(crate) fn __rust_thunk___Z20AlsoTemplateOverloadv();
    }
}

const _: () = assert!(::core::mem::size_of::<Option<&i32>>() == ::core::mem::size_of::<&i32>());
