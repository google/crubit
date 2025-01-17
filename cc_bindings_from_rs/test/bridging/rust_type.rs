// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#![feature(register_tool)]
#![register_tool(__crubit)]

#[__crubit::annotate(
    cpp_type = "crubit::test::TheCppType",
    cpp_type_include = "cc_bindings_from_rs/test/bridging/cc_type.h",
    rust_to_cpp_converter = "convert_rust_to_cpp_type",
    cpp_to_rust_converter = "convert_cpp_to_rust_type"
)]
pub struct TheRustType {
    pub x: i32,
}

#[derive(Default, Clone)]
pub struct NonTriviallyDestructable {
    pub field: i32,
}

impl Drop for NonTriviallyDestructable {
    fn drop(&mut self) {
        self.field = 123;
    }
}

pub fn create_new(x: i32) -> TheRustType {
    TheRustType { x }
}

pub fn get_x(data: TheRustType) -> i32 {
    data.x
}

pub fn into_something_else(data: TheRustType) -> NonTriviallyDestructable {
    NonTriviallyDestructable { field: data.x }
}

mod type_converters {
    use super::*;
    use std::ffi::c_int;
    use std::ffi::c_void;

    /// # Safety
    ///  - rs_in is a valid pointer to an initialized TheRustType.
    ///  - cpp_out is a valid pointer to an uninitialized
    ///    crubit::test::TheCppType.
    #[no_mangle]
    pub unsafe extern "C" fn convert_rust_to_cpp_type(rs_in: *const c_void, cpp_out: *mut c_void) {
        unsafe {
            let rust_type = &*(rs_in as *const TheRustType);
            crubit_test_new_cpp_type(rust_type.x, cpp_out);
        }
    }

    /// # Safety
    ///  - cpp_in is a valid pointer to an uninitialized
    ///    crubit::test::TheCppType.
    ///  - rs_out is a valid pointer to an uninitialized TheRustType.
    #[no_mangle]
    pub unsafe extern "C" fn convert_cpp_to_rust_type(cpp_in: *const c_void, rs_out: *mut c_void) {
        unsafe {
            let output = &mut *(rs_out as *mut ::core::mem::MaybeUninit<TheRustType>);
            let x = crubit_test_cpp_type_get_x(cpp_in);
            output.as_mut_ptr().write(TheRustType { x });
        }
    }

    extern "C" {
        fn crubit_test_new_cpp_type(x: c_int, cc_type_out: *mut c_void);
        fn crubit_test_cpp_type_get_x(cpp_type: *const c_void) -> c_int;
    }
}
