// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#![feature(rustc_private)]

extern crate rustc_middle;

use rustc_middle::ty::TyCtxt;

use arc_anyhow::{bail, Context, Result};
use std::collections::{BTreeSet, HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::rc::Rc;

use cmdline::Cmdline;
use code_gen_utils::CcInclude;
use error_report::{ErrorReport, ErrorReporting, FatalErrors, ReportFatalError, SourceLanguage};
use generate_bindings::{BindingsGenerator, IncludeGuard};
use kythe_metadata::cc_embed_provenance_map;
use run_compiler::{run_compiler, run_compiler_with_input};
use token_stream_printer::{
    cc_tokens_to_formatted_string, cc_tokens_to_formatted_string_with_provenance,
    rs_tokens_to_formatted_string, RustfmtConfig,
};

fn turn_off_clang_format(mut cc_api: String) -> String {
    cc_api.insert_str(
        cc_api.find("#ifndef").unwrap_or_else(|| cc_api.find("#pragma once").unwrap()),
        "// clang-format off\n",
    );
    cc_api
}

fn write_file(path: &Path, content: &str) -> Result<()> {
    std::fs::write(path, content)
        .with_context(|| format!("Error when writing to {}", path.display()))
}

fn new_db<'tcx>(
    cmdline: &Cmdline,
    tcx: TyCtxt<'tcx>,
    errors: Rc<dyn ErrorReporting>,
    fatal_errors: Rc<dyn ReportFatalError>,
) -> BindingsGenerator<'tcx> {
    let mut crate_name_to_features =
        <HashMap<Rc<str>, flagset::FlagSet<crubit_feature::CrubitFeature>>>::new();
    for (crate_name, features) in &cmdline.crate_features {
        let accumulated_features = crate_name_to_features
            .entry(crate_name.as_str().into())
            .or_insert(cmdline.default_crate_features);
        *accumulated_features |= *features
    }
    for (crate_name, features) in &cmdline.crate_disabled_features {
        let accumulated_features = crate_name_to_features
            .entry(crate_name.as_str().into())
            .or_insert(cmdline.default_crate_features);
        *accumulated_features -= *features
    }
    let include_guard = if let Some(include_guard) = &cmdline.h_out_include_guard {
        IncludeGuard::Guard(include_guard.clone())
    } else {
        IncludeGuard::PragmaOnce
    };
    let mut crate_name_to_namespace = <HashMap<Rc<str>, Rc<str>>>::new();
    for (crate_name, namespace) in &cmdline.crate_namespaces {
        // TODO: Check dup.
        crate_name_to_namespace.insert(crate_name.as_str().into(), namespace.as_str().into());
    }
    let mut crate_renames = <HashMap<Rc<str>, Rc<str>>>::new();
    for (name, renamed) in &cmdline.crate_rename {
        crate_renames.insert(name.as_str().into(), renamed.as_str().into());
    }
    let mut crate_name_to_include_paths = <HashMap<Rc<str>, Vec<CcInclude>>>::new();
    for (crate_name, include_path) in &cmdline.crate_headers {
        let name =
            crate_renames.get(crate_name.as_str()).cloned().unwrap_or(crate_name.as_str().into());
        let paths = crate_name_to_include_paths.entry(name).or_default();
        paths.push(CcInclude::user_header(include_path.as_str().into()));
    }
    let mut ignore_symbols_from_files: HashSet<PathBuf> = HashSet::new();
    for file in &cmdline.ignore_symbols_from_files {
        ignore_symbols_from_files.insert(file.to_path_buf());
    }
    generate_bindings::new_database(
        tcx,
        cmdline.source_crate_name.as_ref().map(|s| s.clone().into()),
        cmdline.crubit_support_path_format.clone(),
        cmdline.crubit_debug_path_format.clone(),
        cmdline.default_crate_features,
        cmdline.kythe_annotations,
        cmdline.portable_abi_compatible,
        cmdline.enable_rmeta_interface,
        crate_name_to_include_paths.into(),
        crate_name_to_features.into(),
        crate_name_to_namespace.into(),
        crate_renames.into(),
        errors,
        fatal_errors,
        cmdline.is_golden_test,
        include_guard,
        ignore_symbols_from_files.into(),
    )
}

