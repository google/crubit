// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/annotations:thread_safe

#![rustfmt::skip]
#![feature(cfi_encoding, custom_inner_attributes, impl_trait_in_assoc_type, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![allow(deprecated)]
#![allow(unknown_lints, suspicious_runtime_symbol_definitions)]
#![deny(warnings)]
pub mod crubit {
    pub mod test {
        /// A simple thread-safe struct.
        #[::ctor::recursively_pinned]
        #[cfi_encoding = "N6crubit4test16ThreadSafeStructE"]
        #[repr(C, align(4))]
        ///CRUBIT_ANNOTATE: cpp_type=crubit :: test :: ThreadSafeStruct
        ///CRUBIT_ANNOTATE: cpp_thread_safe=
        pub struct ThreadSafeStruct {
            __opaque: ::core::cell::UnsafeCell<[::core::mem::MaybeUninit<u8>; 4]>,
        }
        unsafe impl Send for ThreadSafeStruct {}
        unsafe impl Sync for ThreadSafeStruct {}
        unsafe impl ::cxx::ExternType for ThreadSafeStruct {
            type Id = ::cxx::type_id!("crubit :: test :: ThreadSafeStruct");
            type Kind = ::cxx::kind::Opaque;
        }
        impl ThreadSafeStruct {
            #[inline(always)]
            pub fn ConstGet(&self) -> ::ffi_11::c_int {
                unsafe { self::thread_safe_struct::ConstGet(self) }
            }
            /// A non-const method for testing the generation behavior.
            /// The implementation doesn't actually do anything non-const, but it doesn't
            /// matter for what we are testing, here.
            #[inline(always)]
            pub fn NonConstGet(&self) -> ::ffi_11::c_int {
                unsafe { self::thread_safe_struct::NonConstGet(self) }
            }
        }

