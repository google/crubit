// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

pub fn roundtrip_string_in_rs_namespace(
    val: cc_std::std::string_wrapper,
) -> cc_std::std::string_wrapper {
    val
}
