// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

fn main() {
    // Can pass this to functions accepting a `Position`, and so on.
    let _ = example_lib::Position { x: 1, y: 2 };
}
