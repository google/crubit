// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:overloads_cc

#![rustfmt::skip]
#![feature(cfi_encoding, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![allow(deprecated)]
#![allow(unknown_lints, suspicious_runtime_symbol_definitions)]
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
#[cfi_encoding = "3Foo"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=Foo
///CRUBIT_ANNOTATE: cpp_move_constructible=
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
    #[inline(always)]
    pub fn BarBridgedInt<'__this>(
        &'__this mut self,
        mut __param_0: crate::__CcTemplateInst6SizeofIiE,
    ) {
        unsafe { self::foo::BarBridgedInt(self, __param_0) }
    }
    #[inline(always)]
    pub fn BarBridgedFloat<'__this>(
        &'__this mut self,
        mut __param_0: crate::__CcTemplateInst6SizeofIfE,
    ) {
        unsafe { self::foo::BarBridgedFloat(self, __param_0) }
    }
    #[inline(always)]
    pub fn Bar<'__this>(&'__this mut self, __param_0: ::ffi_11::c_int) {
        unsafe { self::foo::Bar(self, __param_0) }
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

pub mod foo {
    #[inline(always)]
    pub(crate) fn BarBridgedInt<'__this>(
        __this: &'__this mut crate::Foo,
        mut __param_0: crate::__CcTemplateInst6SizeofIiE,
    ) {
        unsafe { crate::detail::__rust_thunk___ZN3Foo3BarE6SizeofIiE(__this, &mut __param_0) }
    }
    #[inline(always)]
    pub(crate) fn BarBridgedFloat<'__this>(
        __this: &'__this mut crate::Foo,
        mut __param_0: crate::__CcTemplateInst6SizeofIfE,
    ) {
        unsafe { crate::detail::__rust_thunk___ZN3Foo3BarE6SizeofIfE(__this, &mut __param_0) }
    }
    #[inline(always)]
    pub(crate) fn Bar<'__this>(__this: &'__this mut crate::Foo, __param_0: ::ffi_11::c_int) {
        unsafe { crate::detail::__rust_thunk___ZN3Foo3BarEi(__this, __param_0) }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInst6SizeofIfE"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=Sizeof < float >
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInst6SizeofIfE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for __CcTemplateInst6SizeofIfE {}
impl !Sync for __CcTemplateInst6SizeofIfE {}

impl Default for __CcTemplateInst6SizeofIfE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__4304ae3f__ZN6SizeofIfEC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

// error: global variable `Sizeof<float>::size` could not be bound
//   static data members are not supported

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInst6SizeofIiE"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=Sizeof < int >
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInst6SizeofIiE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for __CcTemplateInst6SizeofIiE {}
impl !Sync for __CcTemplateInst6SizeofIiE {}

impl Default for __CcTemplateInst6SizeofIiE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__4304ae3f__ZN6SizeofIiEC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

// error: global variable `Sizeof<int>::size` could not be bound
//   static data members are not supported

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
        pub(crate) unsafe fn __rust_thunk___ZN3Foo3BarE6SizeofIiE<'__this>(
            __this: &'__this mut crate::Foo,
            __param_0: &mut crate::__CcTemplateInst6SizeofIiE,
        );
        pub(crate) unsafe fn __rust_thunk___ZN3Foo3BarE6SizeofIfE<'__this>(
            __this: &'__this mut crate::Foo,
            __param_0: &mut crate::__CcTemplateInst6SizeofIfE,
        );
        #[link_name = "_ZN3Foo3BarEi"]
        pub(crate) unsafe fn __rust_thunk___ZN3Foo3BarEi<'__this>(
            __this: &'__this mut crate::Foo,
            __param_0: ::ffi_11::c_int,
        );
        pub(crate) unsafe fn __rust_thunk__4304ae3f__ZN6SizeofIfEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__4304ae3f__ZN6SizeofIiEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::Foo>() == 1);
    assert!(::core::mem::align_of::<crate::Foo>() == 1);
    static_assertions::assert_impl_all!(crate::Foo: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::Foo: Drop);

    assert!(::core::mem::size_of::<crate::__CcTemplateInst6SizeofIfE>() == 1);
    assert!(::core::mem::align_of::<crate::__CcTemplateInst6SizeofIfE>() == 1);
    static_assertions::assert_impl_all!(crate::__CcTemplateInst6SizeofIfE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInst6SizeofIfE: Drop);

    assert!(::core::mem::size_of::<crate::__CcTemplateInst6SizeofIiE>() == 1);
    assert!(::core::mem::align_of::<crate::__CcTemplateInst6SizeofIiE>() == 1);
    static_assertions::assert_impl_all!(crate::__CcTemplateInst6SizeofIiE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInst6SizeofIiE: Drop);
};
