// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/assume_lifetimes:string_view
// Features: assume_lifetimes, supported, types

#![rustfmt::skip]
#![feature(custom_inner_attributes)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![deny(warnings)]

/// Generated from: rs_bindings_from_cc/test/assume_lifetimes/string_view.h;l=12
#[inline(always)]
pub fn string_view_sink<'s>(mut s: ::cc_std::std::string_view<'s>) {
    unsafe {
        crate::detail::__rust_thunk___Z16string_view_sinkNSt3__u17basic_string_viewIcNS_11char_traitsIcEEEE(&mut s)
    }
}

/// Generated from: rs_bindings_from_cc/test/assume_lifetimes/string_view.h;l=13
#[inline(always)]
pub fn string_view_return<'s>(
    mut s: ::cc_std::std::string_view<'s>,
) -> ::cc_std::std::string_view<'s> {
    unsafe {
        let mut __return = ::core::mem::MaybeUninit::<::cc_std::std::string_view<'s>>::uninit();
        crate::detail::__rust_thunk___Z18string_view_returnNSt3__u17basic_string_viewIcNS_11char_traitsIcEEEE(&raw mut __return as*mut::core::ffi::c_void,&mut s);
        __return.assume_init()
    }
}

/// Generated from: rs_bindings_from_cc/test/assume_lifetimes/string_view.h;l=14
#[inline(always)]
pub fn ambiguous_string_view_return<'a, 'b>(
    mut a: ::cc_std::std::string_view<'a>,
    mut b: ::cc_std::std::string_view<'b>,
) -> ::cc_std::std::__u::raw_string_view {
    unsafe {
        let mut __return =
            ::core::mem::MaybeUninit::<::cc_std::std::__u::raw_string_view>::uninit();
        crate::detail::__rust_thunk___Z28ambiguous_string_view_returnNSt3__u17basic_string_viewIcNS_11char_traitsIcEEEES3_(&raw mut __return as*mut::core::ffi::c_void,&mut a,&mut b);
        __return.assume_init()
    }
}

/// Generated from: rs_bindings_from_cc/test/assume_lifetimes/string_view.h;l=16
#[inline(always)]
pub fn explicit_lifetime_string_view<'a>(mut x: ::cc_std::std::string_view<'a>) {
    unsafe {
        crate::detail::__rust_thunk___Z29explicit_lifetime_string_viewNSt3__u17basic_string_viewIcNS_11char_traitsIcEEEE(&mut x)
    }
}

/// Generated from: rs_bindings_from_cc/test/assume_lifetimes/string_view.h;l=17
#[inline(always)]
pub fn unambiguous_string_view_return_annotated<'a>(
    mut x: ::cc_std::std::string_view<'a>,
    mut y: ::cc_std::std::string_view<'a>,
) -> ::cc_std::std::string_view<'a> {
    unsafe {
        let mut __return = ::core::mem::MaybeUninit::<::cc_std::std::string_view<'a>>::uninit();
        crate::detail::__rust_thunk___Z40unambiguous_string_view_return_annotatedNSt3__u17basic_string_viewIcNS_11char_traitsIcEEEES3_(&raw mut __return as*mut::core::ffi::c_void,&mut x,&mut y);
        __return.assume_init()
    }
}

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

// error: class `std::basic_ostream<char, std::char_traits<char>>` could not be bound
//   incomplete type

// error: class `std::basic_ostream<wchar_t, std::char_traits<wchar_t>>` could not be bound
//   incomplete type

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=287
// error: class `std::basic_string_view<char32_t, std::char_traits<char32_t>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=287
// error: class `std::basic_string_view<char16_t, std::char_traits<char16_t>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=287
// error: class `std::basic_string_view<char8_t, std::char_traits<char8_t>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__iterator/reverse_iterator.h;l=41
// error: class `std::reverse_iterator<const char32_t *>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__iterator/reverse_iterator.h;l=41
// error: class `std::reverse_iterator<const char16_t *>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__iterator/reverse_iterator.h;l=41
// error: class `std::reverse_iterator<const char8_t *>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__iterator/reverse_iterator.h;l=41
// error: class `std::reverse_iterator<const char *>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__iterator/reverse_iterator.h;l=41
// error: class `std::reverse_iterator<const wchar_t *>` could not be bound
//   template instantiation is not yet supported

// error: class `std::basic_filebuf<char, std::char_traits<char>>` could not be bound
//   incomplete type

// error: class `std::basic_filebuf<wchar_t, std::char_traits<wchar_t>>` could not be bound
//   incomplete type

// error: class `std::basic_ifstream<char, std::char_traits<char>>` could not be bound
//   incomplete type

// error: class `std::basic_ifstream<wchar_t, std::char_traits<wchar_t>>` could not be bound
//   incomplete type

// error: class `std::basic_ofstream<char, std::char_traits<char>>` could not be bound
//   incomplete type

// error: class `std::basic_ofstream<wchar_t, std::char_traits<wchar_t>>` could not be bound
//   incomplete type

