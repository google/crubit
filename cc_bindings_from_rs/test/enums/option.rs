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
    pub niche: Option<NonMaxU8>,
    pub nested: Option<Option<NonMaxU8>>,
    pub direct: Option<u8>,
}

impl HasOptions {
    pub fn new(value: u8) -> Self {
        unsafe {
            HasOptions {
                niche: Some(NonMaxU8(value)),
                nested: Some(Some(NonMaxU8(value))),
                direct: Some(value),
            }
        }
    }

    pub fn with_option(value: Option<u8>) -> Self {
        unsafe {
            HasOptions {
                niche: value.map(|v| NonMaxU8(v)),
                nested: Some(value.map(|v| NonMaxU8(v))),
                direct: value,
            }
        }
    }

    pub fn from_ref(value: &Option<u8>) -> Self {
        match value {
            Some(v) => HasOptions::new(*v),
            None => HasOptions::with_none(),
        }
    }

    pub fn with_none() -> Self {
        HasOptions { niche: None, nested: None, direct: None }
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

#[derive(Default)]
pub struct HasDefault {
    pub foo: String,
}

impl HasDefault {
    pub fn new(s: &str) -> Self {
        Self { foo: s.to_string() }
    }

    pub fn get_string_inside_option(&self) -> &str {
        &self.foo
    }
}

pub struct OptDefaultWithDrop {
    pub opt: Option<HasDefault>,
}
impl OptDefaultWithDrop {
    pub fn new(s: &str) -> Self {
        Self { opt: Some(HasDefault { foo: s.to_string() }) }
    }
}

pub struct HasNoDefault {
    pub foo: String,
    pub a: u32,
}
impl HasNoDefault {
    pub fn new(s: &str) -> Self {
        Self { foo: s.to_string(), a: 3033 }
    }
    pub fn get_string_inside_option(&self) -> &str {
        &self.foo
    }
}

pub struct OptNoDefaultWithDrop {
    pub val: Option<HasNoDefault>,
}

impl OptNoDefaultWithDrop {
    pub fn new(s: &str) -> Self {
        Self { val: Some(HasNoDefault { foo: s.to_string(), a: 1045 }) }
    }

    pub fn get_string_inside_option(&self) -> &str {
        self.val.as_ref().unwrap().get_string_inside_option()
    }
}

#[derive(Clone)]
pub struct CloneNoDefault {
    pub val: u8,
}

#[derive(Clone)]
pub struct OptCloneNoDefault {
    pub val: Option<CloneNoDefault>,
}
impl OptCloneNoDefault {
    pub fn new(x: u8) -> Self {
        Self { val: Some(CloneNoDefault { val: x }) }
    }
}

#[derive(Copy, Clone)]
pub struct CopyNoDefault {
    pub val: u8,
}

#[derive(Copy, Clone)]
pub struct OptCopyNoDefault {
    pub val: Option<CopyNoDefault>,
}

impl OptCopyNoDefault {
    pub fn new(x: u8) -> Self {
        Self { val: Some(CopyNoDefault { val: x }) }
    }
}

// 4. Uninhabited type
pub enum UninhabitedEnum {}

pub struct OptUninhabited {
    pub val: Option<UninhabitedEnum>,
}

// 5. Zero sized type
pub struct Unit;

#[derive(Default)]
pub struct OptZst {
    pub val: Option<Unit>,
}

pub fn stringify_len(x: &Option<HasDefault>) -> Option<usize> {
    x.as_ref().map(|y| y.get_string_inside_option().len())
}
