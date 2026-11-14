// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

pub fn return_vec_string() -> Vec<String> {
    vec!["foo".to_string(), "bar".to_string()]
}

pub fn return_vec_str_ref() -> Vec<&'static str> {
    vec!["foo", "bar"]
}

pub fn return_slice_str_ref() -> &'static [&'static str] {
    &["foo", "bar"]
}
