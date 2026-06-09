// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use cref::CRef;
use googletest::gtest;
use struct_fields::*;

// This tests whether Crubit supports template specialization/instantiation in a
// struct field - see b/228868369.
#[gtest]
fn test_template_instantiation_in_return_value_and_parameter_type() {
    // Note that the Rust code below never needs to refer to the
    // mangled name of the Rust struct that the class template
    // specialization/instantiation gets translated to.

    // Class template instantiation used as a type of a public field.
    let s = MyStruct { public_field: 123.into() };
    // SAFETY: s is alive while s.public_field.value() is alive, and no other references to s exist.
    assert_eq!(123, *unsafe { CRef::unchanging(s.public_field.value()) });
}
