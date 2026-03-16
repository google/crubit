// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use ctor::emplace;
use googletest::{expect_false, expect_true, gtest};
use memcpy_movable::crubit::test::{
    AcceptsMemcpyMovable, AcceptsNonMemcpyMovable, MemcpyMovableClass, NonMemcpyMovableClass,
    ReturnsMemcpyMovable, ReturnsNonMemcpyMovable,
};
use std::any::{Any, TypeId};
use std::pin::Pin;

fn type_is<ExpectedType: 'static>(expr: impl Any) -> bool {
    TypeId::of::<ExpectedType>() == expr.type_id()
}

#[gtest]
fn only_memcpy_movable_class_returned_by_value() {
    expect_true!(type_is::<MemcpyMovableClass>(ReturnsMemcpyMovable()));
    // Rather than returning a NonMemcpyMovableClass, this returns a Ctor which produces one.
    expect_false!(type_is::<NonMemcpyMovableClass>(ReturnsNonMemcpyMovable()));

    let _: Pin<&mut MemcpyMovableClass> = emplace!(ReturnsMemcpyMovable());
    let _: Pin<&mut NonMemcpyMovableClass> = emplace!(ReturnsNonMemcpyMovable());

    AcceptsMemcpyMovable(ReturnsMemcpyMovable());
    AcceptsNonMemcpyMovable(ReturnsNonMemcpyMovable());
}
