// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use crubit_annotate::must_bind;

#[must_bind]
pub trait MyTrait {
    fn do_something(&self) -> i32;

    fn consume_self(self) -> i32;

    fn return_self(&self) -> &Self;

    fn no_self() -> i32;

    fn take_and_return_other_types(&self, x: Foo) -> (i32, i32);
}

pub trait DifferentTraitSameName {
    fn do_something(&self) -> i32;
}

#[derive(Default, Clone, Copy)]
pub struct Foo {
    a: (i32, i32),
}
impl Foo {
    pub fn new(x: i32, y: i32) -> Self {
        Self { a: (x, y) }
    }
}

#[allow(dead_code)]
trait DoesNotBind {
    fn do_something(&self) -> i32;
}

pub trait LifetimeTrait<'a> {
    fn trait_do_something(&'a self) -> &'a i32;

    fn function_do_something(&self) -> &i32;
}

#[derive(Default, Clone, Copy)]
pub struct MyStruct {
    x: i32,
}

impl MyStruct {
    pub fn new(x: i32) -> Self {
        Self { x }
    }
}

impl MyTrait for MyStruct {
    fn do_something(&self) -> i32 {
        self.x
    }

    fn consume_self(self) -> i32 {
        self.x
    }

    fn return_self(&self) -> &Self {
        self
    }

    fn no_self() -> i32 {
        7198
    }

    fn take_and_return_other_types(&self, x: Foo) -> (i32, i32) {
        x.a
    }
}

impl DifferentTraitSameName for MyStruct {
    fn do_something(&self) -> i32 {
        self.x
    }
}

impl DoesNotBind for MyStruct {
    fn do_something(&self) -> i32 {
        self.x
    }
}

pub struct LifetimeStruct<'a> {
    x: &'a i32,
}

impl<'a> LifetimeTrait<'a> for LifetimeStruct<'a> {
    fn trait_do_something(&'a self) -> &'a i32 {
        self.x
    }

    fn function_do_something(&self) -> &i32 {
        self.x
    }
}

#[derive(Default, Clone, Copy)]
pub struct MyStruct2 {
    y: i32,
}

impl MyTrait for MyStruct2 {
    fn do_something(&self) -> i32 {
        self.y
    }

    fn consume_self(self) -> i32 {
        self.y
    }

    fn return_self(&self) -> &Self {
        self
    }

    fn no_self() -> i32 {
        4356
    }

    fn take_and_return_other_types(&self, x: Foo) -> (i32, i32) {
        x.a
    }
}

pub trait AssociatedTypeTrait {
    type MyAssocType;

    type UnsupportedAssocType;

    fn get_my_assoc_type(&self) -> Self::MyAssocType;

    fn get_unsupported_assoc_type(&self) -> Self::UnsupportedAssocType;
}

#[derive(Default, Clone)]
pub struct AssociatedTypeStruct {
    b: i32,
    a: String,
}

impl AssociatedTypeTrait for AssociatedTypeStruct {
    type MyAssocType = i32;
    fn get_my_assoc_type(&self) -> i32 {
        self.b
    }

    // These should not receive bindings because String is unsupported.
    type UnsupportedAssocType = String;
    fn get_unsupported_assoc_type(&self) -> String {
        self.a.clone()
    }
}

pub trait TraitWithAssociatedConst {
    const CONST_INT: i32;

    const CONST_STRUCT: Self;
}

#[derive(Default, Clone, Copy)]
pub struct StructWithAssociatedConst {
    pub x: i32,
}

impl TraitWithAssociatedConst for StructWithAssociatedConst {
    const CONST_INT: i32 = 10;

    const CONST_STRUCT: Self = Self { x: 10 };
}
