#![rustfmt::skip]
// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#![feature(const_maybe_uninit_as_ptr, const_ptr_offset_from, custom_inner_attributes)]

use memoffset_unstable_const::offset_of;
use static_assertions::const_assert_eq;

// File comment

// TODO(b/202933018): Re-enable once namespaces are supported
// namespace ns {
// a

/// Foo
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Foo {
    /// A field
    pub i: i32,
    /// Another field
    pub j: i32,
}

// rs_bindings_from_cc/test/golden/comment.h;l=11
// Error while generating bindings for item 'Foo::Foo':
// Nested classes are not supported yet

// rs_bindings_from_cc/test/golden/comment.h;l=11
// Error while generating bindings for item 'Foo::Foo':
// Parameter type 'const struct Foo &' is not supported

// rs_bindings_from_cc/test/golden/comment.h;l=11
// Error while generating bindings for item 'Foo::operator=':
// Parameter type 'const struct Foo &' is not supported

// <unknown location>
// Error while generating bindings for item 'Foo::operator=':
// Return type 'struct Foo &' is not supported

// rs_bindings_from_cc/test/golden/comment.h;l=11
// Error while generating bindings for item 'Foo::Foo':
// Parameter type 'struct Foo &&' is not supported

// rs_bindings_from_cc/test/golden/comment.h;l=11
// Error while generating bindings for item 'Foo::operator=':
// Parameter type 'struct Foo &&' is not supported

// <unknown location>
// Error while generating bindings for item 'Foo::operator=':
// Return type 'struct Foo &' is not supported

// b

// }  // namespace ns

// c

/// foo
#[inline(always)]
pub fn foo() -> () {
    unsafe { crate::detail::__rust_thunk__foo() }
}

/// Bar
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Bar {
    pub i: i32,
}

// rs_bindings_from_cc/test/golden/comment.h;l=37
// Error while generating bindings for item 'Bar::Bar':
// Nested classes are not supported yet

// rs_bindings_from_cc/test/golden/comment.h;l=37
// Error while generating bindings for item 'Bar::Bar':
// Parameter type 'const struct Bar &' is not supported

// rs_bindings_from_cc/test/golden/comment.h;l=37
// Error while generating bindings for item 'Bar::operator=':
// Parameter type 'const struct Bar &' is not supported

// <unknown location>
// Error while generating bindings for item 'Bar::operator=':
// Return type 'struct Bar &' is not supported

// rs_bindings_from_cc/test/golden/comment.h;l=37
// Error while generating bindings for item 'Bar::Bar':
// Parameter type 'struct Bar &&' is not supported

// rs_bindings_from_cc/test/golden/comment.h;l=37
// Error while generating bindings for item 'Bar::operator=':
// Parameter type 'struct Bar &&' is not supported

// <unknown location>
// Error while generating bindings for item 'Bar::operator=':
// Return type 'struct Bar &' is not supported

/// d
#[derive(Clone, Copy)]
#[repr(C)]
pub struct HasNoComments {
    pub i: i32,
}

// rs_bindings_from_cc/test/golden/comment.h;l=43
// Error while generating bindings for item 'HasNoComments::HasNoComments':
// Nested classes are not supported yet

// rs_bindings_from_cc/test/golden/comment.h;l=43
// Error while generating bindings for item 'HasNoComments::HasNoComments':
// Parameter type 'const struct HasNoComments &' is not supported

// rs_bindings_from_cc/test/golden/comment.h;l=43
// Error while generating bindings for item 'HasNoComments::operator=':
// Parameter type 'const struct HasNoComments &' is not supported

// <unknown location>
// Error while generating bindings for item 'HasNoComments::operator=':
// Return type 'struct HasNoComments &' is not supported

// rs_bindings_from_cc/test/golden/comment.h;l=43
// Error while generating bindings for item 'HasNoComments::HasNoComments':
// Parameter type 'struct HasNoComments &&' is not supported

// rs_bindings_from_cc/test/golden/comment.h;l=43
// Error while generating bindings for item 'HasNoComments::operator=':
// Parameter type 'struct HasNoComments &&' is not supported

// <unknown location>
// Error while generating bindings for item 'HasNoComments::operator=':
// Return type 'struct HasNoComments &' is not supported

// e

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_COMMENT_H_

mod detail {
    use super::*;
    extern "C" {
        pub(crate) fn __rust_constructor_thunk__Foo(__this: *mut Foo) -> ();
        pub(crate) fn __rust_thunk__foo() -> ();
        pub(crate) fn __rust_constructor_thunk__Bar(__this: *mut Bar) -> ();
        pub(crate) fn __rust_constructor_thunk__HasNoComments(__this: *mut HasNoComments) -> ();
    }
}

const_assert_eq!(std::mem::size_of::<Foo>(), 8usize);
const_assert_eq!(std::mem::align_of::<Foo>(), 4usize);
const_assert_eq!(offset_of!(Foo, i) * 8, 0usize);
const_assert_eq!(offset_of!(Foo, j) * 8, 32usize);

const_assert_eq!(std::mem::size_of::<Bar>(), 4usize);
const_assert_eq!(std::mem::align_of::<Bar>(), 4usize);
const_assert_eq!(offset_of!(Bar, i) * 8, 0usize);

const_assert_eq!(std::mem::size_of::<HasNoComments>(), 4usize);
const_assert_eq!(std::mem::align_of::<HasNoComments>(), 4usize);
const_assert_eq!(offset_of!(HasNoComments, i) * 8, 0usize);
