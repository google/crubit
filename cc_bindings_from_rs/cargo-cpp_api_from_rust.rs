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
use cargo_metadata::{Message, Metadata, Package, PackageId, Resolve};
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
    let mut cmd = cargo_metadata::MetadataCommand::new();
    if let Some(ref path) = cli.manifest.manifest_path {
        cmd.manifest_path(path);
    }
    if cli.features.all_features {
        cmd.features(cargo_metadata::CargoOpt::AllFeatures);
    }
    if cli.features.no_default_features {
        cmd.features(cargo_metadata::CargoOpt::NoDefaultFeatures);
    }
    if !cli.features.features.is_empty() {
        cmd.features(cargo_metadata::CargoOpt::SomeFeatures(cli.features.features.clone()));
    }
    let metadata = cmd.exec()?;

    let root = metadata.root_package().ok_or_else(|| anyhow!("Failed to find root package"))?;

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
    build_args.extend(cli.build_args.clone());

    let pkg_to_artifact = build_crate_and_stream_artifacts(&build_args)?;

    let ctx = BindingGenerationContext::new(pkg_to_artifact, root.clone(), &metadata, target_dir)?;
    let lib_rs_content = ctx.generate_bindings()?;

    ctx.compile_staticlib(lib_rs_content, &metadata)?;

    Ok(())
}

use std::process::Child;

fn stream_cargo_build(
    args: &[String],
) -> Result<(Child, impl Iterator<Item = Result<Message, std::io::Error>>)> {
    let mut build_command = Command::new(cargo_bin());
    build_command.args(args);

    let mut command = build_command
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|err| anyhow!("Failed to spawn cargo: {}", err))?;

    let reader = std::io::BufReader::new(
        command.stdout.take().ok_or_else(|| anyhow!("Failed to open cargo stdout"))?,
    );
    Ok((command, cargo_metadata::Message::parse_stream(reader)))
}

