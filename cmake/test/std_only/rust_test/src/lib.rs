// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[unsafe(no_mangle)]
pub extern "C" fn rust_test_main() -> i32 {
    let s = cc_std::std::string_wrapper::from("hi");
    assert_eq!(s.to_str(), Ok("hi"));
    let mut v = cc_std::std::vector::<i32>::new();
    v.push(7);
    assert_eq!(v.len(), 1);
    0
}
