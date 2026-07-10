// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/absl_flat_hash_map:isolated

#![rustfmt::skip]
#![feature(custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![allow(deprecated)]
#![allow(unknown_lints, suspicious_runtime_symbol_definitions)]
#![deny(warnings)]
pub mod absl { // error: class `absl::flat_hash_map` could not be bound
               //   Class templates are not yet supported
}

// namespace absl

pub mod crubit {
    pub mod test {
        pub type MapWithTwoParams = crate::__CcTemplateInstN4absl13flat_hash_mapIimLi42EEE;

        pub type MapWithThreeParams = crate::__CcTemplateInstN4absl13flat_hash_mapIimLi42EEE;

        // error: type alias `crubit::test::MapWithBridgedKey` could not be bound
        //   depends on `crubit::test::MapWithBridgedKey` which cannot be bound because `crate::Bridged` cannot be used as a template argument because it is a bridged type
        //   See crubit.rs/types.

        // error: type alias `crubit::test::MapWithBridgedValue` could not be bound
        //   depends on `crubit::test::MapWithBridgedValue` which cannot be bound because `crate::Bridged` cannot be used as a template argument because it is a bridged type
        //   See crubit.rs/types.

        forward_declare::forward_declare!(pub Incomplete = forward_declare::symbol!(":: crubit :: test :: Incomplete"));

        // error: type alias `crubit::test::MapWithIncompleteKey` could not be bound
        //   depends on `crubit::test::MapWithIncompleteKey` which cannot be bound because `crate::crubit::test::Incomplete` can't be used in a Rust absl::flat_hash_map<K, _> because it is an incomplete type

        // error: type alias `crubit::test::MapWithIncompleteValue` could not be bound
        //   depends on `crubit::test::MapWithIncompleteValue` which cannot be bound because `crate::crubit::test::Incomplete` can't be used in a Rust absl::flat_hash_map<_, V> because it is an incomplete type

        #[::ctor::recursively_pinned]
        #[repr(C)]
        ///CRUBIT_ANNOTATE: cpp_type=:: crubit :: test :: NoDestructor
        pub struct NoDestructor {
            __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 1],
        }
        impl !Send for NoDestructor {}
        impl !Sync for NoDestructor {}
        unsafe impl ::cxx::ExternType for NoDestructor {
            type Id = ::cxx::type_id!(":: crubit :: test :: NoDestructor");
            type Kind = ::cxx::kind::Opaque;
        }
        forward_declare::unsafe_define!(
            forward_declare::symbol!(":: crubit :: test :: NoDestructor"),
            crate::crubit::test::NoDestructor
        );

        // error: constructor `crubit::test::NoDestructor::NoDestructor` could not be bound
        //   `NoDestructor` can't be used by-value because it has a non-public or deleted destructor

        // error: constructor `crubit::test::NoDestructor::NoDestructor` could not be bound
        //   `NoDestructor` can't be used by-value because it has a non-public or deleted destructor

        impl<'__param_0> ::ctor::Assign<&'__param_0 Self> for NoDestructor {
            #[inline(always)]
            fn assign<'__this>(
                self: ::core::pin::Pin<&'__this mut Self>,
                __param_0: &'__param_0 Self,
            ) {
                unsafe {
                    crate::detail::__rust_thunk___ZN6crubit4test12NoDestructoraSERKS1_(
                        self, __param_0,
                    );
                }
            }
        }

        // error: type alias `crubit::test::MapWithNoDestructorKey` could not be bound
        //   depends on `crubit::test::MapWithNoDestructorKey` which cannot be bound because `crate::crubit::test::NoDestructor` can't be used in a Rust absl::flat_hash_map<K, _> because it has a deleted or non-public destructor

        // error: type alias `crubit::test::MapWithNoDestructorValue` could not be bound
        //   depends on `crubit::test::MapWithNoDestructorValue` which cannot be bound because `crate::crubit::test::NoDestructor` can't be used in a Rust absl::flat_hash_map<_, V> because it has a deleted or non-public destructor

        #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
        #[repr(C)]
        ///CRUBIT_ANNOTATE: cpp_type=:: crubit :: test :: NoDelete
        pub struct NoDelete {
            __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
        }
        impl !Send for NoDelete {}
        impl !Sync for NoDelete {}
        unsafe impl ::cxx::ExternType for NoDelete {
            type Id = ::cxx::type_id!(":: crubit :: test :: NoDelete");
            type Kind = ::cxx::kind::Trivial;
        }
        forward_declare::unsafe_define!(
            forward_declare::symbol!(":: crubit :: test :: NoDelete"),
            crate::crubit::test::NoDelete
        );

        impl Default for NoDelete {
            #[inline(always)]
            fn default() -> Self {
                let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
                unsafe {
                    crate::detail::__rust_thunk___ZN6crubit4test8NoDeleteC1Ev(
                        &raw mut tmp as *mut _,
                    );
                    tmp.assume_init()
                }
            }
        }

        // error: type alias `crubit::test::MapWithNoDeleteKey` could not be bound
        //   depends on `crubit::test::MapWithNoDeleteKey` which cannot be bound because `crate::crubit::test::NoDelete` can't be used in a Rust absl::flat_hash_map<K, _> because it has a deleted or non-public operator delete

        // error: type alias `crubit::test::MapWithNoDeleteValue` could not be bound
        //   depends on `crubit::test::MapWithNoDeleteValue` which cannot be bound because `crate::crubit::test::NoDelete` can't be used in a Rust absl::flat_hash_map<_, V> because it has a deleted or non-public operator delete
    }
}

