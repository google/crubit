// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::gtest;

#[gtest]
fn test_no_extern_c_known_failure() {
    // See KNOWN_FAILURES.md: Linkage Mismatches for headers without `extern "C"`.
    use no_extern_c_known_failure::no_extern_c_func_unmangled;
    no_extern_c_func_unmangled();
}
