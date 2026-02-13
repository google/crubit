// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
use formattables::{
    CanAbslStringify, CanAbslStringifyAndOstream, CanAbslStringifyByFill, CanAbslStringifyByFormat,
    CanOstream, DerivesDebug, FormattableEnum,
};
use googletest::prelude::*;
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
fn test_debug_works() {
    expect_eq!(format!("{:?}", CanAbslStringify { value: "hello".into() }), "hello");
}

#[gtest]
fn test_impl_display_and_derive_debug() {
    let object = DerivesDebug { display: "Display".into(), only_debug: "OnlyDebug".into() };

    expect_eq!(object.to_string(), "Display");
    expect_that!(format!("{:?}", object), contains_substring("only_debug"));
}

#[gtest]
fn test_enum_known_works() {
    expect_eq!(FormattableEnum::kKnown.to_string(), "Known");
}

#[gtest]
fn test_enum_unknown_works() {
    expect_eq!(FormattableEnum::from(123).to_string(), "123");
}
