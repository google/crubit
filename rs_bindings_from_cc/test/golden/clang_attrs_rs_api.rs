// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for C++ target
// //rs_bindings_from_cc/test/golden:clang_attrs_cc
#![rustfmt::skip]
#![feature(const_ptr_offset_from, custom_inner_attributes, negative_impls)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use ::std as rust_std;
use memoffset_unstable_const::offset_of;

// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[repr(C, align(64))]
pub struct HasCustomAlignment {
    /// Prevent empty C++ struct being zero-size in Rust.
    placeholder: rust_std::mem::MaybeUninit<u8>,
}
forward_declare::unsafe_define!(forward_declare::symbol!("HasCustomAlignment"), HasCustomAlignment);

impl !Unpin for HasCustomAlignment {}

// rs_bindings_from_cc/test/golden/clang_attrs.h;l=8
// Error while generating bindings for item 'HasCustomAlignment::HasCustomAlignment':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// rs_bindings_from_cc/test/golden/clang_attrs.h;l=8
// Error while generating bindings for item 'HasCustomAlignment::HasCustomAlignment':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// rs_bindings_from_cc/test/golden/clang_attrs.h;l=8
// Error while generating bindings for item 'HasCustomAlignment::HasCustomAlignment':
// Parameter #0 is not supported: Unsupported type 'struct HasCustomAlignment &&': Unsupported type: && without lifetime

// rs_bindings_from_cc/test/golden/clang_attrs.h;l=8
// Error while generating bindings for item 'HasCustomAlignment::operator=':
// Bindings for this kind of operator are not supported

// rs_bindings_from_cc/test/golden/clang_attrs.h;l=8
// Error while generating bindings for item 'HasCustomAlignment::operator=':
// Parameter #0 is not supported: Unsupported type 'struct HasCustomAlignment &&': Unsupported type: && without lifetime

#[repr(C)]
pub struct HasFieldWithCustomAlignment {
    pub field: HasCustomAlignment,
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("HasFieldWithCustomAlignment"),
    HasFieldWithCustomAlignment
);

impl !Unpin for HasFieldWithCustomAlignment {}

// rs_bindings_from_cc/test/golden/clang_attrs.h;l=10
// Error while generating bindings for item 'HasFieldWithCustomAlignment::HasFieldWithCustomAlignment':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// rs_bindings_from_cc/test/golden/clang_attrs.h;l=10
// Error while generating bindings for item 'HasFieldWithCustomAlignment::HasFieldWithCustomAlignment':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// rs_bindings_from_cc/test/golden/clang_attrs.h;l=10
// Error while generating bindings for item 'HasFieldWithCustomAlignment::HasFieldWithCustomAlignment':
// Parameter #0 is not supported: Unsupported type 'struct HasFieldWithCustomAlignment &&': Unsupported type: && without lifetime

// rs_bindings_from_cc/test/golden/clang_attrs.h;l=10
// Error while generating bindings for item 'HasFieldWithCustomAlignment::operator=':
// Bindings for this kind of operator are not supported

// rs_bindings_from_cc/test/golden/clang_attrs.h;l=10
// Error while generating bindings for item 'HasFieldWithCustomAlignment::operator=':
// Parameter #0 is not supported: Unsupported type 'struct HasFieldWithCustomAlignment &&': Unsupported type: && without lifetime

#[repr(C, align(64))]
pub struct InheritsFromBaseWithCustomAlignment {
    __base_class_subobjects: [rust_std::mem::MaybeUninit<u8>; 0],
    /// Prevent empty C++ struct being zero-size in Rust.
    placeholder: rust_std::mem::MaybeUninit<u8>,
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("InheritsFromBaseWithCustomAlignment"),
    InheritsFromBaseWithCustomAlignment
);

impl !Unpin for InheritsFromBaseWithCustomAlignment {}

// rs_bindings_from_cc/test/golden/clang_attrs.h;l=14
// Error while generating bindings for item 'InheritsFromBaseWithCustomAlignment::InheritsFromBaseWithCustomAlignment':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// rs_bindings_from_cc/test/golden/clang_attrs.h;l=14
// Error while generating bindings for item 'InheritsFromBaseWithCustomAlignment::InheritsFromBaseWithCustomAlignment':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// rs_bindings_from_cc/test/golden/clang_attrs.h;l=14
// Error while generating bindings for item 'InheritsFromBaseWithCustomAlignment::InheritsFromBaseWithCustomAlignment':
// Parameter #0 is not supported: Unsupported type 'struct InheritsFromBaseWithCustomAlignment &&': Unsupported type: && without lifetime

