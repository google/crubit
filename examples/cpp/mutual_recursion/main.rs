// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

fn main() {
    // Rust callers can call the Rust implementation directly. `is_odd` calls
    // the C++ `parity::IsEven`, which calls back into `is_odd`.
    //
    // Nothing else in this binary refers to `parity.cc`, which is why `:parity`
    // needs `alwayslink = True` (see the BUILD file).
    println!("is_odd(7) = {}", is_odd::is_odd(7));
}
