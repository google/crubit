// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/consume_absl:absl_functional
// Features: callables, supported, types

#![rustfmt::skip]
#![feature(custom_inner_attributes)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![deny(warnings)]

extern crate alloc;

pub mod absl_functional_internal {
    /// Calls the invocable and returns void.
    ///
    /// Generated from: rs_bindings_from_cc/test/consume_absl/absl_functional.h;l=13
    #[inline(always)]
    pub fn CallVoidVoid(
        f: ::alloc::boxed::Box<
            dyn ::core::ops::FnOnce() + ::core::marker::Send + ::core::marker::Sync + 'static,
        >,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN24absl_functional_internal12CallVoidVoidEN4absl12AnyInvocableIFvvOEEE(::bridge_rust::unstable_encode!(@::any_invocable::AnyInvocableAbi::<dyn::core::ops::FnOnce()+::core::marker::Send+::core::marker::Sync+'static>::new(::alloc::boxed::Box::new(||{ ::core::panic!("moved-from value") }),|managed: ::any_invocable::ManagedState,invoker: unsafe extern "C" fn()|->::alloc::boxed::Box<dyn::core::ops::FnOnce()+::core::marker::Send+::core::marker::Sync+'static>{ let c_invoker=unsafe{ ::core::mem::transmute::<unsafe extern "C" fn(),unsafe extern "C" fn(*mut::any_invocable::TypeErasedState)>(invoker) };::alloc::boxed::Box::new(move||{ unsafe{ c_invoker(managed.state()) }; }) },),::any_invocable::AnyInvocableAbi<dyn::core::ops::FnOnce()+::core::marker::Send+::core::marker::Sync+'static>,f).as_ptr()as*const u8)
        }
    }

    /// Returns an invocable that returns 42.
    ///
    /// Generated from: rs_bindings_from_cc/test/consume_absl/absl_functional.h;l=16
    #[inline(always)]
    pub fn ReturnIntVoid() -> ::alloc::boxed::Box<
        dyn ::core::ops::Fn(::ffi_11::c_int) -> ::ffi_11::c_int
            + ::core::marker::Send
            + ::core::marker::Sync
            + 'static,
    > {
        unsafe {
            ::bridge_rust::unstable_return!(@::any_invocable::AnyInvocableAbi::<dyn::core::ops::Fn(::ffi_11::c_int)->::ffi_11::c_int+::core::marker::Send+::core::marker::Sync+'static>::new(::alloc::boxed::Box::new(|_: ::ffi_11::c_int|->::ffi_11::c_int{ ::core::panic!("moved-from value") }),|managed: ::any_invocable::ManagedState,invoker: unsafe extern "C" fn()|->::alloc::boxed::Box<dyn::core::ops::Fn(::ffi_11::c_int)->::ffi_11::c_int+::core::marker::Send+::core::marker::Sync+'static>{ let c_invoker=unsafe{ ::core::mem::transmute::<unsafe extern "C" fn(),unsafe extern "C" fn(*mut::any_invocable::TypeErasedState,::ffi_11::c_int)->::ffi_11::c_int>(invoker) };::alloc::boxed::Box::new(move|param_0: ::ffi_11::c_int|->::ffi_11::c_int{ unsafe{ c_invoker(managed.state(),param_0) } }) },),::any_invocable::AnyInvocableAbi<dyn::core::ops::Fn(::ffi_11::c_int)->::ffi_11::c_int+::core::marker::Send+::core::marker::Sync+'static>,|__return_abi_buffer|{ crate::detail::__rust_thunk___ZN24absl_functional_internal13ReturnIntVoidEv(__return_abi_buffer,); })
        }
    }
}

// namespace absl_functional_internal

