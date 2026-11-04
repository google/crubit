// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use const_array_fields::{
    AlignedLargeArrays, ArrayPtrFields, ByteFlag, ChecksumLargeBlock, CountMatchingScopedDecayed,
    EnumArrays, IntRow, MakeEnumArrays, MakeMatrixStruct, MakePrimitiveArrays,
    MakeUnionOfArraysFromWords, MakeUnionOfEnumArrays, NonTrivialHasArrayFields, PackedArrayStruct,
    ReadEnumMatrixCell, ReadMatrixCell, ReadUnionOfArraysWord, ReadUnionOfEnumArraysRawWord,
    ScaleInPlaceDecayed, ScopedChannel, SumDecayedAlias, SumEnumFlags, SumPackedArrayStruct,
    SumPrimitiveInts, TrailingZeroLengthArrayHeader, UnscopedColor,
};
use googletest::{expect_eq, gtest};
use std::mem::offset_of;

#[gtest]
fn test_1d_primitive_array_fields_roundtrip() {
    let mut s = MakePrimitiveArrays();
    expect_eq!(s.flags, [true, false, true]);
    expect_eq!(s.chars, [b'a', b'b', b'c', 0]);
    expect_eq!(s.ints, [10, 20, 30, 40]);
    expect_eq!(s.doubles, [1.5, -2.25]);
    expect_eq!(s.bytes, [0xAA, 0xBB, 0xCC, 0xDD]);
    expect_eq!(s.const_ints, [7, 9]);

    s.ints[2] = 300;
    expect_eq!(SumPrimitiveInts(&s), 10 + 20 + 300 + 40 + 7 + 9);
}

#[gtest]
fn test_asymmetric_2d_and_aliased_array_indexing_across_ffi() {
    let mut m = MakeMatrixStruct();
    let _: [[::core::ffi::c_int; 4]; 2] = m.matrix;
    let _: [IntRow; 2] = m.alias_matrix;

    expect_eq!(m.matrix, [[0, 1, 2, 3], [100, 101, 102, 103]]);
    expect_eq!(m.alias_matrix, [[0, 1, 2, 3], [1000, 1001, 1002, 1003]]);

    // Mutate [1][3] on the Rust side and read back from C++ to prove identical row-major stride.
    m.matrix[1][3] = 9876;
    expect_eq!(ReadMatrixCell(&m, 1, 3), 9876);
    expect_eq!(ReadMatrixCell(&m, 1, 2), 102);

    // Pointers to C-style arrays in fields are erased to opaque byte blobs without preventing the
    // enclosing struct from binding.
    let ptrs = ArrayPtrFields::default();
    expect_eq!(ptrs.id, 42);
}

#[gtest]
fn test_nontrivial_destructor_struct_with_primitive_array_field() {
    let mut s = NonTrivialHasArrayFields::default();
    expect_eq!(s.bar, [11, 22, 33, 44]);
    s.bar[2] = 330;
    expect_eq!(s.bar, [11, 22, 330, 44]);
}

#[gtest]
fn test_union_array_type_punning_across_ffi() {
    let mut u = MakeUnionOfArraysFromWords(0x04030201u32.to_le(), 0x08070605u32.to_le());
    // SAFETY: `words` is the active member of the union, so should be safe to read.
    let words = unsafe { u.words };
    expect_eq!(words, [0x04030201u32.to_le(), 0x08070605u32.to_le()]);

    u.words = [0x40302010u32.to_le(), 0x80706050u32.to_le()];
    expect_eq!(
        // SAFETY: `words` is still the active member of the union, and we read in bounds.
        unsafe { ReadUnionOfArraysWord(&u, 0) },
        u32::from_le_bytes([0x10, 0x20, 0x30, 0x40])
    );
    expect_eq!(
        // SAFETY: `words` is still the active member of the union, and we read in bounds.
        unsafe { ReadUnionOfArraysWord(&u, 1) },
        u32::from_le_bytes([0x50, 0x60, 0x70, 0x80])
    );
}

#[gtest]
fn test_large_arrays_and_overaligned_layout() {
    expect_eq!(align_of::<AlignedLargeArrays>(), 16);
    expect_eq!(size_of::<AlignedLargeArrays>(), 320);
    expect_eq!(offset_of!(AlignedLargeArrays, data), 0);
    expect_eq!(offset_of!(AlignedLargeArrays, block), 64);

    let mut a = AlignedLargeArrays::default();
    expect_eq!(a.data[0], 0);
    expect_eq!(a.block[255], 0);

    a.data[0] = 3i8.into();
    a.data[63] = 7i8.into();
    a.block[0] = 11;
    a.block[255] = 21;

    // Exercise Copy and Clone for N > 32 arrays (64 and 256 elements).
    let copied = a;
    let cloned = copied.clone();
    expect_eq!(ChecksumLargeBlock(&a), 42);
    expect_eq!(ChecksumLargeBlock(&cloned), 42);
}

#[gtest]
fn test_decayed_array_parameter_interop_with_array_fields() {
    let mut s = MakePrimitiveArrays();
    // SAFETY: Though ScaleInPlaceDecayed takes a raw pointer, it is non-null and we also pass along
    // the actual length of the array so that accesses are in bounds. The array should also be
    // safe to mutate: it stays live, is not captured, and there are no concurrent reads/writes.
    unsafe {
        ScaleInPlaceDecayed(s.ints.as_mut_ptr(), s.ints.len(), 3);
    }
    expect_eq!(s.ints, [30, 60, 90, 120]);
    // SAFETY: As with ScaleInPlaceDecayed, the pointer is non-null and reads are in bounds.
    let total = unsafe { SumDecayedAlias(s.ints.as_ptr(), s.ints.len()) };
    expect_eq!(total, 300);
}

