// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! This crate is used as a test input for `cc_bindings_from_rs` and the
//! generated C++ bindings are then tested via `consts_test.cc`.

#![cfg_attr(rustfmt, rustfmt_skip)] // It's nice to keep everything on one line :)

use crubit_annotate::must_bind;

#[must_bind] pub const RUST_TRUE: bool = true;
#[must_bind] pub const RUST_FALSE: bool = false;
#[must_bind] pub const RUST_INT8_MIN: i8 = i8::MIN;
#[must_bind] pub const RUST_INT8_MAX: i8 = i8::MAX;
#[must_bind] pub const RUST_INT16_MIN: i16 = i16::MIN;
#[must_bind] pub const RUST_INT16_MAX: i16 = i16::MAX;
#[must_bind] pub const RUST_INT32_MIN: i32 = i32::MIN;
#[must_bind] pub const RUST_INT32_MAX: i32 = i32::MAX;
#[must_bind] pub const RUST_INT64_MIN: i64 = i64::MIN;
#[must_bind] pub const RUST_INT64_MAX: i64 = i64::MAX;
#[must_bind] pub const RUST_UINT8_MIN: u8 = u8::MIN;
#[must_bind] pub const RUST_UINT8_MAX: u8 = u8::MAX;
#[must_bind] pub const RUST_UINT16_MIN: u16 = u16::MIN;
#[must_bind] pub const RUST_UINT16_MAX: u16 = u16::MAX;
#[must_bind] pub const RUST_UINT32_MIN: u32 = u32::MIN;
#[must_bind] pub const RUST_UINT32_MAX: u32 = u32::MAX;
#[must_bind] pub const RUST_UINT64_MIN: u64 = u64::MIN;
#[must_bind] pub const RUST_UINT64_MAX: u64 = u64::MAX;
#[must_bind] pub const RUST_ISIZE_MIN: isize = isize::MIN;
#[must_bind] pub const RUST_ISIZE_MAX: isize = isize::MAX;
#[must_bind] pub const RUST_USIZE_MIN: isize = isize::MIN;
#[must_bind] pub const RUST_USIZE_MAX: isize = isize::MAX;
#[must_bind] pub const RUST_F32_MIN: f32 = f32::MIN;
#[must_bind] pub const RUST_F32_MAX: f32 = f32::MAX;
#[must_bind] pub const RUST_F64_MIN: f64 = f64::MIN;
#[must_bind] pub const RUST_F64_MAX: f64 = f64::MAX;

#[must_bind] pub const INT_POS: i32 = 42;
#[must_bind] pub const INT_NEG: i32 = -17;
#[must_bind] pub const FLOAT_32: f32 = 0.125; // 2^(-3)
#[must_bind] pub const FLOAT_64: f64 = 0.0078125; // 2^(-7)
#[must_bind] pub const LARGE_INT: i64 = 9223372036854775807;
#[must_bind] pub const UNSIGNED_INT: u32 = 4294967295;
#[must_bind] pub const SLICE_LENGTH: usize = "hello world".len();
#[must_bind] pub const ISIZE: isize = 42;
#[must_bind] pub const CHAR: core::ffi::c_char = 42;

pub struct TyWithAssocConsts(#[allow(dead_code)] u8);

impl TyWithAssocConsts {
    pub const ASSOC_42: i32 = 42;
}