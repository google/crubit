// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::prelude::*;

#[gtest]
fn test_mutable_field() {
    let s = mutable_fields::SomeStruct::default();

    use std::any::{Any, TypeId};
    assert!(
        s.mutable_field.type_id() == TypeId::of::<std::cell::Cell<std::ffi::c_int>>(),
        "mutable fields should be in an Cell, but instead got: {:?} ({})",
        s.mutable_field,
        std::any::type_name_of_val(&s.mutable_field),
    );
}
