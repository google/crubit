// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! This crate is used as a test input for `cc_bindings_from_rs` and the
//! generated C++ bindings are then tested via `functions_test.cc`.

/// APIs for testing various function calling conventions and linking options:
/// - `#[no_mangle]`
/// - `#[export_name = ...]`
/// - `extern "C"` vs default/Rust ABI
/// - etc.
pub mod fn_abi_tests {

    /// Testing one of simpler function bindings:
    /// - `extern "C"` means that no thunk is required
    /// - `#[no_mangle]` means that the function is already exposed with the
    ///   desired, public name (and just needs to be redeclared in C++).
    #[no_mangle]
    pub extern "C" fn get_42_as_f64_via_no_mangle_extern_c() -> f64 {
        42.0
    }

    /// Testing `#[export_name = ...]` - the generated bindings need to
    /// forward/proxy the call into a function with a different name.
    #[export_name = "custom_export_name_for_add_i32"]
    pub extern "C" fn add_i32_via_extern_c_with_export_name(x: i32, y: i32) -> i32 {
        x + y
    }

    /// Testing bindings for an `extern "C"` function (no thunk required) with a
    /// mangled name. This test verifies that:
    /// * `cc_bindings_from_rs` can correctly discover mangled names that
    ///   `rustc` produces
    /// * Bazel support for `cc_bindings_from_rs` invokes it with the same
    ///   command line flags as the ones used when invoking `rustc` when
    ///   building the `functions` crate.
    ///
    /// TODO(b/262904507): Bazel integration is currently broken and the
    /// coresponding test is commented out in `functions_test.cc`.
    pub extern "C" fn add_i32_via_extern_c_with_mangling(x: i32, y: i32) -> i32 {
        x + y
    }

    /// Testing the default / Rust ABI (one used in absence of `extern "C"`).
    pub fn add_i32_via_rust_abi(x: i32, y: i32) -> i32 {
        x + y
    }
}

/// APIs for testing various kinds of function parameter types.
pub mod fn_param_ty_tests {
    /// Testing a type that maps to a built-in C++ type (spelled with a
    /// keyword). `float` is one such example.
    pub fn add_f64(x: f64, y: f64) -> f64 {
        x + y
    }

    /// Testing a type that requires `#include`ing a standard C++ header.
    /// `std::int32_t` is one such example - it requires `#include <cstdint>`.
    pub fn add_i32(x: i32, y: i32) -> i32 {
        x + y
    }
}

/// APIs for testing functions that return the unit / `()` / `void` type.
pub mod unit_ret_ty_tests {
    use std::sync::Mutex;

    static G_I32: Mutex<i32> = Mutex::new(0);

    // Presence of the API below tests how bindings handle functions returning
    // `void`.
    #[export_name = "custom_export_name_for_get_global_i32"]
    pub extern "C" fn set_global_i32_via_extern_c_with_export_name(x: i32) {
        *G_I32.lock().unwrap() = x;
    }

    #[no_mangle]
    pub extern "C" fn get_global_i32_via_extern_c_with_export_name() -> i32 {
        *G_I32.lock().unwrap()
    }
}

pub mod other_fn_param_tests {
    pub fn add_i32_via_rust_abi_with_duplicated_param_names(x: i32, y: i32, _: i32, _: i32) -> i32 {
        x + y
    }
}
