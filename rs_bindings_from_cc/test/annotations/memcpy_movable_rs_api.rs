// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/annotations:memcpy_movable
// Features: custom_ffi_types, non_unpin_ctor, std_unique_ptr, std_vector, supported

#![rustfmt::skip]
#![feature(
    allocator_api,
    cfg_sanitize,
    custom_inner_attributes,
    impl_trait_in_assoc_type,
    negative_impls
)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

pub mod crubit {
    pub mod test {
        /// Tests that a class annotated with `CRUBIT_UNSAFE_MEMCPY_MOVABLE` is
        /// moved via memcpy in Rust bindings even if it has non-trivial move operations.
        ///
        /// Generated from: rs_bindings_from_cc/test/annotations/memcpy_movable.h;l=14
        #[repr(C, align(4))]
        ///CRUBIT_ANNOTATE: cpp_type=crubit :: test :: MemcpyMovableClass
        pub struct MemcpyMovableClass {
            __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 0],
            /// Reason for representing this field as a blob of bytes:
            /// crubit.rs/errors/unknown_attribute: unknown field attributes are only supported with experimental features enabled on //rs_bindings_from_cc/test/annotations:memcpy_movable
            /// Unknown attribute: maybe_unused`
            pub(crate) unused_: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 4],
        }
        impl !Send for MemcpyMovableClass {}
        impl !Sync for MemcpyMovableClass {}
        unsafe impl ::cxx::ExternType for MemcpyMovableClass {
            type Id = ::cxx::type_id!("crubit :: test :: MemcpyMovableClass");
            type Kind = ::cxx::kind::Trivial;
        }

        /// Generated from: rs_bindings_from_cc/test/annotations/memcpy_movable.h;l=16
        impl Default for MemcpyMovableClass {
            #[inline(always)]
            fn default() -> Self {
                let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
                unsafe {
                    crate::detail::__rust_thunk___ZN6crubit4test18MemcpyMovableClassC1Ev(
                        &raw mut tmp as *mut _,
                    );
                    tmp.assume_init()
                }
            }
        }

        /// Generated from: rs_bindings_from_cc/test/annotations/memcpy_movable.h;l=17
        impl Clone for MemcpyMovableClass {
            #[inline(always)]
            fn clone(&self) -> Self {
                let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
                unsafe {
                    crate::detail::__rust_thunk___ZN6crubit4test18MemcpyMovableClassC1ERKS1_(
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

        /// Generated from: rs_bindings_from_cc/test/annotations/memcpy_movable.h;l=18
        impl From<::ctor::RvalueReference<'_, Self>> for MemcpyMovableClass {
            #[inline(always)]
            fn from(args: ::ctor::RvalueReference<'_, Self>) -> Self {
                let mut __param_0 = args;
                let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
                unsafe {
                    crate::detail::__rust_thunk___ZN6crubit4test18MemcpyMovableClassC1EOS1_(
                        &raw mut tmp as *mut _,
                        __param_0,
                    );
                    tmp.assume_init()
                }
            }
        }
        impl ::ctor::CtorNew<::ctor::RvalueReference<'_, Self>> for MemcpyMovableClass {
            type CtorType = Self;
            type Error = ::ctor::Infallible;
            #[inline(always)]
            fn ctor_new(args: ::ctor::RvalueReference<'_, Self>) -> Self::CtorType {
                <Self as From<::ctor::RvalueReference<'_, Self>>>::from(args)
            }
        }

        /// Generated from: rs_bindings_from_cc/test/annotations/memcpy_movable.h;l=19
        impl ::ctor::UnpinAssign<&Self> for MemcpyMovableClass {
            #[inline(always)]
            fn unpin_assign(&mut self, __param_0: &Self) {
                unsafe {
                    crate::detail::__rust_thunk___ZN6crubit4test18MemcpyMovableClassaSERKS1_(
                        self, __param_0,
                    );
                }
            }
        }

        /// Generated from: rs_bindings_from_cc/test/annotations/memcpy_movable.h;l=20
        impl ::ctor::UnpinAssign<::ctor::RvalueReference<'_, Self>> for MemcpyMovableClass {
            #[inline(always)]
            fn unpin_assign(&mut self, __param_0: ::ctor::RvalueReference<'_, Self>) {
                unsafe {
                    crate::detail::__rust_thunk___ZN6crubit4test18MemcpyMovableClassaSEOS1_(
                        self, __param_0,
                    );
                }
            }
        }

        /// Generated from: rs_bindings_from_cc/test/annotations/memcpy_movable.h;l=26
        #[::ctor::recursively_pinned]
        #[repr(C, align(4))]
        ///CRUBIT_ANNOTATE: cpp_type=crubit :: test :: NonMemcpyMovableClass
        pub struct NonMemcpyMovableClass {
            __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 0],
            /// Reason for representing this field as a blob of bytes:
            /// crubit.rs/errors/unknown_attribute: unknown field attributes are only supported with experimental features enabled on //rs_bindings_from_cc/test/annotations:memcpy_movable
            /// Unknown attribute: maybe_unused`
            pub(crate) unused_: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 4],
        }
        impl !Send for NonMemcpyMovableClass {}
        impl !Sync for NonMemcpyMovableClass {}
        unsafe impl ::cxx::ExternType for NonMemcpyMovableClass {
            type Id = ::cxx::type_id!("crubit :: test :: NonMemcpyMovableClass");
            type Kind = ::cxx::kind::Opaque;
        }

        /// Generated from: rs_bindings_from_cc/test/annotations/memcpy_movable.h;l=28
        impl ::ctor::CtorNew<()> for NonMemcpyMovableClass {
            type CtorType = ::ctor::Ctor![Self];
            type Error = ::ctor::Infallible;
            #[inline(always)]
            fn ctor_new(args: ()) -> Self::CtorType {
                let () = args;
                unsafe {
                    ::ctor::FnCtor::new(move |dest: *mut Self| {
                        crate::detail::__rust_thunk___ZN6crubit4test21NonMemcpyMovableClassC1Ev(
                            dest as *mut ::core::ffi::c_void,
                        );
                    })
                }
            }
        }

        /// Generated from: rs_bindings_from_cc/test/annotations/memcpy_movable.h;l=29
        impl<'__unelided> ::ctor::CtorNew<&'__unelided Self> for NonMemcpyMovableClass {
            type CtorType =
                impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__unelided>;
            type Error = ::ctor::Infallible;
            #[inline(always)]
            fn ctor_new(args: &'__unelided Self) -> Self::CtorType {
                let mut __param_0 = args;
                unsafe {
                    ::ctor::FnCtor::new(move |dest: *mut Self| {
                        crate::detail::__rust_thunk___ZN6crubit4test21NonMemcpyMovableClassC1ERKS1_(
                            dest as *mut ::core::ffi::c_void,
                            __param_0,
                        );
                    })
                }
            }
        }
        impl<'__unelided> ::ctor::CtorNew<(&'__unelided Self,)> for NonMemcpyMovableClass {
            type CtorType =
                impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__unelided>;
            type Error = ::ctor::Infallible;
            #[inline(always)]
            fn ctor_new(args: (&'__unelided Self,)) -> Self::CtorType {
                let (arg,) = args;
                <Self as ::ctor::CtorNew<&'__unelided Self>>::ctor_new(arg)
            }
        }

        /// Generated from: rs_bindings_from_cc/test/annotations/memcpy_movable.h;l=30
        impl<'__unelided> ::ctor::CtorNew<::ctor::RvalueReference<'__unelided, Self>>
            for NonMemcpyMovableClass
        {
            type CtorType =
                impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__unelided>;
            type Error = ::ctor::Infallible;
            #[inline(always)]
            fn ctor_new(args: ::ctor::RvalueReference<'__unelided, Self>) -> Self::CtorType {
                let mut __param_0 = args;
                unsafe {
                    ::ctor::FnCtor::new(move |dest: *mut Self| {
                        crate::detail::__rust_thunk___ZN6crubit4test21NonMemcpyMovableClassC1EOS1_(
                            dest as *mut ::core::ffi::c_void,
                            __param_0,
                        );
                    })
                }
            }
        }
        impl<'__unelided> ::ctor::CtorNew<(::ctor::RvalueReference<'__unelided, Self>,)>
            for NonMemcpyMovableClass
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

        /// Generated from: rs_bindings_from_cc/test/annotations/memcpy_movable.h;l=31
        impl ::ctor::Assign<&Self> for NonMemcpyMovableClass {
            #[inline(always)]
            fn assign(self: ::core::pin::Pin<&mut Self>, __param_0: &Self) {
                unsafe {
                    crate::detail::__rust_thunk___ZN6crubit4test21NonMemcpyMovableClassaSERKS1_(
                        self, __param_0,
                    );
                }
            }
        }

        /// Generated from: rs_bindings_from_cc/test/annotations/memcpy_movable.h;l=34
        impl ::ctor::Assign<::ctor::RvalueReference<'_, Self>> for NonMemcpyMovableClass {
            #[inline(always)]
            fn assign(
                self: ::core::pin::Pin<&mut Self>,
                __param_0: ::ctor::RvalueReference<'_, Self>,
            ) {
                unsafe {
                    crate::detail::__rust_thunk___ZN6crubit4test21NonMemcpyMovableClassaSEOS1_(
                        self, __param_0,
                    );
                }
            }
        }

        /// Generated from: rs_bindings_from_cc/test/annotations/memcpy_movable.h;l=40
        #[inline(always)]
        pub fn ReturnsMemcpyMovable() -> crate::crubit::test::MemcpyMovableClass {
            unsafe {
                let mut __return =
                    ::core::mem::MaybeUninit::<crate::crubit::test::MemcpyMovableClass>::uninit();
                crate::detail::__rust_thunk___ZN6crubit4test20ReturnsMemcpyMovableEv(
                    &raw mut __return as *mut ::core::ffi::c_void,
                );
                __return.assume_init()
            }
        }

        /// Generated from: rs_bindings_from_cc/test/annotations/memcpy_movable.h;l=43
        #[inline(always)]
        pub fn ReturnsNonMemcpyMovable() -> ::ctor::Ctor![crate::crubit::test::NonMemcpyMovableClass]
        {
            unsafe {
                ::ctor::FnCtor::new(move |dest: *mut crate::crubit::test::NonMemcpyMovableClass| {
                    crate::detail::__rust_thunk___ZN6crubit4test23ReturnsNonMemcpyMovableEv(
                        dest as *mut ::core::ffi::c_void,
                    );
                })
            }
        }

        /// Generated from: rs_bindings_from_cc/test/annotations/memcpy_movable.h;l=47
        #[inline(always)]
        pub fn AcceptsMemcpyMovable(mut __param_0: crate::crubit::test::MemcpyMovableClass) {
            unsafe {
                crate::detail::__rust_thunk___ZN6crubit4test20AcceptsMemcpyMovableENS0_18MemcpyMovableClassE(&mut __param_0)
            }
        }

        /// Generated from: rs_bindings_from_cc/test/annotations/memcpy_movable.h;l=48
        #[inline(always)]
        pub fn AcceptsNonMemcpyMovable(
            __param_0: ::ctor::Ctor![crate::crubit::test::NonMemcpyMovableClass],
        ) {
            unsafe {
                crate::detail::__rust_thunk___ZN6crubit4test23AcceptsNonMemcpyMovableENS0_21NonMemcpyMovableClassE(::core::pin::Pin::into_inner_unchecked(::ctor::emplace!(__param_0)))
            }
        }

        // Uncomment to see bindings generation error:
        //   Dynamic classes (classes with virtual functions or bases) are not movable
        //   via memcpy.
        // class CRUBIT_UNSAFE_MEMCPY_MOVABLE IllegallyMemcpyMovable {
        //  public:
        //   virtual ~IllegallyMemcpyMovable() = default;
        // };
    }
}

