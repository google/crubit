// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// arrays_golden

#![allow(unused_unsafe, deprecated, non_snake_case, unreachable_code)]
#![allow(improper_ctypes_definitions)]
#![deny(warnings)]

extern crate alloc;
extern crate core;
const _: () = assert!(::std::mem::size_of::<::arrays_golden::ArrayStruct>() == 8);
const _: () = assert!(::std::mem::align_of::<::arrays_golden::ArrayStruct>() == 4);
const _: () = assert!(::core::mem::offset_of!(::arrays_golden::ArrayStruct, array) == 0);
const _: () = assert!(::std::mem::size_of::<::arrays_golden::HasDrop>() == 4);
const _: () = assert!(::std::mem::align_of::<::arrays_golden::HasDrop>() == 4);
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_Drop_udrop_uarrays_ugolden_x0000003a_x0000003aHasDrop(
    __self: *mut ::arrays_golden::HasDrop,
) {
    unsafe { ::core::ptr::drop_in_place(__self) };
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_new(x: i32, __ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::arrays_golden::HasDrop::new(x);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
const _: () = assert!(::core::mem::offset_of!(::arrays_golden::HasDrop, x) == 0);
const _: () = assert!(::std::mem::size_of::<::arrays_golden::HasDropAndDefault>() == 4);
const _: () = assert!(::std::mem::align_of::<::arrays_golden::HasDropAndDefault>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Default_udefault_uarrays_ugolden_x0000003a_x0000003aHasDropAndDefault(
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value =
            <::arrays_golden::HasDropAndDefault as ::core::default::Default>::default();
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_Drop_udrop_uarrays_ugolden_x0000003a_x0000003aHasDropAndDefault(
    __self: *mut ::arrays_golden::HasDropAndDefault,
) {
    unsafe { ::core::ptr::drop_in_place(__self) };
}
const _: () = assert!(::core::mem::offset_of!(::arrays_golden::HasDropAndDefault, x) == 0);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_function_uwith_uarray_uid(
    array: *mut [i32; 2],
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let array = array.read();
        let __rs_return_value = ::arrays_golden::function_with_array_id(array);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_function_uwith_uarray_ustruct_uid(
    array_struct: *mut ::arrays_golden::ArrayStruct,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let array_struct = array_struct.read();
        let __rs_return_value = ::arrays_golden::function_with_array_struct_id(array_struct);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_function_uwith_uarray_utuple_uid(
    array_tup: *const [*const core::ffi::c_void; 2usize],
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let array_tup = (
            {
                let array_tup_0: *mut [i32; 2] =
                    ((*array_tup)[0usize] as *const *mut [i32; 2]).read();
                let array_tup_0 = array_tup_0.read();
                array_tup_0
            },
            {
                let array_tup_1: *mut [i32; 2] =
                    ((*array_tup)[1usize] as *const *mut [i32; 2]).read();
                let array_tup_1 = array_tup_1.read();
                array_tup_1
            },
        );
        let __rs_return_value = ::arrays_golden::function_with_array_tuple_id(array_tup);
        let (__rs_return_value_0, __rs_return_value_1) = __rs_return_value;
        let [__ret_ptr_0, __ret_ptr_1] = *(__ret_ptr as *mut [*mut core::ffi::c_void; 2usize]);
        ::core::ptr::write(__ret_ptr_0 as *mut _, __rs_return_value_0);
        ::core::ptr::write(__ret_ptr_1 as *mut _, __rs_return_value_1);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_function_uwith_uconst_uarray_uptr_uid(
    array_ptr: *const [i32; 2],
) -> *const [i32; 2] {
    unsafe { ::arrays_golden::function_with_const_array_ptr_id(array_ptr) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_function_uwith_uempty_uarray(
    array: *mut [i32; 0],
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let array = array.read();
        let __rs_return_value = ::arrays_golden::function_with_empty_array(array);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_function_uwith_uhas_udrop_uand_udefault_uarray_uid(
    array: *mut [::arrays_golden::HasDropAndDefault; 2],
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let array = array.read();
        let __rs_return_value = ::arrays_golden::function_with_has_drop_and_default_array_id(array);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_function_uwith_uhas_udrop_uarray_uid(
    array: *mut [::arrays_golden::HasDrop; 2],
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let array = array.read();
        let __rs_return_value = ::arrays_golden::function_with_has_drop_array_id(array);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_function_uwith_uhas_udrop_uret_uonly(
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = ::arrays_golden::function_with_has_drop_ret_only();
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_function_uwith_umut_uarray_unamed_usize_uptr_uid(
    array_ptr: *const [i32; 3],
) -> *const [i32; 3] {
    unsafe { ::arrays_golden::function_with_mut_array_named_size_ptr_id(array_ptr) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_function_uwith_unested_uarrays(
    array: *mut [[i32; 2]; 2],
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let array = array.read();
        let __rs_return_value = ::arrays_golden::function_with_nested_arrays(array);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_function_uwith_unested_udrop_udefault_uarrays(
    array: *mut [[::arrays_golden::HasDropAndDefault; 2]; 2],
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let array = array.read();
        let __rs_return_value = ::arrays_golden::function_with_nested_drop_default_arrays(array);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_function_uwith_utuple_uarray_uid(
    tup_array: *mut [(i32, i32); 2],
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let tup_array = tup_array.read();
        let __rs_return_value = ::arrays_golden::function_with_tuple_array_id(tup_array);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Default_udefault_u_x00000028i32_x0000002c_x00000020i32_x00000029(
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = <(i32, i32) as ::core::default::Default>::default();
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
const _: () = assert!(::core::mem::offset_of!((i32, i32,), 0) == 0);
const _: () = assert!(::core::mem::offset_of!((i32, i32,), 1) == 4);
