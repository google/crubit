// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use aligned_fields::{
    GetMid, GetReservedByte, GetTail, GetTypedefVal, GetX, MakeOveralignedFields, OveralignedFields,
};
use core::mem::{align_of, offset_of, size_of};
use googletest::prelude::*;

#[gtest]
fn test_overaligned_fields_layout() {
    expect_eq!(align_of::<OveralignedFields>(), 16);
    expect_eq!(size_of::<OveralignedFields>(), 64);
    expect_eq!(offset_of!(OveralignedFields, prefix), 0);
    // `mid` follows `typedef_val` (offset 8..12) and `x` (offset 16..20); without
    // per-field alignment padding, `typedef_val` and `x` would be at 4..8 and 8..12,
    // placing `mid` at offset 12 instead of 20.
    expect_eq!(offset_of!(OveralignedFields, mid), 20);
    // `tail` follows `reserved` (offset 32..48); without per-field alignment padding,
    // `reserved` (alignment 1) would be at 21..37, placing `tail` at 37 instead of 48.
    expect_eq!(offset_of!(OveralignedFields, tail), 48);
}

#[gtest]
fn test_overaligned_fields_by_value_and_field_access() {
    let mut s = MakeOveralignedFields();
    // Non-overaligned fields (`prefix`, `mid`, `tail`) are public fields in Rust.
    expect_eq!(s.prefix, 10);
    expect_eq!(s.mid, 40);
    expect_eq!(s.tail, 50);

    // Mutate the public fields in Rust and pass `s` by value back to C++, verifying
    // that C++ sees the updated public fields and that the opaque byte blobs preserve
    // `typedef_val`, `x`, and `reserved` without overlapping.
    s.prefix = 11;
    s.mid = 41;
    s.tail = 51;
    // Note: Functions accepting `OveralignedFields` are `unsafe` in Rust because the
    // overaligned fields are represented as opaque `MaybeUninit<u8>` blobs.
    unsafe {
        expect_eq!(GetMid(s), 41);
        expect_eq!(GetTail(s), 51);
        expect_eq!(GetTypedefVal(s), 2000);
        expect_eq!(GetX(s), 3000);
        expect_eq!(GetReservedByte(s, 0), 100);
        expect_eq!(GetReservedByte(s, 15), 115);
    }
}
