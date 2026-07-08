// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use declval::declval;
use googletest::prelude::*;

struct NoDefaultOrClone {
    _x: i32,
}

trait MyTrait {
    fn get_value(&self) -> i32;
}

impl MyTrait for NoDefaultOrClone {
    fn get_value(&self) -> i32 {
        42
    }
}

fn requires_my_trait<T: MyTrait>(val: T) -> i32 {
    val.get_value()
}

#[gtest]
fn test_unevaluated_type_inference() {
    fn inner<F>(_: impl FnOnce() -> F) -> core::any::TypeId
    where
        F: 'static,
    {
        core::any::TypeId::of::<F>()
    }

    // Notice we pass || requires_my_trait(declval::<NoDefaultOrClone>()),
    // but inner never calls the closure. `declval` provides exact type `NoDefaultOrClone`,
    // passing the trait bound `T: MyTrait` cleanly even though `NoDefaultOrClone` cannot be instantiated!
    let type_id = inner(|| requires_my_trait(declval::<NoDefaultOrClone>()));
    assert_eq!(type_id, core::any::TypeId::of::<i32>());
}

#[gtest]
#[should_panic(expected = "`declval` must only be used in unevaluated contexts")]
fn test_declval_panics_at_runtime() {
    let _val = declval::<u32>();
}
