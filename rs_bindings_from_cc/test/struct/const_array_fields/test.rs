// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use const_array_fields::*;
use googletest::gtest;

#[gtest]
fn test_1d_primitive_array_fields_roundtrip() {
    let mut s = MakePrimitiveArrays();
    assert_eq!(s.flags, [true, false, true]);
    assert_eq!(s.chars, [b'a', b'b', b'c', 0]);
    assert_eq!(s.ints, [10, 20, 30, 40]);
    assert_eq!(s.doubles, [1.5, -2.25]);
    assert_eq!(s.bytes, [0xAA, 0xBB, 0xCC, 0xDD]);
    assert_eq!(s.const_ints, [7, 9]);

    s.ints[2] = 300;
    assert_eq!(SumPrimitiveInts(&s), 10 + 20 + 300 + 40 + 7 + 9);
}

#[gtest]
fn test_asymmetric_2d_and_aliased_array_indexing_across_ffi() {
    let mut m = MakeMatrixStruct();
    let _: [[::core::ffi::c_int; 4]; 2] = m.matrix;
    let _: [IntRow; 2] = m.alias_matrix;

    assert_eq!(m.matrix, [[0, 1, 2, 3], [100, 101, 102, 103]]);
    assert_eq!(m.alias_matrix, [[0, 1, 2, 3], [1000, 1001, 1002, 1003]]);

    // Mutate [1][3] on the Rust side and read back from C++ to prove identical row-major stride.
    m.matrix[1][3] = 9876;
    assert_eq!(ReadMatrixCell(&m, 1, 3), 9876);
    assert_eq!(ReadMatrixCell(&m, 1, 2), 102);

    // Pointers to C-style arrays in fields are erased to opaque byte blobs without preventing the
    // enclosing struct from binding.
    let ptrs = ArrayPtrFields::default();
    assert_eq!(ptrs.id, 42);
}

#[gtest]
fn test_nontrivial_destructor_struct_with_primitive_array_field() {
    let mut s = NonTrivialHasArrayFields::default();
    assert_eq!(s.bar, [11, 22, 33, 44]);
    s.bar[2] = 330;
    assert_eq!(s.bar, [11, 22, 330, 44]);
}

#[gtest]
fn test_union_array_type_punning_across_ffi() {
    let mut u = MakeUnionOfArraysFromWords(0x04030201u32.to_le(), 0x08070605u32.to_le());
    let bytes = unsafe { u.bytes };
    assert_eq!(bytes, [1, 2, 3, 4, 5, 6, 7, 8]);

    u.bytes = [0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80];
    assert_eq!(
        unsafe { ReadUnionOfArraysWord(&u, 0) },
        u32::from_ne_bytes([0x10, 0x20, 0x30, 0x40])
    );
    assert_eq!(
        unsafe { ReadUnionOfArraysWord(&u, 1) },
        u32::from_ne_bytes([0x50, 0x60, 0x70, 0x80])
    );
}

#[gtest]
fn test_large_arrays_and_overaligned_layout() {
    assert_eq!(core::mem::align_of::<AlignedLargeArrays>(), 16);
    assert_eq!(core::mem::size_of::<AlignedLargeArrays>(), 320);
    assert_eq!(core::mem::offset_of!(AlignedLargeArrays, data), 0);
    assert_eq!(core::mem::offset_of!(AlignedLargeArrays, block), 64);

    let mut a = AlignedLargeArrays::default();
    assert_eq!(a.data[0], 0);
    assert_eq!(a.block[255], 0);

    a.data[0] = 3i8.into();
    a.data[63] = 7i8.into();
    a.block[0] = 11;
    a.block[255] = 21;

    // Exercise Copy and Clone for N > 32 arrays (64 and 256 elements).
    let copied = a;
    let cloned = copied.clone();
    assert_eq!(ChecksumLargeBlock(&a), 42);
    assert_eq!(ChecksumLargeBlock(&cloned), 42);
}

#[gtest]
fn test_decayed_array_parameter_interop_with_array_fields() {
    let mut s = MakePrimitiveArrays();
    unsafe {
        ScaleInPlaceDecayed(s.ints.as_mut_ptr(), 3);
    }
    assert_eq!(s.ints, [30, 60, 90, 120]);
    let total = unsafe { SumDecayedAlias(s.ints.as_ptr()) };
    assert_eq!(total, 300);
}

