// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/assume_lifetimes:string_view
// Features: assume_lifetimes, supported

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
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
// Error while generating bindings for struct 'std::integral_constant<bool, false>':
// Can't generate bindings for std::integral_constant<bool, false>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/assume_lifetimes:string_view needs [//features:wrapper] for std::integral_constant<bool, false> (crate::__CcTemplateInstNSt3__u17integral_constantIbLb0EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__type_traits/integral_constant.h;l=21
// Error while generating bindings for struct 'std::integral_constant<bool, true>':
// Can't generate bindings for std::integral_constant<bool, true>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/assume_lifetimes:string_view needs [//features:wrapper] for std::integral_constant<bool, true> (crate::__CcTemplateInstNSt3__u17integral_constantIbLb1EEE is a template instantiation)

// Error while generating bindings for class 'std::basic_string<char32_t, std::char_traits<char32_t>, std::pmr::polymorphic_allocator<char32_t>>':
// Can't generate bindings for std::basic_string<char32_t, std::char_traits<char32_t>, std::pmr::polymorphic_allocator<char32_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/assume_lifetimes:string_view needs [//features:wrapper] for std::basic_string<char32_t, std::char_traits<char32_t>, std::pmr::polymorphic_allocator<char32_t>> (incomplete type)

// Error while generating bindings for class 'std::basic_string<char32_t, std::char_traits<char32_t>, std::allocator<char32_t>>':
// Can't generate bindings for std::basic_string<char32_t, std::char_traits<char32_t>, std::allocator<char32_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/assume_lifetimes:string_view needs [//features:wrapper] for std::basic_string<char32_t, std::char_traits<char32_t>, std::allocator<char32_t>> (incomplete type)

// Error while generating bindings for class 'std::basic_string<char16_t, std::char_traits<char16_t>, std::pmr::polymorphic_allocator<char16_t>>':
// Can't generate bindings for std::basic_string<char16_t, std::char_traits<char16_t>, std::pmr::polymorphic_allocator<char16_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/assume_lifetimes:string_view needs [//features:wrapper] for std::basic_string<char16_t, std::char_traits<char16_t>, std::pmr::polymorphic_allocator<char16_t>> (incomplete type)

// Error while generating bindings for class 'std::basic_string<char16_t, std::char_traits<char16_t>, std::allocator<char16_t>>':
// Can't generate bindings for std::basic_string<char16_t, std::char_traits<char16_t>, std::allocator<char16_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/assume_lifetimes:string_view needs [//features:wrapper] for std::basic_string<char16_t, std::char_traits<char16_t>, std::allocator<char16_t>> (incomplete type)

// Generated from: nowhere/llvm/src/libcxx/include/__fwd/string.h;l=42
// Error while generating bindings for class 'std::basic_string':
// Unsupported type 'char8_t': Unsupported builtin type

// Generated from: nowhere/llvm/src/libcxx/include/__fwd/string.h;l=42
// Error while generating bindings for class 'std::basic_string':
// Unsupported type 'char8_t': Unsupported builtin type

// Error while generating bindings for class 'std::basic_string<char, std::char_traits<char>, std::pmr::polymorphic_allocator<char>>':
// Can't generate bindings for std::basic_string<char, std::char_traits<char>, std::pmr::polymorphic_allocator<char>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/assume_lifetimes:string_view needs [//features:wrapper] for std::basic_string<char, std::char_traits<char>, std::pmr::polymorphic_allocator<char>> (incomplete type)

// Error while generating bindings for class 'std::basic_string<char, std::char_traits<char>, std::allocator<char>>':
// Can't generate bindings for std::basic_string<char, std::char_traits<char>, std::allocator<char>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/assume_lifetimes:string_view needs [//features:wrapper] for std::basic_string<char, std::char_traits<char>, std::allocator<char>> (incomplete type)

// Generated from: nowhere/llvm/src/libcxx/include/__fwd/string.h;l=42
// Error while generating bindings for class 'std::basic_string':
// Unsupported type 'wchar_t': Unsupported builtin type

