// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

pub fn roundtrip_string(val: cc_std::std::string_wrapper) -> cc_std::std::string_wrapper {
    val
}

pub fn create_from_rust() -> cc_std::std::string_wrapper {
    cc_std::std::string_wrapper::from("hello world from Rust")
}

pub fn compute_string_length(val: cc_std::std::string_wrapper) -> usize {
    val.len()
}
