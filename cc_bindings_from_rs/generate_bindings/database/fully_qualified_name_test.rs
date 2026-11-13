// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#![feature(rustc_private)]

extern crate rustc_middle;
extern crate rustc_span;

use database::{
    rename_c_stdlib_functions, rename_clang_builtin_macros, ExportedPath, PublicPathTable,
};
use run_compiler_test_support::run_compiler_for_testing;
use rustc_middle::ty::TyCtxt;
use rustc_span::def_id::{CrateNum, DefId, DefIndex, LOCAL_CRATE};
use rustc_span::symbol::Symbol;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::rc::Rc;

#[test]
fn test_rename_clang_builtin_macros() {
    for macro_name in ["unix", "linux", "WIN32", "WINNT", "WIN64", "spirv", "sun"] {
        assert_eq!(
            rename_clang_builtin_macros(Rc::from(macro_name)).as_ref(),
            format!("rs_{macro_name}")
        );
    }
    assert_eq!(rename_clang_builtin_macros(Rc::from("remove")).as_ref(), "remove");
    assert_eq!(rename_clang_builtin_macros(Rc::from("memchr")).as_ref(), "memchr");
    assert_eq!(rename_clang_builtin_macros(Rc::from("my_module")).as_ref(), "my_module");
}

#[test]
fn test_rename_c_stdlib_functions() {
    for func in [
        "remove",
        "rename",
        "free",
        "exit",
        "abort",
        "signal",
        "system",
        "malloc",
        "calloc",
        "realloc",
        "memcpy",
        "memmove",
        "memset",
        "strcpy",
        "strncpy",
        "strcat",
        "strncat",
        "strcmp",
        "strncmp",
        "strlen",
        "strchr",
        "strrchr",
        "strstr",
        "strtok",
        "strerror",
        "memchr",
        "abs",
        "labs",
        "llabs",
        "div",
        "ldiv",
        "lldiv",
        "rand",
        "srand",
        "bsearch",
        "qsort",
        "getenv",
        "sin",
        "cos",
        "tan",
        "asin",
        "acos",
        "atan",
        "atan2",
        "sinh",
        "cosh",
        "tanh",
        "exp",
        "log",
        "log10",
        "pow",
        "sqrt",
        "ceil",
        "floor",
        "fabs",
        "frexp",
        "ldexp",
        "modf",
        "clock",
        "difftime",
        "mktime",
        "time",
        "asctime",
        "ctime",
        "gmtime",
        "localtime",
        "strftime",
        "printf",
        "scanf",
        "puts",
        "fopen",
        "fclose",
        "fread",
        "fwrite",
        "fseek",
        "ftell",
        "rewind",
        "perror",
        "tmpfile",
        "tmpnam",
    ] {
        assert_eq!(rename_c_stdlib_functions(Rc::from(func)).as_ref(), format!("rs_{func}"));
    }
    assert_eq!(rename_c_stdlib_functions(Rc::from("unix")).as_ref(), "unix");
    assert_eq!(rename_c_stdlib_functions(Rc::from("linux")).as_ref(), "linux");
    assert_eq!(rename_c_stdlib_functions(Rc::from("WIN32")).as_ref(), "WIN32");
    assert_eq!(rename_c_stdlib_functions(Rc::from("my_module")).as_ref(), "my_module");
}

fn exported_path(path: &[&str], name: &str) -> ExportedPath {
    ExportedPath {
        def_id: LOCAL_CRATE.as_def_id(),
        path: path.iter().map(|s| Symbol::intern(s)).collect(),
        name: Symbol::intern(name),
        type_alias_def_id: None,
        is_doc_hidden: false,
        is_reexport: false,
        krate: LOCAL_CRATE,
    }
}

