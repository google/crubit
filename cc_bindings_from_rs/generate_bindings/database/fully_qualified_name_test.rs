// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#![feature(rustc_private)]

extern crate rustc_span;

use database::{rename_c_stdlib_functions, rename_clang_builtin_macros, ExportedPath};
use rustc_span::def_id::LOCAL_CRATE;
use rustc_span::symbol::Symbol;
use std::cmp::Ordering;
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
