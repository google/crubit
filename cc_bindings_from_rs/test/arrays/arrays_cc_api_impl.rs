// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// arrays_golden
// Features: non_unpin_ctor, std_unique_ptr, std_vector, supported

#![allow(unused_unsafe, deprecated, non_snake_case, unreachable_code)]
#![allow(improper_ctypes_definitions)]
#![deny(warnings)]

#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_function_uwith_unested_uarrays(
    array: &'static mut ::core::mem::MaybeUninit<[[i32; 2]; 2]>,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let array = array.assume_init_read();
        let __rs_return_value = ::arrays_golden::function_with_nested_arrays(array);
        (__ret_ptr as *mut [[i32; 2]; 2]).write(__rs_return_value);
    }
}
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
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_function_uwith_uarray_uid(
    array: &'static mut ::core::mem::MaybeUninit<[i32; 2]>,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let array = array.assume_init_read();
        let __rs_return_value = ::arrays_golden::function_with_array_id(array);
        (__ret_ptr as *mut [i32; 2]).write(__rs_return_value);
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
                let array_tup_0: &'static mut ::core::mem::MaybeUninit<[i32; 2]> = ((*array_tup)
                    [0usize]
                    as *const &'static mut ::core::mem::MaybeUninit<[i32; 2]>)
                    .read();
                let array_tup_0 = array_tup_0.assume_init_read();
                array_tup_0
            },
            {
                let array_tup_1: &'static mut ::core::mem::MaybeUninit<[i32; 2]> = ((*array_tup)
                    [1usize]
                    as *const &'static mut ::core::mem::MaybeUninit<[i32; 2]>)
                    .read();
                let array_tup_1 = array_tup_1.assume_init_read();
                array_tup_1
            },
        );
        let __rs_return_value = ::arrays_golden::function_with_array_tuple_id(array_tup);
        let (__rs_return_value_0, __rs_return_value_1) = __rs_return_value;
        let [__ret_ptr_0, __ret_ptr_1] = *(__ret_ptr as *mut [*mut core::ffi::c_void; 2usize]);
        (__ret_ptr_0 as *mut [i32; 2]).write(__rs_return_value_0);
        (__ret_ptr_1 as *mut [i32; 2]).write(__rs_return_value_1);
    }
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
unsafe extern "C" fn __crubit_thunk_function_uwith_uhas_udrop_uret_uonly(
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = ::arrays_golden::function_with_has_drop_ret_only();
        (__ret_ptr as *mut [::arrays_golden::HasDrop; 2]).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_function_uwith_uhas_udrop_uand_udefault_uarray_uid(
    array: &'static mut ::core::mem::MaybeUninit<[::arrays_golden::HasDropAndDefault; 2]>,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let array = array.assume_init_read();
        let __rs_return_value = ::arrays_golden::function_with_has_drop_and_default_array_id(array);
        (__ret_ptr as *mut [::arrays_golden::HasDropAndDefault; 2]).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_function_uwith_umut_uarray_unamed_usize_uptr_uid(
    array_ptr: *const [i32; 3],
) -> *const [i32; 3] {
    unsafe { ::arrays_golden::function_with_mut_array_named_size_ptr_id(array_ptr) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_function_uwith_uconst_uarray_uptr_uid(
    array_ptr: *const [i32; 2],
) -> *const [i32; 2] {
    unsafe { ::arrays_golden::function_with_const_array_ptr_id(array_ptr) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_function_uwith_uhas_udrop_uarray_uid(
    array: &'static mut ::core::mem::MaybeUninit<[::arrays_golden::HasDrop; 2]>,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let array = array.assume_init_read();
        let __rs_return_value = ::arrays_golden::function_with_has_drop_array_id(array);
        (__ret_ptr as *mut [::arrays_golden::HasDrop; 2]).write(__rs_return_value);
    }
}
const _: () = assert!(::std::mem::size_of::<::arrays_golden::HasDropAndDefault>() == 4);
const _: () = assert!(::std::mem::align_of::<::arrays_golden::HasDropAndDefault>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_default(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value =
            <::arrays_golden::HasDropAndDefault as ::core::default::Default>::default();
        (__ret_ptr as *mut ::arrays_golden::HasDropAndDefault).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_drop(
    __self: &'static mut ::core::mem::MaybeUninit<::arrays_golden::HasDropAndDefault>,
) {
    unsafe { __self.assume_init_drop() };
}
const _: () = assert!(::core::mem::offset_of!(::arrays_golden::HasDropAndDefault, x) == 0);
const _: () = assert!(::std::mem::size_of::<::arrays_golden::HasDrop>() == 4);
const _: () = assert!(::std::mem::align_of::<::arrays_golden::HasDrop>() == 4);
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_drop(
    __self: &'static mut ::core::mem::MaybeUninit<::arrays_golden::HasDrop>,
) {
    unsafe { __self.assume_init_drop() };
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_new(x: i32, __ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::arrays_golden::HasDrop::new(x);
        (__ret_ptr as *mut ::arrays_golden::HasDrop).write(__rs_return_value);
    }
}
const _: () = assert!(::core::mem::offset_of!(::arrays_golden::HasDrop, x) == 0);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_function_uwith_unested_udrop_udefault_uarrays(
    array: &'static mut ::core::mem::MaybeUninit<[[::arrays_golden::HasDropAndDefault; 2]; 2]>,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let array = array.assume_init_read();
        let __rs_return_value = ::arrays_golden::function_with_nested_drop_default_arrays(array);
        (__ret_ptr as *mut [[::arrays_golden::HasDropAndDefault; 2]; 2]).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_function_uwith_uempty_uarray(
    array: &'static mut ::core::mem::MaybeUninit<[i32; 0]>,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let array = array.assume_init_read();
        let __rs_return_value = ::arrays_golden::function_with_empty_array(array);
        (__ret_ptr as *mut [i32; 0]).write(__rs_return_value);
    }
}