// Generated from: nowhere/llvm/src/libcxx/include/__type_traits/integral_constant.h;l=21
// error: struct `std::integral_constant<bool, false>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__type_traits/integral_constant.h;l=21
// error: struct `std::integral_constant<bool, true>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__iterator/iterator_traits.h;l=340
// error: struct `std::iterator_traits<char32_t *>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__iterator/iterator_traits.h;l=340
// error: struct `std::iterator_traits<char16_t *>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__iterator/iterator_traits.h;l=340
// error: struct `std::iterator_traits<const char32_t *>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__iterator/iterator_traits.h;l=340
// error: struct `std::iterator_traits<const char16_t *>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__iterator/iterator_traits.h;l=340
// error: struct `std::iterator_traits<const char *>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__iterator/iterator_traits.h;l=340
// error: struct `std::iterator_traits<char *>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__memory/pointer_traits.h;l=110
// error: struct `std::pointer_traits<char32_t *>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__memory/pointer_traits.h;l=110
// error: struct `std::pointer_traits<char16_t *>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__memory/pointer_traits.h;l=110
// error: struct `std::pointer_traits<char *>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__functional/bind.h;l=51
// error: struct `std::placeholders::__ph<10>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__functional/bind.h;l=51
// error: struct `std::placeholders::__ph<1>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__functional/bind.h;l=51
// error: struct `std::placeholders::__ph<2>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__functional/bind.h;l=51
// error: struct `std::placeholders::__ph<3>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__functional/bind.h;l=51
// error: struct `std::placeholders::__ph<4>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__functional/bind.h;l=51
// error: struct `std::placeholders::__ph<5>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__functional/bind.h;l=51
// error: struct `std::placeholders::__ph<6>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__functional/bind.h;l=51
// error: struct `std::placeholders::__ph<7>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__functional/bind.h;l=51
// error: struct `std::placeholders::__ph<8>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__functional/bind.h;l=51
// error: struct `std::placeholders::__ph<9>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__memory/allocator.h;l=62
// error: class `std::allocator<char32_t>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__memory/allocator.h;l=62
// error: class `std::allocator<char16_t>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__memory/allocator.h;l=62
// error: class `std::allocator<char>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__memory_resource/polymorphic_allocator.h;l=45
// error: class `std::pmr::polymorphic_allocator<char32_t>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__memory_resource/polymorphic_allocator.h;l=45
// error: class `std::pmr::polymorphic_allocator<char16_t>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__memory_resource/polymorphic_allocator.h;l=45
// error: class `std::pmr::polymorphic_allocator<char>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/string;l=734
// error: class `std::basic_string<char32_t, std::char_traits<char32_t>, std::pmr::polymorphic_allocator<char32_t>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/string;l=734
// error: class `std::basic_string<char32_t, std::char_traits<char32_t>, std::allocator<char32_t>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/string;l=734
// error: class `std::basic_string<char16_t, std::char_traits<char16_t>, std::pmr::polymorphic_allocator<char16_t>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/string;l=734
// error: class `std::basic_string<char16_t, std::char_traits<char16_t>, std::allocator<char16_t>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__fwd/string.h;l=42
// error: class `std::basic_string` could not be bound
//   Unsupported type 'char8_t': Unsupported builtin type

// Generated from: nowhere/llvm/src/libcxx/include/__fwd/string.h;l=42
// error: class `std::basic_string` could not be bound
//   Unsupported type 'char8_t': Unsupported builtin type

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

// Generated from: nowhere/llvm/src/libcxx/include/__memory/allocator_traits.h;l=97
// error: struct `std::__alloc_traits_difference_type<std::pmr::polymorphic_allocator<char32_t>, char32_t *, void>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__memory/allocator_traits.h;l=97
// error: struct `std::__alloc_traits_difference_type<std::pmr::polymorphic_allocator<char16_t>, char16_t *, void>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__memory/allocator_traits.h;l=97
// error: struct `std::__alloc_traits_difference_type<std::pmr::polymorphic_allocator<char>, char *, void>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__memory/allocator_traits.h;l=233
// error: struct `std::__allocator_traits_base<std::pmr::polymorphic_allocator<char32_t>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__memory/allocator_traits.h;l=233
// error: struct `std::__allocator_traits_base<std::pmr::polymorphic_allocator<char16_t>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__memory/allocator_traits.h;l=233
// error: struct `std::__allocator_traits_base<std::pmr::polymorphic_allocator<char>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__memory/allocator_traits.h;l=265
// error: struct `std::__allocator_traits_base<std::allocator<char32_t>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__memory/allocator_traits.h;l=265
// error: struct `std::__allocator_traits_base<std::allocator<char16_t>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__memory/allocator_traits.h;l=265
// error: struct `std::__allocator_traits_base<std::allocator<char>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__memory/allocator_traits.h;l=297
// error: struct `std::allocator_traits<std::pmr::polymorphic_allocator<char32_t>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__memory/allocator_traits.h;l=297
// error: struct `std::allocator_traits<std::pmr::polymorphic_allocator<char16_t>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__memory/allocator_traits.h;l=297
// error: struct `std::allocator_traits<std::pmr::polymorphic_allocator<char>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__memory/allocator_traits.h;l=297
// error: struct `std::allocator_traits<std::allocator<char32_t>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__memory/allocator_traits.h;l=297
// error: struct `std::allocator_traits<std::allocator<char16_t>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__memory/allocator_traits.h;l=297
// error: struct `std::allocator_traits<std::allocator<char>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/initializer_list;l=62
// error: class `std::initializer_list<char32_t>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/initializer_list;l=62
// error: class `std::initializer_list<char16_t>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/initializer_list;l=62
// error: class `std::initializer_list<char>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__iterator/reverse_iterator.h;l=41
// error: class `std::reverse_iterator<std::__wrap_iter<char32_t *>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__iterator/reverse_iterator.h;l=41
// error: class `std::reverse_iterator<std::__wrap_iter<char16_t *>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__iterator/reverse_iterator.h;l=41
// error: class `std::reverse_iterator<std::__wrap_iter<const char32_t *>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__iterator/reverse_iterator.h;l=41
// error: class `std::reverse_iterator<std::__wrap_iter<const char16_t *>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__iterator/reverse_iterator.h;l=41
// error: class `std::reverse_iterator<std::__wrap_iter<const char *>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__iterator/reverse_iterator.h;l=41
// error: class `std::reverse_iterator<std::__wrap_iter<char *>>` could not be bound
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

