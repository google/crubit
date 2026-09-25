// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/consume_absl:absl_functional

#![rustfmt::skip]
#![feature(cfi_encoding, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![allow(deprecated)]
#![allow(unknown_lints, suspicious_runtime_symbol_definitions)]
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
        crate::detail::__rust_thunk___Z12CallVoidVoidN4absl12AnyInvocableIFvvOEEE(::crubit_support::bridge::unstable_encode!(@::any_invocable::AnyInvocableAbi::<dyn::core::ops::FnOnce()+::core::marker::Send+::core::marker::Sync+'static>::new(::alloc::boxed::Box::new(||{ ::core::panic!("moved-from value") }),|raw_any_invocable: ::cc_std::std::unique_ptr<::any_invocable::RawAnyInvocable>|->::alloc::boxed::Box<dyn::core::ops::FnOnce()+::core::marker::Send+::core::marker::Sync+'static>{ ::alloc::boxed::Box::new(move||{ unsafe{ crate::detail::__crubit_invoke_any_invocable___CcTemplateInstN4absl12AnyInvocableIFvvOEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fconsume_5fabsl_3aabsl_5ffunctional(::cc_std::std::unique_ptr::as_ptr(&raw_any_invocable)as*mut _) }; }) },),::any_invocable::AnyInvocableAbi<dyn::core::ops::FnOnce()+::core::marker::Send+::core::marker::Sync+'static>,f).as_ptr()as*const u8)
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
        ::crubit_support::bridge::unstable_return!(@::any_invocable::AnyInvocableAbi::<dyn::core::ops::Fn(::ffi_11::c_int)->::ffi_11::c_int+::core::marker::Send+::core::marker::Sync+'static>::new(::alloc::boxed::Box::new(|_: ::ffi_11::c_int|->::ffi_11::c_int{ ::core::panic!("moved-from value") }),|raw_any_invocable: ::cc_std::std::unique_ptr<::any_invocable::RawAnyInvocable>|->::alloc::boxed::Box<dyn::core::ops::Fn(::ffi_11::c_int)->::ffi_11::c_int+::core::marker::Send+::core::marker::Sync+'static>{ ::alloc::boxed::Box::new(move|param_0: ::ffi_11::c_int|->::ffi_11::c_int{ unsafe{ crate::detail::__crubit_invoke_any_invocable___CcTemplateInstN4absl12AnyInvocableIKFiiEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fconsume_5fabsl_3aabsl_5ffunctional(::cc_std::std::unique_ptr::as_ptr(&raw_any_invocable)as*mut _,param_0) } }) },),::any_invocable::AnyInvocableAbi<dyn::core::ops::Fn(::ffi_11::c_int)->::ffi_11::c_int+::core::marker::Send+::core::marker::Sync+'static>,|__crubit_return_abi_buffer|{ crate::detail::__rust_thunk___Z15ReturnIntMapperv(__crubit_return_abi_buffer,); })
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
        ::crubit_support::bridge::unstable_return!(@::any_invocable::AnyInvocableAbi::<dyn::core::ops::Fn(crate::MyOption<::ffi_11::c_int>)->crate::MyOption<::ffi_11::c_int>+::core::marker::Send+::core::marker::Sync+'static>::new(::alloc::boxed::Box::new(|_: crate::MyOption<::ffi_11::c_int>|->crate::MyOption<::ffi_11::c_int>{ ::core::panic!("moved-from value") }),|raw_any_invocable: ::cc_std::std::unique_ptr<::any_invocable::RawAnyInvocable>|->::alloc::boxed::Box<dyn::core::ops::Fn(crate::MyOption<::ffi_11::c_int>)->crate::MyOption<::ffi_11::c_int>+::core::marker::Send+::core::marker::Sync+'static>{ ::alloc::boxed::Box::new(move|param_0: crate::MyOption<::ffi_11::c_int>|->crate::MyOption<::ffi_11::c_int>{ ::crubit_support::bridge::unstable_return!(@crate::MyOptionAbi(::crubit_support::bridge::transmute_abi::<::core::ffi::c_int>()),crate::MyOptionAbi<::crubit_support::bridge::TransmuteAbi<::core::ffi::c_int>>,|out|{ unsafe{ crate::detail::__crubit_invoke_any_invocable___CcTemplateInstN4absl12AnyInvocableIKF8MyOptionIiES2_EEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fconsume_5fabsl_3aabsl_5ffunctional(::cc_std::std::unique_ptr::as_ptr(&raw_any_invocable)as*mut _,::crubit_support::bridge::unstable_encode!(@crate::MyOptionAbi(::crubit_support::bridge::transmute_abi::<::core::ffi::c_int>()),crate::MyOptionAbi<::crubit_support::bridge::TransmuteAbi<::core::ffi::c_int>>,param_0).as_ptr()as*const u8,out) } }) }) },),::any_invocable::AnyInvocableAbi<dyn::core::ops::Fn(crate::MyOption<::ffi_11::c_int>)->crate::MyOption<::ffi_11::c_int>+::core::marker::Send+::core::marker::Sync+'static>,|__crubit_return_abi_buffer|{ crate::detail::__rust_thunk___Z17MyOptionIntMapperv(__crubit_return_abi_buffer,); })
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
        ::crubit_support::bridge::unstable_return!(@::any_invocable::AnyInvocableAbi::<dyn::core::ops::Fn(::ffi_11::c_int)->::ffi_11::c_int+::core::marker::Send+::core::marker::Sync+'static>::new(::alloc::boxed::Box::new(|_: ::ffi_11::c_int|->::ffi_11::c_int{ ::core::panic!("moved-from value") }),|raw_any_invocable: ::cc_std::std::unique_ptr<::any_invocable::RawAnyInvocable>|->::alloc::boxed::Box<dyn::core::ops::Fn(::ffi_11::c_int)->::ffi_11::c_int+::core::marker::Send+::core::marker::Sync+'static>{ ::alloc::boxed::Box::new(move|param_0: ::ffi_11::c_int|->::ffi_11::c_int{ unsafe{ crate::detail::__crubit_invoke_any_invocable___CcTemplateInstN4absl12AnyInvocableIFiiEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fconsume_5fabsl_3aabsl_5ffunctional(::cc_std::std::unique_ptr::as_ptr(&raw_any_invocable)as*mut _,param_0) } }) },),::any_invocable::AnyInvocableAbi<dyn::core::ops::Fn(::ffi_11::c_int)->::ffi_11::c_int+::core::marker::Send+::core::marker::Sync+'static>,|__crubit_return_abi_buffer|{ crate::detail::__rust_thunk___Z23ReturnNonConstIntMapperv(__crubit_return_abi_buffer,); })
    }
}