// rs_bindings_from_cc/test/golden/clang_attrs.h;l=14
// Error while generating bindings for item 'InheritsFromBaseWithCustomAlignment::operator=':
// Bindings for this kind of operator are not supported

// rs_bindings_from_cc/test/golden/clang_attrs.h;l=14
// Error while generating bindings for item 'InheritsFromBaseWithCustomAlignment::operator=':
// Parameter #0 is not supported: Unsupported type 'struct InheritsFromBaseWithCustomAlignment &&': Unsupported type: && without lifetime

impl<'a> From<&'a InheritsFromBaseWithCustomAlignment> for &'a HasCustomAlignment {
    fn from(x: &'a InheritsFromBaseWithCustomAlignment) -> Self {
        unsafe { &*((x as *const _ as *const u8).offset(0) as *const HasCustomAlignment) }
    }
}

#[repr(C, align(64))]
pub struct HasCustomAlignmentWithGnuAttr {
    /// Prevent empty C++ struct being zero-size in Rust.
    placeholder: rust_std::mem::MaybeUninit<u8>,
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("HasCustomAlignmentWithGnuAttr"),
    HasCustomAlignmentWithGnuAttr
);

impl !Unpin for HasCustomAlignmentWithGnuAttr {}

// rs_bindings_from_cc/test/golden/clang_attrs.h;l=16
// Error while generating bindings for item 'HasCustomAlignmentWithGnuAttr::HasCustomAlignmentWithGnuAttr':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// rs_bindings_from_cc/test/golden/clang_attrs.h;l=16
// Error while generating bindings for item 'HasCustomAlignmentWithGnuAttr::HasCustomAlignmentWithGnuAttr':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// rs_bindings_from_cc/test/golden/clang_attrs.h;l=16
// Error while generating bindings for item 'HasCustomAlignmentWithGnuAttr::HasCustomAlignmentWithGnuAttr':
// Parameter #0 is not supported: Unsupported type 'struct HasCustomAlignmentWithGnuAttr &&': Unsupported type: && without lifetime

// rs_bindings_from_cc/test/golden/clang_attrs.h;l=16
// Error while generating bindings for item 'HasCustomAlignmentWithGnuAttr::operator=':
// Bindings for this kind of operator are not supported

// rs_bindings_from_cc/test/golden/clang_attrs.h;l=16
// Error while generating bindings for item 'HasCustomAlignmentWithGnuAttr::operator=':
// Parameter #0 is not supported: Unsupported type 'struct HasCustomAlignmentWithGnuAttr &&': Unsupported type: && without lifetime

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_CLANG_ATTRS_H_

const _: () = assert!(rust_std::mem::size_of::<Option<&i32>>() == rust_std::mem::size_of::<&i32>());

const _: () = assert!(rust_std::mem::size_of::<HasCustomAlignment>() == 64usize);
const _: () = assert!(rust_std::mem::align_of::<HasCustomAlignment>() == 64usize);
const _: () = {
    static_assertions::assert_not_impl_all!(HasCustomAlignment: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_all!(HasCustomAlignment: Drop);
};

const _: () = assert!(rust_std::mem::size_of::<HasFieldWithCustomAlignment>() == 64usize);
const _: () = assert!(rust_std::mem::align_of::<HasFieldWithCustomAlignment>() == 64usize);
const _: () = {
    static_assertions::assert_not_impl_all!(HasFieldWithCustomAlignment: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_all!(HasFieldWithCustomAlignment: Drop);
};
const _: () = assert!(offset_of!(HasFieldWithCustomAlignment, field) * 8 == 0usize);

const _: () = assert!(rust_std::mem::size_of::<InheritsFromBaseWithCustomAlignment>() == 64usize);
const _: () = assert!(rust_std::mem::align_of::<InheritsFromBaseWithCustomAlignment>() == 64usize);
const _: () = {
    static_assertions::assert_not_impl_all!(InheritsFromBaseWithCustomAlignment: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_all!(InheritsFromBaseWithCustomAlignment: Drop);
};

const _: () = assert!(rust_std::mem::size_of::<HasCustomAlignmentWithGnuAttr>() == 64usize);
const _: () = assert!(rust_std::mem::align_of::<HasCustomAlignmentWithGnuAttr>() == 64usize);
const _: () = {
    static_assertions::assert_not_impl_all!(HasCustomAlignmentWithGnuAttr: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_all!(HasCustomAlignmentWithGnuAttr: Drop);
};
