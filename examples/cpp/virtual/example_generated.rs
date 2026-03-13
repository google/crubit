// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //examples/cpp/virtual:example_lib
// Features: supported, types

#![rustfmt::skip]
#![feature(custom_inner_attributes, impl_trait_in_assoc_type, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![deny(warnings)]

/// Generated from: examples/cpp/virtual/example.h;l=13
#[::ctor::recursively_pinned(PinnedDrop)]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=RustDerived
pub struct RustDerived {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 8],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) rust_: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 16],
}
impl !Send for RustDerived {}
impl !Sync for RustDerived {}
unsafe impl ::cxx::ExternType for RustDerived {
    type Id = ::cxx::type_id!("RustDerived");
    type Kind = ::cxx::kind::Opaque;
}
impl RustDerived {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    ///
    /// Generated from: examples/cpp/virtual/example.h;l=16
    #[inline(always)]
    pub unsafe fn Method1(__this: *const Self) -> ::ffi_11::c_int {
        crate::detail::__rust_thunk___ZNK11RustDerived7Method1Ev(__this)
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    ///
    /// Generated from: examples/cpp/virtual/example.h;l=18
    #[inline(always)]
    pub unsafe fn Upcast(__this: *mut Self) -> *mut ::base::ExampleBase {
        crate::detail::__rust_thunk___ZN11RustDerived6UpcastEv(__this)
    }
}

/// Generated from: examples/cpp/virtual/example.h;l=13
impl<'__unelided> ::ctor::CtorNew<::ctor::RvalueReference<'__unelided, Self>> for RustDerived {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__unelided>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'__unelided, Self>) -> Self::CtorType {
        let mut __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(move |dest: *mut Self| {
                crate::detail::__rust_thunk___ZN11RustDerivedC1EOS_(
                    dest as *mut ::core::ffi::c_void,
                    __param_0,
                );
            })
        }
    }
}
impl<'__unelided> ::ctor::CtorNew<(::ctor::RvalueReference<'__unelided, Self>,)> for RustDerived {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__unelided>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (::ctor::RvalueReference<'__unelided, Self>,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ctor::RvalueReference<'__unelided, Self>>>::ctor_new(arg)
    }
}

/// Generated from: examples/cpp/virtual/example.h;l=13
impl ::ctor::PinnedDrop for RustDerived {
    #[inline(always)]
    unsafe fn pinned_drop<'a>(self: ::core::pin::Pin<&'a mut Self>) {
        crate::detail::__rust_thunk___ZN11RustDerivedD1Ev(self)
    }
}

/// Generated from: examples/cpp/virtual/example.h;l=13
impl ::ctor::Assign<::ctor::RvalueReference<'_, Self>> for RustDerived {
    #[inline(always)]
    fn assign(self: ::core::pin::Pin<&mut Self>, __param_0: ::ctor::RvalueReference<'_, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN11RustDerivedaSEOS_(self, __param_0);
        }
    }
}

/// Generated from: examples/cpp/virtual/example.h;l=15
impl ::ctor::CtorNew<::definition::RustDerived> for RustDerived {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::definition::RustDerived) -> Self::CtorType {
        let mut rust = args;
        unsafe {
            ::ctor::FnCtor::new(move |dest: *mut Self| {
                crate::detail::__rust_thunk___ZN11RustDerivedC1EN10definition11RustDerivedE(
                    dest as *mut ::core::ffi::c_void,
                    &mut rust,
                );
            })
        }
    }
}
impl ::ctor::CtorNew<(::definition::RustDerived,)> for RustDerived {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (::definition::RustDerived,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::definition::RustDerived>>::ctor_new(arg)
    }
}

unsafe impl ::operator::Delete for crate::RustDerived {
    #[inline(always)]
    unsafe fn delete(p: *mut Self) {
        crate::detail::__crubit_operator_delete__11RustDerived___2f_2fthird_5fparty_2fcrubit_2fexamples_2fcpp_2fvirtual_3aexample_5flib(p);
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

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN11RustDerivedC1EOS_<'__unelided>(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'__unelided, crate::RustDerived>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN11RustDerivedD1Ev<'a>(
            __this: ::core::pin::Pin<&'a mut crate::RustDerived>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN11RustDerivedaSEOS_<'__return_lifetime>(
            __this: ::core::pin::Pin<&mut crate::RustDerived>,
            __param_0: ::ctor::RvalueReference<'_, crate::RustDerived>,
        ) -> ::core::pin::Pin<&'__return_lifetime mut crate::RustDerived>;
        pub(crate) unsafe fn __rust_thunk___ZN11RustDerivedC1EN10definition11RustDerivedE(
            __this: *mut ::core::ffi::c_void,
            rust: &mut ::definition::RustDerived,
        );
        pub(crate) unsafe fn __rust_thunk___ZNK11RustDerived7Method1Ev(
            __this: *const crate::RustDerived,
        ) -> ::ffi_11::c_int;
        pub(crate) unsafe fn __rust_thunk___ZN11RustDerived6UpcastEv(
            __this: *mut crate::RustDerived,
        ) -> *mut ::base::ExampleBase;
        pub(crate) unsafe fn __crubit_operator_delete__11RustDerived___2f_2fthird_5fparty_2fcrubit_2fexamples_2fcpp_2fvirtual_3aexample_5flib(
            ptr: *mut crate::RustDerived,
        );
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::RustDerived>() == 24);
    assert!(::core::mem::align_of::<crate::RustDerived>() == 8);
    static_assertions::assert_impl_all!(crate::RustDerived: Drop);
    static_assertions::assert_not_impl_any!(crate::RustDerived: Copy);
    assert!(::core::mem::offset_of!(crate::RustDerived, rust_) == 8);
};
