// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use std::ffi::OsStr;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

pub fn print_link_search<T: AsRef<OsStr>>(s: T) -> io::Result<()> {
    print!("cargo::rustc-link-search=native=");
    io::stdout().write(s.as_ref().as_encoded_bytes())?;
    print!("\n");
    Ok(())
}

pub fn print_link_libs<T: AsRef<OsStr>>(libs: &[T]) -> io::Result<()> {
    for lib in libs {
        print!("cargo::rustc-link-lib=");
        io::stdout().write(lib.as_ref().as_encoded_bytes())?;
        print!("\n");
    }
    Ok(())
}

pub fn add_include_path<P: AsRef<Path>>(build: &mut cc::Build, path: P, system: bool) {
    if system {
        if cfg!(unix) {
            build.flag(&format!("-isystem{}", path.as_ref().display()));
        } else {
            build.flag(&format!("-I{}", path.as_ref().display()));
        }
    } else {
        build.flag(&format!("-I{}", path.as_ref().display()));
    }
}

pub fn print_env_to_path(env_var: &str) -> Option<PathBuf> {
    println!("cargo:rerun-if-env-changed={}", env_var);
    Some(Path::new(&std::env::var_os(env_var)?).to_owned())
}

pub fn add_source_file<P: AsRef<Path>>(build: &mut cc::Build, path: P) -> io::Result<()> {
    print!("cargo::rerun-if-changed=");
    io::stdout().write(path.as_ref().as_os_str().as_encoded_bytes())?;
    print!("\n");
    build.file(path.as_ref());
    Ok(())
}

pub fn print_compiler_deps() {
    println!("cargo:rerun-if-env-changed=CC");
    println!("cargo:rerun-if-env-changed=CXX");
    println!("cargo:rerun-if-env-changed=LD");
    println!("cargo:rerun-if-env-changed=CFLAGS");
    println!("cargo:rerun-if-env-changed=CXXFLAGS");
    println!("cargo:rerun-if-env-changed=LDFLAGS");
}
