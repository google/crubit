// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:callables_cc

#![rustfmt::skip]
#![feature(custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![deny(warnings)]

extern crate alloc;

#[inline(always)]
pub fn invoke_once(
    f: ::alloc::boxed::Box<
        dyn ::core::ops::FnOnce() + ::core::marker::Send + ::core::marker::Sync + 'static,
    >,
) {
    unsafe {
        crate::detail::__rust_thunk___Z11invoke_onceN6rs_std11DynCallableIFvvOEEE(::bridge_rust::unstable_encode!(@::dyn_callable_rs::DynCallableAbi::<dyn::core::ops::FnOnce()+::core::marker::Send+::core::marker::Sync+'static>::new(::alloc::boxed::Box::new(||{ ::core::panic!("moved-from value") }),),::dyn_callable_rs::DynCallableAbi<dyn::core::ops::FnOnce()+::core::marker::Send+::core::marker::Sync+'static>,f).as_ptr()as*const u8)
    }
}

#[inline(always)]
pub fn invoke(
    f: ::alloc::boxed::Box<
        dyn ::core::ops::FnMut() + ::core::marker::Send + ::core::marker::Sync + 'static,
    >,
) {
    unsafe {
        crate::detail::__rust_thunk___Z6invokeN6rs_std11DynCallableIFvvEEE(::bridge_rust::unstable_encode!(@::dyn_callable_rs::DynCallableAbi::<dyn::core::ops::FnMut()+::core::marker::Send+::core::marker::Sync+'static>::new(::alloc::boxed::Box::new(||{ ::core::panic!("moved-from value") }),),::dyn_callable_rs::DynCallableAbi<dyn::core::ops::FnMut()+::core::marker::Send+::core::marker::Sync+'static>,f).as_ptr()as*const u8)
    }
}

#[inline(always)]
pub fn invoke_const(
    f: ::alloc::boxed::Box<
        dyn ::core::ops::Fn() + ::core::marker::Send + ::core::marker::Sync + 'static,
    >,
) {
    unsafe {
        crate::detail::__rust_thunk___Z12invoke_constN6rs_std11DynCallableIKFvvEEE(::bridge_rust::unstable_encode!(@::dyn_callable_rs::DynCallableAbi::<dyn::core::ops::Fn()+::core::marker::Send+::core::marker::Sync+'static>::new(::alloc::boxed::Box::new(||{ ::core::panic!("moved-from value") }),),::dyn_callable_rs::DynCallableAbi<dyn::core::ops::Fn()+::core::marker::Send+::core::marker::Sync+'static>,f).as_ptr()as*const u8)
    }
}

#[inline(always)]
pub fn map_int(
    f: ::alloc::boxed::Box<
        dyn ::core::ops::Fn(::ffi_11::c_int) -> ::ffi_11::c_int
            + ::core::marker::Send
            + ::core::marker::Sync
            + 'static,
    >,
    arg: ::ffi_11::c_int,
) -> ::ffi_11::c_int {
    unsafe {
        crate::detail::__rust_thunk___Z7map_intN6rs_std11DynCallableIKFiiEEEi(::bridge_rust::unstable_encode!(@::dyn_callable_rs::DynCallableAbi::<dyn::core::ops::Fn(::ffi_11::c_int)->::ffi_11::c_int+::core::marker::Send+::core::marker::Sync+'static>::new(::alloc::boxed::Box::new(|_: ::ffi_11::c_int|->::ffi_11::c_int{ ::core::panic!("moved-from value") }),),::dyn_callable_rs::DynCallableAbi<dyn::core::ops::Fn(::ffi_11::c_int)->::ffi_11::c_int+::core::marker::Send+::core::marker::Sync+'static>,f).as_ptr()as*const u8,arg)
    }
}

