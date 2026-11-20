// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/function/simple:simple

#![rustfmt::skip]
#![feature(cfi_encoding, custom_inner_attributes, impl_trait_in_assoc_type, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![allow(deprecated)]
#![allow(unknown_lints, suspicious_runtime_symbol_definitions)]
#![deny(warnings)]
#[inline(always)]
pub fn return_value() -> ::ffi_11::c_int {
    unsafe { crate::detail::__rust_thunk___Z12return_valuev() }
}

#[inline(always)]
pub fn return_pointer() -> *mut ::ffi_11::c_int {
    unsafe { crate::detail::__rust_thunk___Z14return_pointerv() }
}

#[inline(always)]
pub fn return_reference() -> *mut ::ffi_11::c_int {
    unsafe { crate::detail::__rust_thunk___Z16return_referencev() }
}

/// # Safety
///
/// The caller must ensure that the following unsafe arguments are not misused by the function:
/// * `i`: raw pointer
#[inline(always)]
pub unsafe fn take_pointer(i: *mut ::ffi_11::c_int) {
    unsafe { crate::detail::__rust_thunk___Z12take_pointerPi(i) }
}

/// # Safety
///
/// The caller must ensure that the following unsafe arguments are not misused by the function:
/// * `i`: raw pointer
#[inline(always)]
pub unsafe fn take_reference(i: *mut ::ffi_11::c_int) {
    unsafe { crate::detail::__rust_thunk___Z14take_referenceRi(i) }
}

#[inline(always)]
pub fn forward_pointer(i: *const ::ffi_11::c_int) -> *const ::ffi_11::c_int {
    unsafe { crate::detail::__rust_thunk___Z15forward_pointerPKi(i) }
}

/// # Safety
///
/// The caller must ensure that the following unsafe arguments are not misused by the function:
/// * `i`: raw pointer
#[inline(always)]
pub unsafe fn forward_reference(i: *const ::ffi_11::c_int) -> *const ::ffi_11::c_int {
    unsafe { crate::detail::__rust_thunk___Z17forward_referenceRKi(i) }
}

#[inline(always)]
pub fn multiply(x: ::ffi_11::c_int, y: ::ffi_11::c_int) -> ::ffi_11::c_int {
    unsafe { crate::detail::__rust_thunk___Z8multiplyii(x, y) }
}

#[inline(always)]
pub fn multiply_with_unnamed_parameters(
    __param_0: ::ffi_11::c_int,
    __param_1: ::ffi_11::c_int,
) -> ::ffi_11::c_int {
    unsafe {
        crate::detail::__rust_thunk___Z32multiply_with_unnamed_parametersii(__param_0, __param_1)
    }
}

#[inline(always)]
pub fn multiply_with_keyword_named_parameters(
    __param_0: ::ffi_11::c_int,
    __param_1: ::ffi_11::c_int,
    __param_2: ::ffi_11::c_int,
) -> ::ffi_11::c_int {
    unsafe {
        crate::detail::__rust_thunk___Z38multiply_with_keyword_named_parametersiii(
            __param_0, __param_1, __param_2,
        )
    }
}

/// https://cdecl.org/?q=int+%28*get_multiply_function%28%29%29%28int%2C+int%29:
/// declare foo as function returning pointer to function (int, int) returning
/// int
#[inline(always)]
pub fn get_pointer_to_multiply_function(
) -> Option<extern "C" fn(::ffi_11::c_int, ::ffi_11::c_int) -> ::ffi_11::c_int> {
    unsafe { crate::detail::__rust_thunk___Z32get_pointer_to_multiply_functionv() }
}

/// Same as above, but returning a *reference* to a function.
#[inline(always)]
pub fn get_reference_to_multiply_function(
) -> extern "C" fn(::ffi_11::c_int, ::ffi_11::c_int) -> ::ffi_11::c_int {
    unsafe { crate::detail::__rust_thunk___Z34get_reference_to_multiply_functionv() }
}

#[inline(always)]
pub fn inline_get_pointer_to_multiply_function(
) -> Option<extern "C" fn(::ffi_11::c_int, ::ffi_11::c_int) -> ::ffi_11::c_int> {
    unsafe { crate::detail::__rust_thunk___Z39inline_get_pointer_to_multiply_functionv() }
}

