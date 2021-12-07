#![rustfmt::skip]
// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#![feature(const_maybe_uninit_as_ptr, const_ptr_offset_from, custom_inner_attributes)]

use memoffset_unstable_const::offset_of;

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
// Empty parameter names are not supported

// rs_bindings_from_cc/test/golden/comment.h;l=11
// Error while generating bindings for item 'Foo::operator=':
// Empty parameter names are not supported

// rs_bindings_from_cc/test/golden/comment.h;l=11
// Error while generating bindings for item 'Foo::Foo':
// Parameter type 'struct Foo &&' is not supported

// rs_bindings_from_cc/test/golden/comment.h;l=11
// Error while generating bindings for item 'Foo::operator=':
// Parameter type 'struct Foo &&' is not supported

// b

// }  // namespace ns

// c

/// foo
#[inline(always)]
pub fn foo() -> () {
    unsafe { crate::detail::__rust_thunk___Z3foov() }
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
// Empty parameter names are not supported

// rs_bindings_from_cc/test/golden/comment.h;l=37
// Error while generating bindings for item 'Bar::operator=':
// Empty parameter names are not supported

// rs_bindings_from_cc/test/golden/comment.h;l=37
// Error while generating bindings for item 'Bar::Bar':
// Parameter type 'struct Bar &&' is not supported

// rs_bindings_from_cc/test/golden/comment.h;l=37
// Error while generating bindings for item 'Bar::operator=':
// Parameter type 'struct Bar &&' is not supported

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
// Empty parameter names are not supported

// rs_bindings_from_cc/test/golden/comment.h;l=43
// Error while generating bindings for item 'HasNoComments::operator=':
// Empty parameter names are not supported

// rs_bindings_from_cc/test/golden/comment.h;l=43
// Error while generating bindings for item 'HasNoComments::HasNoComments':
// Parameter type 'struct HasNoComments &&' is not supported

// rs_bindings_from_cc/test/golden/comment.h;l=43
// Error while generating bindings for item 'HasNoComments::operator=':
// Parameter type 'struct HasNoComments &&' is not supported

// e

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_COMMENT_H_

mod detail {
    use super::*;
    extern "C" {
        pub(crate) fn __rust_thunk___ZN3FooC1Ev(__this: *mut Foo) -> ();
        pub(crate) fn __rust_thunk___Z3foov() -> ();
        pub(crate) fn __rust_thunk___ZN3BarC1Ev(__this: *mut Bar) -> ();
        pub(crate) fn __rust_thunk___ZN13HasNoCommentsC1Ev(__this: *mut HasNoComments) -> ();
    }
}

const _: () = assert!(std::mem::size_of::<Option<&i32>>() == std::mem::size_of::<&i32>());

const _: () = assert!(std::mem::size_of::<Foo>() == 8usize);
const _: () = assert!(std::mem::align_of::<Foo>() == 4usize);
const _: () = assert!(offset_of!(Foo, i) * 8 == 0usize);
const _: () = assert!(offset_of!(Foo, j) * 8 == 32usize);

const _: () = assert!(std::mem::size_of::<Bar>() == 4usize);
const _: () = assert!(std::mem::align_of::<Bar>() == 4usize);
const _: () = assert!(offset_of!(Bar, i) * 8 == 0usize);

const _: () = assert!(std::mem::size_of::<HasNoComments>() == 4usize);
const _: () = assert!(std::mem::align_of::<HasNoComments>() == 4usize);
const _: () = assert!(offset_of!(HasNoComments, i) * 8 == 0usize);
