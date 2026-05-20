// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/namespace/nested_items:nested_items
// Features: callables, supported, types

#![rustfmt::skip]
#![feature(custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![allow(deprecated)]
#![deny(warnings)]

pub mod same {
    /// Generated from: rs_bindings_from_cc/test/namespace/nested_items/nested_items.h;l=11
    #[inline(always)]
    pub fn AFunction() -> ::ffi_11::c_int {
        unsafe { crate::detail::__rust_thunk___ZN4same9AFunctionEv() }
    }
}

// namespace same

/// Generated from: rs_bindings_from_cc/test/namespace/nested_items/nested_items.h;l=14
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=Same
pub struct Same {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for Same {}
impl !Sync for Same {}
unsafe impl ::cxx::ExternType for Same {
    type Id = ::cxx::type_id!("Same");
    type Kind = ::cxx::kind::Trivial;
}
impl Same {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    ///
    /// Generated from: rs_bindings_from_cc/test/namespace/nested_items/nested_items.h;l=25
    #[inline(always)]
    pub unsafe fn Method(__this: *mut Self) -> ::ffi_11::c_int {
        unsafe { self::same_items::Method(__this) }
    }
}

/// Generated from: rs_bindings_from_cc/test/namespace/nested_items/nested_items.h;l=14
impl Default for Same {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN4SameC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

pub mod same_items {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    ///
    /// Generated from: rs_bindings_from_cc/test/namespace/nested_items/nested_items.h;l=25
    #[inline(always)]
    pub(crate) unsafe fn Method(__this: *mut crate::Same) -> ::ffi_11::c_int {
        unsafe { crate::detail::__rust_thunk___ZN4Same6MethodEv(__this) }
    }
    /// Generated from: rs_bindings_from_cc/test/namespace/nested_items/nested_items.h;l=16
    #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
    #[repr(C)]
    ///CRUBIT_ANNOTATE: cpp_type=Same :: NestedItem
    pub struct NestedItem {
        __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
    }
    impl !Send for NestedItem {}
    impl !Sync for NestedItem {}
    unsafe impl ::cxx::ExternType for NestedItem {
        type Id = ::cxx::type_id!("Same :: NestedItem");
        type Kind = ::cxx::kind::Trivial;
    }
    impl NestedItem {
        /// # Safety
        ///
        /// The caller must ensure that the following unsafe arguments are not misused by the function:
        /// * `__this`: raw pointer
        ///
        /// Generated from: rs_bindings_from_cc/test/namespace/nested_items/nested_items.h;l=17
        #[inline(always)]
        pub unsafe fn NestedItemFunction(__this: *mut Self) -> ::ffi_11::c_int {
            unsafe { self::nested_item::NestedItemFunction(__this) }
        }
    }

    /// Generated from: rs_bindings_from_cc/test/namespace/nested_items/nested_items.h;l=16
    impl Default for NestedItem {
        #[inline(always)]
        fn default() -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk___ZN4Same10NestedItemC1Ev(&raw mut tmp as *mut _);
                tmp.assume_init()
            }
        }
    }

    pub mod nested_item {
        /// # Safety
        ///
        /// The caller must ensure that the following unsafe arguments are not misused by the function:
        /// * `__this`: raw pointer
        ///
        /// Generated from: rs_bindings_from_cc/test/namespace/nested_items/nested_items.h;l=17
        #[inline(always)]
        pub(crate) unsafe fn NestedItemFunction(
            __this: *mut crate::same_items::NestedItem,
        ) -> ::ffi_11::c_int {
            unsafe {
                crate::detail::__rust_thunk___ZN4Same10NestedItem18NestedItemFunctionEv(__this)
            }
        }
    }

