// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[cfg(feature = "required_feature")]
#[unsafe(no_mangle)]
pub extern "C" fn foo() -> i32 {
    42
}

#[cfg(not(feature = "required_feature"))]
const _: () = "this is a type error, because required_feature was unset";
