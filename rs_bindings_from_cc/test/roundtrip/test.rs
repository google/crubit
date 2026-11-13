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

#[gtest]
fn test_rust_string_parameter_and_return_type() {
    use roundtrip::std_test::EchoString;
    let s = String::from("hello from rust");
    let roundtrip = EchoString(s);
    expect_eq!(roundtrip, "hello from rust");
}

#[gtest]
fn test_struct_with_rust_string_supported() {
    use ctor::CtorNew;
    use roundtrip::std_test::{
        MakeStructWithString, NonTrivialAbiStructWithString, TrivialAbiStructWithString,
    };
    let mut s = MakeStructWithString(String::from("my_method"), 42);
    expect_eq!(unsafe { TrivialAbiStructWithString::GetId(&raw const s) }, 42);
    expect_eq!(unsafe { TrivialAbiStructWithString::IntoMethodName(&raw mut s) }, "my_method");

    let mut s2 = ctor::emplace!(<NonTrivialAbiStructWithString as CtorNew<()>>::ctor_new(()));
    expect_eq!(
        unsafe {
            NonTrivialAbiStructWithString::IntoMethodName(::std::pin::Pin::into_inner_unchecked(
                s2.as_mut(),
            ) as &mut _)
        },
        ""
    );
}

#[gtest]
fn test_struct_with_rust_string_field_experimental_trivial_abi() {
    use roundtrip_experimental::std_experimental_test::{
        MakeStructWithString, TrivialAbiStructWithString,
    };
    let s = MakeStructWithString(String::from("my_method"), 42);
    expect_eq!(&*s.method_name, "my_method");
    expect_eq!(s.id, 42);

    let constructed = TrivialAbiStructWithString {
        method_name: ::std::mem::ManuallyDrop::new(String::from("constructed")),
        id: 7,
    };
    expect_eq!(&*constructed.method_name, "constructed");
}

#[gtest]
fn test_struct_with_rust_string_field_experimental_non_trivial_abi() {
    use ctor::CtorNew;
    use roundtrip_experimental::std_experimental_test::NonTrivialAbiStructWithString;
    let s = ctor::emplace!(<NonTrivialAbiStructWithString as CtorNew<()>>::ctor_new(()));
    expect_eq!(&*s.method_name, "");
}
