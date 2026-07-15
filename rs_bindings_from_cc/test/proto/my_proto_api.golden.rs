// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/proto:my_proto_api

#![rustfmt::skip]
#![feature(custom_inner_attributes)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![allow(deprecated)]
#![allow(unknown_lints, suspicious_runtime_symbol_definitions)]
#![deny(warnings)]
pub mod test {
    #[inline(always)]
    pub fn MakeRequest(num: i64) -> ::my_proto::my_message::Request {
        unsafe {
            ::bridge_rust::unstable_return!(@::my_proto::ProtoMessageRustBridge(::core::marker::PhantomData),::my_proto::ProtoMessageRustBridge<::my_proto::my_message::Request>,|__crubit_return_abi_buffer|{ crate::detail::__rust_thunk___ZN4test11MakeRequestEl(__crubit_return_abi_buffer,num); })
        }
    }

    #[inline(always)]
    pub fn ReturnValue() -> ::my_proto::MyMessage {
        unsafe {
            ::bridge_rust::unstable_return!(@::my_proto::ProtoMessageRustBridge(::core::marker::PhantomData),::my_proto::ProtoMessageRustBridge<::my_proto::MyMessage>,|__crubit_return_abi_buffer|{ crate::detail::__rust_thunk___ZN4test11ReturnValueEv(__crubit_return_abi_buffer,); })
        }
    }

    #[inline(always)]
    pub fn ExtractFromValue(msg: ::my_proto::MyMessage) -> i64 {
        unsafe {
            crate::detail::__rust_thunk___ZN4test16ExtractFromValueEN10my_package9MyMessageE(::bridge_rust::unstable_encode!(@::my_proto::ProtoMessageRustBridge(::core::marker::PhantomData),::my_proto::ProtoMessageRustBridge<::my_proto::MyMessage>,msg).as_ptr()as*const u8)
        }
    }

    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `msg`: raw pointer
    #[inline(always)]
    pub unsafe fn ExtractFromConstPtr(
        msg: *const ::forward_declare::Incomplete<
            ::forward_declare::symbol!("my_package :: MyMessage"),
            (),
        >,
    ) -> i64 {
        unsafe {
            crate::detail::__rust_thunk___ZN4test19ExtractFromConstPtrEPKN10my_package9MyMessageE(
                msg,
            )
        }
    }

    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `msg`: raw pointer
    #[inline(always)]
    pub unsafe fn ExtractFromConstRef(
        msg: *const ::forward_declare::Incomplete<
            ::forward_declare::symbol!("my_package :: MyMessage"),
            (),
        >,
    ) -> i64 {
        unsafe {
            crate::detail::__rust_thunk___ZN4test19ExtractFromConstRefERKN10my_package9MyMessageE(
                msg,
            )
        }
    }

    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `msg`: raw pointer
    #[inline(always)]
    pub unsafe fn ExtractFromMutablePtr(
        msg: *mut ::forward_declare::Incomplete<
            ::forward_declare::symbol!("my_package :: MyMessage"),
            (),
        >,
    ) -> i64 {
        unsafe {
            crate::detail::__rust_thunk___ZN4test21ExtractFromMutablePtrEPN10my_package9MyMessageE(
                msg,
            )
        }
    }

    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `msg`: raw pointer
    #[inline(always)]
    pub unsafe fn ExtractFromMutableRef(
        msg: *mut ::forward_declare::Incomplete<
            ::forward_declare::symbol!("my_package :: MyMessage"),
            (),
        >,
    ) -> i64 {
        unsafe {
            crate::detail::__rust_thunk___ZN4test21ExtractFromMutableRefERN10my_package9MyMessageE(
                msg,
            )
        }
    }

    #[inline(always)]
    pub fn GetMutMsgPtr(
    ) -> *mut ::forward_declare::Incomplete<::forward_declare::symbol!("my_package :: MyMessage"), ()>
    {
        unsafe { crate::detail::__rust_thunk___ZN4test12GetMutMsgPtrEv() }
    }

    #[inline(always)]
    pub fn GetConstMsgPtr() -> *const ::forward_declare::Incomplete<
        ::forward_declare::symbol!("my_package :: MyMessage"),
        (),
    > {
        unsafe { crate::detail::__rust_thunk___ZN4test14GetConstMsgPtrEv() }
    }
}

// namespace test

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

// error: class `absl::container_internal::HashtableInlineDataImpl<absl::container_internal::kCapacityByLog>` could not be bound
//   Crubit is not enabled on defining target:
//     third_party/absl/container/internal/raw_hash_set.h
//   template instantiation is not yet supported

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN4test11MakeRequestEl(
            __return_abi_buffer: *mut ::core::ffi::c_uchar,
            num: i64,
        );
        pub(crate) unsafe fn __rust_thunk___ZN4test11ReturnValueEv(
            __return_abi_buffer: *mut ::core::ffi::c_uchar,
        );
        pub(crate) unsafe fn __rust_thunk___ZN4test16ExtractFromValueEN10my_package9MyMessageE(
            msg: *const ::core::ffi::c_uchar,
        ) -> i64;
        pub(crate) unsafe fn __rust_thunk___ZN4test19ExtractFromConstPtrEPKN10my_package9MyMessageE(
            msg: *const ::forward_declare::Incomplete<
                ::forward_declare::symbol!("my_package :: MyMessage"),
                (),
            >,
        ) -> i64;
        pub(crate) unsafe fn __rust_thunk___ZN4test19ExtractFromConstRefERKN10my_package9MyMessageE(
            msg: *const ::forward_declare::Incomplete<
                ::forward_declare::symbol!("my_package :: MyMessage"),
                (),
            >,
        ) -> i64;
        pub(crate) unsafe fn __rust_thunk___ZN4test21ExtractFromMutablePtrEPN10my_package9MyMessageE(
            msg: *mut ::forward_declare::Incomplete<
                ::forward_declare::symbol!("my_package :: MyMessage"),
                (),
            >,
        ) -> i64;
        pub(crate) unsafe fn __rust_thunk___ZN4test21ExtractFromMutableRefERN10my_package9MyMessageE(
            msg: *mut ::forward_declare::Incomplete<
                ::forward_declare::symbol!("my_package :: MyMessage"),
                (),
            >,
        ) -> i64;
        pub(crate) unsafe fn __rust_thunk___ZN4test12GetMutMsgPtrEv(
        ) -> *mut ::forward_declare::Incomplete<
            ::forward_declare::symbol!("my_package :: MyMessage"),
            (),
        >;
        pub(crate) unsafe fn __rust_thunk___ZN4test14GetConstMsgPtrEv(
        ) -> *const ::forward_declare::Incomplete<
            ::forward_declare::symbol!("my_package :: MyMessage"),
            (),
        >;
    }
}
