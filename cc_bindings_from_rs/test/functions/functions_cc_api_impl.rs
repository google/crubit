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
unsafe extern "C" fn __crubit_thunk_char_uto_uascii_ulowercase(
    c: *mut char,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let c = c.read();
        let __rs_return_value = ::functions_golden::fn_param_ty_tests::char_to_ascii_lowercase(c);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
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
unsafe extern "C" fn __crubit_thunk_sum_ubytes(bytes: *mut &'static [u8]) -> u32 {
    unsafe {
        let bytes = bytes.read();
        ::functions_golden::fn_param_ty_tests::sum_bytes(bytes)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_prefix_usums_u_x00000026mut_x00000020_x0000005bi32_x0000005d(
    arg: *mut &'static mut [i32],
) -> () {
    unsafe {
        let arg = arg.read();
        ::functions_golden::generic_fn_tests::as_mut_trait_tests::prefix_sums(arg)
    }
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
unsafe extern "C" fn __crubit_thunk_new(x: i32, __ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value =
            ::functions_golden::generic_fn_tests::as_ref_trait_tests::MyStruct::new(x);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_diverse_ulifetimes_u_x00000026_x0000005bi32_x0000005d(
    arg1: *mut &'static [i32],
    arg2: *mut &'static [i32],
    arg3: *mut &'static [i32],
    result: *mut &'static mut [i32],
) -> () {
    unsafe {
        let arg1 = arg1.read();
        let arg2 = arg2.read();
        let arg3 = arg3.read();
        let result = result.read();
        ::functions_golden::generic_fn_tests::as_ref_trait_tests::diverse_lifetimes(
            arg1, arg2, arg3, result,
        )
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_slice_uref_usum_u_x00000026_x0000005bi32_x0000005d(
    arg: *mut &'static [i32],
) -> i32 {
    unsafe {
        let arg = arg.read();
        ::functions_golden::generic_fn_tests::as_ref_trait_tests::slice_ref_sum(arg)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_static_ulifetime_urequirement_u_x00000026_x0000005bi32_x0000005d(
    arg: *mut &'static [i32],
) -> i32 {
    unsafe {
        let arg = arg.read();
        ::functions_golden::generic_fn_tests::as_ref_trait_tests::static_lifetime_requirement(arg)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_struct_uref_u_x00000026functions_ugolden_x0000003a_x0000003ageneric_ufn_utests_x0000003a_x0000003aas_uref_utrait_utests_x0000003a_x0000003aMyStruct(
    arg: &'static ::functions_golden::generic_fn_tests::as_ref_trait_tests::MyStruct,
) -> i32 {
    unsafe { ::functions_golden::generic_fn_tests::as_ref_trait_tests::struct_ref(arg) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_two_uargs_u_x00000026_x0000005bi32_x0000005d_u_x00000026_x0000005bi32_x0000005d(
    x: *mut &'static [i32],
    y: *mut &'static [i32],
) -> i32 {
    unsafe {
        let x = x.read();
        let y = y.read();
        ::functions_golden::generic_fn_tests::as_ref_trait_tests::two_args(x, y)
    }
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
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
const _: () = assert!(
    ::core::mem::offset_of!(
        ::functions_golden::generic_fn_tests::ctor_trait_tests::CppMovable,
        value
    ) == 0
);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_accept_uctor_uctor_x0000003a_x0000003aByValue_x0000003c_x00000027_u_x0000002c_x00000020functions_ugolden_x0000003a_x0000003ageneric_ufn_utests_x0000003a_x0000003actor_utrait_utests_x0000003a_x0000003aCppMovable_x0000003e_uctor_x0000003a_x0000003aByValue_x0000003c_x00000027_u_x0000002c_x00000020functions_ugolden_x0000003a_x0000003ageneric_ufn_utests_x0000003a_x0000003actor_utrait_utests_x0000003a_x0000003aCppMovable_x0000003e(
    c1: ::ctor::ByValue<
        'static,
        ::functions_golden::generic_fn_tests::ctor_trait_tests::CppMovable,
    >,
    c2: ::ctor::ByValue<
        'static,
        ::functions_golden::generic_fn_tests::ctor_trait_tests::CppMovable,
    >,
) -> i32 {
    unsafe { ::functions_golden::generic_fn_tests::ctor_trait_tests::accept_ctor(c1, c2) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_accept_uctor_uarray_uctor_x0000003a_x0000003aByValue_x0000003c_x00000027_u_x0000002c_x00000020functions_ugolden_x0000003a_x0000003ageneric_ufn_utests_x0000003a_x0000003actor_utrait_utests_x0000003a_x0000003aCppMovable_x0000003e(
    a: *mut [::ctor::ByValue<
        'static,
        ::functions_golden::generic_fn_tests::ctor_trait_tests::CppMovable,
    >; 3],
) -> i32 {
    unsafe {
        let a = a.read();
        ::functions_golden::generic_fn_tests::ctor_trait_tests::accept_ctor_array(a)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_accept_uctor_utuple_uctor_x0000003a_x0000003aByValue_x0000003c_x00000027_u_x0000002c_x00000020functions_ugolden_x0000003a_x0000003ageneric_ufn_utests_x0000003a_x0000003actor_utrait_utests_x0000003a_x0000003aCppMovable_x0000003e(
    c: *const [*const core::ffi::c_void; 1usize],
) -> i32 {
    unsafe {
        let c = ({
            let c_0: ::ctor::ByValue<
                'static,
                ::functions_golden::generic_fn_tests::ctor_trait_tests::CppMovable,
            > = ((*c)[0usize]
                as *const ::ctor::ByValue<
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
    a: *mut [::ctor::RvalueReference<
        'static,
        ::functions_golden::generic_fn_tests::ctor_trait_tests::CppMovable,
    >; 3],
) -> i32 {
    unsafe {
        let a = a.read();
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
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
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
        ::core::ptr::write(__ret_ptr_0 as *mut _, __rs_return_value_0);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_basic_utest_ui32(arg: i32) -> i32 {
    unsafe { ::functions_golden::generic_fn_tests::into_trait_tests::basic_test(arg) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_generic_uparam_unested_udeeper_uin_uparam_uty_ui32(
    xs: *mut [i32; 3],
) -> i32 {
    unsafe {
        let xs = xs.read();
        ::functions_golden::generic_fn_tests::into_trait_tests::generic_param_nested_deeper_in_param_ty(xs)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_multiple_ugeneric_uparams_ui32_ui32(x: i32, y: i32) -> i32 {
    unsafe { ::functions_golden::generic_fn_tests::into_trait_tests::multiple_generic_params(x, y) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_return_utype_ui32() -> i32 {
    unsafe { ::functions_golden::generic_fn_tests::into_trait_tests::return_type() }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_reused_ugeneric_uparam_ui32(x: i32, y: i32) -> i32 {
    unsafe { ::functions_golden::generic_fn_tests::into_trait_tests::reused_generic_param(x, y) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_where_uclause_ui32(x: i32) -> i32 {
    unsafe { ::functions_golden::generic_fn_tests::into_trait_tests::where_clause(x) }
}
const _: () = assert!(::std::mem::size_of::<::functions_golden::non_null_tests::SomeStruct>() == 4);
const _: () =
    assert!(::std::mem::align_of::<::functions_golden::non_null_tests::SomeStruct>() == 4);
const _: () =
    assert!(::core::mem::offset_of!(::functions_golden::non_null_tests::SomeStruct, value) == 0);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_get_unon_unull_uvalue(ptr: ::core::ptr::NonNull<i32>) -> i32 {
    unsafe { ::functions_golden::non_null_tests::get_non_null_value(ptr) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_return_unon_unull(
    ptr: ::core::ptr::NonNull<i32>,
) -> ::core::ptr::NonNull<i32> {
    unsafe { ::functions_golden::non_null_tests::return_non_null(ptr) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_return_unon_unull_uc_uvoid(
    ptr: ::core::ptr::NonNull<::core::ffi::c_void>,
) -> ::core::ptr::NonNull<::core::ffi::c_void> {
    unsafe { ::functions_golden::non_null_tests::return_non_null_c_void(ptr) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_return_unon_unull_umaybe_uuninit(
    ptr: ::core::ptr::NonNull<std::mem::MaybeUninit<i32>>,
) -> ::core::ptr::NonNull<std::mem::MaybeUninit<i32>> {
    unsafe { ::functions_golden::non_null_tests::return_non_null_maybe_uninit(ptr) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_return_unon_unull_ustruct(
    ptr: ::core::ptr::NonNull<::functions_golden::non_null_tests::SomeStruct>,
) -> ::core::ptr::NonNull<::functions_golden::non_null_tests::SomeStruct> {
    unsafe { ::functions_golden::non_null_tests::return_non_null_struct(ptr) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_set_unon_unull_uvalue(
    ptr: ::core::ptr::NonNull<i32>,
    value: i32,
) -> () {
    unsafe { ::functions_golden::non_null_tests::set_non_null_value(ptr, value) }
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
const _: () = assert!(
    ::std::mem::size_of::<::functions_golden::thread_safety_tests::ThreadSafeStruct>() == 4
);
const _: () = assert!(
    ::std::mem::align_of::<::functions_golden::thread_safety_tests::ThreadSafeStruct>() == 4
);
const _: () = assert!(
    ::core::mem::offset_of!(::functions_golden::thread_safety_tests::ThreadSafeStruct, value) == 0
);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_accepts_uthread_usafe_uref(
    _x: &'static ::functions_golden::thread_safety_tests::ThreadSafeStruct,
) -> () {
    unsafe { ::functions_golden::thread_safety_tests::accepts_thread_safe_ref(_x) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_returns_uthread_usafe_uref(
    x: &'static ::functions_golden::thread_safety_tests::ThreadSafeStruct,
) -> &'static ::functions_golden::thread_safety_tests::ThreadSafeStruct {
    unsafe { ::functions_golden::thread_safety_tests::returns_thread_safe_ref(x) }
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
