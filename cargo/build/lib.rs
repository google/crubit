// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

mod absl;
mod clang;
mod flags;
mod paths;
mod protobuf;

use std::collections::BTreeSet;
use std::path::Path;

/// Build a C++ library of `sources`, with paths specified relative to the
/// source root.
///
/// `path_to_src_root` gives the root of the repo, where paths are specified
/// relative to.
///
/// All C++ libraries can make use of ABSL, LLVM, Protobuf, and Clang, as they are
/// included in the include path, and are added to the link step.
pub fn compile_cc_lib<P1: AsRef<Path>, P2: AsRef<Path>, P3: AsRef<Path>>(
    path_to_src_root: P1,
    sources: &[P2],
    proto_sources: &[P3],
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
    std::fs::create_dir_all(&obj_dir)?;

    paths::print_compiler_deps();

    // ===== Abseil ======

    let absl_include_dirs = absl::collect_absl_includes();
    let (absl_lib_dirs, absl_libs) = absl::collect_absl_libs();

    // ===== LLVM libtooling =====

    // TODO: Use llvm-config instead of LIBCLANG_STATIC_PATH?
    let clang_include_dirs = clang::collect_clang_includes();
    let (clang_lib_dirs, clang_libs) = clang::collect_clang_libs();

    // ===== Protobuf =====

    let proto_include_dirs = protobuf::collect_protobuf_includes();
    let (proto_lib_dirs, proto_libs) = protobuf::collect_protobuf_libs();
    let gen_proto_sources =
        protobuf::collect_generated_proto_sources(proto_sources, &proto_include_dirs);

    // ===== Linking directives =====

    // Combine and deduplicate link search paths and libraries across dependencies.
    // `BTreeSet` deduplicates and sorts without an additional `itertools` dep.
    let all_lib_dirs = [absl_lib_dirs, clang_lib_dirs, proto_lib_dirs]
        .into_iter()
        .flatten()
        .collect::<BTreeSet<_>>();
    paths::print_link_searches(all_lib_dirs)?;
    let all_libs =
        [absl_libs, clang_libs, proto_libs].into_iter().flatten().collect::<BTreeSet<_>>();
    paths::print_link_libs(all_libs)?;

    // ===== The cc lib ======

    let mut cc_lib = cc::Build::new();
    cc_lib.out_dir(&obj_dir);
    for f in flags::CC_FLAGS {
        cc_lib.flag(f);
    }
    cc_lib.include(path_to_src_root.as_ref());
    let mut num_sources = 0;
    for p in sources.iter().map(|p| path_to_src_root.as_ref().join(p.as_ref())) {
        if p.exists() {
            paths::add_source_file(&mut cc_lib, &p)?;
            num_sources += 1;
        } else {
            // Trigger a rebuild if a copybara-stripped file is added later
            println!("cargo::rerun-if-changed={}", p.display());
            println!("cargo::warning=Skipping internal-only source file: {}", p.display());
        }
    }
    for p in &gen_proto_sources {
        paths::add_source_file(&mut cc_lib, p)?;
        num_sources += 1;
    }
    if num_sources == 0 {
        let placeholder = obj_dir.join("empty.cc");
        std::fs::write(&placeholder, "// Empty placeholder for header-only library\n")?;
        cc_lib.file(&placeholder);
    }
    for p in absl_include_dirs.into_iter().chain(clang_include_dirs).chain(proto_include_dirs) {
        paths::add_include_path(&mut cc_lib, p, false);
    }
    cc_lib.cpp(true);
    cc_lib.compile(&name);

    paths::print_link_search(&obj_dir)?;
    paths::print_link_libs(&[name])?;

    Ok(())
}
