// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:nested_types_cc

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![no_std]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

#[derive(Clone, Copy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=Foo
pub struct Foo {
    pub foo: ::core::ffi::c_int,
}
impl !Send for Foo {}
impl !Sync for Foo {}
unsafe impl ::cxx::ExternType for Foo {
    type Id = ::cxx::type_id!("Foo");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(forward_declare::symbol!("Foo"), crate::Foo);

impl Default for Foo {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN3FooC1Ev(&raw mut tmp as *mut ::core::ffi::c_void);
            tmp.assume_init()
        }
    }
}

impl From<::ctor::RvalueReference<'_, Self>> for Foo {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'_, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN3FooC1EOS_(
                &raw mut tmp as *mut ::core::ffi::c_void,
                __param_0,
            );
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<::ctor::RvalueReference<'_, Self>> for Foo {
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'_, Self>) -> Self::CtorType {
        <Self as From<::ctor::RvalueReference<'_, Self>>>::from(args)
    }
}

impl ::ctor::UnpinAssign<&Self> for Foo {
    #[inline(always)]
    fn unpin_assign(&mut self, __param_0: &Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN3FooaSERKS_(self, __param_0);
        }
    }
}

impl ::ctor::UnpinAssign<::ctor::RvalueReference<'_, Self>> for Foo {
    #[inline(always)]
    fn unpin_assign(&mut self, __param_0: ::ctor::RvalueReference<'_, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN3FooaSEOS_(self, __param_0);
        }
    }
}

pub mod foo {
    #[allow(unused_imports)]
    use super::*;

    #[derive(Clone, Copy)]
    #[repr(C)]
    ///CRUBIT_ANNOTATE: cpp_type=Foo :: Bar
    pub struct Bar {
        pub bar: ::core::ffi::c_int,
    }
    impl !Send for Bar {}
    impl !Sync for Bar {}
    unsafe impl ::cxx::ExternType for Bar {
        type Id = ::cxx::type_id!("Foo :: Bar");
        type Kind = ::cxx::kind::Trivial;
    }
    forward_declare::unsafe_define!(forward_declare::symbol!("Foo :: Bar"), crate::foo::Bar);

    impl Default for Bar {
        #[inline(always)]
        fn default() -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk___ZN3Foo3BarC1Ev(
                    &raw mut tmp as *mut ::core::ffi::c_void,
                );
                tmp.assume_init()
            }
        }
    }

    impl From<::ctor::RvalueReference<'_, Self>> for Bar {
        #[inline(always)]
        fn from(__param_0: ::ctor::RvalueReference<'_, Self>) -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk___ZN3Foo3BarC1EOS0_(
                    &raw mut tmp as *mut ::core::ffi::c_void,
                    __param_0,
                );
                tmp.assume_init()
            }
        }
    }
    impl ::ctor::CtorNew<::ctor::RvalueReference<'_, Self>> for Bar {
        type CtorType = Self;
        type Error = ::ctor::Infallible;
        #[inline(always)]
        fn ctor_new(args: ::ctor::RvalueReference<'_, Self>) -> Self::CtorType {
            <Self as From<::ctor::RvalueReference<'_, Self>>>::from(args)
        }
    }

    impl ::ctor::UnpinAssign<&Self> for Bar {
        #[inline(always)]
        fn unpin_assign(&mut self, __param_0: &Self) {
            unsafe {
                crate::detail::__rust_thunk___ZN3Foo3BaraSERKS0_(self, __param_0);
            }
        }
    }

    impl ::ctor::UnpinAssign<::ctor::RvalueReference<'_, Self>> for Bar {
        #[inline(always)]
        fn unpin_assign(&mut self, __param_0: ::ctor::RvalueReference<'_, Self>) {
            unsafe {
                crate::detail::__rust_thunk___ZN3Foo3BaraSEOS0_(self, __param_0);
            }
        }
    }

    pub mod bar {
        #[allow(unused_imports)]
        use super::*;

        #[derive(Clone, Copy)]
        #[repr(C)]
        ///CRUBIT_ANNOTATE: cpp_type=Foo :: Bar :: Baz
        pub struct Baz {
            pub baz: ::core::ffi::c_int,
        }
        impl !Send for Baz {}
        impl !Sync for Baz {}
        unsafe impl ::cxx::ExternType for Baz {
            type Id = ::cxx::type_id!("Foo :: Bar :: Baz");
            type Kind = ::cxx::kind::Trivial;
        }
        forward_declare::unsafe_define!(
            forward_declare::symbol!("Foo :: Bar :: Baz"),
            crate::foo::bar::Baz
        );

        impl Default for Baz {
            #[inline(always)]
            fn default() -> Self {
                let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
                unsafe {
                    crate::detail::__rust_thunk___ZN3Foo3Bar3BazC1Ev(
                        &raw mut tmp as *mut ::core::ffi::c_void,
                    );
                    tmp.assume_init()
                }
            }
        }

        impl From<::ctor::RvalueReference<'_, Self>> for Baz {
            #[inline(always)]
            fn from(__param_0: ::ctor::RvalueReference<'_, Self>) -> Self {
                let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
                unsafe {
                    crate::detail::__rust_thunk___ZN3Foo3Bar3BazC1EOS1_(
                        &raw mut tmp as *mut ::core::ffi::c_void,
                        __param_0,
                    );
                    tmp.assume_init()
                }
            }
        }
        impl ::ctor::CtorNew<::ctor::RvalueReference<'_, Self>> for Baz {
            type CtorType = Self;
            type Error = ::ctor::Infallible;
            #[inline(always)]
            fn ctor_new(args: ::ctor::RvalueReference<'_, Self>) -> Self::CtorType {
                <Self as From<::ctor::RvalueReference<'_, Self>>>::from(args)
            }
        }

        impl ::ctor::UnpinAssign<&Self> for Baz {
            #[inline(always)]
            fn unpin_assign(&mut self, __param_0: &Self) {
                unsafe {
                    crate::detail::__rust_thunk___ZN3Foo3Bar3BazaSERKS1_(self, __param_0);
                }
            }
        }

        impl ::ctor::UnpinAssign<::ctor::RvalueReference<'_, Self>> for Baz {
            #[inline(always)]
            fn unpin_assign(&mut self, __param_0: ::ctor::RvalueReference<'_, Self>) {
                unsafe {
                    crate::detail::__rust_thunk___ZN3Foo3Bar3BazaSEOS1_(self, __param_0);
                }
            }
        }
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=already_snake_case
pub struct already_snake_case {
    pub i: ::core::ffi::c_int,
}
impl !Send for already_snake_case {}
impl !Sync for already_snake_case {}
unsafe impl ::cxx::ExternType for already_snake_case {
    type Id = ::cxx::type_id!("already_snake_case");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("already_snake_case"),
    crate::already_snake_case
);

impl Default for already_snake_case {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN18already_snake_caseC1Ev(
                &raw mut tmp as *mut ::core::ffi::c_void,
            );
            tmp.assume_init()
        }
    }
}

impl From<::ctor::RvalueReference<'_, Self>> for already_snake_case {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'_, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN18already_snake_caseC1EOS_(
                &raw mut tmp as *mut ::core::ffi::c_void,
                __param_0,
            );
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<::ctor::RvalueReference<'_, Self>> for already_snake_case {
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'_, Self>) -> Self::CtorType {
        <Self as From<::ctor::RvalueReference<'_, Self>>>::from(args)
    }
}

impl ::ctor::UnpinAssign<&Self> for already_snake_case {
    #[inline(always)]
    fn unpin_assign(&mut self, __param_0: &Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN18already_snake_caseaSERKS_(self, __param_0);
        }
    }
}

impl ::ctor::UnpinAssign<::ctor::RvalueReference<'_, Self>> for already_snake_case {
    #[inline(always)]
    fn unpin_assign(&mut self, __param_0: ::ctor::RvalueReference<'_, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN18already_snake_caseaSEOS_(self, __param_0);
        }
    }
}

// Error while generating bindings for type alias 'Inner':
// parent record has nested items, but the module to contain them could not be generated because another item named `already_snake_case` already exists

#[derive(Clone, Copy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=ConflictingSnakeCaseNames
pub struct ConflictingSnakeCaseNames {
    pub i: ::core::ffi::c_int,
}
impl !Send for ConflictingSnakeCaseNames {}
impl !Sync for ConflictingSnakeCaseNames {}
unsafe impl ::cxx::ExternType for ConflictingSnakeCaseNames {
    type Id = ::cxx::type_id!("ConflictingSnakeCaseNames");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("ConflictingSnakeCaseNames"),
    crate::ConflictingSnakeCaseNames
);

impl Default for ConflictingSnakeCaseNames {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN25ConflictingSnakeCaseNamesC1Ev(
                &raw mut tmp as *mut ::core::ffi::c_void,
            );
            tmp.assume_init()
        }
    }
}

impl From<::ctor::RvalueReference<'_, Self>> for ConflictingSnakeCaseNames {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'_, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN25ConflictingSnakeCaseNamesC1EOS_(
                &raw mut tmp as *mut ::core::ffi::c_void,
                __param_0,
            );
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<::ctor::RvalueReference<'_, Self>> for ConflictingSnakeCaseNames {
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'_, Self>) -> Self::CtorType {
        <Self as From<::ctor::RvalueReference<'_, Self>>>::from(args)
    }
}

