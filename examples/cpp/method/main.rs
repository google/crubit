// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use example_lib::foo::Bar;

fn main() {
    unsafe {
        let mut x = Bar { x: 1 };
        Bar::MyMethod(&mut x as *mut Bar);
    }
}
