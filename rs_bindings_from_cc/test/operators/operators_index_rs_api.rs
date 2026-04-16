// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/operators:operators_index
// Features: fmt, leading_colons_for_cpp_type, supported, types

#![rustfmt::skip]
#![feature(custom_inner_attributes, impl_trait_in_assoc_type, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![allow(deprecated)]
#![deny(warnings)]

pub mod crubit {
    pub mod test {
        /// Generated from: rs_bindings_from_cc/test/operators/operators_index.h;l=10
        #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
        #[repr(C)]
        ///CRUBIT_ANNOTATE: cpp_type=:: crubit :: test :: ItemUnpin
        pub struct ItemUnpin {
            pub value: ::ffi_11::c_int,
        }
        impl !Send for ItemUnpin {}
        impl !Sync for ItemUnpin {}
        unsafe impl ::cxx::ExternType for ItemUnpin {
            type Id = ::cxx::type_id!(":: crubit :: test :: ItemUnpin");
            type Kind = ::cxx::kind::Trivial;
        }

        /// Generated from: rs_bindings_from_cc/test/operators/operators_index.h;l=10
        impl Default for ItemUnpin {
            #[inline(always)]
            fn default() -> Self {
                let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
                unsafe {
                    crate::detail::__rust_thunk___ZN6crubit4test9ItemUnpinC1Ev(
                        &raw mut tmp as *mut _,
                    );
                    tmp.assume_init()
                }
            }
        }

        /// Generated from: rs_bindings_from_cc/test/operators/operators_index.h;l=14
        #[::ctor::recursively_pinned(PinnedDrop)]
        #[repr(C)]
        ///CRUBIT_ANNOTATE: cpp_type=:: crubit :: test :: ItemNonUnpin
        pub struct ItemNonUnpin {
            pub value: ::ffi_11::c_int,
        }
        impl !Send for ItemNonUnpin {}
        impl !Sync for ItemNonUnpin {}
        unsafe impl ::cxx::ExternType for ItemNonUnpin {
            type Id = ::cxx::type_id!(":: crubit :: test :: ItemNonUnpin");
            type Kind = ::cxx::kind::Opaque;
        }