fn build_crate_and_stream_artifacts(
    build_args: &[String],
) -> Result<HashMap<String, ArtifactInfo>> {
    let mut args = vec!["rustc".to_string(), "--message-format=json".to_string()];
    args.extend(build_args.iter().cloned());

    let mut pkg_to_artifact = HashMap::new();
    let (command, stream) = stream_cargo_build(&args)?;
    for message in stream {
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
                    artifact.package_id.repr.clone(),
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

    Ok(pkg_to_artifact)
}

struct Directories {
    target_dir: Utf8PathBuf,
    profile_dir: Utf8PathBuf,
    deps_dir: Utf8PathBuf,
    headers_dir: Utf8PathBuf,
}
impl Directories {
    fn new(target_dir: Utf8PathBuf, profile_dir: Utf8PathBuf) -> Self {
        let deps_dir = target_dir.join(&profile_dir).join("deps");
        let headers_dir = target_dir.join(&profile_dir).join("include");
        Directories { target_dir, profile_dir, deps_dir, headers_dir }
    }
}

struct BindingGenerationContext {
    pkg_to_artifact: HashMap<String, ArtifactInfo>,
    root: Package,
    ordered: Vec<PackageId>,
    dirs: Directories,
    resolve: Resolve,
}
impl BindingGenerationContext {
    fn new(
        pkg_to_artifact: HashMap<String, ArtifactInfo>,
        root: Package,
        metadata: &Metadata,
        target_dir: &Utf8PathBuf,
    ) -> Result<Self> {
        // It's important we check the path of root (and not one of our dependencies) or else we'll get
        // the wrong path.
        let profile_dir = pkg_to_artifact
            .get(&root.id.repr)
            .and_then(|artifact_info| {
                let path = &artifact_info.path;
                let rel_path = path.strip_prefix(&target_dir).ok()?;
                rel_path.parent().map(|p| p.to_owned())
            })
            .ok_or_else(|| anyhow!("Failed to find root package artifact"))?;
        let dirs = Directories::new(target_dir.to_owned(), profile_dir);

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
        Ok(Self { pkg_to_artifact, root, ordered, dirs, resolve: resolve.clone() })
    }

    fn generate_bindings(&self) -> Result<String> {
        let mut pkg_to_header = HashMap::new();
        let mut lib_rs_content = String::new();
        let deps_dir = &self.dirs.deps_dir;
        let headers_dir = &self.dirs.headers_dir;

        fs::create_dir_all(headers_dir)?;
        for pkg_id in self.ordered.iter() {
            let Some(artifact_info) = self.pkg_to_artifact.get(&pkg_id.repr) else {
                continue;
            };
            let crate_name = &artifact_info.name;
            let rs_crate_name = crate_name.replace('-', "_");
            let hash = &artifact_info.hash;
            let intermediate_h = deps_dir.join(format!("{}-{}.h", crate_name, hash));
            let final_h = headers_dir.join(format!("{}.h", crate_name));
            let intermediate_rs = deps_dir.join(format!("lib{}-{}.rs", crate_name, hash));

            if artifact_info.fresh && intermediate_h.exists() && intermediate_rs.exists() {
                pkg_to_header.insert(pkg_id.repr.clone(), intermediate_h.to_string());
                lib_rs_content.push_str(&format!(
                    "#[path = {:?}]\nmod r#{};\n",
                    intermediate_rs, rs_crate_name
                ));
                if !final_h.exists() {
                    fs::copy(&intermediate_h, &final_h)?;
                }
                continue;
            }

            let mut current_args = vec![
                "cpp_api_from_rust".to_string(),
                "--crubit-support-path-format=<support/{header}>".to_string(),
                "--enable-rmeta-interface".to_string(),
                format!("--source-crate-name={}", crate_name),
                format!("--h-out={}", intermediate_h),
                format!("--rs-out={}", intermediate_rs),
                format!("--extern={}={}", crate_name, artifact_info.path),
                format!("-Ldependency={}", deps_dir.as_str()),
            ];
            let resolve_node = self
                .resolve
                .nodes
                .iter()
                .find(|n| &n.id == pkg_id)
                .expect("Package must be in resolve graph");
            for dep_pkg_id in &resolve_node.dependencies {
                if let Some(dep_artifact) = self.pkg_to_artifact.get(&dep_pkg_id.repr) {
                    // TODO: Handle overlapping crate names better here.
                    current_args
                        .push(format!("--extern={}={}", dep_artifact.name, dep_artifact.path));
                    if let Some(dep_header) = pkg_to_header.get(&dep_pkg_id.repr) {
                        current_args
                            .push(format!("--crate-header={}={}", dep_artifact.name, dep_header));
                    }
                }
            }
            lib_rs_content
                .push_str(&format!("#[path = {:?}]\nmod r#{};\n", intermediate_rs, rs_crate_name));

            let cmdline = Cmdline::new(&current_args).map_err(|err| {
                match err.downcast_ref::<clap::Error>() {
                    // Explicitly call `clap::Error::exit`, because 1) it results in *colored* output and
                    // 2) it uses a zero exit code for specific "errors" (e.g. for `--help` output).
                    Some(clap_err) => {
                        let _: std::convert::Infallible = clap_err.exit();
                    }

                    // Return `err` from `main`.  This will print the error message (no color codes
                    // though) and terminate the process with a non-zero exit code.
                    None => err,
                }
            })?;
            cpp_api_from_rust_lib::run_with_cmdline_args(&cmdline)?;
            pkg_to_header.insert(pkg_id.repr.clone(), format!("{}.h", crate_name));

            // Final outputs: copy/rename from deps/ to their final locations.
            fs::copy(&intermediate_h, &final_h)?;
        }

        Ok(lib_rs_content)
    }

    fn compile_staticlib(&self, mut lib_rs_content: String, metadata: &Metadata) -> Result<()> {
        let root_name = &self.root.name;
        let deps_dir = &self.dirs.deps_dir;
        let target_dir = &self.dirs.target_dir;
        let profile_dir = &self.dirs.profile_dir;
        let lib_rs_path = deps_dir.join(format!("{}_cc_api.rs", root_name));
        let root_crate_name = root_name.replace('-', "_");
        lib_rs_content.push_str(&format!("pub use r#{}::*;\n", root_crate_name));
        fs::write(&lib_rs_path, lib_rs_content)?;

        let static_lib_path = target_dir.join(&profile_dir).join(format!("lib{}.a", root_name));
        let mut cargo_toml_content = format!(
            r#"[package]
name = "{root_name}-cc-api"
version = "0.1.0"
edition = "{edition}"

[workspace]

[lib]
path = "{lib_rs_filename}"
crate-type = ["staticlib"]

[dependencies]
bridge_rust = {{ package = "crubit_bridge_rust", version = "0.0.1" }}
    "#,
            root_name = root_name,
            edition = self.root.edition,
            lib_rs_filename = lib_rs_path.file_name().unwrap(),
        );

        let pkg_id_to_package: HashMap<_, _> =
            metadata.packages.iter().map(|p| (p.id.clone(), p)).collect();
        for pkg in self
            .ordered
            .iter()
            .filter(|pkg_id| self.pkg_to_artifact.contains_key(&pkg_id.repr))
            .filter_map(|pkg_id| pkg_id_to_package.get(pkg_id))
        {
            if pkg.source.as_ref().is_some_and(|source| source.is_crates_io()) {
                cargo_toml_content.push_str(&format!("{} = \"{}\"\n", pkg.name, pkg.version));
            } else {
                // If it's not crates.io, we may need a more complex way to resolve it.
                // For now, we'll try to use the path if it's available.
                cargo_toml_content.push_str(&format!(
                    "{} = {{ path = {:?} }}\n",
                    pkg.name,
                    pkg.manifest_path
                        .parent()
                        .expect("Manifest path expected to have at least a Cargo.toml segment")
                ));
            }
        }

        let cargo_toml_path = deps_dir.join("Cargo.toml");
        fs::write(&cargo_toml_path, cargo_toml_content)?;

        let mut cargo_build = vec![
            "build".to_string(),
            "--manifest-path".to_string(),
            cargo_toml_path.to_string(),
            "--message-format=json".to_string(),
        ];

        if profile_dir.as_str() == "release" {
            cargo_build.push("--release".to_string());
        }
        let (child, stream) = stream_cargo_build(&cargo_build)?;
        let mut cargo_static_lib_path = None;
        for message in stream {
            let message =
                message.map_err(|err| anyhow!("Failed to parse cargo message: {}", err))?;
            // TODO: Extract an iterator wrapper that prints out lines out output and returns artifacts.
            let Message::CompilerArtifact(artifact) = message else {
                continue;
            };
            if artifact.target.kind.contains(&cargo_metadata::TargetKind::StaticLib) {
                cargo_static_lib_path = artifact.filenames.first().cloned();
            }
        }

        let output = child
            .wait_with_output()
            .map_err(|err| anyhow!("Failed to build staticlib: {}", err))?;
        if !output.status.success() {
            bail!("Cargo build of bindings failed");
        }
        let cargo_static_lib_path =
            cargo_static_lib_path.ok_or_else(|| anyhow!("Failed to find staticlib output"))?;

        fs::copy(&cargo_static_lib_path, &static_lib_path).map_err(|err| {
            anyhow!(
                "Failed to copy cargo output to final staticlib path: {}\n{}",
                cargo_static_lib_path,
                err
            )
        })?;

        Ok(())
    }
}

fn cargo_bin() -> &'static ffi::OsStr {
    static CARGO_BIN: std::sync::OnceLock<ffi::OsString> = std::sync::OnceLock::new();
    CARGO_BIN.get_or_init(|| env::var_os("CARGO").unwrap_or_else(|| "cargo".into()))
}
