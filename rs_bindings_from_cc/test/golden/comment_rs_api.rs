#![rustfmt::skip]
// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

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

// rs_bindings_from_cc/test/golden/comment.h;l=11
// Error while generating bindings for item 'Foo::Foo':
// Nested classes are not supported yet

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

impl From<*const Foo> for Foo {
    #[inline(always)]
    fn from(__param_0: *const Foo) -> Self {
        let mut tmp = std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN3FooC1ERKS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

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
pub fn foo() {
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

impl From<*const Bar> for Bar {
    #[inline(always)]
    fn from(__param_0: *const Bar) -> Self {
        let mut tmp = std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN3BarC1ERKS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

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

impl From<*const HasNoComments> for HasNoComments {
    #[inline(always)]
    fn from(__param_0: *const HasNoComments) -> Self {
        let mut tmp = std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN13HasNoCommentsC1ERKS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

// rs_bindings_from_cc/test/golden/comment.h;l=43
// Error while generating bindings for item 'HasNoComments::HasNoComments':
// Parameter type 'struct HasNoComments &&' is not supported

// rs_bindings_from_cc/test/golden/comment.h;l=43
// Error while generating bindings for item 'HasNoComments::operator=':
// Parameter type 'struct HasNoComments &&' is not supported

// e

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_COMMENT_H_

mod detail {
    #[allow(unused_imports)]
    use super::*;
    extern "C" {
        pub(crate) fn __rust_thunk___ZN3FooC1Ev(__this: &mut std::mem::MaybeUninit<Foo>);
        pub(crate) fn __rust_thunk___ZN3FooC1ERKS_(
            __this: &mut std::mem::MaybeUninit<Foo>,
            __param_0: *const Foo,
        );
        pub(crate) fn __rust_thunk___Z3foov();
        pub(crate) fn __rust_thunk___ZN3BarC1Ev(__this: &mut std::mem::MaybeUninit<Bar>);
        pub(crate) fn __rust_thunk___ZN3BarC1ERKS_(
            __this: &mut std::mem::MaybeUninit<Bar>,
            __param_0: *const Bar,
        );
        pub(crate) fn __rust_thunk___ZN13HasNoCommentsC1Ev(
            __this: &mut std::mem::MaybeUninit<HasNoComments>,
        );
        pub(crate) fn __rust_thunk___ZN13HasNoCommentsC1ERKS_(
            __this: &mut std::mem::MaybeUninit<HasNoComments>,
            __param_0: *const HasNoComments,
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
