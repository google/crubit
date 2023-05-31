// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[test]
fn test_round_trip() {
    let value = leaf_rs_lib::wrap(4);
    assert_eq!(middle_cc_lib::crubit::Unwrap(value), 4);

    let value = middle_cc_lib::crubit::Wrap(2);
    assert_eq!(leaf_rs_lib::unwrap(value), 2);
}
