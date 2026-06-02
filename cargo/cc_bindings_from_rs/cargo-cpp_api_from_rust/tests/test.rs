// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
use std::path::PathBuf;
use std::process::Command;

fn setup_command(tmp_dir: &tempfile::TempDir, project_dir: &std::path::Path) -> Command {
    // Locate the cargo-cpp_api_from_rust binary.
    let binary_path = PathBuf::from(env!("CARGO_BIN_EXE_cargo-cpp_api_from_rust"));

    let orig_path = std::env::var("PATH").unwrap_or_default();
    let new_path = format!(
        "{}:{}",
        binary_path.parent().expect("Binary path should have a directory").display(),
        orig_path
    );

    let mut cmd = Command::new(env!("CARGO"));

    cmd.current_dir(project_dir);
    cmd.env("CARGO_TARGET_DIR", tmp_dir.path().display().to_string());
    cmd.env("PATH", new_path);
    cmd
}

#[test] // allow_core_test
fn test_subcommand_end_to_end() -> Result<(), Box<dyn std::error::Error>> {
    let tmp_dir = tempfile::tempdir()?;
    let cwd = std::env::current_dir()?;
    let project_dir = cwd.join("tests/test_project");

    let mut cmd = setup_command(&tmp_dir, &project_dir);
    cmd.arg("cpp_api_from_rust");

    let output = cmd.output().expect("Failed to execute");

    if !output.status.success() {
        println!("{}", String::from_utf8_lossy(&output.stdout));
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        panic!("cargo-cpp_api_from_rust failed");
    }

    // Verify output files
    let target_dir = tmp_dir.path();
    let debug_dir = target_dir.join("debug");
    let headers_dir = debug_dir.join("include").join("crubit");

    // We expect to find our generated bindings in the output and the final staticlib.
    assert!(headers_dir.join("test_project.h").exists());
    assert!(debug_dir.join("libtest_project.a").exists());

    Ok(())
}

#[test] // allow_core_test
fn test_subcommand_target_dir() -> Result<(), Box<dyn std::error::Error>> {
    let tmp_dir = tempfile::tempdir()?;
    let cwd = std::env::current_dir()?;
    let project_dir = cwd.join("tests/test_project");
    let explicit_target_dir = tmp_dir.path().join("explicit_target_dir");

    let mut cmd = setup_command(&tmp_dir, &project_dir);
    cmd.arg("cpp_api_from_rust");
    cmd.arg("--target-dir");
    cmd.arg(&explicit_target_dir);

    let output = cmd.output().expect("Failed to execute");

    if !output.status.success() {
        println!("{}", String::from_utf8_lossy(&output.stdout));
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        panic!("cargo-cpp_api_from_rust failed");
    }

    // Verify output files in the explicit target dir
    let debug_dir = explicit_target_dir.join("debug");
    let headers_dir = debug_dir.join("include").join("crubit");

    assert!(headers_dir.join("test_project.h").exists());
    assert!(debug_dir.join("libtest_project.a").exists());

    Ok(())
}

#[test] // allow_core_test
fn test_subcommand_build_args() -> Result<(), Box<dyn std::error::Error>> {
    let tmp_dir = tempfile::tempdir()?;
    let cwd = std::env::current_dir()?;
    let project_dir = cwd.join("tests/test_project");

    let mut cmd = setup_command(&tmp_dir, &project_dir);
    cmd.arg("cpp_api_from_rust");
    cmd.arg("--");
    cmd.arg("--profile=release");

    let output = cmd.output().expect("Failed to execute");

    if !output.status.success() {
        println!("{}", String::from_utf8_lossy(&output.stdout));
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        panic!("cargo-cpp_api_from_rust failed");
    }

    // Verify output files in the release directory
    let release_dir = tmp_dir.path().join("release");
    let headers_dir = release_dir.join("include").join("crubit");

    assert!(headers_dir.join("test_project.h").exists());
    assert!(release_dir.join("libtest_project.a").exists());

    Ok(())
}

