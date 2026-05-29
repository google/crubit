// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use crubit_annotate::{cpp_layout_equivalent, cpp_specialization, must_bind};

#[cpp_layout_equivalent(
    cpp_type = "crubit::test::MyOptional<{T}>",
    include_path = "cc_bindings_from_rs/test/bridging/cc_generics.h"
)]
#[repr(C)]
pub struct MyOptional<T> {
    pub has_value: bool,
    pub value: T,
}

impl MyOptional<i32> {
    pub fn new_i32_in_impl_block(x: i32) -> MyOptional<i32> {
        MyOptional { has_value: true, value: x }
    }
}

#[must_bind]
pub fn return_optional_by_value(x: i32) -> MyOptional<i32> {
    MyOptional { has_value: true, value: x }
}

#[must_bind]
pub fn accept_optional_by_value(opt: MyOptional<i32>) -> i32 {
    if opt.has_value {
        opt.value
    } else {
        -1
    }
}

#[must_bind]
pub fn accept_optional_by_reference(opt: &MyOptional<i32>) -> i32 {
    if opt.has_value {
        opt.value
    } else {
        -1
    }
}

#[cpp_layout_equivalent(
    cpp_type = "crubit::test::MyStatusOr<{T}>",
    include_path = "cc_bindings_from_rs/test/bridging/cc_generics.h"
)]
#[repr(C)]
pub struct MyStatusOr<T> {
    pub has_value: bool,
    pub value: T,
}

#[cpp_specialization(
    cpp_type = "crubit::test::MyStatus",
    include_path = "cc_bindings_from_rs/test/bridging/cc_generics.h"
)]
pub type MyStatus = MyStatusOr<()>;

#[must_bind]
pub fn return_status_or_unit() -> MyStatusOr<()> {
    MyStatusOr { has_value: true, value: () }
}

#[must_bind]
pub fn return_status() -> MyStatus {
    return_status_or_unit()
}

pub type MyStatusAlias = MyStatusOr<()>;

#[must_bind]
pub fn return_status_alias() -> MyStatusAlias {
    return_status_or_unit()
}

pub type UnitAlias = ();
#[must_bind]
pub fn return_status_or_unit_alias() -> MyStatusOr<UnitAlias> {
    return_status_or_unit()
}

#[must_bind]
pub fn accept_status_or_unit(status: MyStatusOr<()>) -> bool {
    status.has_value
}

#[must_bind]
pub fn accept_status(status: MyStatus) -> bool {
    status.has_value
}

#[must_bind]
pub fn return_status_non_unit(status: MyStatusOr<u32>) -> MyStatusOr<u64> {
    MyStatusOr { has_value: status.has_value, value: status.value as u64 }
}

#[cpp_layout_equivalent(
    cpp_type = "crubit::test::MyPair<{T1}, {T2}>",
    include_path = "cc_bindings_from_rs/test/bridging/cc_generics.h"
)]
#[repr(C)]
pub struct MyPair<T1, T2> {
    pub first: T1,
    pub second: T2,
}

#[cpp_specialization(
    cpp_type = "crubit::test::MyIntBoolPair",
    include_path = "cc_bindings_from_rs/test/bridging/cc_generics.h"
)]
pub type MyIntBoolPair = MyPair<i32, bool>;

#[must_bind]
pub fn create_bool_bool_pair() -> MyPair<bool, bool> {
    MyPair { first: true, second: true }
}

#[must_bind]
pub fn create_int_bool_pair() -> MyPair<i32, bool> {
    MyPair { first: 42, second: true }
}

pub(crate) mod private_mod {
    pub type SecretInt = i32;
}

pub mod private_mod_with_specialization {
    use crate::private_mod::SecretInt;
    use crate::MyStatusOr;

    // Test specialization with parameter named several ways, including via a non-public alias.
    #[crubit_annotate::cpp_specialization(
        cpp_type = "crubit::test::MyStatus",
        include_path = "cc_bindings_from_rs/test/bridging/cc_generics.h"
    )]
    pub type MyStatusOrSecretInt = MyStatusOr<SecretInt>;
}

#[must_bind]
pub fn create_status_with_secret() -> MyStatusOr<private_mod::SecretInt> {
    MyStatusOr { has_value: true, value: 42 }
}

#[must_bind]
pub fn create_status_with_private_secret() -> MyStatusOr<private_mod::SecretInt> {
    MyStatusOr { has_value: true, value: 42 }
}

#[must_bind]
pub fn create_status_with_secret_alias() -> private_mod_with_specialization::MyStatusOrSecretInt {
    MyStatusOr { has_value: true, value: 42 }
}

#[must_bind]
pub fn is_ok_secret(status: MyStatusOr<private_mod::SecretInt>) -> bool {
    status.has_value
}
