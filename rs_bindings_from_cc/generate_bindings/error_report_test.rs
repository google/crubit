// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! End-to-end tests for the error report produced by `rs_bindings_from_cc`.

use arc_anyhow::Result;
use crubit_feature::CrubitFeature;
use error_report::{Blocker, ErrorReportEntry};
use googletest::prelude::*;
use ir::{BazelLabel, IR};
use ir_testing::{make_test_ir, DEPENDENCY_TARGET, TESTING_TARGET};
use multiplatform_ir_testing::{ir_proto_from_cc, ir_proto_from_cc_dependency};
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

/// Returns `(message, blocked_by)` for each error of `entry`.
fn errors_and_blockers(entry: &ErrorReportEntry) -> Vec<(String, Option<Blocker>)> {
    entry.errors.iter().map(|e| (e.to_string(), e.blocked_by().cloned())).collect()
}

/// Returns a `Blocker` for a struct named `name` defined in `target`.
fn struct_blocker(name: &str, target: &str) -> Blocker {
    Blocker {
        name: name.into(),
        unique_name: Some(format!("c:@S@{name}").into()),
        defining_target: Some(target.into()),
    }
}

/// Restricts the current target to the `supported` feature set, so that uses of types without
/// bindings are errors rather than being tolerated as `wrapper` features.
fn use_supported_features(ir: &mut IR) {
    let current_target = ir.current_target().clone();
    *ir.target_crubit_features_mut(&current_target) =
        CrubitFeature::Supported | CrubitFeature::Types;
}

/// Returns the error report for `header`, which uses `dep_header` from a dependency that does not
/// enable Crubit. The current target uses `supported` features.
fn report_with_dependency_without_crubit(
    header: &str,
    dep_header: &str,
) -> Result<Vec<ErrorReportEntry>> {
    let proto = ir_proto_from_cc_dependency(header, dep_header)?;
    let mut ir = make_test_ir(&proto)?;
    use_supported_features(&mut ir);
    *ir.target_crubit_features_mut(&BazelLabel::from(DEPENDENCY_TARGET)) = Default::default();
    Ok(parse_report(&generate_error_report_json_for_test(ir)?))
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

#[gtest]
fn test_blocked_by_target_without_crubit() -> Result<()> {
    let entries = report_with_dependency_without_crubit(
        r#"
        void TakesDepStruct(DepStruct* d);
        DepStruct* ReturnsDepStruct();
        "#,
        "struct DepStruct { int x; };",
    )?;

    for name in ["TakesDepStruct", "ReturnsDepStruct"] {
        expect_that!(
            errors_and_blockers(find_entry(&entries, name)),
            elements_are![(anything(), some(eq(&struct_blocker("DepStruct", DEPENDENCY_TARGET))))],
            "for {name}"
        );
    }
    Ok(())
}

#[gtest]
fn test_blocked_by_unsupported_type() -> Result<()> {
    let proto = ir_proto_from_cc(
        r#"
        struct __Hidden { int x; };
        struct Visible { int x; };
        void UseHidden(Visible* v, __Hidden* h, __Hidden* h2);
        "#,
    )?;
    let mut ir = make_test_ir(&proto)?;
    use_supported_features(&mut ir);
    let entries = parse_report(&generate_error_report_json_for_test(ir)?);

    // `Visible` has bindings, so it doesn't block anything; each `__Hidden` parameter does, and is
    // reported with the message for that parameter.
    let hidden = struct_blocker("__Hidden", TESTING_TARGET);
    expect_that!(
        errors_and_blockers(find_entry(&entries, "UseHidden")),
        elements_are![
            (starts_with("Unsupported parameter type `__Hidden* h`"), some(eq(&hidden))),
            (starts_with("Unsupported parameter type `__Hidden* h2`"), some(eq(&hidden))),
        ]
    );
    // `__Hidden` itself fails for an intrinsic reason.
    expect_that!(errors_and_blockers(find_entry(&entries, "__Hidden")), each((anything(), none())));
    Ok(())
}

#[gtest]
fn test_intrinsic_errors_are_not_blocked() -> Result<()> {
    let proto = ir_proto_from_cc("consteval int ConstevalFn() { return 0; }")?;
    let entries = parse_report(&generate_error_report_json_for_test(make_test_ir(&proto)?)?);

    expect_that!(
        errors_and_blockers(find_entry(&entries, "ConstevalFn")),
        elements_are![(eq("consteval functions are not supported"), none())]
    );
    Ok(())
}

#[gtest]
fn test_blocked_by_type_alias() -> Result<()> {
    let entries = report_with_dependency_without_crubit(
        r#"
        using Alias = DepStruct;
        void UseAlias(Alias* a);
        "#,
        "struct DepStruct { int x; };",
    )?;

    // The alias fails because of `DepStruct`, ...
    expect_that!(
        errors_and_blockers(find_entry(&entries, "Alias")),
        elements_are![(anything(), some(eq(&struct_blocker("DepStruct", DEPENDENCY_TARGET))))]
    );
    // ... and the function fails because of the alias, not (directly) because of `DepStruct`.
    expect_that!(
        errors_and_blockers(find_entry(&entries, "UseAlias")),
        elements_are![(
            anything(),
            some(eq(&Blocker {
                name: "Alias".into(),
                unique_name: Some("c:@Alias".into()),
                defining_target: Some(TESTING_TARGET.into()),
            }))
        )]
    );
    Ok(())
}

#[gtest]
fn test_blocked_by_error_type_blocker() -> Result<()> {
    let mut proto = ir_proto_from_cc("void TakesUnowned(int* t);")?;
    let mut items = proto.top_level_items.get(TESTING_TARGET).expect("target items").clone();
    let func = items
        .iter_mut()
        .find_map(|item| match item {
            ir::Item::Func(f)
                if f.rs_name().as_identifier().map(|i| i.identifier()) == Some("TakesUnowned") =>
            {
                Some(f)
            }
            _ => None,
        })
        .expect("TakesUnowned should exist");
    let mut error_type = ir::ErrorType::default();
    error_type.error.set_fmt("Unsupported type '$0': $1".into());
    error_type.error.set_message("Unsupported type 'Unowned<int>': unknown target".into());
    let mut blocker = ir::Blocker::default();
    blocker.set_name("Unowned".into());
    blocker.set_unique_name("c:@ST>1#T@Unowned".into());
    blocker.set_defining_target("//_unknown_target:test/unowned.h".into());
    error_type.set_blocked_by(blocker);
    let mut pointee = ir::CcType::default();
    pointee.set_variant(ir::CcTypeVariant::Error(ir::FfiRc::new(error_type)));
    let mut updated_func = (**func).clone();
    if let ir::CcTypeVariant::Pointer(ptr) = updated_func.params_mut()[0].type_mut().variant_mut() {
        ptr.set_pointee_type(pointee);
    } else {
        panic!("expected pointer parameter");
    }
    *func = ir::FfiRc::new(updated_func);
    proto.top_level_items.insert(TESTING_TARGET.into(), items);

    let entries = parse_report(&generate_error_report_json_for_test(make_test_ir(&proto)?)?);
    expect_that!(
        errors_and_blockers(find_entry(&entries, "TakesUnowned")),
        elements_are![(
            starts_with("Parameter #0 is not supported:"),
            some(eq(&Blocker {
                name: "Unowned".into(),
                unique_name: Some("c:@ST>1#T@Unowned".into()),
                defining_target: Some("//_unknown_target:test/unowned.h".into()),
            }))
        )]
    );
    Ok(())
}
