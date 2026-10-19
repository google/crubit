// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use core::mem::offset_of;
use googletest::prelude::*;
use packed::*;

#[gtest]
fn test_packed_struct_layout_and_size() {
    expect_eq!(size_of::<PackedStruct>(), kPackedStructSize);
    expect_eq!(align_of::<PackedStruct>(), kPackedStructAlignment);
    expect_eq!(offset_of!(PackedStruct, a), kPackedStructOffsetA);
    expect_eq!(offset_of!(PackedStruct, b), kPackedStructOffsetB);
    expect_eq!(offset_of!(PackedStruct, c), kPackedStructOffsetC);
}

#[gtest]
fn test_packed_struct_array_stride() {
    expect_eq!(size_of::<[PackedStruct; 2]>(), kPackedStructArray2Size);
    expect_eq!(size_of::<[PackedStruct; 3]>(), kPackedStructArray3Size);
}

#[gtest]
fn test_packed_struct_pass_by_value() {
    let s = MakePackedStruct(10, 200, 3000);
    expect_eq!({ s.a }, 10);
    expect_eq!({ s.b }, 200);
    expect_eq!({ s.c }, 3000);
    expect_eq!(SumPackedStruct(s), 3210);
}

#[gtest]
fn test_packed_struct_array_interop_no_copy() {
    let mut arr =
        [MakePackedStruct(1, 10, 100), MakePackedStruct(2, 20, 200), MakePackedStruct(3, 30, 300)];

    // Pass slice pointer directly to C++
    let sum = unsafe { SumPackedArray(arr.as_ptr(), arr.len()) };
    expect_eq!(sum, (1 + 10 + 100) + (2 + 20 + 200) + (3 + 30 + 300));

    // C++ modifies in place across FFI
    unsafe { IncrementPackedArray(arr.as_mut_ptr(), arr.len()) };
    expect_eq!({ arr[0].a }, 2);
    expect_eq!({ arr[0].b }, 11);
    expect_eq!({ arr[0].c }, 101);
    expect_eq!({ arr[1].a }, 3);
    expect_eq!({ arr[1].b }, 21);
    expect_eq!({ arr[1].c }, 201);
    expect_eq!({ arr[2].a }, 4);
    expect_eq!({ arr[2].b }, 31);
    expect_eq!({ arr[2].c }, 301);
}

#[gtest]
fn test_pragma_push_pack_1_layout() {
    expect_eq!(size_of::<PragmaPushPacked1>(), kPragmaPushPacked1Size);
    expect_eq!(align_of::<PragmaPushPacked1>(), kPragmaPushPacked1Alignment);
    expect_eq!(offset_of!(PragmaPushPacked1, a), kPragmaPushPacked1OffsetA);
    expect_eq!(offset_of!(PragmaPushPacked1, b), kPragmaPushPacked1OffsetB);
    expect_eq!(offset_of!(PragmaPushPacked1, c), kPragmaPushPacked1OffsetC);
}

#[gtest]
fn test_pragma_push_pack_1_array_stride() {
    expect_eq!(size_of::<[PragmaPushPacked1; 2]>(), kPragmaPushPacked1Array2Size);
    expect_eq!(size_of::<[PragmaPushPacked1; 3]>(), kPragmaPushPacked1Array3Size);
}

#[gtest]
fn test_bare_pack_1_layout() {
    expect_eq!(size_of::<BarePacked1>(), kBarePacked1Size);
    expect_eq!(align_of::<BarePacked1>(), kBarePacked1Alignment);
    expect_eq!(offset_of!(BarePacked1, a), kBarePacked1OffsetA);
    expect_eq!(offset_of!(BarePacked1, b), kBarePacked1OffsetB);
}

#[gtest]
fn test_bare_pack_1_array_stride() {
    expect_eq!(size_of::<[BarePacked1; 2]>(), kBarePacked1Array2Size);
    expect_eq!(size_of::<[BarePacked1; 3]>(), kBarePacked1Array3Size);
}

