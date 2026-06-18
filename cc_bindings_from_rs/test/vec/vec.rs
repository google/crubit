// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

pub struct StructWithVec {
    pub v: Vec<i32>,
}

impl StructWithVec {
    pub fn new(val: i32) -> Self {
        StructWithVec { v: vec![val, val * 2, val * 3] }
    }
}

pub fn return_vec() -> Vec<i32> {
    vec![1, 2, 3]
}

pub fn take_vec(v: Vec<i32>) -> i32 {
    v.iter().sum()
}

pub fn return_u8_vec() -> Vec<u8> {
    b"Hello".to_vec()
}
