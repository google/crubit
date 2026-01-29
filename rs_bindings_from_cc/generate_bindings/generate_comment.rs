// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Generate comments for the bindings.

use database::code_snippet::{ApiSnippets, DocCommentAttr, GeneratedItem};
use database::BindingsGenerator;
use ffi_types::Environment;
use ir::{Comment, GenericItem, Item, UnsupportedItem, IR};
use std::collections::HashMap;
use std::fmt::Write as _;
use std::rc::Rc;

/// Top-level comments that help identify where the generated bindings came
/// from.
pub fn generate_top_level_comment(ir: &IR, environment: Environment) -> String {
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

    let mut result = format!(
        "// Automatically @generated Rust bindings for the following C++ target:\n\
            // {target}\n"
    );

    if environment == Environment::Production {
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

pub fn generate_doc_comment(
    comment: Option<&str>,
    safety: Option<&str>,
    source_loc: Option<&str>,
    environment: Environment,
) -> Option<DocCommentAttr> {
    let source_loc = match environment {
        Environment::Production => source_loc,
        Environment::GoldenTest => None,
    };

    // If a safety doc is provided, append a "# Safety" section to `comment`.
    let comment = if let Some(safety) = safety {
        let safety_comment = format!("# Safety\n\n{}", safety.trim());
        if let Some(comment) = comment {
            Some(format!("{comment}\n\n{safety_comment}"))
        } else {
            Some(safety_comment)
        }
    } else {
        comment.map(|s| s.to_owned())
    };

    let (comment, sep, source_loc) = match (comment, source_loc) {
        (None, None) => return None,
        (None, Some(source_loc)) => (String::new(), "", source_loc),
        (Some(comment), Some(source_loc)) => (comment, "\n\n", source_loc),
        (Some(comment), None) => (comment, "", ""),
    };
    // token_stream_printer (and rustfmt) don't put a space between /// and the doc
    // comment, let's add it here so our comments are pretty.
    Some(DocCommentAttr(format!(" {comment}{sep}{source_loc}").replace('\n', "\n ").into()))
}

/// Generates Rust source code for a given `UnsupportedItem`.
pub fn generate_unsupported(db: &dyn BindingsGenerator, item: Rc<UnsupportedItem>) -> ApiSnippets {
    Item::UnsupportedItem(item.clone()).assert_in_error_scope(db.ir(), db.errors());
    for error in item.errors() {
        db.errors().report(error);
    }

    let source_loc = item.source_loc();
    let source_loc = match &source_loc {
        Some(loc) if db.environment() == Environment::Production => loc.as_ref(),
        _ => "",
    };

    let mut message = String::new();
    if !source_loc.is_empty() {
        writeln!(&mut message, "{source_loc}").unwrap();
    }
    writeln!(
        &mut message,
        "Error while generating bindings for {} '{}':",
        item.unsupported_kind(),
        item.name.as_ref()
    )
    .unwrap();
    for (index, error) in item.errors().iter().enumerate() {
        if index != 0 {
            message.push_str("\n\n");
        }
        write!(&mut message, "{error:#}").unwrap();
    }

    if item.must_bind {
        db.fatal_errors().report(&message);
    }

    ApiSnippets {
        generated_items: HashMap::from([(
            item.id,
            GeneratedItem::Comment { message: message.into() },
        )]),
        ..Default::default()
    }
}

/// Generates Rust source code for a given `Comment`.
pub fn generate_comment(comment: Rc<Comment>) -> ApiSnippets {
    ApiSnippets {
        generated_items: HashMap::from([(
            comment.id,
            GeneratedItem::Comment { message: comment.text.clone() },
        )]),
        ..Default::default()
    }
}
