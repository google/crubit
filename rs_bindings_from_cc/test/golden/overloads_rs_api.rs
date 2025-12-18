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
impl Foo {
    #[inline(always)]
    pub unsafe fn Bar(__this: *mut Self, __param_0: ::core::ffi::c_int) {
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

// Error while generating bindings for function 'Foo::BarBridgedInt':
// Can't generate bindings for Foo::BarBridgedInt, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:overloads_cc needs [//features:wrapper] for Foo::BarBridgedInt (the type of __param_0 (parameter #1): error: Can't generate bindings for Sizeof<int>, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:overloads_cc needs [//features:wrapper] for Sizeof<int> (crate::__CcTemplateInst6SizeofIiE is a template instantiation)
// //rs_bindings_from_cc/test/golden:overloads_cc needs [//features:wrapper] for Sizeof<int> (crate::__CcTemplateInst6SizeofIiE is a template instantiation))

// Error while generating bindings for function 'Foo::BarBridgedFloat':
// Can't generate bindings for Foo::BarBridgedFloat, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:overloads_cc needs [//features:wrapper] for Foo::BarBridgedFloat (the type of __param_0 (parameter #1): error: Can't generate bindings for Sizeof<float>, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:overloads_cc needs [//features:wrapper] for Sizeof<float> (crate::__CcTemplateInst6SizeofIfE is a template instantiation)
// //rs_bindings_from_cc/test/golden:overloads_cc needs [//features:wrapper] for Sizeof<float> (crate::__CcTemplateInst6SizeofIfE is a template instantiation))

// Error while generating bindings for struct 'Sizeof<float>':
// Can't generate bindings for Sizeof<float>, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:overloads_cc needs [//features:wrapper] for Sizeof<float> (crate::__CcTemplateInst6SizeofIfE is a template instantiation)
// //rs_bindings_from_cc/test/golden:overloads_cc needs [//features:wrapper] for Sizeof<float> (crate::__CcTemplateInst6SizeofIfE is a template instantiation)

// Error while generating bindings for struct 'Sizeof<int>':
// Can't generate bindings for Sizeof<int>, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:overloads_cc needs [//features:wrapper] for Sizeof<int> (crate::__CcTemplateInst6SizeofIiE is a template instantiation)
// //rs_bindings_from_cc/test/golden:overloads_cc needs [//features:wrapper] for Sizeof<int> (crate::__CcTemplateInst6SizeofIiE is a template instantiation)

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
        #[link_name = "_ZN3Foo3BarEi"]
        pub(crate) unsafe fn __rust_thunk___ZN3Foo3BarEi(
            __this: *mut crate::Foo,
            __param_0: ::core::ffi::c_int,
        );
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::Foo>() == 1);
    assert!(::core::mem::align_of::<crate::Foo>() == 1);
    static_assertions::assert_impl_all!(crate::Foo: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::Foo: Drop);
};
