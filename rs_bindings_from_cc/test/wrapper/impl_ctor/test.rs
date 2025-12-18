// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::{expect_eq, gtest};
use impl_ctor::{create, read};

#[gtest]
fn test_wrapped_library() {
    let mut x = create();
    expect_eq!(x.value, 42);

    // Writing a public field requires pin-projecting to the field,
    // but since `c_int` is `Unpin`, the field itself requires no
    // more pin stuff.
    *x.as_mut().project_pin().value = 100;

    expect_eq!(x.value, 100);
    expect_eq!(read(x), 100);
}
