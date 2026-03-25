// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/struct/default_member_functions:default_member_functions
// Features: supported, types

#![rustfmt::skip]
#![feature(custom_inner_attributes, impl_trait_in_assoc_type, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![deny(warnings)]

extern crate core as __rust_core;
/// Generated from: rs_bindings_from_cc/test/struct/default_member_functions/default_member_functions.h;l=10
#[::ctor::recursively_pinned]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=Uncopyable
pub struct Uncopyable {
    __non_field_data: [::__rust_core::cell::Cell<::__rust_core::mem::MaybeUninit<u8>>; 1],
}
impl !Send for Uncopyable {}
impl !Sync for Uncopyable {}
unsafe impl ::cxx::ExternType for Uncopyable {
    type Id = ::cxx::type_id!("Uncopyable");
    type Kind = ::cxx::kind::Opaque;
}

/// Generated from: rs_bindings_from_cc/test/struct/default_member_functions/default_member_functions.h;l=12
impl ::ctor::CtorNew<()> for Uncopyable {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(move |dest: *mut Self| {
                crate::detail::__rust_thunk___ZN10UncopyableC1Ev(
                    dest as *mut ::__rust_core::ffi::c_void,
                );
            })
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/struct/default_member_functions/default_member_functions.h;l=17
#[::ctor::recursively_pinned(PinnedDrop)]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=UncopyableDespiteDecl
pub struct UncopyableDespiteDecl {
    __non_field_data: [::__rust_core::cell::Cell<::__rust_core::mem::MaybeUninit<u8>>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) vector_: [::__rust_core::cell::Cell<::__rust_core::mem::MaybeUninit<u8>>; 24],
}
impl !Send for UncopyableDespiteDecl {}
impl !Sync for UncopyableDespiteDecl {}
unsafe impl ::cxx::ExternType for UncopyableDespiteDecl {
    type Id = ::cxx::type_id!("UncopyableDespiteDecl");
    type Kind = ::cxx::kind::Opaque;
}

/// Generated from: rs_bindings_from_cc/test/struct/default_member_functions/default_member_functions.h;l=17
impl ::ctor::PinnedDrop for UncopyableDespiteDecl {
    #[inline(always)]
    unsafe fn pinned_drop<'a>(self: ::__rust_core::pin::Pin<&'a mut Self>) {
        crate::detail::__rust_thunk___ZN21UncopyableDespiteDeclD1Ev(self)
    }
}

// Generated from: rs_bindings_from_cc/test/struct/default_member_functions/default_member_functions.h;l=17
// error: function `UncopyableDespiteDecl::operator=` could not be bound
//   Implicit copy assignment is considered invalid

/// Generated from: rs_bindings_from_cc/test/struct/default_member_functions/default_member_functions.h;l=19
impl ::ctor::CtorNew<()> for UncopyableDespiteDecl {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(move |dest: *mut Self| {
                crate::detail::__rust_thunk___ZN21UncopyableDespiteDeclC1Ev(
                    dest as *mut ::__rust_core::ffi::c_void,
                );
            })
        }
    }
}

// Generated from: rs_bindings_from_cc/test/struct/default_member_functions/default_member_functions.h;l=21
// error: constructor `UncopyableDespiteDecl::UncopyableDespiteDecl` could not be bound
//   Would use an unavailable copy constructor for UncopyableDespiteDecl

// Generated from: nowhere/llvm/src/libcxx/include/__type_traits/integral_constant.h;l=21
// error: struct `std::integral_constant<bool, false>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__type_traits/integral_constant.h;l=21
// error: struct `std::integral_constant<bool, true>` could not be bound
//   template instantiation is not yet supported

// error: class `std::basic_string<char32_t, std::char_traits<char32_t>, std::pmr::polymorphic_allocator<char32_t>>` could not be bound
//   incomplete type

// error: class `std::basic_string<char32_t, std::char_traits<char32_t>, std::allocator<char32_t>>` could not be bound
//   incomplete type

// error: class `std::basic_string<char16_t, std::char_traits<char16_t>, std::pmr::polymorphic_allocator<char16_t>>` could not be bound
//   incomplete type

// error: class `std::basic_string<char16_t, std::char_traits<char16_t>, std::allocator<char16_t>>` could not be bound
//   incomplete type

// Generated from: nowhere/llvm/src/libcxx/include/__fwd/string.h;l=42
// error: class `std::basic_string` could not be bound
//   Unsupported type 'char8_t': Unsupported builtin type

// Generated from: nowhere/llvm/src/libcxx/include/__fwd/string.h;l=42
// error: class `std::basic_string` could not be bound
//   Unsupported type 'char8_t': Unsupported builtin type

// error: class `std::basic_string<char, std::char_traits<char>, std::pmr::polymorphic_allocator<char>>` could not be bound
//   incomplete type

// error: class `std::basic_string<char, std::char_traits<char>, std::allocator<char>>` could not be bound
//   incomplete type

// Generated from: nowhere/llvm/src/libcxx/include/__fwd/string.h;l=42
// error: class `std::basic_string` could not be bound
//   Unsupported type 'wchar_t': Unsupported builtin type

// Generated from: nowhere/llvm/src/libcxx/include/__fwd/string.h;l=42
// error: class `std::basic_string` could not be bound
//   Unsupported type 'wchar_t': Unsupported builtin type

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN10UncopyableC1Ev(
            __this: *mut ::__rust_core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN21UncopyableDespiteDeclD1Ev<'a>(
            __this: ::__rust_core::pin::Pin<&'a mut crate::UncopyableDespiteDecl>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN21UncopyableDespiteDeclC1Ev(
            __this: *mut ::__rust_core::ffi::c_void,
        );
    }
}

const _: () = {
    assert!(::__rust_core::mem::size_of::<crate::Uncopyable>() == 1);
    assert!(::__rust_core::mem::align_of::<crate::Uncopyable>() == 1);
    static_assertions::assert_not_impl_any!(crate::Uncopyable: Copy,Drop);

    assert!(::__rust_core::mem::size_of::<crate::UncopyableDespiteDecl>() == 24);
    assert!(::__rust_core::mem::align_of::<crate::UncopyableDespiteDecl>() == 8);
    static_assertions::assert_impl_all!(crate::UncopyableDespiteDecl: Drop);
    static_assertions::assert_not_impl_any!(crate::UncopyableDespiteDecl: Copy);
    assert!(::__rust_core::mem::offset_of!(crate::UncopyableDespiteDecl, vector_) == 0);
};