        impl ::ctor::CtorNew<()> for ThreadSafeStruct {
            type CtorType = ::ctor::Ctor![Self];
            type Error = ::ctor::Infallible;
            #[inline(always)]
            fn ctor_new(args: ()) -> Self::CtorType {
                let () = args;
                unsafe {
                    ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                        crate::detail::__rust_thunk___ZN6crubit4test16ThreadSafeStructC1Ev(
                            __crubit_dest as *mut ::core::ffi::c_void,
                        );
                    })
                }
            }
        }

        pub mod thread_safe_struct {
            #[inline(always)]
            pub(crate) fn ConstGet(
                __this: &crate::crubit::test::ThreadSafeStruct,
            ) -> ::ffi_11::c_int {
                unsafe {
                    crate::detail::__rust_thunk___ZNK6crubit4test16ThreadSafeStruct8ConstGetEv(
                        __this,
                    )
                }
            }
            /// A non-const method for testing the generation behavior.
            /// The implementation doesn't actually do anything non-const, but it doesn't
            /// matter for what we are testing, here.
            #[inline(always)]
            pub(crate) fn NonConstGet(
                __this: &crate::crubit::test::ThreadSafeStruct,
            ) -> ::ffi_11::c_int {
                unsafe {
                    crate::detail::__rust_thunk___ZN6crubit4test16ThreadSafeStruct11NonConstGetEv(
                        __this,
                    )
                }
            }
        }

        /// A regular (non-thread-safe) struct for comparison.
        #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
        #[cfi_encoding = "N6crubit4test13RegularStructE"]
        #[repr(C, align(4))]
        ///CRUBIT_ANNOTATE: cpp_type=crubit :: test :: RegularStruct
        pub struct RegularStruct {
            __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
            /// Reason for representing this field as a blob of bytes:
            /// Types of non-public C++ fields can be elided away
            pub(crate) x_: [::core::mem::MaybeUninit<u8>; 4],
        }
        impl !Send for RegularStruct {}
        impl !Sync for RegularStruct {}
        unsafe impl ::cxx::ExternType for RegularStruct {
            type Id = ::cxx::type_id!("crubit :: test :: RegularStruct");
            type Kind = ::cxx::kind::Trivial;
        }
        impl RegularStruct {
            /// # Safety
            ///
            /// The caller must ensure that the following unsafe arguments are not misused by the function:
            /// * `__this`: raw pointer
            #[inline(always)]
            pub unsafe fn ConstGet(__this: *const Self) -> ::ffi_11::c_int {
                unsafe { self::regular_struct::ConstGet(__this) }
            }
            /// # Safety
            ///
            /// The caller must ensure that the following unsafe arguments are not misused by the function:
            /// * `__this`: raw pointer
            #[inline(always)]
            pub unsafe fn NonConstGet(__this: *mut Self) -> ::ffi_11::c_int {
                unsafe { self::regular_struct::NonConstGet(__this) }
            }
        }

        impl Default for RegularStruct {
            #[inline(always)]
            fn default() -> Self {
                let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
                unsafe {
                    crate::detail::__rust_thunk___ZN6crubit4test13RegularStructC1Ev(
                        &raw mut tmp as *mut _,
                    );
                    tmp.assume_init()
                }
            }
        }

        pub mod regular_struct {
            /// # Safety
            ///
            /// The caller must ensure that the following unsafe arguments are not misused by the function:
            /// * `__this`: raw pointer
            #[inline(always)]
            pub(crate) unsafe fn ConstGet(
                __this: *const crate::crubit::test::RegularStruct,
            ) -> ::ffi_11::c_int {
                unsafe {
                    crate::detail::__rust_thunk___ZNK6crubit4test13RegularStruct8ConstGetEv(__this)
                }
            }
            /// # Safety
            ///
            /// The caller must ensure that the following unsafe arguments are not misused by the function:
            /// * `__this`: raw pointer
            #[inline(always)]
            pub(crate) unsafe fn NonConstGet(
                __this: *mut crate::crubit::test::RegularStruct,
            ) -> ::ffi_11::c_int {
                unsafe {
                    crate::detail::__rust_thunk___ZN6crubit4test13RegularStruct11NonConstGetEv(
                        __this,
                    )
                }
            }
        }

        /// Unpin thread-safe type with custom assignment
        #[cfi_encoding = "N6crubit4test15ThreadSafeUnpinE"]
        #[repr(C)]
        ///CRUBIT_ANNOTATE: cpp_type=crubit :: test :: ThreadSafeUnpin
        ///CRUBIT_ANNOTATE: cpp_thread_safe=
        pub struct ThreadSafeUnpin {
            __opaque: ::core::cell::UnsafeCell<[::core::mem::MaybeUninit<u8>; 1]>,
        }
        unsafe impl Send for ThreadSafeUnpin {}
        unsafe impl Sync for ThreadSafeUnpin {}
        unsafe impl ::cxx::ExternType for ThreadSafeUnpin {
            type Id = ::cxx::type_id!("crubit :: test :: ThreadSafeUnpin");
            type Kind = ::cxx::kind::Trivial;
        }

        impl Default for ThreadSafeUnpin {
            #[inline(always)]
            fn default() -> Self {
                let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
                unsafe {
                    crate::detail::__rust_thunk___ZN6crubit4test15ThreadSafeUnpinC1Ev(
                        &raw mut tmp as *mut _,
                    );
                    tmp.assume_init()
                }
            }
        }

        impl Clone for ThreadSafeUnpin {
            #[inline(always)]
            fn clone(&self) -> Self {
                let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
                unsafe {
                    crate::detail::__rust_thunk___ZN6crubit4test15ThreadSafeUnpinC1ERKS1_(
                        &raw mut tmp as *mut _,
                        self,
                    );
                    tmp.assume_init()
                }
            }
            fn clone_from(&mut self, other: &Self) {
                use ::ctor::UnpinAssign;
                self.unpin_assign(other);
            }
        }

        impl From<::ctor::RvalueReference<'_, Self>> for ThreadSafeUnpin {
            #[inline(always)]
            fn from(args: ::ctor::RvalueReference<'_, Self>) -> Self {
                let mut __param_0 = args;
                let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
                unsafe {
                    crate::detail::__rust_thunk___ZN6crubit4test15ThreadSafeUnpinC1EOS1_(
                        &raw mut tmp as *mut _,
                        __param_0,
                    );
                    tmp.assume_init()
                }
            }
        }
        impl ::ctor::CtorNew<::ctor::RvalueReference<'_, Self>> for ThreadSafeUnpin {
            type CtorType = Self;
            type Error = ::ctor::Infallible;
            #[inline(always)]
            fn ctor_new(args: ::ctor::RvalueReference<'_, Self>) -> Self::CtorType {
                <Self as From<::ctor::RvalueReference<'_, Self>>>::from(args)
            }
        }

        impl ::ctor::UnpinAssign<&Self> for ThreadSafeUnpin {
            #[inline(always)]
            fn unpin_assign(&mut self, __param_0: &Self) {
                unsafe {
                    crate::detail::__rust_thunk___ZN6crubit4test15ThreadSafeUnpinaSERKS1_(
                        self, __param_0,
                    );
                }
            }
        }

        impl ::ctor::UnpinAssign<::ctor::RvalueReference<'_, Self>> for ThreadSafeUnpin {
            #[inline(always)]
            fn unpin_assign(&mut self, __param_0: ::ctor::RvalueReference<'_, Self>) {
                unsafe {
                    crate::detail::__rust_thunk___ZN6crubit4test15ThreadSafeUnpinaSEOS1_(
                        self, __param_0,
                    );
                }
            }
        }

        /// Pinned thread-safe type with custom assignment
        #[::ctor::recursively_pinned(PinnedDrop)]
        #[cfi_encoding = "N6crubit4test16ThreadSafePinnedE"]
        #[repr(C)]
        ///CRUBIT_ANNOTATE: cpp_type=crubit :: test :: ThreadSafePinned
        ///CRUBIT_ANNOTATE: cpp_thread_safe=
        pub struct ThreadSafePinned {
            __opaque: ::core::cell::UnsafeCell<[::core::mem::MaybeUninit<u8>; 1]>,
        }
        unsafe impl Send for ThreadSafePinned {}
        unsafe impl Sync for ThreadSafePinned {}
        unsafe impl ::cxx::ExternType for ThreadSafePinned {
            type Id = ::cxx::type_id!("crubit :: test :: ThreadSafePinned");
            type Kind = ::cxx::kind::Opaque;
        }

        impl ::ctor::CtorNew<()> for ThreadSafePinned {
            type CtorType = ::ctor::Ctor![Self];
            type Error = ::ctor::Infallible;
            #[inline(always)]
            fn ctor_new(args: ()) -> Self::CtorType {
                let () = args;
                unsafe {
                    ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                        crate::detail::__rust_thunk___ZN6crubit4test16ThreadSafePinnedC1Ev(
                            __crubit_dest as *mut ::core::ffi::c_void,
                        );
                    })
                }
            }
        }

        impl<'__unelided> ::ctor::CtorNew<&'__unelided Self> for ThreadSafePinned {
            type CtorType =
                impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__unelided>;
            type Error = ::ctor::Infallible;
            #[inline(always)]
            fn ctor_new(args: &'__unelided Self) -> Self::CtorType {
                let mut __param_0 = args;
                unsafe {
                    ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                        crate::detail::__rust_thunk___ZN6crubit4test16ThreadSafePinnedC1ERKS1_(
                            __crubit_dest as *mut ::core::ffi::c_void,
                            __param_0,
                        );
                    })
                }
            }
        }
        impl<'__unelided> ::ctor::CtorNew<(&'__unelided Self,)> for ThreadSafePinned {
            type CtorType =
                impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__unelided>;
            type Error = ::ctor::Infallible;
            #[inline(always)]
            fn ctor_new(args: (&'__unelided Self,)) -> Self::CtorType {
                let (arg,) = args;
                <Self as ::ctor::CtorNew<&'__unelided Self>>::ctor_new(arg)
            }
        }

        impl<'__unelided> ::ctor::CtorNew<::ctor::RvalueReference<'__unelided, Self>> for ThreadSafePinned {
            type CtorType =
                impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__unelided>;
            type Error = ::ctor::Infallible;
            #[inline(always)]
            fn ctor_new(args: ::ctor::RvalueReference<'__unelided, Self>) -> Self::CtorType {
                let mut __param_0 = args;
                unsafe {
                    ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                        crate::detail::__rust_thunk___ZN6crubit4test16ThreadSafePinnedC1EOS1_(
                            __crubit_dest as *mut ::core::ffi::c_void,
                            __param_0,
                        );
                    })
                }
            }
        }
        impl<'__unelided> ::ctor::CtorNew<(::ctor::RvalueReference<'__unelided, Self>,)>
            for ThreadSafePinned
        {
            type CtorType =
                impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__unelided>;
            type Error = ::ctor::Infallible;
            #[inline(always)]
            fn ctor_new(args: (::ctor::RvalueReference<'__unelided, Self>,)) -> Self::CtorType {
                let (arg,) = args;
                <Self as ::ctor::CtorNew<::ctor::RvalueReference<'__unelided, Self>>>::ctor_new(arg)
            }
        }

        impl ::ctor::Assign<&Self> for ThreadSafePinned {
            #[inline(always)]
            fn assign(self: ::core::pin::Pin<&mut Self>, __param_0: &Self) {
                unsafe {
                    crate::detail::__rust_thunk___ZN6crubit4test16ThreadSafePinnedaSERKS1_(
                        self, __param_0,
                    );
                }
            }
        }

        impl ::ctor::Assign<::ctor::RvalueReference<'_, Self>> for ThreadSafePinned {
            #[inline(always)]
            fn assign(
                self: ::core::pin::Pin<&mut Self>,
                __param_0: ::ctor::RvalueReference<'_, Self>,
            ) {
                unsafe {
                    crate::detail::__rust_thunk___ZN6crubit4test16ThreadSafePinnedaSEOS1_(
                        self, __param_0,
                    );
                }
            }
        }

        impl ::ctor::PinnedDrop for ThreadSafePinned {
            #[inline(always)]
            unsafe fn pinned_drop<'a>(self: ::core::pin::Pin<&'a mut Self>) {
                unsafe { crate::detail::__rust_thunk___ZN6crubit4test16ThreadSafePinnedD1Ev(self) }
            }
        }
    }
}

