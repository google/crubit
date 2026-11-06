// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Without the `option_references` Crubit feature, `Option<&T>` and `Option<*const T>` would both
//! be `rs::Option<T const*>` in C++, even though they have different layouts. Crubit only binds the
//! former, so that the generated header compiles (b/575916406).

pub fn by_ref(value: &i32) -> Option<&i32> {
    Some(value)
}

pub fn by_ptr(value: &i32) -> Option<*const i32> {
    Some(value)
}

#[derive(Clone, Copy, Default)]
pub struct Value {
    pub value: i32,
}

impl Value {
    pub fn get(&self) -> Option<&i32> {
        Some(&self.value)
    }

    pub fn get_ptr(&self) -> Option<*const i32> {
        Some(&self.value)
    }
}