// Generated from: nowhere/llvm/src/libcxx/include/__fwd/string.h;l=42
// Error while generating bindings for class 'std::basic_string':
// Unsupported type 'wchar_t': Unsupported builtin type

// Error while generating bindings for class 'std::basic_ostream<char, std::char_traits<char>>':
// Can't generate bindings for std::basic_ostream<char, std::char_traits<char>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/assume_lifetimes:string_view needs [//features:wrapper] for std::basic_ostream<char, std::char_traits<char>> (incomplete type)

// Error while generating bindings for class 'std::basic_ostream<wchar_t, std::char_traits<wchar_t>>':
// Can't generate bindings for std::basic_ostream<wchar_t, std::char_traits<wchar_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/assume_lifetimes:string_view needs [//features:wrapper] for std::basic_ostream<wchar_t, std::char_traits<wchar_t>> (incomplete type)

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=287
// Error while generating bindings for class 'std::basic_string_view<char32_t, std::char_traits<char32_t>>':
// Can't generate bindings for std::basic_string_view<char32_t, std::char_traits<char32_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/assume_lifetimes:string_view needs [//features:wrapper] for std::basic_string_view<char32_t, std::char_traits<char32_t>> (crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=287
// Error while generating bindings for class 'std::basic_string_view<char16_t, std::char_traits<char16_t>>':
// Can't generate bindings for std::basic_string_view<char16_t, std::char_traits<char16_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/assume_lifetimes:string_view needs [//features:wrapper] for std::basic_string_view<char16_t, std::char_traits<char16_t>> (crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=287
// Error while generating bindings for class 'std::basic_string_view<char8_t, std::char_traits<char8_t>>':
// Can't generate bindings for std::basic_string_view<char8_t, std::char_traits<char8_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/assume_lifetimes:string_view needs [//features:wrapper] for std::basic_string_view<char8_t, std::char_traits<char8_t>> (crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__iterator/reverse_iterator.h;l=41
// Error while generating bindings for class 'std::reverse_iterator<const char32_t *>':
// Can't generate bindings for std::reverse_iterator<const char32_t *>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/assume_lifetimes:string_view needs [//features:wrapper] for std::reverse_iterator<const char32_t *> (crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDiEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__iterator/reverse_iterator.h;l=41
// Error while generating bindings for class 'std::reverse_iterator<const char16_t *>':
// Can't generate bindings for std::reverse_iterator<const char16_t *>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/assume_lifetimes:string_view needs [//features:wrapper] for std::reverse_iterator<const char16_t *> (crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDsEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__iterator/reverse_iterator.h;l=41
// Error while generating bindings for class 'std::reverse_iterator<const char8_t *>':
// Can't generate bindings for std::reverse_iterator<const char8_t *>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/assume_lifetimes:string_view needs [//features:wrapper] for std::reverse_iterator<const char8_t *> (crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDuEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__iterator/reverse_iterator.h;l=41
// Error while generating bindings for class 'std::reverse_iterator<const char *>':
// Can't generate bindings for std::reverse_iterator<const char *>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/assume_lifetimes:string_view needs [//features:wrapper] for std::reverse_iterator<const char *> (crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKcEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__iterator/reverse_iterator.h;l=41
// Error while generating bindings for class 'std::reverse_iterator<const wchar_t *>':
// Can't generate bindings for std::reverse_iterator<const wchar_t *>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/assume_lifetimes:string_view needs [//features:wrapper] for std::reverse_iterator<const wchar_t *> (crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKwEE is a template instantiation)

// Error while generating bindings for class 'std::basic_filebuf<char, std::char_traits<char>>':
// Can't generate bindings for std::basic_filebuf<char, std::char_traits<char>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/assume_lifetimes:string_view needs [//features:wrapper] for std::basic_filebuf<char, std::char_traits<char>> (incomplete type)

// Error while generating bindings for class 'std::basic_filebuf<wchar_t, std::char_traits<wchar_t>>':
// Can't generate bindings for std::basic_filebuf<wchar_t, std::char_traits<wchar_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/assume_lifetimes:string_view needs [//features:wrapper] for std::basic_filebuf<wchar_t, std::char_traits<wchar_t>> (incomplete type)

