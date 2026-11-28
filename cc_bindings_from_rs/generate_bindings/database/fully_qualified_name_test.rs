// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#![feature(rustc_private)]

extern crate rustc_middle;
extern crate rustc_span;

use database::{
    rename_c_stdlib_functions, rename_clang_builtin_macros, CratePublicPaths, ExportedPath,
    MergedPublicPaths,
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
        cc_alias_name: None,
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

fn spellings<'a>(paths: impl Iterator<Item = &'a ExportedPath>) -> Vec<String> {
    paths.map(spelling).collect()
}

/// The paths that each module of `krate` exports, a module at a time.
fn module_spellings(paths: &MergedPublicPaths, krate: CrateNum) -> Vec<Vec<String>> {
    paths
        .modules(krate)
        .into_iter()
        .map(|module| module.map(|index| spelling(paths.path(index))).collect())
        .collect()
}

/// The smallest path of a definition is its canonical path, whatever the order the paths arrive
/// in, and the others are its aliases in sorted order.
#[test]
fn test_crate_public_paths_picks_canonical_path_and_sorts_aliases() {
    rustc_span::create_default_session_globals_then(|| {
        let item = LOCAL_CRATE.as_def_id();
        let other_item = DefId { krate: LOCAL_CRATE, index: DefIndex::from_u32(1) };
        let mut paths = CratePublicPaths::default();
        paths.insert_module([exported_path(&["b", "c"], "Item")]);
        paths.insert_module([exported_path(&["z"], "Item")]);
        paths.insert_module([ExportedPath { is_doc_hidden: true, ..exported_path(&[], "Item") }]);
        paths.insert_module([exported_path(&["a"], "Renamed")]);
        paths.insert_module([exported_path(&["a"], "Item")]);
        paths.insert_module([ExportedPath { def_id: other_item, ..exported_path(&[], "Other") }]);
        // A path equal to an alias is not stored again.
        paths.insert_module([exported_path(&["z"], "Item")]);

        assert_eq!(spelling(paths.canonical(item).unwrap()), "a::Item");
        assert_eq!(spellings(paths.aliases(item)), ["a::Renamed", "z::Item", "b::c::Item", "Item"]);
        assert_eq!(spelling(paths.canonical(other_item).unwrap()), "Other");
        assert_eq!(paths.aliases(other_item).count(), 0);

        assert_eq!(paths.def_ids().collect::<HashSet<_>>(), HashSet::from([item, other_item]));
        let unknown_item = DefId { krate: LOCAL_CRATE, index: DefIndex::from_u32(2) };
        assert!(paths.canonical(unknown_item).is_none());
        assert_eq!(paths.aliases(unknown_item).count(), 0);
    });
}

fn crate_named(tcx: TyCtxt<'_>, name: &str) -> CrateNum {
    *tcx.crates(()).iter().find(|krate| tcx.crate_name(**krate).as_str() == name).unwrap()
}

/// The paths of a definition, with each path's crate, after the crates in `krate_paths` have been
/// merged in that order: the canonical path, then the aliases.
fn combined_paths(
    tcx: TyCtxt<'_>,
    def_id: DefId,
    krate_paths: &[&CratePublicPaths],
) -> ((CrateNum, String), Vec<String>) {
    let mut all_paths = MergedPublicPaths::default();
    for paths in krate_paths {
        all_paths.extend(tcx, paths, |kept_def_id| kept_def_id == def_id);
    }
    let canonical = all_paths.canonical(def_id).unwrap();
    ((canonical.krate, spelling(canonical)), spellings(all_paths.aliases(def_id)))
}

