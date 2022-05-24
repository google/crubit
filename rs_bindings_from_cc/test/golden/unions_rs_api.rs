// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for C++ target
// //rs_bindings_from_cc/test/golden:unions_cc
#![rustfmt::skip]
#![feature(const_ptr_offset_from, custom_inner_attributes, negative_impls)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![deny(warnings)]

use ::std as rust_std;

// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[derive(Clone, Copy)]
#[repr(C)]
pub union EmptyUnion {
    __non_field_data: [crate::rust_std::mem::MaybeUninit<u8>; 1],
}
forward_declare::unsafe_define!(forward_declare::symbol!("EmptyUnion"), crate::EmptyUnion);

// rs_bindings_from_cc/test/golden/unions.h;l=8
// Error while generating bindings for item 'EmptyUnion::EmptyUnion':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// rs_bindings_from_cc/test/golden/unions.h;l=8
// Error while generating bindings for item 'EmptyUnion::EmptyUnion':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// rs_bindings_from_cc/test/golden/unions.h;l=8
// Error while generating bindings for item 'EmptyUnion::EmptyUnion':
// Parameter #0 is not supported: Unsupported type 'union EmptyUnion &&': Unsupported type: && without lifetime

// rs_bindings_from_cc/test/golden/unions.h;l=8
// Error while generating bindings for item 'EmptyUnion::operator=':
// Bindings for this kind of operator are not supported

// rs_bindings_from_cc/test/golden/unions.h;l=8
// Error while generating bindings for item 'EmptyUnion::operator=':
// Parameter #0 is not supported: Unsupported type 'union EmptyUnion &&': Unsupported type: && without lifetime

#[ctor::recursively_pinned]
#[repr(C)]
pub struct Nontrivial {
    __non_field_data: [crate::rust_std::mem::MaybeUninit<u8>; 0],
    pub field: i32,
}
forward_declare::unsafe_define!(forward_declare::symbol!("Nontrivial"), crate::Nontrivial);

// rs_bindings_from_cc/test/golden/unions.h;l=11
// Error while generating bindings for item 'Nontrivial::Nontrivial':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// rs_bindings_from_cc/test/golden/unions.h;l=12
// Error while generating bindings for item 'Nontrivial::Nontrivial':
// Parameter #0 is not supported: Unsupported type 'struct Nontrivial &&': Unsupported type: && without lifetime

#[derive(Clone, Copy)]
#[repr(C)]
pub union NonEmptyUnion {
    pub bool_field: bool,
    pub char_field: u8,
    pub int_field: i32,
    pub long_long_field: i64,
}
forward_declare::unsafe_define!(forward_declare::symbol!("NonEmptyUnion"), crate::NonEmptyUnion);

// rs_bindings_from_cc/test/golden/unions.h;l=17
// Error while generating bindings for item 'NonEmptyUnion::NonEmptyUnion':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// rs_bindings_from_cc/test/golden/unions.h;l=17
// Error while generating bindings for item 'NonEmptyUnion::NonEmptyUnion':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// rs_bindings_from_cc/test/golden/unions.h;l=17
// Error while generating bindings for item 'NonEmptyUnion::NonEmptyUnion':
// Parameter #0 is not supported: Unsupported type 'union NonEmptyUnion &&': Unsupported type: && without lifetime

// rs_bindings_from_cc/test/golden/unions.h;l=17
// Error while generating bindings for item 'NonEmptyUnion::operator=':
// Bindings for this kind of operator are not supported

// rs_bindings_from_cc/test/golden/unions.h;l=17
// Error while generating bindings for item 'NonEmptyUnion::operator=':
// Parameter #0 is not supported: Unsupported type 'union NonEmptyUnion &&': Unsupported type: && without lifetime

#[repr(C)]
pub union NonCopyUnion {
    pub trivial_member: bool,
    pub nontrivial_member: crate::rust_std::mem::ManuallyDrop<crate::Nontrivial>,
}
forward_declare::unsafe_define!(forward_declare::symbol!("NonCopyUnion"), crate::NonCopyUnion);

