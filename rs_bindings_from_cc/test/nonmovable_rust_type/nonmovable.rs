// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! A Rust type which is not C++ move constructible.
//!
//! `NonMovable` implements neither `Default` nor `Clone`, so `cc_bindings_from_rs`
//! deletes its C++ move constructor (there is no valid moved-from value to leave
//! behind) and gives it a relocating constructor instead.

pub struct NonMovable {
    buf: Box<u8>,
}

impl NonMovable {
    pub fn from_byte(byte: u8) -> Self {
        Self { buf: Box::new(byte) }
    }

    pub fn read_byte(&self) -> u8 {
        *self.buf
    }
}