#[inline(always)]
pub fn apply_binary_op(
    x: ::ffi_11::c_int,
    y: ::ffi_11::c_int,
    op: Option<extern "C" fn(::ffi_11::c_int, ::ffi_11::c_int) -> ::ffi_11::c_int>,
) -> ::ffi_11::c_int {
    unsafe { crate::detail::__rust_thunk___Z15apply_binary_opiiPFiiiE(x, y, op) }
}

#[inline(always)]
pub fn noinline_multiply(x: ::ffi_11::c_int, y: ::ffi_11::c_int) -> ::ffi_11::c_int {
    unsafe { crate::detail::__rust_thunk___Z17noinline_multiplyii(x, y) }
}

/// Not Rust-movable: type is neither copyable nor movable
#[::ctor::recursively_pinned(PinnedDrop)]
#[cfi_encoding = "22NonMovableWithNoInline"]
#[repr(C, align(4))]
#[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_type=NonMovableWithNoInline")]
pub struct NonMovableWithNoInline {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) value_: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 4],
}
/// Cannot infer `Send` because:
/// * field `value_` is represented as opaque bytes
impl !Send for NonMovableWithNoInline {}
/// Cannot infer `Sync` because:
/// * field `value_` is represented as opaque bytes
impl !Sync for NonMovableWithNoInline {}
unsafe impl ::cxx::ExternType for NonMovableWithNoInline {
    type Id = ::cxx::type_id!("NonMovableWithNoInline");
    type Kind = ::cxx::kind::Opaque;
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("NonMovableWithNoInline"),
    crate::NonMovableWithNoInline
);
impl NonMovableWithNoInline {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub unsafe fn value(__this: *const Self) -> ::ffi_11::c_int {
        unsafe { self::non_movable_with_no_inline::value(__this) }
    }
}

impl ::ctor::CtorNew<::ffi_11::c_int> for NonMovableWithNoInline {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ffi_11::c_int) -> Self::CtorType {
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                let mut value = args;
                crate::detail::__rust_thunk___ZN22NonMovableWithNoInlineC1Ei(
                    __crubit_dest as *mut ::core::ffi::c_void,
                    value,
                );
            })
        }
    }
}
impl ::ctor::CtorNew<(::ffi_11::c_int,)> for NonMovableWithNoInline {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (::ffi_11::c_int,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ffi_11::c_int>>::ctor_new(arg)
    }
}

impl ::ctor::PinnedDrop for NonMovableWithNoInline {
    #[inline(always)]
    unsafe fn pinned_drop<'a>(self: ::core::pin::Pin<&'a mut Self>) {
        unsafe { crate::detail::__rust_thunk___ZN22NonMovableWithNoInlineD1Ev(self) }
    }
}

pub mod non_movable_with_no_inline {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: do_not_bind=")]
    #[inline(always)]
    pub unsafe fn value(__this: *const crate::NonMovableWithNoInline) -> ::ffi_11::c_int {
        unsafe { crate::detail::__rust_thunk___ZNK22NonMovableWithNoInline5valueEv(__this) }
    }
}

