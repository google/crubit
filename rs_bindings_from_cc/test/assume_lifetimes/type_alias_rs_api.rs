// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/assume_lifetimes:type_alias

#![rustfmt::skip]
#![feature(cfi_encoding, custom_inner_attributes, impl_trait_in_assoc_type, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![allow(deprecated)]
#![allow(unknown_lints, suspicious_runtime_symbol_definitions)]
#![deny(warnings)]
#[::ctor::recursively_pinned(PinnedDrop)]
#[cfi_encoding = "13TypeAliasCtor"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=TypeAliasCtor
pub struct TypeAliasCtor {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 1],
}
impl !Send for TypeAliasCtor {}
impl !Sync for TypeAliasCtor {}
unsafe impl ::cxx::ExternType for TypeAliasCtor {
    type Id = ::cxx::type_id!("TypeAliasCtor");
    type Kind = ::cxx::kind::Opaque;
}

impl<'__param_0> ::ctor::CtorNew<&'__param_0 Self> for TypeAliasCtor {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__param_0>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: &'__param_0 Self) -> Self::CtorType {
        let mut __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk___ZN13TypeAliasCtorC1ERKS_(
                    __crubit_dest as *mut ::core::ffi::c_void,
                    __param_0,
                );
            })
        }
    }
}
impl<'__param_0> ::ctor::CtorNew<(&'__param_0 Self,)> for TypeAliasCtor {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__param_0>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (&'__param_0 Self,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'__param_0 Self>>::ctor_new(arg)
    }
}

impl<'__param_0> ::ctor::Assign<&'__param_0 Self> for TypeAliasCtor {
    #[inline(always)]
    fn assign<'__this>(self: ::core::pin::Pin<&'__this mut Self>, __param_0: &'__param_0 Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN13TypeAliasCtoraSERKS_(self, __param_0);
        }
    }
}

impl<'a> ::ctor::CtorNew<::cc_std::std::string_view<'a>> for TypeAliasCtor {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'a>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::cc_std::std::string_view<'a>) -> Self::CtorType {
        let mut a = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk___ZN13TypeAliasCtorC1ENSt3__u17basic_string_viewIcNS0_11char_traitsIcEEEE(__crubit_dest as*mut::core::ffi::c_void,&mut a);
            })
        }
    }
}
impl<'a> ::ctor::CtorNew<(::cc_std::std::string_view<'a>,)> for TypeAliasCtor {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'a>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (::cc_std::std::string_view<'a>,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::cc_std::std::string_view<'a>>>::ctor_new(arg)
    }
}

impl ::ctor::PinnedDrop for TypeAliasCtor {
    #[inline(always)]
    unsafe fn pinned_drop<'__this>(self: ::core::pin::Pin<&'__this mut Self>) {
        unsafe { crate::detail::__rust_thunk___ZN13TypeAliasCtorD1Ev(self) }
    }
}

// error: class `std::basic_string<char32_t, struct std::char_traits<char32_t>, class std::pmr::polymorphic_allocator<char32_t>>` could not be bound
//   incomplete type

// error: class `std::basic_string<char32_t, struct std::char_traits<char32_t>, class std::allocator<char32_t>>` could not be bound
//   incomplete type

// error: class `std::basic_string<char16_t, struct std::char_traits<char16_t>, class std::pmr::polymorphic_allocator<char16_t>>` could not be bound
//   incomplete type

// error: class `std::basic_string<char16_t, struct std::char_traits<char16_t>, class std::allocator<char16_t>>` could not be bound
//   incomplete type

// error: class `std::basic_string<char, struct std::char_traits<char>, class std::pmr::polymorphic_allocator<char>>` could not be bound
//   incomplete type

// error: class `std::basic_string<char, struct std::char_traits<char>, class std::allocator<char>>` could not be bound
//   incomplete type

// error: class `std::basic_ostream<char, struct std::char_traits<char>>` could not be bound
//   incomplete type

// error: class `std::basic_ostream<wchar_t, struct std::char_traits<wchar_t>>` could not be bound
//   incomplete type

// error: class `std::basic_filebuf<char, struct std::char_traits<char>>` could not be bound
//   incomplete type

// error: class `std::basic_filebuf<wchar_t, struct std::char_traits<wchar_t>>` could not be bound
//   incomplete type

// error: class `std::basic_ifstream<char, struct std::char_traits<char>>` could not be bound
//   incomplete type