        /// Generated from: rs_bindings_from_cc/test/operators/operators_index.h;l=14
        impl ::ctor::CtorNew<()> for ItemNonUnpin {
            type CtorType = ::ctor::Ctor![Self];
            type Error = ::ctor::Infallible;
            #[inline(always)]
            fn ctor_new(args: ()) -> Self::CtorType {
                let () = args;
                unsafe {
                    ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                        crate::detail::__rust_thunk___ZN6crubit4test12ItemNonUnpinC1Ev(
                            __crubit_dest as *mut ::core::ffi::c_void,
                        );
                    })
                }
            }
        }

        /// Generated from: rs_bindings_from_cc/test/operators/operators_index.h;l=14
        impl<'__unelided> ::ctor::CtorNew<&'__unelided Self> for ItemNonUnpin {
            type CtorType =
                impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__unelided>;
            type Error = ::ctor::Infallible;
            #[inline(always)]
            fn ctor_new(args: &'__unelided Self) -> Self::CtorType {
                let mut __param_0 = args;
                unsafe {
                    ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                        crate::detail::__rust_thunk___ZN6crubit4test12ItemNonUnpinC1ERKS1_(
                            __crubit_dest as *mut ::core::ffi::c_void,
                            __param_0,
                        );
                    })
                }
            }
        }
        impl<'__unelided> ::ctor::CtorNew<(&'__unelided Self,)> for ItemNonUnpin {
            type CtorType =
                impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__unelided>;
            type Error = ::ctor::Infallible;
            #[inline(always)]
            fn ctor_new(args: (&'__unelided Self,)) -> Self::CtorType {
                let (arg,) = args;
                <Self as ::ctor::CtorNew<&'__unelided Self>>::ctor_new(arg)
            }
        }

        /// Generated from: rs_bindings_from_cc/test/operators/operators_index.h;l=14
        impl ::ctor::Assign<&Self> for ItemNonUnpin {
            #[inline(always)]
            fn assign(self: ::core::pin::Pin<&mut Self>, __param_0: &Self) {
                unsafe {
                    crate::detail::__rust_thunk___ZN6crubit4test12ItemNonUnpinaSERKS1_(
                        self, __param_0,
                    );
                }
            }
        }

        /// Generated from: rs_bindings_from_cc/test/operators/operators_index.h;l=17
        impl ::ctor::PinnedDrop for ItemNonUnpin {
            #[inline(always)]
            unsafe fn pinned_drop<'a>(self: ::core::pin::Pin<&'a mut Self>) {
                unsafe { crate::detail::__rust_thunk___ZN6crubit4test12ItemNonUnpinD1Ev(self) }
            }
        }

        /// Generated from: rs_bindings_from_cc/test/operators/operators_index.h;l=20
        #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
        #[repr(C, align(4))]
        ///CRUBIT_ANNOTATE: cpp_type=:: crubit :: test :: ContainerUnpinItemUnpin
        pub struct ContainerUnpinItemUnpin {
            __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
            /// Reason for representing this field as a blob of bytes:
            /// Types of non-public C++ fields can be elided away
            pub(crate) items_: [::core::mem::MaybeUninit<u8>; 40],
        }
        impl !Send for ContainerUnpinItemUnpin {}
        impl !Sync for ContainerUnpinItemUnpin {}
        unsafe impl ::cxx::ExternType for ContainerUnpinItemUnpin {
            type Id = ::cxx::type_id!(":: crubit :: test :: ContainerUnpinItemUnpin");
            type Kind = ::cxx::kind::Trivial;
        }

        /// Generated from: rs_bindings_from_cc/test/operators/operators_index.h;l=22
        impl Default for ContainerUnpinItemUnpin {
            #[inline(always)]
            fn default() -> Self {
                let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
                unsafe {
                    crate::detail::__rust_thunk___ZN6crubit4test23ContainerUnpinItemUnpinC1Ev(
                        &raw mut tmp as *mut _,
                    );
                    tmp.assume_init()
                }
            }
        }

        /// Generated from: rs_bindings_from_cc/test/operators/operators_index.h;l=23
        impl ::operator::CcIndex<::ffi_11::c_uint> for ContainerUnpinItemUnpin {
            type Output<'ctnr> = &'ctnr crate::crubit::test::ItemUnpin;
            #[inline(always)]
            fn cc_index<'ctnr>(&'ctnr self, index: ::ffi_11::c_uint) -> Self::Output<'ctnr> {
                unsafe {
                    crate::detail::__rust_thunk___ZNK6crubit4test23ContainerUnpinItemUnpinixEj(
                        self, index,
                    )
                }
            }
        }
        impl ::core::ops::Index<::ffi_11::c_uint> for ContainerUnpinItemUnpin {
            type Output = crate::crubit::test::ItemUnpin;
            #[inline(always)]
            fn index(&self, index: ::ffi_11::c_uint) -> &Self::Output {
                ::operator::CcIndex::cc_index(self, index)
            }
        }

        /// Generated from: rs_bindings_from_cc/test/operators/operators_index.h;l=26
        impl ::operator::CcIndexMut<::ffi_11::c_uint> for ContainerUnpinItemUnpin {
            type Output<'ctnr> = &'ctnr mut crate::crubit::test::ItemUnpin;
            #[inline(always)]
            fn cc_index_mut<'ctnr>(
                self: ::core::pin::Pin<&'ctnr mut Self>,
                index: ::ffi_11::c_uint,
            ) -> Self::Output<'ctnr> {
                unsafe {
                    crate::detail::__rust_thunk___ZN6crubit4test23ContainerUnpinItemUnpinixEj(
                        self.get_unchecked_mut(),
                        index,
                    )
                }
            }
        }
        impl ::core::ops::IndexMut<::ffi_11::c_uint> for ContainerUnpinItemUnpin {
            #[inline(always)]
            fn index_mut(&mut self, index: ::ffi_11::c_uint) -> &mut Self::Output {
                ::operator::CcIndexMut::cc_index_mut(::core::pin::Pin::new(self), index)
            }
        }

        /// # Safety
        ///
        /// To call a function that accepts this type, you must uphold these requirements:
        /// * Document why the following public unsafe fields of this type cannot be misused by callee:
        ///   * `items_storage_`: Rust type is unknown; safety requirements cannot be automatically generated: Unsupported type 'crubit::test::ItemNonUnpin[10]': Unsupported clang::Type class 'ConstantArray'
        ///   * `items_`: raw pointer
        ///
        /// Generated from: rs_bindings_from_cc/test/operators/operators_index.h;l=32
        #[::ctor::recursively_pinned(PinnedDrop)]
        #[repr(C, align(8))]
        ///CRUBIT_ANNOTATE: cpp_type=:: crubit :: test :: ContainerUnpinItemNonUnpin
        pub struct ContainerUnpinItemNonUnpin {
            __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 0],
            /// Reason for representing this field as a blob of bytes:
            /// Unsupported type 'crubit::test::ItemNonUnpin[10]': Unsupported clang::Type class 'ConstantArray'
            pub(crate) items_storage_: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 40],
            pub items_: *mut crate::crubit::test::ItemNonUnpin,
        }
        impl !Send for ContainerUnpinItemNonUnpin {}
        impl !Sync for ContainerUnpinItemNonUnpin {}
        unsafe impl ::cxx::ExternType for ContainerUnpinItemNonUnpin {
            type Id = ::cxx::type_id!(":: crubit :: test :: ContainerUnpinItemNonUnpin");
            type Kind = ::cxx::kind::Opaque;
        }

        /// Generated from: rs_bindings_from_cc/test/operators/operators_index.h;l=32
        impl<'__unelided> ::ctor::CtorNew<&'__unelided Self> for ContainerUnpinItemNonUnpin {
            type CtorType =
                impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__unelided>;
            type Error = ::ctor::Infallible;
            #[inline(always)]
            fn ctor_new(args: &'__unelided Self) -> Self::CtorType {
                let mut __param_0 = args;
                unsafe {
                    ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                        crate::detail::__rust_thunk___ZN6crubit4test26ContainerUnpinItemNonUnpinC1ERKS1_(__crubit_dest as*mut::core::ffi::c_void,__param_0);
                    })
                }
            }
        }
        impl<'__unelided> ::ctor::CtorNew<(&'__unelided Self,)> for ContainerUnpinItemNonUnpin {
            type CtorType =
                impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__unelided>;
            type Error = ::ctor::Infallible;
            #[inline(always)]
            fn ctor_new(args: (&'__unelided Self,)) -> Self::CtorType {
                let (arg,) = args;
                <Self as ::ctor::CtorNew<&'__unelided Self>>::ctor_new(arg)
            }
        }

        /// Generated from: rs_bindings_from_cc/test/operators/operators_index.h;l=32
        impl<'__unelided> ::ctor::CtorNew<::ctor::RvalueReference<'__unelided, Self>>
            for ContainerUnpinItemNonUnpin
        {
            type CtorType =
                impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__unelided>;
            type Error = ::ctor::Infallible;
            #[inline(always)]
            fn ctor_new(args: ::ctor::RvalueReference<'__unelided, Self>) -> Self::CtorType {
                let mut __param_0 = args;
                unsafe {
                    ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                        crate::detail::__rust_thunk___ZN6crubit4test26ContainerUnpinItemNonUnpinC1EOS1_(__crubit_dest as*mut::core::ffi::c_void,__param_0);
                    })
                }
            }
        }
        impl<'__unelided> ::ctor::CtorNew<(::ctor::RvalueReference<'__unelided, Self>,)>
            for ContainerUnpinItemNonUnpin
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

        /// Generated from: rs_bindings_from_cc/test/operators/operators_index.h;l=32
        impl ::ctor::PinnedDrop for ContainerUnpinItemNonUnpin {
            #[inline(always)]
            unsafe fn pinned_drop<'a>(self: ::core::pin::Pin<&'a mut Self>) {
                unsafe {
                    crate::detail::__rust_thunk___ZN6crubit4test26ContainerUnpinItemNonUnpinD1Ev(
                        self,
                    )
                }
            }
        }

        /// Generated from: rs_bindings_from_cc/test/operators/operators_index.h;l=32
        impl ::ctor::Assign<&Self> for ContainerUnpinItemNonUnpin {
            #[inline(always)]
            fn assign(self: ::core::pin::Pin<&mut Self>, __param_0: &Self) {
                unsafe {
                    crate::detail::__rust_thunk___ZN6crubit4test26ContainerUnpinItemNonUnpinaSERKS1_(self,__param_0);
                }
            }
        }

        /// Generated from: rs_bindings_from_cc/test/operators/operators_index.h;l=32
        impl ::ctor::Assign<::ctor::RvalueReference<'_, Self>> for ContainerUnpinItemNonUnpin {
            #[inline(always)]
            fn assign(
                self: ::core::pin::Pin<&mut Self>,
                __param_0: ::ctor::RvalueReference<'_, Self>,
            ) {
                unsafe {
                    crate::detail::__rust_thunk___ZN6crubit4test26ContainerUnpinItemNonUnpinaSEOS1_(
                        self, __param_0,
                    );
                }
            }
        }

        /// Generated from: rs_bindings_from_cc/test/operators/operators_index.h;l=34
        impl ::ctor::CtorNew<()> for ContainerUnpinItemNonUnpin {
            type CtorType = ::ctor::Ctor![Self];
            type Error = ::ctor::Infallible;
            #[inline(always)]
            fn ctor_new(args: ()) -> Self::CtorType {
                let () = args;
                unsafe {
                    ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                        crate::detail::__rust_thunk___ZN6crubit4test26ContainerUnpinItemNonUnpinC1Ev(__crubit_dest as*mut::core::ffi::c_void);
                    })
                }
            }
        }

        /// Generated from: rs_bindings_from_cc/test/operators/operators_index.h;l=35
        impl ::ctor::UnsafeCtorNew<*mut crate::crubit::test::ItemNonUnpin> for ContainerUnpinItemNonUnpin {
            type CtorType = ::ctor::Ctor![Self];
            type Error = ::ctor::Infallible;
            #[inline(always)]
            unsafe fn ctor_new(args: *mut crate::crubit::test::ItemNonUnpin) -> Self::CtorType {
                let mut items = args;
                unsafe {
                    ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                        crate::detail::__rust_thunk___ZN6crubit4test26ContainerUnpinItemNonUnpinC1EPNS0_12ItemNonUnpinE(__crubit_dest as*mut::core::ffi::c_void,items);
                    })
                }
            }
        }
        impl ::ctor::UnsafeCtorNew<(*mut crate::crubit::test::ItemNonUnpin,)>
            for ContainerUnpinItemNonUnpin
        {
            type CtorType = ::ctor::Ctor![Self];
            type Error = ::ctor::Infallible;
            #[inline(always)]
            unsafe fn ctor_new(args: (*mut crate::crubit::test::ItemNonUnpin,)) -> Self::CtorType {
                let (arg,) = args;
                unsafe {
                    <Self as::ctor::UnsafeCtorNew<*mut crate::crubit::test::ItemNonUnpin>>::ctor_new(arg)
                }
            }
        }

        /// Generated from: rs_bindings_from_cc/test/operators/operators_index.h;l=37
        impl ::operator::CcIndex<::ffi_11::c_uint> for ContainerUnpinItemNonUnpin {
            type Output<'ctnr> = &'ctnr crate::crubit::test::ItemNonUnpin;
            #[inline(always)]
            fn cc_index<'ctnr>(&'ctnr self, index: ::ffi_11::c_uint) -> Self::Output<'ctnr> {
                unsafe {
                    crate::detail::__rust_thunk___ZNK6crubit4test26ContainerUnpinItemNonUnpinixEj(
                        self, index,
                    )
                }
            }
        }
        impl<'ctnr> ::core::ops::Index<::ffi_11::c_uint>
            for ::core::pin::Pin<&'ctnr mut ContainerUnpinItemNonUnpin>
        {
            type Output = crate::crubit::test::ItemNonUnpin;
            #[inline(always)]
            fn index(&self, index: ::ffi_11::c_uint) -> &Self::Output {
                ::operator::CcIndex::cc_index(self.as_ref().get_ref(), index)
            }
        }

        /// Generated from: rs_bindings_from_cc/test/operators/operators_index.h;l=40
        impl ::operator::CcIndexMut<::ffi_11::c_uint> for ContainerUnpinItemNonUnpin {
            type Output<'ctnr> = ::core::pin::Pin<&'ctnr mut crate::crubit::test::ItemNonUnpin>;
            #[inline(always)]
            fn cc_index_mut<'ctnr>(
                self: ::core::pin::Pin<&'ctnr mut Self>,
                index: ::ffi_11::c_uint,
            ) -> Self::Output<'ctnr> {
                unsafe {
                    crate::detail::__rust_thunk___ZN6crubit4test26ContainerUnpinItemNonUnpinixEj(
                        self, index,
                    )
                }
            }
        }

        /// Generated from: rs_bindings_from_cc/test/operators/operators_index.h;l=47
        #[::ctor::recursively_pinned(PinnedDrop)]
        #[repr(C, align(4))]
        ///CRUBIT_ANNOTATE: cpp_type=:: crubit :: test :: ContainerNonUnpinItemUnpin
        pub struct ContainerNonUnpinItemUnpin {
            __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 0],
            /// Reason for representing this field as a blob of bytes:
            /// Types of non-public C++ fields can be elided away
            pub(crate) items_: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 40],
        }
        impl !Send for ContainerNonUnpinItemUnpin {}
        impl !Sync for ContainerNonUnpinItemUnpin {}
        unsafe impl ::cxx::ExternType for ContainerNonUnpinItemUnpin {
            type Id = ::cxx::type_id!(":: crubit :: test :: ContainerNonUnpinItemUnpin");
            type Kind = ::cxx::kind::Opaque;
        }

        /// Generated from: rs_bindings_from_cc/test/operators/operators_index.h;l=47
        impl<'__unelided> ::ctor::CtorNew<&'__unelided Self> for ContainerNonUnpinItemUnpin {
            type CtorType =
                impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__unelided>;
            type Error = ::ctor::Infallible;
            #[inline(always)]
            fn ctor_new(args: &'__unelided Self) -> Self::CtorType {
                let mut __param_0 = args;
                unsafe {
                    ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                        crate::detail::__rust_thunk___ZN6crubit4test26ContainerNonUnpinItemUnpinC1ERKS1_(__crubit_dest as*mut::core::ffi::c_void,__param_0);
                    })
                }
            }
        }
        impl<'__unelided> ::ctor::CtorNew<(&'__unelided Self,)> for ContainerNonUnpinItemUnpin {
            type CtorType =
                impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__unelided>;
            type Error = ::ctor::Infallible;
            #[inline(always)]
            fn ctor_new(args: (&'__unelided Self,)) -> Self::CtorType {
                let (arg,) = args;
                <Self as ::ctor::CtorNew<&'__unelided Self>>::ctor_new(arg)
            }
        }

        /// Generated from: rs_bindings_from_cc/test/operators/operators_index.h;l=47
        impl ::ctor::Assign<&Self> for ContainerNonUnpinItemUnpin {
            #[inline(always)]
            fn assign(self: ::core::pin::Pin<&mut Self>, __param_0: &Self) {
                unsafe {
                    crate::detail::__rust_thunk___ZN6crubit4test26ContainerNonUnpinItemUnpinaSERKS1_(self,__param_0);
                }
            }
        }

        /// Generated from: rs_bindings_from_cc/test/operators/operators_index.h;l=49
        impl ::ctor::CtorNew<()> for ContainerNonUnpinItemUnpin {
            type CtorType = ::ctor::Ctor![Self];
            type Error = ::ctor::Infallible;
            #[inline(always)]
            fn ctor_new(args: ()) -> Self::CtorType {
                let () = args;
                unsafe {
                    ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                        crate::detail::__rust_thunk___ZN6crubit4test26ContainerNonUnpinItemUnpinC1Ev(__crubit_dest as*mut::core::ffi::c_void);
                    })
                }
            }
        }

        /// Generated from: rs_bindings_from_cc/test/operators/operators_index.h;l=51
        impl ::ctor::PinnedDrop for ContainerNonUnpinItemUnpin {
            #[inline(always)]
            unsafe fn pinned_drop<'a>(self: ::core::pin::Pin<&'a mut Self>) {
                unsafe {
                    crate::detail::__rust_thunk___ZN6crubit4test26ContainerNonUnpinItemUnpinD1Ev(
                        self,
                    )
                }
            }
        }

        /// Generated from: rs_bindings_from_cc/test/operators/operators_index.h;l=53
        impl ::operator::CcIndex<::ffi_11::c_uint> for ContainerNonUnpinItemUnpin {
            type Output<'ctnr> = &'ctnr crate::crubit::test::ItemUnpin;
            #[inline(always)]
            fn cc_index<'ctnr>(&'ctnr self, index: ::ffi_11::c_uint) -> Self::Output<'ctnr> {
                unsafe {
                    crate::detail::__rust_thunk___ZNK6crubit4test26ContainerNonUnpinItemUnpinixEj(
                        self, index,
                    )
                }
            }
        }
        impl<'ctnr> ::core::ops::Index<::ffi_11::c_uint>
            for ::core::pin::Pin<&'ctnr mut ContainerNonUnpinItemUnpin>
        {
            type Output = crate::crubit::test::ItemUnpin;
            #[inline(always)]
            fn index(&self, index: ::ffi_11::c_uint) -> &Self::Output {
                ::operator::CcIndex::cc_index(self.as_ref().get_ref(), index)
            }
        }

        /// Generated from: rs_bindings_from_cc/test/operators/operators_index.h;l=56
        impl ::operator::CcIndexMut<::ffi_11::c_uint> for ContainerNonUnpinItemUnpin {
            type Output<'ctnr> = &'ctnr mut crate::crubit::test::ItemUnpin;
            #[inline(always)]
            fn cc_index_mut<'ctnr>(
                self: ::core::pin::Pin<&'ctnr mut Self>,
                index: ::ffi_11::c_uint,
            ) -> Self::Output<'ctnr> {
                unsafe {
                    crate::detail::__rust_thunk___ZN6crubit4test26ContainerNonUnpinItemUnpinixEj(
                        self, index,
                    )
                }
            }
        }
        impl<'ctnr> ::core::ops::IndexMut<::ffi_11::c_uint>
            for ::core::pin::Pin<&'ctnr mut ContainerNonUnpinItemUnpin>
        {
            #[inline(always)]
            fn index_mut(&mut self, index: ::ffi_11::c_uint) -> &mut Self::Output {
                ::operator::CcIndexMut::cc_index_mut(self.as_mut(), index)
            }
        }

        /// Generated from: rs_bindings_from_cc/test/operators/operators_index.h;l=62
        #[::ctor::recursively_pinned(PinnedDrop)]
        #[repr(C, align(4))]
        ///CRUBIT_ANNOTATE: cpp_type=:: crubit :: test :: ContainerNonUnpinItemNonUnpin
        pub struct ContainerNonUnpinItemNonUnpin {
            __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 0],
            /// Reason for representing this field as a blob of bytes:
            /// Types of non-public C++ fields can be elided away
            pub(crate) items_: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 40],
        }
        impl !Send for ContainerNonUnpinItemNonUnpin {}
        impl !Sync for ContainerNonUnpinItemNonUnpin {}
        unsafe impl ::cxx::ExternType for ContainerNonUnpinItemNonUnpin {
            type Id = ::cxx::type_id!(":: crubit :: test :: ContainerNonUnpinItemNonUnpin");
            type Kind = ::cxx::kind::Opaque;
        }

        /// Generated from: rs_bindings_from_cc/test/operators/operators_index.h;l=62
        impl<'__unelided> ::ctor::CtorNew<&'__unelided Self> for ContainerNonUnpinItemNonUnpin {
            type CtorType =
                impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__unelided>;
            type Error = ::ctor::Infallible;
            #[inline(always)]
            fn ctor_new(args: &'__unelided Self) -> Self::CtorType {
                let mut __param_0 = args;
                unsafe {
                    ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                        crate::detail::__rust_thunk___ZN6crubit4test29ContainerNonUnpinItemNonUnpinC1ERKS1_(__crubit_dest as*mut::core::ffi::c_void,__param_0);
                    })
                }
            }
        }
        impl<'__unelided> ::ctor::CtorNew<(&'__unelided Self,)> for ContainerNonUnpinItemNonUnpin {
            type CtorType =
                impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__unelided>;
            type Error = ::ctor::Infallible;
            #[inline(always)]
            fn ctor_new(args: (&'__unelided Self,)) -> Self::CtorType {
                let (arg,) = args;
                <Self as ::ctor::CtorNew<&'__unelided Self>>::ctor_new(arg)
            }
        }

        /// Generated from: rs_bindings_from_cc/test/operators/operators_index.h;l=62
        impl ::ctor::Assign<&Self> for ContainerNonUnpinItemNonUnpin {
            #[inline(always)]
            fn assign(self: ::core::pin::Pin<&mut Self>, __param_0: &Self) {
                unsafe {
                    crate::detail::__rust_thunk___ZN6crubit4test29ContainerNonUnpinItemNonUnpinaSERKS1_(self,__param_0);
                }
            }
        }

        /// Generated from: rs_bindings_from_cc/test/operators/operators_index.h;l=64
        impl ::ctor::CtorNew<()> for ContainerNonUnpinItemNonUnpin {
            type CtorType = ::ctor::Ctor![Self];
            type Error = ::ctor::Infallible;
            #[inline(always)]
            fn ctor_new(args: ()) -> Self::CtorType {
                let () = args;
                unsafe {
                    ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                        crate::detail::__rust_thunk___ZN6crubit4test29ContainerNonUnpinItemNonUnpinC1Ev(__crubit_dest as*mut::core::ffi::c_void);
                    })
                }
            }
        }

        /// Generated from: rs_bindings_from_cc/test/operators/operators_index.h;l=66
        impl ::ctor::PinnedDrop for ContainerNonUnpinItemNonUnpin {
            #[inline(always)]
            unsafe fn pinned_drop<'a>(self: ::core::pin::Pin<&'a mut Self>) {
                unsafe {
                    crate::detail::__rust_thunk___ZN6crubit4test29ContainerNonUnpinItemNonUnpinD1Ev(
                        self,
                    )
                }
            }
        }

        /// Generated from: rs_bindings_from_cc/test/operators/operators_index.h;l=68
        impl ::operator::CcIndex<::ffi_11::c_uint> for ContainerNonUnpinItemNonUnpin {
            type Output<'ctnr> = &'ctnr crate::crubit::test::ItemNonUnpin;
            #[inline(always)]
            fn cc_index<'ctnr>(&'ctnr self, index: ::ffi_11::c_uint) -> Self::Output<'ctnr> {
                unsafe {
                    crate::detail::__rust_thunk___ZNK6crubit4test29ContainerNonUnpinItemNonUnpinixEj(
                        self, index,
                    )
                }
            }
        }
        impl<'ctnr> ::core::ops::Index<::ffi_11::c_uint>
            for ::core::pin::Pin<&'ctnr mut ContainerNonUnpinItemNonUnpin>
        {
            type Output = crate::crubit::test::ItemNonUnpin;
            #[inline(always)]
            fn index(&self, index: ::ffi_11::c_uint) -> &Self::Output {
                ::operator::CcIndex::cc_index(self.as_ref().get_ref(), index)
            }
        }

        /// Generated from: rs_bindings_from_cc/test/operators/operators_index.h;l=71
        impl ::operator::CcIndexMut<::ffi_11::c_uint> for ContainerNonUnpinItemNonUnpin {
            type Output<'ctnr> = ::core::pin::Pin<&'ctnr mut crate::crubit::test::ItemNonUnpin>;
            #[inline(always)]
            fn cc_index_mut<'ctnr>(
                self: ::core::pin::Pin<&'ctnr mut Self>,
                index: ::ffi_11::c_uint,
            ) -> Self::Output<'ctnr> {
                unsafe {
                    crate::detail::__rust_thunk___ZN6crubit4test29ContainerNonUnpinItemNonUnpinixEj(
                        self, index,
                    )
                }
            }
        }

        /// Generated from: rs_bindings_from_cc/test/operators/operators_index.h;l=77
        #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
        #[repr(C)]
        ///CRUBIT_ANNOTATE: cpp_type=:: crubit :: test :: ContainerValue
        pub struct ContainerValue {
            pub value: ::ffi_11::c_int,
        }
        impl !Send for ContainerValue {}
        impl !Sync for ContainerValue {}
        unsafe impl ::cxx::ExternType for ContainerValue {
            type Id = ::cxx::type_id!(":: crubit :: test :: ContainerValue");
            type Kind = ::cxx::kind::Trivial;
        }

        /// Generated from: rs_bindings_from_cc/test/operators/operators_index.h;l=77
        impl Default for ContainerValue {
            #[inline(always)]
            fn default() -> Self {
                let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
                unsafe {
                    crate::detail::__rust_thunk___ZN6crubit4test14ContainerValueC1Ev(
                        &raw mut tmp as *mut _,
                    );
                    tmp.assume_init()
                }
            }
        }

        // Generated from: rs_bindings_from_cc/test/operators/operators_index.h;l=80
        // error: function `crubit::test::ContainerValue::operator[]` could not be bound
        //   operator[] should return a reference, found int

        /// R-value qualified overloads are not supported.
        ///
        /// Generated from: rs_bindings_from_cc/test/operators/operators_index.h;l=84
        #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
        #[repr(C)]
        ///CRUBIT_ANNOTATE: cpp_type=:: crubit :: test :: ContainerRvalue
        pub struct ContainerRvalue {
            pub value: ::ffi_11::c_int,
        }
        impl !Send for ContainerRvalue {}
        impl !Sync for ContainerRvalue {}
        unsafe impl ::cxx::ExternType for ContainerRvalue {
            type Id = ::cxx::type_id!(":: crubit :: test :: ContainerRvalue");
            type Kind = ::cxx::kind::Trivial;
        }

        /// Generated from: rs_bindings_from_cc/test/operators/operators_index.h;l=84
        impl Default for ContainerRvalue {
            #[inline(always)]
            fn default() -> Self {
                let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
                unsafe {
                    crate::detail::__rust_thunk___ZN6crubit4test15ContainerRvalueC1Ev(
                        &raw mut tmp as *mut _,
                    );
                    tmp.assume_init()
                }
            }
        }

        // Generated from: rs_bindings_from_cc/test/operators/operators_index.h;l=86
        // error: function `crubit::test::ContainerRvalue::operator[]` could not be bound
        //   R-value qualified operator[] is not supported

        /// The following two cases where we have
        /// - non-const references returned from const indexing, or
        /// - const references returned from non-const indexing
        /// are invalid overload signatures and should not generate bindings.
        ///
        /// Generated from: rs_bindings_from_cc/test/operators/operators_index.h;l=93
        #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
        #[repr(C)]
        ///CRUBIT_ANNOTATE: cpp_type=:: crubit :: test :: ContainerMutRefFromConst
        pub struct ContainerMutRefFromConst {
            pub value: ::ffi_11::c_int,
        }
        impl !Send for ContainerMutRefFromConst {}
        impl !Sync for ContainerMutRefFromConst {}
        unsafe impl ::cxx::ExternType for ContainerMutRefFromConst {
            type Id = ::cxx::type_id!(":: crubit :: test :: ContainerMutRefFromConst");
            type Kind = ::cxx::kind::Trivial;
        }

        /// Generated from: rs_bindings_from_cc/test/operators/operators_index.h;l=93
        impl Default for ContainerMutRefFromConst {
            #[inline(always)]
            fn default() -> Self {
                let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
                unsafe {
                    crate::detail::__rust_thunk___ZN6crubit4test24ContainerMutRefFromConstC1Ev(
                        &raw mut tmp as *mut _,
                    );
                    tmp.assume_init()
                }
            }
        }

        // Generated from: rs_bindings_from_cc/test/operators/operators_index.h;l=95
        // error: function `crubit::test::ContainerMutRefFromConst::operator[]` could not be bound
        //   operator[] must either:
        //   (a) be a const method that returns a const reference, or,
        //   (b) be a non-const method that returns a non-const reference.
        //   Instead found a method: (which is const?)=true, and (whose return value is const?)=false

        /// Generated from: rs_bindings_from_cc/test/operators/operators_index.h;l=98
        #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
        #[repr(C)]
        ///CRUBIT_ANNOTATE: cpp_type=:: crubit :: test :: ContainerConstRefFromMut
        pub struct ContainerConstRefFromMut {
            pub value: ::ffi_11::c_int,
        }
        impl !Send for ContainerConstRefFromMut {}
        impl !Sync for ContainerConstRefFromMut {}
        unsafe impl ::cxx::ExternType for ContainerConstRefFromMut {
            type Id = ::cxx::type_id!(":: crubit :: test :: ContainerConstRefFromMut");
            type Kind = ::cxx::kind::Trivial;
        }

        /// Generated from: rs_bindings_from_cc/test/operators/operators_index.h;l=98
        impl Default for ContainerConstRefFromMut {
            #[inline(always)]
            fn default() -> Self {
                let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
                unsafe {
                    crate::detail::__rust_thunk___ZN6crubit4test24ContainerConstRefFromMutC1Ev(
                        &raw mut tmp as *mut _,
                    );
                    tmp.assume_init()
                }
            }
        }

        // Generated from: rs_bindings_from_cc/test/operators/operators_index.h;l=100
        // error: function `crubit::test::ContainerConstRefFromMut::operator[]` could not be bound
        //   operator[] must either:
        //   (a) be a const method that returns a const reference, or,
        //   (b) be a non-const method that returns a non-const reference.
        //   Instead found a method: (which is const?)=false, and (whose return value is const?)=true
    }
}

