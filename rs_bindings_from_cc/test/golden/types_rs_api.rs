#![rustfmt::skip]
// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#![feature(const_ptr_offset_from, custom_inner_attributes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use memoffset_unstable_const::offset_of;

pub type __builtin_ms_va_list = *mut u8;

pub type __u_char = u8;

pub type __u_short = u16;

pub type __u_int = u32;

pub type __u_long = u64;

pub type __int8_t = i8;

pub type __uint8_t = u8;

pub type __int16_t = i16;

pub type __uint16_t = u16;

pub type __int32_t = i32;

pub type __uint32_t = u32;

pub type __int64_t = i64;

pub type __uint64_t = u64;

pub type __quad_t = i64;

pub type __u_quad_t = u64;

pub type __intmax_t = i64;

pub type __uintmax_t = u64;

pub type __dev_t = u64;

pub type __uid_t = u32;

pub type __gid_t = u32;

pub type __ino_t = u64;

pub type __ino64_t = u64;

pub type __mode_t = u32;

pub type __nlink_t = u64;

pub type __off_t = i64;

pub type __off64_t = i64;

pub type __pid_t = i32;

pub type __clock_t = i64;

pub type __rlim_t = u64;

pub type __rlim64_t = u64;

pub type __id_t = u32;

pub type __time_t = i64;

pub type __useconds_t = u32;

pub type __suseconds_t = i64;

pub type __daddr_t = i32;

pub type __key_t = i32;

pub type __clockid_t = i32;

pub type __timer_t = *mut ();

pub type __blksize_t = i64;

pub type __blkcnt_t = i64;

pub type __blkcnt64_t = i64;

pub type __fsblkcnt_t = u64;

pub type __fsblkcnt64_t = u64;

pub type __fsfilcnt_t = u64;

pub type __fsfilcnt64_t = u64;

pub type __fsword_t = i64;

pub type __ssize_t = i64;

pub type __syscall_slong_t = i64;

pub type __syscall_ulong_t = u64;

pub type __loff_t = __off64_t;

pub type __caddr_t = *mut u8;

pub type __intptr_t = i64;

pub type __socklen_t = u32;

pub type __sig_atomic_t = i32;

pub type int_least8_t = i8;

pub type int_least16_t = i16;

pub type int_least32_t = i32;

pub type int_least64_t = i64;

pub type uint_least8_t = u8;

pub type uint_least16_t = u16;

pub type uint_least32_t = u32;

pub type uint_least64_t = u64;

pub type int_fast8_t = i8;

pub type int_fast16_t = i64;

pub type int_fast32_t = i64;

pub type int_fast64_t = i64;

pub type uint_fast8_t = u8;

pub type uint_fast16_t = u64;

pub type uint_fast32_t = u64;

pub type uint_fast64_t = u64;

pub type intmax_t = __intmax_t;

pub type uintmax_t = __uintmax_t;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct SomeStruct {
    /// Prevent empty C++ struct being zero-size in Rust.
    placeholder: std::mem::MaybeUninit<u8>,
}

