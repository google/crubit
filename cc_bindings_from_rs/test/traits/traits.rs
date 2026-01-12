// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use crubit_annotate::must_bind;

#[must_bind]
pub trait MyTrait {
    fn do_something(&self) -> i32;
}

trait DoesNotBind {
    fn do_something(&self) -> i32;
}

pub trait GenericTrait<T> {
    fn generic_do_something(&self) -> T;
}

pub trait LifetimeTrait<'a> {
    fn trait_do_something(&'a self) -> &'a i32;

    fn function_do_something<'b>(&'b self) -> &'b i32;
}

pub struct MyStruct {
    x: i32,
}

impl MyTrait for MyStruct {
    fn do_something(&self) -> i32 {
        self.x
    }
}

impl DoesNotBind for MyStruct {
    fn do_something(&self) -> i32 {
        self.x
    }
}

impl GenericTrait<i32> for MyStruct {
    fn generic_do_something(&self) -> i32 {
        self.x
    }
}

impl<'a> LifetimeTrait<'a> for MyStruct {
    fn trait_do_something(&'a self) -> &'a i32 {
        &self.x
    }

    fn function_do_something<'b>(&'b self) -> &'b i32 {
        &self.x
    }
}