impl ::ctor::UnpinAssign<&Self> for ConflictingSnakeCaseNames {
    #[inline(always)]
    fn unpin_assign(&mut self, __param_0: &Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN25ConflictingSnakeCaseNamesaSERKS_(self, __param_0);
        }
    }
}

impl ::ctor::UnpinAssign<::ctor::RvalueReference<'_, Self>> for ConflictingSnakeCaseNames {
    #[inline(always)]
    fn unpin_assign(&mut self, __param_0: ::ctor::RvalueReference<'_, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN25ConflictingSnakeCaseNamesaSEOS_(self, __param_0);
        }
    }
}

// Error while generating bindings for type alias 'Inner':
// records ["ConflictingSnakeCaseNames", "ConflictingSnakeCaseNames_"] all have nested items, but all map to the same nested module name: `conflicting_snake_case_names`

#[derive(Clone, Copy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=ConflictingSnakeCaseNames_
pub struct ConflictingSnakeCaseNames_ {
    pub i: ::core::ffi::c_int,
}
impl !Send for ConflictingSnakeCaseNames_ {}
impl !Sync for ConflictingSnakeCaseNames_ {}
unsafe impl ::cxx::ExternType for ConflictingSnakeCaseNames_ {
    type Id = ::cxx::type_id!("ConflictingSnakeCaseNames_");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("ConflictingSnakeCaseNames_"),
    crate::ConflictingSnakeCaseNames_
);

impl Default for ConflictingSnakeCaseNames_ {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN26ConflictingSnakeCaseNames_C1Ev(
                &raw mut tmp as *mut ::core::ffi::c_void,
            );
            tmp.assume_init()
        }
    }
}

impl From<::ctor::RvalueReference<'_, Self>> for ConflictingSnakeCaseNames_ {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'_, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN26ConflictingSnakeCaseNames_C1EOS_(
                &raw mut tmp as *mut ::core::ffi::c_void,
                __param_0,
            );
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<::ctor::RvalueReference<'_, Self>> for ConflictingSnakeCaseNames_ {
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'_, Self>) -> Self::CtorType {
        <Self as From<::ctor::RvalueReference<'_, Self>>>::from(args)
    }
}

impl ::ctor::UnpinAssign<&Self> for ConflictingSnakeCaseNames_ {
    #[inline(always)]
    fn unpin_assign(&mut self, __param_0: &Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN26ConflictingSnakeCaseNames_aSERKS_(self, __param_0);
        }
    }
}

impl ::ctor::UnpinAssign<::ctor::RvalueReference<'_, Self>> for ConflictingSnakeCaseNames_ {
    #[inline(always)]
    fn unpin_assign(&mut self, __param_0: ::ctor::RvalueReference<'_, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN26ConflictingSnakeCaseNames_aSEOS_(self, __param_0);
        }
    }
}

// Error while generating bindings for type alias 'Inner':
// records ["ConflictingSnakeCaseNames", "ConflictingSnakeCaseNames_"] all have nested items, but all map to the same nested module name: `conflicting_snake_case_names`

#[derive(Clone, Copy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=OnlyOneHasNestedItems
pub struct OnlyOneHasNestedItems {
    pub i: crate::only_one_has_nested_items::Inner,
}
impl !Send for OnlyOneHasNestedItems {}
impl !Sync for OnlyOneHasNestedItems {}
unsafe impl ::cxx::ExternType for OnlyOneHasNestedItems {
    type Id = ::cxx::type_id!("OnlyOneHasNestedItems");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("OnlyOneHasNestedItems"),
    crate::OnlyOneHasNestedItems
);

impl Default for OnlyOneHasNestedItems {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN21OnlyOneHasNestedItemsC1Ev(
                &raw mut tmp as *mut ::core::ffi::c_void,
            );
            tmp.assume_init()
        }
    }
}

impl From<::ctor::RvalueReference<'_, Self>> for OnlyOneHasNestedItems {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'_, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN21OnlyOneHasNestedItemsC1EOS_(
                &raw mut tmp as *mut ::core::ffi::c_void,
                __param_0,
            );
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<::ctor::RvalueReference<'_, Self>> for OnlyOneHasNestedItems {
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'_, Self>) -> Self::CtorType {
        <Self as From<::ctor::RvalueReference<'_, Self>>>::from(args)
    }
}

impl ::ctor::UnpinAssign<&Self> for OnlyOneHasNestedItems {
    #[inline(always)]
    fn unpin_assign(&mut self, __param_0: &Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN21OnlyOneHasNestedItemsaSERKS_(self, __param_0);
        }
    }
}

impl ::ctor::UnpinAssign<::ctor::RvalueReference<'_, Self>> for OnlyOneHasNestedItems {
    #[inline(always)]
    fn unpin_assign(&mut self, __param_0: ::ctor::RvalueReference<'_, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN21OnlyOneHasNestedItemsaSEOS_(self, __param_0);
        }
    }
}

pub mod only_one_has_nested_items {
    #[allow(unused_imports)]
    use super::*;

    pub type Inner = ::core::ffi::c_int;
}

#[derive(Clone, Copy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=OnlyOneHasNestedItems_
pub struct OnlyOneHasNestedItems_ {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for OnlyOneHasNestedItems_ {}
impl !Sync for OnlyOneHasNestedItems_ {}
unsafe impl ::cxx::ExternType for OnlyOneHasNestedItems_ {
    type Id = ::cxx::type_id!("OnlyOneHasNestedItems_");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("OnlyOneHasNestedItems_"),
    crate::OnlyOneHasNestedItems_
);

impl Default for OnlyOneHasNestedItems_ {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN22OnlyOneHasNestedItems_C1Ev(
                &raw mut tmp as *mut ::core::ffi::c_void,
            );
            tmp.assume_init()
        }
    }
}

impl From<::ctor::RvalueReference<'_, Self>> for OnlyOneHasNestedItems_ {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'_, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN22OnlyOneHasNestedItems_C1EOS_(
                &raw mut tmp as *mut ::core::ffi::c_void,
                __param_0,
            );
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<::ctor::RvalueReference<'_, Self>> for OnlyOneHasNestedItems_ {
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'_, Self>) -> Self::CtorType {
        <Self as From<::ctor::RvalueReference<'_, Self>>>::from(args)
    }
}

impl ::ctor::UnpinAssign<&Self> for OnlyOneHasNestedItems_ {
    #[inline(always)]
    fn unpin_assign(&mut self, __param_0: &Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN22OnlyOneHasNestedItems_aSERKS_(self, __param_0);
        }
    }
}

impl ::ctor::UnpinAssign<::ctor::RvalueReference<'_, Self>> for OnlyOneHasNestedItems_ {
    #[inline(always)]
    fn unpin_assign(&mut self, __param_0: ::ctor::RvalueReference<'_, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN22OnlyOneHasNestedItems_aSEOS_(self, __param_0);
        }
    }
}

// no nested items

#[derive(Clone, Copy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=SameNameAsNamespace
pub struct SameNameAsNamespace {
    pub i: ::core::ffi::c_int,
}
impl !Send for SameNameAsNamespace {}
impl !Sync for SameNameAsNamespace {}
unsafe impl ::cxx::ExternType for SameNameAsNamespace {
    type Id = ::cxx::type_id!("SameNameAsNamespace");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("SameNameAsNamespace"),
    crate::SameNameAsNamespace
);

impl Default for SameNameAsNamespace {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN19SameNameAsNamespaceC1Ev(
                &raw mut tmp as *mut ::core::ffi::c_void,
            );
            tmp.assume_init()
        }
    }
}

impl From<::ctor::RvalueReference<'_, Self>> for SameNameAsNamespace {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'_, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN19SameNameAsNamespaceC1EOS_(
                &raw mut tmp as *mut ::core::ffi::c_void,
                __param_0,
            );
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<::ctor::RvalueReference<'_, Self>> for SameNameAsNamespace {
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'_, Self>) -> Self::CtorType {
        <Self as From<::ctor::RvalueReference<'_, Self>>>::from(args)
    }
}

impl ::ctor::UnpinAssign<&Self> for SameNameAsNamespace {
    #[inline(always)]
    fn unpin_assign(&mut self, __param_0: &Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN19SameNameAsNamespaceaSERKS_(self, __param_0);
        }
    }
}

impl ::ctor::UnpinAssign<::ctor::RvalueReference<'_, Self>> for SameNameAsNamespace {
    #[inline(always)]
    fn unpin_assign(&mut self, __param_0: ::ctor::RvalueReference<'_, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN19SameNameAsNamespaceaSEOS_(self, __param_0);
        }
    }
}

// Error while generating bindings for type alias 'Inner':
// parent record has nested items, but the module to contain them could not be generated because another item named `same_name_as_namespace` already exists

// namespace same_name_as_namespace