// Error while generating bindings for class 'std::basic_ifstream<char, std::char_traits<char>>':
// Can't generate bindings for std::basic_ifstream<char, std::char_traits<char>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/assume_lifetimes:string_view needs [//features:wrapper] for std::basic_ifstream<char, std::char_traits<char>> (incomplete type)

// Error while generating bindings for class 'std::basic_ifstream<wchar_t, std::char_traits<wchar_t>>':
// Can't generate bindings for std::basic_ifstream<wchar_t, std::char_traits<wchar_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/assume_lifetimes:string_view needs [//features:wrapper] for std::basic_ifstream<wchar_t, std::char_traits<wchar_t>> (incomplete type)

// Error while generating bindings for class 'std::basic_ofstream<char, std::char_traits<char>>':
// Can't generate bindings for std::basic_ofstream<char, std::char_traits<char>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/assume_lifetimes:string_view needs [//features:wrapper] for std::basic_ofstream<char, std::char_traits<char>> (incomplete type)

// Error while generating bindings for class 'std::basic_ofstream<wchar_t, std::char_traits<wchar_t>>':
// Can't generate bindings for std::basic_ofstream<wchar_t, std::char_traits<wchar_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/assume_lifetimes:string_view needs [//features:wrapper] for std::basic_ofstream<wchar_t, std::char_traits<wchar_t>> (incomplete type)

// Error while generating bindings for class 'std::basic_fstream<char, std::char_traits<char>>':
// Can't generate bindings for std::basic_fstream<char, std::char_traits<char>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/assume_lifetimes:string_view needs [//features:wrapper] for std::basic_fstream<char, std::char_traits<char>> (incomplete type)

// Error while generating bindings for class 'std::basic_fstream<wchar_t, std::char_traits<wchar_t>>':
// Can't generate bindings for std::basic_fstream<wchar_t, std::char_traits<wchar_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/assume_lifetimes:string_view needs [//features:wrapper] for std::basic_fstream<wchar_t, std::char_traits<wchar_t>> (incomplete type)

// Error while generating bindings for class 'std::basic_ios<char, std::char_traits<char>>':
// Can't generate bindings for std::basic_ios<char, std::char_traits<char>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/assume_lifetimes:string_view needs [//features:wrapper] for std::basic_ios<char, std::char_traits<char>> (incomplete type)

// Error while generating bindings for class 'std::basic_ios<wchar_t, std::char_traits<wchar_t>>':
// Can't generate bindings for std::basic_ios<wchar_t, std::char_traits<wchar_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/assume_lifetimes:string_view needs [//features:wrapper] for std::basic_ios<wchar_t, std::char_traits<wchar_t>> (incomplete type)

// Error while generating bindings for class 'std::basic_istream<char, std::char_traits<char>>':
// Can't generate bindings for std::basic_istream<char, std::char_traits<char>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/assume_lifetimes:string_view needs [//features:wrapper] for std::basic_istream<char, std::char_traits<char>> (incomplete type)

// Error while generating bindings for class 'std::basic_istream<wchar_t, std::char_traits<wchar_t>>':
// Can't generate bindings for std::basic_istream<wchar_t, std::char_traits<wchar_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/assume_lifetimes:string_view needs [//features:wrapper] for std::basic_istream<wchar_t, std::char_traits<wchar_t>> (incomplete type)

// Error while generating bindings for class 'std::basic_iostream<char, std::char_traits<char>>':
// Can't generate bindings for std::basic_iostream<char, std::char_traits<char>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/assume_lifetimes:string_view needs [//features:wrapper] for std::basic_iostream<char, std::char_traits<char>> (incomplete type)

// Error while generating bindings for class 'std::basic_iostream<wchar_t, std::char_traits<wchar_t>>':
// Can't generate bindings for std::basic_iostream<wchar_t, std::char_traits<wchar_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/assume_lifetimes:string_view needs [//features:wrapper] for std::basic_iostream<wchar_t, std::char_traits<wchar_t>> (incomplete type)

