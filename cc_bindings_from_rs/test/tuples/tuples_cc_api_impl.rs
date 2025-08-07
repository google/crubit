// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// tuples_golden
// Features: do_not_hardcode_status_bridge, experimental, infer_operator_lifetimes, supported, unsafe_types, wrapper

#![allow(unused_unsafe)]
#![allow(improper_ctypes_definitions)]

#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_return_uunit_uis_unot_utuple() -> () {
    unsafe { ::tuples_golden::return_unit_is_not_tuple() }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_return_uc_uabi_ucompatible_ufive_uin_utuple(
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = ::tuples_golden::return_c_abi_compatible_five_in_tuple();
        let (__rs_return_value_0,) = __rs_return_value;
        let [__ret_ptr_0] = *(__ret_ptr as *mut [*mut core::ffi::c_void; 1usize]);
        (__ret_ptr_0 as *mut i32).write(__rs_return_value_0);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_param_uc_uabi_ucompatible_ufive_uin_utuple(
    five: *const [*const core::ffi::c_void; 1usize],
) -> () {
    unsafe {
        let five = ({
            let five_0: i32 = ((*five)[0usize] as *const i32).read();
            five_0
        },);
        ::tuples_golden::param_c_abi_compatible_five_in_tuple(five)
    }
}
const _: () = assert!(::std::mem::size_of::<::tuples_golden::AdtHoldingFiveAndSix>() == 8);
const _: () = assert!(::std::mem::align_of::<::tuples_golden::AdtHoldingFiveAndSix>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_return_uadt_uin_utuple(
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = ::tuples_golden::return_adt_in_tuple();
        let (__rs_return_value_0,) = __rs_return_value;
        let [__ret_ptr_0] = *(__ret_ptr as *mut [*mut core::ffi::c_void; 1usize]);
        (__ret_ptr_0 as *mut ::tuples_golden::AdtHoldingFiveAndSix).write(__rs_return_value_0);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_param_uadt_uin_utuple(
    adt: *const [*const core::ffi::c_void; 1usize],
) -> () {
    unsafe {
        let adt = ({
            let adt_0: &'static mut ::core::mem::MaybeUninit<
                ::tuples_golden::AdtHoldingFiveAndSix,
            > = ((*adt)[0usize]
                as *const &'static mut ::core::mem::MaybeUninit<
                    ::tuples_golden::AdtHoldingFiveAndSix,
                >)
                .read();
            let adt_0 = adt_0.assume_init_read();
            adt_0
        },);
        ::tuples_golden::param_adt_in_tuple(adt)
    }
}
const _: () = assert!(::std::mem::size_of::<::tuples_golden::NontrivialDrop>() == 1);
const _: () = assert!(::std::mem::align_of::<::tuples_golden::NontrivialDrop>() == 1);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_default(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value =
            <::tuples_golden::NontrivialDrop as ::core::default::Default>::default();
        (__ret_ptr as *mut ::tuples_golden::NontrivialDrop).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_drop(
    __self: &'static mut ::core::mem::MaybeUninit<::tuples_golden::NontrivialDrop>,
) {
    unsafe { __self.assume_init_drop() };
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_return_unew_unontrivial_udrop_uin_utuple(
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = ::tuples_golden::return_new_nontrivial_drop_in_tuple();
        let (__rs_return_value_0,) = __rs_return_value;
        let [__ret_ptr_0] = *(__ret_ptr as *mut [*mut core::ffi::c_void; 1usize]);
        (__ret_ptr_0 as *mut ::tuples_golden::NontrivialDrop).write(__rs_return_value_0);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_param_unontrivial_udrop_uin_utuple(
    nontrivial_drop: *const [*const core::ffi::c_void; 1usize],
) -> () {
    unsafe {
        let nontrivial_drop = ({
            let nontrivial_drop_0: &'static mut ::core::mem::MaybeUninit<
                ::tuples_golden::NontrivialDrop,
            > = ((*nontrivial_drop)[0usize]
                as *const &'static mut ::core::mem::MaybeUninit<::tuples_golden::NontrivialDrop>)
                .read();
            let nontrivial_drop_0 = nontrivial_drop_0.assume_init_read();
            nontrivial_drop_0
        },);
        ::tuples_golden::param_nontrivial_drop_in_tuple(nontrivial_drop)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_assert_unontrivial_udrop_ucount(drop_count: u8) -> () {
    unsafe { ::tuples_golden::assert_nontrivial_drop_count(drop_count) }
}
const _: () = assert!(::std::mem::size_of::<::tuples_golden::NonCppMovable>() == 1);
const _: () = assert!(::std::mem::align_of::<::tuples_golden::NonCppMovable>() == 1);
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_drop(
    __self: &'static mut ::core::mem::MaybeUninit<::tuples_golden::NonCppMovable>,
) {
    unsafe { __self.assume_init_drop() };
}
const _: () = assert!(::core::mem::offset_of!(::tuples_golden::NonCppMovable, value) == 0);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_param_unested_utuples(
    v: *const [*const core::ffi::c_void; 2usize],
) -> () {
    unsafe {
        let v = (
            {
                let v_0: *const [*const core::ffi::c_void; 2usize] =
                    ((*v)[0usize] as *const *const [*const core::ffi::c_void; 2usize]).read();
                let v_0 = (
                    {
                        let v_0_0: i32 = ((*v_0)[0usize] as *const i32).read();
                        v_0_0
                    },
                    {
                        let v_0_1: i32 = ((*v_0)[1usize] as *const i32).read();
                        v_0_1
                    },
                );
                v_0
            },
            {
                let v_1: i32 = ((*v)[1usize] as *const i32).read();
                v_1
            },
        );
        ::tuples_golden::param_nested_tuples(v)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_return_unested_utuples(
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = ::tuples_golden::return_nested_tuples();
        let (__rs_return_value_0, __rs_return_value_1) = __rs_return_value;
        let [__ret_ptr_0, __ret_ptr_1] = *(__ret_ptr as *mut [*mut core::ffi::c_void; 2usize]);
        let (__rs_return_value_0_0, __rs_return_value_0_1) = __rs_return_value_0;
        let [__ret_ptr_0_0, __ret_ptr_0_1] =
            *(__ret_ptr_0 as *mut [*mut core::ffi::c_void; 2usize]);
        (__ret_ptr_0_0 as *mut i32).write(__rs_return_value_0_0);
        (__ret_ptr_0_1 as *mut i32).write(__rs_return_value_0_1);
        (__ret_ptr_1 as *mut i32).write(__rs_return_value_1);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_param_utriply_unested_utuple(
    v: *const [*const core::ffi::c_void; 1usize],
) -> () {
    unsafe {
        let v = ({
            let v_0: *const [*const core::ffi::c_void; 1usize] =
                ((*v)[0usize] as *const *const [*const core::ffi::c_void; 1usize]).read();
            let v_0 = ({
                let v_0_0: *const [*const core::ffi::c_void; 1usize] =
                    ((*v_0)[0usize] as *const *const [*const core::ffi::c_void; 1usize]).read();
                let v_0_0 = ({
                    let v_0_0_0: i32 = ((*v_0_0)[0usize] as *const i32).read();
                    v_0_0_0
                },);
                v_0_0
            },);
            v_0
        },);
        ::tuples_golden::param_triply_nested_tuple(v)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_return_utriply_unested_utuple(
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = ::tuples_golden::return_triply_nested_tuple();
        let (__rs_return_value_0,) = __rs_return_value;
        let [__ret_ptr_0] = *(__ret_ptr as *mut [*mut core::ffi::c_void; 1usize]);
        let (__rs_return_value_0_0,) = __rs_return_value_0;
        let [__ret_ptr_0_0] = *(__ret_ptr_0 as *mut [*mut core::ffi::c_void; 1usize]);
        let (__rs_return_value_0_0_0,) = __rs_return_value_0_0;
        let [__ret_ptr_0_0_0] = *(__ret_ptr_0_0 as *mut [*mut core::ffi::c_void; 1usize]);
        (__ret_ptr_0_0_0 as *mut i32).write(__rs_return_value_0_0_0);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_param_uffi_ualias_uin_utuple(
    five: *const [*const core::ffi::c_void; 1usize],
) -> () {
    unsafe {
        let five = ({
            let five_0: i8 = ((*five)[0usize] as *const i8).read();
            five_0
        },);
        ::tuples_golden::param_ffi_alias_in_tuple(five)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_return_uffi_ualias_uin_utuple(
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = ::tuples_golden::return_ffi_alias_in_tuple();
        let (__rs_return_value_0,) = __rs_return_value;
        let [__ret_ptr_0] = *(__ret_ptr as *mut [*mut core::ffi::c_void; 1usize]);
        (__ret_ptr_0 as *mut i8).write(__rs_return_value_0);
    }
}
const _: () = assert!(::std::mem::size_of::<::tuples_golden::TupleStruct>() == 4);
const _: () = assert!(::std::mem::align_of::<::tuples_golden::TupleStruct>() == 4);
const _: () = assert!(::core::mem::offset_of!(::tuples_golden::TupleStruct, tuple_field) == 0);
const _: () =
    assert!(::core::mem::offset_of!(::tuples_golden::TupleStruct, empty_tuple_field) == 4);