pub mod same_name_as_namespace {
    #[derive(Clone, Copy)]
    #[repr(C)]
    ///CRUBIT_ANNOTATE: cpp_type=same_name_as_namespace :: Foo
    pub struct Foo {
        __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
    }
    impl !Send for Foo {}
    impl !Sync for Foo {}
    unsafe impl ::cxx::ExternType for Foo {
        type Id = ::cxx::type_id!("same_name_as_namespace :: Foo");
        type Kind = ::cxx::kind::Trivial;
    }
    forward_declare::unsafe_define!(
        forward_declare::symbol!("same_name_as_namespace :: Foo"),
        crate::same_name_as_namespace::Foo
    );

    impl Default for Foo {
        #[inline(always)]
        fn default() -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk___ZN22same_name_as_namespace3FooC1Ev(
                    &raw mut tmp as *mut ::core::ffi::c_void,
                );
                tmp.assume_init()
            }
        }
    }

    impl From<::ctor::RvalueReference<'_, Self>> for Foo {
        #[inline(always)]
        fn from(__param_0: ::ctor::RvalueReference<'_, Self>) -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk___ZN22same_name_as_namespace3FooC1EOS0_(
                    &raw mut tmp as *mut ::core::ffi::c_void,
                    __param_0,
                );
                tmp.assume_init()
            }
        }
    }
    impl ::ctor::CtorNew<::ctor::RvalueReference<'_, Self>> for Foo {
        type CtorType = Self;
        type Error = ::ctor::Infallible;
        #[inline(always)]
        fn ctor_new(args: ::ctor::RvalueReference<'_, Self>) -> Self::CtorType {
            <Self as From<::ctor::RvalueReference<'_, Self>>>::from(args)
        }
    }

    impl ::ctor::UnpinAssign<&Self> for Foo {
        #[inline(always)]
        fn unpin_assign(&mut self, __param_0: &Self) {
            unsafe {
                crate::detail::__rust_thunk___ZN22same_name_as_namespace3FooaSERKS0_(
                    self, __param_0,
                );
            }
        }
    }

    impl ::ctor::UnpinAssign<::ctor::RvalueReference<'_, Self>> for Foo {
        #[inline(always)]
        fn unpin_assign(&mut self, __param_0: ::ctor::RvalueReference<'_, Self>) {
            unsafe {
                crate::detail::__rust_thunk___ZN22same_name_as_namespace3FooaSEOS0_(
                    self, __param_0,
                );
            }
        }
    }

    #[derive(Clone, Copy)]
    #[repr(C)]
    ///CRUBIT_ANNOTATE: cpp_type=same_name_as_namespace :: Bar
    pub struct Bar {
        __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
    }
    impl !Send for Bar {}
    impl !Sync for Bar {}
    unsafe impl ::cxx::ExternType for Bar {
        type Id = ::cxx::type_id!("same_name_as_namespace :: Bar");
        type Kind = ::cxx::kind::Trivial;
    }
    forward_declare::unsafe_define!(
        forward_declare::symbol!("same_name_as_namespace :: Bar"),
        crate::same_name_as_namespace::Bar
    );

    impl Default for Bar {
        #[inline(always)]
        fn default() -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk___ZN22same_name_as_namespace3BarC1Ev(
                    &raw mut tmp as *mut ::core::ffi::c_void,
                );
                tmp.assume_init()
            }
        }
    }

    impl From<::ctor::RvalueReference<'_, Self>> for Bar {
        #[inline(always)]
        fn from(__param_0: ::ctor::RvalueReference<'_, Self>) -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk___ZN22same_name_as_namespace3BarC1EOS0_(
                    &raw mut tmp as *mut ::core::ffi::c_void,
                    __param_0,
                );
                tmp.assume_init()
            }
        }
    }
    impl ::ctor::CtorNew<::ctor::RvalueReference<'_, Self>> for Bar {
        type CtorType = Self;
        type Error = ::ctor::Infallible;
        #[inline(always)]
        fn ctor_new(args: ::ctor::RvalueReference<'_, Self>) -> Self::CtorType {
            <Self as From<::ctor::RvalueReference<'_, Self>>>::from(args)
        }
    }

    impl ::ctor::UnpinAssign<&Self> for Bar {
        #[inline(always)]
        fn unpin_assign(&mut self, __param_0: &Self) {
            unsafe {
                crate::detail::__rust_thunk___ZN22same_name_as_namespace3BaraSERKS0_(
                    self, __param_0,
                );
            }
        }
    }

    impl ::ctor::UnpinAssign<::ctor::RvalueReference<'_, Self>> for Bar {
        #[inline(always)]
        fn unpin_assign(&mut self, __param_0: ::ctor::RvalueReference<'_, Self>) {
            unsafe {
                crate::detail::__rust_thunk___ZN22same_name_as_namespace3BaraSEOS0_(
                    self, __param_0,
                );
            }
        }
    }
}

// namespace same_name_as_namespace

pub mod no_longer_top_level {
    #[derive(Clone, Copy)]
    #[repr(C)]
    ///CRUBIT_ANNOTATE: cpp_type=no_longer_top_level :: already_snake_case
    pub struct already_snake_case {
        pub i: ::core::ffi::c_int,
    }
    impl !Send for already_snake_case {}
    impl !Sync for already_snake_case {}
    unsafe impl ::cxx::ExternType for already_snake_case {
        type Id = ::cxx::type_id!("no_longer_top_level :: already_snake_case");
        type Kind = ::cxx::kind::Trivial;
    }
    forward_declare::unsafe_define!(
        forward_declare::symbol!("no_longer_top_level :: already_snake_case"),
        crate::no_longer_top_level::already_snake_case
    );

