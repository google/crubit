// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::{expect_eq, gtest};

#[gtest]
fn test_rust_char_parameter_type_and_return_type() {
    use roundtrip::char_test::NextChar;
    let roundtrip = NextChar('a');
    expect_eq!(roundtrip, 'b');
}

#[gtest]
fn test_rust_char_field_type() {
    use roundtrip::char_test::SomeStruct;
    let s = SomeStruct { c: 'x' };
    expect_eq!('x', unsafe { SomeStruct::GetChar(&raw const s) });
}

#[gtest]
fn test_rust_char_via_type_alias() {
    use roundtrip::char_test::NextCharViaTypeAlias;
    let roundtrip = NextCharViaTypeAlias('a');
    expect_eq!(roundtrip, 'b');
}

#[gtest]
fn test_rust_char_via_import() {
    use roundtrip::char_test::NextCharViaImport;
    let roundtrip = NextCharViaImport('a');
    expect_eq!(roundtrip, 'b');
}
