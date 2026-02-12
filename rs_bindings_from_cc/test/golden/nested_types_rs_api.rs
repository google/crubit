// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:nested_types_cc

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=Foo
pub struct Foo {
    pub foo: ::ffi_11::c_int,
}
impl !Send for Foo {}
impl !Sync for Foo {}
unsafe impl ::cxx::ExternType for Foo {
    type Id = ::cxx::type_id!("Foo");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for Foo {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN3FooC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

pub mod foo {
    #[allow(unused_imports)]
    use super::*;

    #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
    #[repr(C)]
    ///CRUBIT_ANNOTATE: cpp_type=Foo :: Bar
    pub struct Bar {
        pub bar: ::ffi_11::c_int,
    }
    impl !Send for Bar {}
    impl !Sync for Bar {}
    unsafe impl ::cxx::ExternType for Bar {
        type Id = ::cxx::type_id!("Foo :: Bar");
        type Kind = ::cxx::kind::Trivial;
    }

    impl Default for Bar {
        #[inline(always)]
        fn default() -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk___ZN3Foo3BarC1Ev(&raw mut tmp as *mut _);
                tmp.assume_init()
            }
        }
    }

    pub mod bar {
        #[allow(unused_imports)]
        use super::*;

        #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
        #[repr(C)]
        ///CRUBIT_ANNOTATE: cpp_type=Foo :: Bar :: Baz
        pub struct Baz {
            pub baz: ::ffi_11::c_int,
        }
        impl !Send for Baz {}
        impl !Sync for Baz {}
        unsafe impl ::cxx::ExternType for Baz {
            type Id = ::cxx::type_id!("Foo :: Bar :: Baz");
            type Kind = ::cxx::kind::Trivial;
        }

        impl Default for Baz {
            #[inline(always)]
            fn default() -> Self {
                let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
                unsafe {
                    crate::detail::__rust_thunk___ZN3Foo3Bar3BazC1Ev(&raw mut tmp as *mut _);
                    tmp.assume_init()
                }
            }
        }
    }
}

