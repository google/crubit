// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[derive(Copy, Clone)]
pub struct TupleStructOnePublicArg(pub i32);

impl TupleStructOnePublicArg {
    pub fn create(arg: i32) -> Self {
        Self(arg)
    }
    pub fn get_arg(self) -> i32 {
        self.0
    }
}

#[derive(Copy, Clone)]
pub struct TupleStructOnePrivateArg(i32);

impl TupleStructOnePrivateArg {
    pub fn create(arg: i32) -> Self {
        Self(arg)
    }
    pub fn get_arg(self) -> i32 {
        self.0
    }
}

#[derive(Copy, Clone)]
pub struct TupleStructTwoPublicArgs(pub i32, pub i32);

impl TupleStructTwoPublicArgs {
    pub fn create(first_arg: i32, second_arg: i32) -> Self {
        Self(first_arg, second_arg)
    }

    pub fn get_first_arg(self) -> i32 {
        self.0
    }

    pub fn get_second_arg(self) -> i32 {
        self.1
    }
}

#[derive(Copy, Clone)]
pub struct TupleStructTwoPrivateArgs(i32, i32);

impl TupleStructTwoPrivateArgs {
    pub fn create(first_arg: i32, second_arg: i32) -> Self {
        Self(first_arg, second_arg)
    }

    pub fn get_first_arg(self) -> i32 {
        self.0
    }

    pub fn get_second_arg(self) -> i32 {
        self.1
    }
}