// Generated from: nowhere/llvm/src/libcxx/include/__iterator/wrap_iter.h;l=35
// error: class `std::__wrap_iter<char32_t *>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__iterator/wrap_iter.h;l=35
// error: class `std::__wrap_iter<char16_t *>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__iterator/wrap_iter.h;l=35
// error: class `std::__wrap_iter<const char32_t *>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__iterator/wrap_iter.h;l=35
// error: class `std::__wrap_iter<const char16_t *>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__iterator/wrap_iter.h;l=35
// error: class `std::__wrap_iter<const char *>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__iterator/wrap_iter.h;l=35
// error: class `std::__wrap_iter<char *>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__memory/allocate_at_least.h;l=23
// error: struct `std::__allocation_result<char32_t *, unsigned long>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__memory/allocate_at_least.h;l=23
// error: struct `std::__allocation_result<char16_t *, unsigned long>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__memory/allocate_at_least.h;l=23
// error: struct `std::__allocation_result<char *, unsigned long>` could not be bound
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

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=287
// error: class `std::basic_string_view<char32_t, std::char_traits<char32_t>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=287
// error: class `std::basic_string_view<char16_t, std::char_traits<char16_t>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=287
// error: class `std::basic_string_view<char8_t, std::char_traits<char8_t>>` could not be bound
//   template instantiation is not yet supported

// Generated from: third_party/absl/functional/internal/any_invocable.h;l=413
// error: class `absl::internal_any_invocable::CoreImpl<false, int, int>` could not be bound
//   template instantiation is not yet supported
//   template instantiation is not yet supported

// Generated from: third_party/absl/functional/internal/any_invocable.h;l=413
// error: class `absl::internal_any_invocable::CoreImpl<false, void>` could not be bound
//   template instantiation is not yet supported
//   template instantiation is not yet supported

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN24absl_functional_internal12CallVoidVoidEN4absl12AnyInvocableIFvvOEEE(
            f: *const ::core::ffi::c_uchar,
        );
        pub(crate) unsafe fn __rust_thunk___ZN24absl_functional_internal13ReturnIntVoidEv(
            __return_abi_buffer: *mut ::core::ffi::c_uchar,
        );
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __crubit_invoker___CcTemplateInstN4absl12AnyInvocableIFvvOEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fconsume_5fabsl_3aabsl_5ffunctional(
        f: *mut ::alloc::boxed::Box<
            dyn ::core::ops::FnOnce() + ::core::marker::Send + ::core::marker::Sync + 'static,
        >,
    ) {
        (unsafe {
            ::core::ptr::replace(
                f,
                ::alloc::boxed::Box::new(|| {
                    ::core::unreachable!("Called FnOnce after it was moved");
                }),
            )
        })();
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __crubit_manager___CcTemplateInstN4absl12AnyInvocableIFvvOEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fconsume_5fabsl_3aabsl_5ffunctional(
        operation: ::dyn_callable_rs::FunctionToCall,
        from: *mut ::alloc::boxed::Box<
            dyn ::core::ops::FnOnce() + ::core::marker::Send + ::core::marker::Sync + 'static,
        >,
        to: *mut ::alloc::boxed::Box<
            dyn ::core::ops::FnOnce() + ::core::marker::Send + ::core::marker::Sync + 'static,
        >,
    ) {
        ::dyn_callable_rs::manager(operation, from, to);
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __crubit_invoker___CcTemplateInstN4absl12AnyInvocableIKFiiEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fconsume_5fabsl_3aabsl_5ffunctional(
        f: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn(::ffi_11::c_int) -> ::ffi_11::c_int
                + ::core::marker::Send
                + ::core::marker::Sync
                + 'static,
        >,
        param_0: ::ffi_11::c_int,
    ) -> ::ffi_11::c_int {
        (unsafe { &*f })(param_0)
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __crubit_manager___CcTemplateInstN4absl12AnyInvocableIKFiiEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fconsume_5fabsl_3aabsl_5ffunctional(
        operation: ::dyn_callable_rs::FunctionToCall,
        from: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn(::ffi_11::c_int) -> ::ffi_11::c_int
                + ::core::marker::Send
                + ::core::marker::Sync
                + 'static,
        >,
        to: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn(::ffi_11::c_int) -> ::ffi_11::c_int
                + ::core::marker::Send
                + ::core::marker::Sync
                + 'static,
        >,
    ) {
        ::dyn_callable_rs::manager(operation, from, to);
    }
}