#[inline(always)]
pub fn map_bridged(
    f: ::alloc::boxed::Box<
        dyn ::core::ops::Fn(crate::RustBridged) -> crate::RustBridged
            + ::core::marker::Send
            + ::core::marker::Sync
            + 'static,
    >,
    arg: crate::RustBridged,
) -> crate::RustBridged {
    unsafe {
        ::bridge_rust::unstable_return!(@crate::RustBridgedAbi,crate::RustBridgedAbi,|__return_abi_buffer|{ crate::detail::__rust_thunk___Z11map_bridgedN6rs_std11DynCallableIKF7BridgedS1_EEES1_(__return_abi_buffer,::bridge_rust::unstable_encode!(@::dyn_callable_rs::DynCallableAbi::<dyn::core::ops::Fn(crate::RustBridged)->crate::RustBridged+::core::marker::Send+::core::marker::Sync+'static>::new(::alloc::boxed::Box::new(|_: crate::RustBridged|->crate::RustBridged{ ::core::panic!("moved-from value") }),),::dyn_callable_rs::DynCallableAbi<dyn::core::ops::Fn(crate::RustBridged)->crate::RustBridged+::core::marker::Send+::core::marker::Sync+'static>,f).as_ptr()as*const u8,::bridge_rust::unstable_encode!(@crate::RustBridgedAbi,crate::RustBridgedAbi,arg).as_ptr()as*const u8); })
    }
}

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

#[inline(always)]
pub fn map_abi_compatible(
    f: ::alloc::boxed::Box<
        dyn ::core::ops::Fn(crate::ABICompatible) -> crate::ABICompatible
            + ::core::marker::Send
            + ::core::marker::Sync
            + 'static,
    >,
    mut arg: crate::ABICompatible,
) -> crate::ABICompatible {
    unsafe {
        let mut __return = ::core::mem::MaybeUninit::<crate::ABICompatible>::uninit();
        crate::detail::__rust_thunk___Z18map_abi_compatibleN6rs_std11DynCallableIKF13ABICompatibleS1_EEES1_(&raw mut __return as*mut::core::ffi::c_void,::bridge_rust::unstable_encode!(@::dyn_callable_rs::DynCallableAbi::<dyn::core::ops::Fn(crate::ABICompatible)->crate::ABICompatible+::core::marker::Send+::core::marker::Sync+'static>::new(::alloc::boxed::Box::new(|_: crate::ABICompatible|->crate::ABICompatible{ ::core::panic!("moved-from value") }),),::dyn_callable_rs::DynCallableAbi<dyn::core::ops::Fn(crate::ABICompatible)->crate::ABICompatible+::core::marker::Send+::core::marker::Sync+'static>,f).as_ptr()as*const u8,&mut arg);
        __return.assume_init()
    }
}

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
        unsafe {
            let mut __return = ::core::mem::MaybeUninit::<Self>::uninit();
            crate::detail::__rust_thunk___ZN16LayoutCompatible6CreateEi(
                &raw mut __return as *mut ::core::ffi::c_void,
                x,
            );
            __return.assume_init()
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub unsafe fn get(__this: *const Self) -> ::ffi_11::c_int {
        crate::detail::__rust_thunk___ZNK16LayoutCompatible3getEv(__this)
    }
}

#[inline(always)]
pub fn map_layout_compatible(
    f: ::alloc::boxed::Box<
        dyn ::core::ops::Fn(crate::LayoutCompatible) -> crate::LayoutCompatible
            + ::core::marker::Send
            + ::core::marker::Sync
            + 'static,
    >,
    mut arg: crate::LayoutCompatible,
) -> crate::LayoutCompatible {
    unsafe {
        let mut __return = ::core::mem::MaybeUninit::<crate::LayoutCompatible>::uninit();
        crate::detail::__rust_thunk___Z21map_layout_compatibleN6rs_std11DynCallableIKF16LayoutCompatibleS1_EEES1_(&raw mut __return as*mut::core::ffi::c_void,::bridge_rust::unstable_encode!(@::dyn_callable_rs::DynCallableAbi::<dyn::core::ops::Fn(crate::LayoutCompatible)->crate::LayoutCompatible+::core::marker::Send+::core::marker::Sync+'static>::new(::alloc::boxed::Box::new(|_: crate::LayoutCompatible|->crate::LayoutCompatible{ ::core::panic!("moved-from value") }),),::dyn_callable_rs::DynCallableAbi<dyn::core::ops::Fn(crate::LayoutCompatible)->crate::LayoutCompatible+::core::marker::Send+::core::marker::Sync+'static>,f).as_ptr()as*const u8,&mut arg);
        __return.assume_init()
    }
}

// error: struct `std::integral_constant<bool, false>` could not be bound
//   template instantiation is not yet supported

// error: struct `std::integral_constant<bool, true>` could not be bound
//   template instantiation is not yet supported

// error: class `std::initializer_list<char32_t>` could not be bound
//   template instantiation is not yet supported

// error: class `std::initializer_list<char16_t>` could not be bound
//   template instantiation is not yet supported

