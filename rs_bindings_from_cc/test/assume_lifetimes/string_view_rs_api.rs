// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/assume_lifetimes:string_view

#![rustfmt::skip]
#![feature(custom_inner_attributes)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![allow(deprecated)]
#![allow(unknown_lints, suspicious_runtime_symbol_definitions)]
#![deny(warnings)]
#[inline(always)]
pub fn string_view_sink<'s>(s: ::cc_std::std::string_view<'s>) {
    let mut s = ::core::mem::MaybeUninit::new(s);
    unsafe {
        crate::detail::__rust_thunk___Z16string_view_sinkNSt3__u17basic_string_viewIcNS_11char_traitsIcEEEE(s.as_mut_ptr())
    }
}

#[inline(always)]
pub fn string_view_return<'s>(s: ::cc_std::std::string_view<'s>) -> ::cc_std::std::string_view<'s> {
    let mut s = ::core::mem::MaybeUninit::new(s);
    unsafe {
        let mut __crubit_return =
            ::core::mem::MaybeUninit::<::cc_std::std::string_view<'s>>::uninit();
        crate::detail::__rust_thunk___Z18string_view_returnNSt3__u17basic_string_viewIcNS_11char_traitsIcEEEE(&raw mut __crubit_return as*mut::core::ffi::c_void,s.as_mut_ptr());
        __crubit_return.assume_init()
    }
}

#[inline(always)]
pub fn ambiguous_string_view_return<'a, 'b>(
    a: ::cc_std::std::string_view<'a>,
    b: ::cc_std::std::string_view<'b>,
) -> ::cc_std::std::__u::raw_string_view {
    let mut a = ::core::mem::MaybeUninit::new(a);
    let mut b = ::core::mem::MaybeUninit::new(b);
    unsafe {
        let mut __crubit_return =
            ::core::mem::MaybeUninit::<::cc_std::std::__u::raw_string_view>::uninit();
        crate::detail::__rust_thunk___Z28ambiguous_string_view_returnNSt3__u17basic_string_viewIcNS_11char_traitsIcEEEES3_(&raw mut __crubit_return as*mut::core::ffi::c_void,a.as_mut_ptr(),b.as_mut_ptr());
        __crubit_return.assume_init()
    }
}

#[inline(always)]
pub fn explicit_lifetime_string_view<'a>(x: ::cc_std::std::string_view<'a>) {
    let mut x = ::core::mem::MaybeUninit::new(x);
    unsafe {
        crate::detail::__rust_thunk___Z29explicit_lifetime_string_viewNSt3__u17basic_string_viewIcNS_11char_traitsIcEEEE(x.as_mut_ptr())
    }
}

#[inline(always)]
pub fn unambiguous_string_view_return_annotated<'a>(
    x: ::cc_std::std::string_view<'a>,
    y: ::cc_std::std::string_view<'a>,
) -> ::cc_std::std::string_view<'a> {
    let mut x = ::core::mem::MaybeUninit::new(x);
    let mut y = ::core::mem::MaybeUninit::new(y);
    unsafe {
        let mut __crubit_return =
            ::core::mem::MaybeUninit::<::cc_std::std::string_view<'a>>::uninit();
        crate::detail::__rust_thunk___Z40unambiguous_string_view_return_annotatedNSt3__u17basic_string_viewIcNS_11char_traitsIcEEEES3_(&raw mut __crubit_return as*mut::core::ffi::c_void,x.as_mut_ptr(),y.as_mut_ptr());
        __crubit_return.assume_init()
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
        pub(crate) unsafe fn __rust_thunk___Z16string_view_sinkNSt3__u17basic_string_viewIcNS_11char_traitsIcEEEE<
            's,
        >(
            s: *mut ::cc_std::std::string_view<'s>,
        );
        pub(crate) unsafe fn __rust_thunk___Z18string_view_returnNSt3__u17basic_string_viewIcNS_11char_traitsIcEEEE<
            's,
        >(
            __return: *mut ::core::ffi::c_void,
            s: *mut ::cc_std::std::string_view<'s>,
        );
        pub(crate) unsafe fn __rust_thunk___Z28ambiguous_string_view_returnNSt3__u17basic_string_viewIcNS_11char_traitsIcEEEES3_<
            'a,
            'b,
        >(
            __return: *mut ::core::ffi::c_void,
            a: *mut ::cc_std::std::string_view<'a>,
            b: *mut ::cc_std::std::string_view<'b>,
        );
        pub(crate) unsafe fn __rust_thunk___Z29explicit_lifetime_string_viewNSt3__u17basic_string_viewIcNS_11char_traitsIcEEEE<
            'a,
        >(
            x: *mut ::cc_std::std::string_view<'a>,
        );
        pub(crate) unsafe fn __rust_thunk___Z40unambiguous_string_view_return_annotatedNSt3__u17basic_string_viewIcNS_11char_traitsIcEEEES3_<
            'a,
        >(
            __return: *mut ::core::ffi::c_void,
            x: *mut ::cc_std::std::string_view<'a>,
            y: *mut ::cc_std::std::string_view<'a>,
        );
    }
}
