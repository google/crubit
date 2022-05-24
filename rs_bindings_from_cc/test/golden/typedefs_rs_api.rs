// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for C++ target
// //rs_bindings_from_cc/test/golden:typedefs_cc
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

#[ctor::recursively_pinned]
#[repr(C)]
pub struct SomeStruct {
    __non_field_data: [crate::rust_std::mem::MaybeUninit<u8>; 1],
}
forward_declare::unsafe_define!(forward_declare::symbol!("SomeStruct"), crate::SomeStruct);

// rs_bindings_from_cc/test/golden/typedefs.h;l=8
// Error while generating bindings for item 'SomeStruct::SomeStruct':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// rs_bindings_from_cc/test/golden/typedefs.h;l=8
// Error while generating bindings for item 'SomeStruct::SomeStruct':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// rs_bindings_from_cc/test/golden/typedefs.h;l=8
// Error while generating bindings for item 'SomeStruct::SomeStruct':
// Parameter #0 is not supported: Unsupported type 'struct SomeStruct &&': Unsupported type: && without lifetime

// rs_bindings_from_cc/test/golden/typedefs.h;l=8
// Error while generating bindings for item 'SomeStruct::operator=':
// Bindings for this kind of operator are not supported

// rs_bindings_from_cc/test/golden/typedefs.h;l=8
// Error while generating bindings for item 'SomeStruct::operator=':
// Parameter #0 is not supported: Unsupported type 'struct SomeStruct &&': Unsupported type: && without lifetime

// rs_bindings_from_cc/test/golden/typedefs.h;l=9
// Error while generating bindings for item 'SomeStruct':
// Typedef only used to introduce a name in C. Not importing.

// rs_bindings_from_cc/test/golden/typedefs.h;l=11
// Error while generating bindings for item 'SomeOtherStruct':
// Unsupported type 'struct SomeOtherStruct': No generated bindings found for ''

#[derive(Clone, Copy)]
#[repr(C)]
pub union SomeUnion {
    __non_field_data: [crate::rust_std::mem::MaybeUninit<u8>; 1],
}
forward_declare::unsafe_define!(forward_declare::symbol!("SomeUnion"), crate::SomeUnion);

// rs_bindings_from_cc/test/golden/typedefs.h;l=14
// Error while generating bindings for item 'SomeUnion::SomeUnion':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// rs_bindings_from_cc/test/golden/typedefs.h;l=14
// Error while generating bindings for item 'SomeUnion::SomeUnion':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// rs_bindings_from_cc/test/golden/typedefs.h;l=14
// Error while generating bindings for item 'SomeUnion::SomeUnion':
// Parameter #0 is not supported: Unsupported type 'union SomeUnion &&': Unsupported type: && without lifetime

// rs_bindings_from_cc/test/golden/typedefs.h;l=14
// Error while generating bindings for item 'SomeUnion::operator=':
// Bindings for this kind of operator are not supported

// rs_bindings_from_cc/test/golden/typedefs.h;l=14
// Error while generating bindings for item 'SomeUnion::operator=':
// Parameter #0 is not supported: Unsupported type 'union SomeUnion &&': Unsupported type: && without lifetime

// rs_bindings_from_cc/test/golden/typedefs.h;l=15
// Error while generating bindings for item 'SomeUnion':
// Typedef only used to introduce a name in C. Not importing.

// rs_bindings_from_cc/test/golden/typedefs.h;l=17
// Error while generating bindings for item 'SomeOtherUnion':
// Unsupported type 'union SomeOtherUnion': No generated bindings found for ''

// THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_TYPEDEFS_H_

const _: () = assert!(rust_std::mem::size_of::<Option<&i32>>() == rust_std::mem::size_of::<&i32>());

const _: () = assert!(rust_std::mem::size_of::<crate::SomeStruct>() == 1);
const _: () = assert!(rust_std::mem::align_of::<crate::SomeStruct>() == 1);
const _: () = {
    static_assertions::assert_not_impl_all!(crate::SomeStruct: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_all!(crate::SomeStruct: Drop);
};

const _: () = assert!(rust_std::mem::size_of::<crate::SomeUnion>() == 1);
const _: () = assert!(rust_std::mem::align_of::<crate::SomeUnion>() == 1);
const _: () = {
    static_assertions::assert_impl_all!(crate::SomeUnion: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::SomeUnion: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_all!(crate::SomeUnion: Drop);
};