// error: class `std::basic_ifstream<wchar_t, struct std::char_traits<wchar_t>>` could not be bound
//   incomplete type

// error: class `std::basic_ofstream<char, struct std::char_traits<char>>` could not be bound
//   incomplete type

// error: class `std::basic_ofstream<wchar_t, struct std::char_traits<wchar_t>>` could not be bound
//   incomplete type

// error: class `std::basic_fstream<char, struct std::char_traits<char>>` could not be bound
//   incomplete type

// error: class `std::basic_fstream<wchar_t, struct std::char_traits<wchar_t>>` could not be bound
//   incomplete type

// error: class `std::basic_ios<char, struct std::char_traits<char>>` could not be bound
//   incomplete type

// error: class `std::basic_ios<wchar_t, struct std::char_traits<wchar_t>>` could not be bound
//   incomplete type

// error: class `std::basic_istream<char, struct std::char_traits<char>>` could not be bound
//   incomplete type

// error: class `std::basic_istream<wchar_t, struct std::char_traits<wchar_t>>` could not be bound
//   incomplete type

// error: class `std::basic_iostream<char, struct std::char_traits<char>>` could not be bound
//   incomplete type

// error: class `std::basic_iostream<wchar_t, struct std::char_traits<wchar_t>>` could not be bound
//   incomplete type

// error: class `std::basic_stringbuf<char, struct std::char_traits<char>, class std::allocator<char>>` could not be bound
//   incomplete type

// error: class `std::basic_stringbuf<wchar_t, struct std::char_traits<wchar_t>, class std::allocator<wchar_t>>` could not be bound
//   incomplete type

// error: class `std::basic_istringstream<char, struct std::char_traits<char>, class std::allocator<char>>` could not be bound
//   incomplete type

// error: class `std::basic_istringstream<wchar_t, struct std::char_traits<wchar_t>, class std::allocator<wchar_t>>` could not be bound
//   incomplete type

// error: class `std::basic_ostringstream<char, struct std::char_traits<char>, class std::allocator<char>>` could not be bound
//   incomplete type

// error: class `std::basic_ostringstream<wchar_t, struct std::char_traits<wchar_t>, class std::allocator<wchar_t>>` could not be bound
//   incomplete type

// error: class `std::basic_stringstream<char, struct std::char_traits<char>, class std::allocator<char>>` could not be bound
//   incomplete type

// error: class `std::basic_stringstream<wchar_t, struct std::char_traits<wchar_t>, class std::allocator<wchar_t>>` could not be bound
//   incomplete type

// error: class `std::basic_streambuf<char, struct std::char_traits<char>>` could not be bound
//   incomplete type

// error: class `std::basic_streambuf<wchar_t, struct std::char_traits<wchar_t>>` could not be bound
//   incomplete type

// error: class `std::fpos<__mbstate_t>` could not be bound
//   incomplete type

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN13TypeAliasCtorC1ERKS_<'__param_0>(
            __this: *mut ::core::ffi::c_void,
            __param_0: &'__param_0 crate::TypeAliasCtor,
        );
        pub(crate) unsafe fn __rust_thunk___ZN13TypeAliasCtoraSERKS_<'__param_0, '__this>(
            __this: ::core::pin::Pin<&'__this mut crate::TypeAliasCtor>,
            __param_0: &'__param_0 crate::TypeAliasCtor,
        ) -> ::core::pin::Pin<&'__this mut crate::TypeAliasCtor>;
        pub(crate) unsafe fn __rust_thunk___ZN13TypeAliasCtorC1ENSt3__u17basic_string_viewIcNS0_11char_traitsIcEEEE<
            'a,
        >(
            __this: *mut ::core::ffi::c_void,
            a: &mut ::cc_std::std::string_view<'a>,
        );
        #[link_name = "_ZN13TypeAliasCtorD1Ev"]
        pub(crate) unsafe fn __rust_thunk___ZN13TypeAliasCtorD1Ev<'__this>(
            __this: ::core::pin::Pin<&'__this mut crate::TypeAliasCtor>,
        );
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::TypeAliasCtor>() == 1);
    assert!(::core::mem::align_of::<crate::TypeAliasCtor>() == 1);
    static_assertions::assert_impl_all!(crate::TypeAliasCtor: Drop);
    static_assertions::assert_not_impl_any!(crate::TypeAliasCtor: Copy);
};
