// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:overloads_cc

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![no_std]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

// Error while generating bindings for function 'Overload':
// Cannot generate bindings for overloaded function

// Error while generating bindings for function 'Overload':
// Cannot generate bindings for overloaded function

/// Both Overload2() overloads should be generated, because one should be
/// renamed.
#[inline(always)]
pub fn Overload2() {
    unsafe { crate::detail::__rust_thunk___Z9Overload2v() }
}

#[inline(always)]
pub fn RenamedOverload2(__param_0: ::core::ffi::c_int) {
    unsafe { crate::detail::__rust_thunk___Z9Overlaod2i(__param_0) }
}

// Error while generating bindings for function 'UncallableOverload':
// Cannot generate bindings for overloaded function

// Error while generating bindings for function 'UncallableOverload':
// Cannot generate bindings for overloaded function

// Error while generating bindings for class 'Sizeof':
// Class templates are not supported yet

// Error while generating bindings for function 'UncallableOverload':
// Function templates are not supported yet

#[inline(always)]
pub fn AlsoTemplateOverload() {
    unsafe { crate::detail::__rust_thunk___Z20AlsoTemplateOverloadv() }
}

// Error while generating bindings for function 'AlsoTemplateOverload':
// Function templates are not supported yet

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

impl Foo {
    #[inline(always)]
    pub unsafe fn BarBridgedInt(
        __this: *mut Self,
        mut __param_0: crate::__CcTemplateInst6SizeofIiE,
    ) {
        crate::detail::__rust_thunk___ZN3Foo3BarE6SizeofIiE(__this, &mut __param_0)
    }
}

impl Foo {
    #[inline(always)]
    pub unsafe fn BarBridgedFloat(
        __this: *mut Self,
        mut __param_0: crate::__CcTemplateInst6SizeofIfE,
    ) {
        crate::detail::__rust_thunk___ZN3Foo3BarE6SizeofIfE(__this, &mut __param_0)
    }
}

impl Foo {
    #[inline(always)]
    pub unsafe fn Bar(__this: *mut Self, __param_0: ::core::ffi::c_int) {
        crate::detail::__rust_thunk___ZN3Foo3BarEi(__this, __param_0)
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=Sizeof < float >
pub struct __CcTemplateInst6SizeofIfE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for __CcTemplateInst6SizeofIfE {}
impl !Sync for __CcTemplateInst6SizeofIfE {}
forward_declare::unsafe_define!(
    forward_declare::symbol!("Sizeof < float >"),
    crate::__CcTemplateInst6SizeofIfE
);

impl Default for __CcTemplateInst6SizeofIfE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN6SizeofIfEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3aoverloads_5fcc(&raw mut tmp as*mut _);
            tmp.assume_init()
        }
    }
}

// Error while generating bindings for global variable 'Sizeof<float>::size':
// static data members are not supported

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=Sizeof < int >
pub struct __CcTemplateInst6SizeofIiE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for __CcTemplateInst6SizeofIiE {}
impl !Sync for __CcTemplateInst6SizeofIiE {}
forward_declare::unsafe_define!(
    forward_declare::symbol!("Sizeof < int >"),
    crate::__CcTemplateInst6SizeofIiE
);

impl Default for __CcTemplateInst6SizeofIiE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN6SizeofIiEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3aoverloads_5fcc(&raw mut tmp as*mut _);
            tmp.assume_init()
        }
    }
}

// Error while generating bindings for global variable 'Sizeof<int>::size':
// static data members are not supported

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        #[link_name = "_Z9Overload2v"]
        pub(crate) unsafe fn __rust_thunk___Z9Overload2v();
        #[link_name = "_Z9Overlaod2i"]
        pub(crate) unsafe fn __rust_thunk___Z9Overlaod2i(__param_0: ::core::ffi::c_int);
        pub(crate) unsafe fn __rust_thunk___Z20AlsoTemplateOverloadv();
        pub(crate) unsafe fn __rust_thunk___ZN3FooC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN3Foo3BarE6SizeofIiE(
            __this: *mut crate::Foo,
            __param_0: &mut crate::__CcTemplateInst6SizeofIiE,
        );
        pub(crate) unsafe fn __rust_thunk___ZN3Foo3BarE6SizeofIfE(
            __this: *mut crate::Foo,
            __param_0: &mut crate::__CcTemplateInst6SizeofIfE,
        );
        #[link_name = "_ZN3Foo3BarEi"]
        pub(crate) unsafe fn __rust_thunk___ZN3Foo3BarEi(
            __this: *mut crate::Foo,
            __param_0: ::core::ffi::c_int,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6SizeofIfEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3aoverloads_5fcc(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6SizeofIiEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3aoverloads_5fcc(
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
