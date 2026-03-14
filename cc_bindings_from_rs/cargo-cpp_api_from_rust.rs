// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! This cargo subcommand generates C++ bindings for a given Cargo project.
//! Expectation is that it is invoked within a cargo project directory via:
//!
//! ```shell
//! $ cargo cpp_api_from_rust \
//!   --<cargo-flag> \
//!   ...
//! ```
//! outputting a static library and header file that contain the generated C++ bindings.
//! These outputs can be consumed by a C++ build system (e.g. CMake) to compile Rust into a C++
//! codebase.
//!
//! The cargo project being built must specify a single library target in the root manifest.
//! Generating bindings for multiple packages, such as in a workspace, is not supported. The
//! subcommand will build your target Rust crate, failing with the compiler error if it fails to
//! build.

#![feature(rustc_private)]
#![feature(never_type)]

use arc_anyhow::{anyhow, bail, Error, Result};
use cargo_metadata::camino::Utf8PathBuf;
use cargo_metadata::Message;
use clap::Parser;
use cmdline::Cmdline;

use std::env;
use std::ffi;
use std::path::PathBuf;
use std::process::{Command, Stdio};

#[derive(Debug, Parser)]
#[clap(name = "cargo-cpp_api_from_rust")]
#[clap(about = "Generates C++ bindings for a Cargo project", long_about = None)]
#[command(styles = clap_cargo::style::CLAP_STYLING)]
struct Cli {
    #[command(flatten)]
    manifest: clap_cargo::Manifest,
    #[command(flatten)]
    features: clap_cargo::Features,
    #[arg(long, value_name = "DIRECTORY")]
    target_dir: Option<Utf8PathBuf>,
    #[arg(last = true)]
    build_args: Vec<String>,
}

