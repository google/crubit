// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

extern crate std;
use core::mem::MaybeUninit;
use std::os::raw::c_void;
use std::slice;
use std::string::String;

pub struct MyRustStruct {
    pub s: String,
}

impl MyRustStruct {
    pub fn new(s: &str) -> Self {
        MyRustStruct { s: String::from(s) }
    }
}

#[no_mangle]
pub extern "C" fn rust_to_cpp_converter(x: *const c_void, output: *mut c_void) {
    unsafe {
        let x = &*(x as *const MyRustStruct);
        crate::ffi_create_my_cpp_struct(x.s.as_ptr() as _, x.s.len(), output);
    }
}

#[no_mangle]
pub extern "C" fn cpp_to_rust_converter(input: *const c_void, output: *mut c_void) {
    unsafe {
        let output = &mut *(output as *mut MaybeUninit<MyRustStruct>);
        let buffer = crate::ffi_get_buffer(input);
        let len = crate::ffi_get_buffer_len(input);
        output.as_mut_ptr().write(MyRustStruct {
            s: String::from_utf8_unchecked(slice::from_raw_parts(buffer as _, len).into()),
        });
    }
}
