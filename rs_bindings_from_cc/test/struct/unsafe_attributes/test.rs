// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#![deny(warnings)]

use unsafe_attributes::*;

#[googletest::gtest]
#[deny(unsafe_code)]
fn test_safe_struct() {
    UseSafeStructUnannotated(SafeStructUnannotated::default());
    UseUnsafeStructAnnotatedSafe(UnsafeStructAnnotatedSafe::default());
}

#[googletest::gtest]
#[deny(unused_unsafe)]
fn test_unsafe_struct() {
    unsafe { UseSafeStructAnnotatedUnsafe(SafeStructAnnotatedUnsafe::default()) };
    unsafe { UseUnsafeStructUnannotated(UnsafeStructUnannotated::default()) };
}
