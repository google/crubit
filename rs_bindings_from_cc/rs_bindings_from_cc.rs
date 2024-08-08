// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use std::ffi::{c_char, CString};

fn main() -> Result<(), i32> {
    let mut args: Vec<Vec<u8>> = std::env::args_os()
        .map(|s| {
            CString::new(s.into_encoded_bytes())
                .unwrap()
                .into_bytes_with_nul()
        })
        .collect();
    // Pointers to each Vec storage in `args`, which must outlive this.
    let mut ptrs: Vec<*mut c_char> = args
        .iter_mut()
        .map(|s| s.as_mut_ptr() as *mut c_char)
        .collect();
    let argc: i32 = ptrs.len().try_into().unwrap();
    let argv: *mut *mut c_char = ptrs.as_mut_ptr();
    let r = unsafe { rs_bindings_from_cc_sys::crubit_rs_bindings_from_cc_main(argc, argv) };
    if r == 0 {
        Ok(())
    } else {
        Err(r)
    }
}
