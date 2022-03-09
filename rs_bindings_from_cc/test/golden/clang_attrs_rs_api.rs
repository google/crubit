// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for C++ target
// //rs_bindings_from_cc/test/golden:clang_attrs_cc
#![rustfmt::skip]
#![feature(const_ptr_offset_from, custom_inner_attributes, negative_impls)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use memoffset_unstable_const::offset_of;

pub type __builtin_ms_va_list = *mut u8;

#[repr(C, align(64))]
pub struct HasCustomAlignment {
    /// Prevent empty C++ struct being zero-size in Rust.
    placeholder: std::mem::MaybeUninit<u8>,
}

impl !Unpin for HasCustomAlignment {}

// rs_bindings_from_cc/test/golden/clang_attrs.h;l=4
// Error while generating bindings for item 'HasCustomAlignment::HasCustomAlignment':
// Bindings for constructors of non-trivial types are not supported yet

// rs_bindings_from_cc/test/golden/clang_attrs.h;l=4
// Error while generating bindings for item 'HasCustomAlignment::HasCustomAlignment':
// Bindings for constructors of non-trivial types are not supported yet

// rs_bindings_from_cc/test/golden/clang_attrs.h;l=4
// Error while generating bindings for item 'HasCustomAlignment::HasCustomAlignment':
// Parameter #0 is not supported: Unsupported type 'struct HasCustomAlignment &&': Unsupported clang::Type class 'RValueReference'

// rs_bindings_from_cc/test/golden/clang_attrs.h;l=4
// Error while generating bindings for item 'HasCustomAlignment::operator=':
// Bindings for this kind of operator are not supported

// rs_bindings_from_cc/test/golden/clang_attrs.h;l=4
// Error while generating bindings for item 'HasCustomAlignment::operator=':
// Parameter #0 is not supported: Unsupported type 'struct HasCustomAlignment &&': Unsupported clang::Type class 'RValueReference'

#[repr(C)]
pub struct HasFieldWithCustomAlignment {
    pub field: HasCustomAlignment,
}

impl !Unpin for HasFieldWithCustomAlignment {}

// rs_bindings_from_cc/test/golden/clang_attrs.h;l=6
// Error while generating bindings for item 'HasFieldWithCustomAlignment::HasFieldWithCustomAlignment':
// Bindings for constructors of non-trivial types are not supported yet

// rs_bindings_from_cc/test/golden/clang_attrs.h;l=6
// Error while generating bindings for item 'HasFieldWithCustomAlignment::HasFieldWithCustomAlignment':
// Bindings for constructors of non-trivial types are not supported yet

// rs_bindings_from_cc/test/golden/clang_attrs.h;l=6
// Error while generating bindings for item 'HasFieldWithCustomAlignment::HasFieldWithCustomAlignment':
// Parameter #0 is not supported: Unsupported type 'struct HasFieldWithCustomAlignment &&': Unsupported clang::Type class 'RValueReference'

// rs_bindings_from_cc/test/golden/clang_attrs.h;l=6
// Error while generating bindings for item 'HasFieldWithCustomAlignment::operator=':
// Bindings for this kind of operator are not supported

// rs_bindings_from_cc/test/golden/clang_attrs.h;l=6
// Error while generating bindings for item 'HasFieldWithCustomAlignment::operator=':
// Parameter #0 is not supported: Unsupported type 'struct HasFieldWithCustomAlignment &&': Unsupported clang::Type class 'RValueReference'

#[repr(C, align(64))]
pub struct InheritsFromBaseWithCustomAlignment {
    __base_class_subobjects: [std::mem::MaybeUninit<u8>; 0],
    /// Prevent empty C++ struct being zero-size in Rust.
    placeholder: std::mem::MaybeUninit<u8>,
}
impl<'a> From<&'a InheritsFromBaseWithCustomAlignment> for &'a HasCustomAlignment {
    fn from(x: &'a InheritsFromBaseWithCustomAlignment) -> Self {
        unsafe { &*((x as *const _ as *const u8).offset(0) as *const HasCustomAlignment) }
    }
}

impl !Unpin for InheritsFromBaseWithCustomAlignment {}