/// When the paths of several crates are combined, the crate that ranks highest for a definition
/// supplies its canonical path, whichever crate is added first, and between crates of the same
/// rank the smallest canonical path wins. The paths of each other crate follow the aliases of the
/// canonical path's crate, in their own order.
#[test]
fn test_merged_public_paths_extend_respects_crate_ownership() {
    run_compiler_for_testing("", |tcx| {
        let (core, std) = (crate_named(tcx, "core"), crate_named(tcx, "std"));
        let table = |paths: &[ExportedPath]| {
            let mut table = CratePublicPaths::default();
            table.insert_module(paths.iter().cloned());
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
        let mut kept_paths = MergedPublicPaths::default();
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

/// The paths inserted together as one module are listed together, in the order they were given.
#[test]
fn test_merged_public_paths_lists_paths_by_module() {
    run_compiler_for_testing("", |tcx| {
        let item = LOCAL_CRATE.as_def_id();
        let other_item = DefId { krate: LOCAL_CRATE, index: DefIndex::from_u32(1) };
        let mut paths = CratePublicPaths::default();
        paths.insert_module([
            exported_path(&[], "Item"),
            ExportedPath { def_id: other_item, ..exported_path(&[], "Other") },
        ]);
        // A module that exports nothing is not listed.
        paths.insert_module([]);
        paths.insert_module([
            exported_path(&["m"], "Renamed"),
            ExportedPath { def_id: other_item, ..exported_path(&["m"], "Other") },
            exported_path(&["m"], "Item"),
            // A path equal to an alias is not stored, so its module does not list it twice.
            exported_path(&["m"], "Renamed"),
        ]);
        let mut all_paths = MergedPublicPaths::default();
        all_paths.extend(tcx, &paths, |_| true);

        assert_eq!(
            module_spellings(&all_paths, LOCAL_CRATE),
            [vec!["Item", "Other"], vec!["m::Renamed", "m::Other", "m::Item"]]
        );
        assert!(all_paths.modules(CrateNum::from_u32(1)).is_empty());
        assert_eq!(spelling(all_paths.canonical(item).unwrap()), "Item");
        assert_eq!(spellings(all_paths.aliases(item)), ["m::Item", "m::Renamed"]);
    });
}

/// After the paths of a second crate are added, each crate's modules still list their own paths,
/// and a path reached through its module is the one reached through its definition.
#[test]
fn test_merged_public_paths_modules_survive_extend() {
    run_compiler_for_testing("", |tcx| {
        let core = crate_named(tcx, "core");
        let core_item = core.as_def_id();
        let local_item = LOCAL_CRATE.as_def_id();
        let path = |def_id: DefId, krate: CrateNum, path: &[&str], name: &str| ExportedPath {
            def_id,
            krate,
            ..exported_path(path, name)
        };

        let mut core_paths = CratePublicPaths::default();
        core_paths.insert_module([
            path(core_item, core, &["m"], "Item"),
            path(core_item, core, &["m"], "Renamed"),
        ]);
        core_paths.insert_module([path(core_item, core, &["m", "n"], "Deep")]);
        let mut local_paths = CratePublicPaths::default();
        local_paths.insert_module([
            path(core_item, LOCAL_CRATE, &[], "Item"),
            path(local_item, LOCAL_CRATE, &[], "Local"),
        ]);
        local_paths.insert_module([path(local_item, LOCAL_CRATE, &["x"], "Local")]);

        let mut all_paths = MergedPublicPaths::default();
        all_paths.extend(tcx, &core_paths, |_| true);
        all_paths.extend(tcx, &local_paths, |_| true);

        assert_eq!(
            module_spellings(&all_paths, core),
            [vec!["m::Item", "m::Renamed"], vec!["m::n::Deep"]]
        );
        assert_eq!(
            module_spellings(&all_paths, LOCAL_CRATE),
            [vec!["Item", "Local"], vec!["x::Local"]]
        );
        let local_modules = all_paths
            .modules(LOCAL_CRATE)
            .into_iter()
            .map(|module| module.collect::<Vec<_>>())
            .collect::<Vec<_>>();
        for index in local_modules.iter().flatten() {
            assert_eq!(all_paths.path(*index).krate, LOCAL_CRATE);
        }

        // `core` defines `core_item`, so the local crate's path for it is an alias, while the
        // local crate's shortest path for its own definition is canonical.
        let (reexport, local, nested_local) =
            (local_modules[0][0], local_modules[0][1], local_modules[1][0]);
        assert!(!all_paths.is_canonical(reexport));
        assert!(all_paths.is_canonical(local));
        assert!(!all_paths.is_canonical(nested_local));

        // A name stored through a module's path is seen from the path's definition.
        let (alias_name, canonical_name) = (Symbol::intern("Item_"), Symbol::intern("Local_"));
        all_paths.set_cc_alias_name(reexport, Ok(alias_name));
        all_paths.set_cc_alias_name(local, Ok(canonical_name));
        let named = |path: &ExportedPath, expected: Symbol| {
            path.cc_alias_name
                .as_ref()
                .is_some_and(|name| name.as_ref().is_ok_and(|n| *n == expected))
        };
        let aliases = all_paths.aliases(core_item).collect::<Vec<_>>();
        assert_eq!(aliases.len(), 3);
        for alias in aliases {
            assert_eq!(named(alias, alias_name), alias.krate == LOCAL_CRATE);
        }
        assert!(named(all_paths.canonical(local_item).unwrap(), canonical_name));
        assert!(all_paths.canonical(core_item).unwrap().cc_alias_name.is_none());
        assert!(local_paths.canonical(local_item).unwrap().cc_alias_name.is_none());
    });
}
