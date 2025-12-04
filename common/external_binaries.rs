// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Note: using std::ffi instead of ffi_11 to avoid a very confusing build-time dependency
// on the run-time support libraries.
use std::ffi::{c_char, CString};
use std::sync::LazyLock;

macro_rules! env_or_default {
    ($name:literal, $default:expr) => {
        // Wrapping in a macro because const unwrap_or isn't stable yet.
        match option_env!($name) {
            Some(value) => value,
            None => $default,
        }
    };
}

pub const RUSTFMT_EXE_PATH: &str = env_or_default!("CRUBIT_RUSTFMT_EXE_PATH", "rustfmt");
pub const CLANG_FORMAT_EXE_PATH: &str =
    env_or_default!("CRUBIT_CLANG_FORMAT_EXE_PATH", "clang-format");

#[no_mangle]
pub extern "C" fn crubit_rustfmt_exe_path() -> *const c_char {
    static C_RUSTFMT_EXE_PATH: LazyLock<CString> =
        LazyLock::new(|| CString::new(RUSTFMT_EXE_PATH).unwrap());
    C_RUSTFMT_EXE_PATH.as_ref().as_ptr()
}

#[no_mangle]
pub extern "C" fn crubit_clang_format_exe_path() -> *const c_char {
    static C_PATH: LazyLock<CString> =
        LazyLock::new(|| CString::new(CLANG_FORMAT_EXE_PATH).unwrap());
    C_PATH.as_ref().as_ptr()
}
