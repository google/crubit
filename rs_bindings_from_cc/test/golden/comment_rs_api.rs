// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for C++ target
// //rs_bindings_from_cc/test/golden:comment_cc
#![rustfmt::skip]
#![feature(const_ptr_offset_from, custom_inner_attributes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use memoffset_unstable_const::offset_of;

pub type __builtin_ms_va_list = *mut u8;

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

impl Default for Foo {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN3FooC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

// rs_bindings_from_cc/test/golden/comment.h;l=13
// Error while generating bindings for item 'Foo::Foo':
// Parameter #0 is not supported: Unsupported type 'struct Foo &&'

// rs_bindings_from_cc/test/golden/comment.h;l=13
// Error while generating bindings for item 'Foo::operator=':
// Bindings for this kind of operator are not supported

// rs_bindings_from_cc/test/golden/comment.h;l=13
// Error while generating bindings for item 'Foo::operator=':
// Parameter #0 is not supported: Unsupported type 'struct Foo &&'

// b

// }  // namespace ns

// c

/// foo
#[inline(always)]
pub fn foo() {
    unsafe { crate::detail::__rust_thunk___Z3foov() }
}

/// Bar
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Bar {
    pub i: i32,
}

impl Default for Bar {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN3BarC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

// rs_bindings_from_cc/test/golden/comment.h;l=39
// Error while generating bindings for item 'Bar::Bar':
// Parameter #0 is not supported: Unsupported type 'struct Bar &&'

// rs_bindings_from_cc/test/golden/comment.h;l=39
// Error while generating bindings for item 'Bar::operator=':
// Bindings for this kind of operator are not supported

// rs_bindings_from_cc/test/golden/comment.h;l=39
// Error while generating bindings for item 'Bar::operator=':
// Parameter #0 is not supported: Unsupported type 'struct Bar &&'

/// d
#[derive(Clone, Copy)]
#[repr(C)]
pub struct HasNoComments {
    pub i: i32,
}

impl Default for HasNoComments {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN13HasNoCommentsC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

// rs_bindings_from_cc/test/golden/comment.h;l=45
// Error while generating bindings for item 'HasNoComments::HasNoComments':
// Parameter #0 is not supported: Unsupported type 'struct HasNoComments &&'

// rs_bindings_from_cc/test/golden/comment.h;l=45
// Error while generating bindings for item 'HasNoComments::operator=':
// Bindings for this kind of operator are not supported

// rs_bindings_from_cc/test/golden/comment.h;l=45
// Error while generating bindings for item 'HasNoComments::operator=':
// Parameter #0 is not supported: Unsupported type 'struct HasNoComments &&'

// e

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_COMMENT_H_

mod detail {
    #[allow(unused_imports)]
    use super::*;
    extern "C" {
        pub(crate) fn __rust_thunk___ZN3FooC1Ev<'a>(__this: &'a mut std::mem::MaybeUninit<Foo>);
        pub(crate) fn __rust_thunk___Z3foov();
        pub(crate) fn __rust_thunk___ZN3BarC1Ev<'a>(__this: &'a mut std::mem::MaybeUninit<Bar>);
        pub(crate) fn __rust_thunk___ZN13HasNoCommentsC1Ev<'a>(
            __this: &'a mut std::mem::MaybeUninit<HasNoComments>,
        );
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