/// `ExportedPath`'s final tie-break must compare path segments as strings, not by interner index,
/// which depends on the order in which the names were interned.
#[test]
fn test_exported_path_ord_ignores_interning_order() {
    rustc_span::create_default_session_globals_then(|| {
        // Intern the lexicographically larger segment first, so that its interner index is
        // smaller.
        let later = exported_path(&["zzz_crubit_test_module"], "Item");
        let earlier = exported_path(&["aaa_crubit_test_module"], "Item");
        assert!(later.path[0].as_u32() < earlier.path[0].as_u32());
        assert_eq!(earlier.cmp(&later), Ordering::Less);
        assert_eq!(later.cmp(&earlier), Ordering::Greater);
    });
}

/// Spells out a path, for comparing lists of paths.
fn spelling(path: &ExportedPath) -> String {
    path.path.iter().chain([&path.name]).map(Symbol::as_str).collect::<Vec<_>>().join("::")
}

fn alias_spellings(paths: &PublicPathTable, def_id: DefId) -> Vec<String> {
    paths.aliases(def_id).map(spelling).collect()
}

/// The smallest path of a definition is its canonical path, whatever the order the paths arrive
/// in, and the others are its aliases in sorted order.
#[test]
fn test_public_path_table_picks_canonical_path_and_sorts_aliases() {
    rustc_span::create_default_session_globals_then(|| {
        let item = LOCAL_CRATE.as_def_id();
        let other_item = DefId { krate: LOCAL_CRATE, index: DefIndex::from_u32(1) };
        let mut paths = PublicPathTable::default();
        paths.insert(exported_path(&["b", "c"], "Item"));
        paths.insert(exported_path(&["z"], "Item"));
        paths.insert(ExportedPath { is_doc_hidden: true, ..exported_path(&[], "Item") });
        paths.insert(exported_path(&["a"], "Renamed"));
        paths.insert(exported_path(&["a"], "Item"));
        paths.insert(ExportedPath { def_id: other_item, ..exported_path(&[], "Other") });
        // A path equal to an alias is not stored again.
        paths.insert(exported_path(&["z"], "Item"));

        assert_eq!(spelling(paths.canonical(item).unwrap()), "a::Item");
        assert_eq!(alias_spellings(&paths, item), ["a::Renamed", "z::Item", "b::c::Item", "Item"]);
        assert_eq!(spelling(paths.canonical(other_item).unwrap()), "Other");
        assert_eq!(alias_spellings(&paths, other_item), [] as [&str; 0]);

        assert_eq!(paths.def_ids().collect::<HashSet<_>>(), HashSet::from([item, other_item]));
        let unknown_item = DefId { krate: LOCAL_CRATE, index: DefIndex::from_u32(2) };
        assert!(paths.canonical(unknown_item).is_none());
        assert_eq!(paths.aliases(unknown_item).count(), 0);
    });
}

/// The paths of a definition, with each path's crate, after the tables in `krate_paths` have been
/// added to an empty table in that order: the canonical path, then the aliases.
fn combined_paths(
    tcx: TyCtxt<'_>,
    def_id: DefId,
    krate_paths: &[&PublicPathTable],
) -> ((CrateNum, String), Vec<String>) {
    let mut all_paths = PublicPathTable::default();
    for paths in krate_paths {
        all_paths.extend(tcx, paths, |kept_def_id| kept_def_id == def_id);
    }
    let canonical = all_paths.canonical(def_id).unwrap();
    ((canonical.krate, spelling(canonical)), alias_spellings(&all_paths, def_id))
}