    impl Default for already_snake_case {
        #[inline(always)]
        fn default() -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk___ZN19no_longer_top_level18already_snake_caseC1Ev(
                    &raw mut tmp as *mut ::core::ffi::c_void,
                );
                tmp.assume_init()
            }
        }
    }

    impl From<::ctor::RvalueReference<'_, Self>> for already_snake_case {
        #[inline(always)]
        fn from(__param_0: ::ctor::RvalueReference<'_, Self>) -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk___ZN19no_longer_top_level18already_snake_caseC1EOS0_(
                    &raw mut tmp as *mut ::core::ffi::c_void,
                    __param_0,
                );
                tmp.assume_init()
            }
        }
    }
    impl ::ctor::CtorNew<::ctor::RvalueReference<'_, Self>> for already_snake_case {
        type CtorType = Self;
        type Error = ::ctor::Infallible;
        #[inline(always)]
        fn ctor_new(args: ::ctor::RvalueReference<'_, Self>) -> Self::CtorType {
            <Self as From<::ctor::RvalueReference<'_, Self>>>::from(args)
        }
    }

    impl ::ctor::UnpinAssign<&Self> for already_snake_case {
        #[inline(always)]
        fn unpin_assign(&mut self, __param_0: &Self) {
            unsafe {
                crate::detail::__rust_thunk___ZN19no_longer_top_level18already_snake_caseaSERKS0_(
                    self, __param_0,
                );
            }
        }
    }

    impl ::ctor::UnpinAssign<::ctor::RvalueReference<'_, Self>> for already_snake_case {
        #[inline(always)]
        fn unpin_assign(&mut self, __param_0: ::ctor::RvalueReference<'_, Self>) {
            unsafe {
                crate::detail::__rust_thunk___ZN19no_longer_top_level18already_snake_caseaSEOS0_(
                    self, __param_0,
                );
            }
        }
    }

    // Error while generating bindings for type alias 'Inner':
    // parent record has nested items, but the module to contain them could not be generated because another item named `already_snake_case` already exists

    #[derive(Clone, Copy)]
    #[repr(C)]
    ///CRUBIT_ANNOTATE: cpp_type=no_longer_top_level :: ConflictingSnakeCaseNames
    pub struct ConflictingSnakeCaseNames {
        pub i: ::core::ffi::c_int,
    }
    impl !Send for ConflictingSnakeCaseNames {}
    impl !Sync for ConflictingSnakeCaseNames {}
    unsafe impl ::cxx::ExternType for ConflictingSnakeCaseNames {
        type Id = ::cxx::type_id!("no_longer_top_level :: ConflictingSnakeCaseNames");
        type Kind = ::cxx::kind::Trivial;
    }
    forward_declare::unsafe_define!(
        forward_declare::symbol!("no_longer_top_level :: ConflictingSnakeCaseNames"),
        crate::no_longer_top_level::ConflictingSnakeCaseNames
    );

    impl Default for ConflictingSnakeCaseNames {
        #[inline(always)]
        fn default() -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk___ZN19no_longer_top_level25ConflictingSnakeCaseNamesC1Ev(&raw mut tmp as*mut::core::ffi::c_void);
                tmp.assume_init()
            }
        }
    }

    impl From<::ctor::RvalueReference<'_, Self>> for ConflictingSnakeCaseNames {
        #[inline(always)]
        fn from(__param_0: ::ctor::RvalueReference<'_, Self>) -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk___ZN19no_longer_top_level25ConflictingSnakeCaseNamesC1EOS0_(&raw mut tmp as*mut::core::ffi::c_void,__param_0);
                tmp.assume_init()
            }
        }
    }
    impl ::ctor::CtorNew<::ctor::RvalueReference<'_, Self>> for ConflictingSnakeCaseNames {
        type CtorType = Self;
        type Error = ::ctor::Infallible;
        #[inline(always)]
        fn ctor_new(args: ::ctor::RvalueReference<'_, Self>) -> Self::CtorType {
            <Self as From<::ctor::RvalueReference<'_, Self>>>::from(args)
        }
    }

    impl ::ctor::UnpinAssign<&Self> for ConflictingSnakeCaseNames {
        #[inline(always)]
        fn unpin_assign(&mut self, __param_0: &Self) {
            unsafe {
                crate::detail::__rust_thunk___ZN19no_longer_top_level25ConflictingSnakeCaseNamesaSERKS0_(self,__param_0);
            }
        }
    }

    impl ::ctor::UnpinAssign<::ctor::RvalueReference<'_, Self>> for ConflictingSnakeCaseNames {
        #[inline(always)]
        fn unpin_assign(&mut self, __param_0: ::ctor::RvalueReference<'_, Self>) {
            unsafe {
                crate::detail::__rust_thunk___ZN19no_longer_top_level25ConflictingSnakeCaseNamesaSEOS0_(self,__param_0);
            }
        }
    }

    // Error while generating bindings for type alias 'Inner':
    // records ["ConflictingSnakeCaseNames", "ConflictingSnakeCaseNames_"] all have nested items, but all map to the same nested module name: `conflicting_snake_case_names`

    #[derive(Clone, Copy)]
    #[repr(C)]
    ///CRUBIT_ANNOTATE: cpp_type=no_longer_top_level :: ConflictingSnakeCaseNames_
    pub struct ConflictingSnakeCaseNames_ {
        pub i: ::core::ffi::c_int,
    }
    impl !Send for ConflictingSnakeCaseNames_ {}
    impl !Sync for ConflictingSnakeCaseNames_ {}
    unsafe impl ::cxx::ExternType for ConflictingSnakeCaseNames_ {
        type Id = ::cxx::type_id!("no_longer_top_level :: ConflictingSnakeCaseNames_");
        type Kind = ::cxx::kind::Trivial;
    }
    forward_declare::unsafe_define!(
        forward_declare::symbol!("no_longer_top_level :: ConflictingSnakeCaseNames_"),
        crate::no_longer_top_level::ConflictingSnakeCaseNames_
    );

    impl Default for ConflictingSnakeCaseNames_ {
        #[inline(always)]
        fn default() -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk___ZN19no_longer_top_level26ConflictingSnakeCaseNames_C1Ev(&raw mut tmp as*mut::core::ffi::c_void);
                tmp.assume_init()
            }
        }
    }

    impl From<::ctor::RvalueReference<'_, Self>> for ConflictingSnakeCaseNames_ {
        #[inline(always)]
        fn from(__param_0: ::ctor::RvalueReference<'_, Self>) -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk___ZN19no_longer_top_level26ConflictingSnakeCaseNames_C1EOS0_(&raw mut tmp as*mut::core::ffi::c_void,__param_0);
                tmp.assume_init()
            }
        }
    }
    impl ::ctor::CtorNew<::ctor::RvalueReference<'_, Self>> for ConflictingSnakeCaseNames_ {
        type CtorType = Self;
        type Error = ::ctor::Infallible;
        #[inline(always)]
        fn ctor_new(args: ::ctor::RvalueReference<'_, Self>) -> Self::CtorType {
            <Self as From<::ctor::RvalueReference<'_, Self>>>::from(args)
        }
    }

    impl ::ctor::UnpinAssign<&Self> for ConflictingSnakeCaseNames_ {
        #[inline(always)]
        fn unpin_assign(&mut self, __param_0: &Self) {
            unsafe {
                crate::detail::__rust_thunk___ZN19no_longer_top_level26ConflictingSnakeCaseNames_aSERKS0_(self,__param_0);
            }
        }
    }

    impl ::ctor::UnpinAssign<::ctor::RvalueReference<'_, Self>> for ConflictingSnakeCaseNames_ {
        #[inline(always)]
        fn unpin_assign(&mut self, __param_0: ::ctor::RvalueReference<'_, Self>) {
            unsafe {
                crate::detail::__rust_thunk___ZN19no_longer_top_level26ConflictingSnakeCaseNames_aSEOS0_(self,__param_0);
            }
        }
    }

    // Error while generating bindings for type alias 'Inner':
    // records ["ConflictingSnakeCaseNames", "ConflictingSnakeCaseNames_"] all have nested items, but all map to the same nested module name: `conflicting_snake_case_names`

    #[derive(Clone, Copy)]
    #[repr(C)]
    ///CRUBIT_ANNOTATE: cpp_type=no_longer_top_level :: OnlyOneHasNestedItems
    pub struct OnlyOneHasNestedItems {
        pub i: crate::no_longer_top_level::only_one_has_nested_items::Inner,
    }
    impl !Send for OnlyOneHasNestedItems {}
    impl !Sync for OnlyOneHasNestedItems {}
    unsafe impl ::cxx::ExternType for OnlyOneHasNestedItems {
        type Id = ::cxx::type_id!("no_longer_top_level :: OnlyOneHasNestedItems");
        type Kind = ::cxx::kind::Trivial;
    }
    forward_declare::unsafe_define!(
        forward_declare::symbol!("no_longer_top_level :: OnlyOneHasNestedItems"),
        crate::no_longer_top_level::OnlyOneHasNestedItems
    );

    impl Default for OnlyOneHasNestedItems {
        #[inline(always)]
        fn default() -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk___ZN19no_longer_top_level21OnlyOneHasNestedItemsC1Ev(
                    &raw mut tmp as *mut ::core::ffi::c_void,
                );
                tmp.assume_init()
            }
        }
    }

    impl From<::ctor::RvalueReference<'_, Self>> for OnlyOneHasNestedItems {
        #[inline(always)]
        fn from(__param_0: ::ctor::RvalueReference<'_, Self>) -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk___ZN19no_longer_top_level21OnlyOneHasNestedItemsC1EOS0_(
                    &raw mut tmp as *mut ::core::ffi::c_void,
                    __param_0,
                );
                tmp.assume_init()
            }
        }
    }
    impl ::ctor::CtorNew<::ctor::RvalueReference<'_, Self>> for OnlyOneHasNestedItems {
        type CtorType = Self;
        type Error = ::ctor::Infallible;
        #[inline(always)]
        fn ctor_new(args: ::ctor::RvalueReference<'_, Self>) -> Self::CtorType {
            <Self as From<::ctor::RvalueReference<'_, Self>>>::from(args)
        }
    }

    impl ::ctor::UnpinAssign<&Self> for OnlyOneHasNestedItems {
        #[inline(always)]
        fn unpin_assign(&mut self, __param_0: &Self) {
            unsafe {
                crate::detail::__rust_thunk___ZN19no_longer_top_level21OnlyOneHasNestedItemsaSERKS0_(self,__param_0);
            }
        }
    }

    impl ::ctor::UnpinAssign<::ctor::RvalueReference<'_, Self>> for OnlyOneHasNestedItems {
        #[inline(always)]
        fn unpin_assign(&mut self, __param_0: ::ctor::RvalueReference<'_, Self>) {
            unsafe {
                crate::detail::__rust_thunk___ZN19no_longer_top_level21OnlyOneHasNestedItemsaSEOS0_(
                    self, __param_0,
                );
            }
        }
    }

    pub mod only_one_has_nested_items {
        #[allow(unused_imports)]
        use super::*;

        pub type Inner = ::core::ffi::c_int;
    }

    #[derive(Clone, Copy)]
    #[repr(C)]
    ///CRUBIT_ANNOTATE: cpp_type=no_longer_top_level :: OnlyOneHasNestedItems_
    pub struct OnlyOneHasNestedItems_ {
        __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
    }
    impl !Send for OnlyOneHasNestedItems_ {}
    impl !Sync for OnlyOneHasNestedItems_ {}
    unsafe impl ::cxx::ExternType for OnlyOneHasNestedItems_ {
        type Id = ::cxx::type_id!("no_longer_top_level :: OnlyOneHasNestedItems_");
        type Kind = ::cxx::kind::Trivial;
    }
    forward_declare::unsafe_define!(
        forward_declare::symbol!("no_longer_top_level :: OnlyOneHasNestedItems_"),
        crate::no_longer_top_level::OnlyOneHasNestedItems_
    );

    impl Default for OnlyOneHasNestedItems_ {
        #[inline(always)]
        fn default() -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk___ZN19no_longer_top_level22OnlyOneHasNestedItems_C1Ev(
                    &raw mut tmp as *mut ::core::ffi::c_void,
                );
                tmp.assume_init()
            }
        }
    }

    impl From<::ctor::RvalueReference<'_, Self>> for OnlyOneHasNestedItems_ {
        #[inline(always)]
        fn from(__param_0: ::ctor::RvalueReference<'_, Self>) -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk___ZN19no_longer_top_level22OnlyOneHasNestedItems_C1EOS0_(&raw mut tmp as*mut::core::ffi::c_void,__param_0);
                tmp.assume_init()
            }
        }
    }
    impl ::ctor::CtorNew<::ctor::RvalueReference<'_, Self>> for OnlyOneHasNestedItems_ {
        type CtorType = Self;
        type Error = ::ctor::Infallible;
        #[inline(always)]
        fn ctor_new(args: ::ctor::RvalueReference<'_, Self>) -> Self::CtorType {
            <Self as From<::ctor::RvalueReference<'_, Self>>>::from(args)
        }
    }

    impl ::ctor::UnpinAssign<&Self> for OnlyOneHasNestedItems_ {
        #[inline(always)]
        fn unpin_assign(&mut self, __param_0: &Self) {
            unsafe {
                crate::detail::__rust_thunk___ZN19no_longer_top_level22OnlyOneHasNestedItems_aSERKS0_(self,__param_0);
            }
        }
    }

    impl ::ctor::UnpinAssign<::ctor::RvalueReference<'_, Self>> for OnlyOneHasNestedItems_ {
        #[inline(always)]
        fn unpin_assign(&mut self, __param_0: ::ctor::RvalueReference<'_, Self>) {
            unsafe {
                crate::detail::__rust_thunk___ZN19no_longer_top_level22OnlyOneHasNestedItems_aSEOS0_(self,__param_0);
            }
        }
    }

    // no nested items

    #[derive(Clone, Copy)]
    #[repr(C)]
    ///CRUBIT_ANNOTATE: cpp_type=no_longer_top_level :: SameNameAsNamespace
    pub struct SameNameAsNamespace {
        pub i: ::core::ffi::c_int,
    }
    impl !Send for SameNameAsNamespace {}
    impl !Sync for SameNameAsNamespace {}
    unsafe impl ::cxx::ExternType for SameNameAsNamespace {
        type Id = ::cxx::type_id!("no_longer_top_level :: SameNameAsNamespace");
        type Kind = ::cxx::kind::Trivial;
    }
    forward_declare::unsafe_define!(
        forward_declare::symbol!("no_longer_top_level :: SameNameAsNamespace"),
        crate::no_longer_top_level::SameNameAsNamespace
    );

    impl Default for SameNameAsNamespace {
        #[inline(always)]
        fn default() -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk___ZN19no_longer_top_level19SameNameAsNamespaceC1Ev(
                    &raw mut tmp as *mut ::core::ffi::c_void,
                );
                tmp.assume_init()
            }
        }
    }

    impl From<::ctor::RvalueReference<'_, Self>> for SameNameAsNamespace {
        #[inline(always)]
        fn from(__param_0: ::ctor::RvalueReference<'_, Self>) -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk___ZN19no_longer_top_level19SameNameAsNamespaceC1EOS0_(
                    &raw mut tmp as *mut ::core::ffi::c_void,
                    __param_0,
                );
                tmp.assume_init()
            }
        }
    }
    impl ::ctor::CtorNew<::ctor::RvalueReference<'_, Self>> for SameNameAsNamespace {
        type CtorType = Self;
        type Error = ::ctor::Infallible;
        #[inline(always)]
        fn ctor_new(args: ::ctor::RvalueReference<'_, Self>) -> Self::CtorType {
            <Self as From<::ctor::RvalueReference<'_, Self>>>::from(args)
        }
    }

    impl ::ctor::UnpinAssign<&Self> for SameNameAsNamespace {
        #[inline(always)]
        fn unpin_assign(&mut self, __param_0: &Self) {
            unsafe {
                crate::detail::__rust_thunk___ZN19no_longer_top_level19SameNameAsNamespaceaSERKS0_(
                    self, __param_0,
                );
            }
        }
    }

    impl ::ctor::UnpinAssign<::ctor::RvalueReference<'_, Self>> for SameNameAsNamespace {
        #[inline(always)]
        fn unpin_assign(&mut self, __param_0: ::ctor::RvalueReference<'_, Self>) {
            unsafe {
                crate::detail::__rust_thunk___ZN19no_longer_top_level19SameNameAsNamespaceaSEOS0_(
                    self, __param_0,
                );
            }
        }
    }

    // Error while generating bindings for type alias 'Inner':
    // parent record has nested items, but the module to contain them could not be generated because another item named `same_name_as_namespace` already exists

    // namespace same_name_as_namespace

    pub mod same_name_as_namespace {
        #[derive(Clone, Copy)]
        #[repr(C)]
        ///CRUBIT_ANNOTATE: cpp_type=no_longer_top_level :: same_name_as_namespace :: Foo
        pub struct Foo {
            __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
        }
        impl !Send for Foo {}
        impl !Sync for Foo {}
        unsafe impl ::cxx::ExternType for Foo {
            type Id = ::cxx::type_id!("no_longer_top_level :: same_name_as_namespace :: Foo");
            type Kind = ::cxx::kind::Trivial;
        }
        forward_declare::unsafe_define!(
            forward_declare::symbol!("no_longer_top_level :: same_name_as_namespace :: Foo"),
            crate::no_longer_top_level::same_name_as_namespace::Foo
        );

        impl Default for Foo {
            #[inline(always)]
            fn default() -> Self {
                let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
                unsafe {
                    crate::detail::__rust_thunk___ZN19no_longer_top_level22same_name_as_namespace3FooC1Ev(&raw mut tmp as*mut::core::ffi::c_void);
                    tmp.assume_init()
                }
            }
        }

        impl From<::ctor::RvalueReference<'_, Self>> for Foo {
            #[inline(always)]
            fn from(__param_0: ::ctor::RvalueReference<'_, Self>) -> Self {
                let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
                unsafe {
                    crate::detail::__rust_thunk___ZN19no_longer_top_level22same_name_as_namespace3FooC1EOS1_(&raw mut tmp as*mut::core::ffi::c_void,__param_0);
                    tmp.assume_init()
                }
            }
        }
        impl ::ctor::CtorNew<::ctor::RvalueReference<'_, Self>> for Foo {
            type CtorType = Self;
            type Error = ::ctor::Infallible;
            #[inline(always)]
            fn ctor_new(args: ::ctor::RvalueReference<'_, Self>) -> Self::CtorType {
                <Self as From<::ctor::RvalueReference<'_, Self>>>::from(args)
            }
        }

        impl ::ctor::UnpinAssign<&Self> for Foo {
            #[inline(always)]
            fn unpin_assign(&mut self, __param_0: &Self) {
                unsafe {
                    crate::detail::__rust_thunk___ZN19no_longer_top_level22same_name_as_namespace3FooaSERKS1_(self,__param_0);
                }
            }
        }

        impl ::ctor::UnpinAssign<::ctor::RvalueReference<'_, Self>> for Foo {
            #[inline(always)]
            fn unpin_assign(&mut self, __param_0: ::ctor::RvalueReference<'_, Self>) {
                unsafe {
                    crate::detail::__rust_thunk___ZN19no_longer_top_level22same_name_as_namespace3FooaSEOS1_(self,__param_0);
                }
            }
        }

        #[derive(Clone, Copy)]
        #[repr(C)]
        ///CRUBIT_ANNOTATE: cpp_type=no_longer_top_level :: same_name_as_namespace :: Bar
        pub struct Bar {
            __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
        }
        impl !Send for Bar {}
        impl !Sync for Bar {}
        unsafe impl ::cxx::ExternType for Bar {
            type Id = ::cxx::type_id!("no_longer_top_level :: same_name_as_namespace :: Bar");
            type Kind = ::cxx::kind::Trivial;
        }
        forward_declare::unsafe_define!(
            forward_declare::symbol!("no_longer_top_level :: same_name_as_namespace :: Bar"),
            crate::no_longer_top_level::same_name_as_namespace::Bar
        );

        impl Default for Bar {
            #[inline(always)]
            fn default() -> Self {
                let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
                unsafe {
                    crate::detail::__rust_thunk___ZN19no_longer_top_level22same_name_as_namespace3BarC1Ev(&raw mut tmp as*mut::core::ffi::c_void);
                    tmp.assume_init()
                }
            }
        }

        impl From<::ctor::RvalueReference<'_, Self>> for Bar {
            #[inline(always)]
            fn from(__param_0: ::ctor::RvalueReference<'_, Self>) -> Self {
                let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
                unsafe {
                    crate::detail::__rust_thunk___ZN19no_longer_top_level22same_name_as_namespace3BarC1EOS1_(&raw mut tmp as*mut::core::ffi::c_void,__param_0);
                    tmp.assume_init()
                }
            }
        }
        impl ::ctor::CtorNew<::ctor::RvalueReference<'_, Self>> for Bar {
            type CtorType = Self;
            type Error = ::ctor::Infallible;
            #[inline(always)]
            fn ctor_new(args: ::ctor::RvalueReference<'_, Self>) -> Self::CtorType {
                <Self as From<::ctor::RvalueReference<'_, Self>>>::from(args)
            }
        }

        impl ::ctor::UnpinAssign<&Self> for Bar {
            #[inline(always)]
            fn unpin_assign(&mut self, __param_0: &Self) {
                unsafe {
                    crate::detail::__rust_thunk___ZN19no_longer_top_level22same_name_as_namespace3BaraSERKS1_(self,__param_0);
                }
            }
        }

        impl ::ctor::UnpinAssign<::ctor::RvalueReference<'_, Self>> for Bar {
            #[inline(always)]
            fn unpin_assign(&mut self, __param_0: ::ctor::RvalueReference<'_, Self>) {
                unsafe {
                    crate::detail::__rust_thunk___ZN19no_longer_top_level22same_name_as_namespace3BaraSEOS1_(self,__param_0);
                }
            }
        }
    }

    // namespace same_name_as_namespace
}

