// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// never_golden
// Features: non_unpin_ctor, std_unique_ptr, std_vector, supported

#![allow(unused_unsafe, deprecated, non_snake_case, unreachable_code)]
#![allow(improper_ctypes_definitions)]
#![deny(warnings)]

#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_never_ureturn() -> ! {
    unsafe { ::never_golden::never_return() }
}
const _: () = assert!(::std::mem::size_of::<::never_golden::NeverStruct>() == 4);
const _: () = assert!(::std::mem::align_of::<::never_golden::NeverStruct>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_default(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value =
            <::never_golden::NeverStruct as ::core::default::Default>::default();
        (__ret_ptr as *mut ::never_golden::NeverStruct).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_associated_ufn_unever_ureturn() -> ! {
    unsafe { ::never_golden::NeverStruct::associated_fn_never_return() }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_method_unever_ureturn(
    __self: &'static ::never_golden::NeverStruct,
) -> ! {
    unsafe { ::never_golden::NeverStruct::method_never_return(__self) }
}
