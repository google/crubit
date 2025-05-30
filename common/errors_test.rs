// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use error_report::{anyhow, ErrorReport, ErrorReporting};
use errors::Errors;
use googletest::{expect_eq, fail, gtest, OrFail};

#[gtest]
fn test_errors_consolidate_on_empty_list_returns_ok() -> googletest::Result<()> {
    let errors = Errors::new();
    errors.consolidate().or_fail()?;
    Ok(())
}

#[gtest]
fn test_errors_consolidate_on_nonempty_list_returns_reportable_error() -> googletest::Result<()> {
    let errors = Errors::new();
    errors.add(anyhow!("abc"));
    errors.add(anyhow!("def"));
    let Err(error) = errors.consolidate() else {
        return fail!();
    };

    let report = ErrorReport::new();
    report.report(&error.into());
    expect_eq!(
        serde_json::from_str::<serde_json::Value>(&report.to_json_string()).unwrap(),
        serde_json::json!({
            "abc": { "count": 1 },
            "def": { "count": 1 },
        }),
    );
    Ok(())
}

#[gtest]
#[should_panic]
fn test_errors_drop_with_unconsolidated_errors_panics() {
    let errors = Errors::new();
    errors.add(anyhow!("abc"));
}