// namespace no_longer_top_level

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN3FooC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN3FooC1EOS_(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'_, crate::Foo>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN3FooaSERKS_<'__return_lifetime>(
            __this: &mut crate::Foo,
            __param_0: &crate::Foo,
        ) -> &'__return_lifetime mut crate::Foo;
        pub(crate) unsafe fn __rust_thunk___ZN3FooaSEOS_<'__return_lifetime>(
            __this: &mut crate::Foo,
            __param_0: ::ctor::RvalueReference<'_, crate::Foo>,
        ) -> &'__return_lifetime mut crate::Foo;
        pub(crate) unsafe fn __rust_thunk___ZN3Foo3BarC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN3Foo3BarC1EOS0_(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'_, crate::foo::Bar>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN3Foo3BaraSERKS0_<'__return_lifetime>(
            __this: &mut crate::foo::Bar,
            __param_0: &crate::foo::Bar,
        ) -> &'__return_lifetime mut crate::foo::Bar;
        pub(crate) unsafe fn __rust_thunk___ZN3Foo3BaraSEOS0_<'__return_lifetime>(
            __this: &mut crate::foo::Bar,
            __param_0: ::ctor::RvalueReference<'_, crate::foo::Bar>,
        ) -> &'__return_lifetime mut crate::foo::Bar;
        pub(crate) unsafe fn __rust_thunk___ZN3Foo3Bar3BazC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN3Foo3Bar3BazC1EOS1_(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'_, crate::foo::bar::Baz>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN3Foo3Bar3BazaSERKS1_<'__return_lifetime>(
            __this: &mut crate::foo::bar::Baz,
            __param_0: &crate::foo::bar::Baz,
        ) -> &'__return_lifetime mut crate::foo::bar::Baz;
        pub(crate) unsafe fn __rust_thunk___ZN3Foo3Bar3BazaSEOS1_<'__return_lifetime>(
            __this: &mut crate::foo::bar::Baz,
            __param_0: ::ctor::RvalueReference<'_, crate::foo::bar::Baz>,
        ) -> &'__return_lifetime mut crate::foo::bar::Baz;
        pub(crate) unsafe fn __rust_thunk___ZN18already_snake_caseC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN18already_snake_caseC1EOS_(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'_, crate::already_snake_case>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN18already_snake_caseaSERKS_<'__return_lifetime>(
            __this: &mut crate::already_snake_case,
            __param_0: &crate::already_snake_case,
        ) -> &'__return_lifetime mut crate::already_snake_case;
        pub(crate) unsafe fn __rust_thunk___ZN18already_snake_caseaSEOS_<'__return_lifetime>(
            __this: &mut crate::already_snake_case,
            __param_0: ::ctor::RvalueReference<'_, crate::already_snake_case>,
        ) -> &'__return_lifetime mut crate::already_snake_case;
        pub(crate) unsafe fn __rust_thunk___ZN25ConflictingSnakeCaseNamesC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN25ConflictingSnakeCaseNamesC1EOS_(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'_, crate::ConflictingSnakeCaseNames>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN25ConflictingSnakeCaseNamesaSERKS_<
            '__return_lifetime,
        >(
            __this: &mut crate::ConflictingSnakeCaseNames,
            __param_0: &crate::ConflictingSnakeCaseNames,
        ) -> &'__return_lifetime mut crate::ConflictingSnakeCaseNames;
        pub(crate) unsafe fn __rust_thunk___ZN25ConflictingSnakeCaseNamesaSEOS_<
            '__return_lifetime,
        >(
            __this: &mut crate::ConflictingSnakeCaseNames,
            __param_0: ::ctor::RvalueReference<'_, crate::ConflictingSnakeCaseNames>,
        ) -> &'__return_lifetime mut crate::ConflictingSnakeCaseNames;
        pub(crate) unsafe fn __rust_thunk___ZN26ConflictingSnakeCaseNames_C1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN26ConflictingSnakeCaseNames_C1EOS_(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'_, crate::ConflictingSnakeCaseNames_>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN26ConflictingSnakeCaseNames_aSERKS_<
            '__return_lifetime,
        >(
            __this: &mut crate::ConflictingSnakeCaseNames_,
            __param_0: &crate::ConflictingSnakeCaseNames_,
        ) -> &'__return_lifetime mut crate::ConflictingSnakeCaseNames_;
        pub(crate) unsafe fn __rust_thunk___ZN26ConflictingSnakeCaseNames_aSEOS_<
            '__return_lifetime,
        >(
            __this: &mut crate::ConflictingSnakeCaseNames_,
            __param_0: ::ctor::RvalueReference<'_, crate::ConflictingSnakeCaseNames_>,
        ) -> &'__return_lifetime mut crate::ConflictingSnakeCaseNames_;
        pub(crate) unsafe fn __rust_thunk___ZN21OnlyOneHasNestedItemsC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN21OnlyOneHasNestedItemsC1EOS_(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'_, crate::OnlyOneHasNestedItems>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN21OnlyOneHasNestedItemsaSERKS_<'__return_lifetime>(
            __this: &mut crate::OnlyOneHasNestedItems,
            __param_0: &crate::OnlyOneHasNestedItems,
        ) -> &'__return_lifetime mut crate::OnlyOneHasNestedItems;
        pub(crate) unsafe fn __rust_thunk___ZN21OnlyOneHasNestedItemsaSEOS_<'__return_lifetime>(
            __this: &mut crate::OnlyOneHasNestedItems,
            __param_0: ::ctor::RvalueReference<'_, crate::OnlyOneHasNestedItems>,
        ) -> &'__return_lifetime mut crate::OnlyOneHasNestedItems;
        pub(crate) unsafe fn __rust_thunk___ZN22OnlyOneHasNestedItems_C1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN22OnlyOneHasNestedItems_C1EOS_(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'_, crate::OnlyOneHasNestedItems_>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN22OnlyOneHasNestedItems_aSERKS_<'__return_lifetime>(
            __this: &mut crate::OnlyOneHasNestedItems_,
            __param_0: &crate::OnlyOneHasNestedItems_,
        ) -> &'__return_lifetime mut crate::OnlyOneHasNestedItems_;
        pub(crate) unsafe fn __rust_thunk___ZN22OnlyOneHasNestedItems_aSEOS_<'__return_lifetime>(
            __this: &mut crate::OnlyOneHasNestedItems_,
            __param_0: ::ctor::RvalueReference<'_, crate::OnlyOneHasNestedItems_>,
        ) -> &'__return_lifetime mut crate::OnlyOneHasNestedItems_;
        pub(crate) unsafe fn __rust_thunk___ZN19SameNameAsNamespaceC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN19SameNameAsNamespaceC1EOS_(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'_, crate::SameNameAsNamespace>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN19SameNameAsNamespaceaSERKS_<'__return_lifetime>(
            __this: &mut crate::SameNameAsNamespace,
            __param_0: &crate::SameNameAsNamespace,
        ) -> &'__return_lifetime mut crate::SameNameAsNamespace;
        pub(crate) unsafe fn __rust_thunk___ZN19SameNameAsNamespaceaSEOS_<'__return_lifetime>(
            __this: &mut crate::SameNameAsNamespace,
            __param_0: ::ctor::RvalueReference<'_, crate::SameNameAsNamespace>,
        ) -> &'__return_lifetime mut crate::SameNameAsNamespace;
        pub(crate) unsafe fn __rust_thunk___ZN22same_name_as_namespace3FooC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN22same_name_as_namespace3FooC1EOS0_(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'_, crate::same_name_as_namespace::Foo>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN22same_name_as_namespace3FooaSERKS0_<
            '__return_lifetime,
        >(
            __this: &mut crate::same_name_as_namespace::Foo,
            __param_0: &crate::same_name_as_namespace::Foo,
        ) -> &'__return_lifetime mut crate::same_name_as_namespace::Foo;
        pub(crate) unsafe fn __rust_thunk___ZN22same_name_as_namespace3FooaSEOS0_<
            '__return_lifetime,
        >(
            __this: &mut crate::same_name_as_namespace::Foo,
            __param_0: ::ctor::RvalueReference<'_, crate::same_name_as_namespace::Foo>,
        ) -> &'__return_lifetime mut crate::same_name_as_namespace::Foo;
        pub(crate) unsafe fn __rust_thunk___ZN22same_name_as_namespace3BarC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN22same_name_as_namespace3BarC1EOS0_(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'_, crate::same_name_as_namespace::Bar>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN22same_name_as_namespace3BaraSERKS0_<
            '__return_lifetime,
        >(
            __this: &mut crate::same_name_as_namespace::Bar,
            __param_0: &crate::same_name_as_namespace::Bar,
        ) -> &'__return_lifetime mut crate::same_name_as_namespace::Bar;
        pub(crate) unsafe fn __rust_thunk___ZN22same_name_as_namespace3BaraSEOS0_<
            '__return_lifetime,
        >(
            __this: &mut crate::same_name_as_namespace::Bar,
            __param_0: ::ctor::RvalueReference<'_, crate::same_name_as_namespace::Bar>,
        ) -> &'__return_lifetime mut crate::same_name_as_namespace::Bar;
        pub(crate) unsafe fn __rust_thunk___ZN19no_longer_top_level18already_snake_caseC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN19no_longer_top_level18already_snake_caseC1EOS0_(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'_, crate::no_longer_top_level::already_snake_case>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN19no_longer_top_level18already_snake_caseaSERKS0_<
            '__return_lifetime,
        >(
            __this: &mut crate::no_longer_top_level::already_snake_case,
            __param_0: &crate::no_longer_top_level::already_snake_case,
        ) -> &'__return_lifetime mut crate::no_longer_top_level::already_snake_case;
        pub(crate) unsafe fn __rust_thunk___ZN19no_longer_top_level18already_snake_caseaSEOS0_<
            '__return_lifetime,
        >(
            __this: &mut crate::no_longer_top_level::already_snake_case,
            __param_0: ::ctor::RvalueReference<'_, crate::no_longer_top_level::already_snake_case>,
        ) -> &'__return_lifetime mut crate::no_longer_top_level::already_snake_case;
        pub(crate) unsafe fn __rust_thunk___ZN19no_longer_top_level25ConflictingSnakeCaseNamesC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN19no_longer_top_level25ConflictingSnakeCaseNamesC1EOS0_(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<
                '_,
                crate::no_longer_top_level::ConflictingSnakeCaseNames,
            >,
        );
        pub(crate) unsafe fn __rust_thunk___ZN19no_longer_top_level25ConflictingSnakeCaseNamesaSERKS0_<
            '__return_lifetime,
        >(
            __this: &mut crate::no_longer_top_level::ConflictingSnakeCaseNames,
            __param_0: &crate::no_longer_top_level::ConflictingSnakeCaseNames,
        ) -> &'__return_lifetime mut crate::no_longer_top_level::ConflictingSnakeCaseNames;
        pub(crate) unsafe fn __rust_thunk___ZN19no_longer_top_level25ConflictingSnakeCaseNamesaSEOS0_<
            '__return_lifetime,
        >(
            __this: &mut crate::no_longer_top_level::ConflictingSnakeCaseNames,
            __param_0: ::ctor::RvalueReference<
                '_,
                crate::no_longer_top_level::ConflictingSnakeCaseNames,
            >,
        ) -> &'__return_lifetime mut crate::no_longer_top_level::ConflictingSnakeCaseNames;
        pub(crate) unsafe fn __rust_thunk___ZN19no_longer_top_level26ConflictingSnakeCaseNames_C1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN19no_longer_top_level26ConflictingSnakeCaseNames_C1EOS0_(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<
                '_,
                crate::no_longer_top_level::ConflictingSnakeCaseNames_,
            >,
        );
        pub(crate) unsafe fn __rust_thunk___ZN19no_longer_top_level26ConflictingSnakeCaseNames_aSERKS0_<
            '__return_lifetime,
        >(
            __this: &mut crate::no_longer_top_level::ConflictingSnakeCaseNames_,
            __param_0: &crate::no_longer_top_level::ConflictingSnakeCaseNames_,
        ) -> &'__return_lifetime mut crate::no_longer_top_level::ConflictingSnakeCaseNames_;
        pub(crate) unsafe fn __rust_thunk___ZN19no_longer_top_level26ConflictingSnakeCaseNames_aSEOS0_<
            '__return_lifetime,
        >(
            __this: &mut crate::no_longer_top_level::ConflictingSnakeCaseNames_,
            __param_0: ::ctor::RvalueReference<
                '_,
                crate::no_longer_top_level::ConflictingSnakeCaseNames_,
            >,
        ) -> &'__return_lifetime mut crate::no_longer_top_level::ConflictingSnakeCaseNames_;
        pub(crate) unsafe fn __rust_thunk___ZN19no_longer_top_level21OnlyOneHasNestedItemsC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN19no_longer_top_level21OnlyOneHasNestedItemsC1EOS0_(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<
                '_,
                crate::no_longer_top_level::OnlyOneHasNestedItems,
            >,
        );
        pub(crate) unsafe fn __rust_thunk___ZN19no_longer_top_level21OnlyOneHasNestedItemsaSERKS0_<
            '__return_lifetime,
        >(
            __this: &mut crate::no_longer_top_level::OnlyOneHasNestedItems,
            __param_0: &crate::no_longer_top_level::OnlyOneHasNestedItems,
        ) -> &'__return_lifetime mut crate::no_longer_top_level::OnlyOneHasNestedItems;
        pub(crate) unsafe fn __rust_thunk___ZN19no_longer_top_level21OnlyOneHasNestedItemsaSEOS0_<
            '__return_lifetime,
        >(
            __this: &mut crate::no_longer_top_level::OnlyOneHasNestedItems,
            __param_0: ::ctor::RvalueReference<
                '_,
                crate::no_longer_top_level::OnlyOneHasNestedItems,
            >,
        ) -> &'__return_lifetime mut crate::no_longer_top_level::OnlyOneHasNestedItems;
        pub(crate) unsafe fn __rust_thunk___ZN19no_longer_top_level22OnlyOneHasNestedItems_C1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN19no_longer_top_level22OnlyOneHasNestedItems_C1EOS0_(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<
                '_,
                crate::no_longer_top_level::OnlyOneHasNestedItems_,
            >,
        );
        pub(crate) unsafe fn __rust_thunk___ZN19no_longer_top_level22OnlyOneHasNestedItems_aSERKS0_<
            '__return_lifetime,
        >(
            __this: &mut crate::no_longer_top_level::OnlyOneHasNestedItems_,
            __param_0: &crate::no_longer_top_level::OnlyOneHasNestedItems_,
        ) -> &'__return_lifetime mut crate::no_longer_top_level::OnlyOneHasNestedItems_;
        pub(crate) unsafe fn __rust_thunk___ZN19no_longer_top_level22OnlyOneHasNestedItems_aSEOS0_<
            '__return_lifetime,
        >(
            __this: &mut crate::no_longer_top_level::OnlyOneHasNestedItems_,
            __param_0: ::ctor::RvalueReference<
                '_,
                crate::no_longer_top_level::OnlyOneHasNestedItems_,
            >,
        ) -> &'__return_lifetime mut crate::no_longer_top_level::OnlyOneHasNestedItems_;
        pub(crate) unsafe fn __rust_thunk___ZN19no_longer_top_level19SameNameAsNamespaceC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN19no_longer_top_level19SameNameAsNamespaceC1EOS0_(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'_, crate::no_longer_top_level::SameNameAsNamespace>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN19no_longer_top_level19SameNameAsNamespaceaSERKS0_<
            '__return_lifetime,
        >(
            __this: &mut crate::no_longer_top_level::SameNameAsNamespace,
            __param_0: &crate::no_longer_top_level::SameNameAsNamespace,
        ) -> &'__return_lifetime mut crate::no_longer_top_level::SameNameAsNamespace;
        pub(crate) unsafe fn __rust_thunk___ZN19no_longer_top_level19SameNameAsNamespaceaSEOS0_<
            '__return_lifetime,
        >(
            __this: &mut crate::no_longer_top_level::SameNameAsNamespace,
            __param_0: ::ctor::RvalueReference<'_, crate::no_longer_top_level::SameNameAsNamespace>,
        ) -> &'__return_lifetime mut crate::no_longer_top_level::SameNameAsNamespace;
        pub(crate) unsafe fn __rust_thunk___ZN19no_longer_top_level22same_name_as_namespace3FooC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN19no_longer_top_level22same_name_as_namespace3FooC1EOS1_(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<
                '_,
                crate::no_longer_top_level::same_name_as_namespace::Foo,
            >,
        );
        pub(crate) unsafe fn __rust_thunk___ZN19no_longer_top_level22same_name_as_namespace3FooaSERKS1_<
            '__return_lifetime,
        >(
            __this: &mut crate::no_longer_top_level::same_name_as_namespace::Foo,
            __param_0: &crate::no_longer_top_level::same_name_as_namespace::Foo,
        ) -> &'__return_lifetime mut crate::no_longer_top_level::same_name_as_namespace::Foo;
        pub(crate) unsafe fn __rust_thunk___ZN19no_longer_top_level22same_name_as_namespace3FooaSEOS1_<
            '__return_lifetime,
        >(
            __this: &mut crate::no_longer_top_level::same_name_as_namespace::Foo,
            __param_0: ::ctor::RvalueReference<
                '_,
                crate::no_longer_top_level::same_name_as_namespace::Foo,
            >,
        ) -> &'__return_lifetime mut crate::no_longer_top_level::same_name_as_namespace::Foo;
        pub(crate) unsafe fn __rust_thunk___ZN19no_longer_top_level22same_name_as_namespace3BarC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN19no_longer_top_level22same_name_as_namespace3BarC1EOS1_(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<
                '_,
                crate::no_longer_top_level::same_name_as_namespace::Bar,
            >,
        );
        pub(crate) unsafe fn __rust_thunk___ZN19no_longer_top_level22same_name_as_namespace3BaraSERKS1_<
            '__return_lifetime,
        >(
            __this: &mut crate::no_longer_top_level::same_name_as_namespace::Bar,
            __param_0: &crate::no_longer_top_level::same_name_as_namespace::Bar,
        ) -> &'__return_lifetime mut crate::no_longer_top_level::same_name_as_namespace::Bar;
        pub(crate) unsafe fn __rust_thunk___ZN19no_longer_top_level22same_name_as_namespace3BaraSEOS1_<
            '__return_lifetime,
        >(
            __this: &mut crate::no_longer_top_level::same_name_as_namespace::Bar,
            __param_0: ::ctor::RvalueReference<
                '_,
                crate::no_longer_top_level::same_name_as_namespace::Bar,
            >,
        ) -> &'__return_lifetime mut crate::no_longer_top_level::same_name_as_namespace::Bar;
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::foo::bar::Baz>() == 4);
    assert!(::core::mem::align_of::<crate::foo::bar::Baz>() == 4);
    static_assertions::assert_impl_all!(crate::foo::bar::Baz: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::foo::bar::Baz: Drop);
    assert!(::core::mem::offset_of!(crate::foo::bar::Baz, baz) == 0);
    assert!(::core::mem::size_of::<crate::foo::Bar>() == 4);
    assert!(::core::mem::align_of::<crate::foo::Bar>() == 4);
    static_assertions::assert_impl_all!(crate::foo::Bar: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::foo::Bar: Drop);
    assert!(::core::mem::offset_of!(crate::foo::Bar, bar) == 0);
    assert!(::core::mem::size_of::<crate::Foo>() == 4);
    assert!(::core::mem::align_of::<crate::Foo>() == 4);
    static_assertions::assert_impl_all!(crate::Foo: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::Foo: Drop);
    assert!(::core::mem::offset_of!(crate::Foo, foo) == 0);
    assert!(::core::mem::size_of::<crate::already_snake_case>() == 4);
    assert!(::core::mem::align_of::<crate::already_snake_case>() == 4);
    static_assertions::assert_impl_all!(crate::already_snake_case: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::already_snake_case: Drop);
    assert!(::core::mem::offset_of!(crate::already_snake_case, i) == 0);
    assert!(::core::mem::size_of::<crate::ConflictingSnakeCaseNames>() == 4);
    assert!(::core::mem::align_of::<crate::ConflictingSnakeCaseNames>() == 4);
    static_assertions::assert_impl_all!(crate::ConflictingSnakeCaseNames: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::ConflictingSnakeCaseNames: Drop);
    assert!(::core::mem::offset_of!(crate::ConflictingSnakeCaseNames, i) == 0);
    assert!(::core::mem::size_of::<crate::ConflictingSnakeCaseNames_>() == 4);
    assert!(::core::mem::align_of::<crate::ConflictingSnakeCaseNames_>() == 4);
    static_assertions::assert_impl_all!(crate::ConflictingSnakeCaseNames_: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::ConflictingSnakeCaseNames_: Drop);
    assert!(::core::mem::offset_of!(crate::ConflictingSnakeCaseNames_, i) == 0);
    assert!(::core::mem::size_of::<crate::OnlyOneHasNestedItems>() == 4);
    assert!(::core::mem::align_of::<crate::OnlyOneHasNestedItems>() == 4);
    static_assertions::assert_impl_all!(crate::OnlyOneHasNestedItems: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::OnlyOneHasNestedItems: Drop);
    assert!(::core::mem::offset_of!(crate::OnlyOneHasNestedItems, i) == 0);
    assert!(::core::mem::size_of::<crate::OnlyOneHasNestedItems_>() == 1);
    assert!(::core::mem::align_of::<crate::OnlyOneHasNestedItems_>() == 1);
    static_assertions::assert_impl_all!(crate::OnlyOneHasNestedItems_: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::OnlyOneHasNestedItems_: Drop);

    assert!(::core::mem::size_of::<crate::SameNameAsNamespace>() == 4);
    assert!(::core::mem::align_of::<crate::SameNameAsNamespace>() == 4);
    static_assertions::assert_impl_all!(crate::SameNameAsNamespace: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::SameNameAsNamespace: Drop);
    assert!(::core::mem::offset_of!(crate::SameNameAsNamespace, i) == 0);
    assert!(::core::mem::size_of::<crate::same_name_as_namespace::Foo>() == 1);
    assert!(::core::mem::align_of::<crate::same_name_as_namespace::Foo>() == 1);
    static_assertions::assert_impl_all!(crate::same_name_as_namespace::Foo: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::same_name_as_namespace::Foo: Drop);

    assert!(::core::mem::size_of::<crate::same_name_as_namespace::Bar>() == 1);
    assert!(::core::mem::align_of::<crate::same_name_as_namespace::Bar>() == 1);
    static_assertions::assert_impl_all!(crate::same_name_as_namespace::Bar: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::same_name_as_namespace::Bar: Drop);

    assert!(::core::mem::size_of::<crate::no_longer_top_level::already_snake_case>() == 4);
    assert!(::core::mem::align_of::<crate::no_longer_top_level::already_snake_case>() == 4);
    static_assertions::assert_impl_all!(crate::no_longer_top_level::already_snake_case: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::no_longer_top_level::already_snake_case: Drop);
    assert!(::core::mem::offset_of!(crate::no_longer_top_level::already_snake_case, i) == 0);
    assert!(::core::mem::size_of::<crate::no_longer_top_level::ConflictingSnakeCaseNames>() == 4);
    assert!(::core::mem::align_of::<crate::no_longer_top_level::ConflictingSnakeCaseNames>() == 4);
    static_assertions::assert_impl_all!(crate::no_longer_top_level::ConflictingSnakeCaseNames: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::no_longer_top_level::ConflictingSnakeCaseNames: Drop);
    assert!(::core::mem::offset_of!(crate::no_longer_top_level::ConflictingSnakeCaseNames, i) == 0);
    assert!(::core::mem::size_of::<crate::no_longer_top_level::ConflictingSnakeCaseNames_>() == 4);
    assert!(::core::mem::align_of::<crate::no_longer_top_level::ConflictingSnakeCaseNames_>() == 4);
    static_assertions::assert_impl_all!(crate::no_longer_top_level::ConflictingSnakeCaseNames_: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::no_longer_top_level::ConflictingSnakeCaseNames_: Drop);
    assert!(
        ::core::mem::offset_of!(crate::no_longer_top_level::ConflictingSnakeCaseNames_, i) == 0
    );
    assert!(::core::mem::size_of::<crate::no_longer_top_level::OnlyOneHasNestedItems>() == 4);
    assert!(::core::mem::align_of::<crate::no_longer_top_level::OnlyOneHasNestedItems>() == 4);
    static_assertions::assert_impl_all!(crate::no_longer_top_level::OnlyOneHasNestedItems: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::no_longer_top_level::OnlyOneHasNestedItems: Drop);
    assert!(::core::mem::offset_of!(crate::no_longer_top_level::OnlyOneHasNestedItems, i) == 0);
    assert!(::core::mem::size_of::<crate::no_longer_top_level::OnlyOneHasNestedItems_>() == 1);
    assert!(::core::mem::align_of::<crate::no_longer_top_level::OnlyOneHasNestedItems_>() == 1);
    static_assertions::assert_impl_all!(crate::no_longer_top_level::OnlyOneHasNestedItems_: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::no_longer_top_level::OnlyOneHasNestedItems_: Drop);

    assert!(::core::mem::size_of::<crate::no_longer_top_level::SameNameAsNamespace>() == 4);
    assert!(::core::mem::align_of::<crate::no_longer_top_level::SameNameAsNamespace>() == 4);
    static_assertions::assert_impl_all!(crate::no_longer_top_level::SameNameAsNamespace: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::no_longer_top_level::SameNameAsNamespace: Drop);
    assert!(::core::mem::offset_of!(crate::no_longer_top_level::SameNameAsNamespace, i) == 0);
    assert!(::core::mem::size_of::<crate::no_longer_top_level::same_name_as_namespace::Foo>() == 1);
    assert!(
        ::core::mem::align_of::<crate::no_longer_top_level::same_name_as_namespace::Foo>() == 1
    );
    static_assertions::assert_impl_all!(crate::no_longer_top_level::same_name_as_namespace::Foo: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::no_longer_top_level::same_name_as_namespace::Foo: Drop);

    assert!(::core::mem::size_of::<crate::no_longer_top_level::same_name_as_namespace::Bar>() == 1);
    assert!(
        ::core::mem::align_of::<crate::no_longer_top_level::same_name_as_namespace::Bar>() == 1
    );
    static_assertions::assert_impl_all!(crate::no_longer_top_level::same_name_as_namespace::Bar: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::no_longer_top_level::same_name_as_namespace::Bar: Drop);
};
