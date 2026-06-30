// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/consume_absl:absl_functional

#![rustfmt::skip]
#![feature(custom_inner_attributes)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![allow(deprecated)]
#![deny(warnings)]

extern crate alloc;

// error: class `MyOption` could not be bound
//   Class templates are not yet supported

/// Calls the invocable and returns void.
#[inline(always)]
pub fn CallVoidVoid(
    f: ::alloc::boxed::Box<
        dyn ::core::ops::FnOnce() + ::core::marker::Send + ::core::marker::Sync + 'static,
    >,
) {
    unsafe {
        crate::detail::__rust_thunk___Z12CallVoidVoidN4absl12AnyInvocableIFvvOEEE(::bridge_rust::unstable_encode!(@::any_invocable::AnyInvocableAbi::<dyn::core::ops::FnOnce()+::core::marker::Send+::core::marker::Sync+'static>::new(::alloc::boxed::Box::new(||{ ::core::panic!("moved-from value") }),|raw_any_invocable: ::cc_std::std::unique_ptr<::any_invocable::RawAnyInvocable>|->::alloc::boxed::Box<dyn::core::ops::FnOnce()+::core::marker::Send+::core::marker::Sync+'static>{ ::alloc::boxed::Box::new(move||{ unsafe{ crate::detail::__crubit_invoke_any_invocable___CcTemplateInstN4absl12AnyInvocableIFvvOEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fconsume_5fabsl_3aabsl_5ffunctional(raw_any_invocable.get()) }; }) },),::any_invocable::AnyInvocableAbi<dyn::core::ops::FnOnce()+::core::marker::Send+::core::marker::Sync+'static>,f).as_ptr()as*const u8)
    }
}

/// Returns an invocable that returns 42.
#[inline(always)]
pub fn ReturnIntMapper() -> ::alloc::boxed::Box<
    dyn ::core::ops::Fn(::ffi_11::c_int) -> ::ffi_11::c_int
        + ::core::marker::Send
        + ::core::marker::Sync
        + 'static,
> {
    unsafe {
        ::bridge_rust::unstable_return!(@::any_invocable::AnyInvocableAbi::<dyn::core::ops::Fn(::ffi_11::c_int)->::ffi_11::c_int+::core::marker::Send+::core::marker::Sync+'static>::new(::alloc::boxed::Box::new(|_: ::ffi_11::c_int|->::ffi_11::c_int{ ::core::panic!("moved-from value") }),|raw_any_invocable: ::cc_std::std::unique_ptr<::any_invocable::RawAnyInvocable>|->::alloc::boxed::Box<dyn::core::ops::Fn(::ffi_11::c_int)->::ffi_11::c_int+::core::marker::Send+::core::marker::Sync+'static>{ ::alloc::boxed::Box::new(move|param_0: ::ffi_11::c_int|->::ffi_11::c_int{ unsafe{ crate::detail::__crubit_invoke_any_invocable___CcTemplateInstN4absl12AnyInvocableIKFiiEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fconsume_5fabsl_3aabsl_5ffunctional(raw_any_invocable.get(),param_0) } }) },),::any_invocable::AnyInvocableAbi<dyn::core::ops::Fn(::ffi_11::c_int)->::ffi_11::c_int+::core::marker::Send+::core::marker::Sync+'static>,|__crubit_return_abi_buffer|{ crate::detail::__rust_thunk___Z15ReturnIntMapperv(__crubit_return_abi_buffer,); })
    }
}

/// Returns an AnyInvocable that takes a MyOption<int> and returns a
/// MyOption<int>.
#[inline(always)]
pub fn MyOptionIntMapper() -> ::alloc::boxed::Box<
    dyn ::core::ops::Fn(crate::MyOption<::ffi_11::c_int>) -> crate::MyOption<::ffi_11::c_int>
        + ::core::marker::Send
        + ::core::marker::Sync
        + 'static,
> {
    unsafe {
        ::bridge_rust::unstable_return!(@::any_invocable::AnyInvocableAbi::<dyn::core::ops::Fn(crate::MyOption<::ffi_11::c_int>)->crate::MyOption<::ffi_11::c_int>+::core::marker::Send+::core::marker::Sync+'static>::new(::alloc::boxed::Box::new(|_: crate::MyOption<::ffi_11::c_int>|->crate::MyOption<::ffi_11::c_int>{ ::core::panic!("moved-from value") }),|raw_any_invocable: ::cc_std::std::unique_ptr<::any_invocable::RawAnyInvocable>|->::alloc::boxed::Box<dyn::core::ops::Fn(crate::MyOption<::ffi_11::c_int>)->crate::MyOption<::ffi_11::c_int>+::core::marker::Send+::core::marker::Sync+'static>{ ::alloc::boxed::Box::new(move|param_0: crate::MyOption<::ffi_11::c_int>|->crate::MyOption<::ffi_11::c_int>{ ::bridge_rust::unstable_return!(@crate::MyOptionAbi(::bridge_rust::transmute_abi::<::core::ffi::c_int>()),crate::MyOptionAbi<::bridge_rust::TransmuteAbi<::core::ffi::c_int>>,|out|{ unsafe{ crate::detail::__crubit_invoke_any_invocable___CcTemplateInstN4absl12AnyInvocableIKF8MyOptionIiES2_EEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fconsume_5fabsl_3aabsl_5ffunctional(raw_any_invocable.get(),::bridge_rust::unstable_encode!(@crate::MyOptionAbi(::bridge_rust::transmute_abi::<::core::ffi::c_int>()),crate::MyOptionAbi<::bridge_rust::TransmuteAbi<::core::ffi::c_int>>,param_0).as_ptr()as*const u8,out) } }) }) },),::any_invocable::AnyInvocableAbi<dyn::core::ops::Fn(crate::MyOption<::ffi_11::c_int>)->crate::MyOption<::ffi_11::c_int>+::core::marker::Send+::core::marker::Sync+'static>,|__crubit_return_abi_buffer|{ crate::detail::__rust_thunk___Z17MyOptionIntMapperv(__crubit_return_abi_buffer,); })
    }
}

/// Returns an AnyInvocable without const qualifier, which Crubit promotes to
/// Fn in Rust.
#[inline(always)]
pub fn ReturnNonConstIntMapper() -> ::alloc::boxed::Box<
    dyn ::core::ops::Fn(::ffi_11::c_int) -> ::ffi_11::c_int
        + ::core::marker::Send
        + ::core::marker::Sync
        + 'static,
> {
    unsafe {
        ::bridge_rust::unstable_return!(@::any_invocable::AnyInvocableAbi::<dyn::core::ops::Fn(::ffi_11::c_int)->::ffi_11::c_int+::core::marker::Send+::core::marker::Sync+'static>::new(::alloc::boxed::Box::new(|_: ::ffi_11::c_int|->::ffi_11::c_int{ ::core::panic!("moved-from value") }),|raw_any_invocable: ::cc_std::std::unique_ptr<::any_invocable::RawAnyInvocable>|->::alloc::boxed::Box<dyn::core::ops::Fn(::ffi_11::c_int)->::ffi_11::c_int+::core::marker::Send+::core::marker::Sync+'static>{ ::alloc::boxed::Box::new(move|param_0: ::ffi_11::c_int|->::ffi_11::c_int{ unsafe{ crate::detail::__crubit_invoke_any_invocable___CcTemplateInstN4absl12AnyInvocableIFiiEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fconsume_5fabsl_3aabsl_5ffunctional(raw_any_invocable.get(),param_0) } }) },),::any_invocable::AnyInvocableAbi<dyn::core::ops::Fn(::ffi_11::c_int)->::ffi_11::c_int+::core::marker::Send+::core::marker::Sync+'static>,|__crubit_return_abi_buffer|{ crate::detail::__rust_thunk___Z23ReturnNonConstIntMapperv(__crubit_return_abi_buffer,); })
    }
}

// error: struct `Incomplete` could not be bound
//   incomplete type

// error: function `ReturnIncompleteMapper` could not be bound
//   Return type is not supported: Unsupported type 'absl::AnyInvocable<Incomplete (Incomplete) const>': Failed to create bindings for template specialization type absl::AnyInvocable<Incomplete (Incomplete) const>: Return type of callable is incomplete: struct Incomplete

// error: function `CallIncompleteMapper` could not be bound
//   Parameter #0 is not supported: Unsupported type 'absl::AnyInvocable<Incomplete (Incomplete) const>': Failed to create bindings for template specialization type absl::AnyInvocable<Incomplete (Incomplete) const>: Return type of callable is incomplete: struct Incomplete

// error: class `std::basic_ostream<char, std::char_traits<char>>` could not be bound
//   incomplete type

// error: class `std::basic_ostream<wchar_t, std::char_traits<wchar_t>>` could not be bound
//   incomplete type

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

// error: class `absl::internal_any_invocable::CoreImpl<false, MyOption<int>, MyOption<int>>` could not be bound
//   Crubit is not enabled on defining target:
//     third_party/absl/functional/internal/any_invocable.h
//   template instantiation is not yet supported

// error: class `absl::internal_any_invocable::CoreImpl<false, int, int>` could not be bound
//   Crubit is not enabled on defining target:
//     third_party/absl/functional/internal/any_invocable.h
//   template instantiation is not yet supported

// error: class `absl::internal_any_invocable::CoreImpl<false, void>` could not be bound
//   Crubit is not enabled on defining target:
//     third_party/absl/functional/internal/any_invocable.h
//   template instantiation is not yet supported

// error: class `absl::AnyInvocable` could not be bound
//   Return type of callable is incomplete: struct Incomplete

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___Z12CallVoidVoidN4absl12AnyInvocableIFvvOEEE(
            f: *const ::core::ffi::c_uchar,
        );
        pub(crate) unsafe fn __rust_thunk___Z15ReturnIntMapperv(
            __return_abi_buffer: *mut ::core::ffi::c_uchar,
        );
        pub(crate) unsafe fn __rust_thunk___Z17MyOptionIntMapperv(
            __return_abi_buffer: *mut ::core::ffi::c_uchar,
        );
        pub(crate) unsafe fn __rust_thunk___Z23ReturnNonConstIntMapperv(
            __return_abi_buffer: *mut ::core::ffi::c_uchar,
        );
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __crubit_invoker___CcTemplateInstN4absl12AnyInvocableIFiiEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fconsume_5fabsl_3aabsl_5ffunctional(
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
    unsafe extern "C" fn __crubit_manager___CcTemplateInstN4absl12AnyInvocableIFiiEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fconsume_5fabsl_3aabsl_5ffunctional(
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
        unsafe {
            ::dyn_callable_rs::manager(operation, from, to);
        }
    }
    unsafe extern "C" {
        pub(crate) unsafe fn __crubit_invoke_any_invocable___CcTemplateInstN4absl12AnyInvocableIFiiEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fconsume_5fabsl_3aabsl_5ffunctional(
            f: *mut ::any_invocable::RawAnyInvocable,
            param_0: ::ffi_11::c_int,
        ) -> ::ffi_11::c_int;
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
        unsafe {
            ::dyn_callable_rs::manager(operation, from, to);
        }
    }
    unsafe extern "C" {
        pub(crate) unsafe fn __crubit_invoke_any_invocable___CcTemplateInstN4absl12AnyInvocableIFvvOEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fconsume_5fabsl_3aabsl_5ffunctional(
            f: *mut ::any_invocable::RawAnyInvocable,
        );
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __crubit_invoker___CcTemplateInstN4absl12AnyInvocableIKF8MyOptionIiES2_EEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fconsume_5fabsl_3aabsl_5ffunctional(
        f: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn(
                    crate::MyOption<::ffi_11::c_int>,
                ) -> crate::MyOption<::ffi_11::c_int>
                + ::core::marker::Send
                + ::core::marker::Sync
                + 'static,
        >,
        param_0: *mut ::core::ffi::c_uchar,
        bridge_buffer: *mut ::core::ffi::c_uchar,
    ) {
        let param_0 = unsafe {
            ::bridge_rust::internal::decode(
                crate::MyOptionAbi(::bridge_rust::transmute_abi::<::core::ffi::c_int>()),
                param_0,
            )
        };
        unsafe {
            ::bridge_rust::internal::encode(
                crate::MyOptionAbi(::bridge_rust::transmute_abi::<::core::ffi::c_int>()),
                bridge_buffer,
                (unsafe { &*f })(param_0),
            )
        };
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __crubit_manager___CcTemplateInstN4absl12AnyInvocableIKF8MyOptionIiES2_EEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fconsume_5fabsl_3aabsl_5ffunctional(
        operation: ::dyn_callable_rs::FunctionToCall,
        from: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn(
                    crate::MyOption<::ffi_11::c_int>,
                ) -> crate::MyOption<::ffi_11::c_int>
                + ::core::marker::Send
                + ::core::marker::Sync
                + 'static,
        >,
        to: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn(
                    crate::MyOption<::ffi_11::c_int>,
                ) -> crate::MyOption<::ffi_11::c_int>
                + ::core::marker::Send
                + ::core::marker::Sync
                + 'static,
        >,
    ) {
        unsafe {
            ::dyn_callable_rs::manager(operation, from, to);
        }
    }
    unsafe extern "C" {
        pub(crate) unsafe fn __crubit_invoke_any_invocable___CcTemplateInstN4absl12AnyInvocableIKF8MyOptionIiES2_EEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fconsume_5fabsl_3aabsl_5ffunctional(
            f: *mut ::any_invocable::RawAnyInvocable,
            param_0: *const ::core::ffi::c_uchar,
            out: *mut ::core::ffi::c_uchar,
        );
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
        unsafe {
            ::dyn_callable_rs::manager(operation, from, to);
        }
    }
    unsafe extern "C" {
        pub(crate) unsafe fn __crubit_invoke_any_invocable___CcTemplateInstN4absl12AnyInvocableIKFiiEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fconsume_5fabsl_3aabsl_5ffunctional(
            f: *mut ::any_invocable::RawAnyInvocable,
            param_0: ::ffi_11::c_int,
        ) -> ::ffi_11::c_int;
    }
}
