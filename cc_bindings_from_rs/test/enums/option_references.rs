// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Tests `Option<&T>` and `Option<&mut T>` with the `option_references` feature, which maps them
//! to `rs::Option<T const&>` and `rs::Option<T&>` in C++.

#[derive(Clone, Copy, Default)]
pub struct Value {
    pub x: i32,
}

/// Returns `v` if `v.x` is positive.
pub fn if_positive(v: &Value) -> Option<&Value> {
    (v.x > 0).then_some(v)
}

/// Returns `v` if `v.x` is positive.
pub fn if_positive_mut(v: &mut Value) -> Option<&mut Value> {
    if v.x > 0 {
        Some(v)
    } else {
        None
    }
}

/// `Option<*const Value>` used to have the same C++ type as `Option<&Value>`, even though their
/// layouts differ (b/575916406).
pub fn as_ptr(v: &Value) -> Option<*const Value> {
    Some(v as *const Value)
}

pub fn x_or(v: Option<&Value>, default: i32) -> i32 {
    v.map_or(default, |v| v.x)
}

pub fn increment(v: Option<&mut Value>) {
    if let Some(v) = v {
        v.x += 1;
    }
}

pub fn same_value(a: Option<&Value>, b: Option<&Value>) -> bool {
    match (a, b) {
        (Some(a), Some(b)) => core::ptr::eq(a, b),
        (None, None) => true,
        _ => false,
    }
}

#[derive(Clone, Copy, Default)]
pub struct Values {
    values: [i32; 3],
}

impl Values {
    pub fn new(a: i32, b: i32, c: i32) -> Values {
        Values { values: [a, b, c] }
    }

    pub fn get(&self, index: usize) -> Option<&i32> {
        self.values.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut i32> {
        self.values.get_mut(index)
    }

    pub fn iter(&self) -> ValuesIter<'_> {
        ValuesIter { values: &self.values }
    }
}

pub struct ValuesIter<'a> {
    values: &'a [i32],
}

impl<'a> Iterator for ValuesIter<'a> {
    type Item = &'a i32;
    fn next(&mut self) -> Option<&'a i32> {
        self.values.split_off_first()
    }
}
