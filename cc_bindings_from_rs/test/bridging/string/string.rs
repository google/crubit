// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

pub fn roundtrip_string(val: cc_std::std::string) -> cc_std::std::string {
    val
}

pub fn create_from_rust() -> cc_std::std::string {
    cc_std::std::string::from("hello world from Rust")
}

pub fn compute_string_length(val: cc_std::std::string) -> usize {
    val.len()
}