#[gtest]
fn test_enum_array_fields_1d_2d_alias_template_and_default_roundtrip() {
    let zeroed = EnumArrays::default();
    assert_eq!(zeroed.flags, [ByteFlag::kNone; 4]);
    assert_eq!(zeroed.large_enums[0], UnscopedColor::from(0));
    assert_eq!(zeroed.large_enums[39], UnscopedColor::from(0));

    let mut e = MakeEnumArrays();
    assert_eq!(e.unscoped, [UnscopedColor::kRed, UnscopedColor::kGreen, UnscopedColor::kBlue]);
    assert_eq!(e.scoped, [ScopedChannel::kAlpha, ScopedChannel::kBeta, ScopedChannel::kGamma]);
    assert_eq!(e.flags, [ByteFlag::kNone, ByteFlag::kRead, ByteFlag::kWrite, ByteFlag::kExec]);
    assert_eq!(
        e.matrix,
        [
            [ScopedChannel::kAlpha, ScopedChannel::kBeta, ScopedChannel::kGamma],
            [ScopedChannel::kGamma, ScopedChannel::kAlpha, ScopedChannel::kBeta],
        ]
    );
    assert_eq!(e.alias_grid[0][0], ScopedChannel::kBeta);
    assert_eq!(e.alias_grid[1][2], ScopedChannel::kGamma);
    assert_eq!(e.tmpl_enums, [ScopedChannel::kAlpha, ScopedChannel::kGamma]);
    assert_eq!(e.large_enums[0], UnscopedColor::kRed);
    assert_eq!(e.large_enums[39], UnscopedColor::kBlue);

    // Mutate from Rust across 1D, asymmetric 2D, and Copy/Clone round-trips, then verify in C++.
    e.flags[0] = ByteFlag::kExec;
    e.matrix[1][2] = ScopedChannel::kGamma;
    e.scoped[0] = ScopedChannel::kGamma;
    let copied = e;
    let cloned = copied.clone();

    assert_eq!(SumEnumFlags(&cloned), 4 + 1 + 2 + 4);
    assert_eq!(ReadEnumMatrixCell(&cloned, 1, 2), ScopedChannel::kGamma);
    assert_eq!(ReadEnumMatrixCell(&cloned, 1, 1), ScopedChannel::kAlpha);
    assert_eq!(
        unsafe { CountMatchingScopedDecayed(cloned.scoped.as_ptr(), ScopedChannel::kGamma) },
        2
    );
}

#[gtest]
fn test_enum_union_array_fields_across_ffi() {
    let mut u = MakeUnionOfEnumArrays(ScopedChannel::kAlpha, ScopedChannel::kGamma);
    assert_eq!(unsafe { u.channels }, [ScopedChannel::kAlpha, ScopedChannel::kGamma]);
    assert_eq!(unsafe { ReadUnionOfEnumArraysRawWord(&u, 0) }, 10);
    assert_eq!(unsafe { ReadUnionOfEnumArraysRawWord(&u, 1) }, 30);

    u.unscoped = [UnscopedColor::kBlue, UnscopedColor::kGreen];
    let copied = u;
    let cloned = copied.clone();
    assert_eq!(unsafe { ReadUnionOfEnumArraysRawWord(&cloned, 0) }, 3);
    assert_eq!(unsafe { ReadUnionOfEnumArraysRawWord(&cloned, 1) }, 2);
}

#[gtest]
fn test_packed_array_struct() {
    assert_eq!(core::mem::align_of::<PackedArrayStruct>(), 1);
    assert_eq!(core::mem::size_of::<PackedArrayStruct>(), 17);
    let mut p = PackedArrayStruct {
        tag: 7,
        ints: [100, 200],
        channels: [ScopedChannel::kAlpha, ScopedChannel::kGamma],
    };
    p.ints[1] = 250;
    let ints = p.ints;
    let channels = p.channels;
    assert_eq!(ints, [100, 250]);
    assert_eq!(channels, [ScopedChannel::kAlpha, ScopedChannel::kGamma]);
    assert_eq!(SumPackedArrayStruct(&p), 7 + 100 + 250 + 10 + 30);
}

#[gtest]
fn test_trailing_zero_length_array_header_alignment() {
    assert_eq!(core::mem::align_of::<TrailingZeroLengthArrayHeader>(), 8);
    assert_eq!(core::mem::size_of::<TrailingZeroLengthArrayHeader>(), 8);
    let hdr = TrailingZeroLengthArrayHeader { num_queues: 4, pad: [0; 6] };
    assert_eq!(hdr.num_queues, 4);
    assert_eq!(hdr.pad, [0; 6]);
}
