// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
use crubit_annotate::must_bind;

#[derive(Copy, Clone)]
#[must_bind]
pub struct TupleStructOnePublicArg(pub i32);

impl TupleStructOnePublicArg {
    #[must_bind]
    pub fn create(arg: i32) -> Self {
        Self(arg)
    }

    #[must_bind]
    pub fn get_arg(self) -> i32 {
        self.0
    }
}

#[derive(Copy, Clone)]
#[must_bind]
pub struct TupleStructOnePrivateArg(i32);

impl TupleStructOnePrivateArg {
    #[must_bind]
    pub fn create(arg: i32) -> Self {
        Self(arg)
    }

    #[must_bind]
    pub fn get_arg(self) -> i32 {
        self.0
    }
}

#[derive(Copy, Clone)]
#[must_bind]
pub struct TupleStructTwoPublicArgs(pub i32, pub i32);

impl TupleStructTwoPublicArgs {
    #[must_bind]
    pub fn create(first_arg: i32, second_arg: i32) -> Self {
        Self(first_arg, second_arg)
    }

    #[must_bind]
    pub fn get_first_arg(self) -> i32 {
        self.0
    }

    #[must_bind]
    pub fn get_second_arg(self) -> i32 {
        self.1
    }
}

#[derive(Copy, Clone)]
#[must_bind]
pub struct TupleStructTwoPrivateArgs(i32, i32);

impl TupleStructTwoPrivateArgs {
    #[must_bind]
    pub fn create(first_arg: i32, second_arg: i32) -> Self {
        Self(first_arg, second_arg)
    }

    #[must_bind]
    pub fn get_first_arg(self) -> i32 {
        self.0
    }

    #[must_bind]
    pub fn get_second_arg(self) -> i32 {
        self.1
    }
}

#[derive(Copy, Clone)]
#[must_bind]
pub struct TupleStructOnePublicArgOnePrivateArg(pub i32, i32);

impl TupleStructOnePublicArgOnePrivateArg {
    #[must_bind]
    pub fn create(first_arg: i32, second_arg: i32) -> Self {
        Self(first_arg, second_arg)
    }

    #[must_bind]
    pub fn get_second_arg(self) -> i32 {
        self.1
    }
}

#[derive(Default, Copy, Clone)]
#[must_bind]
pub struct TupleStructWithInvalidArgumentType(pub (i32, i32));

impl TupleStructWithInvalidArgumentType {
    #[must_bind]
    pub fn create((first_arg, second_arg): (i32, i32)) -> Self {
        Self((first_arg, second_arg))
    }

    #[must_bind]
    pub fn get_arg(self) -> (i32, i32) {
        self.0
    }
}

#[derive(Default, Copy, Clone)]
#[non_exhaustive]
#[must_bind]
pub struct TupleStructWithNonExhaustiveCtor(pub i32, pub i32);

impl TupleStructWithNonExhaustiveCtor {
    #[must_bind]
    pub fn create(first_arg: i32, second_arg: i32) -> Self {
        Self(first_arg, second_arg)
    }
}

// We wrap our generic inside a newtype, so that we're testing whether or not the type can move and
// not whether or not we support generic types as fields.
pub struct DontMoveMe {
    pub value: Box<i32>,
}

#[must_bind]
pub struct TupleStructWithCppImmovableType(pub i32, pub DontMoveMe);

impl TupleStructWithCppImmovableType {
    #[must_bind]
    pub fn create(first_arg: i32, second_arg: i32) -> Self {
        Self(first_arg, DontMoveMe { value: Box::new(second_arg) })
    }

    #[must_bind]
    pub fn get_first_arg(&self) -> i32 {
        self.0
    }

    #[must_bind]
    pub fn get_second_arg(&self) -> &i32 {
        &self.1.value
    }
}

#[derive(Copy, Clone)]
#[must_bind]
pub struct CopyNoDefault {
    pub value: i32,
}
impl CopyNoDefault {
    #[must_bind]
    pub fn create(value: i32) -> Self {
        Self { value }
    }
}

#[must_bind]
pub struct TupleStructWithNoDefault(pub CopyNoDefault);

#[derive(Default)]
pub struct DefaultNoCopyNoClone {
    pub value: i32,
}

#[must_bind]
pub struct TupleStructWithDefaultNoCopyNoClone(pub DefaultNoCopyNoClone);

#[derive(Clone)]
pub struct CloneNoDefault {
    pub value: Box<i32>,
}

#[must_bind]
pub struct TupleStructWithCloneNoDefault(pub CloneNoDefault);

impl TupleStructWithCloneNoDefault {
    #[must_bind]
    pub fn create(value: i32) -> Self {
        Self(CloneNoDefault { value: Box::new(value) })
    }

    #[must_bind]
    pub fn get_value(&self) -> &i32 {
        &self.0.value
    }
}

#[must_bind]
#[derive(Default, Clone)]
pub struct DefaultAndCloneNoUnpin {
    pub value: i32,
    pub _marker: std::marker::PhantomPinned,
}

#[must_bind]
pub struct TupleStructWithDefaultAndCloneNoUnpin(pub DefaultAndCloneNoUnpin);
impl TupleStructWithDefaultAndCloneNoUnpin {
    #[must_bind]
    pub fn create() -> Self {
        Self(DefaultAndCloneNoUnpin::default())
    }

    #[must_bind]
    pub fn get_arg(&self) -> i32 {
        self.0.value
    }
}