// error: class `std::initializer_list<char>` could not be bound
//   template instantiation is not yet supported

// error: struct `std::iterator_traits<char32_t *>` could not be bound
//   template instantiation is not yet supported

// error: struct `std::iterator_traits<char16_t *>` could not be bound
//   template instantiation is not yet supported

// error: struct `std::iterator_traits<const char32_t *>` could not be bound
//   template instantiation is not yet supported

// error: struct `std::iterator_traits<const char16_t *>` could not be bound
//   template instantiation is not yet supported

// error: struct `std::iterator_traits<const char *>` could not be bound
//   template instantiation is not yet supported

// error: struct `std::iterator_traits<char *>` could not be bound
//   template instantiation is not yet supported

// error: struct `std::pointer_traits<char32_t *>` could not be bound
//   template instantiation is not yet supported

// error: struct `std::pointer_traits<char16_t *>` could not be bound
//   template instantiation is not yet supported

// error: struct `std::pointer_traits<char *>` could not be bound
//   template instantiation is not yet supported

// error: struct `std::placeholders::__ph<10>` could not be bound
//   template instantiation is not yet supported

// error: struct `std::placeholders::__ph<1>` could not be bound
//   template instantiation is not yet supported

// error: struct `std::placeholders::__ph<2>` could not be bound
//   template instantiation is not yet supported

// error: struct `std::placeholders::__ph<3>` could not be bound
//   template instantiation is not yet supported

// error: struct `std::placeholders::__ph<4>` could not be bound
//   template instantiation is not yet supported

// error: struct `std::placeholders::__ph<5>` could not be bound
//   template instantiation is not yet supported

// error: struct `std::placeholders::__ph<6>` could not be bound
//   template instantiation is not yet supported

// error: struct `std::placeholders::__ph<7>` could not be bound
//   template instantiation is not yet supported

// error: struct `std::placeholders::__ph<8>` could not be bound
//   template instantiation is not yet supported

// error: struct `std::placeholders::__ph<9>` could not be bound
//   template instantiation is not yet supported

// error: class `std::allocator<char32_t>` could not be bound
//   template instantiation is not yet supported

// error: class `std::allocator<char16_t>` could not be bound
//   template instantiation is not yet supported

// error: class `std::allocator<char>` could not be bound
//   template instantiation is not yet supported

// error: class `std::pmr::polymorphic_allocator<char32_t>` could not be bound
//   template instantiation is not yet supported

// error: class `std::pmr::polymorphic_allocator<char16_t>` could not be bound
//   template instantiation is not yet supported

// error: class `std::pmr::polymorphic_allocator<char>` could not be bound
//   template instantiation is not yet supported

// error: class `std::basic_string<char32_t, std::char_traits<char32_t>, std::pmr::polymorphic_allocator<char32_t>>` could not be bound
//   template instantiation is not yet supported

// error: class `std::basic_string<char32_t, std::char_traits<char32_t>, std::allocator<char32_t>>` could not be bound
//   template instantiation is not yet supported

// error: class `std::basic_string<char16_t, std::char_traits<char16_t>, std::pmr::polymorphic_allocator<char16_t>>` could not be bound
//   template instantiation is not yet supported

// error: class `std::basic_string<char16_t, std::char_traits<char16_t>, std::allocator<char16_t>>` could not be bound
//   template instantiation is not yet supported

// error: class `std::basic_string` could not be bound
//   Unsupported type 'char8_t': Unsupported builtin type

// error: class `std::basic_string` could not be bound
//   Unsupported type 'char8_t': Unsupported builtin type

// error: class `std::basic_string` could not be bound
//   Unsupported type 'wchar_t': Unsupported builtin type

// error: class `std::basic_string` could not be bound
//   Unsupported type 'wchar_t': Unsupported builtin type

// error: class `std::basic_ostream<char, std::char_traits<char>>` could not be bound
//   incomplete type

// error: class `std::basic_ostream<wchar_t, std::char_traits<wchar_t>>` could not be bound
//   incomplete type

// error: struct `std::__alloc_traits_difference_type<std::pmr::polymorphic_allocator<char32_t>, char32_t *, void>` could not be bound
//   template instantiation is not yet supported

// error: struct `std::__alloc_traits_difference_type<std::pmr::polymorphic_allocator<char16_t>, char16_t *, void>` could not be bound
//   template instantiation is not yet supported

