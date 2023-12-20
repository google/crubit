// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:comment_cc
// Features: experimental, extern_c, supported

#![rustfmt::skip]
#![feature(custom_inner_attributes, negative_impls, register_tool)]
#![allow(stable_features)]
#![no_std]
#![register_tool(__crubit)]
#![allow(improper_ctypes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![deny(warnings)]

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
#[__crubit::annotate(cc_type = "Foo")]
pub struct Foo {
    /// A field
    pub i: ::core::ffi::c_int,
    /// Another field
    pub j: ::core::ffi::c_int,
}
impl !Send for Foo {}
impl !Sync for Foo {}
forward_declare::unsafe_define!(forward_declare::symbol!("Foo"), crate::Foo);

impl Default for Foo {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN3FooC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b> From<::ctor::RvalueReference<'b, Self>> for Foo {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN3FooC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

impl<'b> ::ctor::UnpinAssign<&'b Self> for Foo {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN3FooaSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::UnpinAssign<::ctor::RvalueReference<'b, Self>> for Foo {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: ::ctor::RvalueReference<'b, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN3FooaSEOS_(self, __param_0);
        }
    }
}

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
#[__crubit::annotate(cc_type = "Bar")]
pub struct Bar {
    pub i: ::core::ffi::c_int,
}
impl !Send for Bar {}
impl !Sync for Bar {}
forward_declare::unsafe_define!(forward_declare::symbol!("Bar"), crate::Bar);

impl Default for Bar {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN3BarC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b> From<::ctor::RvalueReference<'b, Self>> for Bar {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN3BarC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

impl<'b> ::ctor::UnpinAssign<&'b Self> for Bar {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN3BaraSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::UnpinAssign<::ctor::RvalueReference<'b, Self>> for Bar {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: ::ctor::RvalueReference<'b, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN3BaraSEOS_(self, __param_0);
        }
    }
}

/// d
#[derive(Clone, Copy)]
#[repr(C)]
#[__crubit::annotate(cc_type = "HasNoComments")]
pub struct HasNoComments {
    pub i: ::core::ffi::c_int,
}
impl !Send for HasNoComments {}
impl !Sync for HasNoComments {}
forward_declare::unsafe_define!(forward_declare::symbol!("HasNoComments"), crate::HasNoComments);

impl Default for HasNoComments {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN13HasNoCommentsC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b> From<::ctor::RvalueReference<'b, Self>> for HasNoComments {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN13HasNoCommentsC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

impl<'b> ::ctor::UnpinAssign<&'b Self> for HasNoComments {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN13HasNoCommentsaSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::UnpinAssign<::ctor::RvalueReference<'b, Self>> for HasNoComments {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: ::ctor::RvalueReference<'b, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN13HasNoCommentsaSEOS_(self, __param_0);
        }
    }
}

// e

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_COMMENT_H_

mod detail {
    #[allow(unused_imports)]
    use super::*;
    extern "C" {
        pub(crate) fn __rust_thunk___ZN3FooC1Ev<'a>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::Foo>,
        );
        pub(crate) fn __rust_thunk___ZN3FooC1EOS_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::Foo>,
            __param_0: ::ctor::RvalueReference<'b, crate::Foo>,
        );
        pub(crate) fn __rust_thunk___ZN3FooaSERKS_<'a, 'b>(
            __this: &'a mut crate::Foo,
            __param_0: &'b crate::Foo,
        ) -> &'a mut crate::Foo;
        pub(crate) fn __rust_thunk___ZN3FooaSEOS_<'a, 'b>(
            __this: &'a mut crate::Foo,
            __param_0: ::ctor::RvalueReference<'b, crate::Foo>,
        ) -> &'a mut crate::Foo;
        pub(crate) fn __rust_thunk___Z3foov();
        pub(crate) fn __rust_thunk___ZN3BarC1Ev<'a>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::Bar>,
        );
        pub(crate) fn __rust_thunk___ZN3BarC1EOS_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::Bar>,
            __param_0: ::ctor::RvalueReference<'b, crate::Bar>,
        );
        pub(crate) fn __rust_thunk___ZN3BaraSERKS_<'a, 'b>(
            __this: &'a mut crate::Bar,
            __param_0: &'b crate::Bar,
        ) -> &'a mut crate::Bar;
        pub(crate) fn __rust_thunk___ZN3BaraSEOS_<'a, 'b>(
            __this: &'a mut crate::Bar,
            __param_0: ::ctor::RvalueReference<'b, crate::Bar>,
        ) -> &'a mut crate::Bar;
        pub(crate) fn __rust_thunk___ZN13HasNoCommentsC1Ev<'a>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::HasNoComments>,
        );
        pub(crate) fn __rust_thunk___ZN13HasNoCommentsC1EOS_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::HasNoComments>,
            __param_0: ::ctor::RvalueReference<'b, crate::HasNoComments>,
        );
        pub(crate) fn __rust_thunk___ZN13HasNoCommentsaSERKS_<'a, 'b>(
            __this: &'a mut crate::HasNoComments,
            __param_0: &'b crate::HasNoComments,
        ) -> &'a mut crate::HasNoComments;
        pub(crate) fn __rust_thunk___ZN13HasNoCommentsaSEOS_<'a, 'b>(
            __this: &'a mut crate::HasNoComments,
            __param_0: ::ctor::RvalueReference<'b, crate::HasNoComments>,
        ) -> &'a mut crate::HasNoComments;
    }
}

const _: () = assert!(::core::mem::size_of::<Option<&i32>>() == ::core::mem::size_of::<&i32>());

const _: () = assert!(::core::mem::size_of::<crate::Foo>() == 8);
const _: () = assert!(::core::mem::align_of::<crate::Foo>() == 4);
const _: () = {
    static_assertions::assert_impl_all!(crate::Foo:Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::Foo:Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::Foo:Drop);
};
const _: () = assert!(memoffset::offset_of!(crate::Foo, i) == 0);
const _: () = assert!(memoffset::offset_of!(crate::Foo, j) == 4);

const _: () = assert!(::core::mem::size_of::<crate::Bar>() == 4);
const _: () = assert!(::core::mem::align_of::<crate::Bar>() == 4);
const _: () = {
    static_assertions::assert_impl_all!(crate::Bar:Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::Bar:Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::Bar:Drop);
};
const _: () = assert!(memoffset::offset_of!(crate::Bar, i) == 0);

const _: () = assert!(::core::mem::size_of::<crate::HasNoComments>() == 4);
const _: () = assert!(::core::mem::align_of::<crate::HasNoComments>() == 4);
const _: () = {
    static_assertions::assert_impl_all!(crate::HasNoComments:Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::HasNoComments:Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::HasNoComments:Drop);
};
const _: () = assert!(memoffset::offset_of!(crate::HasNoComments, i) == 0);
