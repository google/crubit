// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! This crate is used as a test input for `cc_bindings_from_rs` and the
//! generated C++ bindings are then tested via `structs_test.cc`.

pub fn create_repr_c_point_via_free_function(x: i32, y: i32) -> ReprCPoint {
    ReprCPoint { x, y }
}

pub fn get_x_of_repr_c_point_via_free_function(p: ReprCPoint) -> i32 {
    p.x
}

pub fn create_default_repr_point_via_free_function(x: i32, y: i32) -> DefaultReprPoint {
    DefaultReprPoint { x, y }
}

pub fn get_x_of_default_repr_point_via_free_function(p: DefaultReprPoint) -> i32 {
    p.x
}

// `ReprCPoint` is defined *after* functions that take or return this type,
// because preserving this order in the generated `..._cc_api.h` file would
// trigger a C++ error (because in C++ one can't refer to a struct before it has
// been defined).
#[repr(C)]
pub struct ReprCPoint {
    pub x: i32,
    pub y: i32,
}

// `DefaultReprPoint` is defined *after* functions that take or return this
// type, because preserving this order in the generated `..._cc_api.h` file
// would trigger a C++ error (because in C++ one can't refer to a struct before
// it has been defined).
pub struct DefaultReprPoint {
    pub x: i32,
    pub y: i32,
}
