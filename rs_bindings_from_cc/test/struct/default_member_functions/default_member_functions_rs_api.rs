// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/struct/default_member_functions:default_member_functions
// Features: non_unpin_ctor, std_unique_ptr, std_vector, supported

#![rustfmt::skip]
#![feature(
    allocator_api,
    cfg_sanitize,
    custom_inner_attributes,
    impl_trait_in_assoc_type,
    negative_impls
)]
#![allow(stable_features)]
#![no_std]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

/// Generated from: rs_bindings_from_cc/test/struct/default_member_functions/default_member_functions.h;l=10
#[::ctor::recursively_pinned]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=Uncopyable
pub struct Uncopyable {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for Uncopyable {}
impl !Sync for Uncopyable {}
unsafe impl ::cxx::ExternType for Uncopyable {
    type Id = ::cxx::type_id!("Uncopyable");
    type Kind = ::cxx::kind::Opaque;
}

/// Generated from: rs_bindings_from_cc/test/struct/default_member_functions/default_member_functions.h;l=12
impl ::ctor::CtorNew<()> for Uncopyable {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(move |dest: *mut Self| {
                crate::detail::__rust_thunk___ZN10UncopyableC1Ev(dest as *mut ::core::ffi::c_void);
            })
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/struct/default_member_functions/default_member_functions.h;l=17
#[::ctor::recursively_pinned(PinnedDrop)]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=UncopyableDespiteDecl
pub struct UncopyableDespiteDecl {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) vector_: [::core::mem::MaybeUninit<u8>; 24],
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
    unsafe fn pinned_drop<'a>(self: ::core::pin::Pin<&'a mut Self>) {
        crate::detail::__rust_thunk___ZN21UncopyableDespiteDeclD1Ev(self)
    }
}

// Generated from: rs_bindings_from_cc/test/struct/default_member_functions/default_member_functions.h;l=17
// Error while generating bindings for function 'UncopyableDespiteDecl::operator=':
// Implicit copy assignment is considered invalid

/// Generated from: rs_bindings_from_cc/test/struct/default_member_functions/default_member_functions.h;l=19
impl ::ctor::CtorNew<()> for UncopyableDespiteDecl {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(move |dest: *mut Self| {
                crate::detail::__rust_thunk___ZN21UncopyableDespiteDeclC1Ev(
                    dest as *mut ::core::ffi::c_void,
                );
            })
        }
    }
}

// Generated from: rs_bindings_from_cc/test/struct/default_member_functions/default_member_functions.h;l=21
// Error while generating bindings for constructor 'UncopyableDespiteDecl::UncopyableDespiteDecl':
// Would use an unavailable copy constructor for UncopyableDespiteDecl

// Generated from: nowhere/llvm/src/libcxx/include/__type_traits/integral_constant.h;l=21
// Error while generating bindings for struct 'std::integral_constant<bool, false>':
// Can't generate bindings for std::integral_constant<bool, false>, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/struct/default_member_functions:default_member_functions needs [//features:wrapper] for std::integral_constant<bool, false> (crate::__CcTemplateInstNSt3__u17integral_constantIbLb0EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__type_traits/integral_constant.h;l=21
// Error while generating bindings for struct 'std::integral_constant<bool, true>':
// Can't generate bindings for std::integral_constant<bool, true>, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/struct/default_member_functions:default_member_functions needs [//features:wrapper] for std::integral_constant<bool, true> (crate::__CcTemplateInstNSt3__u17integral_constantIbLb1EEE is a template instantiation)

// Error while generating bindings for class 'std::basic_string<char32_t, std::char_traits<char32_t>, std::pmr::polymorphic_allocator<char32_t>>':
// Can't generate bindings for std::basic_string<char32_t, std::char_traits<char32_t>, std::pmr::polymorphic_allocator<char32_t>>, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/struct/default_member_functions:default_member_functions needs [//features:wrapper] for std::basic_string<char32_t, std::char_traits<char32_t>, std::pmr::polymorphic_allocator<char32_t>> (incomplete type)

// Error while generating bindings for class 'std::basic_string<char32_t, std::char_traits<char32_t>, std::allocator<char32_t>>':
// Can't generate bindings for std::basic_string<char32_t, std::char_traits<char32_t>, std::allocator<char32_t>>, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/struct/default_member_functions:default_member_functions needs [//features:wrapper] for std::basic_string<char32_t, std::char_traits<char32_t>, std::allocator<char32_t>> (incomplete type)

// Error while generating bindings for class 'std::basic_string<char16_t, std::char_traits<char16_t>, std::pmr::polymorphic_allocator<char16_t>>':
// Can't generate bindings for std::basic_string<char16_t, std::char_traits<char16_t>, std::pmr::polymorphic_allocator<char16_t>>, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/struct/default_member_functions:default_member_functions needs [//features:wrapper] for std::basic_string<char16_t, std::char_traits<char16_t>, std::pmr::polymorphic_allocator<char16_t>> (incomplete type)

// Error while generating bindings for class 'std::basic_string<char16_t, std::char_traits<char16_t>, std::allocator<char16_t>>':
// Can't generate bindings for std::basic_string<char16_t, std::char_traits<char16_t>, std::allocator<char16_t>>, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/struct/default_member_functions:default_member_functions needs [//features:wrapper] for std::basic_string<char16_t, std::char_traits<char16_t>, std::allocator<char16_t>> (incomplete type)

// Generated from: nowhere/llvm/src/libcxx/include/__fwd/string.h;l=42
// Error while generating bindings for class 'std::basic_string':
// Unsupported type 'char8_t': Unsupported builtin type

// Generated from: nowhere/llvm/src/libcxx/include/__fwd/string.h;l=42
// Error while generating bindings for class 'std::basic_string':
// Unsupported type 'char8_t': Unsupported builtin type

// Error while generating bindings for class 'std::basic_string<char, std::char_traits<char>, std::pmr::polymorphic_allocator<char>>':
// Can't generate bindings for std::basic_string<char, std::char_traits<char>, std::pmr::polymorphic_allocator<char>>, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/struct/default_member_functions:default_member_functions needs [//features:wrapper] for std::basic_string<char, std::char_traits<char>, std::pmr::polymorphic_allocator<char>> (incomplete type)

// Error while generating bindings for class 'std::basic_string<char, std::char_traits<char>, std::allocator<char>>':
// Can't generate bindings for std::basic_string<char, std::char_traits<char>, std::allocator<char>>, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/struct/default_member_functions:default_member_functions needs [//features:wrapper] for std::basic_string<char, std::char_traits<char>, std::allocator<char>> (incomplete type)

// Generated from: nowhere/llvm/src/libcxx/include/__fwd/string.h;l=42
// Error while generating bindings for class 'std::basic_string':
// Unsupported type 'wchar_t': Unsupported builtin type

// Generated from: nowhere/llvm/src/libcxx/include/__fwd/string.h;l=42
// Error while generating bindings for class 'std::basic_string':
// Unsupported type 'wchar_t': Unsupported builtin type

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN10UncopyableC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN21UncopyableDespiteDeclD1Ev<'a>(
            __this: ::core::pin::Pin<&'a mut crate::UncopyableDespiteDecl>,
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
