// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#![feature(register_tool)]
#![register_tool(__crubit)]

use std::ffi::c_void;
use std::marker::PhantomData;

#[__crubit::annotate(
    cpp_type = "crubit::test::TheCppType*",
    cpp_type_include = "cc_bindings_from_rs/test/bridging/cc_type.h",
    rust_to_cpp_converter = "crubit_test_rust_owned_struct_to_cpp_owned_pointer",
    cpp_to_rust_converter = "crubit_test_cpp_owned_pointer_to_rust_owned_struct"
)]
pub struct RustOwned {
    ptr: *mut c_void,
}

pub fn pass_through(val: RustOwned) -> RustOwned {
    val
}

#[__crubit::annotate(
    cpp_type = "const crubit::test::TheCppType*",
    cpp_type_include = "cc_bindings_from_rs/test/bridging/cc_type.h"
)]
#[repr(transparent)]
pub struct RustView<'a> {
    ptr: *const core::ffi::c_void,
    _phantom: PhantomData<&'a RustOwned>,
}

impl RustView<'_> {
    pub fn x(&self) -> i32 {
        unsafe { type_converters::crubit_test_cpp_type_get_x(self.ptr) }
    }
}

pub fn get_x_from_view(view: RustView) -> i32 {
    view.x()
}

mod type_converters {
    use super::*;
    use std::ffi::{c_int, c_void};

    #[no_mangle]
    pub unsafe extern "C" fn crubit_test_rust_owned_struct_to_cpp_owned_pointer(
        rs_in: *const c_void,
        cpp_out: *mut *mut c_void,
    ) {
        unsafe {
            let rs_owned = &*(rs_in as *const RustOwned);
            cpp_out.write(rs_owned.ptr);
        }
    }

    #[no_mangle]
    pub unsafe extern "C" fn crubit_test_cpp_owned_pointer_to_rust_owned_struct(
        cpp_in: *mut c_void,
        rs_out: *mut c_void,
    ) {
        unsafe {
            let output = &mut *(rs_out as *mut ::core::mem::MaybeUninit<RustOwned>);
            output.as_mut_ptr().write(RustOwned { ptr: cpp_in });
        }
    }

    extern "C" {
        pub(super) fn crubit_test_cpp_type_get_x(cpp_type: *const c_void) -> c_int;
    }
}
