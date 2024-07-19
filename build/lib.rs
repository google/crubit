// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

mod absl;
mod clang;
mod flags;
mod paths;

use std::path::Path;

/// Build a C++ library of `sources`, with paths specified relative to the source
/// root.
///
/// `path_to_src_root` gives the root of the repo, where paths are specified
/// relative to.
///
/// All C++ libraries can make use of ABSL, LLVM, and Clang, as they are included
/// in the include path, and are added to the link step.
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

    paths::print_compiler_deps();

    // ===== Abseil ======

    let absl_include_dir = paths::print_env_to_path("ABSL_INCLUDE_DIR")
        .expect("ABSL_INCLUDE_DIR must be specified in the environment");
    let (absl_lib_dir, absl_libs) = absl::collect_absl_libs();
    paths::print_link_search(absl_lib_dir)?;
    paths::print_link_libs(&absl_libs)?;

    // ===== libtooling =====

    // TODO: Use llvm-config instead of LIBCLANG_STATIC_PATH?
    let libclang_static_path = paths::print_env_to_path("LIBCLANG_STATIC_PATH")
        .expect("LIBCLANG_STATIC_PATH must be specified in the environment");
    let libclang_include_path = libclang_static_path.parent().unwrap().join("include");
    let clang_libs = clang::collect_clang_libs(&libclang_static_path);
    paths::print_link_search(&libclang_static_path)?;
    paths::print_link_libs(&clang_libs)?;

    // llvm depends on zlib.
    paths::print_link_libs(&["z"])?;

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
    paths::add_include_path(&mut cc_lib, absl_include_dir, true);
    paths::add_include_path(&mut cc_lib, libclang_include_path, true);
    cc_lib.compile(&name);

    paths::print_link_search(&obj_dir)?;
    paths::print_link_libs(&[name])?;

    Ok(())
}
