// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use error_report::{anyhow, ErrorReport, ErrorReporting};
use errors::Errors;
use googletest::prelude::*;

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
        serde_json::json!([
            {
                "name": "",
                "errors": [
                    { "fmt": "abc" },
                    { "fmt": "def" },
                ],
            },
        ]),
    );
    Ok(())
}

#[gtest]
#[should_panic]
fn test_errors_drop_with_unconsolidated_errors_panics() {
    let errors = Errors::new();
    errors.add(anyhow!("abc"));
}

#[gtest]
fn consume_error() {
    // With a non-error result we should get back Some, and no error should be added.
    {
        let errors = Errors::new();
        expect_eq!(Some(17), errors.consume_error::<u64>(Ok(17)));
        expect_eq!(Some(()), errors.consolidate().ok());
    }

    // With an error result we should get back None, and the error should be added.
    {
        let errors = Errors::new();
        expect_eq!(None, errors.consume_error::<u64>(Err(anyhow!(""))));
        expect_false!(errors.consolidate().is_ok());
    }
}
