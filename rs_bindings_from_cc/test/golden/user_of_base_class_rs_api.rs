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

/// The same as Derived from inheritance.h, but in a different build target.
///
/// This tests inheritance across library boundaries.
///
/// TODO(b/216195042): Correctly namespace base classes in generated Rust code.
#[derive(Clone, Copy)]
#[repr(C, align(8))]
pub struct Derived2 {
    __base_class_subobjects: [std::mem::MaybeUninit<u8>; 12],
    pub derived_1: u8,
}
impl<'a> From<&'a Derived2> for &'a Base0 {
    fn from(x: &'a Derived2) -> Self {
        unsafe { &*((x as *const _ as *const u8).offset(0) as *const Base0) }
    }
}
impl<'a> From<&'a Derived2> for &'a Base1 {
    fn from(x: &'a Derived2) -> Self {
        unsafe { &*((x as *const _ as *const u8).offset(0) as *const Base1) }
    }
}
impl<'a> From<&'a Derived2> for &'a Base2 {
    fn from(x: &'a Derived2) -> Self {
        unsafe { &*((x as *const _ as *const u8).offset(10) as *const Base2) }
    }
}

// rs_bindings_from_cc/test/golden/user_of_base_class.h;l=11
// Error while generating bindings for item 'Derived2::Derived2':
// Parameter type 'struct Derived2 &&' is not supported

// rs_bindings_from_cc/test/golden/user_of_base_class.h;l=11
// Error while generating bindings for item 'Derived2::operator=':
// Parameter type 'struct Derived2 &&' is not supported

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_USER_OF_BASE_CLASS_H_

const _: () = assert!(std::mem::size_of::<Option<&i32>>() == std::mem::size_of::<&i32>());

const _: () = assert!(std::mem::size_of::<Derived2>() == 16usize);
const _: () = assert!(std::mem::align_of::<Derived2>() == 8usize);
const _: () = assert!(offset_of!(Derived2, derived_1) * 8 == 96usize);
