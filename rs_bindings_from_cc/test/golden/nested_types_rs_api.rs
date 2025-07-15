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

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
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

pub mod foo {
    #[allow(unused_imports)]
    use super::*;

    #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
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

    pub mod bar {
        #[allow(unused_imports)]
        use super::*;

        #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
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
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
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

// Error while generating bindings for type alias 'Inner':
// parent record has nested items, but the module to contain them could not be generated because another item named `already_snake_case` already exists

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
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

// Error while generating bindings for type alias 'Inner':
// records ["ConflictingSnakeCaseNames", "ConflictingSnakeCaseNames_"] all have nested items, but all map to the same nested module name: `conflicting_snake_case_names`

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
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

// Error while generating bindings for type alias 'Inner':
// records ["ConflictingSnakeCaseNames", "ConflictingSnakeCaseNames_"] all have nested items, but all map to the same nested module name: `conflicting_snake_case_names`

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

pub mod only_one_has_nested_items {
    #[allow(unused_imports)]
    use super::*;

    pub type Inner = ::core::ffi::c_int;
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

// no nested items

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
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

// Error while generating bindings for type alias 'Inner':
// parent record has nested items, but the module to contain them could not be generated because another item named `same_name_as_namespace` already exists

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
}

// namespace same_name_as_namespace

pub mod no_longer_top_level {
    #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
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

    // Error while generating bindings for type alias 'Inner':
    // parent record has nested items, but the module to contain them could not be generated because another item named `already_snake_case` already exists

    #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
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

    // Error while generating bindings for type alias 'Inner':
    // records ["ConflictingSnakeCaseNames", "ConflictingSnakeCaseNames_"] all have nested items, but all map to the same nested module name: `conflicting_snake_case_names`

    #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
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

    // Error while generating bindings for type alias 'Inner':
    // records ["ConflictingSnakeCaseNames", "ConflictingSnakeCaseNames_"] all have nested items, but all map to the same nested module name: `conflicting_snake_case_names`

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

    pub mod only_one_has_nested_items {
        #[allow(unused_imports)]
        use super::*;

        pub type Inner = ::core::ffi::c_int;
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

    // no nested items

    #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
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

    // Error while generating bindings for type alias 'Inner':
    // parent record has nested items, but the module to contain them could not be generated because another item named `same_name_as_namespace` already exists

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
    }

    // namespace same_name_as_namespace
}

// namespace no_longer_top_level

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