// namespace crubit::test

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test9ItemUnpinC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test12ItemNonUnpinC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test12ItemNonUnpinC1ERKS1_<'__unelided>(
            __this: *mut ::core::ffi::c_void,
            __param_0: &'__unelided crate::crubit::test::ItemNonUnpin,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test12ItemNonUnpinaSERKS1_<
            '__return_lifetime,
        >(
            __this: ::core::pin::Pin<&mut crate::crubit::test::ItemNonUnpin>,
            __param_0: &crate::crubit::test::ItemNonUnpin,
        ) -> ::core::pin::Pin<&'__return_lifetime mut crate::crubit::test::ItemNonUnpin>;
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test12ItemNonUnpinD1Ev<'a>(
            __this: ::core::pin::Pin<&'a mut crate::crubit::test::ItemNonUnpin>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test23ContainerUnpinItemUnpinC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZNK6crubit4test23ContainerUnpinItemUnpinixEj<
            '__return_lifetime,
        >(
            __this: &crate::crubit::test::ContainerUnpinItemUnpin,
            index: ::ffi_11::c_uint,
        ) -> &'__return_lifetime crate::crubit::test::ItemUnpin;
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test23ContainerUnpinItemUnpinixEj<
            '__return_lifetime,
        >(
            __this: &mut crate::crubit::test::ContainerUnpinItemUnpin,
            index: ::ffi_11::c_uint,
        ) -> &'__return_lifetime mut crate::crubit::test::ItemUnpin;
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test26ContainerUnpinItemNonUnpinC1ERKS1_<
            '__unelided,
        >(
            __this: *mut ::core::ffi::c_void,
            __param_0: &'__unelided crate::crubit::test::ContainerUnpinItemNonUnpin,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test26ContainerUnpinItemNonUnpinC1EOS1_<
            '__unelided,
        >(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<
                '__unelided,
                crate::crubit::test::ContainerUnpinItemNonUnpin,
            >,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test26ContainerUnpinItemNonUnpinD1Ev<'a>(
            __this: ::core::pin::Pin<&'a mut crate::crubit::test::ContainerUnpinItemNonUnpin>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test26ContainerUnpinItemNonUnpinaSERKS1_<
            '__return_lifetime,
        >(
            __this: ::core::pin::Pin<&mut crate::crubit::test::ContainerUnpinItemNonUnpin>,
            __param_0: &crate::crubit::test::ContainerUnpinItemNonUnpin,
        ) -> ::core::pin::Pin<&'__return_lifetime mut crate::crubit::test::ContainerUnpinItemNonUnpin>;
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test26ContainerUnpinItemNonUnpinaSEOS1_<
            '__return_lifetime,
        >(
            __this: ::core::pin::Pin<&mut crate::crubit::test::ContainerUnpinItemNonUnpin>,
            __param_0: ::ctor::RvalueReference<'_, crate::crubit::test::ContainerUnpinItemNonUnpin>,
        ) -> ::core::pin::Pin<&'__return_lifetime mut crate::crubit::test::ContainerUnpinItemNonUnpin>;
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test26ContainerUnpinItemNonUnpinC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test26ContainerUnpinItemNonUnpinC1EPNS0_12ItemNonUnpinE(
            __this: *mut ::core::ffi::c_void,
            items: *mut crate::crubit::test::ItemNonUnpin,
        );
        pub(crate) unsafe fn __rust_thunk___ZNK6crubit4test26ContainerUnpinItemNonUnpinixEj<
            '__return_lifetime,
        >(
            __this: &crate::crubit::test::ContainerUnpinItemNonUnpin,
            index: ::ffi_11::c_uint,
        ) -> &'__return_lifetime crate::crubit::test::ItemNonUnpin;
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test26ContainerUnpinItemNonUnpinixEj<
            '__return_lifetime,
        >(
            __this: ::core::pin::Pin<&mut crate::crubit::test::ContainerUnpinItemNonUnpin>,
            index: ::ffi_11::c_uint,
        ) -> ::core::pin::Pin<&'__return_lifetime mut crate::crubit::test::ItemNonUnpin>;
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test26ContainerNonUnpinItemUnpinC1ERKS1_<
            '__unelided,
        >(
            __this: *mut ::core::ffi::c_void,
            __param_0: &'__unelided crate::crubit::test::ContainerNonUnpinItemUnpin,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test26ContainerNonUnpinItemUnpinaSERKS1_<
            '__return_lifetime,
        >(
            __this: ::core::pin::Pin<&mut crate::crubit::test::ContainerNonUnpinItemUnpin>,
            __param_0: &crate::crubit::test::ContainerNonUnpinItemUnpin,
        ) -> ::core::pin::Pin<&'__return_lifetime mut crate::crubit::test::ContainerNonUnpinItemUnpin>;
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test26ContainerNonUnpinItemUnpinC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test26ContainerNonUnpinItemUnpinD1Ev<'a>(
            __this: ::core::pin::Pin<&'a mut crate::crubit::test::ContainerNonUnpinItemUnpin>,
        );
        pub(crate) unsafe fn __rust_thunk___ZNK6crubit4test26ContainerNonUnpinItemUnpinixEj<
            '__return_lifetime,
        >(
            __this: &crate::crubit::test::ContainerNonUnpinItemUnpin,
            index: ::ffi_11::c_uint,
        ) -> &'__return_lifetime crate::crubit::test::ItemUnpin;
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test26ContainerNonUnpinItemUnpinixEj<
            '__return_lifetime,
        >(
            __this: ::core::pin::Pin<&mut crate::crubit::test::ContainerNonUnpinItemUnpin>,
            index: ::ffi_11::c_uint,
        ) -> &'__return_lifetime mut crate::crubit::test::ItemUnpin;
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test29ContainerNonUnpinItemNonUnpinC1ERKS1_<
            '__unelided,
        >(
            __this: *mut ::core::ffi::c_void,
            __param_0: &'__unelided crate::crubit::test::ContainerNonUnpinItemNonUnpin,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test29ContainerNonUnpinItemNonUnpinaSERKS1_<
            '__return_lifetime,
        >(
            __this: ::core::pin::Pin<&mut crate::crubit::test::ContainerNonUnpinItemNonUnpin>,
            __param_0: &crate::crubit::test::ContainerNonUnpinItemNonUnpin,
        ) -> ::core::pin::Pin<
            &'__return_lifetime mut crate::crubit::test::ContainerNonUnpinItemNonUnpin,
        >;
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test29ContainerNonUnpinItemNonUnpinC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test29ContainerNonUnpinItemNonUnpinD1Ev<'a>(
            __this: ::core::pin::Pin<&'a mut crate::crubit::test::ContainerNonUnpinItemNonUnpin>,
        );
        pub(crate) unsafe fn __rust_thunk___ZNK6crubit4test29ContainerNonUnpinItemNonUnpinixEj<
            '__return_lifetime,
        >(
            __this: &crate::crubit::test::ContainerNonUnpinItemNonUnpin,
            index: ::ffi_11::c_uint,
        ) -> &'__return_lifetime crate::crubit::test::ItemNonUnpin;
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test29ContainerNonUnpinItemNonUnpinixEj<
            '__return_lifetime,
        >(
            __this: ::core::pin::Pin<&mut crate::crubit::test::ContainerNonUnpinItemNonUnpin>,
            index: ::ffi_11::c_uint,
        ) -> ::core::pin::Pin<&'__return_lifetime mut crate::crubit::test::ItemNonUnpin>;
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test14ContainerValueC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test15ContainerRvalueC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test24ContainerMutRefFromConstC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test24ContainerConstRefFromMutC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::crubit::test::ItemUnpin>() == 4);
    assert!(::core::mem::align_of::<crate::crubit::test::ItemUnpin>() == 4);
    static_assertions::assert_impl_all!(crate::crubit::test::ItemUnpin: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::crubit::test::ItemUnpin: Drop);
    assert!(::core::mem::offset_of!(crate::crubit::test::ItemUnpin, value) == 0);
    assert!(::core::mem::size_of::<crate::crubit::test::ItemNonUnpin>() == 4);
    assert!(::core::mem::align_of::<crate::crubit::test::ItemNonUnpin>() == 4);
    static_assertions::assert_impl_all!(crate::crubit::test::ItemNonUnpin: Drop);
    static_assertions::assert_not_impl_any!(crate::crubit::test::ItemNonUnpin: Copy);
    assert!(::core::mem::offset_of!(crate::crubit::test::ItemNonUnpin, value) == 0);
    static_assertions::assert_impl_all!(::ffi_11::c_int: Copy);
    assert!(::core::mem::size_of::<crate::crubit::test::ContainerUnpinItemUnpin>() == 40);
    assert!(::core::mem::align_of::<crate::crubit::test::ContainerUnpinItemUnpin>() == 4);
    static_assertions::assert_impl_all!(crate::crubit::test::ContainerUnpinItemUnpin: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::crubit::test::ContainerUnpinItemUnpin: Drop);
    assert!(::core::mem::offset_of!(crate::crubit::test::ContainerUnpinItemUnpin, items_) == 0);
    assert!(::core::mem::size_of::<crate::crubit::test::ContainerUnpinItemNonUnpin>() == 48);
    assert!(::core::mem::align_of::<crate::crubit::test::ContainerUnpinItemNonUnpin>() == 8);
    static_assertions::assert_impl_all!(crate::crubit::test::ContainerUnpinItemNonUnpin: Drop);
    static_assertions::assert_not_impl_any!(crate::crubit::test::ContainerUnpinItemNonUnpin: Copy);
    assert!(
        ::core::mem::offset_of!(crate::crubit::test::ContainerUnpinItemNonUnpin, items_storage_)
            == 0
    );
    assert!(::core::mem::offset_of!(crate::crubit::test::ContainerUnpinItemNonUnpin, items_) == 40);
    static_assertions::assert_impl_all!(*mut crate::crubit::test::ItemNonUnpin: Copy);
    assert!(::core::mem::size_of::<crate::crubit::test::ContainerNonUnpinItemUnpin>() == 40);
    assert!(::core::mem::align_of::<crate::crubit::test::ContainerNonUnpinItemUnpin>() == 4);
    static_assertions::assert_impl_all!(crate::crubit::test::ContainerNonUnpinItemUnpin: Drop);
    static_assertions::assert_not_impl_any!(crate::crubit::test::ContainerNonUnpinItemUnpin: Copy);
    assert!(::core::mem::offset_of!(crate::crubit::test::ContainerNonUnpinItemUnpin, items_) == 0);
    assert!(::core::mem::size_of::<crate::crubit::test::ContainerNonUnpinItemNonUnpin>() == 40);
    assert!(::core::mem::align_of::<crate::crubit::test::ContainerNonUnpinItemNonUnpin>() == 4);
    static_assertions::assert_impl_all!(crate::crubit::test::ContainerNonUnpinItemNonUnpin: Drop);
    static_assertions::assert_not_impl_any!(crate::crubit::test::ContainerNonUnpinItemNonUnpin: Copy);
    assert!(
        ::core::mem::offset_of!(crate::crubit::test::ContainerNonUnpinItemNonUnpin, items_) == 0
    );
    assert!(::core::mem::size_of::<crate::crubit::test::ContainerValue>() == 4);
    assert!(::core::mem::align_of::<crate::crubit::test::ContainerValue>() == 4);
    static_assertions::assert_impl_all!(crate::crubit::test::ContainerValue: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::crubit::test::ContainerValue: Drop);
    assert!(::core::mem::offset_of!(crate::crubit::test::ContainerValue, value) == 0);
    assert!(::core::mem::size_of::<crate::crubit::test::ContainerRvalue>() == 4);
    assert!(::core::mem::align_of::<crate::crubit::test::ContainerRvalue>() == 4);
    static_assertions::assert_impl_all!(crate::crubit::test::ContainerRvalue: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::crubit::test::ContainerRvalue: Drop);
    assert!(::core::mem::offset_of!(crate::crubit::test::ContainerRvalue, value) == 0);
    assert!(::core::mem::size_of::<crate::crubit::test::ContainerMutRefFromConst>() == 4);
    assert!(::core::mem::align_of::<crate::crubit::test::ContainerMutRefFromConst>() == 4);
    static_assertions::assert_impl_all!(crate::crubit::test::ContainerMutRefFromConst: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::crubit::test::ContainerMutRefFromConst: Drop);
    assert!(::core::mem::offset_of!(crate::crubit::test::ContainerMutRefFromConst, value) == 0);
    assert!(::core::mem::size_of::<crate::crubit::test::ContainerConstRefFromMut>() == 4);
    assert!(::core::mem::align_of::<crate::crubit::test::ContainerConstRefFromMut>() == 4);
    static_assertions::assert_impl_all!(crate::crubit::test::ContainerConstRefFromMut: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::crubit::test::ContainerConstRefFromMut: Drop);
    assert!(::core::mem::offset_of!(crate::crubit::test::ContainerConstRefFromMut, value) == 0);
};
