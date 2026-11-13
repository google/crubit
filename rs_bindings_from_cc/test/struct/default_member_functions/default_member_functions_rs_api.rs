// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/struct/default_member_functions:default_member_functions

#![rustfmt::skip]
#![feature(cfi_encoding, custom_inner_attributes, impl_trait_in_assoc_type, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![allow(deprecated)]
#![allow(unknown_lints, suspicious_runtime_symbol_definitions)]
#![deny(warnings)]
/// Not Rust-movable: type is neither copyable nor movable
#[::ctor::recursively_pinned]
#[cfi_encoding = "10Uncopyable"]
#[repr(C)]
#[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_type=Uncopyable")]
pub struct Uncopyable {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 1],
}
unsafe impl ::cxx::ExternType for Uncopyable {
    type Id = ::cxx::type_id!("Uncopyable");
    type Kind = ::cxx::kind::Opaque;
}
forward_declare::unsafe_define!(forward_declare::symbol!("Uncopyable"), crate::Uncopyable);

impl ::ctor::CtorNew<()> for Uncopyable {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                let () = args;
                crate::detail::__rust_thunk___ZN10UncopyableC1Ev(
                    __crubit_dest as *mut ::core::ffi::c_void,
                );
            })
        }
    }
}

#[cfi_encoding = "21UncopyableDespiteDecl"]
#[repr(C, align(8))]
#[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_type=UncopyableDespiteDecl")]
pub struct UncopyableDespiteDecl {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) vector_: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 24],
}
/// Cannot infer `Send` because:
/// * field `vector_` is represented as opaque bytes
impl !Send for UncopyableDespiteDecl {}
/// Cannot infer `Sync` because:
/// * field `vector_` is represented as opaque bytes
impl !Sync for UncopyableDespiteDecl {}
unsafe impl ::cxx::ExternType for UncopyableDespiteDecl {
    type Id = ::cxx::type_id!("UncopyableDespiteDecl");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("UncopyableDespiteDecl"),
    crate::UncopyableDespiteDecl
);

impl Drop for UncopyableDespiteDecl {
    #[inline(always)]
    fn drop<'__this>(&'__this mut self) {
        unsafe { crate::detail::__rust_thunk___ZN21UncopyableDespiteDeclD1Ev(self) }
    }
}

// error: function `UncopyableDespiteDecl::operator=` could not be bound
//   Defaulted function relies on an invalid decl: UncopyableDespiteDecl::operator=

impl Default for UncopyableDespiteDecl {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN21UncopyableDespiteDeclC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

// error: constructor `UncopyableDespiteDecl::UncopyableDespiteDecl` could not be bound
//   Defaulted function relies on an invalid decl: UncopyableDespiteDecl::UncopyableDespiteDecl

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN10UncopyableC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN21UncopyableDespiteDeclD1Ev<'__this>(
            __this: &'__this mut crate::UncopyableDespiteDecl,
        );
        pub(crate) unsafe fn __rust_thunk___ZN21UncopyableDespiteDeclC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::Uncopyable>() == 1);
    assert!(::core::mem::align_of::<crate::Uncopyable>() == 1);
    static_assertions::assert_not_impl_any!(crate::Uncopyable: Copy,Drop);

    assert!(::core::mem::size_of::<crate::UncopyableDespiteDecl>() == 24);
    assert!(::core::mem::align_of::<crate::UncopyableDespiteDecl>() == 8);
    static_assertions::assert_impl_all!(crate::UncopyableDespiteDecl: Drop);
    static_assertions::assert_not_impl_any!(crate::UncopyableDespiteDecl: Copy);
    assert!(::core::mem::offset_of!(crate::UncopyableDespiteDecl, vector_) == 0);
};
