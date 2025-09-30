// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
use dyn_format::Format;
use googletest::prelude::*;

#[gtest]
fn test_empty() {
    let format = Format::parse_with_metavars("", &[]).unwrap();
    assert_that!(format.format(&[]), eq(""));
}

#[gtest]
fn test_unused_variable() {
    let format = Format::parse_with_metavars("", &["x"]).unwrap();
    assert_that!(format.format(&["a"]), eq(""));
}

#[gtest]
fn test_1_variable() {
    let format = Format::parse_with_metavars("{  x } is {x}", &["x"]).unwrap();
    assert_that!(format.format(&["a"]), eq("a is a"));
}

#[gtest]
fn test_escape() {
    let format = Format::parse_with_metavars("{{x}} is {{x}}", &[]).unwrap();
    assert_that!(format.format(&[]), eq("{x} is {x}"));
}

#[gtest]
fn test_unmatched_open() {
    let format = Format::parse_with_metavars("{x", &[]);
    assert_that!(
        format,
        err(displays_as(contains_substring("invalid format string: unmatched `{`")))
    );
}

#[gtest]
fn test_unmatched_close() {
    let format = Format::parse_with_metavars("}", &[]);
    assert_that!(
        format,
        err(displays_as(contains_substring("invalid format string: unmatched `}`")))
    );
}

#[gtest]
fn test_unknown_variable() {
    let format = Format::parse_with_metavars("{y}", &["x"]);
    assert_that!(
        format,
        err(displays_as(contains_substring("invalid format string: unknown variable `y`")))
    );
}

#[gtest]
fn test_mixed_escape() {
    let format = Format::parse_with_metavars("{{{x}}}", &["x"]).unwrap();
    assert_that!(format.format(&["a"]), eq("{a}"));
}
