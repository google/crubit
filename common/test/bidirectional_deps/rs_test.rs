// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::gtest;

#[gtest]
fn test_round_trip() {
    let value = leaf_rs_lib::wrap(4);
    assert_eq!(middle_cc_lib::crubit::Unwrap(value), 4);

    let value = middle_cc_lib::crubit::Wrap(2);
    assert_eq!(leaf_rs_lib::unwrap(value), 2);
}

#[gtest]
fn test_enum_round_trip() {
    let value = leaf_rs_lib::wrap_enum(2);
    assert_eq!(middle_cc_lib::crubit::UnwrapEnum(value), 2);

    let value = middle_cc_lib::crubit::WrapEnum(1);
    assert_eq!(leaf_rs_lib::unwrap_enum(value), 1);
}

#[gtest]
fn test_alias_round_trip() {
    let value: middle_cc_lib::LeafRsTypeAlias2 = leaf_rs_lib::wrap(2);
    assert_eq!(middle_cc_lib::crubit::Unwrap(value), 2);
}

#[gtest]
fn test_lifetime_round_trip() {
    let static_value: leaf_rs_lib::LeafRsLifetime<'static> =
        middle_cc_lib::crubit::MakeStaticLifetime();
    assert_eq!(leaf_rs_lib::unwrap_lifetime(static_value), 42);

    let unannotated_value: leaf_rs_lib::LeafRsLifetime<'static> =
        middle_cc_lib::crubit::MakeUnannotatedLifetime();
    assert_eq!(middle_cc_lib::crubit::UnwrapLifetime(unannotated_value), 42);

    let y: u8 = 9;
    let x: u8 = 7;
    let ambiguous_value: leaf_rs_lib::LeafRsLifetime<'static> =
        middle_cc_lib::crubit::MakeAmbiguousLifetime(&x, &y);
    assert_eq!(middle_cc_lib::crubit::UnwrapLifetime(ambiguous_value), 7);

    let static_alias: leaf_rs_lib::StaticLeafRsLifetime =
        middle_cc_lib::crubit::MakeStaticLifetimeAlias();
    assert_eq!(leaf_rs_lib::unwrap_lifetime(static_alias), 42);

    let borrowed: leaf_rs_lib::LeafRsLifetime<'_> = middle_cc_lib::crubit::WrapLifetime(&x);
    assert_eq!(middle_cc_lib::crubit::UnwrapLifetime(borrowed), 7);

    let elided: leaf_rs_lib::LeafRsLifetime<'_> = middle_cc_lib::crubit::WrapLifetimeElided(&x);
    assert_eq!(leaf_rs_lib::unwrap_lifetime(elided), 7);

    let alias_borrowed: middle_cc_lib::LeafRsLifetimeAlias2<'_> =
        middle_cc_lib::crubit::WrapLifetimeAlias(&x);
    assert_eq!(middle_cc_lib::crubit::UnwrapLifetime(alias_borrowed), 7);
}
