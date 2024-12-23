// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Generate comments for the bindings.

use crate::code_snippet::GeneratedItem;
use crate::db::{BindingsGenerator, Database};
use arc_anyhow::Result;
use ffi_types::SourceLocationDocComment;
use ir::{Comment, GenericItem, UnsupportedItem, IR};
use proc_macro2::TokenStream;
use quote::quote;
use std::rc::Rc;

/// Top-level comments that help identify where the generated bindings came
/// from.
pub fn generate_top_level_comment(ir: Rc<IR>) -> String {
    // The "@generated" marker is an informal convention for identifying
    // automatically generated code.  This marker is recognized by `rustfmt`
    // (see the `format_generated_files` option [1]) and some other tools.
    // For more info see https://generated.at/.
    //
    // [1]
    // https://rust-lang.github.io/rustfmt/?version=v1.4.38&search=#format_generated_files
    //
    // TODO(b/255784681): It would be nice to include "by $argv[0]"" in the
    // @generated comment below.  OTOH, `std::env::current_exe()` in our
    // current build environment returns a guid-like path... :-/
    //
    // TODO(b/255784681): Consider including cmdline arguments.
    let target = &ir.current_target().0;

    let crubit_features = {
        let mut crubit_features: Vec<&str> = ir
            .target_crubit_features(ir.current_target())
            .into_iter()
            .map(|feature| feature.short_name())
            .collect();
        crubit_features.sort();
        if crubit_features.is_empty() {
            "<none>".to_string()
        } else {
            crubit_features.join(", ")
        }
    };
    format!(
        "// Automatically @generated Rust bindings for the following C++ target:\n\
            // {target}\n\
            // Features: {crubit_features}\n"
    )
}

pub fn generate_doc_comment(
    comment: Option<&str>,
    source_loc: Option<&str>,
    generate_source_loc_doc_comment: SourceLocationDocComment,
) -> TokenStream {
    let source_loc = match generate_source_loc_doc_comment {
        SourceLocationDocComment::Enabled => source_loc,
        SourceLocationDocComment::Disabled => None,
    };
    let (comment, sep, source_loc) = match (comment, source_loc) {
        (None, None) => return quote! {},
        (None, Some(source_loc)) => ("", "", source_loc),
        (Some(comment), Some(source_loc)) => (comment, "\n\n", source_loc),
        (Some(comment), None) => (comment, "", ""),
    };
    // token_stream_printer (and rustfmt) don't put a space between /// and the doc
    // comment, let's add it here so our comments are pretty.
    let doc_comment = format!(" {comment}{sep}{source_loc}").replace('\n', "\n ");
    quote! {#[doc = #doc_comment]}
}

/// Generates Rust source code for a given `UnsupportedItem`.
pub fn generate_unsupported(db: &Database, item: &UnsupportedItem) -> Result<GeneratedItem> {
    for error in item.errors() {
        db.errors().insert(error);
    }

    let source_loc = item.source_loc();
    let source_loc = match &source_loc {
        Some(loc) if db.generate_source_loc_doc_comment() == SourceLocationDocComment::Enabled => {
            loc.as_ref()
        }
        _ => "",
    };

    let mut message = format!(
        "{source_loc}{}Error while generating bindings for item '{}':\n",
        if source_loc.is_empty() { "" } else { "\n" },
        item.name.as_ref(),
    );
    for (index, error) in item.errors().iter().enumerate() {
        message = format!("{message}{}{:#}", if index == 0 { "" } else { "\n\n" }, error);
    }
    Ok(GeneratedItem { item: quote! { __COMMENT__ #message }, ..Default::default() })
}

/// Generates Rust source code for a given `Comment`.
pub fn generate_comment(comment: &Comment) -> Result<GeneratedItem> {
    let text = comment.text.as_ref();
    Ok(quote! { __COMMENT__ #text }.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use arc_anyhow::Result;
    use error_report::ErrorReport;
    use googletest::prelude::*;
    use ir::ItemId;
    use ir_testing::make_ir_from_items;
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
        assert_rs_matches!(
            actual,
            quote! {#[doc = " Some doc comment\n \n google3/some/header;l=12"]}
        );
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
        let actual = generate_doc_comment(
            Some("Some doc comment"),
            None,
            SourceLocationDocComment::Disabled,
        );
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
    }

    #[gtest]
    fn test_generate_unsupported_item_with_source_loc_enabled() -> Result<()> {
        let db = Database::new(
            Rc::new(make_ir_from_items([])),
            Rc::new(ErrorReport::new()),
            SourceLocationDocComment::Enabled,
        );
        let actual = generate_unsupported(
            &db,
            &UnsupportedItem::new_with_static_message(
                &db.ir(),
                &TestItem { source_loc: Some("Generated from: google3/some/header;l=1".into()) },
                "unsupported_message",
            ),
        )?;
        let expected = "Generated from: google3/some/header;l=1\nError while generating bindings for item 'test_item':\nunsupported_message";
        assert_rs_matches!(actual.item, quote! { __COMMENT__ #expected});
        Ok(())
    }

    /// Not all items currently have source_loc(), e.g. comments.
    ///
    /// For these, we omit the mention of the location.
    #[gtest]
    fn test_generate_unsupported_item_with_missing_source_loc() -> Result<()> {
        let db = Database::new(
            Rc::new(make_ir_from_items([])),
            Rc::new(ErrorReport::new()),
            SourceLocationDocComment::Enabled,
        );
        let actual = generate_unsupported(
            &db,
            &UnsupportedItem::new_with_static_message(
                &db.ir(),
                &TestItem { source_loc: None },
                "unsupported_message",
            ),
        )?;
        let expected = "Error while generating bindings for item 'test_item':\nunsupported_message";
        assert_rs_matches!(actual.item, quote! { __COMMENT__ #expected});
        Ok(())
    }

    #[gtest]
    fn test_generate_unsupported_item_with_source_loc_disabled() -> Result<()> {
        let db = Database::new(
            Rc::new(make_ir_from_items([])),
            Rc::new(ErrorReport::new()),
            SourceLocationDocComment::Disabled,
        );
        let actual = generate_unsupported(
            &db,
            &UnsupportedItem::new_with_static_message(
                &db.ir(),
                &TestItem { source_loc: Some("Generated from: google3/some/header;l=1".into()) },
                "unsupported_message",
            ),
        )?;
        let expected = "Error while generating bindings for item 'test_item':\nunsupported_message";
        assert_rs_matches!(actual.item, quote! { __COMMENT__ #expected});
        Ok(())
    }
}