#[test] // allow_core_test
fn test_subcommand_failing_project() -> Result<(), Box<dyn std::error::Error>> {
    let tmp_dir = tempfile::tempdir()?;
    let cwd = std::env::current_dir()?;
    let project_dir = cwd.join("tests/failing_project");

    let mut cmd = setup_command(&tmp_dir, &project_dir);
    cmd.arg("cpp_api_from_rust");

    let output = cmd.output().expect("Failed to execute");

    // The subcommand should fail.
    assert!(!output.status.success());

    // Verify output files are NOT generated
    let debug_dir = tmp_dir.path().join("debug");
    let headers_dir = debug_dir.join("include").join("crubit");

    assert!(!headers_dir.join("failing_project.h").exists());
    assert!(!debug_dir.join("libfailing_project.a").exists());

    Ok(())
}

#[test] // allow_core_test
fn test_subcommand_caching() -> Result<(), Box<dyn std::error::Error>> {
    let tmp_dir = tempfile::tempdir()?;
    let cwd = std::env::current_dir()?;
    let project_dir = cwd.join("tests/test_project");

    let mut cmd = setup_command(&tmp_dir, &project_dir);
    cmd.arg("cpp_api_from_rust");

    // First run
    let output = cmd.output().expect("Failed to execute");
    assert!(output.status.success());

    let debug_dir = tmp_dir.path().join("debug");
    // Second run - should be cached
    let mut cmd = setup_command(&tmp_dir, &project_dir);
    cmd.arg("cpp_api_from_rust");
    let output = cmd.output().expect("Failed to execute");
    assert!(output.status.success());

    let deps_dir = debug_dir.join("deps");
    let intermediate_h = std::fs::read_dir(&deps_dir)?
        .filter_map(|e| e.ok())
        .find(|e| {
            e.file_name().to_string_lossy().starts_with("test_project-")
                && e.file_name().to_string_lossy().ends_with(".h")
        })
        .unwrap()
        .path();

    let intermediate_mtime1 = std::fs::metadata(&intermediate_h)?.modified()?;

    let mut cmd = setup_command(&tmp_dir, &project_dir);
    cmd.arg("cpp_api_from_rust");
    cmd.output().expect("Failed to execute");

    let intermediate_mtime2 = std::fs::metadata(&intermediate_h)?.modified()?;

    assert_eq!(
        intermediate_mtime1, intermediate_mtime2,
        "Intermediate file was rewritten even though it should have been cached"
    );

    Ok(())
}

#[test] // allow_core_test
fn test_subcommand_with_dependency() -> Result<(), Box<dyn std::error::Error>> {
    let tmp_dir = tempfile::tempdir()?;
    let cwd = std::env::current_dir()?;
    let project_dir = cwd.join("tests/test_with_dependency");

    let mut cmd = setup_command(&tmp_dir, &project_dir);
    cmd.arg("cpp_api_from_rust");

    let output = cmd.output().expect("Failed to execute");

    if !output.status.success() {
        println!("{}", String::from_utf8_lossy(&output.stdout));
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        panic!("cargo-cpp_api_from_rust failed");
    }

    // Verify output files
    let target_dir = tmp_dir.path();
    let debug_dir = target_dir.join("debug");
    let headers_dir = debug_dir.join("include").join("crubit");

    // We expect to find headers for both the root crate and its dependency.
    assert!(headers_dir.join("test_with_dependency.h").exists());
    assert!(headers_dir.join("test_dependency.h").exists());

    // We expect the final staticlib for the root crate.
    assert!(debug_dir.join("libtest_with_dependency.a").exists());

    Ok(())
}