// namespace crubit::test

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test16ThreadSafeStructC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZNK6crubit4test16ThreadSafeStruct8ConstGetEv(
            __this: &crate::crubit::test::ThreadSafeStruct,
        ) -> ::ffi_11::c_int;
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test16ThreadSafeStruct11NonConstGetEv(
            __this: &crate::crubit::test::ThreadSafeStruct,
        ) -> ::ffi_11::c_int;
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test13RegularStructC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZNK6crubit4test13RegularStruct8ConstGetEv(
            __this: *const crate::crubit::test::RegularStruct,
        ) -> ::ffi_11::c_int;
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test13RegularStruct11NonConstGetEv(
            __this: *mut crate::crubit::test::RegularStruct,
        ) -> ::ffi_11::c_int;
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test15ThreadSafeUnpinC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test15ThreadSafeUnpinC1ERKS1_(
            __this: *mut ::core::ffi::c_void,
            __param_0: &crate::crubit::test::ThreadSafeUnpin,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test15ThreadSafeUnpinC1EOS1_(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'_, crate::crubit::test::ThreadSafeUnpin>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test15ThreadSafeUnpinaSERKS1_<
            '__return_lifetime,
        >(
            __this: &mut crate::crubit::test::ThreadSafeUnpin,
            __param_0: &crate::crubit::test::ThreadSafeUnpin,
        ) -> &'__return_lifetime mut crate::crubit::test::ThreadSafeUnpin;
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test15ThreadSafeUnpinaSEOS1_<
            '__return_lifetime,
        >(
            __this: &mut crate::crubit::test::ThreadSafeUnpin,
            __param_0: ::ctor::RvalueReference<'_, crate::crubit::test::ThreadSafeUnpin>,
        ) -> &'__return_lifetime mut crate::crubit::test::ThreadSafeUnpin;
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test16ThreadSafePinnedC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test16ThreadSafePinnedC1ERKS1_<'__unelided>(
            __this: *mut ::core::ffi::c_void,
            __param_0: &'__unelided crate::crubit::test::ThreadSafePinned,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test16ThreadSafePinnedC1EOS1_<'__unelided>(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'__unelided, crate::crubit::test::ThreadSafePinned>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test16ThreadSafePinnedaSERKS1_<
            '__return_lifetime,
        >(
            __this: ::core::pin::Pin<&mut crate::crubit::test::ThreadSafePinned>,
            __param_0: &crate::crubit::test::ThreadSafePinned,
        ) -> ::core::pin::Pin<&'__return_lifetime mut crate::crubit::test::ThreadSafePinned>;
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test16ThreadSafePinnedaSEOS1_<
            '__return_lifetime,
        >(
            __this: ::core::pin::Pin<&mut crate::crubit::test::ThreadSafePinned>,
            __param_0: ::ctor::RvalueReference<'_, crate::crubit::test::ThreadSafePinned>,
        ) -> ::core::pin::Pin<&'__return_lifetime mut crate::crubit::test::ThreadSafePinned>;
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test16ThreadSafePinnedD1Ev<'a>(
            __this: ::core::pin::Pin<&'a mut crate::crubit::test::ThreadSafePinned>,
        );
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::crubit::test::ThreadSafeStruct>() == 4);
    assert!(::core::mem::align_of::<crate::crubit::test::ThreadSafeStruct>() == 4);
    static_assertions::assert_not_impl_any!(crate::crubit::test::ThreadSafeStruct: Copy,Drop);
    assert!(::core::mem::size_of::<crate::crubit::test::RegularStruct>() == 4);
    assert!(::core::mem::align_of::<crate::crubit::test::RegularStruct>() == 4);
    static_assertions::assert_impl_all!(crate::crubit::test::RegularStruct: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::crubit::test::RegularStruct: Drop);
    assert!(::core::mem::offset_of!(crate::crubit::test::RegularStruct, x_) == 0);
    assert!(::core::mem::size_of::<crate::crubit::test::ThreadSafeUnpin>() == 1);
    assert!(::core::mem::align_of::<crate::crubit::test::ThreadSafeUnpin>() == 1);
    static_assertions::assert_not_impl_any!(crate::crubit::test::ThreadSafeUnpin: Copy,Drop);
    assert!(::core::mem::size_of::<crate::crubit::test::ThreadSafePinned>() == 1);
    assert!(::core::mem::align_of::<crate::crubit::test::ThreadSafePinned>() == 1);
    static_assertions::assert_impl_all!(crate::crubit::test::ThreadSafePinned: Drop);
    static_assertions::assert_not_impl_any!(crate::crubit::test::ThreadSafePinned: Copy);
};
