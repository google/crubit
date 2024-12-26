// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

extern crate alloc;

use crate::crubit_cc_std_internal::conversion_function_helpers;
use alloc::string::String;
use alloc::vec::Vec;
use core::ffi::c_void;
use core::mem::MaybeUninit;
use core::ops::Deref;

#[allow(non_snake_case)]
#[repr(C)]
#[derive(Debug, Default, Eq, PartialEq)]
pub struct string {
    data: Vec<u8>,
}

impl From<String> for string {
    fn from(s: String) -> Self {
        Self { data: s.into_bytes() }
    }
}

impl From<&str> for string {
    fn from(s: &str) -> Self {
        Self { data: s.as_bytes().to_vec() }
    }
}

impl Deref for string {
    type Target = Vec<u8>;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_string_to_cpp_string(x: *const c_void, output: *mut c_void) {
    let x = unsafe { &*(x as *const string) };
    let bytes = x.data.as_slice();
    let size = bytes.len();
    let buffer = bytes.as_ptr();
    unsafe {
        conversion_function_helpers::StringCreateInPlace(output, buffer as _, size);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cpp_string_to_rust_string(input: *const c_void, output: *mut c_void) {
    unsafe {
        let size = conversion_function_helpers::StringGetSize(input);
        let buffer = conversion_function_helpers::StringGetData(input) as *const u8;
        //TODO(b/351976622): Remove this copy and store the string directly.
        let copy_buffer = core::slice::from_raw_parts(buffer, size).to_vec();
        let output = &mut *(output as *mut MaybeUninit<string>);
        output.as_mut_ptr().write(string { data: copy_buffer });
    }
}