// namespace crubit::test

// Generated from: nowhere/llvm/src/libcxx/include/__type_traits/integral_constant.h;l=21
// Error while generating bindings for struct 'std::integral_constant<bool, false>':
// Can't generate bindings for std::integral_constant<bool, false>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/annotations:memcpy_movable needs [//features:wrapper] for std::integral_constant<bool, false> (crate::__CcTemplateInstNSt3__u17integral_constantIbLb0EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__type_traits/integral_constant.h;l=21
// Error while generating bindings for struct 'std::integral_constant<bool, true>':
// Can't generate bindings for std::integral_constant<bool, true>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/annotations:memcpy_movable needs [//features:wrapper] for std::integral_constant<bool, true> (crate::__CcTemplateInstNSt3__u17integral_constantIbLb1EEE is a template instantiation)

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test18MemcpyMovableClassC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test18MemcpyMovableClassC1ERKS1_(
            __this: *mut ::core::ffi::c_void,
            __param_0: &crate::crubit::test::MemcpyMovableClass,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test18MemcpyMovableClassC1EOS1_(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'_, crate::crubit::test::MemcpyMovableClass>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test18MemcpyMovableClassaSERKS1_<
            '__return_lifetime,
        >(
            __this: &mut crate::crubit::test::MemcpyMovableClass,
            __param_0: &crate::crubit::test::MemcpyMovableClass,
        ) -> &'__return_lifetime mut crate::crubit::test::MemcpyMovableClass;
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test18MemcpyMovableClassaSEOS1_<
            '__return_lifetime,
        >(
            __this: &mut crate::crubit::test::MemcpyMovableClass,
            __param_0: ::ctor::RvalueReference<'_, crate::crubit::test::MemcpyMovableClass>,
        ) -> &'__return_lifetime mut crate::crubit::test::MemcpyMovableClass;
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test21NonMemcpyMovableClassC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test21NonMemcpyMovableClassC1ERKS1_<
            '__unelided,
        >(
            __this: *mut ::core::ffi::c_void,
            __param_0: &'__unelided crate::crubit::test::NonMemcpyMovableClass,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test21NonMemcpyMovableClassC1EOS1_<
            '__unelided,
        >(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<
                '__unelided,
                crate::crubit::test::NonMemcpyMovableClass,
            >,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test21NonMemcpyMovableClassaSERKS1_<
            '__return_lifetime,
        >(
            __this: ::core::pin::Pin<&mut crate::crubit::test::NonMemcpyMovableClass>,
            __param_0: &crate::crubit::test::NonMemcpyMovableClass,
        ) -> ::core::pin::Pin<&'__return_lifetime mut crate::crubit::test::NonMemcpyMovableClass>;
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test21NonMemcpyMovableClassaSEOS1_<
            '__return_lifetime,
        >(
            __this: ::core::pin::Pin<&mut crate::crubit::test::NonMemcpyMovableClass>,
            __param_0: ::ctor::RvalueReference<'_, crate::crubit::test::NonMemcpyMovableClass>,
        ) -> ::core::pin::Pin<&'__return_lifetime mut crate::crubit::test::NonMemcpyMovableClass>;
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test20ReturnsMemcpyMovableEv(
            __return: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test23ReturnsNonMemcpyMovableEv(
            __return: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test20AcceptsMemcpyMovableENS0_18MemcpyMovableClassE(
            __param_0: &mut crate::crubit::test::MemcpyMovableClass,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test23AcceptsNonMemcpyMovableENS0_21NonMemcpyMovableClassE(
            __param_0: &mut crate::crubit::test::NonMemcpyMovableClass,
        );
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::crubit::test::MemcpyMovableClass>() == 4);
    assert!(::core::mem::align_of::<crate::crubit::test::MemcpyMovableClass>() == 4);
    static_assertions::assert_not_impl_any!(crate::crubit::test::MemcpyMovableClass: Copy,Drop);
    assert!(::core::mem::offset_of!(crate::crubit::test::MemcpyMovableClass, unused_) == 0);
    assert!(::core::mem::size_of::<crate::crubit::test::NonMemcpyMovableClass>() == 4);
    assert!(::core::mem::align_of::<crate::crubit::test::NonMemcpyMovableClass>() == 4);
    static_assertions::assert_not_impl_any!(crate::crubit::test::NonMemcpyMovableClass: Copy,Drop);
    assert!(::core::mem::offset_of!(crate::crubit::test::NonMemcpyMovableClass, unused_) == 0);
};
