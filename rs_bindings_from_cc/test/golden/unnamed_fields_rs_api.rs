// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for C++ target
// //rs_bindings_from_cc/test/golden:unnamed_fields_cc
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

#[repr(C)]
pub struct WithUnnamedFields {
    pub foo: i32,
    pub __unnamed_field1: i32,
    pub bar: i32,
    pub __unnamed_field3: i32,
    pub baz: i32,
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("WithUnnamedFields"),
    crate::WithUnnamedFields
);

impl !Unpin for WithUnnamedFields {}

// rs_bindings_from_cc/test/golden/unnamed_fields.h;l=8
// Error while generating bindings for item 'WithUnnamedFields::WithUnnamedFields':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// rs_bindings_from_cc/test/golden/unnamed_fields.h;l=8
// Error while generating bindings for item 'WithUnnamedFields::WithUnnamedFields':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// rs_bindings_from_cc/test/golden/unnamed_fields.h;l=8
// Error while generating bindings for item 'WithUnnamedFields::WithUnnamedFields':
// Parameter #0 is not supported: Unsupported type 'struct WithUnnamedFields &&': Unsupported type: && without lifetime

// rs_bindings_from_cc/test/golden/unnamed_fields.h;l=8
// Error while generating bindings for item 'WithUnnamedFields::operator=':
// Bindings for this kind of operator are not supported

// rs_bindings_from_cc/test/golden/unnamed_fields.h;l=8
// Error while generating bindings for item 'WithUnnamedFields::operator=':
// Parameter #0 is not supported: Unsupported type 'struct WithUnnamedFields &&': Unsupported type: && without lifetime

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_UNNAMED_FIELDS_H_

const _: () = assert!(rust_std::mem::size_of::<Option<&i32>>() == rust_std::mem::size_of::<&i32>());

const _: () = assert!(rust_std::mem::size_of::<crate::WithUnnamedFields>() == 20usize);
const _: () = assert!(rust_std::mem::align_of::<crate::WithUnnamedFields>() == 4usize);
const _: () = {
    static_assertions::assert_not_impl_all!(crate::WithUnnamedFields: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_all!(crate::WithUnnamedFields: Drop);
};
const _: () =
    assert!(memoffset_unstable_const::offset_of!(crate::WithUnnamedFields, foo) * 8 == 0usize);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::WithUnnamedFields, __unnamed_field1) * 8 == 32usize
);
const _: () =
    assert!(memoffset_unstable_const::offset_of!(crate::WithUnnamedFields, bar) * 8 == 64usize);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::WithUnnamedFields, __unnamed_field3) * 8 == 96usize
);
const _: () =
    assert!(memoffset_unstable_const::offset_of!(crate::WithUnnamedFields, baz) * 8 == 128usize);
