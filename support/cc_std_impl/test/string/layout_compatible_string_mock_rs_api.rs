// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //support/cc_std_impl/test/string:layout_compatible_string_mock_cc

#![rustfmt::skip]
#![feature(cfi_encoding, custom_inner_attributes, impl_trait_in_assoc_type, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![allow(deprecated)]
#![allow(unknown_lints, suspicious_runtime_symbol_definitions)]
#![deny(warnings)]
pub mod std {
    pub mod __u { // error: class `std::char_traits` could not be bound
                  //   Class templates are not yet supported

        // error: class `std::allocator` could not be bound
        //   Class templates are not yet supported

        // error: class `std::basic_string` could not be bound
        //   Class templates are not yet supported

        // error: type alias `std::__u::string` could not be bound
        //   Type alias for std::__u::string suppressed due to being a bridge type
    }
    #[allow(unused_imports)]
    pub use __u::*;

    // namespace __u
}

// namespace std

pub mod test {
    #[inline(always)]
    pub fn RoundTrip(s: ::cc_std::std::string_wrapper) -> ::cc_std::std::string_wrapper {
        unsafe {
            ::bridge_rust::unstable_return!(@::cc_std::std::BoxedCppStringAbi,::cc_std::std::BoxedCppStringAbi,|__crubit_return_abi_buffer|{ crate::detail::__rust_thunk___ZN4test9RoundTripENSt3__u12basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEE(__crubit_return_abi_buffer,::bridge_rust::unstable_encode!(@::cc_std::std::BoxedCppStringAbi,::cc_std::std::BoxedCppStringAbi,s).as_ptr()as*const u8); })
        }
    }

    #[::ctor::recursively_pinned(PinnedDrop)]
    #[cfi_encoding = "N4test12StringStructE"]
    #[repr(C)]
    ///CRUBIT_ANNOTATE: cpp_type=test :: StringStruct
    pub struct StringStruct {
        /// Reason for representing this field as a blob of bytes:
        /// nontrivial fields would be destroyed in the wrong order
        pub(crate) s: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 32],
    }
    impl !Send for StringStruct {}
    impl !Sync for StringStruct {}
    unsafe impl ::cxx::ExternType for StringStruct {
        type Id = ::cxx::type_id!("test :: StringStruct");
        type Kind = ::cxx::kind::Opaque;
    }

    impl ::ctor::CtorNew<()> for StringStruct {
        type CtorType = ::ctor::Ctor![Self];
        type Error = ::ctor::Infallible;
        #[inline(always)]
        fn ctor_new(args: ()) -> Self::CtorType {
            let () = args;
            unsafe {
                ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                    crate::detail::__rust_thunk___ZN4test12StringStructC1Ev(
                        __crubit_dest as *mut ::core::ffi::c_void,
                    );
                })
            }
        }
    }

    impl<'__param_0> ::ctor::CtorNew<&'__param_0 Self> for StringStruct {
        type CtorType =
            impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__param_0>;
        type Error = ::ctor::Infallible;
        #[inline(always)]
        fn ctor_new(args: &'__param_0 Self) -> Self::CtorType {
            let mut __param_0 = args;
            unsafe {
                ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                    crate::detail::__rust_thunk___ZN4test12StringStructC1ERKS0_(
                        __crubit_dest as *mut ::core::ffi::c_void,
                        __param_0,
                    );
                })
            }
        }
    }
    impl<'__param_0> ::ctor::CtorNew<(&'__param_0 Self,)> for StringStruct {
        type CtorType =
            impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__param_0>;
        type Error = ::ctor::Infallible;
        #[inline(always)]
        fn ctor_new(args: (&'__param_0 Self,)) -> Self::CtorType {
            let (arg,) = args;
            <Self as ::ctor::CtorNew<&'__param_0 Self>>::ctor_new(arg)
        }
    }

    impl<'__unelided> ::ctor::CtorNew<::ctor::RvalueReference<'__unelided, Self>> for StringStruct {
        type CtorType =
            impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__unelided>;
        type Error = ::ctor::Infallible;
        #[inline(always)]
        fn ctor_new(args: ::ctor::RvalueReference<'__unelided, Self>) -> Self::CtorType {
            let mut __param_0 = args;
            unsafe {
                ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                    crate::detail::__rust_thunk___ZN4test12StringStructC1EOS0_(
                        __crubit_dest as *mut ::core::ffi::c_void,
                        __param_0,
                    );
                })
            }
        }
    }
    impl<'__unelided> ::ctor::CtorNew<(::ctor::RvalueReference<'__unelided, Self>,)> for StringStruct {
        type CtorType =
            impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__unelided>;
        type Error = ::ctor::Infallible;
        #[inline(always)]
        fn ctor_new(args: (::ctor::RvalueReference<'__unelided, Self>,)) -> Self::CtorType {
            let (arg,) = args;
            <Self as ::ctor::CtorNew<::ctor::RvalueReference<'__unelided, Self>>>::ctor_new(arg)
        }
    }

    impl ::ctor::PinnedDrop for StringStruct {
        #[inline(always)]
        unsafe fn pinned_drop<'__this>(self: ::core::pin::Pin<&'__this mut Self>) {
            unsafe { crate::detail::__rust_thunk___ZN4test12StringStructD1Ev(self) }
        }
    }

    impl<'__param_0> ::ctor::Assign<&'__param_0 Self> for StringStruct {
        #[inline(always)]
        fn assign<'__this>(self: ::core::pin::Pin<&'__this mut Self>, __param_0: &'__param_0 Self) {
            unsafe {
                crate::detail::__rust_thunk___ZN4test12StringStructaSERKS0_(self, __param_0);
            }
        }
    }

    impl ::ctor::Assign<::ctor::RvalueReference<'_, Self>> for StringStruct {
        #[inline(always)]
        fn assign<'__this>(
            self: ::core::pin::Pin<&'__this mut Self>,
            __param_0: ::ctor::RvalueReference<'_, Self>,
        ) {
            unsafe {
                crate::detail::__rust_thunk___ZN4test12StringStructaSEOS0_(self, __param_0);
            }
        }
    }
}

