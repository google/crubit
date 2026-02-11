// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#![feature(rustc_attrs)]

#[rustc_layout_scalar_valid_range_end(250)]
#[derive(Debug)]
pub struct NonMaxU8(u8);

impl NonMaxU8 {
    pub fn value(&self) -> u8 {
        self.0
    }
}

pub struct HasOptions {
    pub a: Option<NonMaxU8>,
    pub b: Option<Option<NonMaxU8>>,
    pub c: Option<u8>,
}

impl HasOptions {
    pub fn new(value: u8) -> Self {
        unsafe {
            HasOptions { a: Some(NonMaxU8(value)), b: Some(Some(NonMaxU8(value))), c: Some(value) }
        }
    }

    pub fn with_option(value: Option<u8>) -> Self {
        unsafe {
            HasOptions {
                a: value.map(|v| NonMaxU8(v)),
                b: Some(value.map(|v| NonMaxU8(v))),
                c: value,
            }
        }
    }

    pub fn with_none() -> Self {
        HasOptions { a: None, b: None, c: None }
    }
}

pub struct HasHasOptions {
    pub me: Option<HasOptions>,
}

impl HasHasOptions {
    pub fn new(value: u8) -> Self {
        HasHasOptions { me: Some(HasOptions::new(value)) }
    }
}
