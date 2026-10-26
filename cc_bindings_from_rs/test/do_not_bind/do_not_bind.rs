// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Coverage for `#[crubit_annotate::do_not_bind]` across the item kinds it
//! claims to support. Each suppressed item is paired with an unsuppressed
//! control so the golden distinguishes "suppressed" from "never bound".

pub struct Struct {
    pub value: i32,
}

#[derive(Default)]
#[crubit_annotate::do_not_bind]
pub struct SuppressedStruct {
    pub value: i32,
}

impl SuppressedStruct {
    pub fn method(&self) -> i32 {
        self.value
    }
}

impl Trait for SuppressedStruct {
    fn suppressed_trait_method(&self) -> i32 {
        7
    }

    fn bound_trait_method(&self) -> i32 {
        8
    }
}

pub fn fn_taking_suppressed_struct(_: SuppressedStruct) {}

pub trait Trait {
    fn suppressed_trait_method(&self) -> i32;
    fn bound_trait_method(&self) -> i32;
}

#[crubit_annotate::do_not_bind]
pub fn suppressed_free_fn() -> i32 {
    1
}

pub fn bound_free_fn() -> i32 {
    2
}

impl Struct {
    #[crubit_annotate::do_not_bind]
    pub fn suppressed_inherent_method(&self) -> i32 {
        3
    }

    pub fn bound_inherent_method(&self) -> i32 {
        4
    }
}

impl Trait for Struct {
    #[crubit_annotate::do_not_bind]
    fn suppressed_trait_method(&self) -> i32 {
        5
    }

    fn bound_trait_method(&self) -> i32 {
        6
    }
}
