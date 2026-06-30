// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/struct/methods:methods

#![rustfmt::skip]
#![feature(custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![allow(deprecated)]
#![deny(warnings)]

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=SomeClass
pub struct SomeClass {
    pub int_var: ::ffi_11::c_int,
}
impl !Send for SomeClass {}
impl !Sync for SomeClass {}
unsafe impl ::cxx::ExternType for SomeClass {
    type Id = ::cxx::type_id!("SomeClass");
    type Kind = ::cxx::kind::Trivial;
}
impl SomeClass {
    #[inline(always)]
    pub fn static_factory_method(int_var_initial_value: ::ffi_11::c_int) -> crate::SomeClass {
        unsafe { self::some_class::static_factory_method(int_var_initial_value) }
    }
    #[inline(always)]
    pub fn static_method_that_multiplies_its_args(
        x: ::ffi_11::c_int,
        y: ::ffi_11::c_int,
    ) -> ::ffi_11::c_int {
        unsafe { self::some_class::static_method_that_multiplies_its_args(x, y) }
    }
    /// Using an `inline` method forces generation of a C++ thunk in
    /// methods_rs_api_impl.cc (helping add test coverage for such thunks).
    #[inline(always)]
    pub fn static_inline_method(arg: ::ffi_11::c_int) -> ::ffi_11::c_int {
        unsafe { self::some_class::static_inline_method(arg) }
    }
}

