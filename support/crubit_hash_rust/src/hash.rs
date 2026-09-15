// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Forwarding module to `crubit_support::hash`.

#![no_std]

pub use crubit_support::hash::*;

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::expect_eq;
    use googletest::gtest;

    #[gtest]
    fn test_forwarding() {
        expect_eq!(hash_u64(&42u32), crubit_support::hash::hash_u64(&42u32));
    }
}
