#![rustfmt::skip]
// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#![feature(const_ptr_offset_from, custom_inner_attributes, negative_impls)]
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

/// Using classes to force these to be non-POD.
/// In the Itanium ABI, the tail padding of POD types cannot be reused by other
/// objects, even if the POD type is potentially-overlapping.
#[repr(C)]
pub struct Base0 {
    /// Prevent empty C++ struct being zero-size in Rust.
    placeholder: std::mem::MaybeUninit<u8>,
}

impl !Unpin for Base0 {}

// rs_bindings_from_cc/test/golden/inheritance.h;l=9
// Error while generating bindings for item 'Base0::Base0':
// Parameter type 'class Base0 &&' is not supported

// rs_bindings_from_cc/test/golden/inheritance.h;l=9
// Error while generating bindings for item 'Base0::operator=':
// Parameter type 'class Base0 &&' is not supported

#[repr(C)]
pub struct Base1 {
    b1_1_: i64,
    b1_2_: u8,
}

impl !Unpin for Base1 {}

// rs_bindings_from_cc/test/golden/inheritance.h;l=10
// Error while generating bindings for item 'Base1::Base1':
// Parameter type 'class Base1 &&' is not supported

// rs_bindings_from_cc/test/golden/inheritance.h;l=10
// Error while generating bindings for item 'Base1::operator=':
// Parameter type 'class Base1 &&' is not supported

#[repr(C)]
pub struct Base2 {
    b2_1_: i16,
}

impl !Unpin for Base2 {}

// rs_bindings_from_cc/test/golden/inheritance.h;l=15
// Error while generating bindings for item 'Base2::Base2':
// Parameter type 'class Base2 &&' is not supported

// rs_bindings_from_cc/test/golden/inheritance.h;l=15
// Error while generating bindings for item 'Base2::operator=':
// Parameter type 'class Base2 &&' is not supported

#[derive(Clone, Copy)]
#[repr(C, align(8))]
pub struct Derived {
    __base_class_subobjects: [std::mem::MaybeUninit<u8>; 12],
    pub derived_1: u8,
}

// rs_bindings_from_cc/test/golden/inheritance.h;l=19
// Error while generating bindings for item 'Derived::Derived':
// Parameter type 'struct Derived &&' is not supported

// rs_bindings_from_cc/test/golden/inheritance.h;l=19
// Error while generating bindings for item 'Derived::operator=':
// Parameter type 'struct Derived &&' is not supported

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_INHERITANCE_H_

const _: () = assert!(std::mem::size_of::<Option<&i32>>() == std::mem::size_of::<&i32>());

const _: () = assert!(std::mem::size_of::<Base0>() == 1usize);
const _: () = assert!(std::mem::align_of::<Base0>() == 1usize);

const _: () = assert!(std::mem::size_of::<Base1>() == 16usize);
const _: () = assert!(std::mem::align_of::<Base1>() == 8usize);
const _: () = assert!(offset_of!(Base1, b1_1_) * 8 == 0usize);
const _: () = assert!(offset_of!(Base1, b1_2_) * 8 == 64usize);

const _: () = assert!(std::mem::size_of::<Base2>() == 2usize);
const _: () = assert!(std::mem::align_of::<Base2>() == 2usize);
const _: () = assert!(offset_of!(Base2, b2_1_) * 8 == 0usize);

const _: () = assert!(std::mem::size_of::<Derived>() == 16usize);
const _: () = assert!(std::mem::align_of::<Derived>() == 8usize);
const _: () = assert!(offset_of!(Derived, derived_1) * 8 == 96usize);