// rs_bindings_from_cc/test/golden/clang_attrs.h;l=10
// Error while generating bindings for item 'InheritsFromBaseWithCustomAlignment::InheritsFromBaseWithCustomAlignment':
// Bindings for constructors of non-trivial types are not supported yet

// rs_bindings_from_cc/test/golden/clang_attrs.h;l=10
// Error while generating bindings for item 'InheritsFromBaseWithCustomAlignment::InheritsFromBaseWithCustomAlignment':
// Bindings for constructors of non-trivial types are not supported yet

// rs_bindings_from_cc/test/golden/clang_attrs.h;l=10
// Error while generating bindings for item 'InheritsFromBaseWithCustomAlignment::InheritsFromBaseWithCustomAlignment':
// Parameter #0 is not supported: Unsupported type 'struct InheritsFromBaseWithCustomAlignment &&': Unsupported clang::Type class 'RValueReference'

// rs_bindings_from_cc/test/golden/clang_attrs.h;l=10
// Error while generating bindings for item 'InheritsFromBaseWithCustomAlignment::operator=':
// Bindings for this kind of operator are not supported

// rs_bindings_from_cc/test/golden/clang_attrs.h;l=10
// Error while generating bindings for item 'InheritsFromBaseWithCustomAlignment::operator=':
// Parameter #0 is not supported: Unsupported type 'struct InheritsFromBaseWithCustomAlignment &&': Unsupported clang::Type class 'RValueReference'

#[repr(C, align(64))]
pub struct HasCustomAlignmentWithGnuAttr {
    /// Prevent empty C++ struct being zero-size in Rust.
    placeholder: std::mem::MaybeUninit<u8>,
}

impl !Unpin for HasCustomAlignmentWithGnuAttr {}

// rs_bindings_from_cc/test/golden/clang_attrs.h;l=12
// Error while generating bindings for item 'HasCustomAlignmentWithGnuAttr::HasCustomAlignmentWithGnuAttr':
// Bindings for constructors of non-trivial types are not supported yet

// rs_bindings_from_cc/test/golden/clang_attrs.h;l=12
// Error while generating bindings for item 'HasCustomAlignmentWithGnuAttr::HasCustomAlignmentWithGnuAttr':
// Bindings for constructors of non-trivial types are not supported yet

// rs_bindings_from_cc/test/golden/clang_attrs.h;l=12
// Error while generating bindings for item 'HasCustomAlignmentWithGnuAttr::HasCustomAlignmentWithGnuAttr':
// Parameter #0 is not supported: Unsupported type 'struct HasCustomAlignmentWithGnuAttr &&': Unsupported clang::Type class 'RValueReference'

// rs_bindings_from_cc/test/golden/clang_attrs.h;l=12
// Error while generating bindings for item 'HasCustomAlignmentWithGnuAttr::operator=':
// Bindings for this kind of operator are not supported

// rs_bindings_from_cc/test/golden/clang_attrs.h;l=12
// Error while generating bindings for item 'HasCustomAlignmentWithGnuAttr::operator=':
// Parameter #0 is not supported: Unsupported type 'struct HasCustomAlignmentWithGnuAttr &&': Unsupported clang::Type class 'RValueReference'

// THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_CLANG_ATTRS_H_

const _: () = assert!(std::mem::size_of::<Option<&i32>>() == std::mem::size_of::<&i32>());

const _: () = assert!(std::mem::size_of::<HasCustomAlignment>() == 64usize);
const _: () = assert!(std::mem::align_of::<HasCustomAlignment>() == 64usize);

const _: () = assert!(std::mem::size_of::<HasFieldWithCustomAlignment>() == 64usize);
const _: () = assert!(std::mem::align_of::<HasFieldWithCustomAlignment>() == 64usize);
const _: () = assert!(offset_of!(HasFieldWithCustomAlignment, field) * 8 == 0usize);

const _: () = assert!(std::mem::size_of::<InheritsFromBaseWithCustomAlignment>() == 64usize);
const _: () = assert!(std::mem::align_of::<InheritsFromBaseWithCustomAlignment>() == 64usize);

const _: () = assert!(std::mem::size_of::<HasCustomAlignmentWithGnuAttr>() == 64usize);
const _: () = assert!(std::mem::align_of::<HasCustomAlignmentWithGnuAttr>() == 64usize);