/// # Safety
///
/// To call a function that accepts this type, you must uphold these requirements:
/// * Document why the following public unsafe fields of this type cannot be misused by callee:
///   * `i`: Crubit cannot assume unknown types are safe: crubit.rs/errors/nested_type: parent record has nested items, but the module to contain them could not be generated because another item named `already_snake_case` already exists
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=already_snake_case
pub struct already_snake_case {
    /// Reason for representing this field as a blob of bytes:
    /// crubit.rs/errors/nested_type: parent record has nested items, but the module to contain them could not be generated because another item named `already_snake_case` already exists
    pub(crate) i: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for already_snake_case {}
impl !Sync for already_snake_case {}
unsafe impl ::cxx::ExternType for already_snake_case {
    type Id = ::cxx::type_id!("already_snake_case");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for already_snake_case {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN18already_snake_caseC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

// Error while generating bindings for struct 'already_snake_case::Inner':
// crubit.rs/errors/nested_type: parent record has nested items, but the module to contain them could not be generated because another item named `already_snake_case` already exists

/// # Safety
///
/// To call a function that accepts this type, you must uphold these requirements:
/// * Document why the following public unsafe fields of this type cannot be misused by callee:
///   * `i`: Crubit cannot assume unknown types are safe: crubit.rs/errors/nested_type: records ["ConflictingSnakeCaseNames", "ConflictingSnakeCaseNames_"] all have nested items, but all map to the same nested module name: `conflicting_snake_case_names`
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=ConflictingSnakeCaseNames
pub struct ConflictingSnakeCaseNames {
    /// Reason for representing this field as a blob of bytes:
    /// crubit.rs/errors/nested_type: records ["ConflictingSnakeCaseNames", "ConflictingSnakeCaseNames_"] all have nested items, but all map to the same nested module name: `conflicting_snake_case_names`
    pub(crate) i: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for ConflictingSnakeCaseNames {}
impl !Sync for ConflictingSnakeCaseNames {}
unsafe impl ::cxx::ExternType for ConflictingSnakeCaseNames {
    type Id = ::cxx::type_id!("ConflictingSnakeCaseNames");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for ConflictingSnakeCaseNames {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN25ConflictingSnakeCaseNamesC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

// Error while generating bindings for struct 'ConflictingSnakeCaseNames::Inner':
// crubit.rs/errors/nested_type: records ["ConflictingSnakeCaseNames", "ConflictingSnakeCaseNames_"] all have nested items, but all map to the same nested module name: `conflicting_snake_case_names`

/// # Safety
///
/// To call a function that accepts this type, you must uphold these requirements:
/// * Document why the following public unsafe fields of this type cannot be misused by callee:
///   * `i`: Crubit cannot assume unknown types are safe: crubit.rs/errors/nested_type: records ["ConflictingSnakeCaseNames", "ConflictingSnakeCaseNames_"] all have nested items, but all map to the same nested module name: `conflicting_snake_case_names`
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=ConflictingSnakeCaseNames_
pub struct ConflictingSnakeCaseNames_ {
    /// Reason for representing this field as a blob of bytes:
    /// crubit.rs/errors/nested_type: records ["ConflictingSnakeCaseNames", "ConflictingSnakeCaseNames_"] all have nested items, but all map to the same nested module name: `conflicting_snake_case_names`
    pub(crate) i: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for ConflictingSnakeCaseNames_ {}
impl !Sync for ConflictingSnakeCaseNames_ {}
unsafe impl ::cxx::ExternType for ConflictingSnakeCaseNames_ {
    type Id = ::cxx::type_id!("ConflictingSnakeCaseNames_");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for ConflictingSnakeCaseNames_ {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN26ConflictingSnakeCaseNames_C1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

// Error while generating bindings for struct 'ConflictingSnakeCaseNames_::Inner':
// crubit.rs/errors/nested_type: records ["ConflictingSnakeCaseNames", "ConflictingSnakeCaseNames_"] all have nested items, but all map to the same nested module name: `conflicting_snake_case_names`

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
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

impl Default for OnlyOneHasNestedItems {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN21OnlyOneHasNestedItemsC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

pub mod only_one_has_nested_items {
    #[allow(unused_imports)]
    use super::*;

    #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
    #[repr(C)]
    ///CRUBIT_ANNOTATE: cpp_type=OnlyOneHasNestedItems :: Inner
    pub struct Inner {
        __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
    }
    impl !Send for Inner {}
    impl !Sync for Inner {}
    unsafe impl ::cxx::ExternType for Inner {
        type Id = ::cxx::type_id!("OnlyOneHasNestedItems :: Inner");
        type Kind = ::cxx::kind::Trivial;
    }

    impl Default for Inner {
        #[inline(always)]
        fn default() -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk___ZN21OnlyOneHasNestedItems5InnerC1Ev(
                    &raw mut tmp as *mut _,
                );
                tmp.assume_init()
            }
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
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

impl Default for OnlyOneHasNestedItems_ {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN22OnlyOneHasNestedItems_C1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

// no nested items

/// # Safety
///
/// To call a function that accepts this type, you must uphold these requirements:
/// * Document why the following public unsafe fields of this type cannot be misused by callee:
///   * `i`: Crubit cannot assume unknown types are safe: crubit.rs/errors/nested_type: parent record has nested items, but the module to contain them could not be generated because another item named `same_name_as_namespace` already exists
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=SameNameAsNamespace
pub struct SameNameAsNamespace {
    /// Reason for representing this field as a blob of bytes:
    /// crubit.rs/errors/nested_type: parent record has nested items, but the module to contain them could not be generated because another item named `same_name_as_namespace` already exists
    pub(crate) i: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for SameNameAsNamespace {}
impl !Sync for SameNameAsNamespace {}
unsafe impl ::cxx::ExternType for SameNameAsNamespace {
    type Id = ::cxx::type_id!("SameNameAsNamespace");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for SameNameAsNamespace {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN19SameNameAsNamespaceC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

// Error while generating bindings for struct 'SameNameAsNamespace::Inner':
// crubit.rs/errors/nested_type: parent record has nested items, but the module to contain them could not be generated because another item named `same_name_as_namespace` already exists

// namespace same_name_as_namespace

pub mod same_name_as_namespace {
    #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
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

    impl Default for Foo {
        #[inline(always)]
        fn default() -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk___ZN22same_name_as_namespace3FooC1Ev(
                    &raw mut tmp as *mut _,
                );
                tmp.assume_init()
            }
        }
    }

    #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
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

    impl Default for Bar {
        #[inline(always)]
        fn default() -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk___ZN22same_name_as_namespace3BarC1Ev(
                    &raw mut tmp as *mut _,
                );
                tmp.assume_init()
            }
        }
    }
}

// namespace same_name_as_namespace

pub mod no_longer_top_level {
    /// # Safety
    ///
    /// To call a function that accepts this type, you must uphold these requirements:
    /// * Document why the following public unsafe fields of this type cannot be misused by callee:
    ///   * `i`: Crubit cannot assume unknown types are safe: crubit.rs/errors/nested_type: parent record has nested items, but the module to contain them could not be generated because another item named `already_snake_case` already exists
    #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
    #[repr(C)]
    ///CRUBIT_ANNOTATE: cpp_type=no_longer_top_level :: already_snake_case
    pub struct already_snake_case {
        /// Reason for representing this field as a blob of bytes:
        /// crubit.rs/errors/nested_type: parent record has nested items, but the module to contain them could not be generated because another item named `already_snake_case` already exists
        pub(crate) i: [::core::mem::MaybeUninit<u8>; 1],
    }
    impl !Send for already_snake_case {}
    impl !Sync for already_snake_case {}
    unsafe impl ::cxx::ExternType for already_snake_case {
        type Id = ::cxx::type_id!("no_longer_top_level :: already_snake_case");
        type Kind = ::cxx::kind::Trivial;
    }

    impl Default for already_snake_case {
        #[inline(always)]
        fn default() -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk___ZN19no_longer_top_level18already_snake_caseC1Ev(
                    &raw mut tmp as *mut _,
                );
                tmp.assume_init()
            }
        }
    }

    // Error while generating bindings for struct 'no_longer_top_level::already_snake_case::Inner':
    // crubit.rs/errors/nested_type: parent record has nested items, but the module to contain them could not be generated because another item named `already_snake_case` already exists

    /// # Safety
    ///
    /// To call a function that accepts this type, you must uphold these requirements:
    /// * Document why the following public unsafe fields of this type cannot be misused by callee:
    ///   * `i`: Crubit cannot assume unknown types are safe: crubit.rs/errors/nested_type: records ["ConflictingSnakeCaseNames", "ConflictingSnakeCaseNames_"] all have nested items, but all map to the same nested module name: `conflicting_snake_case_names`
    #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
    #[repr(C)]
    ///CRUBIT_ANNOTATE: cpp_type=no_longer_top_level :: ConflictingSnakeCaseNames
    pub struct ConflictingSnakeCaseNames {
        /// Reason for representing this field as a blob of bytes:
        /// crubit.rs/errors/nested_type: records ["ConflictingSnakeCaseNames", "ConflictingSnakeCaseNames_"] all have nested items, but all map to the same nested module name: `conflicting_snake_case_names`
        pub(crate) i: [::core::mem::MaybeUninit<u8>; 1],
    }
    impl !Send for ConflictingSnakeCaseNames {}
    impl !Sync for ConflictingSnakeCaseNames {}
    unsafe impl ::cxx::ExternType for ConflictingSnakeCaseNames {
        type Id = ::cxx::type_id!("no_longer_top_level :: ConflictingSnakeCaseNames");
        type Kind = ::cxx::kind::Trivial;
    }

    impl Default for ConflictingSnakeCaseNames {
        #[inline(always)]
        fn default() -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk___ZN19no_longer_top_level25ConflictingSnakeCaseNamesC1Ev(&raw mut tmp as*mut _);
                tmp.assume_init()
            }
        }
    }

    // Error while generating bindings for struct 'no_longer_top_level::ConflictingSnakeCaseNames::Inner':
    // crubit.rs/errors/nested_type: records ["ConflictingSnakeCaseNames", "ConflictingSnakeCaseNames_"] all have nested items, but all map to the same nested module name: `conflicting_snake_case_names`

    /// # Safety
    ///
    /// To call a function that accepts this type, you must uphold these requirements:
    /// * Document why the following public unsafe fields of this type cannot be misused by callee:
    ///   * `i`: Crubit cannot assume unknown types are safe: crubit.rs/errors/nested_type: records ["ConflictingSnakeCaseNames", "ConflictingSnakeCaseNames_"] all have nested items, but all map to the same nested module name: `conflicting_snake_case_names`
    #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
    #[repr(C)]
    ///CRUBIT_ANNOTATE: cpp_type=no_longer_top_level :: ConflictingSnakeCaseNames_
    pub struct ConflictingSnakeCaseNames_ {
        /// Reason for representing this field as a blob of bytes:
        /// crubit.rs/errors/nested_type: records ["ConflictingSnakeCaseNames", "ConflictingSnakeCaseNames_"] all have nested items, but all map to the same nested module name: `conflicting_snake_case_names`
        pub(crate) i: [::core::mem::MaybeUninit<u8>; 1],
    }
    impl !Send for ConflictingSnakeCaseNames_ {}
    impl !Sync for ConflictingSnakeCaseNames_ {}
    unsafe impl ::cxx::ExternType for ConflictingSnakeCaseNames_ {
        type Id = ::cxx::type_id!("no_longer_top_level :: ConflictingSnakeCaseNames_");
        type Kind = ::cxx::kind::Trivial;
    }

    impl Default for ConflictingSnakeCaseNames_ {
        #[inline(always)]
        fn default() -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk___ZN19no_longer_top_level26ConflictingSnakeCaseNames_C1Ev(&raw mut tmp as*mut _);
                tmp.assume_init()
            }
        }
    }

    // Error while generating bindings for struct 'no_longer_top_level::ConflictingSnakeCaseNames_::Inner':
    // crubit.rs/errors/nested_type: records ["ConflictingSnakeCaseNames", "ConflictingSnakeCaseNames_"] all have nested items, but all map to the same nested module name: `conflicting_snake_case_names`

    #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
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

    impl Default for OnlyOneHasNestedItems {
        #[inline(always)]
        fn default() -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk___ZN19no_longer_top_level21OnlyOneHasNestedItemsC1Ev(
                    &raw mut tmp as *mut _,
                );
                tmp.assume_init()
            }
        }
    }

    pub mod only_one_has_nested_items {
        #[allow(unused_imports)]
        use super::*;

        #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
        #[repr(C)]
        ///CRUBIT_ANNOTATE: cpp_type=no_longer_top_level :: OnlyOneHasNestedItems :: Inner
        pub struct Inner {
            __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
        }
        impl !Send for Inner {}
        impl !Sync for Inner {}
        unsafe impl ::cxx::ExternType for Inner {
            type Id = ::cxx::type_id!("no_longer_top_level :: OnlyOneHasNestedItems :: Inner");
            type Kind = ::cxx::kind::Trivial;
        }

        impl Default for Inner {
            #[inline(always)]
            fn default() -> Self {
                let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
                unsafe {
                    crate::detail::__rust_thunk___ZN19no_longer_top_level21OnlyOneHasNestedItems5InnerC1Ev(&raw mut tmp as*mut _);
                    tmp.assume_init()
                }
            }
        }
    }

    #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
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

    impl Default for OnlyOneHasNestedItems_ {
        #[inline(always)]
        fn default() -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk___ZN19no_longer_top_level22OnlyOneHasNestedItems_C1Ev(
                    &raw mut tmp as *mut _,
                );
                tmp.assume_init()
            }
        }
    }

