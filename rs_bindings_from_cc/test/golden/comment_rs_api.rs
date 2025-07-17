// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:comment_cc

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![no_std]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

// File comment

// TODO(b/202933018): Re-enable once namespaces are supported
// namespace ns {
// a

/// Foo
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=Foo
pub struct Foo {
    /// A field
    pub i: ::core::ffi::c_int,
    /// Another field
    pub j: ::core::ffi::c_int,
}
impl !Send for Foo {}
impl !Sync for Foo {}
unsafe impl ::cxx::ExternType for Foo {
    type Id = ::cxx::type_id!("Foo");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(forward_declare::symbol!("Foo"), crate::Foo);

impl Default for Foo {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN3FooC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
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
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=Bar
pub struct Bar {
    pub i: ::core::ffi::c_int,
}
impl !Send for Bar {}
impl !Sync for Bar {}
unsafe impl ::cxx::ExternType for Bar {
    type Id = ::cxx::type_id!("Bar");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(forward_declare::symbol!("Bar"), crate::Bar);

impl Default for Bar {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN3BarC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

/// d
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=HasNoComments
pub struct HasNoComments {
    pub i: ::core::ffi::c_int,
}
impl !Send for HasNoComments {}
impl !Sync for HasNoComments {}
unsafe impl ::cxx::ExternType for HasNoComments {
    type Id = ::cxx::type_id!("HasNoComments");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(forward_declare::symbol!("HasNoComments"), crate::HasNoComments);

impl Default for HasNoComments {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN13HasNoCommentsC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

// e

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN3FooC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___Z3foov();
        pub(crate) unsafe fn __rust_thunk___ZN3BarC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN13HasNoCommentsC1Ev(__this: *mut ::core::ffi::c_void);
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::Foo>() == 8);
    assert!(::core::mem::align_of::<crate::Foo>() == 4);
    static_assertions::assert_impl_all!(crate::Foo: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::Foo: Drop);
    assert!(::core::mem::offset_of!(crate::Foo, i) == 0);
    assert!(::core::mem::offset_of!(crate::Foo, j) == 4);
    assert!(::core::mem::size_of::<crate::Bar>() == 4);
    assert!(::core::mem::align_of::<crate::Bar>() == 4);
    static_assertions::assert_impl_all!(crate::Bar: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::Bar: Drop);
    assert!(::core::mem::offset_of!(crate::Bar, i) == 0);
    assert!(::core::mem::size_of::<crate::HasNoComments>() == 4);
    assert!(::core::mem::align_of::<crate::HasNoComments>() == 4);
    static_assertions::assert_impl_all!(crate::HasNoComments: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::HasNoComments: Drop);
    assert!(::core::mem::offset_of!(crate::HasNoComments, i) == 0);
};
