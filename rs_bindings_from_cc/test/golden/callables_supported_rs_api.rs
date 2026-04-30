// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:callables_supported_cc

#![rustfmt::skip]
#![feature(custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![allow(deprecated)]
#![deny(warnings)]

// error: function `invoke_once` could not be bound
//   while generating bridge param 'f': callables (e.g. `AnyInvocable`) are not yet supported

// error: function `invoke` could not be bound
//   while generating bridge param 'f': callables (e.g. `AnyInvocable`) are not yet supported

// error: function `invoke_const` could not be bound
//   while generating bridge param 'f': callables (e.g. `AnyInvocable`) are not yet supported

// error: function `map_int` could not be bound
//   while generating bridge param 'f': callables (e.g. `AnyInvocable`) are not yet supported

// error: function `map_bridged` could not be bound
//   while generating bridge param 'f': callables (e.g. `AnyInvocable`) are not yet supported

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=ABICompatible
pub struct ABICompatible {
    pub x: ::ffi_11::c_int,
}
impl !Send for ABICompatible {}
impl !Sync for ABICompatible {}
unsafe impl ::cxx::ExternType for ABICompatible {
    type Id = ::cxx::type_id!("ABICompatible");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for ABICompatible {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN13ABICompatibleC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

// error: function `map_abi_compatible` could not be bound
//   while generating bridge param 'f': callables (e.g. `AnyInvocable`) are not yet supported

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C, align(4))]
///CRUBIT_ANNOTATE: cpp_type=LayoutCompatible
pub struct LayoutCompatible {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) private_: [::core::mem::MaybeUninit<u8>; 4],
}
impl !Send for LayoutCompatible {}
impl !Sync for LayoutCompatible {}
unsafe impl ::cxx::ExternType for LayoutCompatible {
    type Id = ::cxx::type_id!("LayoutCompatible");
    type Kind = ::cxx::kind::Trivial;
}
impl LayoutCompatible {
    #[inline(always)]
    pub fn Create(x: ::ffi_11::c_int) -> crate::LayoutCompatible {
        unsafe { self::layout_compatible::Create(x) }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub unsafe fn get(__this: *const Self) -> ::ffi_11::c_int {
        unsafe { self::layout_compatible::get(__this) }
    }
}

pub mod layout_compatible {
    #[inline(always)]
    pub(crate) fn Create(x: ::ffi_11::c_int) -> crate::LayoutCompatible {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<crate::LayoutCompatible>::uninit();
            crate::detail::__rust_thunk___ZN16LayoutCompatible6CreateEi(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                x,
            );
            __crubit_return.assume_init()
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn get(__this: *const crate::LayoutCompatible) -> ::ffi_11::c_int {
        unsafe { crate::detail::__rust_thunk___ZNK16LayoutCompatible3getEv(__this) }
    }
}

// error: function `map_layout_compatible` could not be bound
//   while generating bridge param 'f': callables (e.g. `AnyInvocable`) are not yet supported

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

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN13ABICompatibleC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN16LayoutCompatible6CreateEi(
            __return: *mut ::core::ffi::c_void,
            x: ::ffi_11::c_int,
        );
        pub(crate) unsafe fn __rust_thunk___ZNK16LayoutCompatible3getEv(
            __this: *const crate::LayoutCompatible,
        ) -> ::ffi_11::c_int;
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::ABICompatible>() == 4);
    assert!(::core::mem::align_of::<crate::ABICompatible>() == 4);
    static_assertions::assert_impl_all!(crate::ABICompatible: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::ABICompatible: Drop);
    assert!(::core::mem::offset_of!(crate::ABICompatible, x) == 0);
    assert!(::core::mem::size_of::<crate::LayoutCompatible>() == 4);
    assert!(::core::mem::align_of::<crate::LayoutCompatible>() == 4);
    static_assertions::assert_impl_all!(crate::LayoutCompatible: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::LayoutCompatible: Drop);
    assert!(::core::mem::offset_of!(crate::LayoutCompatible, private_) == 0);
};
