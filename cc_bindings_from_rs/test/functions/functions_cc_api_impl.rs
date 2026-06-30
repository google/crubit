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
    ::std::mem::size_of::<::functions_golden::generic_fn_tests::ctor_trait_tests::CppMovable>()
        == 4
);
const _: () = assert!(
    ::std::mem::align_of::<::functions_golden::generic_fn_tests::ctor_trait_tests::CppMovable>()
        == 4
);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_new(value: i32, __ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value =
            ::functions_golden::generic_fn_tests::ctor_trait_tests::CppMovable::new(value);
        (__ret_ptr as *mut ::functions_golden::generic_fn_tests::ctor_trait_tests::CppMovable)
            .write(__rs_return_value);
    }
}
const _: () = assert!(
    ::core::mem::offset_of!(
        ::functions_golden::generic_fn_tests::ctor_trait_tests::CppMovable,
        value
    ) == 0
);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_accept_uctor(
    c1: ::ctor::RvalueReference<
        'static,
        ::functions_golden::generic_fn_tests::ctor_trait_tests::CppMovable,
    >,
    c2: ::ctor::RvalueReference<
        'static,
        ::functions_golden::generic_fn_tests::ctor_trait_tests::CppMovable,
    >,
) -> i32 {
    unsafe { ::functions_golden::generic_fn_tests::ctor_trait_tests::accept_ctor(c1, c2) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_accept_uctor_uarray(
    a: &'static mut ::core::mem::MaybeUninit<
        [::ctor::RvalueReference<
            'static,
            ::functions_golden::generic_fn_tests::ctor_trait_tests::CppMovable,
        >; 3],
    >,
) -> i32 {
    unsafe {
        let a = a.assume_init_read();
        ::functions_golden::generic_fn_tests::ctor_trait_tests::accept_ctor_array(a)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_accept_uctor_utuple(
    c: *const [*const core::ffi::c_void; 1usize],
) -> i32 {
    unsafe {
        let c = ({
            let c_0: ::ctor::RvalueReference<
                'static,
                ::functions_golden::generic_fn_tests::ctor_trait_tests::CppMovable,
            > = ((*c)[0usize]
                as *const ::ctor::RvalueReference<
                    'static,
                    ::functions_golden::generic_fn_tests::ctor_trait_tests::CppMovable,
                >)
                .read();
            c_0
        },);
        ::functions_golden::generic_fn_tests::ctor_trait_tests::accept_ctor_tuple(c)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_accept_urvalue_uref(
    c: ::ctor::RvalueReference<
        'static,
        ::functions_golden::generic_fn_tests::ctor_trait_tests::CppMovable,
    >,
) -> i32 {
    unsafe { ::functions_golden::generic_fn_tests::ctor_trait_tests::accept_rvalue_ref(c) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_accept_urvalue_ureference_uarray(
    a: &'static mut ::core::mem::MaybeUninit<
        [::ctor::RvalueReference<
            'static,
            ::functions_golden::generic_fn_tests::ctor_trait_tests::CppMovable,
        >; 3],
    >,
) -> i32 {
    unsafe {
        let a = a.assume_init_read();
        ::functions_golden::generic_fn_tests::ctor_trait_tests::accept_rvalue_reference_array(a)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_accept_urvalue_ureference_utuple(
    t: *const [*const core::ffi::c_void; 1usize],
) -> i32 {
    unsafe {
        let t = ({
            let t_0: ::ctor::RvalueReference<
                'static,
                ::functions_golden::generic_fn_tests::ctor_trait_tests::CppMovable,
            > = ((*t)[0usize]
                as *const ::ctor::RvalueReference<
                    'static,
                    ::functions_golden::generic_fn_tests::ctor_trait_tests::CppMovable,
                >)
                .read();
            t_0
        },);
        ::functions_golden::generic_fn_tests::ctor_trait_tests::accept_rvalue_reference_tuple(t)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_return_urvalue_ureference(
    c: ::ctor::RvalueReference<
        'static,
        ::functions_golden::generic_fn_tests::ctor_trait_tests::CppMovable,
    >,
) -> ::ctor::RvalueReference<
    'static,
    ::functions_golden::generic_fn_tests::ctor_trait_tests::CppMovable,
> {
    unsafe { ::functions_golden::generic_fn_tests::ctor_trait_tests::return_rvalue_reference(c) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_return_urvalue_ureference_uarray(
    c: ::ctor::RvalueReference<
        'static,
        ::functions_golden::generic_fn_tests::ctor_trait_tests::CppMovable,
    >,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value =
            ::functions_golden::generic_fn_tests::ctor_trait_tests::return_rvalue_reference_array(
                c,
            );
        (__ret_ptr
            as *mut [::ctor::RvalueReference<
                'static,
                ::functions_golden::generic_fn_tests::ctor_trait_tests::CppMovable,
            >; 1])
            .write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_return_urvalue_ureference_utuple(
    c: ::ctor::RvalueReference<
        'static,
        ::functions_golden::generic_fn_tests::ctor_trait_tests::CppMovable,
    >,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value =
            ::functions_golden::generic_fn_tests::ctor_trait_tests::return_rvalue_reference_tuple(
                c,
            );
        let (__rs_return_value_0,) = __rs_return_value;
        let [__ret_ptr_0] = *(__ret_ptr as *mut [*mut core::ffi::c_void; 1usize]);
        (__ret_ptr_0
            as *mut ::ctor::RvalueReference<
                'static,
                ::functions_golden::generic_fn_tests::ctor_trait_tests::CppMovable,
            >)
            .write(__rs_return_value_0);
    }
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
unsafe extern "C" fn __crubit_thunk_AsRef_uas_uref_ufunctions_ugolden_x0000003a_x0000003ageneric_ufn_utests_x0000003a_x0000003aas_uref_utrait_utests_x0000003a_x0000003aMyStruct_ufunctions_ugolden_x0000003a_x0000003ageneric_ufn_utests_x0000003a_x0000003aas_uref_utrait_utests_x0000003a_x0000003aMyStruct(
    __self: &'static ::functions_golden::generic_fn_tests::as_ref_trait_tests::MyStruct,
) -> &'static ::functions_golden::generic_fn_tests::as_ref_trait_tests::MyStruct {
    unsafe {
        <::functions_golden::generic_fn_tests::as_ref_trait_tests::MyStruct as::core::convert::AsRef<::functions_golden::generic_fn_tests::as_ref_trait_tests::MyStruct>>::as_ref(__self)
    }
}
