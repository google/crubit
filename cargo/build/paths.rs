// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use std::ffi::{OsStr, OsString};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

pub fn print_link_search<T: AsRef<OsStr>>(s: T) -> io::Result<()> {
    print!("cargo::rustc-link-search=native=");
    io::stdout().write_all(s.as_ref().as_encoded_bytes())?;
    println!();
    Ok(())
}

pub fn print_link_searches<T: AsRef<OsStr>>(paths: &[T]) -> io::Result<()> {
    for path in paths {
        print_link_search(path)?;
    }
    Ok(())
}

pub fn print_link_libs<T: AsRef<OsStr>>(libs: &[T]) -> io::Result<()> {
    for lib in libs {
        print!("cargo::rustc-link-lib=");
        io::stdout().write_all(lib.as_ref().as_encoded_bytes())?;
        println!();
    }
    Ok(())
}

#[cfg(unix)]
const LIB_EXTENSION: &str = "a";
#[cfg(windows)]
const LIB_EXTENSION: &str = "lib";

pub fn add_include_path<P: AsRef<Path>>(build: &mut cc::Build, path: P, system: bool) {
    let flag = if system && cfg!(unix) { "-isystem" } else { "-I" };
    build.flag(format!("{flag}{}", path.as_ref().display()));
}

/// Reads paths from an environment variable and validates that all specified paths exist on disk.
/// Paths may be separated by platform path separators (`:` on Unix, `;` on Windows) or commas.
///
/// Emits `cargo::rerun-if-env-changed` for `env_var`.
///
/// # Panics
///
/// Panics if `env_var` is not set in the environment, or if any of the specified paths do not
/// exist.
pub fn get_env_paths(env_var: &str) -> Vec<PathBuf> {
    println!("cargo::rerun-if-env-changed={}", env_var);
    let val = std::env::var(env_var).unwrap_or_else(|_| {
        panic!("\n\nERROR: Required environment variable '{}' is not set.\n\n", env_var);
    });

    val.split(',')
        .flat_map(std::env::split_paths)
        .filter(|path| !path.as_os_str().is_empty())
        .inspect(|path| {
            if !path.exists() {
                panic!(
                    "\n\nERROR: Path '{}' specified in '{}' does not exist.\n\n",
                    path.display(),
                    env_var
                );
            }
        })
        .collect()
}

/// Discovers and validates static library archives (`.a` / `.lib`) in directories specified by
/// `env_var`.
///
/// Filters libraries using `include_lib_fn` and emits `cargo::rerun-if-changed` for each included
/// archive.
///
/// Returns a tuple `(search_directories, library_names)`:
/// - `search_directories`: directories where the libraries were found, suitable for link search
///   paths.
/// - `library_names`: sorted and deduplicated library names with the file extension (`.a` / `.lib`)
///   and Unix `lib` prefix removed, suitable for `cargo::rustc-link-lib`.
///
/// # Panics
///
/// Panics if `env_var` is not set, if any directory does not exist or cannot be read, if any
/// archive has a non-UTF8 filename, or if no matching library archives are found.
pub fn collect_static_libs<F>(env_var: &str, include_lib_fn: F) -> (Vec<PathBuf>, Vec<OsString>)
where
    F: Fn(&str) -> bool,
{
    const { assert!(cfg!(unix) || cfg!(windows)) };

    let lib_dirs = get_env_paths(env_var);
    let mut libs = Vec::new();

    for dir in &lib_dirs {
        let entries = std::fs::read_dir(dir).unwrap_or_else(|e| {
            panic!(
                "\n\nERROR: Unable to read directory '{}' specified in '{}': {}\n\n",
                dir.display(),
                env_var,
                e
            )
        });

        for entry in entries {
            let Ok(entry) = entry else { continue };
            let Ok(meta) = entry.metadata() else { continue };
            if !meta.is_file() {
                continue;
            }
            let path = entry.path();
            if path.extension() != Some(OsStr::new(LIB_EXTENSION)) {
                continue;
            }
            let Some(stem) = path.file_stem() else {
                continue;
            };
            let stem_str = stem.to_str().unwrap_or_else(|| {
                panic!("Non-UTF8 filename in '{}': {:?}", env_var, path.display())
            });
            let libname = if cfg!(windows) {
                // On Windows, the filename without an extension: `name.lib` => `name`.
                stem_str
            } else {
                // On Unix, drop the lib prefix and the extension: `libname.a` => `name`.
                stem_str.strip_prefix("lib").unwrap_or(stem_str)
            };
            if include_lib_fn(libname) {
                println!("cargo::rerun-if-changed={}", path.display());
                libs.push(OsString::from(libname));
            }
        }
    }

    if libs.is_empty() {
        panic!(
            "\n\nERROR: No .{} static library files found in directory specified by '{}' \
             ({:?}).\n\n",
            LIB_EXTENSION, env_var, lib_dirs
        );
    }

    libs.sort_unstable();
    libs.dedup();
    (lib_dirs, libs)
}

pub fn print_env_to_string(env_var: &str) -> Option<String> {
    println!("cargo::rerun-if-env-changed={}", env_var);
    std::env::var(env_var).ok()
}

pub fn add_source_file<P: AsRef<Path>>(build: &mut cc::Build, path: P) -> io::Result<()> {
    print!("cargo::rerun-if-changed=");
    io::stdout().write_all(path.as_ref().as_os_str().as_encoded_bytes())?;
    println!();
    build.file(path.as_ref());
    Ok(())
}

pub fn print_compiler_deps() {
    println!("cargo::rerun-if-env-changed=CC");
    println!("cargo::rerun-if-env-changed=CXX");
    println!("cargo::rerun-if-env-changed=LD");
    println!("cargo::rerun-if-env-changed=CFLAGS");
    println!("cargo::rerun-if-env-changed=CXXFLAGS");
    println!("cargo::rerun-if-env-changed=LDFLAGS");
}
