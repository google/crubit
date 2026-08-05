// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use derived_debuggables::{PrivatelyDerived, PubliclyDerived};
use googletest::{expect_eq, gtest};

#[gtest]
fn test_publicly_derived() {
    expect_eq!(format!("{:?}", PubliclyDerived::default()), "PubliclyDerived { : Base, .. }");
}

#[gtest]
fn test_privately_derived() {
    expect_eq!(format!("{:?}", PrivatelyDerived::default()), "PrivatelyDerived { .. }");
}
