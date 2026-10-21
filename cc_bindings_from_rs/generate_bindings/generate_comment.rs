// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Doc comments, source locations and attribute-derived tags (`[[deprecated]]`, `[[nodiscard]]`)
//! for the generated C++ API.

use database::BindingsGenerator;
use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::quote;
#[rustversion::since(2026-09-27)]
use rustc_attr_ir::{find_attr, AttributeKind};
#[rustversion::before(2026-09-27)]
use rustc_hir::{attrs::AttributeKind, find_attr};
use rustc_middle::ty::TyCtxt;
use rustc_span::def_id::DefId;
use rustc_span::symbol::Symbol;
use std::borrow::Cow;

pub(crate) struct MustUseAttr {
    pub reason: Option<Symbol>,
}

pub(crate) fn must_use_attr_of<'tcx>(tcx: TyCtxt<'tcx>, def_id: DefId) -> Option<MustUseAttr> {
    if !crubit_attr::supports_attrs(tcx.def_kind(def_id)) {
        return None;
    }
    find_attr!(
        tcx,
        def_id,
        AttributeKind::MustUse { reason, .. } => MustUseAttr { reason: *reason }
    )
}

/// Returns the C++ must_use tag for the item identified by `def_id`, or None if there is no such
/// tag.
pub(crate) fn generate_must_use_tag(tcx: TyCtxt, def_id: DefId) -> Option<TokenStream> {
    if let Some(must_use_attr) = must_use_attr_of(tcx, def_id) {
        let cc_must_use_tag = match must_use_attr.reason {
            None => quote! {[[nodiscard]]},
            Some(reason) => {
                let reason = reason.as_str();
                quote! {[[nodiscard(#reason)]]}
            }
        };
        return Some(cc_must_use_tag);
    }
    None
}

/// Returns the C++ deprecated tag for the item identified by `def_id`, if it is
/// deprecated. Otherwise, returns None.
pub(crate) fn generate_deprecated_tag(tcx: TyCtxt, def_id: DefId) -> Option<TokenStream> {
    #[rustversion::since(2026-09-27)]
    use rustc_attr_ir::{find_attr, AttributeKind};
    #[rustversion::before(2026-09-27)]
    use rustc_hir::{attrs::AttributeKind, find_attr};

    // If our definition does not support attributes, get_all_attrs will panic, so we check
    // beforehand and return None.
    if !crubit_attr::supports_attrs(tcx.def_kind(def_id)) {
        return None;
    }

    #[allow(deprecated)]
    let deprecation_attr = find_attr!(tcx.get_all_attrs(def_id), AttributeKind::Deprecated{deprecation, span} => (*deprecation, *span));

    if let Some((deprecation, _span)) = deprecation_attr {
        let cc_deprecated_tag = match deprecation.note {
            None => quote! {[[deprecated]]},
            Some(note_symbol) => {
                let note = note_symbol.as_str();
                quote! {[[deprecated(#note)]]}
            }
        };
        return Some(cc_deprecated_tag);
    }
    None
}

fn generate_kythe_doc_comment(
    db: &BindingsGenerator,
    def_id: DefId,
    doc_comment: String,
) -> TokenStream {
    // Always emit a new capture tag so we can unconditionally emit capture brackets around
    // identifiers. (Otherwise, we might accidentally associate a capture range with the wrong
    // capture tag; it's fine to emit capture tags that never capture anything.)
    let tcx = db.tcx();
    let def_span = tcx.def_ident_span(def_id).unwrap_or_else(|| tcx.def_span(def_id));
    let (start, end) = if def_span.is_dummy() {
        ("0".to_string(), "0".to_string())
    } else {
        // We're assuming that lo and hi are in the same source file.
        let sf = tcx.sess.source_map().lookup_source_file(def_span.lo());
        (
            sf.relative_position(def_span.lo()).0.to_string(),
            sf.relative_position(def_span.hi()).0.to_string(),
        )
    };
    let file_name =
        tcx.sess.source_map().span_to_filename(def_span).prefer_local_unconditionally().to_string();
    quote! { __CAPTURE_TAG__ #file_name #start #end __COMMENT__ #doc_comment}
}

const OUT_DIR_PREFIX: &str = "bazel-out/";
const BIN_DIR_PREFIX: &str = "bazel-bin";
const GENFILES_DIR_PREFIX: &str = "bazel-genfiles";

/// Normalizes source paths for generated files located under the build output directory by
/// stripping dynamic configuration directories (such as transition hashes) and
/// replacing them with stable `bin/` or `genfiles/` paths.
fn normalize_source_path(file_name: &str) -> Cow<'_, str> {
    if let Some((_config, subpath)) =
        file_name.strip_prefix(OUT_DIR_PREFIX).and_then(|rest| rest.split_once('/'))
    {
        if let Some(rel) = subpath.strip_prefix("bin/") {
            return Cow::Owned(format!("{BIN_DIR_PREFIX}/{rel}"));
        }
        if let Some(rel) = subpath.strip_prefix("genfiles/") {
            return Cow::Owned(format!("{GENFILES_DIR_PREFIX}/{rel}"));
        }
    }
    Cow::Borrowed(file_name)
}

pub(crate) fn generate_source_location(db: &BindingsGenerator, def_id: DefId) -> String {
    let tcx = db.tcx();
    let def_span = tcx.def_span(def_id);
    let rustc_span::FileLines { file, lines } = match tcx.sess.source_map().span_to_lines(def_span)
    {
        Ok(filelines) => filelines,
        Err(_) => return "unknown location".to_string(),
    };
    let file_name = file.name.prefer_local_unconditionally().to_string();
    // Virtual paths will have a "./" prefix that we don't want to display.
    let file_name = file_name.strip_prefix("./").unwrap_or(file_name.as_str());
    let file_name = normalize_source_path(file_name);

    // Note: line_index starts at 0, while most everything else starts indexing at 1.
    let line_number = (lines[0].line_index + 1).to_string();
    if let (Some(path_format), true) = (db.crubit_debug_path_format(), file.name.is_real()) {
        return path_format.format(&[file_name.as_ref(), line_number.as_str()]);
    }
    format!("{file_name};l={line_number}")
}

/// Formats the doc comment (if any) associated with the item identified by
/// `local_def_id`, and appends the source location at which the item is
/// defined.
pub(crate) fn generate_doc_comment(db: &BindingsGenerator, def_id: DefId) -> TokenStream {
    #[allow(deprecated)]
    let mut doc_comment: String = db
        .tcx()
        .get_all_attrs(def_id)
        .iter()
        .filter_map(|attr| attr.doc_str())
        .map(|symbol| symbol.to_string())
        .join("\n");

    if !db.is_golden_test() || db.kythe_annotations() {
        if !doc_comment.is_empty() {
            doc_comment.push_str("\n\n");
        }
        doc_comment.push_str("Generated from: ");
        doc_comment.push_str(&generate_source_location(db, def_id));
    }

    if db.kythe_annotations() {
        generate_kythe_doc_comment(db, def_id, doc_comment)
    } else if !doc_comment.is_empty() {
        quote! { __COMMENT__ #doc_comment}
    } else {
        quote! {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_source_path_bin() {
        assert_eq!(
            normalize_source_path(&format!(
                "{OUT_DIR_PREFIX}k8-fastbuild-ST-7be089cf1221/bin/foo/bar.rs"
            )),
            format!("{BIN_DIR_PREFIX}/foo/bar.rs")
        );
        assert_eq!(
            normalize_source_path(&format!(
                "{OUT_DIR_PREFIX}x86_64-opt/bin/third_party/rust/lib.rs"
            )),
            format!("{BIN_DIR_PREFIX}/third_party/rust/lib.rs")
        );
    }

    #[test]
    fn test_normalize_source_path_genfiles() {
        assert_eq!(
            normalize_source_path(&format!("{OUT_DIR_PREFIX}k8-fastbuild/genfiles/foo/bar.rs")),
            format!("{GENFILES_DIR_PREFIX}/foo/bar.rs")
        );
    }

    #[test]
    fn test_normalize_source_path_unmodified() {
        assert_eq!(
            normalize_source_path("test.rs"),
            "test.rs"
        );
        assert_eq!(normalize_source_path("<crubit_unittests.rs>"), "<crubit_unittests.rs>");
    }
}