    /// Generated from: rs_bindings_from_cc/test/namespace/nested_items/nested_items.h;l=20
    #[repr(transparent)]
    #[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
    ///CRUBIT_ANNOTATE: cpp_type=Same :: NestedEnum
    pub struct NestedEnum(::ffi_11::c_int);
    impl NestedEnum {
        pub const kOne: NestedEnum = NestedEnum(::ffi_11::new_c_int(0));
        pub const kTwo: NestedEnum = NestedEnum(::ffi_11::new_c_int(1));
    }
    impl From<::ffi_11::c_int> for NestedEnum {
        fn from(value: ::ffi_11::c_int) -> NestedEnum {
            NestedEnum(value)
        }
    }
    impl From<NestedEnum> for ::ffi_11::c_int {
        fn from(value: NestedEnum) -> ::ffi_11::c_int {
            value.0
        }
    }
}

pub mod foo {
    /// Generated from: rs_bindings_from_cc/test/namespace/nested_items/nested_items.h;l=29
    #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
    #[repr(C)]
    ///CRUBIT_ANNOTATE: cpp_type=foo :: Foo
    pub struct Foo {
        __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
    }
    impl !Send for Foo {}
    impl !Sync for Foo {}
    unsafe impl ::cxx::ExternType for Foo {
        type Id = ::cxx::type_id!("foo :: Foo");
        type Kind = ::cxx::kind::Trivial;
    }

    /// Generated from: rs_bindings_from_cc/test/namespace/nested_items/nested_items.h;l=29
    impl Default for Foo {
        #[inline(always)]
        fn default() -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk___ZN3foo3FooC1Ev(&raw mut tmp as *mut _);
                tmp.assume_init()
            }
        }
    }

    pub mod foo {
        /// Generated from: rs_bindings_from_cc/test/namespace/nested_items/nested_items.h;l=30
        #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
        #[repr(C)]
        ///CRUBIT_ANNOTATE: cpp_type=foo :: Foo :: foo
        pub struct foo {
            __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
        }
        impl !Send for foo {}
        impl !Sync for foo {}
        unsafe impl ::cxx::ExternType for foo {
            type Id = ::cxx::type_id!("foo :: Foo :: foo");
            type Kind = ::cxx::kind::Trivial;
        }
        impl foo {
            /// Generated from: rs_bindings_from_cc/test/namespace/nested_items/nested_items.h;l=32
            #[inline(always)]
            pub fn BFunction() -> ::ffi_11::c_int {
                unsafe { self::foo_items::BFunction() }
            }
        }

        /// Generated from: rs_bindings_from_cc/test/namespace/nested_items/nested_items.h;l=30
        impl Default for foo {
            #[inline(always)]
            fn default() -> Self {
                let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
                unsafe {
                    crate::detail::__rust_thunk___ZN3foo3Foo3fooC1Ev(&raw mut tmp as *mut _);
                    tmp.assume_init()
                }
            }
        }

        pub mod foo_items {
            /// Generated from: rs_bindings_from_cc/test/namespace/nested_items/nested_items.h;l=32
            #[inline(always)]
            pub(crate) fn BFunction() -> ::ffi_11::c_int {
                unsafe { crate::detail::__rust_thunk___ZN3foo3Foo3foo9BFunctionEv() }
            }
            /// Generated from: rs_bindings_from_cc/test/namespace/nested_items/nested_items.h;l=31
            #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
            #[repr(C)]
            ///CRUBIT_ANNOTATE: cpp_type=foo :: Foo :: foo :: Item
            pub struct Item {
                __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
            }
            impl !Send for Item {}
            impl !Sync for Item {}
            unsafe impl ::cxx::ExternType for Item {
                type Id = ::cxx::type_id!("foo :: Foo :: foo :: Item");
                type Kind = ::cxx::kind::Trivial;
            }

            /// Generated from: rs_bindings_from_cc/test/namespace/nested_items/nested_items.h;l=31
            impl Default for Item {
                #[inline(always)]
                fn default() -> Self {
                    let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
                    unsafe {
                        crate::detail::__rust_thunk___ZN3foo3Foo3foo4ItemC1Ev(
                            &raw mut tmp as *mut _,
                        );
                        tmp.assume_init()
                    }
                }
            }
        }
    }
}

// namespace foo