fn run_with_tcx(cmdline: &Cmdline, tcx: TyCtxt) -> Result<()> {
    use generate_bindings::{generate_bindings, BindingsTokens};

    let generate_error_report = cmdline.error_report_out.is_some();
    let (error_report, errors) =
        ErrorReport::new_rc_or_ignore(generate_error_report, SourceLanguage::Rust);
    let fatal_errors = Rc::new(FatalErrors::new());

    let BindingsTokens { mut cc_api, cc_api_impl } = {
        let db = new_db(cmdline, tcx, errors, fatal_errors.clone());
        generate_bindings(&db)?
    };

    if !cmdline.extra_rs_srcs_includes.is_empty() {
        let extra_includes: BTreeSet<CcInclude> = cmdline
            .extra_rs_srcs_includes
            .iter()
            .map(|include| CcInclude::exported_user_header(include.as_str().into()))
            .collect();
        let extra_includes_tokens = code_gen_utils::format_cc_includes(&extra_includes);
        cc_api = quote::quote! {
            #extra_includes_tokens
            #cc_api
        };
    }

    let fatal_error_message = fatal_errors.take_string();
    if !fatal_error_message.is_empty() {
        return Err(arc_anyhow::Error::msg(fatal_error_message));
    }

    if cmdline.kythe_annotations {
        let (cc_api, provenance_map) = cc_tokens_to_formatted_string_with_provenance(
            cc_api,
            cmdline.clang_format_exe_path.as_deref(),
        )?;
        let cc_api = turn_off_clang_format(cc_api);
        let cc_api = cc_embed_provenance_map(
            &provenance_map,
            cmdline
                .kythe_default_corpus
                .as_deref()
                .expect("kythe_default_corpus is required when kythe_annotations is enabled"),
            cc_api,
        );
        write_file(&cmdline.h_out, &cc_api)?;
    } else {
        let cc_api =
            cc_tokens_to_formatted_string(cc_api, cmdline.clang_format_exe_path.as_deref())?;
        let cc_api = turn_off_clang_format(cc_api);
        write_file(&cmdline.h_out, &cc_api)?;
    }

    // We need to touch this file or bazel will complain that we didn't create it.
    if let Some(cpp_out) = &cmdline.cpp_out {
        write_file(cpp_out, "")?;
    }

    {
        let rustfmt_config = cmdline.rustfmt_exe_path.as_ref().map(|rustfmt_path| {
            RustfmtConfig::new(rustfmt_path, cmdline.rustfmt_config_path.as_deref())
        });
        let cc_api_impl = rs_tokens_to_formatted_string(cc_api_impl, rustfmt_config.as_ref())?;
        write_file(&cmdline.rs_out, &cc_api_impl)?;
    }

    if let Some(error_report_out) = &cmdline.error_report_out {
        write_file(error_report_out, &error_report.unwrap().to_json_string())?;
    }

    Ok(())
}

/// Generates bindings using rmeta input files rather than source files and rustc args.
/// This function exists so we can guard it behind a feature flag and should be cleaned up once
/// we've migrated to rmetas entirely.
fn run_with_rmetas(cmdline: &Cmdline) -> Result<()> {
    let mut at_args = vec!["rustc".to_string()];

    for (crate_name, metadata) in &cmdline.r#extern {
        at_args.push(format!("--extern={crate_name}={metadata}"));
    }

    if let Some(target) = &cmdline.target {
        at_args.push(format!("--target={}", target));
    }

    if let Some(sysroot) = &cmdline.sysroot {
        at_args.push(format!("--sysroot={}", sysroot));
    }

    for library_path in &cmdline.library_dirs {
        at_args.push(format!("-L{}", library_path));
    }

    let Some(crate_name) = cmdline.source_crate_name.as_ref() else {
        bail!("--source-crate-name must be provided when using rmetas")
    };
    at_args.push("--crate-type=lib".to_string());

    // Tell rustc to expect an input file.
    // We won't pass a real file, we just need to pass CLI validation, so we can override the
    // input in our callback in run_compiler. We cannot use `-` because then rustc sometimes hangs,
    // waiting for stdin.
    at_args.push("unused_this_does_not_exist.rs".to_string());

    let compiler_args = at_args;

    let mut crate_attrs = Vec::new();

    let probe_input =
        format!("#![feature(no_core)]\n#![no_std]\n#![no_core]\nextern crate r#{};", crate_name);
    // Because this is just to probe for attributes on our underlying crate, we are okay to ignore
    // the result of the compilation. Any errors will be reported below.
    let _ = run_compiler_with_input(&compiler_args, probe_input, |tcx| {
        let used_crates = tcx.used_crates(());
        let has_crate =
            |name: &str| used_crates.iter().any(|&c| tcx.crate_name(c).as_str() == name);

        if !has_crate(crate_name) {
            // If we can't load the metadata for the crate, we won't be able to generate
            // bindings for it and we'll generate the relevant error below.
            return Ok(());
        }

        // Determine `#![no_std]` and `#![no_core]` by checking if `std` or `core`
        // are present in the crate's dependency graph.
        let has_no_std = crate_name == "core"
            || crate_name == "alloc"
            || (!has_crate("std") && crate_name != "std");

        let has_no_core = crate_name == "core"
            || crate_name == "compiler_builtins"
            || (!has_crate("core") && crate_name != "core");

        if has_no_std {
            crate_attrs.push("#![no_std]");
        }
        if has_no_core {
            crate_attrs.extend([
                "#![allow(internal_features)]",
                "#![feature(no_core)]",
                "#![no_core]",
                "#![no_std]",
            ]);
        }
        Ok(())
    });

    let mut input_str = crate_attrs.join("\n");
    if crate_name != "std" {
        input_str.push_str(&format!("\n\nextern crate r#{};\n", crate_name));
    }

    if let Err(e) =
        run_compiler_with_input(&compiler_args, input_str, |tcx| run_with_tcx(cmdline, tcx))
    {
        let bold = "\x1B[1m";
        let italic = "\x1B[3m";
        let reset = "\x1B[0m";
        let red = "\x1B[31m";
        bail!(
            "{bold}Crubit {italic}{red}failed{reset} to generate C++ bindings for Rust crate \
            {bold}`{crate_name}`{reset} due to the following errors:\n{e:#}"
        );
    }
    Ok(())
}

/// Main entrypoint that (unlike `main`) doesn't do any intitializations that
/// should only happen once for the binary (e.g. it doesn't call
/// `init_env_logger`) and therefore can be used from unit tests.
pub fn run_with_cmdline_args(cmdline: &Cmdline) -> Result<()> {
    if cmdline.enable_rmeta_interface {
        run_with_rmetas(cmdline)
    } else {
        run_compiler(&cmdline.rustc_args, |tcx| run_with_tcx(cmdline, tcx))
    }
}