#[gtest]
fn test_enum_array_fields_1d_2d_alias_template_and_default_roundtrip() {
    let zeroed = EnumArrays::default();
    expect_eq!(zeroed.flags, [ByteFlag::kNone; 4]);
    expect_eq!(zeroed.large_enums[0], UnscopedColor::from(0));
    expect_eq!(zeroed.large_enums[39], UnscopedColor::from(0));

    let mut e = MakeEnumArrays();
    expect_eq!(e.unscoped, [UnscopedColor::kRed, UnscopedColor::kGreen, UnscopedColor::kBlue]);
    expect_eq!(e.scoped, [ScopedChannel::kAlpha, ScopedChannel::kBeta, ScopedChannel::kGamma]);
    expect_eq!(e.flags, [ByteFlag::kNone, ByteFlag::kRead, ByteFlag::kWrite, ByteFlag::kExec]);
    expect_eq!(
        e.matrix,
        [
            [ScopedChannel::kAlpha, ScopedChannel::kBeta, ScopedChannel::kGamma],
            [ScopedChannel::kGamma, ScopedChannel::kAlpha, ScopedChannel::kBeta],
        ]
    );
    expect_eq!(e.alias_grid[0][0], ScopedChannel::kBeta);
    expect_eq!(e.alias_grid[1][2], ScopedChannel::kGamma);
    expect_eq!(e.tmpl_enums, [ScopedChannel::kAlpha, ScopedChannel::kGamma]);
    expect_eq!(e.large_enums[0], UnscopedColor::kRed);
    expect_eq!(e.large_enums[39], UnscopedColor::kBlue);

    // Mutate from Rust across 1D, asymmetric 2D, and Copy/Clone round-trips, then verify in C++.
    e.flags[0] = ByteFlag::kExec;
    e.matrix[1][2] = ScopedChannel::kGamma;
    e.scoped[0] = ScopedChannel::kGamma;
    let copied = e;
    let cloned = copied.clone();

    expect_eq!(SumEnumFlags(&cloned), 4 + 1 + 2 + 4);
    expect_eq!(ReadEnumMatrixCell(&cloned, 1, 2), ScopedChannel::kGamma);
    expect_eq!(ReadEnumMatrixCell(&cloned, 1, 1), ScopedChannel::kAlpha);
    expect_eq!(
        // SAFETY: raw pointer is non-null and we pass in the length of the array to bounds check.
        unsafe {
            CountMatchingScopedDecayed(
                cloned.scoped.as_ptr(),
                cloned.scoped.len(),
                ScopedChannel::kGamma,
            )
        },
        2
    );
}

#[gtest]
fn test_enum_union_array_fields_across_ffi() {
    let mut u = MakeUnionOfEnumArrays(ScopedChannel::kAlpha, ScopedChannel::kGamma);
    // SAFETY: `channels` is the active member of the union, so should be safe to read.
    expect_eq!(unsafe { u.channels }, [ScopedChannel::kAlpha, ScopedChannel::kGamma]);
    // SAFETY: `raw_words` is not the active member of the union, but the enum value
    // can be represented as an int32_t. We read in bounds.
    expect_eq!(unsafe { ReadUnionOfEnumArraysRawWord(&u, 0) }, 10);
    // SAFETY: same as above -- can read as int32_t and we read in bounds.
    expect_eq!(unsafe { ReadUnionOfEnumArraysRawWord(&u, 1) }, 30);

    u.unscoped = [UnscopedColor::kBlue, UnscopedColor::kGreen];
    let copied = u;
    let cloned = copied.clone();
    // SAFETY: same as above -- can read as int32_t and we read in bounds.
    expect_eq!(unsafe { ReadUnionOfEnumArraysRawWord(&cloned, 0) }, 3);
    // SAFETY: same as above -- can read as int32_t and we read in bounds.
    expect_eq!(unsafe { ReadUnionOfEnumArraysRawWord(&cloned, 1) }, 2);
}

#[gtest]
fn test_packed_array_struct() {
    expect_eq!(align_of::<PackedArrayStruct>(), 1);
    expect_eq!(size_of::<PackedArrayStruct>(), 17);
    let mut p = PackedArrayStruct {
        tag: 7,
        ints: [100, 200],
        channels: [ScopedChannel::kAlpha, ScopedChannel::kGamma],
    };
    p.ints[1] = 250;
    let ints = p.ints;
    let channels = p.channels;
    expect_eq!(ints, [100, 250]);
    expect_eq!(channels, [ScopedChannel::kAlpha, ScopedChannel::kGamma]);
    expect_eq!(SumPackedArrayStruct(&p), 7 + 100 + 250 + 10 + 30);
}

#[gtest]
fn test_trailing_zero_length_array_header_alignment() {
    expect_eq!(align_of::<TrailingZeroLengthArrayHeader>(), 8);
    expect_eq!(size_of::<TrailingZeroLengthArrayHeader>(), 8);
    let hdr = TrailingZeroLengthArrayHeader { num_queues: 4, pad: [0; 6] };
    expect_eq!(hdr.num_queues, 4);
    expect_eq!(hdr.pad, [0; 6]);
}
