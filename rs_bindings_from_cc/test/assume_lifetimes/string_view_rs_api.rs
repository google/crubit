// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/assume_lifetimes:string_view
// Features: assume_lifetimes, supported

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

/// Generated from: rs_bindings_from_cc/test/assume_lifetimes/string_view.h;l=12
#[inline(always)]
pub fn string_view_sink(mut s: ::cc_std::std::__u::raw_string_view) {
    unsafe {
        crate::detail::__rust_thunk___Z16string_view_sinkNSt3__u17basic_string_viewIcNS_11char_traitsIcEEEE(&mut s)
    }
}

/// Generated from: rs_bindings_from_cc/test/assume_lifetimes/string_view.h;l=13
#[inline(always)]
pub fn string_view_return(
    mut s: ::cc_std::std::__u::raw_string_view,
) -> ::cc_std::std::__u::raw_string_view {
    unsafe {
        let mut __return =
            ::core::mem::MaybeUninit::<::cc_std::std::__u::raw_string_view>::uninit();
        crate::detail::__rust_thunk___Z18string_view_returnNSt3__u17basic_string_viewIcNS_11char_traitsIcEEEE(&raw mut __return as*mut::core::ffi::c_void,&mut s);
        __return.assume_init()
    }
}

/// Generated from: rs_bindings_from_cc/test/assume_lifetimes/string_view.h;l=14
#[inline(always)]
pub fn ambiguous_string_view_return(
    mut a: ::cc_std::std::__u::raw_string_view,
    mut b: ::cc_std::std::__u::raw_string_view,
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
pub fn explicit_lifetime_string_view(
    mut x: ::cc_std::__CcTemplateInstNSt3__u17basic_string_viewIcNS_11char_traitsIcEEEE,
) {
    unsafe {
        crate::detail::__rust_thunk___Z29explicit_lifetime_string_viewNSt3__u17basic_string_viewIcNS_11char_traitsIcEEEE(&mut x)
    }
}

/// Generated from: rs_bindings_from_cc/test/assume_lifetimes/string_view.h;l=17
#[inline(always)]
pub fn unambiguous_string_view_return_annotated(
    mut x: ::cc_std::__CcTemplateInstNSt3__u17basic_string_viewIcNS_11char_traitsIcEEEE,
    mut y: ::cc_std::__CcTemplateInstNSt3__u17basic_string_viewIcNS_11char_traitsIcEEEE,
) -> ::cc_std::std::__u::raw_string_view {
    unsafe {
        let mut __return =
            ::core::mem::MaybeUninit::<::cc_std::std::__u::raw_string_view>::uninit();
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

/// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=287
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=std :: basic_string_view < wchar_t , std :: char_traits < wchar_t >>
pub struct __CcTemplateInstNSt3__u17basic_string_viewIwNS_11char_traitsIwEEEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) __data_: [::core::mem::MaybeUninit<u8>; 8],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) __size_: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for __CcTemplateInstNSt3__u17basic_string_viewIwNS_11char_traitsIwEEEE {}
impl !Sync for __CcTemplateInstNSt3__u17basic_string_viewIwNS_11char_traitsIwEEEE {}
forward_declare::unsafe_define!(
    forward_declare::symbol!(
        "std :: basic_string_view < wchar_t , std :: char_traits < wchar_t >>"
    ),
    crate::__CcTemplateInstNSt3__u17basic_string_viewIwNS_11char_traitsIwEEEE
);

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=290
// Error while generating bindings for type alias 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::traits_type':
// Can't generate bindings for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::traits_type, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/assume_lifetimes:string_view needs [//features:wrapper] for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::traits_type (error: Can't generate bindings for std::char_traits<wchar_t>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/assume_lifetimes:string_view needs [//features:wrapper] for std::char_traits<wchar_t> (crate::__CcTemplateInstNSt3__u11char_traitsIwEE is a template instantiation))

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=291
// Error while generating bindings for type alias 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::value_type':
// Can't generate bindings for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::value_type due to missing bindings for its dependency: Unsupported type 'wchar_t': Unsupported type 'wchar_t': Unsupported builtin type

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=292
// Error while generating bindings for type alias 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::pointer':
// Can't generate bindings for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::pointer due to missing bindings for its dependency: Unsupported type 'wchar_t': Unsupported type 'wchar_t': Unsupported builtin type

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=293
// Error while generating bindings for type alias 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::const_pointer':
// Can't generate bindings for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::const_pointer due to missing bindings for its dependency: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=294
// Error while generating bindings for type alias 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::reference':
// Can't generate bindings for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::reference due to missing bindings for its dependency: Unsupported type 'wchar_t': Unsupported type 'wchar_t': Unsupported builtin type

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=295
// Error while generating bindings for type alias 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::const_reference':
// Can't generate bindings for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::const_reference due to missing bindings for its dependency: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=301
// Error while generating bindings for type alias 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::const_iterator':
// Can't generate bindings for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::const_iterator due to missing bindings for its dependency: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=303
// Error while generating bindings for type alias 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::iterator':
// Can't generate bindings for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::iterator due to missing bindings for its dependency: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=304
// Error while generating bindings for type alias 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::const_reverse_iterator':
// Can't generate bindings for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::const_reverse_iterator, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/assume_lifetimes:string_view needs [//features:wrapper] for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::const_reverse_iterator (error: Can't generate bindings for std::reverse_iterator<const wchar_t *>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/assume_lifetimes:string_view needs [//features:wrapper] for std::reverse_iterator<const wchar_t *> (crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKwEE is a template instantiation))

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=305
// Error while generating bindings for type alias 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::reverse_iterator':
// Can't generate bindings for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::reverse_iterator, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/assume_lifetimes:string_view needs [//features:wrapper] for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::reverse_iterator (error: Unsupported type alias std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::const_reverse_iterator)

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=306
// Error while generating bindings for type alias 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::size_type':
// Can't generate bindings for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::size_type, because it is unsupported: b/485949049: type definitions nested inside templated records are not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=307
// Error while generating bindings for type alias 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::difference_type':
// Can't generate bindings for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::difference_type, because it is unsupported: b/485949049: type definitions nested inside templated records are not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=308
// Error while generating bindings for global variable 'std::basic_string_view<wchar_t>::npos':
// static data members are not supported

// Generated from: nowhere/llvm/src/libcxx/include/__config;l=154
// Expanded at: nowhere/llvm/src/libcxx/include/string_view;l=320
// Error while generating bindings for constructor 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::std::basic_string_view<wchar_t, std::char_traits<wchar_t>>':
// Can't generate bindings for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::std::basic_string_view<wchar_t, std::char_traits<wchar_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/assume_lifetimes:string_view needs [//features:experimental] for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::std::basic_string_view<wchar_t, std::char_traits<wchar_t>> (b/248542210: template instantiation of member function cannot reliably get bindings)

// Generated from: nowhere/llvm/src/libcxx/include/__configuration/attributes.h;l=86
// Expanded at: nowhere/llvm/src/libcxx/include/string_view;l=322
// Error while generating bindings for constructor 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::std::basic_string_view<wchar_t, std::char_traits<wchar_t>>':
// Can't generate bindings for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::std::basic_string_view<wchar_t, std::char_traits<wchar_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/assume_lifetimes:string_view needs [//features:experimental] for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::std::basic_string_view<wchar_t, std::char_traits<wchar_t>> (b/248542210: template instantiation of member function cannot reliably get bindings)

// Generated from: nowhere/llvm/src/libcxx/include/__configuration/attributes.h;l=86
// Expanded at: nowhere/llvm/src/libcxx/include/string_view;l=324
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::operator=':
// Can't generate bindings for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::operator=, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/assume_lifetimes:string_view needs [//features:experimental] for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::operator= (b/248542210: template instantiation of member function cannot reliably get bindings)

// Generated from: nowhere/llvm/src/libcxx/include/__config;l=154
// Expanded at: nowhere/llvm/src/libcxx/include/string_view;l=326
// Error while generating bindings for constructor 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::std::basic_string_view<wchar_t, std::char_traits<wchar_t>>':
// Parameter #0 is not supported: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=342
// Error while generating bindings for function 'std::basic_string_view<wchar_t>::basic_string_view':
// Function templates are not supported yet

// Generated from: nowhere/llvm/src/libcxx/include/__config;l=154
// Expanded at: nowhere/llvm/src/libcxx/include/string_view;l=361
// Error while generating bindings for constructor 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::std::basic_string_view<wchar_t, std::char_traits<wchar_t>>':
// Parameter #0 is not supported: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=370
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::begin':
// Return type is not supported: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=372
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::end':
// Return type is not supported: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=374
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::cbegin':
// Return type is not supported: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=382
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::cend':
// Return type is not supported: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=390
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::rbegin':
// Cannot use an error type by value: Unsupported type alias std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::const_reverse_iterator

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=395
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::rend':
// Cannot use an error type by value: Unsupported type alias std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::const_reverse_iterator

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=399
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::crbegin':
// Cannot use an error type by value: Unsupported type alias std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::const_reverse_iterator

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=404
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::crend':
// Cannot use an error type by value: Unsupported type alias std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::const_reverse_iterator

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=409
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::size':
// Return type is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=411
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::length':
// Return type is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=413
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::max_size':
// Return type is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=417
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::empty':
// Can't generate bindings for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::empty, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/assume_lifetimes:string_view needs [//features:experimental] for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::empty ([[nodiscard]] attribute)

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=420
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::operator[]':
// Parameter #0 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'
// Return type is not supported: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=425
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::at':
// Parameter #0 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'
// Return type is not supported: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=429
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::front':
// Return type is not supported: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=433
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::back':
// Return type is not supported: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=437
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::data':
// Return type is not supported: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type

// Generated from: nowhere/llvm/src/libcxx/include/__config;l=339
// Expanded at: nowhere/llvm/src/libcxx/include/string_view;l=440
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::remove_prefix':
// Parameter #0 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Generated from: nowhere/llvm/src/libcxx/include/__config;l=339
// Expanded at: nowhere/llvm/src/libcxx/include/string_view;l=446
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::remove_suffix':
// Parameter #0 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Generated from: nowhere/llvm/src/libcxx/include/__config;l=339
// Expanded at: nowhere/llvm/src/libcxx/include/string_view;l=451
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::swap':
// Can't generate bindings for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::swap, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/assume_lifetimes:string_view needs [//features:experimental] for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::swap (b/248542210: template instantiation of member function cannot reliably get bindings)

// Generated from: nowhere/llvm/src/libcxx/include/__configuration/attributes.h;l=86
// Expanded at: nowhere/llvm/src/libcxx/include/string_view;l=461
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::copy':
// Parameter #0 is not supported: Unsupported type 'wchar_t': Unsupported type 'wchar_t': Unsupported builtin type
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'
// Parameter #2 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=470
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::substr':
// Parameter #0 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=486
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::compare':
// Can't generate bindings for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::compare, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/assume_lifetimes:string_view needs [//features:experimental] for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::compare ([[nodiscard]] attribute)

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=494
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::compare':
// Parameter #0 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=499
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::compare':
// Parameter #0 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'
// Parameter #3 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'
// Parameter #4 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=504
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::compare':
// Parameter #0 is not supported: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=509
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::compare':
// Parameter #0 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'
// Parameter #2 is not supported: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=514
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::compare':
// Parameter #0 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'
// Parameter #2 is not supported: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type
// Parameter #3 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=521
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::find':
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=526
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::find':
// Parameter #0 is not supported: Unsupported type 'wchar_t': Unsupported type 'wchar_t': Unsupported builtin type
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=531
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::find':
// Parameter #0 is not supported: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'
// Parameter #2 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=538
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::find':
// Parameter #0 is not supported: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=546
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::rfind':
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=551
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::rfind':
// Parameter #0 is not supported: Unsupported type 'wchar_t': Unsupported type 'wchar_t': Unsupported builtin type
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=556
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::rfind':
// Parameter #0 is not supported: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'
// Parameter #2 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=563
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::rfind':
// Parameter #0 is not supported: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=571
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::find_first_of':
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=577
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::find_first_of':
// Parameter #0 is not supported: Unsupported type 'wchar_t': Unsupported type 'wchar_t': Unsupported builtin type
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=582
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::find_first_of':
// Parameter #0 is not supported: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'
// Parameter #2 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=589
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::find_first_of':
// Parameter #0 is not supported: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=597
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::find_last_of':
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=603
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::find_last_of':
// Parameter #0 is not supported: Unsupported type 'wchar_t': Unsupported type 'wchar_t': Unsupported builtin type
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=608
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::find_last_of':
// Parameter #0 is not supported: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'
// Parameter #2 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=615
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::find_last_of':
// Parameter #0 is not supported: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=623
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::find_first_not_of':
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=629
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::find_first_not_of':
// Parameter #0 is not supported: Unsupported type 'wchar_t': Unsupported type 'wchar_t': Unsupported builtin type
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=634
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::find_first_not_of':
// Parameter #0 is not supported: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'
// Parameter #2 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=641
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::find_first_not_of':
// Parameter #0 is not supported: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=649
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::find_last_not_of':
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=655
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::find_last_not_of':
// Parameter #0 is not supported: Unsupported type 'wchar_t': Unsupported type 'wchar_t': Unsupported builtin type
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=660
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::find_last_not_of':
// Parameter #0 is not supported: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'
// Parameter #2 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=667
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::find_last_not_of':
// Parameter #0 is not supported: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=675
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::starts_with':
// Can't generate bindings for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::starts_with, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/assume_lifetimes:string_view needs [//features:experimental] for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::starts_with ([[nodiscard]] attribute)

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=679
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::starts_with':
// Parameter #0 is not supported: Unsupported type alias std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::value_type

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=683
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::starts_with':
// Parameter #0 is not supported: Unsupported type alias std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::value_type

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=688
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::ends_with':
// Can't generate bindings for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::ends_with, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/assume_lifetimes:string_view needs [//features:experimental] for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::ends_with ([[nodiscard]] attribute)

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=692
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::ends_with':
// Parameter #0 is not supported: Unsupported type alias std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::value_type

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=696
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::ends_with':
// Parameter #0 is not supported: Unsupported type alias std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::value_type

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
        pub(crate) unsafe fn __rust_thunk___Z16string_view_sinkNSt3__u17basic_string_viewIcNS_11char_traitsIcEEEE(
            s: &mut ::cc_std::std::__u::raw_string_view,
        );
        pub(crate) unsafe fn __rust_thunk___Z18string_view_returnNSt3__u17basic_string_viewIcNS_11char_traitsIcEEEE(
            __return: *mut ::core::ffi::c_void,
            s: &mut ::cc_std::std::__u::raw_string_view,
        );
        pub(crate) unsafe fn __rust_thunk___Z28ambiguous_string_view_returnNSt3__u17basic_string_viewIcNS_11char_traitsIcEEEES3_(
            __return: *mut ::core::ffi::c_void,
            a: &mut ::cc_std::std::__u::raw_string_view,
            b: &mut ::cc_std::std::__u::raw_string_view,
        );
        pub(crate) unsafe fn __rust_thunk___Z29explicit_lifetime_string_viewNSt3__u17basic_string_viewIcNS_11char_traitsIcEEEE(
            x: &mut ::cc_std::__CcTemplateInstNSt3__u17basic_string_viewIcNS_11char_traitsIcEEEE,
        );
        pub(crate) unsafe fn __rust_thunk___Z40unambiguous_string_view_return_annotatedNSt3__u17basic_string_viewIcNS_11char_traitsIcEEEES3_(
            __return: *mut ::core::ffi::c_void,
            x: &mut ::cc_std::__CcTemplateInstNSt3__u17basic_string_viewIcNS_11char_traitsIcEEEE,
            y: &mut ::cc_std::__CcTemplateInstNSt3__u17basic_string_viewIcNS_11char_traitsIcEEEE,
        );
    }
}

const _: () = {
    assert!(
        ::core::mem::size_of::<
            crate::__CcTemplateInstNSt3__u17basic_string_viewIwNS_11char_traitsIwEEEE,
        >() == 16
    );
    assert!(
        ::core::mem::align_of::<
            crate::__CcTemplateInstNSt3__u17basic_string_viewIwNS_11char_traitsIwEEEE,
        >() == 8
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u17basic_string_viewIwNS_11char_traitsIwEEEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u17basic_string_viewIwNS_11char_traitsIwEEEE: Drop);
    assert!(
        ::core::mem::offset_of!(
            crate::__CcTemplateInstNSt3__u17basic_string_viewIwNS_11char_traitsIwEEEE,
            __data_
        ) == 0
    );
    assert!(
        ::core::mem::offset_of!(
            crate::__CcTemplateInstNSt3__u17basic_string_viewIwNS_11char_traitsIwEEEE,
            __size_
        ) == 8
    );
};
