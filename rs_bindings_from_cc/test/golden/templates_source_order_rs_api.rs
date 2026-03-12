// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:templates_source_order_cc

#![rustfmt::skip]
#![feature(custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![deny(warnings)]

// error: class `MyTemplate` could not be bound
//   Class templates are not yet supported

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=TopLevel
pub struct TopLevel {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for TopLevel {}
impl !Sync for TopLevel {}
unsafe impl ::cxx::ExternType for TopLevel {
    type Id = ::cxx::type_id!("TopLevel");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for TopLevel {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN8TopLevelC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

// error: type alias `Alias1` could not be bound
//   template instantiation is not yet supported
//   template instantiation is not yet supported

// error: type alias `Alias2` could not be bound
//   template instantiation is not yet supported
//   template instantiation is not yet supported

// error: type alias `Alias3` could not be bound
//   template instantiation is not yet supported
//   template instantiation is not yet supported

// error: type alias `Alias4` could not be bound
//   template instantiation is not yet supported
//   template instantiation is not yet supported

// error: type alias `Alias5` could not be bound
//   template instantiation is not yet supported
//   template instantiation is not yet supported

// error: type alias `Alias6` could not be bound
//   template instantiation is not yet supported
//   template instantiation is not yet supported

pub mod test_namespace_bindings {
    #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
    #[repr(C)]
    ///CRUBIT_ANNOTATE: cpp_type=test_namespace_bindings :: Inner
    pub struct Inner {
        __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
    }
    impl !Send for Inner {}
    impl !Sync for Inner {}
    unsafe impl ::cxx::ExternType for Inner {
        type Id = ::cxx::type_id!("test_namespace_bindings :: Inner");
        type Kind = ::cxx::kind::Trivial;
    }

    impl Default for Inner {
        #[inline(always)]
        fn default() -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk___ZN23test_namespace_bindings5InnerC1Ev(
                    &raw mut tmp as *mut _,
                );
                tmp.assume_init()
            }
        }
    }

    // error: type alias `test_namespace_bindings::Alias7` could not be bound
    //   template instantiation is not yet supported
    //   template instantiation is not yet supported

    // error: type alias `test_namespace_bindings::Alias8` could not be bound
    //   template instantiation is not yet supported
    //   template instantiation is not yet supported

    // error: type alias `test_namespace_bindings::Alias9` could not be bound
    //   template instantiation is not yet supported
    //   template instantiation is not yet supported
}

// namespace test_namespace_bindings

// error: class `MyTemplate<TopLevel>` could not be bound
//   template instantiation is not yet supported
//   template instantiation is not yet supported

// error: class `MyTemplate<test_namespace_bindings::Inner>` could not be bound
//   template instantiation is not yet supported
//   template instantiation is not yet supported

// error: class `MyTemplate<MyTemplate<TopLevel>>` could not be bound
//   template instantiation is not yet supported
//   template instantiation is not yet supported

// error: class `MyTemplate<MyTemplate<test_namespace_bindings::Inner>>` could not be bound
//   template instantiation is not yet supported
//   template instantiation is not yet supported

// error: class `MyTemplate<bool>` could not be bound
//   template instantiation is not yet supported
//   template instantiation is not yet supported

// error: class `MyTemplate<char>` could not be bound
//   template instantiation is not yet supported
//   template instantiation is not yet supported

// error: class `MyTemplate<double>` could not be bound
//   template instantiation is not yet supported
//   template instantiation is not yet supported

// error: class `MyTemplate<float>` could not be bound
//   template instantiation is not yet supported
//   template instantiation is not yet supported

// error: class `MyTemplate<int>` could not be bound
//   template instantiation is not yet supported
//   template instantiation is not yet supported

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN8TopLevelC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN23test_namespace_bindings5InnerC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::TopLevel>() == 1);
    assert!(::core::mem::align_of::<crate::TopLevel>() == 1);
    static_assertions::assert_impl_all!(crate::TopLevel: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::TopLevel: Drop);

    assert!(::core::mem::size_of::<crate::test_namespace_bindings::Inner>() == 1);
    assert!(::core::mem::align_of::<crate::test_namespace_bindings::Inner>() == 1);
    static_assertions::assert_impl_all!(crate::test_namespace_bindings::Inner: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::test_namespace_bindings::Inner: Drop);
};