    // no nested items

    /// # Safety
    ///
    /// To call a function that accepts this type, you must uphold these requirements:
    /// * Document why the following public unsafe fields of this type cannot be misused by callee:
    ///   * `i`: Crubit cannot assume unknown types are safe: crubit.rs/errors/nested_type: parent record has nested items, but the module to contain them could not be generated because another item named `same_name_as_namespace` already exists
    #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
    #[repr(C)]
    ///CRUBIT_ANNOTATE: cpp_type=no_longer_top_level :: SameNameAsNamespace
    pub struct SameNameAsNamespace {
        /// Reason for representing this field as a blob of bytes:
        /// crubit.rs/errors/nested_type: parent record has nested items, but the module to contain them could not be generated because another item named `same_name_as_namespace` already exists
        pub(crate) i: [::core::mem::MaybeUninit<u8>; 1],
    }
    impl !Send for SameNameAsNamespace {}
    impl !Sync for SameNameAsNamespace {}
    unsafe impl ::cxx::ExternType for SameNameAsNamespace {
        type Id = ::cxx::type_id!("no_longer_top_level :: SameNameAsNamespace");
        type Kind = ::cxx::kind::Trivial;
    }

    impl Default for SameNameAsNamespace {
        #[inline(always)]
        fn default() -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk___ZN19no_longer_top_level19SameNameAsNamespaceC1Ev(
                    &raw mut tmp as *mut _,
                );
                tmp.assume_init()
            }
        }
    }

    // Error while generating bindings for struct 'no_longer_top_level::SameNameAsNamespace::Inner':
    // crubit.rs/errors/nested_type: parent record has nested items, but the module to contain them could not be generated because another item named `same_name_as_namespace` already exists

    // namespace same_name_as_namespace

    pub mod same_name_as_namespace {
        #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
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

        impl Default for Foo {
            #[inline(always)]
            fn default() -> Self {
                let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
                unsafe {
                    crate::detail::__rust_thunk___ZN19no_longer_top_level22same_name_as_namespace3FooC1Ev(&raw mut tmp as*mut _);
                    tmp.assume_init()
                }
            }
        }

        #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
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

        impl Default for Bar {
            #[inline(always)]
            fn default() -> Self {
                let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
                unsafe {
                    crate::detail::__rust_thunk___ZN19no_longer_top_level22same_name_as_namespace3BarC1Ev(&raw mut tmp as*mut _);
                    tmp.assume_init()
                }
            }
        }
    }

    // namespace same_name_as_namespace
}

