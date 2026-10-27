// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! End-to-end tests for the error report produced by `rs_bindings_from_cc`.

use arc_anyhow::Result;
use error_report::ErrorReportEntry;
use googletest::prelude::*;
use ir_testing::make_test_ir;
use multiplatform_ir_testing::ir_proto_from_cc;
use test_generators::generate_error_report_json_for_test;

/// Parses the JSON error report into its entries.
fn parse_report(json: &str) -> Vec<ErrorReportEntry> {
    serde_json::from_str(json).expect("error report should be valid JSON")
}

/// Returns the unique entry named `name`.
fn find_entry<'a>(entries: &'a [ErrorReportEntry], name: &str) -> &'a ErrorReportEntry {
    let mut matching = entries.iter().filter(|e| &*e.name == name);
    let entry = matching.next().unwrap_or_else(|| panic!("no entry named {name}: {entries:#?}"));
    assert!(matching.next().is_none(), "more than one entry named {name}: {entries:#?}");
    entry
}

#[gtest]
fn test_source_loc_is_reported() -> Result<()> {
    let proto = ir_proto_from_cc(
        r#"
        struct Supported {};
        inline void Unsupported(Supported&&, ...) {}
        "#,
    )?;
    let ir = make_test_ir(&proto)?;
    let entries = parse_report(&generate_error_report_json_for_test(ir)?);

    // Both items that received bindings and items that did not have a source location.
    // (`ir_proto_from_cc` prepends two lines to the header: an `#include` and a blank line.)
    expect_that!(
        find_entry(&entries, "Supported").source_loc.as_deref(),
        some(eq("ir_from_cc_virtual_header.h;l=4"))
    );
    expect_that!(
        find_entry(&entries, "Unsupported").source_loc.as_deref(),
        some(eq("ir_from_cc_virtual_header.h;l=5"))
    );
    Ok(())
}
