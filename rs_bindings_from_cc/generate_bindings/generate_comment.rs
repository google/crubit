// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Generate comments for the bindings.

use arc_anyhow::Result;
use database::code_snippet::ApiSnippets;
use database::{BindingsGenerator, Database};
use ffi_types::SourceLocationDocComment;
use ir::{Comment, GenericItem, UnsupportedItem, IR};
use proc_macro2::TokenStream;
use quote::quote;

/// Top-level comments that help identify where the generated bindings came
/// from.
pub fn generate_top_level_comment(ir: &IR) -> String {
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
pub fn generate_unsupported(db: &Database, item: &UnsupportedItem) -> Result<ApiSnippets> {
    for error in item.errors() {
        db.errors().report(error);
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
    Ok(ApiSnippets { main_api: quote! { __COMMENT__ #message }, ..Default::default() })
}

/// Generates Rust source code for a given `Comment`.
pub fn generate_comment(comment: &Comment) -> Result<ApiSnippets> {
    let text = comment.text.as_ref();
    Ok(quote! { __COMMENT__ #text }.into())
}
