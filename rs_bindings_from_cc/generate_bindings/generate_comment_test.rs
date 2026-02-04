// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Tests for comment generation.

use arc_anyhow::Result;
use database::code_snippet;
use database::{BindingsGenerator, Database};
use error_report::{ErrorReport, FatalErrors};
use ffi_types::Environment;
use generate_bindings::new_database;
use generate_comment::{generate_doc_comment, generate_unsupported};
use googletest::prelude::gtest;
use ir::{BazelLabel, ItemId, UnsupportedItem, UnsupportedItemKind, IR};
use ir_testing::make_ir_from_items;
use quote::quote;
use std::rc::Rc;
use token_stream_matchers::assert_rs_matches;

#[gtest]
fn test_generate_doc_comment_with_no_comment_with_no_source_loc_with_environment_production() {
    let actual = generate_doc_comment(None, None, None, Environment::Production);
    assert!(actual.is_none());
}

#[gtest]
fn test_generate_doc_comment_with_no_comment_with_source_loc_with_environment_production() {
    let actual =
        generate_doc_comment(None, None, Some("some/header;l=11"), Environment::Production);
    assert_rs_matches!(quote! { #actual }, quote! {#[doc = " some/header;l=11"]});
}

#[gtest]
fn test_generate_doc_comment_with_comment_with_source_loc_with_environment_production() {
    let actual = generate_doc_comment(
        Some("Some doc comment"),
        None,
        Some("some/header;l=12"),
        Environment::Production,
    );
    assert_rs_matches!(
        quote! { #actual },
        quote! {#[doc = " Some doc comment\n \n some/header;l=12"]}
    );
}

#[gtest]
fn test_generate_doc_comment_with_comment_with_no_source_loc_with_environment_production() {
    let actual =
        generate_doc_comment(Some("Some doc comment"), None, None, Environment::Production);
    assert_rs_matches!(quote! { #actual }, quote! {#[doc = " Some doc comment"]});
}

#[gtest]
fn test_no_generate_doc_comment_with_no_comment_with_no_source_loc_with_environment_golden_test() {
    let actual = generate_doc_comment(None, None, None, Environment::GoldenTest);
    assert!(actual.is_none());
}

#[gtest]
fn test_no_generate_doc_comment_with_no_comment_with_source_loc_with_environment_golden_test() {
    let actual =
        generate_doc_comment(None, None, Some("some/header;l=13"), Environment::GoldenTest);
    assert!(actual.is_none());
}

#[gtest]
fn test_no_generate_doc_comment_with_comment_with_source_loc_with_environment_golden_test() {
    let actual = generate_doc_comment(
        Some("Some doc comment"),
        None,
        Some("some/header;l=14"),
        Environment::GoldenTest,
    );
    assert_rs_matches!(quote! { #actual }, quote! {#[doc = " Some doc comment"]});
}

#[gtest]
fn test_no_generate_doc_comment_with_comment_with_no_source_loc_with_environment_golden_test() {
    let actual =
        generate_doc_comment(Some("Some doc comment"), None, None, Environment::GoldenTest);
    assert_rs_matches!(quote! { #actual }, quote! {#[doc = " Some doc comment"]});
}

#[gtest]
fn test_generate_doc_comment_with_safety() {
    let actual = generate_doc_comment(
        Some("Some doc comment"),
        Some("Some safety doc"),
        None,
        Environment::GoldenTest,
    );
    assert_rs_matches!(
        quote! { #actual },
        quote! {#[doc = " Some doc comment\n \n # Safety\n \n Some safety doc"]}
    );
}

struct TestItem {
    source_loc: Option<Rc<str>>,
}

const TEST_ITEM_ID: ItemId = ItemId::new_for_testing(123);

impl ir::GenericItem for TestItem {
    fn id(&self) -> ItemId {
        TEST_ITEM_ID
    }
    fn unique_name(&self) -> Option<Rc<str>> {
        None
    }
    fn owning_target(&self) -> Option<BazelLabel> {
        None
    }
    fn debug_name(&self, _: &IR) -> Rc<str> {
        "test_item".into()
    }
    fn source_loc(&self) -> Option<Rc<str>> {
        self.source_loc.clone()
    }
    fn unknown_attr(&self) -> Option<Rc<str>> {
        None
    }
    fn unsupported_kind(&self) -> UnsupportedItemKind {
        UnsupportedItemKind::Other
    }
    fn must_bind(&self) -> bool {
        false
    }
}

struct TestDbFactory {
    ir: IR,
    errors: ErrorReport,
    fatal_errors: FatalErrors,
}
impl TestDbFactory {
    fn new() -> Self {
        Self {
            ir: make_ir_from_items([]),
            errors: ErrorReport::new(),
            fatal_errors: FatalErrors::new(),
        }
    }
    fn make_db(&self, environment: Environment) -> Database {
        new_database(&self.ir, &self.errors, &self.fatal_errors, environment)
    }
}

#[gtest]
fn test_generate_unsupported_item_with_environment_production() -> Result<()> {
    let factory = TestDbFactory::new();
    let db = factory.make_db(Environment::Production);
    let _scope = error_report::ItemScope::new(
        db.errors(),
        error_report::ItemName { name: "test_item".into(), id: TEST_ITEM_ID.as_u64() },
    );
    let actual = generate_unsupported(
        &db,
        UnsupportedItem::new_with_static_message(
            &db.ir(),
            &TestItem { source_loc: Some("Generated from: some/header;l=1".into()) },
            /* path= */ None,
            "unsupported_message",
        )
        .into(),
    )
    .generated_items;
    let actual = code_snippet::generated_items_to_token_stream(&actual, db.ir(), &[TEST_ITEM_ID]);
    let expected = "Generated from: some/header;l=1\nError while generating bindings for item 'test_item':\nunsupported_message";
    assert_rs_matches!(quote! { #actual }, quote! { __COMMENT__ #expected});
    Ok(())
}

/// Not all items currently have source_loc(), e.g. comments.
///
/// For these, we omit the mention of the location.
#[gtest]
fn test_generate_unsupported_item_with_missing_source_loc() -> Result<()> {
    let factory = TestDbFactory::new();
    let db = factory.make_db(Environment::Production);
    let _scope = error_report::ItemScope::new(
        db.errors(),
        error_report::ItemName { name: "test_item".into(), id: TEST_ITEM_ID.as_u64() },
    );
    let actual = generate_unsupported(
        &db,
        UnsupportedItem::new_with_static_message(
            &db.ir(),
            &TestItem { source_loc: None },
            /* path= */ None,
            "unsupported_message",
        )
        .into(),
    )
    .generated_items;
    let actual = code_snippet::generated_items_to_token_stream(&actual, db.ir(), &[TEST_ITEM_ID]);
    let expected = "Error while generating bindings for item 'test_item':\nunsupported_message";
    assert_rs_matches!(quote! { #actual }, quote! { __COMMENT__ #expected});
    Ok(())
}

#[gtest]
fn test_generate_unsupported_item_with_environment_golden_test() -> Result<()> {
    let factory = TestDbFactory::new();
    let db = factory.make_db(Environment::GoldenTest);
    let _scope = error_report::ItemScope::new(
        db.errors(),
        error_report::ItemName { name: "test_item".into(), id: TEST_ITEM_ID.as_u64() },
    );
    let actual = generate_unsupported(
        &db,
        UnsupportedItem::new_with_static_message(
            &db.ir(),
            &TestItem { source_loc: Some("Generated from: some/header;l=1".into()) },
            /* path= */ None,
            "unsupported_message",
        )
        .into(),
    )
    .generated_items;
    let actual = code_snippet::generated_items_to_token_stream(&actual, db.ir(), &[TEST_ITEM_ID]);
    let expected = "Error while generating bindings for item 'test_item':\nunsupported_message";
    assert_rs_matches!(quote! { #actual }, quote! { __COMMENT__ #expected});
    Ok(())
}
