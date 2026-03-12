// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:overloads_cc

#![rustfmt::skip]
#![feature(custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![deny(warnings)]

// error: function `Overload` could not be bound
//   Cannot generate bindings for overloaded function

// error: function `Overload` could not be bound
//   Cannot generate bindings for overloaded function

/// Both Overload2() overloads should be generated, because one should be
/// renamed.
#[inline(always)]
pub fn Overload2() {
    unsafe { crate::detail::__rust_thunk___Z9Overload2v() }
}

#[inline(always)]
pub fn RenamedOverload2(__param_0: ::ffi_11::c_int) {
    unsafe { crate::detail::__rust_thunk___Z9Overlaod2i(__param_0) }
}

// error: function `UncallableOverload` could not be bound
//   Cannot generate bindings for overloaded function

// error: function `UncallableOverload` could not be bound
//   Cannot generate bindings for overloaded function

// error: class `Sizeof` could not be bound
//   Class templates are not yet supported

// error: function `UncallableOverload` could not be bound
//   Function templates are not yet supported

#[inline(always)]
pub fn AlsoTemplateOverload() {
    unsafe { crate::detail::__rust_thunk___Z20AlsoTemplateOverloadv() }
}

// error: function `AlsoTemplateOverload` could not be bound
//   Function templates are not yet supported

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=Foo
pub struct Foo {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for Foo {}
impl !Sync for Foo {}
unsafe impl ::cxx::ExternType for Foo {
    type Id = ::cxx::type_id!("Foo");
    type Kind = ::cxx::kind::Trivial;
}
impl Foo {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub unsafe fn Bar(__this: *mut Self, __param_0: ::ffi_11::c_int) {
        crate::detail::__rust_thunk___ZN3Foo3BarEi(__this, __param_0)
    }
}

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

// error: function `Foo::Bar` could not be bound
//   Unsupported parameter #1 (__param_0): template instantiation is not yet supported
//   template instantiation is not yet supported

// error: function `Foo::Bar` could not be bound
//   Unsupported parameter #1 (__param_0): template instantiation is not yet supported
//   template instantiation is not yet supported

// error: struct `Sizeof<float>` could not be bound
//   template instantiation is not yet supported
//   template instantiation is not yet supported

// error: struct `Sizeof<int>` could not be bound
//   template instantiation is not yet supported
//   template instantiation is not yet supported

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        #[link_name = "_Z9Overload2v"]
        pub(crate) unsafe fn __rust_thunk___Z9Overload2v();
        #[link_name = "_Z9Overlaod2i"]
        pub(crate) unsafe fn __rust_thunk___Z9Overlaod2i(__param_0: ::ffi_11::c_int);
        pub(crate) unsafe fn __rust_thunk___Z20AlsoTemplateOverloadv();
        pub(crate) unsafe fn __rust_thunk___ZN3FooC1Ev(__this: *mut ::core::ffi::c_void);
        #[link_name = "_ZN3Foo3BarEi"]
        pub(crate) unsafe fn __rust_thunk___ZN3Foo3BarEi(
            __this: *mut crate::Foo,
            __param_0: ::ffi_11::c_int,
        );
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::Foo>() == 1);
    assert!(::core::mem::align_of::<crate::Foo>() == 1);
    static_assertions::assert_impl_all!(crate::Foo: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::Foo: Drop);
};