// error: struct `std::__alloc_traits_difference_type<std::pmr::polymorphic_allocator<char>, char *, void>` could not be bound
//   template instantiation is not yet supported

// error: struct `std::__allocator_traits_base<std::pmr::polymorphic_allocator<char32_t>>` could not be bound
//   template instantiation is not yet supported

// error: struct `std::__allocator_traits_base<std::pmr::polymorphic_allocator<char16_t>>` could not be bound
//   template instantiation is not yet supported

// error: struct `std::__allocator_traits_base<std::pmr::polymorphic_allocator<char>>` could not be bound
//   template instantiation is not yet supported

// error: struct `std::__allocator_traits_base<std::allocator<char32_t>>` could not be bound
//   template instantiation is not yet supported

// error: struct `std::__allocator_traits_base<std::allocator<char16_t>>` could not be bound
//   template instantiation is not yet supported

// error: struct `std::__allocator_traits_base<std::allocator<char>>` could not be bound
//   template instantiation is not yet supported

// error: struct `std::allocator_traits<std::pmr::polymorphic_allocator<char32_t>>` could not be bound
//   template instantiation is not yet supported

// error: struct `std::allocator_traits<std::pmr::polymorphic_allocator<char16_t>>` could not be bound
//   template instantiation is not yet supported

// error: struct `std::allocator_traits<std::pmr::polymorphic_allocator<char>>` could not be bound
//   template instantiation is not yet supported

// error: struct `std::allocator_traits<std::allocator<char32_t>>` could not be bound
//   template instantiation is not yet supported

// error: struct `std::allocator_traits<std::allocator<char16_t>>` could not be bound
//   template instantiation is not yet supported

// error: struct `std::allocator_traits<std::allocator<char>>` could not be bound
//   template instantiation is not yet supported

// error: class `std::reverse_iterator<std::__wrap_iter<char32_t *>>` could not be bound
//   template instantiation is not yet supported

// error: class `std::reverse_iterator<std::__wrap_iter<char16_t *>>` could not be bound
//   template instantiation is not yet supported

// error: class `std::reverse_iterator<std::__wrap_iter<const char32_t *>>` could not be bound
//   template instantiation is not yet supported

// error: class `std::reverse_iterator<std::__wrap_iter<const char16_t *>>` could not be bound
//   template instantiation is not yet supported

// error: class `std::reverse_iterator<std::__wrap_iter<const char *>>` could not be bound
//   template instantiation is not yet supported

// error: class `std::reverse_iterator<std::__wrap_iter<char *>>` could not be bound
//   template instantiation is not yet supported

// error: class `std::reverse_iterator<const char32_t *>` could not be bound
//   template instantiation is not yet supported

// error: class `std::reverse_iterator<const char16_t *>` could not be bound
//   template instantiation is not yet supported

// error: class `std::reverse_iterator<const char8_t *>` could not be bound
//   template instantiation is not yet supported

// error: class `std::reverse_iterator<const char *>` could not be bound
//   template instantiation is not yet supported

// error: class `std::reverse_iterator<const wchar_t *>` could not be bound
//   template instantiation is not yet supported

// error: class `std::__wrap_iter<char32_t *>` could not be bound
//   template instantiation is not yet supported

// error: class `std::__wrap_iter<char16_t *>` could not be bound
//   template instantiation is not yet supported

// error: class `std::__wrap_iter<const char32_t *>` could not be bound
//   template instantiation is not yet supported

// error: class `std::__wrap_iter<const char16_t *>` could not be bound
//   template instantiation is not yet supported

// error: class `std::__wrap_iter<const char *>` could not be bound
//   template instantiation is not yet supported

// error: class `std::__wrap_iter<char *>` could not be bound
//   template instantiation is not yet supported

// error: struct `std::__allocation_result<char32_t *, unsigned long>` could not be bound
//   template instantiation is not yet supported

// error: struct `std::__allocation_result<char16_t *, unsigned long>` could not be bound
//   template instantiation is not yet supported

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

// error: struct `std::char_traits<char>` could not be bound
//   template instantiation is not yet supported

// error: struct `std::__char_traits_base<char32_t, unsigned int, 4294967295U>` could not be bound
//   template instantiation is not yet supported

// error: struct `std::__char_traits_base<char16_t, unsigned short, (unsigned short)65535>` could not be bound
//   template instantiation is not yet supported

