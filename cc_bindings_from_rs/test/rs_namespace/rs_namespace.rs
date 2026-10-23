// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! This crate is used as a test input for `cc_bindings_from_rs`. Its bindings
//! live in a C++ namespace containing an inner `rs` namespace (`test_ns::rs`),
//! so the generated bindings must refer to Crubit support types (`StrRef`,
//! `SliceRef`, `Tuple`, `Option`, ...) as `::rs::...` rather than `rs::...`.

pub fn str_len(s: &str) -> usize {
    s.len()
}

pub fn identity_str(s: &'static str) -> &'static str {
    s
}

pub fn slice_sum(s: &[i32]) -> i32 {
    s.iter().sum()
}

pub fn swap_tuple(t: (i32, i32)) -> (i32, i32) {
    (t.1, t.0)
}

pub fn first_of_tuple_ref(t: &(i32, i32)) -> i32 {
    t.0
}

pub fn double_option(o: Option<i32>) -> Option<i32> {
    o.map(|x| x * 2)
}

pub fn option_ref_or_zero(o: &Option<i32>) -> i32 {
    o.unwrap_or(0)
}
