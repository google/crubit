// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Reporting of items that cannot receive bindings: the `error_scope!` that attributes errors to
//! the item being generated, the "unsupported" placeholder snippets emitted in their place, and
//! the hard error for `#[crubit_annotate::must_bind]` items.

#![feature(cfg_accessible)]
#![feature(stmt_expr_attributes)]
#![feature(proc_macro_hygiene)]
#![feature(rustc_private)]

extern crate rustc_data_structures;
extern crate rustc_middle;
extern crate rustc_span;
use arc_anyhow::Error;
use database::code_snippet::CcSnippet;
use database::BindingsGenerator;
use generate_comment::generate_source_location;
use quote::quote;
use rustc_middle::ty;
use rustc_span::def_id::DefId;

pub fn non_lifetime_args<'tcx>(args: &'tcx [ty::GenericArg<'tcx>]) -> &'tcx [ty::GenericArg<'tcx>] {
    if args.iter().any(|arg| !matches!(arg.kind(), ty::GenericArgKind::Lifetime(_))) {
        args
    } else {
        &[]
    }
}

#[doc(hidden)]
pub fn item_name_for_error_report<'tcx>(
    db: &BindingsGenerator<'tcx>,
    def_id: DefId,
    args: &'tcx [ty::GenericArg<'tcx>],
) -> error_report::ItemName {
    let tcx = db.tcx();
    let args = non_lifetime_args(args);
    let crate_name = tcx.crate_name(def_id.krate);
    let def_path = tcx.def_path_str_with_args(def_id, args);
    let name = if def_id.is_local() {
        format!("{crate_name}::{def_path}").into()
    } else {
        def_path.into()
    };
    let id = if args.is_empty() {
        ((def_id.index.as_u32() as u64) << 32) | def_id.krate.as_u32() as u64
    } else {
        #[cfg_accessible(rustc_data_structures::stable_hash)]
        use rustc_data_structures::stable_hash;
        #[cfg_accessible(rustc_data_structures::stable_hasher)]
        use rustc_data_structures::stable_hasher as stable_hash;

        tcx.with_stable_hashing_context(|mut hcx| {
            let mut hasher = stable_hash::StableHasher::new();
            let key = (def_id, args);

            #[cfg_accessible(stable_hash::HashStable)]
            stable_hash::HashStable::hash_stable(&key, &mut hcx, &mut hasher);
            #[cfg_accessible(stable_hash::StableHash)]
            stable_hash::StableHash::stable_hash(&key, &mut hcx, &mut hasher);

            hasher
                .finish::<rustc_data_structures::fingerprint::Fingerprint>()
                .to_smaller_hash()
                .as_u64()
        })
    };
    let defining_target = if def_id.krate == db.source_crate_num() {
        None
    } else {
        Some(crate_name.to_string().into())
    };
    error_report::ItemName { name, id, unique_name: None, defining_target }
}

#[doc(hidden)]
pub fn report_must_bind_error<'tcx>(
    db: &BindingsGenerator<'tcx>,
    def_id: DefId,
    args: &'tcx [ty::GenericArg<'tcx>],
    err: &Error,
) {
    let tcx = db.tcx();
    let args = non_lifetime_args(args);
    let item_path = tcx.def_path_str_with_args(def_id, args);
    let bold = "\x1B[1m";
    let reset = "\x1B[0m";
    let red = "\x1B[31m";
    let must_bind_message = format!(
        "{bold}{red}error:{reset}{bold}{}\n\
        {bold}note:{reset} hard error because `#[crubit_annotate::must_bind]` was applied to `{item_path}`",
        unsupported_def_error_message(db, def_id, args, err, ErrorStyle::Terminal),
    );
    db.fatal_errors().report(&must_bind_message);
}

#[macro_export]
macro_rules! error_scope {
    ($db:expr, $def_id:expr) => {
        $crate::error_scope!($db, $def_id, &[]);
    };
    ($db:expr, $def_id:expr, $args:expr) => {
        let db = $db;
        let errors = db.errors();
        let _error_scope = error_report::ItemScope::new(
            &*errors,
            $crate::item_name_for_error_report(db, $def_id, $args),
        );
    };
}

pub enum ErrorStyle {
    // This message will be formatted as a comment.
    Comment,
    // This message will be printed to a terminal.
    Terminal,
}

pub fn unsupported_def_error_message<'tcx>(
    db: &BindingsGenerator<'tcx>,
    def_id: DefId,
    args: &'tcx [ty::GenericArg<'tcx>],
    err: &Error,
    style: ErrorStyle,
) -> String {
    let tcx = db.tcx();
    let args = non_lifetime_args(args);
    let source_loc = generate_source_location(db, def_id);
    let name = tcx.def_path_str_with_args(def_id, args);
    let def_kind = tcx.def_kind(def_id);
    let kind_str = def_kind.descr(def_id);
    let err_msg = if let ErrorStyle::Terminal = style {
        format!("\n  {err:#}").replace('\n', "\n  ")
    } else {
        format!(" {err:#}")
    };
    let bold = if let ErrorStyle::Terminal = style { "\x1B[1m" } else { "" };
    let reset = if let ErrorStyle::Terminal = style { "\x1B[0m" } else { "" };

    // https://docs.rs/anyhow/latest/anyhow/struct.Error.html#display-representations
    // says: To print causes as well [...], use the alternate selector “{:#}”.
    format!("{bold}{kind_str} `{name}` defined at {source_loc}:{reset}{err_msg}")
}

/// Formats a C++ comment explaining why no bindings have been generated for
/// `local_def_id`.
pub fn generate_unsupported_def<'tcx>(
    db: &BindingsGenerator<'tcx>,
    def_id: DefId,
    err: Error,
) -> CcSnippet<'tcx> {
    generate_unsupported_def_with_args(db, def_id, &[], err)
}

#[doc(hidden)]
pub fn generate_unsupported_def_with_args<'tcx>(
    db: &BindingsGenerator<'tcx>,
    def_id: DefId,
    args: &'tcx [ty::GenericArg<'tcx>],
    err: Error,
) -> CcSnippet<'tcx> {
    db.errors().assert_in_item(item_name_for_error_report(db, def_id, args));
    db.errors().report(&err);
    let msg = format!(
        "Error generating bindings for {}",
        unsupported_def_error_message(db, def_id, args, &err, ErrorStyle::Comment)
    );
    CcSnippet::new(quote! { __NEWLINE__ __NEWLINE__ __COMMENT__ #msg __NEWLINE__ })
}