// namespace crubit::test

// error: class `absl::flat_hash_map<crubit::test::Incomplete, int, 42>` could not be bound
//   `crate::crubit::test::Incomplete` can't be used in a Rust absl::flat_hash_map<K, _> because it is an incomplete type

// error: class `absl::flat_hash_map<crubit::test::NoDestructor, int, 42>` could not be bound
//   `crate::crubit::test::NoDestructor` can't be used in a Rust absl::flat_hash_map<K, _> because it has a deleted or non-public destructor

// error: class `absl::flat_hash_map<crubit::test::Bridged, int, 42>` could not be bound
//   `crate::Bridged` cannot be used as a template argument because it is a bridged type
//   See crubit.rs/types.

// error: class `absl::flat_hash_map<crubit::test::NoDelete, int, 42>` could not be bound
//   `crate::crubit::test::NoDelete` can't be used in a Rust absl::flat_hash_map<K, _> because it has a deleted or non-public operator delete

// error: class `absl::flat_hash_map<int, crubit::test::Incomplete, 42>` could not be bound
//   `crate::crubit::test::Incomplete` can't be used in a Rust absl::flat_hash_map<_, V> because it is an incomplete type

// error: class `absl::flat_hash_map<int, crubit::test::NoDestructor, 42>` could not be bound
//   `crate::crubit::test::NoDestructor` can't be used in a Rust absl::flat_hash_map<_, V> because it has a deleted or non-public destructor

// error: class `absl::flat_hash_map<int, crubit::test::Bridged, 42>` could not be bound
//   `crate::Bridged` cannot be used as a template argument because it is a bridged type
//   See crubit.rs/types.

// error: class `absl::flat_hash_map<int, crubit::test::NoDelete, 42>` could not be bound
//   `crate::crubit::test::NoDelete` can't be used in a Rust absl::flat_hash_map<_, V> because it has a deleted or non-public operator delete

/// An empty implementation of absl::flat_hash_map to test code generation
/// without the absl dependency.
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=:: absl :: flat_hash_map < int , unsigned long , 42 >
pub struct __CcTemplateInstN4absl13flat_hash_mapIimLi42EEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for __CcTemplateInstN4absl13flat_hash_mapIimLi42EEE {}
impl !Sync for __CcTemplateInstN4absl13flat_hash_mapIimLi42EEE {}
forward_declare::unsafe_define!(
    forward_declare::symbol!(":: absl :: flat_hash_map < int , unsigned long , 42 >"),
    crate::__CcTemplateInstN4absl13flat_hash_mapIimLi42EEE
);

impl Default for __CcTemplateInstN4absl13flat_hash_mapIimLi42EEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__25d7606d__ZN4absl13flat_hash_mapIimLi42EEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

// error: function `absl::flat_hash_map<int, unsigned long>::HarmlessTemplateFunction` could not be bound
//   Function templates are not yet supported

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test12NoDestructoraSERKS1_<
            '__param_0,
            '__this,
        >(
            __this: ::core::pin::Pin<&'__this mut crate::crubit::test::NoDestructor>,
            __param_0: &'__param_0 crate::crubit::test::NoDestructor,
        ) -> ::core::pin::Pin<&'__this mut crate::crubit::test::NoDestructor>;
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test8NoDeleteC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__25d7606d__ZN4absl13flat_hash_mapIimLi42EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::crubit::test::NoDestructor>() == 1);
    assert!(::core::mem::align_of::<crate::crubit::test::NoDestructor>() == 1);
    static_assertions::assert_not_impl_any!(crate::crubit::test::NoDestructor: Copy,Drop);

    assert!(::core::mem::size_of::<crate::crubit::test::NoDelete>() == 1);
    assert!(::core::mem::align_of::<crate::crubit::test::NoDelete>() == 1);
    static_assertions::assert_impl_all!(crate::crubit::test::NoDelete: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::crubit::test::NoDelete: Drop);

    assert!(::core::mem::size_of::<crate::__CcTemplateInstN4absl13flat_hash_mapIimLi42EEE>() == 1);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstN4absl13flat_hash_mapIimLi42EEE>() == 1);
    static_assertions::assert_impl_all!(crate::__CcTemplateInstN4absl13flat_hash_mapIimLi42EEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstN4absl13flat_hash_mapIimLi42EEE: Drop);
};
