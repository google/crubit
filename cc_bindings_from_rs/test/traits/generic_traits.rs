// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

pub trait TraitWithGeneric<T> {
    fn foo(&self, t: T) -> T;
}

pub struct StructGeneric {
    pub x: i32,
}

impl StructGeneric {
    pub fn new(x: i32) -> Self {
        Self { x }
    }
}

// This implementation specifies every type argument (i32), so it should receive bindings.
impl TraitWithGeneric<i32> for StructGeneric {
    fn foo(&self, t: i32) -> i32 {
        self.x + t
    }
}

pub trait TraitWithTwoGenerics<T, U> {
    fn bar(&self, t: T, u: U) -> i32;
}

// Specifies both -> should bind.
impl TraitWithTwoGenerics<i32, i32> for StructGeneric {
    fn bar(&self, t: i32, u: i32) -> i32 {
        self.x + t + u
    }
}

pub struct AnotherStruct {
    pub y: i32,
}

// Specifies only one -> should NOT bind.
impl<U> TraitWithTwoGenerics<i32, U> for AnotherStruct {
    fn bar(&self, t: i32, _u: U) -> i32 {
        self.y + t
    }
}

// Trait with a constant parameter should NOT receive bindings.
pub trait TraitWithConst<const N: usize> {
    fn baz(&self) -> usize {
        N
    }
}

impl TraitWithConst<42> for StructGeneric {}
