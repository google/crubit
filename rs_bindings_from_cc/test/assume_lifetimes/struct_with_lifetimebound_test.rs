// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use cref::CRef;
use ctor::{emplace, CtorNew};
use googletest::prelude::*;

#[gtest]
fn struct_with_lifetimebound_member_function_test() {
    let test_struct: struct_with_lifetimebound::StructWithLifetimeboundMemberFunction =
        Default::default();
    let _ = test_struct.f();
}

#[gtest]
fn struct_with_lifetimebound_ref_member_function_test() {
    let test_struct: struct_with_lifetimebound::StructWithLifetimeboundRefMemberFunction =
        Default::default();
    let _ = test_struct.f();
}

#[gtest]
fn drop_class_with_lifetimebound_member_function_test() {
    let drop_class =
        emplace!(struct_with_lifetimebound::DropClassWithLifetimeboundMemberFunction::ctor_new(()));
    let _ = drop_class.f();
}

#[gtest]
fn drop_class_with_lifetimebound_ref_member_function_test() {
    let drop_class = emplace!(
        struct_with_lifetimebound::DropClassWithLifetimeboundRefMemberFunction::ctor_new(())
    );
    let _ = drop_class.f();
}

#[gtest]
fn struct_with_lifetimebound_ctor_test() {
    let plain_struct: struct_with_lifetimebound::PlainStruct = Default::default();
    let _test_struct =
        emplace!(struct_with_lifetimebound::StructWithLifetimeboundCtor::ctor_new(plain_struct));
}

#[gtest]
fn struct_with_lifetimebound_ref_ctor_test() {
    let plain_struct: struct_with_lifetimebound::PlainStruct = Default::default();
    let _test_struct = emplace!(
        struct_with_lifetimebound::StructWithLifetimeboundRefCtor::ctor_new(&plain_struct)
    );
}

#[gtest]
fn drop_struct_with_lifetimebound_ctor_test() {
    let plain_struct: struct_with_lifetimebound::PlainStruct = Default::default();
    let _test_struct = emplace!(
        struct_with_lifetimebound::DropStructWithLifetimeboundCtor::ctor_new(plain_struct)
    );
}

#[gtest]
fn drop_struct_with_lifetimebound_ref_ctor_test() {
    let plain_struct: struct_with_lifetimebound::PlainStruct = Default::default();
    let _test_struct = emplace!(
        struct_with_lifetimebound::DropStructWithLifetimeboundRefCtor::ctor_new(&plain_struct)
    );
}

#[gtest]
fn drop_struct_with_ref_ctor_and_ref_member_function() {
    let plain_struct: struct_with_lifetimebound::PlainStruct = Default::default();
    let test_struct =
        emplace!(struct_with_lifetimebound::DropStructWithRefCtorAndRefMemberFunction::ctor_new(
            &plain_struct
        ));
    let _out: CRef<'_, struct_with_lifetimebound::PlainStruct> = test_struct.f();
}

#[gtest]
fn drop_struct_with_ctor_and_ref_member_function() {
    let plain_struct: struct_with_lifetimebound::PlainStruct = Default::default();
    let test_struct = emplace!(
        struct_with_lifetimebound::DropStructWithCtorAndRefMemberFunction::ctor_new(plain_struct)
    );
    let _out: CRef<'_, struct_with_lifetimebound::PlainStruct> = test_struct.f();
}

#[gtest]
fn drop_struct_with_ref_ctor_and_member_function() {
    let plain_struct: struct_with_lifetimebound::PlainStruct = Default::default();
    let test_struct = emplace!(
        struct_with_lifetimebound::DropStructWithRefCtorAndMemberFunction::ctor_new(&plain_struct)
    );
    let _out: struct_with_lifetimebound::PlainStruct = test_struct.f();
}

#[gtest]
fn drop_struct_with_ctor_and_member_function() {
    let plain_struct: struct_with_lifetimebound::PlainStruct = Default::default();
    let test_struct = emplace!(
        struct_with_lifetimebound::DropStructWithCtorAndMemberFunction::ctor_new(plain_struct)
    );
    let _out: struct_with_lifetimebound::PlainStruct = test_struct.f();
}
