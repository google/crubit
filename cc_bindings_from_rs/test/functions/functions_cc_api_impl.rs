// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// functions_golden
// Features: custom_ffi_types, experimental, non_unpin_ctor, std_unique_ptr, std_vector, supported, wrapper

#![allow(unused_unsafe, deprecated, non_snake_case, unreachable_code)]
#![allow(improper_ctypes_definitions)]
#![deny(warnings)]

#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_add_ui32_uvia_uextern_uc_uwith_umangling(
    x: i32,
    y: i32,
) -> i32 {
    unsafe { ::functions_golden::fn_abi_tests::add_i32_via_extern_c_with_mangling(x, y) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_add_ui32_uvia_urust_uabi(x: i32, y: i32) -> i32 {
    unsafe { ::functions_golden::fn_abi_tests::add_i32_via_rust_abi(x, y) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_add_uf64(x: f64, y: f64) -> f64 {
    unsafe { ::functions_golden::fn_param_ty_tests::add_f64(x, y) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_add_ui32(x: i32, y: i32) -> i32 {
    unsafe { ::functions_golden::fn_param_ty_tests::add_i32(x, y) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_add_ui32_uvia_uptr(
    x: *const i32,
    y: *const i32,
    sum: *mut i32,
) -> () {
    unsafe { ::functions_golden::fn_param_ty_tests::add_i32_via_ptr(x, y, sum) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_char_uto_uascii_ulowercase(c: char) -> char {
    unsafe { ::functions_golden::fn_param_ty_tests::char_to_ascii_lowercase(c) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_apply_ubinary_ui32_uop(
    x: i32,
    y: i32,
    f: extern "C" fn(i32, i32) -> i32,
) -> i32 {
    unsafe { ::functions_golden::fn_param_ty_tests::apply_binary_i32_op(x, y, f) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_get_uref_uto_usmaller_uint(
    x: &'static i32,
    y: &'static i32,
) -> &'static i32 {
    unsafe { ::functions_golden::fn_param_ty_tests::get_ref_to_smaller_int(x, y) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_get_uidentical_uref_uwith_uinferred_ulifetime(
    x: &'static i32,
) -> &'static i32 {
    unsafe { ::functions_golden::fn_param_ty_tests::get_identical_ref_with_inferred_lifetime(x) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_set_umut_uref_uto_usum_uof_uints(
    sum: &'static mut i32,
    x: i32,
    y: i32,
) -> () {
    unsafe { ::functions_golden::fn_param_ty_tests::set_mut_ref_to_sum_of_ints(sum, x, y) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_add_ui32_uvia_urust_uabi_uwith_uduplicated_uparam_unames(
    x: i32,
    y: i32,
    __param_2: i32,
    __param_3: i32,
) -> i32 {
    unsafe {
        ::functions_golden::other_fn_param_tests::add_i32_via_rust_abi_with_duplicated_param_names(
            x, y, __param_2, __param_3,
        )
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_add_ui32(x: i32, y: i32) -> i32 {
    unsafe { ::functions_golden::fn_attribute_tests::add_i32(x, y) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_unsafe_uadd(x: i32, y: i32) -> i32 {
    unsafe { ::functions_golden::unsafe_fn_tests::unsafe_add(x, y) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_no_umsg_uadd(x: i32, y: i32) -> i32 {
    unsafe { ::functions_golden::fn_must_use_tests::no_msg_add(x, y) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_msg_uadd(x: i32, y: i32) -> i32 {
    unsafe { ::functions_golden::fn_must_use_tests::msg_add(x, y) }
}
