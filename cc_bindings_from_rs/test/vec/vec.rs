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

pub fn drop_vec(_v: Vec<i32>) {}

pub fn return_grown_vec() -> Vec<i32> {
    let mut v = Vec::with_capacity(10);
    v.push(10);
    v.push(20);
    v.push(30);
    v
}

#[derive(Default)]
pub struct RustVecOwner {
    v: Vec<i32>,
}

impl RustVecOwner {
    pub fn new() -> Self {
        RustVecOwner { v: Vec::new() }
    }
    pub fn get_mut_vec(&mut self) -> &mut Vec<i32> {
        &mut self.v
    }
    pub fn get_len(&self) -> usize {
        self.v.len()
    }
    pub fn get_element(&self, index: usize) -> i32 {
        self.v[index]
    }
}

pub fn rust_add_elements(v: &mut Vec<i32>) {
    v.push(100);
    v.push(200);
}
