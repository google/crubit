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

#[derive(Default, Copy, Clone)]
pub struct Copyable {
    pub field: u8,
}

impl Copyable {
    pub fn from_byte(byte: u8) -> Self {
        Self { field: byte }
    }

    /// Typically, `self`-by-value methods turn into `&&`-qualified methods in C++.
    /// However, for `Copy` types, there's no need to consume the argument, as it will be copied
    /// regardless.
    pub fn consume_self(mut self) -> u8 {
        let old = self.field;
        // Write a value to be sure that we're writing into a copy rather than the original.
        self.field = 84;
        old
    }
}
