// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
use std::path::PathBuf;
use std::process::Command;

#[test] // allow_core_test
fn test_subcommand_end_to_end() -> Result<(), Box<dyn std::error::Error>> {
    let tmp_dir = tempfile::tempdir()?;
    let cwd = std::env::current_dir()?;
    let project_dir = cwd.join("tests/test_project");

    // Locate the cargo-cpp_api_from_rust binary.
    let binary_path = PathBuf::from(env!("CARGO_BIN_EXE_cargo-cpp_api_from_rust"));

    let orig_path = std::env::var("PATH").unwrap_or_default();
    let new_path = format!(
        "{}:{}",
        binary_path.parent().expect("Binary path should have a directory").display(),
        orig_path
    );

    let mut cmd = Command::new(env!("CARGO"));

    cmd.current_dir(&project_dir);
    cmd.env("CARGO_TARGET_DIR", tmp_dir.path().display().to_string());
    cmd.env("PATH", new_path);
    cmd.arg("cpp_api_from_rust");

    let output = cmd.output().expect("Failed to execute");

    if !output.status.success() {
        println!("{}", String::from_utf8_lossy(&output.stdout));
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        panic!("cargo-cpp_api_from_rust failed");
    }

    // Verify output files
    let target_dir = tmp_dir.path();

    // We expect to find our generated bindings in the output and the final staticlib.
    assert!(target_dir.join("debug/test_project.h").exists());
    assert!(target_dir.join("debug/test_project_cc_api_impl.rs").exists());
    assert!(target_dir.join("debug/test_project.cpp").exists());
    assert!(target_dir.join("debug/libtest_project.a").exists());

    Ok(())
}
