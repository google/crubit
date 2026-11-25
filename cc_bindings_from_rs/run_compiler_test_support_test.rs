// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#![feature(rustc_private)]

use run_compiler_test_support::{find_def_id_by_name, run_compiler_for_testing};

#[test]
#[should_panic(expected = "Test input compilation failed while parsing")]
fn test_run_compiler_for_testing_panic_when_test_input_contains_syntax_errors() {
    run_compiler_for_testing("syntax error here", |_tcx| panic!("This part shouldn't execute"))
}

#[test]
#[should_panic(expected = "Test input compilation failed while analyzing")]
fn test_run_compiler_for_testing_panic_when_test_input_triggers_analysis_errors() {
    run_compiler_for_testing("#![feature(no_such_feature)]", |_tcx| {
        panic!("This part shouldn't execute")
    })
}

#[test]
#[should_panic(expected = "Test input compilation failed while linting")]
fn test_run_compiler_for_testing_panic_when_test_input_triggers_warnings() {
    run_compiler_for_testing("pub fn foo(unused_parameter: i32) {}", |_tcx| {
        panic!("This part shouldn't execute")
    })
}

#[test]
fn test_run_compiler_for_testing_nightly_features_ok_in_test_input() {
    // This test arbitrarily picks `yeet_expr` as an example of a feature that
    // hasn't yet been stabilized.
    let test_src = r#"
            // This test is supposed to test that *nightly* features are ok
            // in the test input.  The `forbid` directive below helps to
            // ensure that we'll realize in the future when the `yeet_expr`
            // feature gets stabilized, making it not quite fitting for use
            // in this test.
            #![forbid(stable_features)]

            #![feature(yeet_expr)]
        "#;
    run_compiler_for_testing(test_src, |_tcx| ())
}

#[test]
fn test_run_compiler_for_testing_stabilized_features_ok_in_test_input() {
    // This test arbitrarily picks `const_ptr_offset_from` as an example of a
    // feature that has been already stabilized.
    run_compiler_for_testing("#![feature(const_ptr_offset_from)]", |_tcx| ())
}

#[test]
#[should_panic(expected = "No items named `missing_name`.\n\
                           Instead found:\n`bar`,\n`foo`,\n`m1`,\n`m2`,\n`std`")]
fn test_find_def_id_by_name_panic_when_no_item_with_matching_name() {
    let test_src = r#"
            pub extern "C" fn foo() {}

            pub mod m1 {
                pub fn bar() {}
            }
            pub mod m2 {
                pub fn bar() {}
            }
        "#;
    run_compiler_for_testing(test_src, |tcx| find_def_id_by_name(tcx, "missing_name"));
}

#[test]
#[should_panic(expected = "More than one item named `some_name`")]
fn test_find_def_id_by_name_panic_when_multiple_items_with_matching_name() {
    let test_src = r#"
            pub mod m1 {
                pub fn some_name() {}
            }
            pub mod m2 {
                pub fn some_name() {}
            }
        "#;
    run_compiler_for_testing(test_src, |tcx| find_def_id_by_name(tcx, "some_name"));
}
