// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

mod absl;
mod clang;
mod flags;
mod paths;

use std::path::Path;

/// Build a C++ library of `sources`, with paths specified relative to the
/// source root.
///
/// `path_to_src_root` gives the root of the repo, where paths are specified
/// relative to.
///
/// All C++ libraries can make use of ABSL, LLVM, and Clang, as they are
/// included in the include path, and are added to the link step.
pub fn compile_cc_lib<P1: AsRef<Path>, P2: AsRef<Path>>(
    path_to_src_root: P1,
    sources: &[P2],
) -> Result<(), std::io::Error> {
    let name = std::env::var("CARGO_PKG_NAME").unwrap();
    let out_dir = std::env::var("OUT_DIR").unwrap();
    // Avoid building object files into the root output directory. If the binary
    // name matches the directory name of a source file, then we get a
    // collision. Put it in a subdir of the target's individual output
    // directory.
    let obj_dir = Path::new(&out_dir).join("obj");
    // Ensure the directory exists. The linker makes the dir on Linux but will
    // fail on Windows.
    if let Err(e) = std::fs::create_dir(&obj_dir) {
        match e.kind() {
            std::io::ErrorKind::AlreadyExists => (),
            _ => Err(e)?,
        }
    }

    paths::print_compiler_deps();

    // ===== Abseil ======

    let absl_include_dirs = absl::collect_absl_includes();
    let (absl_lib_dirs, absl_libs) = absl::collect_absl_libs();
    paths::print_link_searchs(&absl_lib_dirs)?;
    paths::print_link_libs(&absl_libs)?;

    // ===== LLVM libtooling =====

    // TODO: Use llvm-config instead of LIBCLANG_STATIC_PATH?
    let clang_include_dirs = clang::collect_clang_includes();
    let (clang_lib_dirs, clang_libs) = clang::collect_clang_libs();
    paths::print_link_searchs(&clang_lib_dirs)?;
    paths::print_link_libs(&clang_libs)?;

    // ===== The cc lib ======

    let mut cc_lib = cc::Build::new();
    cc_lib.out_dir(&obj_dir);
    for f in flags::CC_FLAGS {
        cc_lib.flag(f);
    }
    cc_lib.include(path_to_src_root.as_ref());
    for p in sources.into_iter().map(|p| path_to_src_root.as_ref().join(p.as_ref())) {
        paths::add_source_file(&mut cc_lib, &p)?;
    }
    for p in absl_include_dirs.into_iter().chain(clang_include_dirs) {
        paths::add_include_path(&mut cc_lib, p, false);
    }
    cc_lib.compile(&name);

    paths::print_link_search(&obj_dir)?;
    paths::print_link_libs(&[name])?;

    Ok(())
}
