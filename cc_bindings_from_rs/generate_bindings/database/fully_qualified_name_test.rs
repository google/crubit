// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use database::{rename_c_stdlib_functions, rename_clang_builtin_macros};
use std::rc::Rc;

#[test]
fn test_rename_clang_builtin_macros() {
    assert_eq!(rename_clang_builtin_macros(Rc::from("unix")).as_ref(), "rs_unix");
    assert_eq!(rename_clang_builtin_macros(Rc::from("linux")).as_ref(), "rs_linux");
    assert_eq!(rename_clang_builtin_macros(Rc::from("remove")).as_ref(), "remove");
    assert_eq!(rename_clang_builtin_macros(Rc::from("my_module")).as_ref(), "my_module");
}

#[test]
fn test_rename_c_stdlib_functions() {
    assert_eq!(rename_c_stdlib_functions(Rc::from("remove")).as_ref(), "rs_remove");
    assert_eq!(rename_c_stdlib_functions(Rc::from("rename")).as_ref(), "rs_rename");
    assert_eq!(rename_c_stdlib_functions(Rc::from("free")).as_ref(), "rs_free");
    assert_eq!(rename_c_stdlib_functions(Rc::from("exit")).as_ref(), "rs_exit");
    assert_eq!(rename_c_stdlib_functions(Rc::from("unix")).as_ref(), "unix");
    assert_eq!(rename_c_stdlib_functions(Rc::from("my_module")).as_ref(), "my_module");
}
