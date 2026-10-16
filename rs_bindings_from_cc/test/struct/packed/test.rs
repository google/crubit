// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::prelude::*;
use packed::*;

#[gtest]
fn test_packed_struct_layout_and_size() {
    assert_eq!(core::mem::size_of::<PackedStruct>(), kPackedStructSize);
    assert_eq!(core::mem::align_of::<PackedStruct>(), kPackedStructAlignment);
    assert_eq!(core::mem::offset_of!(PackedStruct, a), kPackedStructOffsetA);
    assert_eq!(core::mem::offset_of!(PackedStruct, b), kPackedStructOffsetB);
    assert_eq!(core::mem::offset_of!(PackedStruct, c), kPackedStructOffsetC);
}

#[gtest]
fn test_packed_struct_array_stride() {
    assert_eq!(core::mem::size_of::<[PackedStruct; 2]>(), kPackedStructArray2Size);
    assert_eq!(core::mem::size_of::<[PackedStruct; 3]>(), kPackedStructArray3Size);
}

#[gtest]
fn test_packed_struct_pass_by_value() {
    let s = MakePackedStruct(10, 200, 3000);
    assert_eq!({ s.a }, 10);
    assert_eq!({ s.b }, 200);
    assert_eq!({ s.c }, 3000);
    assert_eq!(SumPackedStruct(s), 3210);
}

#[gtest]
fn test_packed_struct_array_interop_no_copy() {
    let mut arr =
        [MakePackedStruct(1, 10, 100), MakePackedStruct(2, 20, 200), MakePackedStruct(3, 30, 300)];

    // Pass slice pointer directly to C++
    let sum = unsafe { SumPackedArray(arr.as_ptr(), arr.len()) };
    assert_eq!(sum, (1 + 10 + 100) + (2 + 20 + 200) + (3 + 30 + 300));

    // C++ modifies in place across FFI
    unsafe { IncrementPackedArray(arr.as_mut_ptr(), arr.len()) };
    assert_eq!({ arr[0].a }, 2);
    assert_eq!({ arr[0].b }, 11);
    assert_eq!({ arr[0].c }, 101);
    assert_eq!({ arr[1].a }, 3);
    assert_eq!({ arr[1].b }, 21);
    assert_eq!({ arr[1].c }, 201);
    assert_eq!({ arr[2].a }, 4);
    assert_eq!({ arr[2].b }, 31);
    assert_eq!({ arr[2].c }, 301);
}

#[gtest]
fn test_pragma_push_pack_1_layout() {
    assert_eq!(core::mem::size_of::<PragmaPushPacked1>(), kPragmaPushPacked1Size);
    assert_eq!(core::mem::align_of::<PragmaPushPacked1>(), kPragmaPushPacked1Alignment);
    assert_eq!(core::mem::offset_of!(PragmaPushPacked1, a), kPragmaPushPacked1OffsetA);
    assert_eq!(core::mem::offset_of!(PragmaPushPacked1, b), kPragmaPushPacked1OffsetB);
    assert_eq!(core::mem::offset_of!(PragmaPushPacked1, c), kPragmaPushPacked1OffsetC);
}

#[gtest]
fn test_pragma_push_pack_1_array_stride() {
    assert_eq!(core::mem::size_of::<[PragmaPushPacked1; 2]>(), kPragmaPushPacked1Array2Size);
    assert_eq!(core::mem::size_of::<[PragmaPushPacked1; 3]>(), kPragmaPushPacked1Array3Size);
}

#[gtest]
fn test_bare_pack_1_layout() {
    assert_eq!(core::mem::size_of::<BarePacked1>(), kBarePacked1Size);
    assert_eq!(core::mem::align_of::<BarePacked1>(), kBarePacked1Alignment);
    assert_eq!(core::mem::offset_of!(BarePacked1, a), kBarePacked1OffsetA);
    assert_eq!(core::mem::offset_of!(BarePacked1, b), kBarePacked1OffsetB);
}

#[gtest]
fn test_bare_pack_1_array_stride() {
    assert_eq!(core::mem::size_of::<[BarePacked1; 2]>(), kBarePacked1Array2Size);
    assert_eq!(core::mem::size_of::<[BarePacked1; 3]>(), kBarePacked1Array3Size);
}

#[gtest]
fn test_bare_pack_2_layout() {
    assert_eq!(core::mem::size_of::<BarePacked2>(), kBarePacked2Size);
    assert_eq!(core::mem::align_of::<BarePacked2>(), kBarePacked2Alignment);
    assert_eq!(core::mem::offset_of!(BarePacked2, a), kBarePacked2OffsetA);
    assert_eq!(core::mem::offset_of!(BarePacked2, b), kBarePacked2OffsetB);
}

#[gtest]
fn test_bare_pack_2_array_stride() {
    assert_eq!(core::mem::size_of::<[BarePacked2; 2]>(), kBarePacked2Array2Size);
    assert_eq!(core::mem::size_of::<[BarePacked2; 3]>(), kBarePacked2Array3Size);
}

#[gtest]
fn test_bare_pack_4_layout() {
    assert_eq!(core::mem::size_of::<BarePacked4>(), kBarePacked4Size);
    assert_eq!(core::mem::align_of::<BarePacked4>(), kBarePacked4Alignment);
    assert_eq!(core::mem::offset_of!(BarePacked4, a), kBarePacked4OffsetA);
    assert_eq!(core::mem::offset_of!(BarePacked4, b), kBarePacked4OffsetB);
}

#[gtest]
fn test_bare_pack_4_array_stride() {
    assert_eq!(core::mem::size_of::<[BarePacked4; 2]>(), kBarePacked4Array2Size);
    assert_eq!(core::mem::size_of::<[BarePacked4; 3]>(), kBarePacked4Array3Size);
}

#[gtest]
fn test_pragma_pack_2_layout() {
    assert_eq!(core::mem::size_of::<PragmaPacked2>(), kPragmaPacked2Size);
    assert_eq!(core::mem::align_of::<PragmaPacked2>(), kPragmaPacked2Alignment);
    assert_eq!(core::mem::offset_of!(PragmaPacked2, a), kPragmaPacked2OffsetA);
    assert_eq!(core::mem::offset_of!(PragmaPacked2, b), kPragmaPacked2OffsetB);
}

#[gtest]
fn test_pragma_pack_2_array_stride() {
    assert_eq!(core::mem::size_of::<[PragmaPacked2; 2]>(), kPragmaPacked2Array2Size);
    assert_eq!(core::mem::size_of::<[PragmaPacked2; 3]>(), kPragmaPacked2Array3Size);
}

#[gtest]
fn test_pragma_pack_4_layout() {
    assert_eq!(core::mem::size_of::<PragmaPacked4>(), kPragmaPacked4Size);
    assert_eq!(core::mem::align_of::<PragmaPacked4>(), kPragmaPacked4Alignment);
    assert_eq!(core::mem::offset_of!(PragmaPacked4, a), kPragmaPacked4OffsetA);
    assert_eq!(core::mem::offset_of!(PragmaPacked4, b), kPragmaPacked4OffsetB);
}

#[gtest]
fn test_pragma_pack_4_array_stride() {
    assert_eq!(core::mem::size_of::<[PragmaPacked4; 2]>(), kPragmaPacked4Array2Size);
    assert_eq!(core::mem::size_of::<[PragmaPacked4; 3]>(), kPragmaPacked4Array3Size);
}