// TODO(b/217419782): Add testcases for pointers to functions that take or
// return takes/returns non-trivially-movable types by value. In particular,
// some function signatures might require going through a C++ thunk - such
// function pointers can't work without a thunk. See also
// <internal link>

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        #[link_name = "_Z12return_valuev"]
        pub(crate) unsafe fn __rust_thunk___Z12return_valuev() -> ::ffi_11::c_int;
        #[link_name = "_Z14return_pointerv"]
        pub(crate) unsafe fn __rust_thunk___Z14return_pointerv() -> *mut ::ffi_11::c_int;
        #[link_name = "_Z16return_referencev"]
        pub(crate) unsafe fn __rust_thunk___Z16return_referencev() -> *mut ::ffi_11::c_int;
        #[link_name = "_Z12take_pointerPi"]
        pub(crate) unsafe fn __rust_thunk___Z12take_pointerPi(i: *mut ::ffi_11::c_int);
        #[link_name = "_Z14take_referenceRi"]
        pub(crate) unsafe fn __rust_thunk___Z14take_referenceRi(i: *mut ::ffi_11::c_int);
        #[link_name = "_Z15forward_pointerPKi"]
        pub(crate) unsafe fn __rust_thunk___Z15forward_pointerPKi(
            i: *const ::ffi_11::c_int,
        ) -> *const ::ffi_11::c_int;
        #[link_name = "_Z17forward_referenceRKi"]
        pub(crate) unsafe fn __rust_thunk___Z17forward_referenceRKi(
            i: *const ::ffi_11::c_int,
        ) -> *const ::ffi_11::c_int;
        #[link_name = "_Z8multiplyii"]
        pub(crate) unsafe fn __rust_thunk___Z8multiplyii(
            x: ::ffi_11::c_int,
            y: ::ffi_11::c_int,
        ) -> ::ffi_11::c_int;
        #[link_name = "_Z32multiply_with_unnamed_parametersii"]
        pub(crate) unsafe fn __rust_thunk___Z32multiply_with_unnamed_parametersii(
            __param_0: ::ffi_11::c_int,
            __param_1: ::ffi_11::c_int,
        ) -> ::ffi_11::c_int;
        #[link_name = "_Z38multiply_with_keyword_named_parametersiii"]
        pub(crate) unsafe fn __rust_thunk___Z38multiply_with_keyword_named_parametersiii(
            __param_0: ::ffi_11::c_int,
            __param_1: ::ffi_11::c_int,
            __param_2: ::ffi_11::c_int,
        ) -> ::ffi_11::c_int;
        #[link_name = "_Z32get_pointer_to_multiply_functionv"]
        pub(crate) unsafe fn __rust_thunk___Z32get_pointer_to_multiply_functionv(
        ) -> Option<extern "C" fn(::ffi_11::c_int, ::ffi_11::c_int) -> ::ffi_11::c_int>;
        #[link_name = "_Z34get_reference_to_multiply_functionv"]
        pub(crate) unsafe fn __rust_thunk___Z34get_reference_to_multiply_functionv(
        ) -> extern "C" fn(::ffi_11::c_int, ::ffi_11::c_int) -> ::ffi_11::c_int;
        pub(crate) unsafe fn __rust_thunk___Z39inline_get_pointer_to_multiply_functionv(
        ) -> Option<extern "C" fn(::ffi_11::c_int, ::ffi_11::c_int) -> ::ffi_11::c_int>;
        pub(crate) unsafe fn __rust_thunk___Z15apply_binary_opiiPFiiiE(
            x: ::ffi_11::c_int,
            y: ::ffi_11::c_int,
            op: Option<extern "C" fn(::ffi_11::c_int, ::ffi_11::c_int) -> ::ffi_11::c_int>,
        ) -> ::ffi_11::c_int;
        #[link_name = "_Z17noinline_multiplyii"]
        pub(crate) unsafe fn __rust_thunk___Z17noinline_multiplyii(
            x: ::ffi_11::c_int,
            y: ::ffi_11::c_int,
        ) -> ::ffi_11::c_int;
        #[link_name = "_ZN22NonMovableWithNoInlineC1Ei"]
        pub(crate) unsafe fn __rust_thunk___ZN22NonMovableWithNoInlineC1Ei(
            __this: *mut ::core::ffi::c_void,
            value: ::ffi_11::c_int,
        );
        #[link_name = "_ZN22NonMovableWithNoInlineD1Ev"]
        pub(crate) unsafe fn __rust_thunk___ZN22NonMovableWithNoInlineD1Ev<'a>(
            __this: ::core::pin::Pin<&'a mut crate::NonMovableWithNoInline>,
        );
        #[link_name = "_ZNK22NonMovableWithNoInline5valueEv"]
        pub(crate) unsafe fn __rust_thunk___ZNK22NonMovableWithNoInline5valueEv(
            __this: *const crate::NonMovableWithNoInline,
        ) -> ::ffi_11::c_int;
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::NonMovableWithNoInline>() == 4);
    assert!(::core::mem::align_of::<crate::NonMovableWithNoInline>() == 4);
    static_assertions::assert_impl_all!(crate::NonMovableWithNoInline: Drop);
    static_assertions::assert_not_impl_any!(crate::NonMovableWithNoInline: Copy);
    assert!(::core::mem::offset_of!(crate::NonMovableWithNoInline, value_) == 0);
};
