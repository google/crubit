// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#![feature(rustc_private)]
#![deny(rustc::internal)]

use anyhow::Result;
use crubit_attr::{get_attrs, CrubitAttrs};
use run_compiler_test_support::{find_def_id_by_name, run_compiler_for_testing};
use rustc_middle::ty::TyCtxt;
use rustc_span::symbol::Symbol;

fn attrs_for_named_def(tcx: TyCtxt, name: &str) -> Result<CrubitAttrs> {
    get_attrs(tcx, find_def_id_by_name(tcx, name).into())
}

#[test]
fn test_bridged_type() {
    let test_src = r#"
            #[doc="CRUBIT_ANNOTATE: cpp_type=CppType"]
            #[doc="CRUBIT_ANNOTATE: include_path=crubit/cpp_type.h"]
            #[doc="CRUBIT_ANNOTATE: rust_to_cpp_converter=rust_to_cpp"]
            #[doc="CRUBIT_ANNOTATE: cpp_to_rust_converter=cpp_to_rust"]
            pub struct SomeStruct;
    "#;
    run_compiler_for_testing(test_src, |tcx| {
        let attrs = attrs_for_named_def(tcx, "SomeStruct").unwrap();

        let mut expected_attrs = CrubitAttrs::default();
        expected_attrs.cpp_type = Some(Symbol::intern("CppType"));
        expected_attrs.include_path = Some(Symbol::intern("crubit/cpp_type.h"));
        expected_attrs.cpp_to_rust_converter = Some(Symbol::intern("cpp_to_rust"));
        expected_attrs.rust_to_cpp_converter = Some(Symbol::intern("rust_to_cpp"));

        assert_eq!(attrs, expected_attrs);
    });
}

#[test]
fn test_missing() {
    let test_src = r#"
            pub struct SomeStruct;
        "#;
    run_compiler_for_testing(test_src, |tcx| {
        let attr = attrs_for_named_def(tcx, "SomeStruct").unwrap();
        assert_eq!(attr, CrubitAttrs::default());
    });
}

#[test]
fn test_cpp_type() {
    let test_src = r#"
            #[doc="CRUBIT_ANNOTATE: cpp_type=A C++ Type"]
            pub struct SomeStruct;
        "#;
    run_compiler_for_testing(test_src, |tcx| {
        let attr = attrs_for_named_def(tcx, "SomeStruct").unwrap();
        assert_eq!(attr.cpp_type.unwrap(), Symbol::intern("A C++ Type"));
    });
}

#[test]
fn test_cpp_name() {
    let test_src = r#"
            #[doc="CRUBIT_ANNOTATE: cpp_name=Create"]
            pub fn new() -> i32 { 0 }
        "#;
    run_compiler_for_testing(test_src, |tcx| {
        let attr = attrs_for_named_def(tcx, "new").unwrap();
        assert_eq!(attr.cpp_name.unwrap(), Symbol::intern("Create"));
    });
}

#[test]
fn test_cpp_name_duplicated() {
    let test_src = r#"
            #[doc="CRUBIT_ANNOTATE: cpp_name=Create"]
            #[doc="CRUBIT_ANNOTATE: cpp_name=Create"]
            pub fn new() -> i32 { 0 }
        "#;
    run_compiler_for_testing(test_src, |tcx| {
        let attr = attrs_for_named_def(tcx, "new");
        assert!(attr.is_err());
    });
}

#[test]
fn test_cpp_type_multi() {
    let test_src = r#"
            #[doc="CRUBIT_ANNOTATE: cpp_type=X"]
            #[doc="CRUBIT_ANNOTATE: cpp_type=X"]
            pub struct SomeStruct;
        "#;
    run_compiler_for_testing(test_src, |tcx| {
        let attr = attrs_for_named_def(tcx, "SomeStruct");
        assert!(attr.is_err());
    });
}
