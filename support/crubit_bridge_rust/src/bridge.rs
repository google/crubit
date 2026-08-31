// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Forwarding module to `crubit_support::bridge`.

pub use crubit_support::bridge::*;
pub use crubit_support::{unstable_encode, unstable_return};

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::expect_eq;
    use googletest::gtest;

    #[gtest]
    fn test_forwarding() {
        type Abi = (TransmuteAbi<u8>, TransmuteAbi<u8>);
        let original = (1, 2);
        let value = unsafe {
            internal::decode::<Abi>(
                Abi::default(),
                unstable_encode!(@ (transmute_abi(), transmute_abi()), Abi, original).as_ptr()
                    as *const u8,
            )
        };
        expect_eq!(value, original);
    }
}
