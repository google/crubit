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
    // SAFETY: `MakeAmbiguousLifetime` borrows `x`, which outlives `ambiguous_value`.
    let ambiguous_value: leaf_rs_lib::LeafRsLifetime<'static> =
        unsafe { middle_cc_lib::crubit::MakeAmbiguousLifetime(&x, &y).into_inner() };
    assert_eq!(middle_cc_lib::crubit::UnwrapLifetime(ambiguous_value), 7);

    let static_alias: leaf_rs_lib::StaticLeafRsLifetime =
        middle_cc_lib::crubit::MakeStaticLifetimeAlias();
    assert_eq!(leaf_rs_lib::unwrap_lifetime(static_alias), 42);

    let borrowed: leaf_rs_lib::LeafRsLifetime<'_> = middle_cc_lib::crubit::WrapLifetime(&x);
    assert_eq!(middle_cc_lib::crubit::UnwrapLifetime(borrowed), 7);

    // SAFETY: `UnwrapUnknownLifetime` only reads `static_value` during the call.
    assert_eq!(
        unsafe {
            middle_cc_lib::crubit::UnwrapUnknownLifetime(crubit_support::Unsafe::new(
                leaf_rs_lib::static_lifetime(),
            ))
        },
        42,
    );

    let elided: leaf_rs_lib::LeafRsLifetime<'_> = middle_cc_lib::crubit::WrapLifetimeElided(&x);
    assert_eq!(leaf_rs_lib::unwrap_lifetime(elided), 7);

    let alias_borrowed: middle_cc_lib::LeafRsLifetimeAlias2<'_> =
        middle_cc_lib::crubit::WrapLifetimeAlias(&x);
    assert_eq!(middle_cc_lib::crubit::UnwrapLifetime(alias_borrowed), 7);
}

#[gtest]
fn test_lifetime_round_trip_no_assume_lifetimes() {
    // SAFETY: `MakeNoAssumeLifetime` returns a `'static` `LeafRsLifetime`.
    let wrapped: crubit_support::Unsafe<leaf_rs_lib::LeafRsLifetime<'static>> =
        unsafe { middle_cc_lib_no_assume_lifetimes::MakeNoAssumeLifetime() };
    assert_eq!(format!("{wrapped:?}"), "Unsafe(..)");
    // SAFETY: as above.
    let unwrapped = unsafe { wrapped.into_inner() };
    assert_eq!(leaf_rs_lib::unwrap_lifetime(unwrapped), 42);

    let x: u8 = 11;
    let borrowed = leaf_rs_lib::wrap_lifetime(&x);
    // SAFETY: `TakeNoAssumeLifetime` only reads `borrowed` during the call and `x` is live.
    assert_eq!(unsafe { middle_cc_lib_no_assume_lifetimes::TakeNoAssumeLifetime(borrowed) }, 11);

    // SAFETY: `MakeNoAssumeAlias` returns a `'static` `LeafRsLifetime`.
    let alias_val: middle_cc_lib_no_assume_lifetimes::NoAssumeAlias =
        unsafe { middle_cc_lib_no_assume_lifetimes::MakeNoAssumeAlias() };
    // SAFETY: as above.
    assert_eq!(unsafe { middle_cc_lib_no_assume_lifetimes::TakeNoAssumeAlias(alias_val) }, 42);

    let holder = middle_cc_lib_no_assume_lifetimes::MakeNoAssumeHolder();
    let holder_copy = holder;
    assert_eq!(format!("{:?}", holder.field), "Unsafe(..)");
    // SAFETY: `MakeNoAssumeHolder` initializes `field` with a `'static` `LeafRsLifetime`.
    assert_eq!(leaf_rs_lib::unwrap_lifetime(unsafe { holder_copy.field.into_inner() }), 42);

    // SAFETY: `MakeNoAssumeBounded` borrows a function-local `static`.
    let bounded: leaf_rs_lib::LeafRsBounded<'static, 'static, 'static> =
        unsafe { middle_cc_lib_no_assume_lifetimes::MakeNoAssumeBounded().into_inner() };
    assert_eq!(leaf_rs_lib::unwrap_bounded(bounded), 5);

    // SAFETY: `MakeNoAssumeStaticBound` borrows a function-local `static`.
    let static_bound: leaf_rs_lib::LeafRsStaticBound<'static> =
        unsafe { middle_cc_lib_no_assume_lifetimes::MakeNoAssumeStaticBound().into_inner() };
    assert_eq!(leaf_rs_lib::unwrap_static_bound(static_bound), 6);
}

#[gtest]
fn test_bounded_lifetimes() {
    let x: u8 = 3;
    let y: u8 = 4;
    let z: u8 = 5;

    let same: leaf_rs_lib::LeafRsBounded<'_, '_, '_> = middle_cc_lib::crubit::WrapBounded(&x, &z);
    assert_eq!(middle_cc_lib::crubit::UnwrapBounded(same), 3);

    let distinct: leaf_rs_lib::LeafRsBounded<'_, '_, '_> =
        middle_cc_lib::crubit::WrapBoundedDistinct(&x, &y, &z);
    assert_eq!(middle_cc_lib::crubit::UnwrapBounded(distinct), 3);

    // Elision gives an unannotated result the lifetime of the only reference parameter when the
    // result takes one lifetime. This result takes three, so its lifetimes are unknown and it is
    // wrapped. (Rust's elision rule would give all three the lifetime of `x`.)
    // SAFETY: `WrapBoundedElided` borrows `x`, which outlives `elided`.
    let elided: leaf_rs_lib::LeafRsBounded<'static, 'static, 'static> =
        unsafe { middle_cc_lib::crubit::WrapBoundedElided(&x).into_inner() };
    assert_eq!(middle_cc_lib::crubit::UnwrapBounded(elided), 3);

    // SAFETY: `MakeBoundedAmbiguous` borrows `x` and `z`, which outlive `ambiguous`.
    let ambiguous: leaf_rs_lib::LeafRsBounded<'static, 'static, 'static> =
        unsafe { middle_cc_lib::crubit::MakeBoundedAmbiguous(&x, &z).into_inner() };
    assert_eq!(middle_cc_lib::crubit::UnwrapBounded(ambiguous), 3);
}

#[gtest]
fn test_static_bound() {
    static VALUE: u8 = 9;

    let explicit: leaf_rs_lib::LeafRsStaticBound<'static> =
        middle_cc_lib::crubit::WrapStaticBound(&VALUE);
    assert_eq!(middle_cc_lib::crubit::UnwrapStaticBound(explicit), 9);

    // The generated signature is `WrapStaticBoundElided<'x>(x: &'x u8) -> LeafRsStaticBound<'x>`;
    // the type's bound adds the implied bound `'x: 'static`, so the argument must be `'static`.
    let elided = middle_cc_lib::crubit::WrapStaticBoundElided(&VALUE);
    assert_eq!(middle_cc_lib::crubit::UnwrapStaticBound(elided), 9);
}

#[gtest]
fn test_callback_with_lifetime_param() {
    // The callback's parameter is `LeafRsLifetime<'_>`: it accepts a value of any lifetime
    // (here one that borrows a local of the C++ caller), with no `Unsafe` wrapper.
    let result =
        middle_cc_lib::crubit::CallWithLifetime(Box::new(|x: leaf_rs_lib::LeafRsLifetime<'_>| {
            leaf_rs_lib::unwrap_lifetime(x)
        }));
    assert_eq!(result, 7);
}
