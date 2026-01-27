// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// struct_with_conflicting_fields_and_member_functions_rust_golden
// Features: assume_lifetimes, custom_ffi_types, experimental, non_unpin_ctor, std_unique_ptr, std_vector, supported, unhardcode_c9_co, wrapper

#![allow(unused_unsafe, deprecated, non_snake_case, unreachable_code)]
#![allow(improper_ctypes_definitions)]
#![deny(warnings)]

const _: () = assert!(
    ::std::mem::size_of::<::struct_with_conflicting_fields_and_member_functions_rust_golden::X>()
        == 12
);
const _: () = assert!(
    ::std::mem::align_of::<::struct_with_conflicting_fields_and_member_functions_rust_golden::X>()
        == 4
);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_a(
    __self: &'static ::struct_with_conflicting_fields_and_member_functions_rust_golden::X,
) -> i32 {
    unsafe { ::struct_with_conflicting_fields_and_member_functions_rust_golden::X::a(__self) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_b(
    __self: &'static ::struct_with_conflicting_fields_and_member_functions_rust_golden::X,
) -> i32 {
    unsafe { ::struct_with_conflicting_fields_and_member_functions_rust_golden::X::b(__self) }
}
const _: () = assert!(
    ::core::mem::offset_of!(
        ::struct_with_conflicting_fields_and_member_functions_rust_golden::X,
        a
    ) == 0
);
