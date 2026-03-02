// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
use cc_std::std::raw_string_view;
use displayables::{
    CanAbslStringify, CanAbslStringifyAndOstream, CanAbslStringifyByFill, CanAbslStringifyByFormat,
    CanOstream, DisplayInRust, DisplayableEnum, TemplatedNotDisplayable, TemplatedStringView,
};
use googletest::prelude::*;
use static_assertions::assert_not_impl_any;
use std::fmt::Display;
use std::io::Write;

#[gtest]
fn test_absl_stringify_works() {
    expect_eq!(CanAbslStringify { value: "hello".into() }.to_string(), "hello");
}

#[gtest]
fn test_absl_stringify_flushes() {
    expect_eq!(CanAbslStringify { value: (&[240u8]).into() }.to_string(), "�");
}

#[gtest]
fn test_absl_stringify_fails() {
    let mut bytes = [0u8; 3];

    expect_that!(
        write!(&mut bytes[..], "{}", CanAbslStringify { value: b"123\xff4".into() }),
        err(anything())
    );
    expect_eq!(&bytes[..], b"123");
}

#[gtest]
fn test_absl_stringify_flush_fails() {
    let mut bytes = [0u8; 3];

    expect_that!(
        write!(&mut bytes[..], "{}", CanAbslStringify { value: b"123\xf0".into() }),
        err(anything())
    );
    expect_eq!(&bytes[..], b"123");
}

#[gtest]
fn test_absl_stringify_by_fill_works() {
    expect_eq!(CanAbslStringifyByFill { count: 3, ch: b'a'.into() }.to_string(), "aaa");
}

#[gtest]
fn test_absl_stringify_by_fill_fails() {
    let mut bytes = [0u8; 3];

    expect_that!(
        write!(&mut bytes[..], "{}", CanAbslStringifyByFill { count: 3, ch: 255u8.into() }),
        err(anything())
    );
    expect_eq!(&bytes[..], "�".as_bytes());
}

#[gtest]
fn test_absl_stringify_by_format_works() {
    expect_eq!(CanAbslStringifyByFormat { value: "hello".into() }.to_string(), "hello");
}

#[gtest]
fn test_ostream_works() {
    expect_eq!(CanOstream { value: "hello".into() }.to_string(), "hello");
}

#[gtest]
fn test_ostream_flushes() {
    expect_eq!(CanOstream { value: (&[240u8]).into() }.to_string(), "�");
}

#[gtest]
fn test_ostream_fails() {
    let mut bytes = [0u8; 3];

    expect_that!(
        write!(&mut bytes[..], "{}", CanOstream { value: b"123\xff4".into() }),
        err(anything())
    );
    expect_eq!(&bytes[..], b"123");
}

#[gtest]
fn test_ostream_flush_fails() {
    let mut bytes = [0u8; 3];

    expect_that!(
        write!(&mut bytes[..], "{}", CanOstream { value: b"123\xf0".into() }),
        err(anything())
    );
    expect_eq!(&bytes[..], b"123");
}

#[gtest]
fn test_prefers_absl_stringify() {
    expect_eq!(
        CanAbslStringifyAndOstream { stringify: "good".into(), ostream: "bad".into() }.to_string(),
        "good"
    );
}

#[gtest]
fn test_enum_known_works() {
    expect_eq!(DisplayableEnum::kKnown.to_string(), "Known");
}

#[gtest]
fn test_enum_unknown_works() {
    expect_eq!(DisplayableEnum::from(123).to_string(), "123");
}

#[gtest]
fn test_crubit_override_display_true() {
    expect_eq!(TemplatedStringView::from(raw_string_view::from("hello")).to_string(), "hello");
}

#[gtest]
fn test_crubit_override_display_false() {
    assert_not_impl_any!(TemplatedNotDisplayable: Display);
    expect_eq!(
        DisplayInRust { cc_value: "bad".into(), rust_value: "good".into() }.to_string(),
        "good"
    );
}
