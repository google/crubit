// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// tuple_structs_golden
// Features: infer_operator_lifetimes, non_unpin_ctor, std_unique_ptr, std_vector, supported

#![allow(unused_unsafe, deprecated, non_snake_case, unreachable_code)]
#![allow(improper_ctypes_definitions)]
#![deny(warnings)]

const _: () =
    assert!(::std::mem::size_of::<::tuple_structs_golden::TupleStructOnePublicArg>() == 4);
const _: () =
    assert!(::std::mem::align_of::<::tuple_structs_golden::TupleStructOnePublicArg>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_create(arg: i32, __ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::tuple_structs_golden::TupleStructOnePublicArg::create(arg);
        (__ret_ptr as *mut ::tuple_structs_golden::TupleStructOnePublicArg)
            .write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_get_uarg(
    __self: &'static mut ::core::mem::MaybeUninit<::tuple_structs_golden::TupleStructOnePublicArg>,
) -> i32 {
    unsafe {
        let __self = __self.assume_init_read();
        ::tuple_structs_golden::TupleStructOnePublicArg::get_arg(__self)
    }
}
const _: () =
    assert!(::core::mem::offset_of!(::tuple_structs_golden::TupleStructOnePublicArg, 0) == 0);
const _: () =
    assert!(::std::mem::size_of::<::tuple_structs_golden::TupleStructOnePrivateArg>() == 4);
const _: () =
    assert!(::std::mem::align_of::<::tuple_structs_golden::TupleStructOnePrivateArg>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_create(arg: i32, __ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::tuple_structs_golden::TupleStructOnePrivateArg::create(arg);
        (__ret_ptr as *mut ::tuple_structs_golden::TupleStructOnePrivateArg)
            .write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_get_uarg(
    __self: &'static mut ::core::mem::MaybeUninit<::tuple_structs_golden::TupleStructOnePrivateArg>,
) -> i32 {
    unsafe {
        let __self = __self.assume_init_read();
        ::tuple_structs_golden::TupleStructOnePrivateArg::get_arg(__self)
    }
}
const _: () =
    assert!(::std::mem::size_of::<::tuple_structs_golden::TupleStructTwoPublicArgs>() == 8);
const _: () =
    assert!(::std::mem::align_of::<::tuple_structs_golden::TupleStructTwoPublicArgs>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_create(
    first_arg: i32,
    second_arg: i32,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value =
            ::tuple_structs_golden::TupleStructTwoPublicArgs::create(first_arg, second_arg);
        (__ret_ptr as *mut ::tuple_structs_golden::TupleStructTwoPublicArgs)
            .write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_get_ufirst_uarg(
    __self: &'static mut ::core::mem::MaybeUninit<::tuple_structs_golden::TupleStructTwoPublicArgs>,
) -> i32 {
    unsafe {
        let __self = __self.assume_init_read();
        ::tuple_structs_golden::TupleStructTwoPublicArgs::get_first_arg(__self)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_get_usecond_uarg(
    __self: &'static mut ::core::mem::MaybeUninit<::tuple_structs_golden::TupleStructTwoPublicArgs>,
) -> i32 {
    unsafe {
        let __self = __self.assume_init_read();
        ::tuple_structs_golden::TupleStructTwoPublicArgs::get_second_arg(__self)
    }
}
const _: () =
    assert!(::core::mem::offset_of!(::tuple_structs_golden::TupleStructTwoPublicArgs, 0) == 0);
const _: () =
    assert!(::core::mem::offset_of!(::tuple_structs_golden::TupleStructTwoPublicArgs, 1) == 4);
const _: () =
    assert!(::std::mem::size_of::<::tuple_structs_golden::TupleStructTwoPrivateArgs>() == 8);
const _: () =
    assert!(::std::mem::align_of::<::tuple_structs_golden::TupleStructTwoPrivateArgs>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_create(
    first_arg: i32,
    second_arg: i32,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value =
            ::tuple_structs_golden::TupleStructTwoPrivateArgs::create(first_arg, second_arg);
        (__ret_ptr as *mut ::tuple_structs_golden::TupleStructTwoPrivateArgs)
            .write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_get_ufirst_uarg(
    __self: &'static mut ::core::mem::MaybeUninit<
        ::tuple_structs_golden::TupleStructTwoPrivateArgs,
    >,
) -> i32 {
    unsafe {
        let __self = __self.assume_init_read();
        ::tuple_structs_golden::TupleStructTwoPrivateArgs::get_first_arg(__self)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_get_usecond_uarg(
    __self: &'static mut ::core::mem::MaybeUninit<
        ::tuple_structs_golden::TupleStructTwoPrivateArgs,
    >,
) -> i32 {
    unsafe {
        let __self = __self.assume_init_read();
        ::tuple_structs_golden::TupleStructTwoPrivateArgs::get_second_arg(__self)
    }
}
const _: () = assert!(
    ::std::mem::size_of::<::tuple_structs_golden::TupleStructOnePublicArgOnePrivateArg>() == 8
);
const _: () = assert!(
    ::std::mem::align_of::<::tuple_structs_golden::TupleStructOnePublicArgOnePrivateArg>() == 4
);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_create(
    first_arg: i32,
    second_arg: i32,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value =
            ::tuple_structs_golden::TupleStructOnePublicArgOnePrivateArg::create(
                first_arg, second_arg,
            );
        (__ret_ptr as *mut ::tuple_structs_golden::TupleStructOnePublicArgOnePrivateArg)
            .write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_get_usecond_uarg(
    __self: &'static mut ::core::mem::MaybeUninit<
        ::tuple_structs_golden::TupleStructOnePublicArgOnePrivateArg,
    >,
) -> i32 {
    unsafe {
        let __self = __self.assume_init_read();
        ::tuple_structs_golden::TupleStructOnePublicArgOnePrivateArg::get_second_arg(__self)
    }
}
const _: () = assert!(
    ::core::mem::offset_of!(::tuple_structs_golden::TupleStructOnePublicArgOnePrivateArg, 0) == 0
);
const _: () = assert!(
    ::std::mem::size_of::<::tuple_structs_golden::TupleStructWithInvalidArgumentType>() == 8
);
const _: () = assert!(
    ::std::mem::align_of::<::tuple_structs_golden::TupleStructWithInvalidArgumentType>() == 4
);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_default(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value=<::tuple_structs_golden::TupleStructWithInvalidArgumentType as::core::default::Default>::default();
        (__ret_ptr as *mut ::tuple_structs_golden::TupleStructWithInvalidArgumentType)
            .write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_create(
    __param_0: *const [*const core::ffi::c_void; 2usize],
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __param_0 = (
            {
                let __param_0_0: i32 = ((*__param_0)[0usize] as *const i32).read();
                __param_0_0
            },
            {
                let __param_0_1: i32 = ((*__param_0)[1usize] as *const i32).read();
                __param_0_1
            },
        );
        let __rs_return_value =
            ::tuple_structs_golden::TupleStructWithInvalidArgumentType::create(__param_0);
        (__ret_ptr as *mut ::tuple_structs_golden::TupleStructWithInvalidArgumentType)
            .write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_get_uarg(
    __self: &'static mut ::core::mem::MaybeUninit<
        ::tuple_structs_golden::TupleStructWithInvalidArgumentType,
    >,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __self = __self.assume_init_read();
        let __rs_return_value =
            ::tuple_structs_golden::TupleStructWithInvalidArgumentType::get_arg(__self);
        let (__rs_return_value_0, __rs_return_value_1) = __rs_return_value;
        let [__ret_ptr_0, __ret_ptr_1] = *(__ret_ptr as *mut [*mut core::ffi::c_void; 2usize]);
        (__ret_ptr_0 as *mut i32).write(__rs_return_value_0);
        (__ret_ptr_1 as *mut i32).write(__rs_return_value_1);
    }
}
const _: () = assert!(
    ::core::mem::offset_of!(::tuple_structs_golden::TupleStructWithInvalidArgumentType, 0) == 0
);
const _: () =
    assert!(::std::mem::size_of::<::tuple_structs_golden::TupleStructWithNonExhaustiveCtor>() == 8);
const _: () = assert!(
    ::std::mem::align_of::<::tuple_structs_golden::TupleStructWithNonExhaustiveCtor>() == 4
);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_default(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value=<::tuple_structs_golden::TupleStructWithNonExhaustiveCtor as::core::default::Default>::default();
        (__ret_ptr as *mut ::tuple_structs_golden::TupleStructWithNonExhaustiveCtor)
            .write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_create(
    first_arg: i32,
    second_arg: i32,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value =
            ::tuple_structs_golden::TupleStructWithNonExhaustiveCtor::create(first_arg, second_arg);
        (__ret_ptr as *mut ::tuple_structs_golden::TupleStructWithNonExhaustiveCtor)
            .write(__rs_return_value);
    }
}
const _: () = assert!(
    ::core::mem::offset_of!(::tuple_structs_golden::TupleStructWithNonExhaustiveCtor, 0) == 0
);
const _: () = assert!(
    ::core::mem::offset_of!(::tuple_structs_golden::TupleStructWithNonExhaustiveCtor, 1) == 4
);
const _: () = assert!(::std::mem::size_of::<::tuple_structs_golden::DontMoveMe>() == 8);
const _: () = assert!(::std::mem::align_of::<::tuple_structs_golden::DontMoveMe>() == 8);
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_drop(
    __self: &'static mut ::core::mem::MaybeUninit<::tuple_structs_golden::DontMoveMe>,
) {
    unsafe { __self.assume_init_drop() };
}
const _: () = assert!(::core::mem::offset_of!(::tuple_structs_golden::DontMoveMe, value) == 0);
const _: () =
    assert!(::std::mem::size_of::<::tuple_structs_golden::TupleStructWithCppImmovableType>() == 16);
const _: () =
    assert!(::std::mem::align_of::<::tuple_structs_golden::TupleStructWithCppImmovableType>() == 8);
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_drop(
    __self: &'static mut ::core::mem::MaybeUninit<
        ::tuple_structs_golden::TupleStructWithCppImmovableType,
    >,
) {
    unsafe { __self.assume_init_drop() };
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_create(
    first_arg: i32,
    second_arg: i32,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value =
            ::tuple_structs_golden::TupleStructWithCppImmovableType::create(first_arg, second_arg);
        (__ret_ptr as *mut ::tuple_structs_golden::TupleStructWithCppImmovableType)
            .write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_get_ufirst_uarg(
    __self: &'static ::tuple_structs_golden::TupleStructWithCppImmovableType,
) -> i32 {
    unsafe { ::tuple_structs_golden::TupleStructWithCppImmovableType::get_first_arg(__self) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_get_usecond_uarg(
    __self: &'static ::tuple_structs_golden::TupleStructWithCppImmovableType,
) -> &'static i32 {
    unsafe { ::tuple_structs_golden::TupleStructWithCppImmovableType::get_second_arg(__self) }
}
const _: () = assert!(
    ::core::mem::offset_of!(::tuple_structs_golden::TupleStructWithCppImmovableType, 1) == 0
);
const _: () = assert!(
    ::core::mem::offset_of!(::tuple_structs_golden::TupleStructWithCppImmovableType, 0) == 8
);
const _: () = assert!(::std::mem::size_of::<::tuple_structs_golden::CopyNoDefault>() == 4);
const _: () = assert!(::std::mem::align_of::<::tuple_structs_golden::CopyNoDefault>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_create(value: i32, __ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::tuple_structs_golden::CopyNoDefault::create(value);
        (__ret_ptr as *mut ::tuple_structs_golden::CopyNoDefault).write(__rs_return_value);
    }
}
const _: () = assert!(::core::mem::offset_of!(::tuple_structs_golden::CopyNoDefault, value) == 0);
const _: () =
    assert!(::std::mem::size_of::<::tuple_structs_golden::TupleStructWithNoDefault>() == 4);
const _: () =
    assert!(::std::mem::align_of::<::tuple_structs_golden::TupleStructWithNoDefault>() == 4);
const _: () =
    assert!(::core::mem::offset_of!(::tuple_structs_golden::TupleStructWithNoDefault, 0) == 0);
const _: () = assert!(::std::mem::size_of::<::tuple_structs_golden::DefaultNoCopyNoClone>() == 4);
const _: () = assert!(::std::mem::align_of::<::tuple_structs_golden::DefaultNoCopyNoClone>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_default(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value =
            <::tuple_structs_golden::DefaultNoCopyNoClone as ::core::default::Default>::default();
        (__ret_ptr as *mut ::tuple_structs_golden::DefaultNoCopyNoClone).write(__rs_return_value);
    }
}
const _: () =
    assert!(::core::mem::offset_of!(::tuple_structs_golden::DefaultNoCopyNoClone, value) == 0);
const _: () = assert!(
    ::std::mem::size_of::<::tuple_structs_golden::TupleStructWithDefaultNoCopyNoClone>() == 4
);
const _: () = assert!(
    ::std::mem::align_of::<::tuple_structs_golden::TupleStructWithDefaultNoCopyNoClone>() == 4
);
const _: () = assert!(
    ::core::mem::offset_of!(::tuple_structs_golden::TupleStructWithDefaultNoCopyNoClone, 0) == 0
);
const _: () = assert!(::std::mem::size_of::<::tuple_structs_golden::CloneNoDefault>() == 8);
const _: () = assert!(::std::mem::align_of::<::tuple_structs_golden::CloneNoDefault>() == 8);
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_drop(
    __self: &'static mut ::core::mem::MaybeUninit<::tuple_structs_golden::CloneNoDefault>,
) {
    unsafe { __self.assume_init_drop() };
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_clone(
    __self: &'static ::tuple_structs_golden::CloneNoDefault,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value =
            <::tuple_structs_golden::CloneNoDefault as ::core::clone::Clone>::clone(__self);
        (__ret_ptr as *mut ::tuple_structs_golden::CloneNoDefault).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_clone_ufrom(
    __self: &'static mut ::tuple_structs_golden::CloneNoDefault,
    source: &'static ::tuple_structs_golden::CloneNoDefault,
) -> () {
    unsafe {
        <::tuple_structs_golden::CloneNoDefault as ::core::clone::Clone>::clone_from(__self, source)
    }
}
const _: () = assert!(::core::mem::offset_of!(::tuple_structs_golden::CloneNoDefault, value) == 0);
const _: () =
    assert!(::std::mem::size_of::<::tuple_structs_golden::TupleStructWithCloneNoDefault>() == 8);
const _: () =
    assert!(::std::mem::align_of::<::tuple_structs_golden::TupleStructWithCloneNoDefault>() == 8);
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_drop(
    __self: &'static mut ::core::mem::MaybeUninit<
        ::tuple_structs_golden::TupleStructWithCloneNoDefault,
    >,
) {
    unsafe { __self.assume_init_drop() };
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_create(value: i32, __ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value =
            ::tuple_structs_golden::TupleStructWithCloneNoDefault::create(value);
        (__ret_ptr as *mut ::tuple_structs_golden::TupleStructWithCloneNoDefault)
            .write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_get_uvalue(
    __self: &'static ::tuple_structs_golden::TupleStructWithCloneNoDefault,
) -> &'static i32 {
    unsafe { ::tuple_structs_golden::TupleStructWithCloneNoDefault::get_value(__self) }
}
const _: () =
    assert!(::core::mem::offset_of!(::tuple_structs_golden::TupleStructWithCloneNoDefault, 0) == 0);
const _: () = assert!(::std::mem::size_of::<::tuple_structs_golden::DefaultAndCloneNoUnpin>() == 4);
const _: () =
    assert!(::std::mem::align_of::<::tuple_structs_golden::DefaultAndCloneNoUnpin>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_default(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value =
            <::tuple_structs_golden::DefaultAndCloneNoUnpin as ::core::default::Default>::default();
        (__ret_ptr as *mut ::tuple_structs_golden::DefaultAndCloneNoUnpin).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_clone(
    __self: &'static ::tuple_structs_golden::DefaultAndCloneNoUnpin,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value =
            <::tuple_structs_golden::DefaultAndCloneNoUnpin as ::core::clone::Clone>::clone(__self);
        (__ret_ptr as *mut ::tuple_structs_golden::DefaultAndCloneNoUnpin).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_clone_ufrom(
    __self: &'static mut ::tuple_structs_golden::DefaultAndCloneNoUnpin,
    source: &'static ::tuple_structs_golden::DefaultAndCloneNoUnpin,
) -> () {
    unsafe {
        <::tuple_structs_golden::DefaultAndCloneNoUnpin as ::core::clone::Clone>::clone_from(
            __self, source,
        )
    }
}
const _: () =
    assert!(::core::mem::offset_of!(::tuple_structs_golden::DefaultAndCloneNoUnpin, value) == 0);
const _: () =
    assert!(::core::mem::offset_of!(::tuple_structs_golden::DefaultAndCloneNoUnpin, _marker) == 4);
const _: () = assert!(
    ::std::mem::size_of::<::tuple_structs_golden::TupleStructWithDefaultAndCloneNoUnpin>() == 4
);
const _: () = assert!(
    ::std::mem::align_of::<::tuple_structs_golden::TupleStructWithDefaultAndCloneNoUnpin>() == 4
);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_create(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value =
            ::tuple_structs_golden::TupleStructWithDefaultAndCloneNoUnpin::create();
        (__ret_ptr as *mut ::tuple_structs_golden::TupleStructWithDefaultAndCloneNoUnpin)
            .write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_get_uarg(
    __self: &'static ::tuple_structs_golden::TupleStructWithDefaultAndCloneNoUnpin,
) -> i32 {
    unsafe { ::tuple_structs_golden::TupleStructWithDefaultAndCloneNoUnpin::get_arg(__self) }
}
const _: () = assert!(
    ::core::mem::offset_of!(::tuple_structs_golden::TupleStructWithDefaultAndCloneNoUnpin, 0) == 0
);
