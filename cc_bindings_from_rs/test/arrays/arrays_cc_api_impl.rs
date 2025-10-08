// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// arrays_golden
// Features: supported

#![allow(unused_unsafe, deprecated, non_snake_case, unreachable_code)]
#![allow(improper_ctypes_definitions)]
#![deny(warnings)]

#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_function_uwith_uconst_uarray_uptr_uid(
    array_ptr: *const [i32; 2],
) -> *const [i32; 2] {
    unsafe { ::arrays_golden::function_with_const_array_ptr_id(array_ptr) }
}
const _: () = assert!(::std::mem::size_of::<::arrays_golden::ArrayStruct>() == 8);
const _: () = assert!(::std::mem::align_of::<::arrays_golden::ArrayStruct>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_default(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value =
            <::arrays_golden::ArrayStruct as ::core::default::Default>::default();
        (__ret_ptr as *mut ::arrays_golden::ArrayStruct).write(__rs_return_value);
    }
}
const _: () = assert!(::core::mem::offset_of!(::arrays_golden::ArrayStruct, array) == 0);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_function_uwith_uarray_ustruct_uid(
    array_struct: &'static mut ::core::mem::MaybeUninit<::arrays_golden::ArrayStruct>,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let array_struct = array_struct.assume_init_read();
        let __rs_return_value = ::arrays_golden::function_with_array_struct_id(array_struct);
        (__ret_ptr as *mut ::arrays_golden::ArrayStruct).write(__rs_return_value);
    }
}
