// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::expect_eq;
use googletest::gtest;
use vec::test;

#[gtest]
fn test_vec_mut_ref() {
    let mut v: Vec<u8> = vec![1, 2, 3];
    test::PushBytes(&mut v);
    expect_eq!(v, vec![1, 2, 3, 4, 5]);
    expect_eq!(test::SumBytes(&v), 15);
}

#[gtest]
fn test_vec_by_value() {
    let v = test::MakeBytes();
    expect_eq!(v, vec![10, 20, 30]);
    expect_eq!(test::ConsumeBytes(v), 60);
}