// error: class `std::basic_fstream<char, std::char_traits<char>>` could not be bound
//   incomplete type

// error: class `std::basic_fstream<wchar_t, std::char_traits<wchar_t>>` could not be bound
//   incomplete type

// error: class `std::basic_ios<char, std::char_traits<char>>` could not be bound
//   incomplete type

// error: class `std::basic_ios<wchar_t, std::char_traits<wchar_t>>` could not be bound
//   incomplete type

// error: class `std::basic_istream<char, std::char_traits<char>>` could not be bound
//   incomplete type

// error: class `std::basic_istream<wchar_t, std::char_traits<wchar_t>>` could not be bound
//   incomplete type

// error: class `std::basic_iostream<char, std::char_traits<char>>` could not be bound
//   incomplete type

// error: class `std::basic_iostream<wchar_t, std::char_traits<wchar_t>>` could not be bound
//   incomplete type

// error: class `std::basic_stringbuf<char, std::char_traits<char>, std::allocator<char>>` could not be bound
//   incomplete type

// error: class `std::basic_stringbuf<wchar_t, std::char_traits<wchar_t>, std::allocator<wchar_t>>` could not be bound
//   incomplete type

// error: class `std::basic_istringstream<char, std::char_traits<char>, std::allocator<char>>` could not be bound
//   incomplete type

// error: class `std::basic_istringstream<wchar_t, std::char_traits<wchar_t>, std::allocator<wchar_t>>` could not be bound
//   incomplete type

// error: class `std::basic_ostringstream<char, std::char_traits<char>, std::allocator<char>>` could not be bound
//   incomplete type

// error: class `std::basic_ostringstream<wchar_t, std::char_traits<wchar_t>, std::allocator<wchar_t>>` could not be bound
//   incomplete type

// error: class `std::basic_stringstream<char, std::char_traits<char>, std::allocator<char>>` could not be bound
//   incomplete type

// error: class `std::basic_stringstream<wchar_t, std::char_traits<wchar_t>, std::allocator<wchar_t>>` could not be bound
//   incomplete type

// error: class `std::basic_streambuf<char, std::char_traits<char>>` could not be bound
//   incomplete type

// error: class `std::basic_streambuf<wchar_t, std::char_traits<wchar_t>>` could not be bound
//   incomplete type

// error: class `std::fpos<__mbstate_t>` could not be bound
//   incomplete type

// Generated from: nowhere/llvm/src/libcxx/include/__string/char_traits.h;l=81
// error: struct `std::char_traits<char>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__string/char_traits.h;l=180
// error: struct `std::__char_traits_base<char32_t, unsigned int, 4294967295U>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__string/char_traits.h;l=180
// error: struct `std::__char_traits_base<char16_t, unsigned short, (unsigned short)65535>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__string/char_traits.h;l=180
// error: struct `std::__char_traits_base<char8_t, unsigned int, 4294967295U>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__string/char_traits.h;l=180
// error: struct `std::__char_traits_base<wchar_t, unsigned int, 4294967295U>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__string/char_traits.h;l=247
// error: struct `std::char_traits<wchar_t>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__string/char_traits.h;l=270
// error: struct `std::char_traits<char8_t>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__string/char_traits.h;l=289
// error: struct `std::char_traits<char16_t>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__string/char_traits.h;l=324
// error: struct `std::char_traits<char32_t>` could not be bound
//   template instantiation is not yet supported

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___Z16string_view_sinkNSt3__u17basic_string_viewIcNS_11char_traitsIcEEEE<
            's,
        >(
            s: &mut ::cc_std::std::string_view<'s>,
        );
        pub(crate) unsafe fn __rust_thunk___Z18string_view_returnNSt3__u17basic_string_viewIcNS_11char_traitsIcEEEE<
            's,
        >(
            __return: *mut ::core::ffi::c_void,
            s: &mut ::cc_std::std::string_view<'s>,
        );
        pub(crate) unsafe fn __rust_thunk___Z28ambiguous_string_view_returnNSt3__u17basic_string_viewIcNS_11char_traitsIcEEEES3_<
            'a,
            'b,
        >(
            __return: *mut ::core::ffi::c_void,
            a: &mut ::cc_std::std::string_view<'a>,
            b: &mut ::cc_std::std::string_view<'b>,
        );
        pub(crate) unsafe fn __rust_thunk___Z29explicit_lifetime_string_viewNSt3__u17basic_string_viewIcNS_11char_traitsIcEEEE<
            'a,
        >(
            x: &mut ::cc_std::std::string_view<'a>,
        );
        pub(crate) unsafe fn __rust_thunk___Z40unambiguous_string_view_return_annotatedNSt3__u17basic_string_viewIcNS_11char_traitsIcEEEES3_<
            'a,
        >(
            __return: *mut ::core::ffi::c_void,
            x: &mut ::cc_std::std::string_view<'a>,
            y: &mut ::cc_std::std::string_view<'a>,
        );
    }
}