// Error while generating bindings for class 'std::basic_stringbuf<char, std::char_traits<char>, std::allocator<char>>':
// Can't generate bindings for std::basic_stringbuf<char, std::char_traits<char>, std::allocator<char>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/assume_lifetimes:string_view needs [//features:wrapper] for std::basic_stringbuf<char, std::char_traits<char>, std::allocator<char>> (incomplete type)

// Error while generating bindings for class 'std::basic_stringbuf<wchar_t, std::char_traits<wchar_t>, std::allocator<wchar_t>>':
// Can't generate bindings for std::basic_stringbuf<wchar_t, std::char_traits<wchar_t>, std::allocator<wchar_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/assume_lifetimes:string_view needs [//features:wrapper] for std::basic_stringbuf<wchar_t, std::char_traits<wchar_t>, std::allocator<wchar_t>> (incomplete type)

// Error while generating bindings for class 'std::basic_istringstream<char, std::char_traits<char>, std::allocator<char>>':
// Can't generate bindings for std::basic_istringstream<char, std::char_traits<char>, std::allocator<char>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/assume_lifetimes:string_view needs [//features:wrapper] for std::basic_istringstream<char, std::char_traits<char>, std::allocator<char>> (incomplete type)

// Error while generating bindings for class 'std::basic_istringstream<wchar_t, std::char_traits<wchar_t>, std::allocator<wchar_t>>':
// Can't generate bindings for std::basic_istringstream<wchar_t, std::char_traits<wchar_t>, std::allocator<wchar_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/assume_lifetimes:string_view needs [//features:wrapper] for std::basic_istringstream<wchar_t, std::char_traits<wchar_t>, std::allocator<wchar_t>> (incomplete type)

// Error while generating bindings for class 'std::basic_ostringstream<char, std::char_traits<char>, std::allocator<char>>':
// Can't generate bindings for std::basic_ostringstream<char, std::char_traits<char>, std::allocator<char>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/assume_lifetimes:string_view needs [//features:wrapper] for std::basic_ostringstream<char, std::char_traits<char>, std::allocator<char>> (incomplete type)

// Error while generating bindings for class 'std::basic_ostringstream<wchar_t, std::char_traits<wchar_t>, std::allocator<wchar_t>>':
// Can't generate bindings for std::basic_ostringstream<wchar_t, std::char_traits<wchar_t>, std::allocator<wchar_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/assume_lifetimes:string_view needs [//features:wrapper] for std::basic_ostringstream<wchar_t, std::char_traits<wchar_t>, std::allocator<wchar_t>> (incomplete type)

// Error while generating bindings for class 'std::basic_stringstream<char, std::char_traits<char>, std::allocator<char>>':
// Can't generate bindings for std::basic_stringstream<char, std::char_traits<char>, std::allocator<char>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/assume_lifetimes:string_view needs [//features:wrapper] for std::basic_stringstream<char, std::char_traits<char>, std::allocator<char>> (incomplete type)

// Error while generating bindings for class 'std::basic_stringstream<wchar_t, std::char_traits<wchar_t>, std::allocator<wchar_t>>':
// Can't generate bindings for std::basic_stringstream<wchar_t, std::char_traits<wchar_t>, std::allocator<wchar_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/assume_lifetimes:string_view needs [//features:wrapper] for std::basic_stringstream<wchar_t, std::char_traits<wchar_t>, std::allocator<wchar_t>> (incomplete type)

// Error while generating bindings for class 'std::basic_streambuf<char, std::char_traits<char>>':
// Can't generate bindings for std::basic_streambuf<char, std::char_traits<char>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/assume_lifetimes:string_view needs [//features:wrapper] for std::basic_streambuf<char, std::char_traits<char>> (incomplete type)

// Error while generating bindings for class 'std::basic_streambuf<wchar_t, std::char_traits<wchar_t>>':
// Can't generate bindings for std::basic_streambuf<wchar_t, std::char_traits<wchar_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/assume_lifetimes:string_view needs [//features:wrapper] for std::basic_streambuf<wchar_t, std::char_traits<wchar_t>> (incomplete type)