impl Default for SomeClass {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN9SomeClassC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

pub mod some_class {
    #[inline(always)]
    pub(crate) fn static_factory_method(
        int_var_initial_value: ::ffi_11::c_int,
    ) -> crate::SomeClass {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<crate::SomeClass>::uninit();
            crate::detail::__rust_thunk___ZN9SomeClass21static_factory_methodEi(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                int_var_initial_value,
            );
            __crubit_return.assume_init()
        }
    }
    #[inline(always)]
    pub(crate) fn static_method_that_multiplies_its_args(
        x: ::ffi_11::c_int,
        y: ::ffi_11::c_int,
    ) -> ::ffi_11::c_int {
        unsafe {
            crate::detail::__rust_thunk___ZN9SomeClass38static_method_that_multiplies_its_argsEii(
                x, y,
            )
        }
    }
    /// Using an `inline` method forces generation of a C++ thunk in
    /// methods_rs_api_impl.cc (helping add test coverage for such thunks).
    #[inline(always)]
    pub(crate) fn static_inline_method(arg: ::ffi_11::c_int) -> ::ffi_11::c_int {
        unsafe { crate::detail::__rust_thunk___ZN9SomeClass20static_inline_methodEi(arg) }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=InstanceMethods
pub struct InstanceMethods {
    pub int_field: ::ffi_11::c_int,
}
impl !Send for InstanceMethods {}
impl !Sync for InstanceMethods {}
unsafe impl ::cxx::ExternType for InstanceMethods {
    type Id = ::cxx::type_id!("InstanceMethods");
    type Kind = ::cxx::kind::Trivial;
}
impl InstanceMethods {
    #[inline(always)]
    pub fn get_int_field(&self) -> ::ffi_11::c_int {
        unsafe { self::instance_methods::get_int_field(self) }
    }
    #[inline(always)]
    pub fn set_int_field(&mut self, new_value: ::ffi_11::c_int) {
        unsafe { self::instance_methods::set_int_field(self, new_value) }
    }
    #[inline(always)]
    pub fn inline_get_int_field(&self) -> ::ffi_11::c_int {
        unsafe { self::instance_methods::inline_get_int_field(self) }
    }
    #[inline(always)]
    pub fn inline_set_int_field(&mut self, new_value: ::ffi_11::c_int) {
        unsafe { self::instance_methods::inline_set_int_field(self, new_value) }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `input_ref`: raw pointer
    #[inline(always)]
    pub unsafe fn takes_and_returns_ref(
        &mut self,
        input_ref: *mut ::ffi_11::c_int,
    ) -> *mut ::ffi_11::c_int {
        unsafe { self::instance_methods::takes_and_returns_ref(self, input_ref) }
    }
    #[inline(always)]
    pub fn ref_qualified(&mut self) {
        unsafe { self::instance_methods::ref_qualified(self) }
    }
    #[inline(always)]
    pub fn const_ref_qualified(&self) {
        unsafe { self::instance_methods::const_ref_qualified(self) }
    }
    #[inline(always)]
    pub fn rvalue_qualified(&mut self) {
        unsafe { self::instance_methods::rvalue_qualified(self) }
    }
}

impl Default for InstanceMethods {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN15InstanceMethodsC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

pub mod instance_methods {
    #[inline(always)]
    pub(crate) fn get_int_field(__this: &crate::InstanceMethods) -> ::ffi_11::c_int {
        unsafe { crate::detail::__rust_thunk___ZNK15InstanceMethods13get_int_fieldEv(__this) }
    }
    #[inline(always)]
    pub(crate) fn set_int_field(__this: &mut crate::InstanceMethods, new_value: ::ffi_11::c_int) {
        unsafe {
            crate::detail::__rust_thunk___ZN15InstanceMethods13set_int_fieldEi(__this, new_value)
        }
    }
    #[inline(always)]
    pub(crate) fn inline_get_int_field(__this: &crate::InstanceMethods) -> ::ffi_11::c_int {
        unsafe {
            crate::detail::__rust_thunk___ZNK15InstanceMethods20inline_get_int_fieldEv(__this)
        }
    }
    #[inline(always)]
    pub(crate) fn inline_set_int_field(
        __this: &mut crate::InstanceMethods,
        new_value: ::ffi_11::c_int,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN15InstanceMethods20inline_set_int_fieldEi(
                __this, new_value,
            )
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `input_ref`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn takes_and_returns_ref(
        __this: &mut crate::InstanceMethods,
        input_ref: *mut ::ffi_11::c_int,
    ) -> *mut ::ffi_11::c_int {
        unsafe {
            crate::detail::__rust_thunk___ZN15InstanceMethods21takes_and_returns_refERi(
                __this, input_ref,
            )
        }
    }
    #[inline(always)]
    pub(crate) fn ref_qualified(__this: &mut crate::InstanceMethods) {
        unsafe { crate::detail::__rust_thunk___ZNR15InstanceMethods13ref_qualifiedEv(__this) }
    }
    #[inline(always)]
    pub(crate) fn const_ref_qualified(__this: &crate::InstanceMethods) {
        unsafe {
            crate::detail::__rust_thunk___ZNKR15InstanceMethods19const_ref_qualifiedEv(__this)
        }
    }
    #[inline(always)]
    pub(crate) fn rvalue_qualified(__this: &mut crate::InstanceMethods) {
        unsafe { crate::detail::__rust_thunk___ZNO15InstanceMethods16rvalue_qualifiedEv(__this) }
    }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN9SomeClassC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN9SomeClass21static_factory_methodEi(
            __return: *mut ::core::ffi::c_void,
            int_var_initial_value: ::ffi_11::c_int,
        );
        #[link_name = "_ZN9SomeClass38static_method_that_multiplies_its_argsEii"]
        pub(crate) unsafe fn __rust_thunk___ZN9SomeClass38static_method_that_multiplies_its_argsEii(
            x: ::ffi_11::c_int,
            y: ::ffi_11::c_int,
        ) -> ::ffi_11::c_int;
        pub(crate) unsafe fn __rust_thunk___ZN9SomeClass20static_inline_methodEi(
            arg: ::ffi_11::c_int,
        ) -> ::ffi_11::c_int;
        pub(crate) unsafe fn __rust_thunk___ZN15InstanceMethodsC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        #[link_name = "_ZNK15InstanceMethods13get_int_fieldEv"]
        pub(crate) unsafe fn __rust_thunk___ZNK15InstanceMethods13get_int_fieldEv(
            __this: &crate::InstanceMethods,
        ) -> ::ffi_11::c_int;
        #[link_name = "_ZN15InstanceMethods13set_int_fieldEi"]
        pub(crate) unsafe fn __rust_thunk___ZN15InstanceMethods13set_int_fieldEi(
            __this: &mut crate::InstanceMethods,
            new_value: ::ffi_11::c_int,
        );
        pub(crate) unsafe fn __rust_thunk___ZNK15InstanceMethods20inline_get_int_fieldEv(
            __this: &crate::InstanceMethods,
        ) -> ::ffi_11::c_int;
        pub(crate) unsafe fn __rust_thunk___ZN15InstanceMethods20inline_set_int_fieldEi(
            __this: &mut crate::InstanceMethods,
            new_value: ::ffi_11::c_int,
        );
        pub(crate) unsafe fn __rust_thunk___ZN15InstanceMethods21takes_and_returns_refERi(
            __this: &mut crate::InstanceMethods,
            input_ref: *mut ::ffi_11::c_int,
        ) -> *mut ::ffi_11::c_int;
        pub(crate) unsafe fn __rust_thunk___ZNR15InstanceMethods13ref_qualifiedEv(
            __this: &mut crate::InstanceMethods,
        );
        pub(crate) unsafe fn __rust_thunk___ZNKR15InstanceMethods19const_ref_qualifiedEv(
            __this: &crate::InstanceMethods,
        );
        pub(crate) unsafe fn __rust_thunk___ZNO15InstanceMethods16rvalue_qualifiedEv(
            __this: &mut crate::InstanceMethods,
        );
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::SomeClass>() == 4);
    assert!(::core::mem::align_of::<crate::SomeClass>() == 4);
    static_assertions::assert_impl_all!(crate::SomeClass: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::SomeClass: Drop);
    assert!(::core::mem::offset_of!(crate::SomeClass, int_var) == 0);
    assert!(::core::mem::size_of::<crate::InstanceMethods>() == 4);
    assert!(::core::mem::align_of::<crate::InstanceMethods>() == 4);
    static_assertions::assert_impl_all!(crate::InstanceMethods: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::InstanceMethods: Drop);
    assert!(::core::mem::offset_of!(crate::InstanceMethods, int_field) == 0);
};
