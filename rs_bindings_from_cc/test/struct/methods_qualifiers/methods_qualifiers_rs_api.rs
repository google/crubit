// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/struct/methods_qualifiers:methods_qualifiers
// Features: assume_lifetimes, assume_this_lifetimes, callables, check_default_initialized, experimental, layout_compat_tuple, leading_colons_for_cpp_type, supported, template_instantiation, types, unsafe_view, wrapper

#![rustfmt::skip]
#![feature(custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![deny(rust_2024_compatibility)]
#![allow(unused)]
#![allow(deprecated)]
#![deny(warnings)]

/// Generated from: rs_bindings_from_cc/test/struct/methods_qualifiers/methods_qualifiers.h;l=10
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=:: UnpinStructWithRefQualifiedMethods
pub struct UnpinStructWithRefQualifiedMethods {
    pub i: ::ffi_11::c_int,
}
impl !Send for UnpinStructWithRefQualifiedMethods {}
impl !Sync for UnpinStructWithRefQualifiedMethods {}
unsafe impl ::cxx::ExternType for UnpinStructWithRefQualifiedMethods {
    type Id = ::cxx::type_id!(":: UnpinStructWithRefQualifiedMethods");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(
    forward_declare::symbol!(":: UnpinStructWithRefQualifiedMethods"),
    crate::UnpinStructWithRefQualifiedMethods
);
impl UnpinStructWithRefQualifiedMethods {
    /// Generated from: rs_bindings_from_cc/test/struct/methods_qualifiers/methods_qualifiers.h;l=12
    #[inline(always)]
    pub fn increment_i<'__this>(&'__this mut self) {
        unsafe { self::unpin_struct_with_ref_qualified_methods::increment_i(self) }
    }
    /// Generated from: rs_bindings_from_cc/test/struct/methods_qualifiers/methods_qualifiers.h;l=13
    #[inline(always)]
    pub fn unqualified_get_i<'__this>(&'__this mut self) -> ::ffi_11::c_int {
        unsafe { self::unpin_struct_with_ref_qualified_methods::unqualified_get_i(self) }
    }
    /// Generated from: rs_bindings_from_cc/test/struct/methods_qualifiers/methods_qualifiers.h;l=14
    #[inline(always)]
    pub fn const_qualified_get_i<'__this>(&'__this self) -> ::ffi_11::c_int {
        unsafe { self::unpin_struct_with_ref_qualified_methods::const_qualified_get_i(self) }
    }
    /// Generated from: rs_bindings_from_cc/test/struct/methods_qualifiers/methods_qualifiers.h;l=15
    #[inline(always)]
    pub fn lvalue_ref_qualified_get_i<'__this>(&'__this mut self) -> ::ffi_11::c_int {
        unsafe { self::unpin_struct_with_ref_qualified_methods::lvalue_ref_qualified_get_i(self) }
    }
    /// Generated from: rs_bindings_from_cc/test/struct/methods_qualifiers/methods_qualifiers.h;l=16
    #[inline(always)]
    pub fn const_lvalue_ref_qualified_get_i<'__this>(&'__this self) -> ::ffi_11::c_int {
        unsafe {
            self::unpin_struct_with_ref_qualified_methods::const_lvalue_ref_qualified_get_i(self)
        }
    }
    /// Generated from: rs_bindings_from_cc/test/struct/methods_qualifiers/methods_qualifiers.h;l=17
    #[inline(always)]
    pub fn rvalue_ref_qualified_get_i<'__this>(&'__this mut self) -> ::ffi_11::c_int {
        unsafe { self::unpin_struct_with_ref_qualified_methods::rvalue_ref_qualified_get_i(self) }
    }
    /// Generated from: rs_bindings_from_cc/test/struct/methods_qualifiers/methods_qualifiers.h;l=18
    #[inline(always)]
    pub fn const_rvalue_ref_qualified_get_i<'__this>(&'__this self) -> ::ffi_11::c_int {
        unsafe {
            self::unpin_struct_with_ref_qualified_methods::const_rvalue_ref_qualified_get_i(self)
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/struct/methods_qualifiers/methods_qualifiers.h;l=10
impl Default for UnpinStructWithRefQualifiedMethods {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN34UnpinStructWithRefQualifiedMethodsC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

pub mod unpin_struct_with_ref_qualified_methods {
    /// Generated from: rs_bindings_from_cc/test/struct/methods_qualifiers/methods_qualifiers.h;l=12
    #[inline(always)]
    pub(crate) fn increment_i<'__this>(
        __this: &'__this mut crate::UnpinStructWithRefQualifiedMethods,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN34UnpinStructWithRefQualifiedMethods11increment_iEv(
                __this,
            )
        }
    }
    /// Generated from: rs_bindings_from_cc/test/struct/methods_qualifiers/methods_qualifiers.h;l=13
    #[inline(always)]
    pub(crate) fn unqualified_get_i<'__this>(
        __this: &'__this mut crate::UnpinStructWithRefQualifiedMethods,
    ) -> ::ffi_11::c_int {
        unsafe {
            crate::detail::__rust_thunk___ZN34UnpinStructWithRefQualifiedMethods17unqualified_get_iEv(__this)
        }
    }
    /// Generated from: rs_bindings_from_cc/test/struct/methods_qualifiers/methods_qualifiers.h;l=14
    #[inline(always)]
    pub(crate) fn const_qualified_get_i<'__this>(
        __this: &'__this crate::UnpinStructWithRefQualifiedMethods,
    ) -> ::ffi_11::c_int {
        unsafe {
            crate::detail::__rust_thunk___ZNK34UnpinStructWithRefQualifiedMethods21const_qualified_get_iEv(__this)
        }
    }
    /// Generated from: rs_bindings_from_cc/test/struct/methods_qualifiers/methods_qualifiers.h;l=15
    #[inline(always)]
    pub(crate) fn lvalue_ref_qualified_get_i<'__this>(
        __this: &'__this mut crate::UnpinStructWithRefQualifiedMethods,
    ) -> ::ffi_11::c_int {
        unsafe {
            crate::detail::__rust_thunk___ZNR34UnpinStructWithRefQualifiedMethods26lvalue_ref_qualified_get_iEv(__this)
        }
    }
    /// Generated from: rs_bindings_from_cc/test/struct/methods_qualifiers/methods_qualifiers.h;l=16
    #[inline(always)]
    pub(crate) fn const_lvalue_ref_qualified_get_i<'__this>(
        __this: &'__this crate::UnpinStructWithRefQualifiedMethods,
    ) -> ::ffi_11::c_int {
        unsafe {
            crate::detail::__rust_thunk___ZNKR34UnpinStructWithRefQualifiedMethods32const_lvalue_ref_qualified_get_iEv(__this)
        }
    }
    /// Generated from: rs_bindings_from_cc/test/struct/methods_qualifiers/methods_qualifiers.h;l=17
    #[inline(always)]
    pub(crate) fn rvalue_ref_qualified_get_i<'__this>(
        __this: &'__this mut crate::UnpinStructWithRefQualifiedMethods,
    ) -> ::ffi_11::c_int {
        unsafe {
            crate::detail::__rust_thunk___ZNO34UnpinStructWithRefQualifiedMethods26rvalue_ref_qualified_get_iEv(__this)
        }
    }
    /// Generated from: rs_bindings_from_cc/test/struct/methods_qualifiers/methods_qualifiers.h;l=18
    #[inline(always)]
    pub(crate) fn const_rvalue_ref_qualified_get_i<'__this>(
        __this: &'__this crate::UnpinStructWithRefQualifiedMethods,
    ) -> ::ffi_11::c_int {
        unsafe {
            crate::detail::__rust_thunk___ZNKO34UnpinStructWithRefQualifiedMethods32const_rvalue_ref_qualified_get_iEv(__this)
        }
    }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN34UnpinStructWithRefQualifiedMethodsC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        #[link_name = "_ZN34UnpinStructWithRefQualifiedMethods11increment_iEv"]
        pub(crate) unsafe fn __rust_thunk___ZN34UnpinStructWithRefQualifiedMethods11increment_iEv<
            '__this,
        >(
            __this: &'__this mut crate::UnpinStructWithRefQualifiedMethods,
        );
        #[link_name = "_ZN34UnpinStructWithRefQualifiedMethods17unqualified_get_iEv"]
        pub(crate) unsafe fn __rust_thunk___ZN34UnpinStructWithRefQualifiedMethods17unqualified_get_iEv<
            '__this,
        >(
            __this: &'__this mut crate::UnpinStructWithRefQualifiedMethods,
        ) -> ::ffi_11::c_int;
        #[link_name = "_ZNK34UnpinStructWithRefQualifiedMethods21const_qualified_get_iEv"]
        pub(crate) unsafe fn __rust_thunk___ZNK34UnpinStructWithRefQualifiedMethods21const_qualified_get_iEv<
            '__this,
        >(
            __this: &'__this crate::UnpinStructWithRefQualifiedMethods,
        ) -> ::ffi_11::c_int;
        #[link_name = "_ZNR34UnpinStructWithRefQualifiedMethods26lvalue_ref_qualified_get_iEv"]
        pub(crate) unsafe fn __rust_thunk___ZNR34UnpinStructWithRefQualifiedMethods26lvalue_ref_qualified_get_iEv<
            '__this,
        >(
            __this: &'__this mut crate::UnpinStructWithRefQualifiedMethods,
        ) -> ::ffi_11::c_int;
        #[link_name = "_ZNKR34UnpinStructWithRefQualifiedMethods32const_lvalue_ref_qualified_get_iEv"]
        pub(crate) unsafe fn __rust_thunk___ZNKR34UnpinStructWithRefQualifiedMethods32const_lvalue_ref_qualified_get_iEv<
            '__this,
        >(
            __this: &'__this crate::UnpinStructWithRefQualifiedMethods,
        ) -> ::ffi_11::c_int;
        #[link_name = "_ZNO34UnpinStructWithRefQualifiedMethods26rvalue_ref_qualified_get_iEv"]
        pub(crate) unsafe fn __rust_thunk___ZNO34UnpinStructWithRefQualifiedMethods26rvalue_ref_qualified_get_iEv<
            '__this,
        >(
            __this: &'__this mut crate::UnpinStructWithRefQualifiedMethods,
        ) -> ::ffi_11::c_int;
        #[link_name = "_ZNKO34UnpinStructWithRefQualifiedMethods32const_rvalue_ref_qualified_get_iEv"]
        pub(crate) unsafe fn __rust_thunk___ZNKO34UnpinStructWithRefQualifiedMethods32const_rvalue_ref_qualified_get_iEv<
            '__this,
        >(
            __this: &'__this crate::UnpinStructWithRefQualifiedMethods,
        ) -> ::ffi_11::c_int;
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::UnpinStructWithRefQualifiedMethods>() == 4);
    assert!(::core::mem::align_of::<crate::UnpinStructWithRefQualifiedMethods>() == 4);
    static_assertions::assert_impl_all!(crate::UnpinStructWithRefQualifiedMethods: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::UnpinStructWithRefQualifiedMethods: Drop);
    assert!(::core::mem::offset_of!(crate::UnpinStructWithRefQualifiedMethods, i) == 0);
};
