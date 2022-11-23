// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! This crate is used as a test input for `cc_bindings_from_rs` and the
//! generated C++ bindings are then tested via `structs_test.cc`.

// TODO(b/258232820): Add test coverage for non-`#[repr(C)]` struct.  See
// cl/489571653.
#[repr(C)]
pub struct ReprCPoint {
    pub x: i32,
    pub y: i32,
}

// TODO(b/258232820): Remove `#[no_mangle]` and `extern "C"` to extend test
// coverage to thunks. See cl/489571653.
#[no_mangle]
pub extern "C" fn create_repr_c_point_via_free_function(x: i32, y: i32) -> ReprCPoint {
    ReprCPoint { x, y }
}

#[no_mangle]
pub extern "C" fn get_x_of_repr_c_point_via_free_function(p: ReprCPoint) -> i32 {
    p.x
}
