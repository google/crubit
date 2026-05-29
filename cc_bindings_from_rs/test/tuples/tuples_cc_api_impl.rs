// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// tuples_golden

#![allow(unused_unsafe, deprecated, non_snake_case, unreachable_code)]
#![allow(improper_ctypes_definitions)]
#![deny(warnings)]

extern crate alloc;
extern crate core;
const _: () = assert!(::std::mem::size_of::<::tuples_golden::AdtHoldingFiveAndSix>() == 8);
const _: () = assert!(::std::mem::align_of::<::tuples_golden::AdtHoldingFiveAndSix>() == 4);
const _: () = assert!(::std::mem::size_of::<::tuples_golden::CloneNoDefault>() == 1);
const _: () = assert!(::std::mem::align_of::<::tuples_golden::CloneNoDefault>() == 1);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_clone(
    __self: &'static ::tuples_golden::CloneNoDefault,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value =
            <::tuples_golden::CloneNoDefault as ::core::clone::Clone>::clone(__self);
        (__ret_ptr as *mut ::tuples_golden::CloneNoDefault).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_clone_ufrom(
    __self: &'static mut ::tuples_golden::CloneNoDefault,
    source: &'static ::tuples_golden::CloneNoDefault,
) -> () {
    unsafe { <::tuples_golden::CloneNoDefault as ::core::clone::Clone>::clone_from(__self, source) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_new(val: u8, __ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::tuples_golden::CloneNoDefault::new(val);
        (__ret_ptr as *mut ::tuples_golden::CloneNoDefault).write(__rs_return_value);
    }
}
const _: () = assert!(::core::mem::offset_of!(::tuples_golden::CloneNoDefault, val) == 0);
const _: () = assert!(::std::mem::size_of::<::tuples_golden::CloneNoDefaultTuple>() == 4);
const _: () = assert!(::std::mem::align_of::<::tuples_golden::CloneNoDefaultTuple>() == 1);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_new(val: u8, __ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::tuples_golden::CloneNoDefaultTuple::new(val);
        (__ret_ptr as *mut ::tuples_golden::CloneNoDefaultTuple).write(__rs_return_value);
    }
}
const _: () =
    assert!(::core::mem::offset_of!(::tuples_golden::CloneNoDefaultTuple, in_tuple1) == 0);
const _: () =
    assert!(::core::mem::offset_of!(::tuples_golden::CloneNoDefaultTuple, in_tuple2) == 2);
const _: () = assert!(::std::mem::size_of::<::tuples_golden::CopyNoDefault>() == 1);
const _: () = assert!(::std::mem::align_of::<::tuples_golden::CopyNoDefault>() == 1);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_new(val: u8, __ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::tuples_golden::CopyNoDefault::new(val);
        (__ret_ptr as *mut ::tuples_golden::CopyNoDefault).write(__rs_return_value);
    }
}
const _: () = assert!(::core::mem::offset_of!(::tuples_golden::CopyNoDefault, val) == 0);
const _: () = assert!(::std::mem::size_of::<::tuples_golden::CopyNoDefaultTuple>() == 4);
const _: () = assert!(::std::mem::align_of::<::tuples_golden::CopyNoDefaultTuple>() == 1);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_new(val: u8, __ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::tuples_golden::CopyNoDefaultTuple::new(val);
        (__ret_ptr as *mut ::tuples_golden::CopyNoDefaultTuple).write(__rs_return_value);
    }
}
const _: () = assert!(::core::mem::offset_of!(::tuples_golden::CopyNoDefaultTuple, in_tuple1) == 0);
const _: () = assert!(::core::mem::offset_of!(::tuples_golden::CopyNoDefaultTuple, in_tuple2) == 2);
const _: () = assert!(::std::mem::size_of::<::tuples_golden::GetsTuple>() == 8);
const _: () = assert!(::std::mem::align_of::<::tuples_golden::GetsTuple>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_new(val: u32, __ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::tuples_golden::GetsTuple::new(val);
        (__ret_ptr as *mut ::tuples_golden::GetsTuple).write(__rs_return_value);
    }
}
const _: () = assert!(::core::mem::offset_of!(::tuples_golden::GetsTuple, value) == 0);
const _: () = assert!(::std::mem::size_of::<::tuples_golden::HasDefault>() == 24);
const _: () = assert!(::std::mem::align_of::<::tuples_golden::HasDefault>() == 8);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_default(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value =
            <::tuples_golden::HasDefault as ::core::default::Default>::default();
        (__ret_ptr as *mut ::tuples_golden::HasDefault).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_drop(
    __self: &'static mut ::core::mem::MaybeUninit<::tuples_golden::HasDefault>,
) {
    unsafe { __self.assume_init_drop() };
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_new(
    val: &'static str,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = ::tuples_golden::HasDefault::new(val);
        (__ret_ptr as *mut ::tuples_golden::HasDefault).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_val(
    __self: &'static ::tuples_golden::HasDefault,
) -> &'static str {
    unsafe { ::tuples_golden::HasDefault::val(__self) }
}
const _: () = assert!(::core::mem::offset_of!(::tuples_golden::HasDefault, val) == 0);
const _: () = assert!(::std::mem::size_of::<::tuples_golden::HasDefaultTuple>() == 64);
const _: () = assert!(::std::mem::align_of::<::tuples_golden::HasDefaultTuple>() == 8);
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_drop(
    __self: &'static mut ::core::mem::MaybeUninit<::tuples_golden::HasDefaultTuple>,
) {
    unsafe { __self.assume_init_drop() };
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_new(
    val: &'static str,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = ::tuples_golden::HasDefaultTuple::new(val);
        (__ret_ptr as *mut ::tuples_golden::HasDefaultTuple).write(__rs_return_value);
    }
}
const _: () = assert!(::core::mem::offset_of!(::tuples_golden::HasDefaultTuple, in_tuple1) == 0);
const _: () = assert!(::core::mem::offset_of!(::tuples_golden::HasDefaultTuple, in_tuple2) == 32);
const _: () = assert!(::std::mem::size_of::<::tuples_golden::HasNoDefault>() == 24);
const _: () = assert!(::std::mem::align_of::<::tuples_golden::HasNoDefault>() == 8);
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_drop(
    __self: &'static mut ::core::mem::MaybeUninit<::tuples_golden::HasNoDefault>,
) {
    unsafe { __self.assume_init_drop() };
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_val(
    __self: &'static ::tuples_golden::HasNoDefault,
) -> &'static str {
    unsafe { ::tuples_golden::HasNoDefault::val(__self) }
}
const _: () = assert!(::core::mem::offset_of!(::tuples_golden::HasNoDefault, val) == 0);
const _: () = assert!(::std::mem::size_of::<::tuples_golden::HasNoDefaultTuple>() == 64);
const _: () = assert!(::std::mem::align_of::<::tuples_golden::HasNoDefaultTuple>() == 8);
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_drop(
    __self: &'static mut ::core::mem::MaybeUninit<::tuples_golden::HasNoDefaultTuple>,
) {
    unsafe { __self.assume_init_drop() };
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_new(
    val: &'static str,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = ::tuples_golden::HasNoDefaultTuple::new(val);
        (__ret_ptr as *mut ::tuples_golden::HasNoDefaultTuple).write(__rs_return_value);
    }
}
const _: () = assert!(::core::mem::offset_of!(::tuples_golden::HasNoDefaultTuple, in_tuple1) == 0);
const _: () = assert!(::core::mem::offset_of!(::tuples_golden::HasNoDefaultTuple, in_tuple2) == 32);
const _: () = assert!(::std::mem::size_of::<::tuples_golden::NestedTupleIntermediate1>() == 24);
const _: () = assert!(::std::mem::align_of::<::tuples_golden::NestedTupleIntermediate1>() == 4);
const _: () = assert!(::core::mem::offset_of!(::tuples_golden::NestedTupleIntermediate1, v1) == 0);
const _: () = assert!(::core::mem::offset_of!(::tuples_golden::NestedTupleIntermediate1, v2) == 12);
const _: () = assert!(::std::mem::size_of::<::tuples_golden::NestedTupleIntermediate2>() == 32);
const _: () = assert!(::std::mem::align_of::<::tuples_golden::NestedTupleIntermediate2>() == 4);
const _: () = assert!(::core::mem::offset_of!(::tuples_golden::NestedTupleIntermediate2, v1) == 0);
const _: () = assert!(::core::mem::offset_of!(::tuples_golden::NestedTupleIntermediate2, v2) == 16);
const _: () = assert!(::std::mem::size_of::<::tuples_golden::NestedTupleStruct>() == 32);
const _: () = assert!(::std::mem::align_of::<::tuples_golden::NestedTupleStruct>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_new(val: u32, __ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::tuples_golden::NestedTupleStruct::new(val);
        (__ret_ptr as *mut ::tuples_golden::NestedTupleStruct).write(__rs_return_value);
    }
}
const _: () = assert!(::core::mem::offset_of!(::tuples_golden::NestedTupleStruct, in_tuple1) == 0);
const _: () = assert!(::core::mem::offset_of!(::tuples_golden::NestedTupleStruct, in_tuple2) == 16);
const _: () = assert!(::std::mem::size_of::<::tuples_golden::NonCppMovable>() == 1);
const _: () = assert!(::std::mem::align_of::<::tuples_golden::NonCppMovable>() == 1);
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_drop(
    __self: &'static mut ::core::mem::MaybeUninit<::tuples_golden::NonCppMovable>,
) {
    unsafe { __self.assume_init_drop() };
}
const _: () = assert!(::core::mem::offset_of!(::tuples_golden::NonCppMovable, value) == 0);
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
const _: () = assert!(::std::mem::size_of::<::tuples_golden::StructWithOptionTuple>() == 32);
const _: () = assert!(::std::mem::align_of::<::tuples_golden::StructWithOptionTuple>() == 8);
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_drop(
    __self: &'static mut ::core::mem::MaybeUninit<::tuples_golden::StructWithOptionTuple>,
) {
    unsafe { __self.assume_init_drop() };
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_new(val: i32, __ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::tuples_golden::StructWithOptionTuple::new(val);
        (__ret_ptr as *mut ::tuples_golden::StructWithOptionTuple).write(__rs_return_value);
    }
}
const _: () =
    assert!(::core::mem::offset_of!(::tuples_golden::StructWithOptionTuple, opt_tuple) == 0);
const _: () = assert!(::std::mem::size_of::<::tuples_golden::TupleStruct>() == 4);
const _: () = assert!(::std::mem::align_of::<::tuples_golden::TupleStruct>() == 4);
const _: () = assert!(::core::mem::offset_of!(::tuples_golden::TupleStruct, tuple_field) == 0);
const _: () =
    assert!(::core::mem::offset_of!(::tuples_golden::TupleStruct, empty_tuple_field) == 4);
const _: () = assert!(::std::mem::size_of::<::tuples_golden::TupleWithSizeTypes>() == 64);
const _: () = assert!(::std::mem::align_of::<::tuples_golden::TupleWithSizeTypes>() == 8);
const _: () =
    assert!(::core::mem::offset_of!(::tuples_golden::TupleWithSizeTypes, uval_in_tuple1) == 0);
const _: () =
    assert!(::core::mem::offset_of!(::tuples_golden::TupleWithSizeTypes, uval_in_tuple2) == 16);
const _: () =
    assert!(::core::mem::offset_of!(::tuples_golden::TupleWithSizeTypes, ival_in_tuple1) == 32);
const _: () =
    assert!(::core::mem::offset_of!(::tuples_golden::TupleWithSizeTypes, ival_in_tuple2) == 48);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_assert_unontrivial_udrop_ucount(drop_count: u8) -> () {
    unsafe { ::tuples_golden::assert_nontrivial_drop_count(drop_count) }
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
unsafe extern "C" fn __crubit_thunk_param_uoption_uin_utuple(
    opt: *const [*const core::ffi::c_void; 1usize],
) -> () {
    unsafe {
        let opt = ({
            let opt_0: *const core::ffi::c_uchar =
                ((*opt)[0usize] as *const *const core::ffi::c_uchar).read();
            let opt_0 = unsafe {
                ::bridge_rust::internal::decode(
                    ::bridge_rust::OptionAbi(::bridge_rust::transmute_abi::<i32>()),
                    opt_0,
                )
            };
            opt_0
        },);
        ::tuples_golden::param_option_in_tuple(opt)
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
unsafe extern "C" fn __crubit_thunk_return_uoption_uin_utuple(
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = ::tuples_golden::return_option_in_tuple();
        let (__rs_return_value_0,) = __rs_return_value;
        let [__ret_ptr_0] = *(__ret_ptr as *mut [*mut core::ffi::c_void; 1usize]);
        unsafe {
            ::bridge_rust::internal::encode(
                ::bridge_rust::OptionAbi(::bridge_rust::transmute_abi::<i32>()),
                __ret_ptr_0 as *mut core::ffi::c_uchar,
                __rs_return_value_0,
            );
        }
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_return_uoption_uin_utuple_uref(
    opt: &'static (::core::option::Option<i32>,),
    __ret_ptr: *mut core::ffi::c_uchar,
) -> () {
    unsafe {
        let __rs_return_value = ::tuples_golden::return_option_in_tuple_ref(opt);
        unsafe {
            ::bridge_rust::internal::encode(
                ::bridge_rust::OptionAbi(::bridge_rust::transmute_abi::<i32>()),
                __ret_ptr as *mut core::ffi::c_uchar,
                __rs_return_value,
            );
        }
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
unsafe extern "C" fn __crubit_thunk_return_uunit_uis_unot_utuple() -> () {
    unsafe { ::tuples_golden::return_unit_is_not_tuple() }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_take_utuple_uclone_uno_udefault_u2(
    r: &'static (u8, ::tuples_golden::CloneNoDefault),
) -> u8 {
    unsafe { ::tuples_golden::take_tuple_clone_no_default_2(r) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_take_utuple_ucopy_uno_udefault_u1(
    r: &'static (::tuples_golden::CopyNoDefault, u8),
) -> u8 {
    unsafe { ::tuples_golden::take_tuple_copy_no_default_1(r) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_take_utuple_uhas_udefault(
    r: &'static (::tuples_golden::HasDefault, u8),
) -> &'static str {
    unsafe { ::tuples_golden::take_tuple_has_default(r) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_default(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = <(((u32, u32), u32), u32) as ::core::default::Default>::default();
        let (__rs_return_value_0, __rs_return_value_1) = __rs_return_value;
        let [__ret_ptr_0, __ret_ptr_1] = *(__ret_ptr as *mut [*mut core::ffi::c_void; 2usize]);
        let (__rs_return_value_0_0, __rs_return_value_0_1) = __rs_return_value_0;
        let [__ret_ptr_0_0, __ret_ptr_0_1] =
            *(__ret_ptr_0 as *mut [*mut core::ffi::c_void; 2usize]);
        let (__rs_return_value_0_0_0, __rs_return_value_0_0_1) = __rs_return_value_0_0;
        let [__ret_ptr_0_0_0, __ret_ptr_0_0_1] =
            *(__ret_ptr_0_0 as *mut [*mut core::ffi::c_void; 2usize]);
        (__ret_ptr_0_0_0 as *mut u32).write(__rs_return_value_0_0_0);
        (__ret_ptr_0_0_1 as *mut u32).write(__rs_return_value_0_0_1);
        (__ret_ptr_0_1 as *mut u32).write(__rs_return_value_0_1);
        (__ret_ptr_1 as *mut u32).write(__rs_return_value_1);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_default(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = <((u32, u32), u32) as ::core::default::Default>::default();
        let (__rs_return_value_0, __rs_return_value_1) = __rs_return_value;
        let [__ret_ptr_0, __ret_ptr_1] = *(__ret_ptr as *mut [*mut core::ffi::c_void; 2usize]);
        let (__rs_return_value_0_0, __rs_return_value_0_1) = __rs_return_value_0;
        let [__ret_ptr_0_0, __ret_ptr_0_1] =
            *(__ret_ptr_0 as *mut [*mut core::ffi::c_void; 2usize]);
        (__ret_ptr_0_0 as *mut u32).write(__rs_return_value_0_0);
        (__ret_ptr_0_1 as *mut u32).write(__rs_return_value_0_1);
        (__ret_ptr_1 as *mut u32).write(__rs_return_value_1);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_default(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = <(i32,) as ::core::default::Default>::default();
        let (__rs_return_value_0,) = __rs_return_value;
        let [__ret_ptr_0] = *(__ret_ptr as *mut [*mut core::ffi::c_void; 1usize]);
        (__ret_ptr_0 as *mut i32).write(__rs_return_value_0);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_default(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = <(i8, isize) as ::core::default::Default>::default();
        let (__rs_return_value_0, __rs_return_value_1) = __rs_return_value;
        let [__ret_ptr_0, __ret_ptr_1] = *(__ret_ptr as *mut [*mut core::ffi::c_void; 2usize]);
        (__ret_ptr_0 as *mut i8).write(__rs_return_value_0);
        (__ret_ptr_1 as *mut isize).write(__rs_return_value_1);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_default(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = <(isize, i8) as ::core::default::Default>::default();
        let (__rs_return_value_0, __rs_return_value_1) = __rs_return_value;
        let [__ret_ptr_0, __ret_ptr_1] = *(__ret_ptr as *mut [*mut core::ffi::c_void; 2usize]);
        (__ret_ptr_0 as *mut isize).write(__rs_return_value_0);
        (__ret_ptr_1 as *mut i8).write(__rs_return_value_1);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_clone(
    __self: &'static (
        ::core::option::Option<i32>,
        ::core::result::Result<i32, ::alloc::string::String>,
    ),
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = <(
            ::core::option::Option<i32>,
            ::core::result::Result<i32, ::alloc::string::String>,
        ) as ::core::clone::Clone>::clone(__self);
        let (__rs_return_value_0, __rs_return_value_1) = __rs_return_value;
        let [__ret_ptr_0, __ret_ptr_1] = *(__ret_ptr as *mut [*mut core::ffi::c_void; 2usize]);
        unsafe {
            ::bridge_rust::internal::encode(
                ::bridge_rust::OptionAbi(::bridge_rust::transmute_abi::<i32>()),
                __ret_ptr_0 as *mut core::ffi::c_uchar,
                __rs_return_value_0,
            );
        }
        (__ret_ptr_1 as *mut ::core::result::Result<i32, ::alloc::string::String>)
            .write(__rs_return_value_1);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_clone_ufrom(
    __self: &'static mut (
        ::core::option::Option<i32>,
        ::core::result::Result<i32, ::alloc::string::String>,
    ),
    source: &'static (
        ::core::option::Option<i32>,
        ::core::result::Result<i32, ::alloc::string::String>,
    ),
) -> () {
    unsafe {
        <(::core::option::Option<i32>,::core::result::Result<i32,::alloc::string::String>,)as::core::clone::Clone>::clone_from(__self,source)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_default(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value =
            <(::core::option::Option<i32>,) as ::core::default::Default>::default();
        let (__rs_return_value_0,) = __rs_return_value;
        let [__ret_ptr_0] = *(__ret_ptr as *mut [*mut core::ffi::c_void; 1usize]);
        unsafe {
            ::bridge_rust::internal::encode(
                ::bridge_rust::OptionAbi(::bridge_rust::transmute_abi::<i32>()),
                __ret_ptr_0 as *mut core::ffi::c_uchar,
                __rs_return_value_0,
            );
        }
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_clone(
    __self: &'static (::tuples_golden::CloneNoDefault, u8),
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value =
            <(::tuples_golden::CloneNoDefault, u8) as ::core::clone::Clone>::clone(__self);
        let (__rs_return_value_0, __rs_return_value_1) = __rs_return_value;
        let [__ret_ptr_0, __ret_ptr_1] = *(__ret_ptr as *mut [*mut core::ffi::c_void; 2usize]);
        (__ret_ptr_0 as *mut ::tuples_golden::CloneNoDefault).write(__rs_return_value_0);
        (__ret_ptr_1 as *mut u8).write(__rs_return_value_1);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_clone_ufrom(
    __self: &'static mut (::tuples_golden::CloneNoDefault, u8),
    source: &'static (::tuples_golden::CloneNoDefault, u8),
) -> () {
    unsafe {
        <(::tuples_golden::CloneNoDefault, u8) as ::core::clone::Clone>::clone_from(__self, source)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_default(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value =
            <(::tuples_golden::HasDefault, u8) as ::core::default::Default>::default();
        let (__rs_return_value_0, __rs_return_value_1) = __rs_return_value;
        let [__ret_ptr_0, __ret_ptr_1] = *(__ret_ptr as *mut [*mut core::ffi::c_void; 2usize]);
        (__ret_ptr_0 as *mut ::tuples_golden::HasDefault).write(__rs_return_value_0);
        (__ret_ptr_1 as *mut u8).write(__rs_return_value_1);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_default(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = <(u32, (u32, (u32, u32))) as ::core::default::Default>::default();
        let (__rs_return_value_0, __rs_return_value_1) = __rs_return_value;
        let [__ret_ptr_0, __ret_ptr_1] = *(__ret_ptr as *mut [*mut core::ffi::c_void; 2usize]);
        (__ret_ptr_0 as *mut u32).write(__rs_return_value_0);
        let (__rs_return_value_1_0, __rs_return_value_1_1) = __rs_return_value_1;
        let [__ret_ptr_1_0, __ret_ptr_1_1] =
            *(__ret_ptr_1 as *mut [*mut core::ffi::c_void; 2usize]);
        (__ret_ptr_1_0 as *mut u32).write(__rs_return_value_1_0);
        let (__rs_return_value_1_1_0, __rs_return_value_1_1_1) = __rs_return_value_1_1;
        let [__ret_ptr_1_1_0, __ret_ptr_1_1_1] =
            *(__ret_ptr_1_1 as *mut [*mut core::ffi::c_void; 2usize]);
        (__ret_ptr_1_1_0 as *mut u32).write(__rs_return_value_1_1_0);
        (__ret_ptr_1_1_1 as *mut u32).write(__rs_return_value_1_1_1);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_default(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = <(u32, (u32, u32)) as ::core::default::Default>::default();
        let (__rs_return_value_0, __rs_return_value_1) = __rs_return_value;
        let [__ret_ptr_0, __ret_ptr_1] = *(__ret_ptr as *mut [*mut core::ffi::c_void; 2usize]);
        (__ret_ptr_0 as *mut u32).write(__rs_return_value_0);
        let (__rs_return_value_1_0, __rs_return_value_1_1) = __rs_return_value_1;
        let [__ret_ptr_1_0, __ret_ptr_1_1] =
            *(__ret_ptr_1 as *mut [*mut core::ffi::c_void; 2usize]);
        (__ret_ptr_1_0 as *mut u32).write(__rs_return_value_1_0);
        (__ret_ptr_1_1 as *mut u32).write(__rs_return_value_1_1);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_default(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = <(u32, u32) as ::core::default::Default>::default();
        let (__rs_return_value_0, __rs_return_value_1) = __rs_return_value;
        let [__ret_ptr_0, __ret_ptr_1] = *(__ret_ptr as *mut [*mut core::ffi::c_void; 2usize]);
        (__ret_ptr_0 as *mut u32).write(__rs_return_value_0);
        (__ret_ptr_1 as *mut u32).write(__rs_return_value_1);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_clone(
    __self: &'static (u8, ::tuples_golden::CloneNoDefault),
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value =
            <(u8, ::tuples_golden::CloneNoDefault) as ::core::clone::Clone>::clone(__self);
        let (__rs_return_value_0, __rs_return_value_1) = __rs_return_value;
        let [__ret_ptr_0, __ret_ptr_1] = *(__ret_ptr as *mut [*mut core::ffi::c_void; 2usize]);
        (__ret_ptr_0 as *mut u8).write(__rs_return_value_0);
        (__ret_ptr_1 as *mut ::tuples_golden::CloneNoDefault).write(__rs_return_value_1);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_clone_ufrom(
    __self: &'static mut (u8, ::tuples_golden::CloneNoDefault),
    source: &'static (u8, ::tuples_golden::CloneNoDefault),
) -> () {
    unsafe {
        <(u8, ::tuples_golden::CloneNoDefault) as ::core::clone::Clone>::clone_from(__self, source)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_default(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value =
            <(u8, ::tuples_golden::HasDefault) as ::core::default::Default>::default();
        let (__rs_return_value_0, __rs_return_value_1) = __rs_return_value;
        let [__ret_ptr_0, __ret_ptr_1] = *(__ret_ptr as *mut [*mut core::ffi::c_void; 2usize]);
        (__ret_ptr_0 as *mut u8).write(__rs_return_value_0);
        (__ret_ptr_1 as *mut ::tuples_golden::HasDefault).write(__rs_return_value_1);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_default(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = <(u8, usize) as ::core::default::Default>::default();
        let (__rs_return_value_0, __rs_return_value_1) = __rs_return_value;
        let [__ret_ptr_0, __ret_ptr_1] = *(__ret_ptr as *mut [*mut core::ffi::c_void; 2usize]);
        (__ret_ptr_0 as *mut u8).write(__rs_return_value_0);
        (__ret_ptr_1 as *mut usize).write(__rs_return_value_1);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_default(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = <(usize, u8) as ::core::default::Default>::default();
        let (__rs_return_value_0, __rs_return_value_1) = __rs_return_value;
        let [__ret_ptr_0, __ret_ptr_1] = *(__ret_ptr as *mut [*mut core::ffi::c_void; 2usize]);
        (__ret_ptr_0 as *mut usize).write(__rs_return_value_0);
        (__ret_ptr_1 as *mut u8).write(__rs_return_value_1);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_clone(
    __self: &'static ::core::result::Result<i32, ::alloc::string::String>,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value =
            <std::result::Result<i32, ::alloc::string::String> as ::core::clone::Clone>::clone(
                __self,
            );
        (__ret_ptr as *mut ::core::result::Result<i32, ::alloc::string::String>)
            .write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_clone_ufrom(
    __self: &'static mut ::core::result::Result<i32, ::alloc::string::String>,
    source: &'static ::core::result::Result<i32, ::alloc::string::String>,
) -> () {
    unsafe {
        <std::result::Result<i32, ::alloc::string::String> as ::core::clone::Clone>::clone_from(
            __self, source,
        )
    }
}
