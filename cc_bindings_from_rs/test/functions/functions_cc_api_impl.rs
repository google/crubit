// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// functions_golden

#![allow(unused_unsafe, deprecated, non_snake_case, unreachable_code)]
#![allow(improper_ctypes_definitions)]
#![deny(warnings)]

extern crate alloc;
extern crate core;
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
unsafe extern "C" fn __crubit_thunk_add_ui32(x: i32, y: i32) -> i32 {
    unsafe { ::functions_golden::fn_attribute_tests::add_i32(x, y) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_msg_uadd(x: i32, y: i32) -> i32 {
    unsafe { ::functions_golden::fn_must_use_tests::msg_add(x, y) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_no_umsg_uadd(x: i32, y: i32) -> i32 {
    unsafe { ::functions_golden::fn_must_use_tests::no_msg_add(x, y) }
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
unsafe extern "C" fn __crubit_thunk_apply_ubinary_ui32_uop(
    x: i32,
    y: i32,
    f: extern "C" fn(i32, i32) -> i32,
) -> i32 {
    unsafe { ::functions_golden::fn_param_ty_tests::apply_binary_i32_op(x, y, f) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_char_uto_uascii_ulowercase(c: char) -> char {
    unsafe { ::functions_golden::fn_param_ty_tests::char_to_ascii_lowercase(c) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_get_uidentical_uref_uwith_uinferred_ulifetime(
    x: &'static i32,
) -> &'static i32 {
    unsafe { ::functions_golden::fn_param_ty_tests::get_identical_ref_with_inferred_lifetime(x) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_get_uref_uto_usmaller_uint(
    x: &'static i32,
    y: &'static i32,
) -> &'static i32 {
    unsafe { ::functions_golden::fn_param_ty_tests::get_ref_to_smaller_int(x, y) }
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
unsafe extern "C" fn __crubit_thunk_prefix_usums(arg: &'static mut [i32]) -> () {
    unsafe { ::functions_golden::generic_fn_tests::as_mut_trait_tests::prefix_sums(arg) }
}
const _: () = assert!(
    ::std::mem::size_of::<::functions_golden::generic_fn_tests::as_ref_trait_tests::MyStruct>()
        == 4
);
const _: () = assert!(
    ::std::mem::align_of::<::functions_golden::generic_fn_tests::as_ref_trait_tests::MyStruct>()
        == 4
);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_new(
    x: i32,
) -> ::functions_golden::generic_fn_tests::as_ref_trait_tests::MyStruct {
    unsafe { ::functions_golden::generic_fn_tests::as_ref_trait_tests::MyStruct::new(x) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_diverse_ulifetimes(
    arg1: &'static [i32],
    arg2: &'static [i32],
    arg3: &'static [i32],
    result: &'static mut [i32],
) -> () {
    unsafe {
        ::functions_golden::generic_fn_tests::as_ref_trait_tests::diverse_lifetimes(
            arg1, arg2, arg3, result,
        )
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_slice_uref_usum(arg: &'static [i32]) -> i32 {
    unsafe { ::functions_golden::generic_fn_tests::as_ref_trait_tests::slice_ref_sum(arg) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_static_ulifetime_urequirement(arg: &'static [i32]) -> i32 {
    unsafe {
        ::functions_golden::generic_fn_tests::as_ref_trait_tests::static_lifetime_requirement(arg)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_struct_uref(
    arg: &'static ::functions_golden::generic_fn_tests::as_ref_trait_tests::MyStruct,
) -> i32 {
    unsafe { ::functions_golden::generic_fn_tests::as_ref_trait_tests::struct_ref(arg) }
}
const _: () = assert!(
    ::std::mem::size_of::<::functions_golden::generic_fn_tests::ctor_trait_tests::Movable>() == 4
);
const _: () = assert!(
    ::std::mem::align_of::<::functions_golden::generic_fn_tests::ctor_trait_tests::Movable>() == 4
);
const _: () = assert!(
    ::core::mem::offset_of!(::functions_golden::generic_fn_tests::ctor_trait_tests::Movable, value)
        == 0
);
const _: () = assert!(
    ::std::mem::size_of::<::functions_golden::generic_fn_tests::ctor_trait_tests::NonMovable>()
        == 4
);
const _: () = assert!(
    ::std::mem::align_of::<::functions_golden::generic_fn_tests::ctor_trait_tests::NonMovable>()
        == 4
);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_new(value: i32, __ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value =
            ::functions_golden::generic_fn_tests::ctor_trait_tests::NonMovable::new(value);
        (__ret_ptr as *mut ::functions_golden::generic_fn_tests::ctor_trait_tests::NonMovable)
            .write(__rs_return_value);
    }
}
const _: () = assert!(
    ::core::mem::offset_of!(
        ::functions_golden::generic_fn_tests::ctor_trait_tests::NonMovable,
        value
    ) == 0
);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_accept_uctor(
    _c: ::ctor::RvalueReference<
        'static,
        ::functions_golden::generic_fn_tests::ctor_trait_tests::NonMovable,
    >,
) -> i32 {
    unsafe { ::functions_golden::generic_fn_tests::ctor_trait_tests::accept_ctor(_c) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_basic_utest(arg: i32) -> i32 {
    unsafe { ::functions_golden::generic_fn_tests::into_trait_tests::basic_test(arg) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_generic_uparam_unested_udeeper_uin_uparam_uty(
    xs: &'static mut ::core::mem::MaybeUninit<[i32; 3]>,
) -> i32 {
    unsafe {
        let xs = xs.assume_init_read();
        ::functions_golden::generic_fn_tests::into_trait_tests::generic_param_nested_deeper_in_param_ty(xs)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_multiple_ugeneric_uparams(x: i32, y: i32) -> i32 {
    unsafe { ::functions_golden::generic_fn_tests::into_trait_tests::multiple_generic_params(x, y) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_return_utype() -> i32 {
    unsafe { ::functions_golden::generic_fn_tests::into_trait_tests::return_type() }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_reused_ugeneric_uparam(x: i32, y: i32) -> i32 {
    unsafe { ::functions_golden::generic_fn_tests::into_trait_tests::reused_generic_param(x, y) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_where_uclause(x: i32) -> i32 {
    unsafe { ::functions_golden::generic_fn_tests::into_trait_tests::where_clause(x) }
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
unsafe extern "C" fn __crubit_thunk_unsafe_uadd(x: i32, y: i32) -> i32 {
    unsafe { ::functions_golden::unsafe_fn_tests::unsafe_add(x, y) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_CtorNew_uctor_unew(
    args: ::ctor::RvalueReference<
        'static,
        ::functions_golden::generic_fn_tests::ctor_trait_tests::NonMovable,
    >,
) -> ::ctor::RvalueReference<
    'static,
    ::functions_golden::generic_fn_tests::ctor_trait_tests::NonMovable,
> {
    unsafe {
        <::functions_golden::generic_fn_tests::ctor_trait_tests::NonMovable as ::ctor::CtorNew<
            ::ctor::RvalueReference<
                'static,
                ::functions_golden::generic_fn_tests::ctor_trait_tests::NonMovable,
            >,
        >>::ctor_new(args)
    }
}
