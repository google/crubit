// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

pub mod test_c_void_ptr {
    use core::ffi::c_void;

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

extern "C" fn i8_func(_: i8) {}
extern "C" fn c_char_func(_: core::ffi::c_char) {}

pub mod return_types {
    use core::ffi;

    pub fn c_void() {}
    pub fn c_void_mut_ptr() -> *mut ffi::c_void {
        core::ptr::null_mut()
    }
    pub fn c_void_const_ptr() -> *const ffi::c_void {
        core::ptr::null()
    }

    pub fn c_char() -> ffi::c_char {
        0
    }
    pub fn c_schar() -> ffi::c_schar {
        0
    }
    pub fn c_uchar() -> ffi::c_uchar {
        0
    }
    pub fn c_short() -> ffi::c_short {
        0
    }
    pub fn c_ushort() -> ffi::c_ushort {
        0
    }
    pub fn c_int() -> ffi::c_int {
        0
    }
    pub fn c_uint() -> ffi::c_uint {
        0
    }
    pub fn c_long() -> ffi::c_long {
        0
    }
    pub fn c_ulong() -> ffi::c_ulong {
        0
    }
    pub fn c_longlong() -> ffi::c_longlong {
        0
    }
    pub fn c_ulonglong() -> ffi::c_ulonglong {
        0
    }
    pub fn c_float() -> ffi::c_float {
        0.0
    }
    pub fn c_double() -> ffi::c_double {
        0.0
    }

    pub fn i8() -> i8 {
        0
    }
    pub fn u8() -> u8 {
        0
    }
    pub fn i16() -> i16 {
        0
    }
    pub fn u16() -> u16 {
        0
    }
    pub fn i32() -> i32 {
        0
    }
    pub fn u32() -> u32 {
        0
    }
    pub fn i64() -> i64 {
        0
    }
    pub fn u64() -> u64 {
        0
    }
    pub fn isize() -> isize {
        0
    }
    pub fn usize() -> usize {
        0
    }
    pub fn f32() -> f32 {
        0.0
    }
    pub fn f64() -> f64 {
        0.0
    }

    pub fn i8_func() -> extern "C" fn(i8) {
        crate::i8_func
    }

    pub fn c_char_func() -> extern "C" fn(ffi::c_char) {
        crate::c_char_func
    }
}

pub mod field_types {
    use core::ffi;
    pub struct Types {
        pub c_void_mut_ptr: *mut ffi::c_void,
        pub c_void_const_ptr: *const ffi::c_void,

        pub c_char: ffi::c_char,
        pub c_schar: ffi::c_schar,
        pub c_uchar: ffi::c_uchar,
        pub c_short: ffi::c_short,
        pub c_ushort: ffi::c_ushort,
        pub c_int: ffi::c_int,
        pub c_uint: ffi::c_uint,
        pub c_long: ffi::c_long,
        pub c_ulong: ffi::c_ulong,
        pub c_longlong: ffi::c_longlong,
        pub c_ulonglong: ffi::c_ulonglong,
        pub c_float: ffi::c_float,
        pub c_double: ffi::c_double,

        pub i8: i8,
        pub u8: u8,
        pub i16: i16,
        pub u16: u16,
        pub i32: i32,
        pub u32: u32,
        pub i64: i64,
        pub u64: u64,
        pub isize: isize,
        pub usize: usize,
        pub f32: f32,
        pub f64: f64,

        pub i8_func: extern "C" fn(i8),
        pub c_char_func: extern "C" fn(ffi::c_char),
    }
}