// error: struct `std::__char_traits_base<char8_t, unsigned int, 4294967295U>` could not be bound
//   template instantiation is not yet supported

// error: struct `std::__char_traits_base<wchar_t, unsigned int, 4294967295U>` could not be bound
//   template instantiation is not yet supported

// error: struct `std::char_traits<wchar_t>` could not be bound
//   template instantiation is not yet supported

// error: struct `std::char_traits<char8_t>` could not be bound
//   template instantiation is not yet supported

// error: struct `std::char_traits<char16_t>` could not be bound
//   template instantiation is not yet supported

// error: struct `std::char_traits<char32_t>` could not be bound
//   template instantiation is not yet supported

// error: class `std::basic_string_view<char32_t, std::char_traits<char32_t>>` could not be bound
//   template instantiation is not yet supported

// error: class `std::basic_string_view<char16_t, std::char_traits<char16_t>>` could not be bound
//   template instantiation is not yet supported

// error: class `std::basic_string_view<char8_t, std::char_traits<char8_t>>` could not be bound
//   template instantiation is not yet supported

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___Z11invoke_onceN6rs_std11DynCallableIFvvOEEE(
            f: *const ::core::ffi::c_uchar,
        );
        pub(crate) unsafe fn __rust_thunk___Z6invokeN6rs_std11DynCallableIFvvEEE(
            f: *const ::core::ffi::c_uchar,
        );
        pub(crate) unsafe fn __rust_thunk___Z12invoke_constN6rs_std11DynCallableIKFvvEEE(
            f: *const ::core::ffi::c_uchar,
        );
        pub(crate) unsafe fn __rust_thunk___Z7map_intN6rs_std11DynCallableIKFiiEEEi(
            f: *const ::core::ffi::c_uchar,
            arg: ::ffi_11::c_int,
        ) -> ::ffi_11::c_int;
        pub(crate) unsafe fn __rust_thunk___Z11map_bridgedN6rs_std11DynCallableIKF7BridgedS1_EEES1_(
            __return_abi_buffer: *mut ::core::ffi::c_uchar,
            f: *const ::core::ffi::c_uchar,
            arg: *const ::core::ffi::c_uchar,
        );
        pub(crate) unsafe fn __rust_thunk___ZN13ABICompatibleC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___Z18map_abi_compatibleN6rs_std11DynCallableIKF13ABICompatibleS1_EEES1_(
            __return: *mut ::core::ffi::c_void,
            f: *const ::core::ffi::c_uchar,
            arg: &mut crate::ABICompatible,
        );
        pub(crate) unsafe fn __rust_thunk___ZN16LayoutCompatible6CreateEi(
            __return: *mut ::core::ffi::c_void,
            x: ::ffi_11::c_int,
        );
        pub(crate) unsafe fn __rust_thunk___ZNK16LayoutCompatible3getEv(
            __this: *const crate::LayoutCompatible,
        ) -> ::ffi_11::c_int;
        pub(crate) unsafe fn __rust_thunk___Z21map_layout_compatibleN6rs_std11DynCallableIKF16LayoutCompatibleS1_EEES1_(
            __return: *mut ::core::ffi::c_void,
            f: *const ::core::ffi::c_uchar,
            arg: &mut crate::LayoutCompatible,
        );
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __crubit_invoker___CcTemplateInstN6rs_std11DynCallableIFvvEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
        f: *mut ::alloc::boxed::Box<
            dyn ::core::ops::FnMut() + ::core::marker::Send + ::core::marker::Sync + 'static,
        >,
    ) {
        (unsafe { &mut *f })();
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __crubit_manager___CcTemplateInstN6rs_std11DynCallableIFvvEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
        operation: ::dyn_callable_rs::FunctionToCall,
        from: *mut ::alloc::boxed::Box<
            dyn ::core::ops::FnMut() + ::core::marker::Send + ::core::marker::Sync + 'static,
        >,
        to: *mut ::alloc::boxed::Box<
            dyn ::core::ops::FnMut() + ::core::marker::Send + ::core::marker::Sync + 'static,
        >,
    ) {
        ::dyn_callable_rs::manager(operation, from, to);
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __crubit_invoker___CcTemplateInstN6rs_std11DynCallableIFvvOEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
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
    unsafe extern "C" fn __crubit_manager___CcTemplateInstN6rs_std11DynCallableIFvvOEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
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
    unsafe extern "C" fn __crubit_invoker___CcTemplateInstN6rs_std11DynCallableIKF13ABICompatibleS1_EEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
        f: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn(crate::ABICompatible) -> crate::ABICompatible
                + ::core::marker::Send
                + ::core::marker::Sync
                + 'static,
        >,
        param_0: *mut crate::ABICompatible,
        out: *mut crate::ABICompatible,
    ) {
        let param_0 = ::core::ptr::read(param_0);
        match (unsafe { &*f })(param_0) {
            result => unsafe {
                ::core::ptr::write(out, result);
            },
        }
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __crubit_manager___CcTemplateInstN6rs_std11DynCallableIKF13ABICompatibleS1_EEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
        operation: ::dyn_callable_rs::FunctionToCall,
        from: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn(crate::ABICompatible) -> crate::ABICompatible
                + ::core::marker::Send
                + ::core::marker::Sync
                + 'static,
        >,
        to: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn(crate::ABICompatible) -> crate::ABICompatible
                + ::core::marker::Send
                + ::core::marker::Sync
                + 'static,
        >,
    ) {
        ::dyn_callable_rs::manager(operation, from, to);
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __crubit_invoker___CcTemplateInstN6rs_std11DynCallableIKF16LayoutCompatibleS1_EEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
        f: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn(crate::LayoutCompatible) -> crate::LayoutCompatible
                + ::core::marker::Send
                + ::core::marker::Sync
                + 'static,
        >,
        param_0: *mut crate::LayoutCompatible,
        out: *mut crate::LayoutCompatible,
    ) {
        let param_0 = ::core::ptr::read(param_0);
        match (unsafe { &*f })(param_0) {
            result => unsafe {
                ::core::ptr::write(out, result);
            },
        }
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __crubit_manager___CcTemplateInstN6rs_std11DynCallableIKF16LayoutCompatibleS1_EEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
        operation: ::dyn_callable_rs::FunctionToCall,
        from: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn(crate::LayoutCompatible) -> crate::LayoutCompatible
                + ::core::marker::Send
                + ::core::marker::Sync
                + 'static,
        >,
        to: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn(crate::LayoutCompatible) -> crate::LayoutCompatible
                + ::core::marker::Send
                + ::core::marker::Sync
                + 'static,
        >,
    ) {
        ::dyn_callable_rs::manager(operation, from, to);
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __crubit_invoker___CcTemplateInstN6rs_std11DynCallableIKF7BridgedS1_EEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
        f: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn(crate::RustBridged) -> crate::RustBridged
                + ::core::marker::Send
                + ::core::marker::Sync
                + 'static,
        >,
        param_0: *mut ::core::ffi::c_uchar,
        bridge_buffer: *mut ::core::ffi::c_uchar,
    ) {
        let param_0 = ::bridge_rust::internal::decode(crate::RustBridgedAbi, param_0);
        ::bridge_rust::internal::encode(
            crate::RustBridgedAbi,
            bridge_buffer,
            (unsafe { &*f })(param_0),
        );
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __crubit_manager___CcTemplateInstN6rs_std11DynCallableIKF7BridgedS1_EEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
        operation: ::dyn_callable_rs::FunctionToCall,
        from: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn(crate::RustBridged) -> crate::RustBridged
                + ::core::marker::Send
                + ::core::marker::Sync
                + 'static,
        >,
        to: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn(crate::RustBridged) -> crate::RustBridged
                + ::core::marker::Send
                + ::core::marker::Sync
                + 'static,
        >,
    ) {
        ::dyn_callable_rs::manager(operation, from, to);
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __crubit_invoker___CcTemplateInstN6rs_std11DynCallableIKFiiEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
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
    unsafe extern "C" fn __crubit_manager___CcTemplateInstN6rs_std11DynCallableIKFiiEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
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
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __crubit_invoker___CcTemplateInstN6rs_std11DynCallableIKFvvEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
        f: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn() + ::core::marker::Send + ::core::marker::Sync + 'static,
        >,
    ) {
        (unsafe { &*f })();
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __crubit_manager___CcTemplateInstN6rs_std11DynCallableIKFvvEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
        operation: ::dyn_callable_rs::FunctionToCall,
        from: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn() + ::core::marker::Send + ::core::marker::Sync + 'static,
        >,
        to: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn() + ::core::marker::Send + ::core::marker::Sync + 'static,
        >,
    ) {
        ::dyn_callable_rs::manager(operation, from, to);
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
