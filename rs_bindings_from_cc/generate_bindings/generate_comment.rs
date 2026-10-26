// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Generate comments for the bindings.

use database::code_snippet::{ApiSnippets, DocCommentAttr, GeneratedItem};
use database::{intern, BindingsGenerator};
use ir::{Comment, DeclLocation, GenericItem, UnsupportedItem, IR};
use std::collections::HashMap;
use std::fmt::Write as _;
use std::rc::Rc;

/// Top-level comments that help identify where the generated bindings came
/// from.
pub fn generate_top_level_comment(ir: &IR, is_golden_test: bool) -> String {
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
    let target = ir.current_target().as_str();

    let mut result = format!(
        "// Automatically @generated Rust bindings for the following C++ target:\n\
            // {target}\n"
    );

    if !is_golden_test {
        // Write the features.
        result.push_str(
            "\
            // Features: ",
        );

        let mut crubit_features: Vec<&str> = ir
            .target_crubit_features(ir.current_target())
            .into_iter()
            .map(|feature| feature.short_name())
            .collect();
        crubit_features.sort();

        if let Some((last_feature, features)) = crubit_features.split_last() {
            for feature in features {
                result.push_str(feature);
                result.push_str(", ");
            }
            result.push_str(last_feature);
        } else {
            result.push_str("<none>");
        }
        result.push('\n');
    }
    result
}

/// Returns the file and the byte offsets of the name of the declaration at `decl_loc`, for Kythe
/// annotations, or None if they aren't available.
///
/// For a declaration produced by a macro expansion, this is the expansion location with offsets
/// `0, 0`.
pub fn kythe_name_location<'pb>(
    decl_loc: Option<DeclLocation<'pb>>,
) -> Option<(&'pb str, u32, u32)> {
    // Open-source builds don't emit Kythe byte offsets.
    let _ = decl_loc;
    None
}

pub fn generate_doc_comment(
    comment: Option<&str>,
    safety: Option<&str>,
    mut decl_loc: Option<DeclLocation<'_>>,
    is_golden_test: bool,
    kythe_annotations: bool,
) -> Option<DocCommentAttr> {
    if is_golden_test && !kythe_annotations {
        decl_loc = None;
    }

    if comment.is_none() && safety.is_none() && decl_loc.is_none() {
        return None;
    }

    // The doc comment is made of the sections that are present, separated by blank lines.
    // token_stream_printer (and rustfmt) don't put a space between /// and the doc
    // comment, so start with one to make our comments pretty.
    let mut doc = String::from(" ");
    let mut separator = "";
    if let Some(comment) = comment {
        doc.push_str(comment);
        separator = "\n\n";
    }
    if let Some(safety) = safety {
        write!(doc, "{separator}# Safety\n\n{}", safety.trim()).unwrap();
        separator = "\n\n";
    }
    if let Some(decl_loc) = decl_loc {
        write!(doc, "{separator}{decl_loc}").unwrap();
    }
    Some(DocCommentAttr(doc.replace('\n', "\n ").into()))
}

/// Generates Rust source code for a given `UnsupportedItem`.
pub fn generate_unsupported<'db>(
    db: &BindingsGenerator<'db>,
    item: Rc<UnsupportedItem<'db>>,
) -> ApiSnippets {
    db.assert_in_error_scope(item.id());

    // Avoid generating unsupported item comments for standard library templates.
    if !item.must_bind() {
        let defined_in_libcxx = item.decl_loc().is_some_and(|decl_loc| {
            [decl_loc.spelling(), decl_loc.expansion()]
                .into_iter()
                .flatten()
                .any(|location| location.path().contains("libcxx"))
        });
        if defined_in_libcxx {
            return ApiSnippets::default();
        }
    }

    for error in db.unsupported_item_errors(Rc::clone(&item)).iter() {
        db.errors().report(error);
    }

    let source_loc = item.decl_loc().map(|decl_loc| decl_loc.to_string());
    let source_loc = match source_loc.as_deref() {
        Some(loc) if !db.is_golden_test() => loc,
        _ => "",
    };

    let mut message = String::new();
    if !source_loc.is_empty() {
        writeln!(&mut message, "{source_loc}").unwrap();
    }

    let (bold, reset, red) = if item.must_bind() {
        // If an item is set to `must_bind`, we colorize the output, as it will appear in the
        // terminal or logs rather than in a comment.
        ("\x1B[1m", "\x1B[0m", "\x1B[31m")
    } else {
        ("", "", "")
    };

    writeln!(
        &mut message,
        "{bold}{red}error:{reset}{bold} {} `{}` could not be bound{reset}",
        item.unsupported_kind(),
        item.name()
    )
    .unwrap();
    for (index, error) in db.unsupported_item_errors(Rc::clone(&item)).iter().enumerate() {
        if index != 0 {
            message.push_str("\n\n");
        }
        // Add indentation to child error output.
        write!(&mut message, "{}", format!("  {error:#}").replace('\n', "\n  ")).unwrap();
    }

    if item.must_bind() {
        db.fatal_errors().report(&message);
    }

    // Note: `item.inline_cpp_source_text()` is populated by the C++ importer when `carcinize` is
    // enabled.
    let generated_item = if let Some(parsed_tokens) = item.source_text_as_token_stream() {
        GeneratedItem::GlobalCpp(quote::quote! {
            __COMMENT__ #message
            ::inline_cpp_macro::global_cpp! {
                #parsed_tokens
            }
        })
    } else {
        GeneratedItem::Comment { message: intern!(db.interner(), "{message}") }
    };

    ApiSnippets {
        generated_items: HashMap::from([(item.id(), generated_item)]),
        ..Default::default()
    }
}

/// Generates Rust source code for a given `Comment`.
pub fn generate_comment(comment: Rc<Comment>) -> ApiSnippets {
    ApiSnippets {
        generated_items: HashMap::from([(
            comment.id(),
            GeneratedItem::Comment { message: Rc::from(comment.text()) },
        )]),
        ..Default::default()
    }
}