#[test] // allow_core_test
fn test_subcommand_with_proc_macro() -> Result<(), Box<dyn std::error::Error>> {
    let tmp_dir = tempfile::tempdir()?;
    let cwd = std::env::current_dir()?;
    let project_dir = cwd.join("tests/test_with_proc_macro");
    let target = "x86_64-unknown-linux-gnu";

    let mut cmd = setup_command(&tmp_dir, &project_dir);
    cmd.arg("cpp_api_from_rust");
    cmd.arg("--");
    cmd.arg("--target");
    cmd.arg(&target);

    let output = cmd.output().expect("Failed to execute");

    if !output.status.success() {
        println!("{}", String::from_utf8_lossy(&output.stdout));
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        panic!("cargo-cpp_api_from_rust failed");
    }

    // Verify output files
    let target_dir = tmp_dir.path();
    let debug_dir = target_dir.join(&target).join("debug");
    let headers_dir = debug_dir.join("include").join("crubit");

    // We expect to find headers for the root crate and its library dependency,
    // but not for the proc-macro crate.
    assert!(headers_dir.join("test_with_proc_macro.h").exists());
    assert!(headers_dir.join("lib_using_proc_macro.h").exists());
    assert!(!headers_dir.join("my_proc_macro.h").exists());

    // We expect the final staticlib for the root crate.
    assert!(debug_dir.join("libtest_with_proc_macro.a").exists());

    Ok(())
}

#[test] // allow_core_test
fn test_subcommand_different_build_configurations() -> Result<(), Box<dyn std::error::Error>> {
    let tmp_dir = tempfile::tempdir()?;
    let cwd = std::env::current_dir()?;
    let project_dir = cwd.join("tests/test_project");

    // First run: default build configuration (debug)
    let mut cmd1 = setup_command(&tmp_dir, &project_dir);
    cmd1.arg("cpp_api_from_rust");
    let output1 = cmd1.output().expect("Failed to execute");
    if !output1.status.success() {
        println!("{}", String::from_utf8_lossy(&output1.stdout));
        eprintln!("{}", String::from_utf8_lossy(&output1.stderr));
        panic!("cargo-cpp_api_from_rust failed on first run");
    }

    // Second run: different build configuration (release)
    let mut cmd2 = setup_command(&tmp_dir, &project_dir);
    cmd2.arg("cpp_api_from_rust");
    cmd2.arg("--");
    cmd2.arg("--profile=release");
    let output2 = cmd2.output().expect("Failed to execute");
    if !output2.status.success() {
        println!("{}", String::from_utf8_lossy(&output2.stdout));
        eprintln!("{}", String::from_utf8_lossy(&output2.stderr));
        panic!("cargo-cpp_api_from_rust failed on second run");
    }

    // Verify that the two build configurations produced two unique cargo projects
    // with different hash suffixes.
    let debug_deps_dir = tmp_dir.path().join("debug").join("deps");
    let release_deps_dir = tmp_dir.path().join("release").join("deps");

    let find_cargo_project = |dir: &std::path::Path| -> Result<String, Box<dyn std::error::Error>> {
        let projects: Vec<_> = std::fs::read_dir(dir)?
            .filter_map(|e| e.ok())
            .filter(|e| {
                let path = e.path();
                path.is_dir() && e.file_name().to_string_lossy().starts_with("test_project-")
            })
            .map(|e| e.file_name().to_string_lossy().to_string())
            .collect();
        assert_eq!(
            projects.len(),
            1,
            "Expected exactly 1 cargo project in {}, found {:?}",
            dir.display(),
            projects
        );
        Ok(projects[0].clone())
    };

    let project1 = find_cargo_project(&debug_deps_dir)?;
    let project2 = find_cargo_project(&release_deps_dir)?;

    let suffix1 = project1.strip_prefix("test_project-").unwrap();
    let suffix2 = project2.strip_prefix("test_project-").unwrap();

    assert_ne!(
        suffix1, suffix2,
        "Expected cargo projects to have different suffixes, but both were {}",
        suffix1
    );

    Ok(())
}
