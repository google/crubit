// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use database::{rename_c_stdlib_functions, rename_clang_builtin_macros};
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
