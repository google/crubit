// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Tests for comment generation.

use arc_anyhow::Result;
use database::db::FatalErrors;
use database::{BindingsGenerator, Database};
use error_report::ErrorReport;
use ffi_types::SourceLocationDocComment;
use generate_bindings::new_database;
use generate_comment::{generate_doc_comment, generate_unsupported};
use googletest::prelude::gtest;
use ir::{ItemId, UnsupportedItem, UnsupportedItemKind, IR};
use ir_testing::make_ir_from_items;
use quote::quote;
use std::rc::Rc;
use token_stream_matchers::assert_rs_matches;

#[gtest]
fn test_generate_doc_comment_with_no_comment_with_no_source_loc_with_source_loc_enabled() {
    let actual = generate_doc_comment(None, None, SourceLocationDocComment::Enabled);
    assert!(actual.is_empty());
}

#[gtest]
fn test_generate_doc_comment_with_no_comment_with_source_loc_with_source_loc_enabled() {
    let actual = generate_doc_comment(
        None,
        Some("google3/some/header;l=11"),
        SourceLocationDocComment::Enabled,
    );
    assert_rs_matches!(actual, quote! {#[doc = " google3/some/header;l=11"]});
}

#[gtest]
fn test_generate_doc_comment_with_comment_with_source_loc_with_source_loc_enabled() {
    let actual = generate_doc_comment(
        Some("Some doc comment"),
        Some("google3/some/header;l=12"),
        SourceLocationDocComment::Enabled,
    );
    assert_rs_matches!(actual, quote! {#[doc = " Some doc comment\n \n google3/some/header;l=12"]});
}

#[gtest]
fn test_generate_doc_comment_with_comment_with_no_source_loc_with_source_loc_enabled() {
    let actual =
        generate_doc_comment(Some("Some doc comment"), None, SourceLocationDocComment::Enabled);
    assert_rs_matches!(actual, quote! {#[doc = " Some doc comment"]});
}

#[gtest]
fn test_no_generate_doc_comment_with_no_comment_with_no_source_loc_with_source_loc_disabled() {
    let actual = generate_doc_comment(None, None, SourceLocationDocComment::Disabled);
    assert!(actual.is_empty());
}

#[gtest]
fn test_no_generate_doc_comment_with_no_comment_with_source_loc_with_source_loc_disabled() {
    let actual = generate_doc_comment(
        None,
        Some("google3/some/header;l=13"),
        SourceLocationDocComment::Disabled,
    );
    assert!(actual.is_empty());
}

#[gtest]
fn test_no_generate_doc_comment_with_comment_with_source_loc_with_source_loc_disabled() {
    let actual = generate_doc_comment(
        Some("Some doc comment"),
        Some("google3/some/header;l=14"),
        SourceLocationDocComment::Disabled,
    );
    assert_rs_matches!(actual, quote! {#[doc = " Some doc comment"]});
}

#[gtest]
fn test_no_generate_doc_comment_with_comment_with_no_source_loc_with_source_loc_disabled() {
    let actual =
        generate_doc_comment(Some("Some doc comment"), None, SourceLocationDocComment::Disabled);
    assert_rs_matches!(actual, quote! {#[doc = " Some doc comment"]});
}

struct TestItem {
    source_loc: Option<Rc<str>>,
}
impl ir::GenericItem for TestItem {
    fn id(&self) -> ItemId {
        ItemId::new_for_testing(123)
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
        UnsupportedItemKind::Unnameable
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
    fn make_db(&self, source_loc_doc_comment: SourceLocationDocComment) -> Database {
        new_database(&self.ir, &self.errors, &self.fatal_errors, source_loc_doc_comment)
    }
}

#[gtest]
fn test_generate_unsupported_item_with_source_loc_enabled() -> Result<()> {
    let factory = TestDbFactory::new();
    let db = factory.make_db(SourceLocationDocComment::Enabled);
    let actual = generate_unsupported(
        &db,
        &UnsupportedItem::new_with_static_message(
            &db.ir(),
            &TestItem { source_loc: Some("Generated from: some/header;l=1".into()) },
            /* path= */ None,
            "unsupported_message",
        ),
    );
    let expected = "Generated from: some/header;l=1\nError while generating bindings for item 'test_item':\nunsupported_message";
    assert_rs_matches!(actual.main_api, quote! { __COMMENT__ #expected});
    Ok(())
}

/// Not all items currently have source_loc(), e.g. comments.
///
/// For these, we omit the mention of the location.
#[gtest]
fn test_generate_unsupported_item_with_missing_source_loc() -> Result<()> {
    let factory = TestDbFactory::new();
    let db = factory.make_db(SourceLocationDocComment::Enabled);
    let actual = generate_unsupported(
        &db,
        &UnsupportedItem::new_with_static_message(
            &db.ir(),
            &TestItem { source_loc: None },
            /* path= */ None,
            "unsupported_message",
        ),
    );
    let expected = "Error while generating bindings for item 'test_item':\nunsupported_message";
    assert_rs_matches!(actual.main_api, quote! { __COMMENT__ #expected});
    Ok(())
}

#[gtest]
fn test_generate_unsupported_item_with_source_loc_disabled() -> Result<()> {
    let factory = TestDbFactory::new();
    let db = factory.make_db(SourceLocationDocComment::Disabled);
    let actual = generate_unsupported(
        &db,
        &UnsupportedItem::new_with_static_message(
            &db.ir(),
            &TestItem { source_loc: Some("Generated from: some/header;l=1".into()) },
            /* path= */ None,
            "unsupported_message",
        ),
    );
    let expected = "Error while generating bindings for item 'test_item':\nunsupported_message";
    assert_rs_matches!(actual.main_api, quote! { __COMMENT__ #expected});
    Ok(())
}
