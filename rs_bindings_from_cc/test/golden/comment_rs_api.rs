// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for C++ target
// //rs_bindings_from_cc/test/golden:comment_cc
#![rustfmt::skip]
#![feature(const_ptr_offset_from, custom_inner_attributes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use ::std as rust_std;

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
forward_declare::unsafe_define!(forward_declare::symbol!("Foo"), crate::Foo);

impl Default for Foo {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = crate::rust_std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN3FooC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b> From<ctor::RvalueReference<'b, crate::Foo>> for Foo {
    #[inline(always)]
    fn from(__param_0: ctor::RvalueReference<'b, crate::Foo>) -> Self {
        let mut tmp = crate::rust_std::mem::MaybeUninit::<Self>::zeroed();
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

// TODO(rosica): This comment appears near fields of a struct, and
// is currently generated below the struct definiton on the Rust side.

// TODO(rosica): This comment appears between fields of a struct, and
// is currently generated below the struct definiton on the Rust side.

// TODO(rosica): This comment appears near fields of a struct, and
// is currently generated below the struct definiton on the Rust side.

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
forward_declare::unsafe_define!(forward_declare::symbol!("Bar"), crate::Bar);

impl Default for Bar {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = crate::rust_std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN3BarC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b> From<ctor::RvalueReference<'b, crate::Bar>> for Bar {
    #[inline(always)]
    fn from(__param_0: ctor::RvalueReference<'b, crate::Bar>) -> Self {
        let mut tmp = crate::rust_std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN3BarC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

// rs_bindings_from_cc/test/golden/comment.h;l=46
// Error while generating bindings for item 'Bar::operator=':
// Bindings for this kind of operator are not supported

// rs_bindings_from_cc/test/golden/comment.h;l=46
// Error while generating bindings for item 'Bar::operator=':
// Bindings for this kind of operator are not supported

/// d
#[derive(Clone, Copy)]
#[repr(C)]
pub struct HasNoComments {
    pub i: i32,
}
forward_declare::unsafe_define!(forward_declare::symbol!("HasNoComments"), crate::HasNoComments);

impl Default for HasNoComments {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = crate::rust_std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN13HasNoCommentsC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b> From<ctor::RvalueReference<'b, crate::HasNoComments>> for HasNoComments {
    #[inline(always)]
    fn from(__param_0: ctor::RvalueReference<'b, crate::HasNoComments>) -> Self {
        let mut tmp = crate::rust_std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN13HasNoCommentsC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

// rs_bindings_from_cc/test/golden/comment.h;l=52
// Error while generating bindings for item 'HasNoComments::operator=':
// Bindings for this kind of operator are not supported

// rs_bindings_from_cc/test/golden/comment.h;l=52
// Error while generating bindings for item 'HasNoComments::operator=':
// Bindings for this kind of operator are not supported

// e

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_COMMENT_H_

mod detail {
    #[allow(unused_imports)]
    use super::*;
    extern "C" {
        pub(crate) fn __rust_thunk___ZN3FooC1Ev<'a>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::Foo>,
        );
        pub(crate) fn __rust_thunk___ZN3FooC1EOS_<'a, 'b>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::Foo>,
            __param_0: ctor::RvalueReference<'b, crate::Foo>,
        );
        pub(crate) fn __rust_thunk___Z3foov();
        pub(crate) fn __rust_thunk___ZN3BarC1Ev<'a>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::Bar>,
        );
        pub(crate) fn __rust_thunk___ZN3BarC1EOS_<'a, 'b>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::Bar>,
            __param_0: ctor::RvalueReference<'b, crate::Bar>,
        );
        pub(crate) fn __rust_thunk___ZN13HasNoCommentsC1Ev<'a>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::HasNoComments>,
        );
        pub(crate) fn __rust_thunk___ZN13HasNoCommentsC1EOS_<'a, 'b>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::HasNoComments>,
            __param_0: ctor::RvalueReference<'b, crate::HasNoComments>,
        );
    }
}

const _: () = assert!(rust_std::mem::size_of::<Option<&i32>>() == rust_std::mem::size_of::<&i32>());

const _: () = assert!(rust_std::mem::size_of::<crate::Foo>() == 8usize);
const _: () = assert!(rust_std::mem::align_of::<crate::Foo>() == 4usize);
const _: () = {
    static_assertions::assert_impl_all!(crate::Foo: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::Foo: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_all!(crate::Foo: Drop);
};
const _: () = assert!(memoffset_unstable_const::offset_of!(crate::Foo, i) * 8 == 0usize);
const _: () = assert!(memoffset_unstable_const::offset_of!(crate::Foo, j) * 8 == 32usize);

const _: () = assert!(rust_std::mem::size_of::<crate::Bar>() == 4usize);
const _: () = assert!(rust_std::mem::align_of::<crate::Bar>() == 4usize);
const _: () = {
    static_assertions::assert_impl_all!(crate::Bar: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::Bar: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_all!(crate::Bar: Drop);
};
const _: () = assert!(memoffset_unstable_const::offset_of!(crate::Bar, i) * 8 == 0usize);

const _: () = assert!(rust_std::mem::size_of::<crate::HasNoComments>() == 4usize);
const _: () = assert!(rust_std::mem::align_of::<crate::HasNoComments>() == 4usize);
const _: () = {
    static_assertions::assert_impl_all!(crate::HasNoComments: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::HasNoComments: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_all!(crate::HasNoComments: Drop);
};
const _: () = assert!(memoffset_unstable_const::offset_of!(crate::HasNoComments, i) * 8 == 0usize);
