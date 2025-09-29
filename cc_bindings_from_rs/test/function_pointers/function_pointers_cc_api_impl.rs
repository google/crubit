// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// function_pointers_golden
// Features: do_not_hardcode_status_bridge, experimental, infer_operator_lifetimes, non_unpin_ctor, std_unique_ptr, std_vector, supported, unsafe_types, wrapper

#![allow(unused_unsafe)]
#![allow(improper_ctypes_definitions)]

const _: () = assert!(::std::mem::size_of::<::function_pointers_golden::HasFnPtrField>() == 8);
const _: () = assert!(::std::mem::align_of::<::function_pointers_golden::HasFnPtrField>() == 8);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_with_uadd_uten(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::function_pointers_golden::HasFnPtrField::with_add_ten();
        (__ret_ptr as *mut ::function_pointers_golden::HasFnPtrField).write(__rs_return_value);
    }
}
const _: () = assert!(::core::mem::offset_of!(::function_pointers_golden::HasFnPtrField, ptr) == 0);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_call_ufn_uptr_uno_uargs_uor_ureturn(
    fn_ptr: extern "C" fn(),
) -> () {
    unsafe { ::function_pointers_golden::call_fn_ptr_no_args_or_return(fn_ptr) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_call_ufn_uptr_uwith_ufive(
    fn_ptr: extern "C" fn(i32) -> i32,
) -> i32 {
    unsafe { ::function_pointers_golden::call_fn_ptr_with_five(fn_ptr) }
}
const _: () = assert!(::std::mem::size_of::<::function_pointers_golden::CStruct>() == 4);
const _: () = assert!(::std::mem::align_of::<::function_pointers_golden::CStruct>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_default(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value =
            <::function_pointers_golden::CStruct as ::core::default::Default>::default();
        (__ret_ptr as *mut ::function_pointers_golden::CStruct).write(__rs_return_value);
    }
}
const _: () = assert!(::core::mem::offset_of!(::function_pointers_golden::CStruct, field) == 0);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_call_ufn_uptr_uwith_urepr_uc_ustruct_uptr_ucontaining_useven(
    fn_ptr: unsafe extern "C" fn(*const ::function_pointers_golden::CStruct) -> i32,
) -> i32 {
    unsafe {
        ::function_pointers_golden::call_fn_ptr_with_repr_c_struct_ptr_containing_seven(fn_ptr)
    }
}