/// When the paths of several crates are combined, the crate that ranks highest for a definition
/// supplies its canonical path, whichever crate is added first, and between crates of the same
/// rank the smallest canonical path wins. The paths of each other crate follow the aliases of the
/// canonical path's crate, in their own order.
#[test]
fn test_public_path_table_extend_respects_crate_ownership() {
    run_compiler_for_testing("", |tcx| {
        let crate_named = |name: &str| {
            *tcx.crates(()).iter().find(|krate| tcx.crate_name(**krate).as_str() == name).unwrap()
        };
        let (core, std) = (crate_named("core"), crate_named("std"));
        let table = |paths: &[ExportedPath]| {
            let mut table = PublicPathTable::default();
            paths.iter().cloned().for_each(|path| table.insert(path));
            table
        };
        let path = |def_id: DefId, krate: CrateNum, path: &[&str], name: &str| ExportedPath {
            def_id,
            krate,
            ..exported_path(path, name)
        };

        // A definition of `core` that `core`, `std` and the local crate each have paths for.
        let core_item = core.as_def_id();
        let core_paths = table(&[
            path(core_item, core, &["m", "n"], "Deep"),
            path(core_item, core, &["m"], "Item"),
            path(std.as_def_id(), core, &[], "NotKept"),
        ]);
        let std_paths = table(&[path(core_item, std, &["s", "t"], "Item")]);
        let local_paths = table(&[
            path(core_item, LOCAL_CRATE, &["x"], "Alias"),
            path(core_item, LOCAL_CRATE, &[], "Item"),
        ]);

        // The defining crate keeps its canonical path, although the local crate's is shorter.
        for krate_paths in [[&local_paths, &core_paths], [&core_paths, &local_paths]] {
            let (canonical, aliases) = combined_paths(tcx, core_item, &krate_paths);
            assert_eq!(canonical, (core, "m::Item".into()));
            assert_eq!(aliases, ["m::n::Deep", "Item", "x::Alias"]);
        }

        // `std` re-exporting a definition of `core` outranks any other re-exporting crate.
        for krate_paths in [[&local_paths, &std_paths], [&std_paths, &local_paths]] {
            let (canonical, aliases) = combined_paths(tcx, core_item, &krate_paths);
            assert_eq!(canonical, (std, "s::t::Item".into()));
            assert_eq!(aliases, ["Item", "x::Alias"]);
        }

        // The defining crate outranks `std` as well.
        for krate_paths in
            [[&local_paths, &std_paths, &core_paths], [&core_paths, &std_paths, &local_paths]]
        {
            let (canonical, aliases) = combined_paths(tcx, core_item, &krate_paths);
            assert_eq!(canonical, (core, "m::Item".into()));
            assert_eq!(aliases, ["m::n::Deep", "s::t::Item", "Item", "x::Alias"]);
        }

        // A definition is only added if it is kept.
        let mut kept_paths = PublicPathTable::default();
        kept_paths.extend(tcx, &core_paths, |def_id| def_id == core_item);
        assert!(kept_paths.canonical(std.as_def_id()).is_none());

        // A definition of `std` that only `core` and the local crate have paths for: neither
        // crate owns it, so the smallest of all their paths is canonical.
        let std_item = std.as_def_id();
        let core_paths = table(&[
            path(std_item, core, &["a", "b"], "Item"),
            path(std_item, core, &["a"], "Item"),
        ]);
        let local_paths = table(&[
            path(std_item, LOCAL_CRATE, &["x"], "Alias"),
            path(std_item, LOCAL_CRATE, &[], "Item"),
        ]);
        for krate_paths in [[&local_paths, &core_paths], [&core_paths, &local_paths]] {
            let (canonical, aliases) = combined_paths(tcx, std_item, &krate_paths);
            assert_eq!(canonical, (LOCAL_CRATE, "Item".into()));
            assert_eq!(aliases, ["x::Alias", "a::Item", "a::b::Item"]);
        }

        // When the canonical paths of two such crates compare equal, the crate added first keeps
        // its path canonical.
        let core_paths = table(&[path(std_item, core, &[], "Item")]);
        let local_paths = table(&[path(std_item, LOCAL_CRATE, &[], "Item")]);
        for (krate_paths, first_krate) in
            [([&local_paths, &core_paths], LOCAL_CRATE), ([&core_paths, &local_paths], core)]
        {
            let (canonical, aliases) = combined_paths(tcx, std_item, &krate_paths);
            assert_eq!(canonical, (first_krate, "Item".into()));
            assert_eq!(aliases, ["Item"]);
        }
    });
}