// TODO(b/448731652): This should support passthrough for arbitrary cargo commandline flags that
// one might want to specify. This is important both for building target package initially, and
// building the final static lib at the end.
fn main() -> Result<()> {
    //! This subcommand executes three subtasks to generate bindings:
    //!
    //! 1. Build the target Rust crate, outputting an rlib file.
    //! 2. Run `cpp_api_from_rust` on the rlib file, outputting C++, rs, and h files.
    //! 3. Build a static lib out of the rs file generated in step 2.
    let mut env_args = std::env::args();
    env_args.next(); // Remove the binary name.
    let cli = Cli::try_parse_from(env_args)?;
    let mut metadata = cli.manifest.metadata();
    cli.features.forward_metadata(&mut metadata);
    let metadata = metadata.exec()?;

    let root = metadata.root_package().ok_or_else(|| anyhow!("Failed to find root package"))?;

    let edition = root.edition;
    let source_crate_name: &str = root.name.as_ref();

    let mut args = vec![
        // Make up a binary name for our call. This will get removed by Cmdline, so it doesn't matter
        // but it must be here otherwise a real cmdline arg will get ate.
        "cpp_api_from_rust".to_string(),
        format!("--source-crate-name={}", source_crate_name),
        "--crubit-support-path-format=<{header}>".to_string(),
        "--enable-rmeta-interface".to_string(),
    ];

    let mut build_args = vec![];
    let mut target_dir = &metadata.target_directory;
    if let Some(cli_target_dir) = &cli.target_dir {
        build_args.push(format!("--target-dir={}", cli_target_dir));
        target_dir = cli_target_dir;
    }
    if let Some(manifest_path) = &cli.manifest.manifest_path {
        build_args.push(format!("--manifest-path={}", manifest_path.display()));
    }
    if cli.features.all_features {
        build_args.push("--all-features".to_string());
    }
    if cli.features.no_default_features {
        build_args.push("--no-default-features".to_string());
    }
    if !cli.features.features.is_empty() {
        build_args.push(format!("--features={}", cli.features.features.join(",")));
    }
    build_args.extend(cli.build_args);

    // Step 1: Build the target Rust crate, outputting an rlib file.
    let mut build_command = Command::new(cargo_bin());
    build_command.args(["build", "--message-format=json"]);
    build_command.args(&build_args);

    let mut command = build_command
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|err| anyhow!("Failed to spawn cargo: {}", err))?;

    let mut profile_dir = None;
    let mut source_crate_rlib = None;
    let reader = std::io::BufReader::new(
        command.stdout.take().ok_or_else(|| anyhow!("Failed to open cargo stdout"))?,
    );
    for message in cargo_metadata::Message::parse_stream(reader) {
        let message = message.map_err(|err| anyhow!("Failed to parse cargo message: {}", err))?;
        let Message::CompilerArtifact(artifact) = message else {
            // We only care about compiler artifacts. Skip other messages, but display any text
            // output from the build.
            let output = match message {
                Message::TextLine(line) => line,
                Message::CompilerMessage(msg) => format!("{}", msg),
                _ => continue,
            };
            eprintln!("{}", output);
            continue;
        };
        let find_metadata_file = artifact.filenames.iter().find(|filename| {
            filename.extension().is_some_and(|ext| ext == "rmeta" || ext == "rlib")
        });
        let Some(filename) = find_metadata_file else {
            continue;
        };
        let name = artifact.target.name;
        if name == source_crate_name {
            profile_dir = filename
                .strip_prefix(&target_dir)
                .ok()
                .and_then(|path| path.components().next())
                .map(|component| component.as_str().to_owned());
            source_crate_rlib = artifact
                .filenames
                .iter()
                .find(|filename| filename.extension().is_some_and(|ext| ext == "rlib"))
                .map(|filename| filename.as_str().to_owned());
        }
        args.push(format!("--extern={}={}", name, filename));
    }

    let (Some(source_crate_rlib), Some(profile_dir)) = (source_crate_rlib, profile_dir) else {
        bail!(
            "Failed to find profile directory or rlib for target crate. Most likely \
        cause is cargo failed to build the specified target."
        );
    };

    let out_dir = target_dir.join(&profile_dir);

    args.extend([
        format!("--h-out={}", out_dir.join(format!("{}.h", source_crate_name))),
        format!("--rs-out={}", out_dir.join(format!("{}_cc_api_impl.rs", source_crate_name))),
        format!("--cpp-out={}", out_dir.join(format!("{}.cpp", source_crate_name))),
        format!("-Ldependency={}", out_dir.join("deps")),
    ]);

    // Wait on `cargo build` to finish to ensure all our files are generated.
    let cargo_output =
        command.wait_with_output().map_err(|err| anyhow!("Failed to wait for cargo: {}", err))?;
    if !cargo_output.status.success() {
        println!("{}", String::from_utf8_lossy(&cargo_output.stdout));
        eprintln!("{}", String::from_utf8_lossy(&cargo_output.stderr));
        bail!("cargo build failed");
    } else {
        println!("cargo suceeded");
    }

    // Step 2: Run `cpp_api_from_rust` on the rlib file, outputting C++, rs, and h files.
    Cmdline::new(&args)
        .map_err(|err| err.into())
        .and_then(|cmdline| cpp_api_from_rust_lib::run_with_cmdline_args(&cmdline))
        .map_err(|err: Error| match err.downcast_ref::<clap::Error>() {
            // Explicitly call `clap::Error::exit`, because 1) it results in *colored* output and
            // 2) it uses a zero exit code for specific "errors" (e.g. for `--help` output).
            Some(clap_err) => {
                let _: ! = clap_err.exit();
            }

            // Return `err` from `main`.  This will print the error message (no color codes
            // though) and terminate the process with a non-zero exit code.
            None => err,
        })?;

    // Step 3: Build a static lib out of the rs file generated in step 2.
    let mut rustc_args = vec![
        "rustc".to_string(),
        out_dir.join(format!("{}_cc_api_impl.rs", source_crate_name)).to_string(),
        format!("--edition={}", edition),
        format!("--crate-name={}_cc_api", source_crate_name),
        "--crate-type=staticlib".to_string(),
        format!("--extern={}={}", source_crate_name, source_crate_rlib),
        format!("-Ldependency={}", out_dir.join("deps")),
        "-o".to_string(),
        out_dir.join(format!("lib{}.a", source_crate_name)).to_string(),
    ];
    cpp_api_from_rust_lib::run_rustc(&rustc_args);

    Ok(())
}

fn cargo_bin() -> &'static ffi::OsStr {
    static CARGO_BIN: std::sync::OnceLock<ffi::OsString> = std::sync::OnceLock::new();
    CARGO_BIN.get_or_init(|| env::var_os("CARGO").unwrap_or_else(|| "cargo".into()))
}
