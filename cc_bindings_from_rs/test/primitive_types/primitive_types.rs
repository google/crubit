// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

pub mod test_c_void_ptr {
    use libc::c_void;

    // As struct member.
    pub struct StructWithCVoidPointerMember {
        pub ptr_const: *const c_void,
        pub ptr_mut: *mut c_void,
    }

    // As function parameter.
    pub fn new_struct_with_c_void_pointer_member(
        ptr_const: *const c_void,
        ptr_mut: *mut c_void,
    ) -> StructWithCVoidPointerMember {
        StructWithCVoidPointerMember { ptr_const, ptr_mut }
    }

    // As function parameter and return type.
    pub fn identity_const_c_void_ptr(ptr: *const c_void) -> *const c_void {
        ptr
    }
    pub fn identity_mut_c_void_ptr(ptr: *mut c_void) -> *mut c_void {
        ptr
    }
}