#[repr(C)]
pub union UnionWithOpaqueField {
    /// Reason for representing this field as a blob of bytes:
    /// Unsupported type 'char[42]': Unsupported clang::Type class 'ConstantArray'
    constant_array_field_not_yet_supported: [crate::rust_std::mem::MaybeUninit<u8>; 42],
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("UnionWithOpaqueField"),
    crate::UnionWithOpaqueField
);

// rs_bindings_from_cc/test/golden/unions.h;l=29
// Error while generating bindings for item 'UnionWithOpaqueField::UnionWithOpaqueField':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// rs_bindings_from_cc/test/golden/unions.h;l=29
// Error while generating bindings for item 'UnionWithOpaqueField::UnionWithOpaqueField':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// rs_bindings_from_cc/test/golden/unions.h;l=29
// Error while generating bindings for item 'UnionWithOpaqueField::UnionWithOpaqueField':
// Parameter #0 is not supported: Unsupported type 'union UnionWithOpaqueField &&': Unsupported type: && without lifetime

// rs_bindings_from_cc/test/golden/unions.h;l=29
// Error while generating bindings for item 'UnionWithOpaqueField::operator=':
// Bindings for this kind of operator are not supported

// rs_bindings_from_cc/test/golden/unions.h;l=29
// Error while generating bindings for item 'UnionWithOpaqueField::operator=':
// Parameter #0 is not supported: Unsupported type 'union UnionWithOpaqueField &&': Unsupported type: && without lifetime

// THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_UNIONS_H_

const _: () = assert!(rust_std::mem::size_of::<Option<&i32>>() == rust_std::mem::size_of::<&i32>());

const _: () = assert!(rust_std::mem::size_of::<crate::EmptyUnion>() == 1);
const _: () = assert!(rust_std::mem::align_of::<crate::EmptyUnion>() == 1);
const _: () = {
    static_assertions::assert_impl_all!(crate::EmptyUnion: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::EmptyUnion: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_all!(crate::EmptyUnion: Drop);
};

const _: () = assert!(rust_std::mem::size_of::<crate::Nontrivial>() == 4);
const _: () = assert!(rust_std::mem::align_of::<crate::Nontrivial>() == 4);
const _: () = {
    static_assertions::assert_not_impl_all!(crate::Nontrivial: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_all!(crate::Nontrivial: Drop);
};
const _: () = assert!(memoffset_unstable_const::offset_of!(crate::Nontrivial, field) == 0);

const _: () = assert!(rust_std::mem::size_of::<crate::NonEmptyUnion>() == 8);
const _: () = assert!(rust_std::mem::align_of::<crate::NonEmptyUnion>() == 8);
const _: () = {
    static_assertions::assert_impl_all!(crate::NonEmptyUnion: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::NonEmptyUnion: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_all!(crate::NonEmptyUnion: Drop);
};
const _: () = {
    static_assertions::assert_impl_all!(bool: Copy);
};
const _: () = {
    static_assertions::assert_impl_all!(u8: Copy);
};
const _: () = {
    static_assertions::assert_impl_all!(i32: Copy);
};
const _: () = {
    static_assertions::assert_impl_all!(i64: Copy);
};

const _: () = assert!(rust_std::mem::size_of::<crate::NonCopyUnion>() == 4);
const _: () = assert!(rust_std::mem::align_of::<crate::NonCopyUnion>() == 4);
const _: () = {
    static_assertions::assert_not_impl_all!(crate::NonCopyUnion: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_all!(crate::NonCopyUnion: Drop);
};
const _: () = {
    static_assertions::assert_impl_all!(bool: Copy);
};

const _: () = assert!(rust_std::mem::size_of::<crate::UnionWithOpaqueField>() == 42);
const _: () = assert!(rust_std::mem::align_of::<crate::UnionWithOpaqueField>() == 1);
const _: () = {
    static_assertions::assert_not_impl_all!(crate::UnionWithOpaqueField: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_all!(crate::UnionWithOpaqueField: Drop);
};