// Error while generating bindings for class 'std::fpos<__mbstate_t>':
// Can't generate bindings for std::fpos<__mbstate_t>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/assume_lifetimes:string_view needs [//features:wrapper] for std::fpos<__mbstate_t> (incomplete type)

// Generated from: nowhere/llvm/src/libcxx/include/__string/char_traits.h;l=81
// Error while generating bindings for struct 'std::char_traits<char>':
// Can't generate bindings for std::char_traits<char>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/assume_lifetimes:string_view needs [//features:wrapper] for std::char_traits<char> (crate::__CcTemplateInstNSt3__u11char_traitsIcEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__string/char_traits.h;l=180
// Error while generating bindings for struct 'std::__char_traits_base<char32_t, unsigned int, 4294967295U>':
// Can't generate bindings for std::__char_traits_base<char32_t, unsigned int, 4294967295U>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/assume_lifetimes:string_view needs [//features:wrapper] for std::__char_traits_base<char32_t, unsigned int, 4294967295U> (crate::__CcTemplateInstNSt3__u18__char_traits_baseIDijLj4294967295EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__string/char_traits.h;l=180
// Error while generating bindings for struct 'std::__char_traits_base<char16_t, unsigned short, (unsigned short)65535>':
// Can't generate bindings for std::__char_traits_base<char16_t, unsigned short, (unsigned short)65535>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/assume_lifetimes:string_view needs [//features:wrapper] for std::__char_traits_base<char16_t, unsigned short, (unsigned short)65535> (crate::__CcTemplateInstNSt3__u18__char_traits_baseIDstLt65535EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__string/char_traits.h;l=180
// Error while generating bindings for struct 'std::__char_traits_base<char8_t, unsigned int, 4294967295U>':
// Can't generate bindings for std::__char_traits_base<char8_t, unsigned int, 4294967295U>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/assume_lifetimes:string_view needs [//features:wrapper] for std::__char_traits_base<char8_t, unsigned int, 4294967295U> (crate::__CcTemplateInstNSt3__u18__char_traits_baseIDujLj4294967295EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__string/char_traits.h;l=180
// Error while generating bindings for struct 'std::__char_traits_base<wchar_t, unsigned int, 4294967295U>':
// Can't generate bindings for std::__char_traits_base<wchar_t, unsigned int, 4294967295U>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/assume_lifetimes:string_view needs [//features:wrapper] for std::__char_traits_base<wchar_t, unsigned int, 4294967295U> (crate::__CcTemplateInstNSt3__u18__char_traits_baseIwjLj4294967295EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__string/char_traits.h;l=247
// Error while generating bindings for struct 'std::char_traits<wchar_t>':
// Can't generate bindings for std::char_traits<wchar_t>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/assume_lifetimes:string_view needs [//features:wrapper] for std::char_traits<wchar_t> (crate::__CcTemplateInstNSt3__u11char_traitsIwEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__string/char_traits.h;l=270
// Error while generating bindings for struct 'std::char_traits<char8_t>':
// Can't generate bindings for std::char_traits<char8_t>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/assume_lifetimes:string_view needs [//features:wrapper] for std::char_traits<char8_t> (crate::__CcTemplateInstNSt3__u11char_traitsIDuEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__string/char_traits.h;l=289
// Error while generating bindings for struct 'std::char_traits<char16_t>':
// Can't generate bindings for std::char_traits<char16_t>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/assume_lifetimes:string_view needs [//features:wrapper] for std::char_traits<char16_t> (crate::__CcTemplateInstNSt3__u11char_traitsIDsEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__string/char_traits.h;l=324
// Error while generating bindings for struct 'std::char_traits<char32_t>':
// Can't generate bindings for std::char_traits<char32_t>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/assume_lifetimes:string_view needs [//features:wrapper] for std::char_traits<char32_t> (crate::__CcTemplateInstNSt3__u11char_traitsIDiEE is a template instantiation)

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
