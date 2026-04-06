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

use arc_anyhow::{anyhow, bail, Result};
use cargo_metadata::camino::Utf8PathBuf;
use cargo_metadata::Message;
use clap::Parser;
use cmdline::Cmdline;

use std::collections::HashMap;
use std::env;
use std::ffi;
use std::fs;
use std::process::{Command, Stdio};
use toposort::{toposort, Dependency};

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

struct ArtifactInfo {
    path: Utf8PathBuf,
    name: String,
    hash: String,
    fresh: bool,
}

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

    let args = vec![
        "cpp_api_from_rust".to_string(),
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

    let mut pkg_to_artifact = HashMap::new();
    let reader = std::io::BufReader::new(
        command.stdout.take().ok_or_else(|| anyhow!("Failed to open cargo stdout"))?,
    );
    for message in cargo_metadata::Message::parse_stream(reader) {
        let message = message.map_err(|err| anyhow!("Failed to parse cargo message: {}", err))?;
        if let Message::CompilerArtifact(artifact) = message {
            let find_metadata_file = artifact.filenames.iter().find(|filename| {
                filename.extension().is_some_and(|ext| ext == "rmeta" || ext == "rlib")
            });
            if let Some(filename) = find_metadata_file {
                let hash = filename
                    .file_stem()
                    .and_then(|s| s.rsplitn(2, '-').next())
                    .unwrap_or("")
                    .to_string();
                pkg_to_artifact.insert(
                    artifact.package_id.repr,
                    ArtifactInfo {
                        path: filename.clone(),
                        name: artifact.target.name.clone(),
                        hash,
                        fresh: artifact.fresh,
                    },
                );
            }
        }
    }

    let cargo_output =
        command.wait_with_output().map_err(|err| anyhow!("Failed to wait for cargo: {}", err))?;
    if !cargo_output.status.success() {
        eprintln!("{}", String::from_utf8_lossy(&cargo_output.stderr));
        bail!("Exiting early due to cargo build failure");
    }

    // It's important we check the path of root (and not one of our dependencies) or else we'll get
    // the wrong path.
    let profile_dir = pkg_to_artifact
        .get(&root.id.repr)
        .and_then(|artifact_info| {
            let path = &artifact_info.path;
            let rel_path = path.strip_prefix(&target_dir).ok()?;
            rel_path.parent().to_owned()
        })
        .ok_or_else(|| anyhow!("Failed to find root package artifact"))?;
    let deps_dir = target_dir.join(&profile_dir).join("deps");

    let resolve = metadata
        .resolve
        .as_ref()
        .ok_or_else(|| anyhow!("Could not determine crate dependencies"))?;
    let ordered = {
        let nodes = resolve.nodes.iter().map(|n| n.id.clone()).collect::<Vec<_>>();
        let deps = resolve
            .nodes
            .iter()
            .flat_map(|n| {
                n.dependencies
                    .iter()
                    .map(|dep| Dependency { predecessor: dep.clone(), successor: n.id.clone() })
            })
            .collect::<Vec<_>>();

        let toposort::TopoSortResult { ordered, .. } = toposort(nodes, deps, |a, b| a.cmp(b));
        ordered
    };

    let mut pkg_to_header = HashMap::new();
    let headers_dir = target_dir.join(&profile_dir).join("headers");
    fs::create_dir_all(&headers_dir)?;

    let mut lib_rs_content = String::new();
    for pkg_id in ordered {
        let artifact_info = match pkg_to_artifact.get(&pkg_id.repr) {
            Some(info) => info,
            None => continue,
        };

        let crate_name = &artifact_info.name;
        let rs_crate_name = crate_name.replace('-', "_");
        let hash = &artifact_info.hash;
        let intermediate_h = deps_dir.join(format!("{}-{}.h", crate_name, hash));
        let final_h = headers_dir.join(format!("{}.h", crate_name));
        let intermediate_rs = deps_dir.join(format!("lib{}-{}.rs", crate_name, hash));

        if artifact_info.fresh && intermediate_h.exists() && intermediate_rs.exists() {
            pkg_to_header.insert(pkg_id.repr, intermediate_h.to_string());
            lib_rs_content.push_str(&format!(
                "#[path = {:?}]pub mod r#{};\n",
                intermediate_rs, rs_crate_name
            ));
            // Final outputs: copy/rename from deps/ to their final locations.
            if !final_h.exists() {
                fs::copy(&intermediate_h, &final_h)?;
            }
            continue;
        }

        let mut current_args = args.clone();
        current_args.extend([
            format!("--source-crate-name={}", crate_name),
            format!("--h-out={}", intermediate_h),
            format!("--rs-out={}", intermediate_rs),
            format!("--extern={}={}", crate_name, artifact_info.path),
            format!("-Ldependency={}", deps_dir.as_str()),
        ]);
        lib_rs_content
            .push_str(&format!("#[path = {:?}]pub mod r#{};\n", intermediate_rs, rs_crate_name));

        let resolve_node = resolve
            .nodes
            .iter()
            .find(|n| n.id == pkg_id)
            .expect("Package must be in resolve graph");
        for dep_pkg_id in &resolve_node.dependencies {
            if let Some(dep_artifact) = pkg_to_artifact.get(&dep_pkg_id.repr) {
                // TODO: Handle overlapping crate names better here.
                current_args.push(format!("--extern={}={}", dep_artifact.name, dep_artifact.path));
                if let Some(dep_header) = pkg_to_header.get(&dep_pkg_id.repr) {
                    current_args
                        .push(format!("--crate-header={}={}", dep_artifact.name, dep_header));
                }
            }
        }

        let cmdline =
            Cmdline::new(&current_args).map_err(|err| match err.downcast_ref::<clap::Error>() {
                // Explicitly call `clap::Error::exit`, because 1) it results in *colored* output and
                // 2) it uses a zero exit code for specific "errors" (e.g. for `--help` output).
                Some(clap_err) => {
                    let _: std::convert::Infallible = clap_err.exit();
                }

                // Return `err` from `main`.  This will print the error message (no color codes
                // though) and terminate the process with a non-zero exit code.
                None => err,
            })?;
        cpp_api_from_rust_lib::run_with_cmdline_args(&cmdline)?;
        pkg_to_header.insert(pkg_id.repr, intermediate_h.to_string());

        // Final outputs: copy/rename from deps/ to their final locations.
        fs::copy(&intermediate_h, &final_h)?;
    }

    let root_name = &root.name;
    let lib_rs_path = deps_dir.join(format!("{}_cc_api.rs", root_name));
    let root_crate_name = root_name.replace('-', "_");
    lib_rs_content.push_str(&format!("pub use r#{}::*;\n", root_crate_name));
    fs::write(&lib_rs_path, lib_rs_content)?;

    let static_lib_path = target_dir.join(&profile_dir).join(format!("lib{}.a", root_name));
    let mut final_rustc_args = vec![
        "rustc".to_string(),
        lib_rs_path.to_string(),
        format!("--edition={}", edition),
        format!("--crate-name={}_cc_api", root_name),
        "--crate-type=staticlib".to_string(),
        format!("-Ldependency={}", deps_dir.as_str()),
        "-o".to_string(),
        static_lib_path.to_string(),
    ];
    for (_, artifact_info) in pkg_to_artifact.iter() {
        final_rustc_args.push(format!("--extern={}={}", artifact_info.name, artifact_info.path));
    }

    cpp_api_from_rust_lib::run_rustc(&final_rustc_args);

    Ok(())
}

fn cargo_bin() -> &'static ffi::OsStr {
    static CARGO_BIN: std::sync::OnceLock<ffi::OsString> = std::sync::OnceLock::new();
    CARGO_BIN.get_or_init(|| env::var_os("CARGO").unwrap_or_else(|| "cargo".into()))
}
