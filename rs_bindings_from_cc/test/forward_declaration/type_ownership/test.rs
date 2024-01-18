// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[test]
fn test_build() {
    // TODO(b/318690257): Currently, the build fails with:
    // error: "the size for values of type `forward_declare::Unsized` cannot be
    // known at compilation time"
    // "help: within `Incomplete<Symbol<...>, ...>`, the trait `Sized` is not
    // implemented for `forward_declare::Unsized`""
}
