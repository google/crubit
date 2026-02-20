// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#![feature(rustc_private)]
#![feature(never_type)]

use arc_anyhow::{anyhow, Error, Result};
use cargo_metadata::{Message, MetadataCommand};
use cmdline::Cmdline;

use std::env;
use std::ffi;
use std::process::{Command, Stdio};

// TODO(b/448731652): This should support passthrough for arbitrary cargo commandline flags that
// one might want to specify. This is important both for building target package initially, and
// building the final static lib at the end.
fn main() -> Result<()> {
    let metadata = MetadataCommand::new().exec().unwrap();

    let root = metadata.root_package().ok_or(anyhow!("Failed to find root package"))?;

    let source_crate_name = root.name.as_ref();

    let mut args = vec![
        // Make up a binary name for our call. This will get removed by Cmdline, so it doesn't matter
        // but it must be here otherwise a real cmdline arg will get ate.
        "cpp_api_from_rust".to_string(),
        format!("--source-crate-name={}", source_crate_name),
        "--crubit-support-path-format=<{header}>".to_string(),
        "--enable-rmeta-interface".to_string(),
    ];

    // TODO: Include cargo flags in this from our own args.
    let mut command = Command::new(cargo_bin())
        //We build, instead of check, because we need the rlib output to compile out final
        // staticlib.
        .args(&["build", "--message-format=json"])
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|err| anyhow!("Failed to spawn cargo: {}", err))?;

    let mut profile_dir = None;
    let mut source_crate_rlib = None;
    let reader = std::io::BufReader::new(
        command.stdout.take().ok_or(anyhow!("Failed to open cargo stdout"))?,
    );
    for message in cargo_metadata::Message::parse_stream(reader) {
        match message.unwrap() {
            Message::CompilerArtifact(artifact) => {
                let filename = artifact
                    .filenames
                    .iter()
                    .find(|filename| {
                        filename.extension().is_some_and(|ext| ext == "rmeta" || ext == "rlib")
                    })
                    .expect("We expect atleast one filename for a check");
                let name = artifact.target.name;
                if name == source_crate_name {
                    profile_dir = filename
                        .strip_prefix(&metadata.target_directory)
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
            _ => {} // We only care about compiler artifacts
        }
    }

    let out_dir = metadata
        .target_directory
        .join(profile_dir.ok_or(anyhow!("Failed to find profile directory for source crate"))?);
    args.extend([
        format!("--h-out={}", out_dir.join(format!("{}.h", source_crate_name))),
        format!("--rs-out={}", out_dir.join(format!("{}_cc_api_impl.rs", source_crate_name))),
        format!("--cpp-out={}", out_dir.join(format!("{}.cpp", source_crate_name))),
        format!("-Ldependency={}", out_dir.join("deps")),
    ]);

    // Wait on `cargo check` to finish to ensure all our files are generated.
    command.wait().map_err(|err| anyhow!("Failed to wait for cargo: {}", err))?;

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

    // Build our static lib.
    let rustc_args = vec![
        "rustc".to_string(),
        out_dir.join(format!("{}_cc_api_impl.rs", source_crate_name)).to_string(),
        "--edition=2021".to_string(),
        format!("--crate-name={}_cc_api", source_crate_name),
        "--crate-type=staticlib".to_string(),
        format!(
            "--extern={}={}",
            source_crate_name,
            source_crate_rlib.expect("Failed to find rlib for source crate")
        ),
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
