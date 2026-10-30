// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Tests for comment generation.

use arc_anyhow::Result;
use database::code_snippet;
use generate_comment::{generate_doc_comment, generate_unsupported};
use googletest::prelude::gtest;
use ir::{BazelLabel, DeclLocation, ItemId, UnsupportedItem, UnsupportedItemKind};
use ir_testing::make_ir_from_items;
use quote::quote;
use test_generators::TestDbFactory;
use token_stream_matchers::{assert_rs_matches, assert_rs_not_matches};

#[gtest]
fn test_generate_doc_comment_with_no_comment_with_no_source_loc_with_environment_production() {
    let actual = generate_doc_comment(None, None, None, false, false);
    assert!(actual.is_none());
}

#[gtest]
fn test_generate_doc_comment_with_no_comment_with_source_loc_with_environment_production() {
    let decl_loc = test_decl_loc();
    let actual = generate_doc_comment(None, None, Some(&decl_loc), false, false);
    let expected = format!(" {decl_loc}");
    assert_rs_matches!(quote! { #actual }, quote! {#[doc = #expected]});
}

#[gtest]
fn test_generate_doc_comment_with_comment_with_source_loc_with_environment_production() {
    let decl_loc = test_decl_loc();
    let actual =
        generate_doc_comment(Some("Some doc comment"), None, Some(&decl_loc), false, false);
    let expected = format!(" Some doc comment\n \n {decl_loc}");
    assert_rs_matches!(quote! { #actual }, quote! {#[doc = #expected]});
}

#[gtest]
fn test_generate_doc_comment_with_comment_with_no_source_loc_with_environment_production() {
    let actual = generate_doc_comment(Some("Some doc comment"), None, None, false, false);
    assert_rs_matches!(quote! { #actual }, quote! {#[doc = " Some doc comment"]});
}

#[gtest]
fn test_no_generate_doc_comment_with_no_comment_with_no_source_loc_with_environment_golden_test() {
    let actual = generate_doc_comment(None, None, None, true, false);
    assert!(actual.is_none());
}

#[gtest]
fn test_no_generate_doc_comment_with_no_comment_with_source_loc_with_environment_golden_test() {
    let decl_loc = test_decl_loc();
    let actual = generate_doc_comment(None, None, Some(&decl_loc), true, false);
    assert!(actual.is_none());
}

#[gtest]
fn test_no_generate_doc_comment_with_comment_with_source_loc_with_environment_golden_test() {
    let decl_loc = test_decl_loc();
    let actual = generate_doc_comment(Some("Some doc comment"), None, Some(&decl_loc), true, false);
    assert_rs_matches!(quote! { #actual }, quote! {#[doc = " Some doc comment"]});
}

#[gtest]
fn test_no_generate_doc_comment_with_comment_with_no_source_loc_with_environment_golden_test() {
    let actual = generate_doc_comment(Some("Some doc comment"), None, None, true, false);
    assert_rs_matches!(quote! { #actual }, quote! {#[doc = " Some doc comment"]});
}

#[gtest]
fn test_generate_doc_comment_with_safety() {
    let actual =
        generate_doc_comment(Some("Some doc comment"), Some("Some safety doc"), None, true, false);
    assert_rs_matches!(
        quote! { #actual },
        quote! {#[doc = " Some doc comment\n \n # Safety\n \n Some safety doc"]}
    );
}

struct TestItem {
    decl_loc: Option<DeclLocation>,
    must_bind: bool,
}

/// The location of the test items: line 1 of `some/header`.
fn test_decl_loc() -> DeclLocation {
    ir::proto!(DeclLocation { spelling: __ { path: "some/header", line: 1u32 } })
}

const TEST_ITEM_ID: ItemId = ItemId::new_for_testing(123);
/// The spelling location of `test_decl_loc()`, as it appears in the error report.
const TEST_ITEM_REPORT_SOURCE_LOC: &str = "some/header;l=1";

impl ir::GenericItem for TestItem {
    fn id(&self) -> ItemId {
        TEST_ITEM_ID
    }
    fn unique_name(&self) -> Option<&str> {
        None
    }
    fn owning_target(&self) -> Option<BazelLabel> {
        None
    }
    fn decl_loc(&self) -> Option<&DeclLocation> {
        self.decl_loc.as_ref()
    }
    fn unknown_attr(&self) -> Option<&str> {
        None
    }
    fn unsupported_kind(&self) -> UnsupportedItemKind {
        UnsupportedItemKind::Other
    }
    fn must_bind(&self) -> bool {
        self.must_bind
    }
}

fn make_factory() -> TestDbFactory {
    let test_item = UnsupportedItem::new_raw(
        "test_item",
        None::<&str>,
        UnsupportedItemKind::Other,
        TEST_ITEM_ID,
        Some(test_decl_loc()),
        None,
        false,
        /* path= */ None,
        Some(ir::FormattedError {
            fmt: "unsupported_message".into(),
            message: "unsupported_message".into(),
        }),
    );
    TestDbFactory::new(make_ir_from_items([test_item.into()]))
}

#[gtest]
fn test_generate_unsupported_item_with_environment_production() -> Result<()> {
    let factory = make_factory();
    let db = factory.make_db();
    let _scope = error_report::ItemScope::new(
        db.errors(),
        error_report::ItemName {
            name: "test_item".into(),
            id: TEST_ITEM_ID.as_u64(),
            unique_name: None,
            defining_target: None,
            source_loc: Some(TEST_ITEM_REPORT_SOURCE_LOC.into()),
        },
    );
    let actual = generate_unsupported(
        &db,
        db.new_unsupported_item_with_static_message(
            &TestItem { decl_loc: Some(test_decl_loc()), must_bind: false },
            /* path= */ None,
            "unsupported_message",
        )
        .into(),
    )
    .generated_items;
    let actual = code_snippet::generated_items_to_token_stream(&actual, &db, &[TEST_ITEM_ID]);
    let expected = format!(
        "{}\nerror: item `test_item` could not be bound\n  unsupported_message",
        test_decl_loc()
    );
    assert_rs_matches!(quote! { #actual }, quote! { __COMMENT__ #expected});
    Ok(())
}

#[gtest]
fn test_generate_unsupported_item_with_global_cpp() -> Result<()> {
    let factory = make_factory();
    let db = factory.make_db();
    let _scope = error_report::ItemScope::new(
        db.errors(),
        error_report::ItemName {
            name: "test_item".into(),
            id: TEST_ITEM_ID.as_u64(),
            unique_name: None,
            defining_target: None,
            source_loc: Some(TEST_ITEM_REPORT_SOURCE_LOC.into()),
        },
    );
    let mut unsupported_item = db.new_unsupported_item_with_static_message(
        &TestItem { decl_loc: Some(test_decl_loc()), must_bind: false },
        /* path= */ None,
        "unsupported_message",
    );
    unsupported_item.set_inline_cpp_source_text("int Add(int a, int b) { return a + b; }".into());
    let actual = generate_unsupported(&db, unsupported_item.into()).generated_items;
    let actual = code_snippet::generated_items_to_token_stream(&actual, &db, &[TEST_ITEM_ID]);
    let expected_comment = format!(
        "{}\nerror: item `test_item` could not be bound\n  unsupported_message",
        test_decl_loc()
    );
    let expected = quote! {
        __COMMENT__ #expected_comment
        ::inline_cpp_macro::global_cpp! {
            int Add(int a, int b) { return a + b; }
        }
    };
    assert_rs_matches!(quote! { #actual }, expected);
    Ok(())
}

#[gtest]
fn test_generate_unsupported_item_no_global_cpp_if_empty_source_text() -> Result<()> {
    let factory = make_factory();
    let db = factory.make_db();
    let _scope = error_report::ItemScope::new(
        db.errors(),
        error_report::ItemName {
            name: "test_item".into(),
            id: TEST_ITEM_ID.as_u64(),
            unique_name: None,
            defining_target: None,
            source_loc: Some(TEST_ITEM_REPORT_SOURCE_LOC.into()),
        },
    );
    let mut unsupported_item = db.new_unsupported_item_with_static_message(
        &TestItem { decl_loc: Some(test_decl_loc()), must_bind: false },
        /* path= */ None,
        "unsupported_message",
    );
    unsupported_item.clear_inline_cpp_source_text();
    let actual = generate_unsupported(&db, unsupported_item.into()).generated_items;
    let actual = code_snippet::generated_items_to_token_stream(&actual, &db, &[TEST_ITEM_ID]);
    assert_rs_not_matches!(actual, quote! { ::inline_cpp_macro::global_cpp! });
    Ok(())
}

/// Not all items currently have decl_loc(), e.g. comments.
///
/// For these, we omit the mention of the location.
#[gtest]
fn test_generate_unsupported_item_with_missing_source_loc() -> Result<()> {
    let factory = make_factory();
    let db = factory.make_db();
    let _scope = error_report::ItemScope::new(
        db.errors(),
        error_report::ItemName {
            name: "test_item".into(),
            id: TEST_ITEM_ID.as_u64(),
            unique_name: None,
            defining_target: None,
            source_loc: Some(TEST_ITEM_REPORT_SOURCE_LOC.into()),
        },
    );
    let actual = generate_unsupported(
        &db,
        db.new_unsupported_item_with_static_message(
            &TestItem { decl_loc: None, must_bind: false },
            /* path= */ None,
            "unsupported_message",
        )
        .into(),
    )
    .generated_items;
    let actual = code_snippet::generated_items_to_token_stream(&actual, &db, &[TEST_ITEM_ID]);
    let expected = "error: item `test_item` could not be bound\n  unsupported_message";
    assert_rs_matches!(quote! { #actual }, quote! { __COMMENT__ #expected});
    Ok(())
}

#[gtest]
fn test_generate_unsupported_item_with_environment_golden_test() -> Result<()> {
    let factory = make_factory();
    let db = factory.make_db_with_golden_test(true);
    let _scope = error_report::ItemScope::new(
        db.errors(),
        error_report::ItemName {
            name: "test_item".into(),
            id: TEST_ITEM_ID.as_u64(),
            unique_name: None,
            defining_target: None,
            source_loc: Some(TEST_ITEM_REPORT_SOURCE_LOC.into()),
        },
    );
    let actual = generate_unsupported(
        &db,
        db.new_unsupported_item_with_static_message(
            &TestItem { decl_loc: Some(test_decl_loc()), must_bind: false },
            /* path= */ None,
            "unsupported_message",
        )
        .into(),
    )
    .generated_items;
    let actual = code_snippet::generated_items_to_token_stream(&actual, &db, &[TEST_ITEM_ID]);
    let expected = "error: item `test_item` could not be bound\n  unsupported_message";
    assert_rs_matches!(quote! { #actual }, quote! { __COMMENT__ #expected});
    Ok(())
}

#[gtest]
fn test_generate_unsupported_function_template_with_global_cpp() -> Result<()> {
    let factory = make_factory();
    let db = factory.make_db();
    let _scope = error_report::ItemScope::new(
        db.errors(),
        error_report::ItemName {
            name: "test_item".into(),
            id: TEST_ITEM_ID.as_u64(),
            unique_name: None,
            defining_target: None,
            source_loc: Some(TEST_ITEM_REPORT_SOURCE_LOC.into()),
        },
    );
    let mut unsupported_item = db.new_unsupported_item_with_static_message(
        &TestItem { decl_loc: Some(test_decl_loc()), must_bind: false },
        /* path= */ None,
        "unsupported_message",
    );
    unsupported_item.set_inline_cpp_source_text(
        "template <typename T> T Add(T a, T b) { return a + b; }".into(),
    );
    let actual = generate_unsupported(&db, unsupported_item.into()).generated_items;
    let actual = code_snippet::generated_items_to_token_stream(&actual, &db, &[TEST_ITEM_ID]);
    let expected_comment = format!(
        "{}\nerror: item `test_item` could not be bound\n  unsupported_message",
        test_decl_loc()
    );
    let expected = quote! {
        __COMMENT__ #expected_comment
        ::inline_cpp_macro::global_cpp! {
            template <typename T>
            T Add(T a, T b) {
              return a + b;
            }
        }
    };
    assert_rs_matches!(quote! { #actual }, expected);
    assert!(factory.fatal_errors().take_string().is_empty());
    Ok(())
}

#[gtest]
fn test_generate_unsupported_function_template_strict_mode_fatal_error() -> Result<()> {
    let factory = make_factory();
    let db = factory.make_db();
    let _scope = error_report::ItemScope::new(
        db.errors(),
        error_report::ItemName {
            name: "test_item".into(),
            id: TEST_ITEM_ID.as_u64(),
            unique_name: None,
            defining_target: None,
            source_loc: Some(TEST_ITEM_REPORT_SOURCE_LOC.into()),
        },
    );
    let mut unsupported_item = db.new_unsupported_item_with_static_message(
        &TestItem { decl_loc: Some(test_decl_loc()), must_bind: true },
        /* path= */ None,
        "unsupported_message",
    );
    unsupported_item.set_inline_cpp_source_text(
        "template <typename T> T Add(T a, T b) { return a + b; }".into(),
    );
    let _ = generate_unsupported(&db, unsupported_item.into());
    assert!(!factory.fatal_errors().take_string().is_empty());
    Ok(())
}

#[gtest]
fn test_generate_unsupported_item_reports_cause() -> Result<()> {
    let factory = make_factory();
    let db = factory.make_db();
    let cause = arc_anyhow::Error::from(error_report::ErrorList::from(vec![
        error_report::anyhow!("first {}", "reason"),
        error_report::anyhow!("second reason"),
    ]));
    let actual = {
        let _scope = error_report::ItemScope::new(
            db.errors(),
            error_report::ItemName {
                name: "test_item".into(),
                id: TEST_ITEM_ID.as_u64(),
                unique_name: None,
                defining_target: None,
                source_loc: Some(TEST_ITEM_REPORT_SOURCE_LOC.into()),
            },
        );
        let unsupported_item = db.new_unsupported_item_with_cause(
            &TestItem { decl_loc: Some(test_decl_loc()), must_bind: false },
            /* path= */ None,
            cause,
        );
        generate_unsupported(&db, unsupported_item.into()).generated_items
    };

    // The comment shows the whole message, as before...
    let actual = code_snippet::generated_items_to_token_stream(&actual, &db, &[TEST_ITEM_ID]);
    let expected = format!(
        "{}\nerror: item `test_item` could not be bound\n  first reason\n  second reason",
        test_decl_loc()
    );
    assert_rs_matches!(quote! { #actual }, quote! { __COMMENT__ #expected});

    // ...but the report has each reason it was formatted from, with its own format string.
    let entries: Vec<error_report::ErrorReportEntry> =
        serde_json::from_str(&factory.errors().to_json_string())?;
    let errors: Vec<(&str, &str)> =
        entries[0].errors.iter().map(|e| (e.fmt(), e.full_error())).collect();
    assert_eq!(errors, [("first {}", "first reason"), ("second reason", "")]);
    Ok(())
}
