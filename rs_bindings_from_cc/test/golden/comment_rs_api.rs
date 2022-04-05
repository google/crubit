// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for C++ target
// //rs_bindings_from_cc/test/golden:comment_cc
#![rustfmt::skip]
#![feature(const_ptr_offset_from, custom_inner_attributes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use ::std as rust_std;
use memoffset_unstable_const::offset_of;
use static_assertions::{assert_impl_all, assert_not_impl_all};

pub type __builtin_ms_va_list = *mut u8;

// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

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
        let mut tmp = rust_std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN3FooC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b> From<ctor::RvalueReference<'b, Foo>> for Foo {
    #[inline(always)]
    fn from(__param_0: ctor::RvalueReference<'b, Foo>) -> Self {
        let mut tmp = rust_std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN3FooC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

// rs_bindings_from_cc/test/golden/comment.h;l=17
// Error while generating bindings for item 'Foo::operator=':
// Bindings for this kind of operator are not supported

// rs_bindings_from_cc/test/golden/comment.h;l=17
// Error while generating bindings for item 'Foo::operator=':
// Bindings for this kind of operator are not supported

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
        let mut tmp = rust_std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN3BarC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b> From<ctor::RvalueReference<'b, Bar>> for Bar {
    #[inline(always)]
    fn from(__param_0: ctor::RvalueReference<'b, Bar>) -> Self {
        let mut tmp = rust_std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN3BarC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

// rs_bindings_from_cc/test/golden/comment.h;l=43
// Error while generating bindings for item 'Bar::operator=':
// Bindings for this kind of operator are not supported

// rs_bindings_from_cc/test/golden/comment.h;l=43
// Error while generating bindings for item 'Bar::operator=':
// Bindings for this kind of operator are not supported

/// d
#[derive(Clone, Copy)]
#[repr(C)]
pub struct HasNoComments {
    pub i: i32,
}

impl Default for HasNoComments {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = rust_std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN13HasNoCommentsC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b> From<ctor::RvalueReference<'b, HasNoComments>> for HasNoComments {
    #[inline(always)]
    fn from(__param_0: ctor::RvalueReference<'b, HasNoComments>) -> Self {
        let mut tmp = rust_std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN13HasNoCommentsC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

// rs_bindings_from_cc/test/golden/comment.h;l=49
// Error while generating bindings for item 'HasNoComments::operator=':
// Bindings for this kind of operator are not supported

// rs_bindings_from_cc/test/golden/comment.h;l=49
// Error while generating bindings for item 'HasNoComments::operator=':
// Bindings for this kind of operator are not supported

// e

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_COMMENT_H_

mod detail {
    #[allow(unused_imports)]
    use super::*;
    extern "C" {
        pub(crate) fn __rust_thunk___ZN3FooC1Ev<'a>(
            __this: &'a mut rust_std::mem::MaybeUninit<Foo>,
        );
        pub(crate) fn __rust_thunk___ZN3FooC1EOS_<'a, 'b>(
            __this: &'a mut rust_std::mem::MaybeUninit<Foo>,
            __param_0: ctor::RvalueReference<'b, Foo>,
        );
        pub(crate) fn __rust_thunk___Z3foov();
        pub(crate) fn __rust_thunk___ZN3BarC1Ev<'a>(
            __this: &'a mut rust_std::mem::MaybeUninit<Bar>,
        );
        pub(crate) fn __rust_thunk___ZN3BarC1EOS_<'a, 'b>(
            __this: &'a mut rust_std::mem::MaybeUninit<Bar>,
            __param_0: ctor::RvalueReference<'b, Bar>,
        );
        pub(crate) fn __rust_thunk___ZN13HasNoCommentsC1Ev<'a>(
            __this: &'a mut rust_std::mem::MaybeUninit<HasNoComments>,
        );
        pub(crate) fn __rust_thunk___ZN13HasNoCommentsC1EOS_<'a, 'b>(
            __this: &'a mut rust_std::mem::MaybeUninit<HasNoComments>,
            __param_0: ctor::RvalueReference<'b, HasNoComments>,
        );
    }
}

const _: () = assert!(rust_std::mem::size_of::<Option<&i32>>() == rust_std::mem::size_of::<&i32>());

const _: () = assert!(rust_std::mem::size_of::<Foo>() == 8usize);
const _: () = assert!(rust_std::mem::align_of::<Foo>() == 4usize);
const _: () = {
    assert_impl_all!(Foo: Clone);
};
const _: () = {
    assert_impl_all!(Foo: Copy);
};
const _: () = {
    assert_not_impl_all!(Foo: Drop);
};
const _: () = assert!(offset_of!(Foo, i) * 8 == 0usize);
const _: () = assert!(offset_of!(Foo, j) * 8 == 32usize);

const _: () = assert!(rust_std::mem::size_of::<Bar>() == 4usize);
const _: () = assert!(rust_std::mem::align_of::<Bar>() == 4usize);
const _: () = {
    assert_impl_all!(Bar: Clone);
};
const _: () = {
    assert_impl_all!(Bar: Copy);
};
const _: () = {
    assert_not_impl_all!(Bar: Drop);
};
const _: () = assert!(offset_of!(Bar, i) * 8 == 0usize);

const _: () = assert!(rust_std::mem::size_of::<HasNoComments>() == 4usize);
const _: () = assert!(rust_std::mem::align_of::<HasNoComments>() == 4usize);
const _: () = {
    assert_impl_all!(HasNoComments: Clone);
};
const _: () = {
    assert_impl_all!(HasNoComments: Copy);
};
const _: () = {
    assert_not_impl_all!(HasNoComments: Drop);
};
const _: () = assert!(offset_of!(HasNoComments, i) * 8 == 0usize);