impl Default for SomeStruct {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN10SomeStructC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

// rs_bindings_from_cc/test/golden/types.h;l=9
// Error while generating bindings for item 'SomeStruct::SomeStruct':
// Parameter type 'struct SomeStruct &&' is not supported

// rs_bindings_from_cc/test/golden/types.h;l=9
// Error while generating bindings for item 'SomeStruct::operator=':
// Parameter type 'struct SomeStruct &&' is not supported

// rs_bindings_from_cc/test/golden/types.h;l=11
// Error while generating bindings for item 'SomeUnion':
// Unions are not supported yet

#[derive(Clone, Copy)]
#[repr(C)]
pub struct FieldTypeTestStruct {
    pub bool_field: bool,
    pub char_field: u8,
    pub unsigned_char_field: u8,
    pub signed_char_field: i8,
    pub char16_t_field: u16,
    pub char32_t_field: u32,
    pub wchar_t_field: i32,
    pub short_field: i16,
    pub int_field: i32,
    pub long_field: i64,
    pub long_long_field: i64,
    pub unsigned_short_field: u16,
    pub unsigned_int_field: u32,
    pub unsigned_long_field: u64,
    pub unsigned_long_long_field: u64,
    pub signed_short_field: i16,
    pub signed_int_field: i32,
    pub signed_long_field: i64,
    pub signed_long_long_field: i64,
    pub int8_t_field: i8,
    pub int16_t_field: i16,
    pub int32_t_field: i32,
    pub int64_t_field: i64,
    pub std_int8_t_field: i8,
    pub std_int16_t_field: i16,
    pub std_int32_t_field: i32,
    pub std_int64_t_field: i64,
    pub uint8_t_field: u8,
    pub uint16_t_field: u16,
    pub uint32_t_field: u32,
    pub uint64_t_field: u64,
    pub std_uint8_t_field: u8,
    pub std_uint16_t_field: u16,
    pub std_uint32_t_field: u32,
    pub std_uint64_t_field: u64,
    pub ptrdiff_t_field: isize,
    pub size_t_field: usize,
    pub intptr_t_field: isize,
    pub uintptr_t_field: usize,
    pub std_ptrdiff_t_field: isize,
    pub std_size_t_field: usize,
    pub std_intptr_t_field: isize,
    pub std_uintptr_t_field: usize,
    pub float_field: f32,
    pub double_field: f64,
    pub ptr_field: *mut i32,
    pub struct_field: SomeStruct,
    pub struct_ptr_field: *mut SomeStruct,
    pub const_struct_ptr_field: *const SomeStruct,
    pub struct_ref_field: *mut SomeStruct,
    pub const_struct_ref_field: *const SomeStruct,
}

// rs_bindings_from_cc/test/golden/types.h;l=13
// Error while generating bindings for item 'FieldTypeTestStruct::FieldTypeTestStruct':
// Parameter type 'struct FieldTypeTestStruct &&' is not supported

#[inline(always)]
pub fn VoidReturningFunction() {
    unsafe { crate::detail::__rust_thunk___Z21VoidReturningFunctionv() }
}

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_TYPES_H_

mod detail {
    #[allow(unused_imports)]
    use super::*;
    extern "C" {
        pub(crate) fn __rust_thunk___ZN10SomeStructC1Ev<'a>(
            __this: &'a mut std::mem::MaybeUninit<SomeStruct>,
        );
        pub(crate) fn __rust_thunk___Z21VoidReturningFunctionv();
    }
}

const _: () = assert!(std::mem::size_of::<Option<&i32>>() == std::mem::size_of::<&i32>());

const _: () = assert!(std::mem::size_of::<SomeStruct>() == 1usize);
const _: () = assert!(std::mem::align_of::<SomeStruct>() == 1usize);

