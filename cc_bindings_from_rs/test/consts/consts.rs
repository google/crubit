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
#[must_bind] pub const RUST_F32_MIN_POSITIVE: f32 = f32::MIN_POSITIVE;
#[must_bind] pub const RUST_F32_EPSILON: f32 = f32::EPSILON;
#[must_bind] pub const RUST_F64_MIN: f64 = f64::MIN;
#[must_bind] pub const RUST_F64_MAX: f64 = f64::MAX;
#[must_bind] pub const RUST_F64_MIN_POSITIVE: f64 = f64::MIN_POSITIVE;
#[must_bind] pub const RUST_F64_EPSILON: f64 = f64::EPSILON;

#[must_bind] pub const INT_POS: i32 = 42;
#[must_bind] pub const INT_NEG: i32 = -17;
#[must_bind] pub const FLOAT_32: f32 = 0.125; // 2^(-3)
#[must_bind] pub const FLOAT_32_WHOLE: f32 = 64.0;
#[must_bind] pub const FLOAT_32_NEG_ZERO: f32 = -0.0;
#[must_bind] pub const FLOAT_32_MIN_SUBNORMAL: f32 = f32::from_bits(1);
#[must_bind] pub const FLOAT_32_MAX_SUBNORMAL: f32 = f32::from_bits(0x007f_ffff);
#[must_bind] pub const FLOAT_32_NEG_SUBNORMAL: f32 = f32::from_bits(0x8000_0001);
#[must_bind] pub const FLOAT_32_INFINITY: f32 = f32::INFINITY;
#[must_bind] pub const FLOAT_32_NEG_INFINITY: f32 = f32::NEG_INFINITY;
#[must_bind] pub const FLOAT_32_NAN: f32 = f32::NAN;
#[must_bind] pub const FLOAT_32_NEG_NAN: f32 = -f32::NAN;
#[must_bind] pub const FLOAT_32_PAYLOAD_NAN: f32 = f32::from_bits(0x7fc0_0042);
#[must_bind] pub const FLOAT_32_NEG_PAYLOAD_NAN: f32 = f32::from_bits(0xffc0_0042);
#[must_bind] pub const FLOAT_32_MAX_PAYLOAD_NAN: f32 = f32::from_bits(0x7fff_ffff);
#[must_bind] pub const FLOAT_32_SIGNALING_NAN: f32 = f32::from_bits(0x7f80_0001);
#[must_bind] pub const FLOAT_32_NEG_SIGNALING_NAN: f32 = f32::from_bits(0xff80_0001);
#[must_bind] pub const FLOAT_64: f64 = 0.0078125; // 2^(-7)
#[must_bind] pub const FLOAT_64_WHOLE: f64 = 64.0;
#[must_bind] pub const FLOAT_64_NEG_ZERO: f64 = -0.0;
#[must_bind] pub const FLOAT_64_MIN_SUBNORMAL: f64 = f64::from_bits(1);
#[must_bind] pub const FLOAT_64_MAX_SUBNORMAL: f64 = f64::from_bits(0x000f_ffff_ffff_ffff);
#[must_bind] pub const FLOAT_64_NEG_SUBNORMAL: f64 = f64::from_bits(0x8000_0000_0000_0001);
#[must_bind] pub const FLOAT_64_INFINITY: f64 = f64::INFINITY;
#[must_bind] pub const FLOAT_64_NEG_INFINITY: f64 = f64::NEG_INFINITY;
#[must_bind] pub const FLOAT_64_NAN: f64 = f64::NAN;
#[must_bind] pub const FLOAT_64_NEG_NAN: f64 = -f64::NAN;
#[must_bind] pub const FLOAT_64_PAYLOAD_NAN: f64 = f64::from_bits(0x7ff8_0000_0000_0042);
#[must_bind] pub const FLOAT_64_NEG_PAYLOAD_NAN: f64 = f64::from_bits(0xfff8_0000_0000_0042);
#[must_bind] pub const FLOAT_64_MAX_PAYLOAD_NAN: f64 = f64::from_bits(0x7fff_ffff_ffff_ffff);
#[must_bind] pub const FLOAT_64_SIGNALING_NAN: f64 = f64::from_bits(0x7ff0_0000_0000_0001);
#[must_bind] pub const FLOAT_64_NEG_SIGNALING_NAN: f64 = f64::from_bits(0xfff0_0000_0000_0001);
#[must_bind] pub const LARGE_INT: i64 = 9223372036854775807;
#[must_bind] pub const UNSIGNED_INT: u32 = 4294967295;
#[must_bind] pub const SLICE_LENGTH: usize = "hello world".len();
#[must_bind] pub const ISIZE: isize = 42;
#[must_bind] pub const CHAR: ffi_11::c_char = ffi_11::c_char::new(42u8);

pub struct TyWithAssocConsts(#[allow(dead_code)] u8);

impl TyWithAssocConsts {
    pub const ASSOC_42: i32 = 42;
    pub const ASSOC_POINT: Point = Point { x: 5, y: 6 };
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(C)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    #[must_bind]
    pub const ZERO: Point = Point { x: 0, y: 0 };
}

#[must_bind]
pub const POINT_CONST: Point = Point { x: 10, y: -20 };

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct TupleStruct(pub i32, pub u32);

#[must_bind]
pub const TUPLE_STRUCT_CONST: TupleStruct = TupleStruct(123, 456);

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct NestedStruct {
    pub point: Point,
    pub tuple: TupleStruct,
}

#[must_bind]
pub const NESTED_STRUCT_CONST: NestedStruct = NestedStruct {
    point: Point { x: 1, y: 2 },
    tuple: TupleStruct(3, 4),
};

#[must_bind]
pub const ARRAY_CONST: [i32; 3] = [10, 20, 30];

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct FloatStruct {
    pub x: f32,
    pub y: f32,
}

#[must_bind]
pub const FLOAT_STRUCT_CONST: FloatStruct = FloatStruct {
    x: -0.0,
    y: f32::INFINITY,
};

#[must_bind]
pub const FLOAT_ARRAY_CONST: [f32; 4] = [64.0, -0.0, f32::NEG_INFINITY, f32::NAN];

#[must_bind]
pub const FLOAT_64_ARRAY_CONST: [f64; 2] = [-0.0, f64::INFINITY];

#[must_bind]
pub fn f32_to_bits(x: f32) -> u32 {
    x.to_bits()
}

#[must_bind]
pub fn f64_to_bits(x: f64) -> u64 {
    x.to_bits()
}

#[must_bind]
pub fn roundtrip_f32(x: f32) -> f32 {
    x
}

#[must_bind]
pub fn roundtrip_f64(x: f64) -> f64 {
    x
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct StructWithArray {
    pub values: [i32; 2],
}

#[must_bind]
pub const STRUCT_WITH_ARRAY_CONST: StructWithArray = StructWithArray {
    values: [100, 200],
};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct StructWithStr<'a> {
    pub msg: &'a str,
    pub count: i32,
}

#[must_bind]
pub const STRUCT_WITH_STR_CONST: StructWithStr<'static> = StructWithStr {
    msg: "hello world",
    count: 42,
};

// Test that const expressions that diverge do not receive bindings.
// It's important that it's a generic function otherwise the compiler will eagerly evaluate the
// const and fail to compile.
pub fn fn_with_unreturnable_const<T>() {
    const {
        panic!("diverging const");
    };
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct StructWithRef<'a> {
    pub r: &'a i32,
}

// Cannot be generated because StructWithRef has a sized reference field and is not a C++ aggregate.
pub const STRUCT_WITH_REF_CONST: StructWithRef<'static> = StructWithRef {
    r: &42,
};