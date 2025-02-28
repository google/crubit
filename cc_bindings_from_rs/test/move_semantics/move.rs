// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! This crate is used as a test input for `cc_bindings_from_rs` and the
//! generated C++ bindings are then tested via `move_test.cc`.

#[derive(Default)]
pub struct Foo {
    buf: Box<u8>,
}

impl Foo {
    pub fn from_byte(byte: u8) -> Self {
        Self { buf: Box::new(byte) }
    }

    pub fn read_byte(&self) -> u8 {
        *self.buf
    }

    pub fn into_byte(self) -> u8 {
        *self.buf
    }
}

pub fn consume_foo(_foo: Foo) {}
