// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::prelude::*;

#[gtest]
fn test_wrapped_library() {
    let mut x = impl_ctor::create();
    assert_eq!(x.value, 42);

    // Writing a public field requires pin-projecting to the field,
    // but since `c_int` is `Unpin`, the field itself requires no
    // more pin stuff.
    *x.as_mut().project_pin().value = 100;

    assert_eq!(x.value, 100);

    // TODO(b/411467353): don't support move constructors yet, so can't do this
    // assert_eq!(wrapped_library::read(value), 100);
}
