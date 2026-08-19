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
        expected_attrs.include_paths = vec![Symbol::intern("crubit/cpp_type.h")];
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
fn test_cpp_name_multi() {
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

#[test]
fn test_include_path_multi() {
    let test_src = r#"
            #[doc="CRUBIT_ANNOTATE: cpp_type=CppType"]
            #[doc="CRUBIT_ANNOTATE: include_path=crubit/cpp_type1.h"]
            #[doc="CRUBIT_ANNOTATE: include_path=crubit/cpp_type2.h"]
            pub struct SomeStruct;
    "#;
    run_compiler_for_testing(test_src, |tcx| {
        let attrs = attrs_for_named_def(tcx, "SomeStruct").unwrap();

        let mut expected_attrs = CrubitAttrs::default();
        expected_attrs.cpp_type = Some(Symbol::intern("CppType"));
        expected_attrs.include_paths =
            vec![Symbol::intern("crubit/cpp_type1.h"), Symbol::intern("crubit/cpp_type2.h")];

        assert_eq!(attrs, expected_attrs);
    });
}

#[test]
fn test_cpp_thread_safe() {
    let test_src = r#"
            #[doc="CRUBIT_ANNOTATE: cpp_thread_safe="]
            pub struct SomeStruct;
    "#;
    run_compiler_for_testing(test_src, |tcx| {
        let attrs = attrs_for_named_def(tcx, "SomeStruct").unwrap();

        let mut expected_attrs = CrubitAttrs::default();
        expected_attrs.cpp_thread_safe = true;

        assert_eq!(attrs, expected_attrs);
    });
}

#[test]
fn test_do_not_bind() {
    let test_src = r#"
            #[doc="CRUBIT_ANNOTATE: do_not_bind="]
            pub fn foo() {}
    "#;
    run_compiler_for_testing(test_src, |tcx| {
        let attrs = attrs_for_named_def(tcx, "foo").unwrap();
        let mut expected_attrs = CrubitAttrs::default();
        expected_attrs.do_not_bind = true;
        assert_eq!(attrs, expected_attrs);
    });
}

#[test]
fn test_do_not_bind_invalid_on_struct() {
    let test_src = r#"
            #[doc="CRUBIT_ANNOTATE: do_not_bind="]
            pub struct SomeStruct;
    "#;
    run_compiler_for_testing(test_src, |tcx| {
        let err = attrs_for_named_def(tcx, "SomeStruct").unwrap_err();
        assert_eq!(
            err.to_string(),
            "`do_not_bind` is explicitly only permitted on functions and methods"
        );
    });
}

#[test]
fn test_skip_mutable_aliasing_check() {
    let test_src = r#"
            #[doc="CRUBIT_ANNOTATE: skip_mutable_aliasing_check="]
            pub fn foo() {}
    "#;
    run_compiler_for_testing(test_src, |tcx| {
        let attrs = attrs_for_named_def(tcx, "foo").unwrap();
        let mut expected_attrs = CrubitAttrs::default();
        expected_attrs.skip_mutable_aliasing_check = true;
        assert_eq!(attrs, expected_attrs);
    });
}

#[test]
fn test_skip_mutable_aliasing_check_invalid_on_struct() {
    let test_src = r#"
            #[doc="CRUBIT_ANNOTATE: skip_mutable_aliasing_check="]
            pub struct SomeStruct;
    "#;
    run_compiler_for_testing(test_src, |tcx| {
        let err = attrs_for_named_def(tcx, "SomeStruct").unwrap_err();
        assert_eq!(
            err.to_string(),
            "`skip_mutable_aliasing_check` is explicitly only permitted on functions and methods"
        );
    });
}
