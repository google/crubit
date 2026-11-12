// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::gtest;
#[gtest]
fn test_qualifiers_known_failure() {
    // See KNOWN_FAILURES.md: C99 `restrict` fails parsing in C++ mode.
    use qualifiers_known_failure::process_restrict;
    let p: *mut core::ffi::c_int = core::ptr::null_mut();
    unsafe { process_restrict(p) };
}
