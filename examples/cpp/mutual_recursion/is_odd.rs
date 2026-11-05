// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! The Rust implementation of `parity::IsOdd`, from `parity.h`.

/// Returns true if `n` is odd. Requires `n >= 0`.
///
/// `parity::IsOdd` (in `parity.cc`) calls this through its C++ bindings, and
/// this calls back into the C++ `parity::IsEven`.
pub fn is_odd(n: i32) -> bool {
    if n == 0 {
        return false;
    }
    parity_decls::parity::IsEven(n - 1)
}