/// Generated from: rs_bindings_from_cc/test/namespace/nested_items/nested_items.h;l=37
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=OuterCpp
pub struct OuterRustName {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for OuterRustName {}
impl !Sync for OuterRustName {}
unsafe impl ::cxx::ExternType for OuterRustName {
    type Id = ::cxx::type_id!("OuterCpp");
    type Kind = ::cxx::kind::Trivial;
}

/// Generated from: rs_bindings_from_cc/test/namespace/nested_items/nested_items.h;l=37
impl Default for OuterRustName {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN8OuterCppC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

pub mod outer_rust_name {
    /// Generated from: rs_bindings_from_cc/test/namespace/nested_items/nested_items.h;l=38
    #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
    #[repr(C)]
    ///CRUBIT_ANNOTATE: cpp_type=OuterCpp :: Inner
    pub struct Inner {
        __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
    }
    impl !Send for Inner {}
    impl !Sync for Inner {}
    unsafe impl ::cxx::ExternType for Inner {
        type Id = ::cxx::type_id!("OuterCpp :: Inner");
        type Kind = ::cxx::kind::Trivial;
    }

    /// Generated from: rs_bindings_from_cc/test/namespace/nested_items/nested_items.h;l=38
    impl Default for Inner {
        #[inline(always)]
        fn default() -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk___ZN8OuterCpp5InnerC1Ev(&raw mut tmp as *mut _);
                tmp.assume_init()
            }
        }
    }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN4same9AFunctionEv() -> ::ffi_11::c_int;
        pub(crate) unsafe fn __rust_thunk___ZN4SameC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN4Same10NestedItemC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN4Same10NestedItem18NestedItemFunctionEv(
            __this: *mut crate::same_items::NestedItem,
        ) -> ::ffi_11::c_int;
        pub(crate) unsafe fn __rust_thunk___ZN4Same6MethodEv(
            __this: *mut crate::Same,
        ) -> ::ffi_11::c_int;
        pub(crate) unsafe fn __rust_thunk___ZN3foo3FooC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN3foo3Foo3fooC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN3foo3Foo3foo4ItemC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN3foo3Foo3foo9BFunctionEv() -> ::ffi_11::c_int;
        pub(crate) unsafe fn __rust_thunk___ZN8OuterCppC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN8OuterCpp5InnerC1Ev(__this: *mut ::core::ffi::c_void);
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::same_items::NestedItem>() == 1);
    assert!(::core::mem::align_of::<crate::same_items::NestedItem>() == 1);
    static_assertions::assert_impl_all!(crate::same_items::NestedItem: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::same_items::NestedItem: Drop);

    assert!(::core::mem::size_of::<crate::Same>() == 1);
    assert!(::core::mem::align_of::<crate::Same>() == 1);
    static_assertions::assert_impl_all!(crate::Same: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::Same: Drop);

    assert!(::core::mem::size_of::<crate::foo::foo::foo_items::Item>() == 1);
    assert!(::core::mem::align_of::<crate::foo::foo::foo_items::Item>() == 1);
    static_assertions::assert_impl_all!(crate::foo::foo::foo_items::Item: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::foo::foo::foo_items::Item: Drop);

    assert!(::core::mem::size_of::<crate::foo::foo::foo>() == 1);
    assert!(::core::mem::align_of::<crate::foo::foo::foo>() == 1);
    static_assertions::assert_impl_all!(crate::foo::foo::foo: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::foo::foo::foo: Drop);

    assert!(::core::mem::size_of::<crate::foo::Foo>() == 1);
    assert!(::core::mem::align_of::<crate::foo::Foo>() == 1);
    static_assertions::assert_impl_all!(crate::foo::Foo: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::foo::Foo: Drop);

    assert!(::core::mem::size_of::<crate::outer_rust_name::Inner>() == 1);
    assert!(::core::mem::align_of::<crate::outer_rust_name::Inner>() == 1);
    static_assertions::assert_impl_all!(crate::outer_rust_name::Inner: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::outer_rust_name::Inner: Drop);

    assert!(::core::mem::size_of::<crate::OuterRustName>() == 1);
    assert!(::core::mem::align_of::<crate::OuterRustName>() == 1);
    static_assertions::assert_impl_all!(crate::OuterRustName: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::OuterRustName: Drop);
};
