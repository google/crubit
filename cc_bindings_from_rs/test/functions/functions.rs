// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! This crate is used as a test input for `cc_bindings_from_rs` and the
//! generated C++ bindings are then tested via `functions_test.cc`.

/// APIs for testing various function calling conventions and linking options:
/// - `#[unsafe(no_mangle)]`
/// - `#[unsafe(export_name = ...)]`
/// - `extern "C"` vs default/Rust ABI
/// - etc.
pub mod fn_abi_tests {

    /// Testing one of simpler function bindings:
    /// - `extern "C"` means that no thunk is required
    /// - `#[unsafe(no_mangle)]` means that the function is already exposed with
    ///   the desired, public name (and just needs to be redeclared in C++).
    #[unsafe(no_mangle)]
    pub extern "C" fn get_42_as_f64_via_no_mangle_extern_c() -> f64 {
        42.0
    }

    /// Testing `#[unsafe(export_name = ...)]` - the generated bindings need to
    /// forward/proxy the call into a function with a different name.
    #[unsafe(export_name = "custom_export_name_for_add_i32")]
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

    pub fn add_i32_via_ptr(x: *const i32, y: *const i32, sum: *mut i32) {
        #![allow(clippy::not_unsafe_ptr_arg_deref)]
        unsafe {
            *sum = *x + *y;
        }
    }

    pub fn char_to_ascii_lowercase(c: char) -> char {
        // This function used to return unmodified `c` value, but (as we learned when
        // authoring `rs_bindings_from_cc/test/struct/abi_class` tests) making
        // some simple calculations below helps to exercise the ABI
        // compatibility between Rust `char` and C++ `rs_std::rs_char`.
        c.to_ascii_lowercase()
    }

    pub fn apply_binary_i32_op(x: i32, y: i32, f: extern "C" fn(i32, i32) -> i32) -> i32 {
        f(x, y)
    }

    pub fn get_ref_to_smaller_int<'a>(x: &'a i32, y: &'a i32) -> &'a i32 {
        if *x < *y { x } else { y }
    }

    pub fn get_identical_ref_with_inferred_lifetime(x: &'_ i32) -> &'_ i32 {
        x
    }

    pub fn set_mut_ref_to_sum_of_ints(sum: &mut i32, x: i32, y: i32) {
        *sum = x + y;
    }
}

/// APIs for testing functions that return the unit / `()` / `void` type.
pub mod unit_ret_ty_tests {
    use std::sync::Mutex;

    static G_I32: Mutex<i32> = Mutex::new(0);

    // Presence of the API below tests how bindings handle functions returning
    // `void`.
    #[unsafe(export_name = "custom_export_name_for_get_global_i32")]
    pub extern "C" fn set_global_i32_via_extern_c_with_export_name(x: i32) {
        *G_I32.lock().unwrap() = x;
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn get_global_i32_via_extern_c_with_export_name() -> i32 {
        *G_I32.lock().unwrap()
    }
}

pub mod other_fn_param_tests {
    pub fn add_i32_via_rust_abi_with_duplicated_param_names(x: i32, y: i32, _: i32, _: i32) -> i32 {
        x + y
    }
}

pub mod fn_attribute_tests {
    #[deprecated(since = "1.2.3", note = "★ Deprecated note for add_i32 ★")]
    pub fn add_i32(x: i32, y: i32) -> i32 {
        x + y
    }
}

pub mod unsafe_fn_tests {
    /// # Safety
    ///
    /// This function has no safety requirements - it is only marked as `unsafe`
    /// to facilitate minimal testing of bindings generated for such functions.
    pub unsafe fn unsafe_add(x: i32, y: i32) -> i32 {
        x + y
    }
}

// Tests the use of the #[must_use] attribute
pub mod fn_must_use_tests {
    #[must_use]
    pub fn no_msg_add(x: i32, y: i32) -> i32 {
        x + y
    }

    #[must_use = "woohoo"]
    pub fn msg_add(x: i32, y: i32) -> i32 {
        x + y
    }
}
