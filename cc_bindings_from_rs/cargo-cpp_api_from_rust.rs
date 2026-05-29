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
use cargo_metadata::{Artifact, Message, Metadata, Package, PackageId, Resolve};
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
) -> Result<(Child, impl Iterator<Item = Result<Artifact, std::io::Error>>)> {
    let mut build_command = Command::new(cargo_bin());
    build_command.args(args);

    let mut command = build_command
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|err| anyhow!("Failed to spawn cargo: {}", err))?;

    let reader = std::io::BufReader::new(
        command.stdout.take().ok_or_else(|| anyhow!("Failed to open cargo stdout"))?,
    );
    // Print out any compiler diagnostics when we walk the iterator leaving only the complier artifacts.
    Ok((
        command,
        cargo_metadata::Message::parse_stream(reader).filter_map(|message| match message {
            Ok(message) => match message {
                Message::CompilerMessage(msg) => {
                    eprint!("{}", msg);
                    None
                }
                Message::TextLine(msg) => {
                    eprint!("{}", msg);
                    None
                }
                Message::CompilerArtifact(artifact) => Some(Ok(artifact)),
                _ => None,
            },
            Err(err) => Some(Err(err)),
        }),
    ))
}

fn build_crate_and_stream_artifacts(
    build_args: &[String],
) -> Result<HashMap<String, ArtifactInfo>> {
    let mut args = vec!["rustc".to_string(), "--message-format=json".to_string()];
    args.extend(build_args.iter().cloned());
    // We rely on filename to extract the metadata hash cargo used for our current build.
    // We attach that hash to the intermediate files we produce to ensure they don't clobber
    // unrelated outputs from different build configurations (the same way cargo does). To do
    // this, our build must emit metadata. The filepath emitted for `.rlib` is the final filepath
    // with no metadata hash included.
    if build_args.contains(&"--".to_string()) {
        args.push("--emit=metadata,link".to_string());
    } else {
        args.extend(["--".to_string(), "--emit=metadata,link".to_string()]);
    }

    let mut pkg_to_artifact = HashMap::new();
    let (command, stream) = stream_cargo_build(&args)?;
    for artifact in stream {
        let artifact = artifact.map_err(|err| anyhow!("Failed to parse cargo message: {}", err))?;
        let find_metadata_file = artifact
            .filenames
            .iter()
            .find(|filename| filename.extension().is_some_and(|ext| ext == "rmeta"));
        let Some(filename) = find_metadata_file else {
            continue;
        };
        let hash =
            filename.file_stem().and_then(|s| s.rsplitn(2, '-').next()).unwrap_or("").to_string();
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

    let cargo_output =
        command.wait_with_output().map_err(|err| anyhow!("Failed to wait for cargo: {}", err))?;
    if !cargo_output.status.success() {
        eprintln!("{}", String::from_utf8_lossy(&cargo_output.stderr));
        bail!("Exiting early due to cargo build failure");
    }

    Ok(pkg_to_artifact)
}

struct Directories {
    /// Target-arch specific target directory, e.g.
    /// "/path/to/target/x86_64-unknown-linux-gnu/release".
    profile_dir: Utf8PathBuf,
    /// Profile name, e.g. "release".
    profile_name: String,
    /// Contains target-arch specific dependencies, e.g.
    /// "/path/to/target/x86_64-unknown-linux-gnu/release/deps".
    deps_dir: Utf8PathBuf,
    /// Contains target-arch specific headers, e.g.
    /// "/path/to/target/x86_64-unknown-linux-gnu/release/include".
    headers_dir: Utf8PathBuf,
    /// Contains host-arch specific dependencies (proc macros), e.g. "/path/to/target/release/deps".
    host_deps_dir: Utf8PathBuf,
}
impl Directories {
    fn new(target_dir: Utf8PathBuf, profile_dir: Utf8PathBuf) -> Result<Self> {
        let profile_name = profile_dir
            .file_name()
            .ok_or_else(|| {
                anyhow!("Expected profile directory '{profile_dir}' to include a profile component")
            })?
            .to_string();
        let profile_dir = target_dir.join(profile_dir);
        let deps_dir = profile_dir.join("deps");
        let headers_dir = profile_dir.join("include");
        let host_deps_dir = target_dir.join(&profile_name).join("deps");
        Ok(Directories { profile_dir, profile_name, deps_dir, headers_dir, host_deps_dir })
    }
}

struct CrateBindingInfo<'a> {
    pkg_id_repr: &'a str,
    crate_name: &'a str,
    rs_crate_name: &'a str,
    artifact_path: &'a cargo_metadata::camino::Utf8Path,
    hash: &'a str,
    fresh: bool,
    is_stdlib: bool,
    stdlib_externs: &'a [(String, Utf8PathBuf)],
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
                rel_path.parent().and_then(|p|
                    // Our path can be to an intermediate that already resides in the `deps`
                    // directory. We only want the profile directory here (with an optional target
                    // specific component). We'll reconstruct the deps path ourselves later.
                    if p.file_name() == Some("deps") {
                        p.parent()
                    } else {
                        Some(p)
                    }
                ).map(|p| p.to_owned())
            })
            .ok_or_else(|| anyhow!("Failed to find root package artifact"))?;
        let dirs = Directories::new(target_dir.to_owned(), profile_dir)?;

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

    fn get_sysroot(&self) -> Result<Utf8PathBuf> {
        let output = std::process::Command::new(cargo_bin())
            .arg("rustc")
            .arg("--manifest-path")
            .arg(&self.root.manifest_path)
            .arg("--")
            .arg("--print")
            .arg("sysroot")
            .output()?;
        if !output.status.success() {
            bail!("Failed to get sysroot from cargo rustc");
        }
        let path = std::str::from_utf8(&output.stdout)?.trim();
        Ok(Utf8PathBuf::from(path))
    }

    fn find_stdlib_rmetas(sysroot: &Utf8PathBuf) -> Result<Vec<(String, Utf8PathBuf)>> {
        // Walk the lib directory to find the matching rmeta files.
        // Usually <sysroot>/lib/rustlib/<target>/lib/
        let rustlib = sysroot.join("lib").join("rustlib");
        let mut target_lib_dir = None;
        if !rustlib.exists() {
            bail!("Could not find target lib dir in sysroot");
        }
        for entry in std::fs::read_dir(rustlib)? {
            let entry = entry?;
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }
            let lib_sub = path.join("lib");
            if !lib_sub.exists() {
                continue;
            }
            // Make sure it's the target directory by checking if any rlib/rmeta is there.
            for sub_entry in std::fs::read_dir(&lib_sub)? {
                let sub_entry = sub_entry?;
                if sub_entry.path().extension().is_some_and(|ext| ext == "rmeta" || ext == "rlib") {
                    target_lib_dir = Some(Utf8PathBuf::from_path_buf(lib_sub).unwrap());
                    break;
                }
            }
            if target_lib_dir.is_some() {
                break;
            }
        }
        let lib_dir =
            target_lib_dir.ok_or_else(|| anyhow!("Could not find target lib dir in sysroot"))?;

        let libs = ["core", "alloc", "std", "proc_macro"];
        let mut rmetas = vec![(String::default(), Utf8PathBuf::default()); 4];
        // This order is important because we rely on it to determine what standard library
        // crates depend on each other. std depends on core and alloc, alloc depends on core, etc.
        for entry in std::fs::read_dir(&lib_dir)? {
            let entry = entry?;
            let path = entry.path();
            let Some(ext) = path.extension() else {
                continue;
            };
            if ext != "rmeta" {
                continue;
            }
            let Some(file_name) = path.file_name().and_then(|s| s.to_str()) else {
                continue;
            };
            for (i, lib) in libs.iter().enumerate() {
                if file_name.starts_with(&format!("lib{}-", lib)) {
                    rmetas[i] = (lib.to_string(), Utf8PathBuf::from_path_buf(path).unwrap());
                    break;
                }
            }
        }
        Ok(rmetas)
    }

    fn generate_crate_bindings(
        &self,
        info: CrateBindingInfo<'_>,
        pkg_to_header: &mut HashMap<String, String>,
        lib_rs_content: &mut String,
    ) -> Result<()> {
        let deps_dir = &self.dirs.deps_dir;
        let headers_dir = &self.dirs.headers_dir;
        let host_deps_dir = &self.dirs.host_deps_dir;

        let hash_suffix =
            if info.hash.is_empty() { "".to_string() } else { format!("-{}", info.hash) };
        let intermediate_h = deps_dir.join(format!("{}{}.h", info.crate_name, hash_suffix));
        let final_h_filename = format!("{}.h", info.crate_name);
        let final_h = headers_dir.join(&final_h_filename);
        let intermediate_rs = deps_dir.join(format!("lib{}{}.rs", info.crate_name, hash_suffix));
        lib_rs_content
            .push_str(&format!("#[path = {:?}]\nmod r#{};\n", intermediate_rs, info.rs_crate_name));

        if info.fresh && intermediate_h.exists() && intermediate_rs.exists() {
            pkg_to_header.insert(info.pkg_id_repr.to_string(), final_h_filename);
            if !final_h.exists() {
                fs::copy(&intermediate_h, &final_h)?;
            }
            return Ok(());
        }

        let mut current_args = vec![
            "cpp_api_from_rust".to_string(),
            "--crubit-support-path-format=<support/{header}>".to_string(),
            "--enable-rmeta-interface".to_string(),
            format!("--source-crate-name={}", info.crate_name),
            format!("--h-out={}", intermediate_h),
            format!("--rs-out={}", intermediate_rs),
            format!("--extern={}={}", info.crate_name, info.artifact_path),
            format!("-Ldependency={}", deps_dir.as_str()),
        ];

        if info.is_stdlib {
            current_args.push(format!("--crate-namespace=self=rs::{}", info.crate_name));
            current_args
                .push(format!("--crate-namespace={}=rs::{}", info.crate_name, info.crate_name));
        } else {
            current_args.push(format!("-Ldependency={}", host_deps_dir.as_str()));
        }

        // Pass preceding stdlib dependencies to the current crate
        for (prev_name, prev_path) in info.stdlib_externs {
            current_args.push(format!("--extern={}={}", prev_name, prev_path));
            if let Some(prev_header) = pkg_to_header.get(prev_name.as_str()) {
                current_args.push(format!("--crate-header={}={}", prev_name, prev_header));
                current_args.push(format!("--crate-namespace={}=rs::{}", prev_name, prev_name));
            }
        }

        if !info.is_stdlib {
            let resolve_node = self
                .resolve
                .nodes
                .iter()
                .find(|n| n.id.repr == info.pkg_id_repr)
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
        }

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
        pkg_to_header.insert(info.pkg_id_repr.to_string(), final_h_filename);

        // Final outputs: copy/rename from deps/ to their final locations.
        fs::copy(&intermediate_h, &final_h)?;
        Ok(())
    }

    fn generate_bindings(&self) -> Result<String> {
        let mut pkg_to_header = HashMap::new();
        let mut lib_rs_content = r#"
extern crate alloc;
extern crate core;
extern crate proc_macro;

"#
        .to_string();
        let headers_dir = &self.dirs.headers_dir;
        let profile_dir = &self.dirs.profile_dir;

        fs::create_dir_all(headers_dir)?;

        // 1. Locate standard library crates and generate bindings for them first.
        let sysroot = self.get_sysroot()?;
        let stdlib_crates = Self::find_stdlib_rmetas(&sysroot)?;
        let mut stdlib_externs = Vec::new();
        for (crate_name, rmeta_path) in &stdlib_crates {
            let rs_crate_name = format!("{}_cc_api", crate_name.replace('-', "_"));
            self.generate_crate_bindings(
                CrateBindingInfo {
                    pkg_id_repr: crate_name,
                    crate_name,
                    rs_crate_name: &rs_crate_name,
                    artifact_path: rmeta_path,
                    hash: "",
                    fresh: true,
                    is_stdlib: true,
                    stdlib_externs: &stdlib_externs,
                },
                &mut pkg_to_header,
                &mut lib_rs_content,
            )?;
            stdlib_externs.push((crate_name.clone(), rmeta_path.clone()));
        }

        // 2. Generate bindings for user packages in topological order.
        for pkg_id in self.ordered.iter() {
            let Some(artifact_info) = self.pkg_to_artifact.get(&pkg_id.repr) else {
                continue;
            };
            if !artifact_info.path.starts_with(&profile_dir) {
                // Skip crates outside profile dir, e.g. proc macros.
                continue;
            }
            let crate_name = &artifact_info.name;
            let rs_crate_name = crate_name.replace('-', "_");
            let hash = &artifact_info.hash;

            self.generate_crate_bindings(
                CrateBindingInfo {
                    pkg_id_repr: &pkg_id.repr,
                    crate_name,
                    rs_crate_name: &rs_crate_name,
                    artifact_path: &artifact_info.path,
                    hash,
                    fresh: artifact_info.fresh,
                    is_stdlib: false, // is_stdlib
                    stdlib_externs: &stdlib_externs,
                },
                &mut pkg_to_header,
                &mut lib_rs_content,
            )?;
        }

        Ok(lib_rs_content)
    }

    fn compile_staticlib(&self, mut lib_rs_content: String, metadata: &Metadata) -> Result<()> {
        let root_name = &self.root.name;
        let deps_dir = &self.dirs.deps_dir;
        let profile_dir = &self.dirs.profile_dir;
        let profile_name = &self.dirs.profile_name;

        let root_artifact = self
            .pkg_to_artifact
            .get(&self.root.id.repr)
            .ok_or_else(|| anyhow!("Failed to find root package artifact"))?;
        let hash_suffix = if root_artifact.hash.is_empty() {
            "".to_string()
        } else {
            format!("-{}", root_artifact.hash)
        };
        let project_dir = deps_dir.join(format!("{}{}", root_name, hash_suffix));
        fs::create_dir_all(&project_dir)?;

        let lib_rs_path = project_dir.join(format!("{}_cc_api.rs", root_name));
        let root_crate_name = root_name.replace('-', "_");
        lib_rs_content.push_str(&format!("pub use r#{}::*;\n", root_crate_name));
        fs::write(&lib_rs_path, lib_rs_content)?;

        let static_lib_path = profile_dir.join(format!("lib{}.a", root_name));
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
            .filter(|pkg_id| {
                self.pkg_to_artifact
                    .get(&pkg_id.repr)
                    // Skip crates outside profile dir, e.g. proc macros.
                    .is_some_and(|art| art.path.starts_with(&profile_dir))
            })
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

        let cargo_toml_path = project_dir.join("Cargo.toml");
        fs::write(&cargo_toml_path, cargo_toml_content)?;

        let mut cargo_build = vec![
            "build".to_string(),
            "--manifest-path".to_string(),
            cargo_toml_path.to_string(),
            "--message-format=json".to_string(),
        ];

        if profile_name.as_str() == "release" {
            cargo_build.push("--release".to_string());
        }
        let (child, stream) = stream_cargo_build(&cargo_build)?;
        let mut cargo_static_lib_path = None;
        for artifact in stream {
            let artifact =
                artifact.map_err(|err| anyhow!("Failed to parse cargo message: {}", err))?;
            if artifact.target.kind.contains(&cargo_metadata::TargetKind::StaticLib) {
                cargo_static_lib_path = artifact.filenames.first().cloned();
                // It's important to consume the entire iterator, even after we've found the
                // artifact, so we don't miss any diagnostics emitted.
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