// namespace no_longer_top_level

/// TODO(b/481667188): Nested should get bindings.
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=ContainsForwardDeclared
pub struct ContainsForwardDeclared {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for ContainsForwardDeclared {}
impl !Sync for ContainsForwardDeclared {}
unsafe impl ::cxx::ExternType for ContainsForwardDeclared {
    type Id = ::cxx::type_id!("ContainsForwardDeclared");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for ContainsForwardDeclared {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN23ContainsForwardDeclaredC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

// Error while generating bindings for struct 'ContainsForwardDeclared::Nested':
// Can't generate bindings for ContainsForwardDeclared::Nested due to missing bindings for its dependency: crubit.rs/errors/nested_type: Could not find parent's module name.
//   This is a bug. The parent's module name should always be
//   in the list. More info:
//     for item: ContainsForwardDeclared::Nested
//     inside parent module contains_forward_declared (originally ContainsForwardDeclared)

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN3FooC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN3Foo3BarC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN3Foo3Bar3BazC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN18already_snake_caseC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN25ConflictingSnakeCaseNamesC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN26ConflictingSnakeCaseNames_C1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN21OnlyOneHasNestedItemsC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN21OnlyOneHasNestedItems5InnerC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN22OnlyOneHasNestedItems_C1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN19SameNameAsNamespaceC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN22same_name_as_namespace3FooC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN22same_name_as_namespace3BarC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN19no_longer_top_level18already_snake_caseC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN19no_longer_top_level25ConflictingSnakeCaseNamesC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN19no_longer_top_level26ConflictingSnakeCaseNames_C1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN19no_longer_top_level21OnlyOneHasNestedItemsC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN19no_longer_top_level21OnlyOneHasNestedItems5InnerC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN19no_longer_top_level22OnlyOneHasNestedItems_C1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN19no_longer_top_level19SameNameAsNamespaceC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN19no_longer_top_level22same_name_as_namespace3FooC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN19no_longer_top_level22same_name_as_namespace3BarC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN23ContainsForwardDeclaredC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
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
    assert!(::core::mem::size_of::<crate::already_snake_case>() == 1);
    assert!(::core::mem::align_of::<crate::already_snake_case>() == 1);
    static_assertions::assert_impl_all!(crate::already_snake_case: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::already_snake_case: Drop);
    assert!(::core::mem::offset_of!(crate::already_snake_case, i) == 0);
    assert!(::core::mem::size_of::<crate::ConflictingSnakeCaseNames>() == 1);
    assert!(::core::mem::align_of::<crate::ConflictingSnakeCaseNames>() == 1);
    static_assertions::assert_impl_all!(crate::ConflictingSnakeCaseNames: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::ConflictingSnakeCaseNames: Drop);
    assert!(::core::mem::offset_of!(crate::ConflictingSnakeCaseNames, i) == 0);
    assert!(::core::mem::size_of::<crate::ConflictingSnakeCaseNames_>() == 1);
    assert!(::core::mem::align_of::<crate::ConflictingSnakeCaseNames_>() == 1);
    static_assertions::assert_impl_all!(crate::ConflictingSnakeCaseNames_: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::ConflictingSnakeCaseNames_: Drop);
    assert!(::core::mem::offset_of!(crate::ConflictingSnakeCaseNames_, i) == 0);
    assert!(::core::mem::size_of::<crate::only_one_has_nested_items::Inner>() == 1);
    assert!(::core::mem::align_of::<crate::only_one_has_nested_items::Inner>() == 1);
    static_assertions::assert_impl_all!(crate::only_one_has_nested_items::Inner: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::only_one_has_nested_items::Inner: Drop);

    assert!(::core::mem::size_of::<crate::OnlyOneHasNestedItems>() == 1);
    assert!(::core::mem::align_of::<crate::OnlyOneHasNestedItems>() == 1);
    static_assertions::assert_impl_all!(crate::OnlyOneHasNestedItems: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::OnlyOneHasNestedItems: Drop);
    assert!(::core::mem::offset_of!(crate::OnlyOneHasNestedItems, i) == 0);
    assert!(::core::mem::size_of::<crate::OnlyOneHasNestedItems_>() == 1);
    assert!(::core::mem::align_of::<crate::OnlyOneHasNestedItems_>() == 1);
    static_assertions::assert_impl_all!(crate::OnlyOneHasNestedItems_: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::OnlyOneHasNestedItems_: Drop);

    assert!(::core::mem::size_of::<crate::SameNameAsNamespace>() == 1);
    assert!(::core::mem::align_of::<crate::SameNameAsNamespace>() == 1);
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

    assert!(::core::mem::size_of::<crate::no_longer_top_level::already_snake_case>() == 1);
    assert!(::core::mem::align_of::<crate::no_longer_top_level::already_snake_case>() == 1);
    static_assertions::assert_impl_all!(crate::no_longer_top_level::already_snake_case: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::no_longer_top_level::already_snake_case: Drop);
    assert!(::core::mem::offset_of!(crate::no_longer_top_level::already_snake_case, i) == 0);
    assert!(::core::mem::size_of::<crate::no_longer_top_level::ConflictingSnakeCaseNames>() == 1);
    assert!(::core::mem::align_of::<crate::no_longer_top_level::ConflictingSnakeCaseNames>() == 1);
    static_assertions::assert_impl_all!(crate::no_longer_top_level::ConflictingSnakeCaseNames: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::no_longer_top_level::ConflictingSnakeCaseNames: Drop);
    assert!(::core::mem::offset_of!(crate::no_longer_top_level::ConflictingSnakeCaseNames, i) == 0);
    assert!(::core::mem::size_of::<crate::no_longer_top_level::ConflictingSnakeCaseNames_>() == 1);
    assert!(::core::mem::align_of::<crate::no_longer_top_level::ConflictingSnakeCaseNames_>() == 1);
    static_assertions::assert_impl_all!(crate::no_longer_top_level::ConflictingSnakeCaseNames_: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::no_longer_top_level::ConflictingSnakeCaseNames_: Drop);
    assert!(
        ::core::mem::offset_of!(crate::no_longer_top_level::ConflictingSnakeCaseNames_, i) == 0
    );
    assert!(
        ::core::mem::size_of::<crate::no_longer_top_level::only_one_has_nested_items::Inner>() == 1
    );
    assert!(
        ::core::mem::align_of::<crate::no_longer_top_level::only_one_has_nested_items::Inner>()
            == 1
    );
    static_assertions::assert_impl_all!(crate::no_longer_top_level::only_one_has_nested_items::Inner: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::no_longer_top_level::only_one_has_nested_items::Inner: Drop);

    assert!(::core::mem::size_of::<crate::no_longer_top_level::OnlyOneHasNestedItems>() == 1);
    assert!(::core::mem::align_of::<crate::no_longer_top_level::OnlyOneHasNestedItems>() == 1);
    static_assertions::assert_impl_all!(crate::no_longer_top_level::OnlyOneHasNestedItems: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::no_longer_top_level::OnlyOneHasNestedItems: Drop);
    assert!(::core::mem::offset_of!(crate::no_longer_top_level::OnlyOneHasNestedItems, i) == 0);
    assert!(::core::mem::size_of::<crate::no_longer_top_level::OnlyOneHasNestedItems_>() == 1);
    assert!(::core::mem::align_of::<crate::no_longer_top_level::OnlyOneHasNestedItems_>() == 1);
    static_assertions::assert_impl_all!(crate::no_longer_top_level::OnlyOneHasNestedItems_: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::no_longer_top_level::OnlyOneHasNestedItems_: Drop);

    assert!(::core::mem::size_of::<crate::no_longer_top_level::SameNameAsNamespace>() == 1);
    assert!(::core::mem::align_of::<crate::no_longer_top_level::SameNameAsNamespace>() == 1);
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

    assert!(::core::mem::size_of::<crate::ContainsForwardDeclared>() == 1);
    assert!(::core::mem::align_of::<crate::ContainsForwardDeclared>() == 1);
    static_assertions::assert_impl_all!(crate::ContainsForwardDeclared: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::ContainsForwardDeclared: Drop);
};