/// A type alias to an AnyInvocable becomes a type alias to the bridged Rust
/// type.
pub type IntMapper = ::alloc::boxed::Box<
    dyn ::core::ops::Fn(::ffi_11::c_int) -> ::ffi_11::c_int
        + ::core::marker::Send
        + ::core::marker::Sync
        + 'static,
>;

/// Type aliases are transparent: the parameter is spelled as the bridged Rust
/// type, not as `IntMapper`.
#[inline(always)]
pub fn CallIntMapper(
    f: ::alloc::boxed::Box<
        dyn ::core::ops::Fn(::ffi_11::c_int) -> ::ffi_11::c_int
            + ::core::marker::Send
            + ::core::marker::Sync
            + 'static,
    >,
    i: ::ffi_11::c_int,
) -> ::ffi_11::c_int {
    unsafe {
        crate::detail::__rust_thunk___Z13CallIntMapperN4absl12AnyInvocableIKFiiEEEi(::crubit_support::bridge::unstable_encode!(@::any_invocable::AnyInvocableAbi::<dyn::core::ops::Fn(::ffi_11::c_int)->::ffi_11::c_int+::core::marker::Send+::core::marker::Sync+'static>::new(::alloc::boxed::Box::new(|_: ::ffi_11::c_int|->::ffi_11::c_int{ ::core::panic!("moved-from value") }),|raw_any_invocable: ::cc_std::std::unique_ptr<::any_invocable::RawAnyInvocable>|->::alloc::boxed::Box<dyn::core::ops::Fn(::ffi_11::c_int)->::ffi_11::c_int+::core::marker::Send+::core::marker::Sync+'static>{ ::alloc::boxed::Box::new(move|param_0: ::ffi_11::c_int|->::ffi_11::c_int{ unsafe{ crate::detail::__crubit_invoke_any_invocable___CcTemplateInstN4absl12AnyInvocableIKFiiEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fconsume_5fabsl_3aabsl_5ffunctional(::cc_std::std::unique_ptr::as_ptr(&raw_any_invocable)as*mut _,param_0) } }) },),::any_invocable::AnyInvocableAbi<dyn::core::ops::Fn(::ffi_11::c_int)->::ffi_11::c_int+::core::marker::Send+::core::marker::Sync+'static>,f).as_ptr()as*const u8,i)
    }
}

/// Member type aliases to an AnyInvocable also receive bindings.
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "6Widget"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=Widget
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct Widget {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for Widget {}
impl !Sync for Widget {}
unsafe impl ::cxx::ExternType for Widget {
    type Id = ::cxx::type_id!("Widget");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for Widget {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN6WidgetC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

pub mod widget {
    pub type DoneCallback = ::alloc::boxed::Box<
        dyn ::core::ops::FnOnce() + ::core::marker::Send + ::core::marker::Sync + 'static,
    >;
}

// error: struct `Incomplete` could not be bound
//   incomplete type

// error: function `ReturnIncompleteMapper` could not be bound
//   Return type is not supported: Unsupported type 'absl::AnyInvocable<Incomplete (Incomplete) const>': Failed to create bindings for template specialization type absl::AnyInvocable<Incomplete (Incomplete) const>: Return type of callable is incomplete: struct Incomplete

// error: function `CallIncompleteMapper` could not be bound
//   Parameter #0 is not supported: Unsupported type 'absl::AnyInvocable<Incomplete (Incomplete) const>': Failed to create bindings for template specialization type absl::AnyInvocable<Incomplete (Incomplete) const>: Return type of callable is incomplete: struct Incomplete

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

// error: class `absl::internal_any_invocable::CoreImpl<false, struct MyOption<int>, struct MyOption<int>>` could not be bound
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
        pub(crate) unsafe fn __rust_thunk___Z13CallIntMapperN4absl12AnyInvocableIKFiiEEEi(
            f: *const ::core::ffi::c_uchar,
            i: ::ffi_11::c_int,
        ) -> ::ffi_11::c_int;
        pub(crate) unsafe fn __rust_thunk___ZN6WidgetC1Ev(__this: *mut ::core::ffi::c_void);
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
            ::crubit_support::bridge::internal::decode(
                crate::MyOptionAbi(::crubit_support::bridge::transmute_abi::<::core::ffi::c_int>()),
                param_0,
            )
        };
        unsafe {
            ::crubit_support::bridge::internal::encode(
                crate::MyOptionAbi(::crubit_support::bridge::transmute_abi::<::core::ffi::c_int>()),
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

const _: () = {
    assert!(::core::mem::size_of::<crate::Widget>() == 1);
    assert!(::core::mem::align_of::<crate::Widget>() == 1);
    static_assertions::assert_impl_all!(crate::Widget: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::Widget: Drop);
};
