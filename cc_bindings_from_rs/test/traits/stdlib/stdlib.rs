// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

pub trait MyTrait {
    fn do_something(&self) -> i32;
}

// This should not generate bindings because i32 is from `std`.
impl MyTrait for i32 {
    fn do_something(&self) -> i32 {
        *self + 1
    }
}

// Deriving `Clone` and `Default` to verify via goldens that they don't get
// trait-shaped bindings (since they already get other, idiomatic C++ bindings).
#[derive(Clone, Default)]
pub struct MyStruct {
    pub x: i32,
}

impl MyStruct {
    pub fn new(x: i32) -> Self {
        MyStruct { x }
    }
}

// Implement `From` (and implicitly `Into`) to verify via goldens that they
// don't get trait-shaped bindings (since they already get other, idiomatic C++
// bindings).
impl From<i32> for MyStruct {
    fn from(x: i32) -> Self {
        MyStruct { x }
    }
}

// Implement `Drop` to verify via goldens that it doesn't get trait-shaped
// bindings (since it already gets other, idiomatic C++ bindings - a
// destructor).
impl Drop for MyStruct {
    fn drop(&mut self) {}
}

// `std::iter::Iterator` should get trait-shaped bindings.
impl Iterator for MyStruct {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.x <= 0 {
            None
        } else {
            self.x -= 1;
            Some(self.x)
        }
    }
}
