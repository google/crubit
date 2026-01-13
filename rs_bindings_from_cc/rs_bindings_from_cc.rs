// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use std::ffi::{c_char, c_int, CString};
use std::process::ExitCode;

unsafe extern "C" {
    // In rs_bindings_from_cc.cc:
    fn crubit_rs_bindings_from_cc_main(argc: c_int, argv: *mut *mut c_char) -> std::ffi::c_int;
}

pub fn main() -> ExitCode {
    let mut args: Vec<Vec<u8>> = std::env::args_os()
        .map(|s| CString::new(s.into_encoded_bytes()).unwrap().into_bytes_with_nul())
        .collect();
    // Pointers to each Vec storage in `args`, which must outlive this.
    let mut ptrs: Vec<*mut c_char> =
        args.iter_mut().map(|s| s.as_mut_ptr() as *mut c_char).collect();
    let argc: i32 = ptrs.len().try_into().unwrap();
    let argv: *mut *mut c_char = ptrs.as_mut_ptr();
    let r = unsafe { crubit_rs_bindings_from_cc_main(argc, argv) };
    ExitCode::from(u8::try_from(r).unwrap_or(u8::MAX))
}