#[gtest]
fn test_bare_pack_2_layout() {
    expect_eq!(size_of::<BarePacked2>(), kBarePacked2Size);
    expect_eq!(align_of::<BarePacked2>(), kBarePacked2Alignment);
    expect_eq!(offset_of!(BarePacked2, a), kBarePacked2OffsetA);
    expect_eq!(offset_of!(BarePacked2, b), kBarePacked2OffsetB);
}

#[gtest]
fn test_bare_pack_2_array_stride() {
    expect_eq!(size_of::<[BarePacked2; 2]>(), kBarePacked2Array2Size);
    expect_eq!(size_of::<[BarePacked2; 3]>(), kBarePacked2Array3Size);
}

#[gtest]
fn test_bare_pack_4_layout() {
    expect_eq!(size_of::<BarePacked4>(), kBarePacked4Size);
    expect_eq!(align_of::<BarePacked4>(), kBarePacked4Alignment);
    expect_eq!(offset_of!(BarePacked4, a), kBarePacked4OffsetA);
    expect_eq!(offset_of!(BarePacked4, b), kBarePacked4OffsetB);
}

#[gtest]
fn test_bare_pack_4_array_stride() {
    expect_eq!(size_of::<[BarePacked4; 2]>(), kBarePacked4Array2Size);
    expect_eq!(size_of::<[BarePacked4; 3]>(), kBarePacked4Array3Size);
}

#[gtest]
fn test_pragma_pack_2_layout() {
    expect_eq!(size_of::<PragmaPacked2>(), kPragmaPacked2Size);
    expect_eq!(align_of::<PragmaPacked2>(), kPragmaPacked2Alignment);
    expect_eq!(offset_of!(PragmaPacked2, a), kPragmaPacked2OffsetA);
    expect_eq!(offset_of!(PragmaPacked2, b), kPragmaPacked2OffsetB);
}

#[gtest]
fn test_pragma_pack_2_array_stride() {
    expect_eq!(size_of::<[PragmaPacked2; 2]>(), kPragmaPacked2Array2Size);
    expect_eq!(size_of::<[PragmaPacked2; 3]>(), kPragmaPacked2Array3Size);
}

#[gtest]
fn test_pragma_pack_4_layout() {
    expect_eq!(size_of::<PragmaPacked4>(), kPragmaPacked4Size);
    expect_eq!(align_of::<PragmaPacked4>(), kPragmaPacked4Alignment);
    expect_eq!(offset_of!(PragmaPacked4, a), kPragmaPacked4OffsetA);
    expect_eq!(offset_of!(PragmaPacked4, b), kPragmaPacked4OffsetB);
}

#[gtest]
fn test_pragma_pack_4_array_stride() {
    expect_eq!(size_of::<[PragmaPacked4; 2]>(), kPragmaPacked4Array2Size);
    expect_eq!(size_of::<[PragmaPacked4; 3]>(), kPragmaPacked4Array3Size);
}

#[gtest]
fn test_packed_struct_debug() {
    expect_eq!(
        format!("{:?}", MakePackedStruct(10, 200, 3000)),
        "PackedStruct { a: 10, b: 200, c: 3000 }"
    );
}

#[gtest]
fn test_pragma_pack_2_debug() {
    let s = PragmaPacked2 { a: 1, b: 2 };
    expect_eq!(format!("{s:?}"), "PragmaPacked2 { a: 1, b: 2 }");
}

#[gtest]
fn test_bare_pack_4_debug() {
    let s = BarePacked4 { a: 1, b: 2 };
    expect_eq!(format!("{s:?}"), "BarePacked4 { a: 1, b: 2 }");
}

#[gtest]
fn test_nested_packed_struct_debug() {
    let s = PackedOuter { x: 1, inner: MakePackedStruct(10, 200, 3000) };
    expect_eq!(
        format!("{s:?}"),
        "PackedOuter { x: 1, inner: PackedStruct { a: 10, b: 200, c: 3000 } }"
    );
}

#[gtest]
fn test_packed_struct_with_non_trivially_copyable_field_debug() {
    // `NonTriviallyCopyable` isn't `Copy`, so it can't be copied out of the (possibly unaligned)
    // field to be printed. Instead, no fields are printed.
    let s = PackedWithNonTriviallyCopyableField::default();
    expect_eq!(format!("{s:?}"), "PackedWithNonTriviallyCopyableField { .. }");
}
