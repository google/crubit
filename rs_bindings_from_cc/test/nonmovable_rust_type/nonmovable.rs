// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! A Rust type which is not C++ move constructible.
//!
//! `NonMovable` implements neither `Default` nor `Clone`, so `cc_bindings_from_rs`
//! deletes its C++ move constructor (there is no valid moved-from value to leave
//! behind) and gives it a relocating constructor instead.

use std::sync::Arc;

pub struct NonMovable {
    /// Shared, so that the creator can observe whether this value is still
    /// alive through `Arc::strong_count`.
    byte: Arc<u8>,
}

impl NonMovable {
    pub fn new(byte: Arc<u8>) -> Self {
        Self { byte }
    }

    pub fn read_byte(&self) -> u8 {
        *self.byte
    }
}