// namespace test

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN4test9RoundTripENSt3__u12basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEE(
            __return_abi_buffer: *mut ::core::ffi::c_uchar,
            s: *const ::core::ffi::c_uchar,
        );
        pub(crate) unsafe fn __rust_thunk___ZN4test12StringStructC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN4test12StringStructC1ERKS0_<'__param_0>(
            __this: *mut ::core::ffi::c_void,
            __param_0: &'__param_0 crate::test::StringStruct,
        );
        pub(crate) unsafe fn __rust_thunk___ZN4test12StringStructC1EOS0_<'__unelided>(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'__unelided, crate::test::StringStruct>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN4test12StringStructD1Ev<'__this>(
            __this: ::core::pin::Pin<&'__this mut crate::test::StringStruct>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN4test12StringStructaSERKS0_<'__param_0, '__this>(
            __this: ::core::pin::Pin<&'__this mut crate::test::StringStruct>,
            __param_0: &'__param_0 crate::test::StringStruct,
        ) -> ::core::pin::Pin<&'__this mut crate::test::StringStruct>;
        pub(crate) unsafe fn __rust_thunk___ZN4test12StringStructaSEOS0_<'__this>(
            __this: ::core::pin::Pin<&'__this mut crate::test::StringStruct>,
            __param_0: ::ctor::RvalueReference<'_, crate::test::StringStruct>,
        ) -> ::core::pin::Pin<&'__this mut crate::test::StringStruct>;
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::test::StringStruct>() == 32);
    assert!(::core::mem::align_of::<crate::test::StringStruct>() == 1);
    static_assertions::assert_impl_all!(crate::test::StringStruct: Drop);
    static_assertions::assert_not_impl_any!(crate::test::StringStruct: Copy);
    assert!(::core::mem::offset_of!(crate::test::StringStruct, s) == 0);
};