const _: () = assert!(std::mem::size_of::<FieldTypeTestStruct>() == 280usize);
const _: () = assert!(std::mem::align_of::<FieldTypeTestStruct>() == 8usize);
const _: () = assert!(offset_of!(FieldTypeTestStruct, bool_field) * 8 == 0usize);
const _: () = assert!(offset_of!(FieldTypeTestStruct, char_field) * 8 == 8usize);
const _: () = assert!(offset_of!(FieldTypeTestStruct, unsigned_char_field) * 8 == 16usize);
const _: () = assert!(offset_of!(FieldTypeTestStruct, signed_char_field) * 8 == 24usize);
const _: () = assert!(offset_of!(FieldTypeTestStruct, char16_t_field) * 8 == 32usize);
const _: () = assert!(offset_of!(FieldTypeTestStruct, char32_t_field) * 8 == 64usize);
const _: () = assert!(offset_of!(FieldTypeTestStruct, wchar_t_field) * 8 == 96usize);
const _: () = assert!(offset_of!(FieldTypeTestStruct, short_field) * 8 == 128usize);
const _: () = assert!(offset_of!(FieldTypeTestStruct, int_field) * 8 == 160usize);
const _: () = assert!(offset_of!(FieldTypeTestStruct, long_field) * 8 == 192usize);
const _: () = assert!(offset_of!(FieldTypeTestStruct, long_long_field) * 8 == 256usize);
const _: () = assert!(offset_of!(FieldTypeTestStruct, unsigned_short_field) * 8 == 320usize);
const _: () = assert!(offset_of!(FieldTypeTestStruct, unsigned_int_field) * 8 == 352usize);
const _: () = assert!(offset_of!(FieldTypeTestStruct, unsigned_long_field) * 8 == 384usize);
const _: () = assert!(offset_of!(FieldTypeTestStruct, unsigned_long_long_field) * 8 == 448usize);
const _: () = assert!(offset_of!(FieldTypeTestStruct, signed_short_field) * 8 == 512usize);
const _: () = assert!(offset_of!(FieldTypeTestStruct, signed_int_field) * 8 == 544usize);
const _: () = assert!(offset_of!(FieldTypeTestStruct, signed_long_field) * 8 == 576usize);
const _: () = assert!(offset_of!(FieldTypeTestStruct, signed_long_long_field) * 8 == 640usize);
const _: () = assert!(offset_of!(FieldTypeTestStruct, int8_t_field) * 8 == 704usize);
const _: () = assert!(offset_of!(FieldTypeTestStruct, int16_t_field) * 8 == 720usize);
const _: () = assert!(offset_of!(FieldTypeTestStruct, int32_t_field) * 8 == 736usize);
const _: () = assert!(offset_of!(FieldTypeTestStruct, int64_t_field) * 8 == 768usize);
const _: () = assert!(offset_of!(FieldTypeTestStruct, std_int8_t_field) * 8 == 832usize);
const _: () = assert!(offset_of!(FieldTypeTestStruct, std_int16_t_field) * 8 == 848usize);
const _: () = assert!(offset_of!(FieldTypeTestStruct, std_int32_t_field) * 8 == 864usize);
const _: () = assert!(offset_of!(FieldTypeTestStruct, std_int64_t_field) * 8 == 896usize);
const _: () = assert!(offset_of!(FieldTypeTestStruct, uint8_t_field) * 8 == 960usize);
const _: () = assert!(offset_of!(FieldTypeTestStruct, uint16_t_field) * 8 == 976usize);
const _: () = assert!(offset_of!(FieldTypeTestStruct, uint32_t_field) * 8 == 992usize);
const _: () = assert!(offset_of!(FieldTypeTestStruct, uint64_t_field) * 8 == 1024usize);
const _: () = assert!(offset_of!(FieldTypeTestStruct, std_uint8_t_field) * 8 == 1088usize);
const _: () = assert!(offset_of!(FieldTypeTestStruct, std_uint16_t_field) * 8 == 1104usize);
const _: () = assert!(offset_of!(FieldTypeTestStruct, std_uint32_t_field) * 8 == 1120usize);
const _: () = assert!(offset_of!(FieldTypeTestStruct, std_uint64_t_field) * 8 == 1152usize);
const _: () = assert!(offset_of!(FieldTypeTestStruct, ptrdiff_t_field) * 8 == 1216usize);
const _: () = assert!(offset_of!(FieldTypeTestStruct, size_t_field) * 8 == 1280usize);
const _: () = assert!(offset_of!(FieldTypeTestStruct, intptr_t_field) * 8 == 1344usize);
const _: () = assert!(offset_of!(FieldTypeTestStruct, uintptr_t_field) * 8 == 1408usize);
const _: () = assert!(offset_of!(FieldTypeTestStruct, std_ptrdiff_t_field) * 8 == 1472usize);
const _: () = assert!(offset_of!(FieldTypeTestStruct, std_size_t_field) * 8 == 1536usize);
const _: () = assert!(offset_of!(FieldTypeTestStruct, std_intptr_t_field) * 8 == 1600usize);
const _: () = assert!(offset_of!(FieldTypeTestStruct, std_uintptr_t_field) * 8 == 1664usize);
const _: () = assert!(offset_of!(FieldTypeTestStruct, float_field) * 8 == 1728usize);
const _: () = assert!(offset_of!(FieldTypeTestStruct, double_field) * 8 == 1792usize);
const _: () = assert!(offset_of!(FieldTypeTestStruct, ptr_field) * 8 == 1856usize);
const _: () = assert!(offset_of!(FieldTypeTestStruct, struct_field) * 8 == 1920usize);
const _: () = assert!(offset_of!(FieldTypeTestStruct, struct_ptr_field) * 8 == 1984usize);
const _: () = assert!(offset_of!(FieldTypeTestStruct, const_struct_ptr_field) * 8 == 2048usize);
const _: () = assert!(offset_of!(FieldTypeTestStruct, struct_ref_field) * 8 == 2112usize);
const _: () = assert!(offset_of!(FieldTypeTestStruct, const_struct_ref_field) * 8 == 2176usize);
