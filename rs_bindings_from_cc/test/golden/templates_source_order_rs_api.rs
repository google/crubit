// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:templates_source_order_cc

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![no_std]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

// Error while generating bindings for class 'MyTemplate':
// Class templates are not supported yet

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

// Error while generating bindings for type alias 'Alias1':
// Can't generate bindings for Alias1, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:templates_source_order_cc needs [//features:wrapper] for Alias1 (error: Can't generate bindings for MyTemplate<int>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:templates_source_order_cc needs [//features:wrapper] for MyTemplate<int> (crate::__CcTemplateInst10MyTemplateIiE is a template instantiation)
// //rs_bindings_from_cc/test/golden:templates_source_order_cc needs [//features:wrapper] for MyTemplate<int> (crate::__CcTemplateInst10MyTemplateIiE is a template instantiation))

// Error while generating bindings for type alias 'Alias2':
// Can't generate bindings for Alias2, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:templates_source_order_cc needs [//features:wrapper] for Alias2 (error: Can't generate bindings for MyTemplate<float>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:templates_source_order_cc needs [//features:wrapper] for MyTemplate<float> (crate::__CcTemplateInst10MyTemplateIfE is a template instantiation)
// //rs_bindings_from_cc/test/golden:templates_source_order_cc needs [//features:wrapper] for MyTemplate<float> (crate::__CcTemplateInst10MyTemplateIfE is a template instantiation))

// Error while generating bindings for type alias 'Alias3':
// Can't generate bindings for Alias3, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:templates_source_order_cc needs [//features:wrapper] for Alias3 (error: Can't generate bindings for MyTemplate<TopLevel>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:templates_source_order_cc needs [//features:wrapper] for MyTemplate<TopLevel> (crate::__CcTemplateInst10MyTemplateI8TopLevelE is a template instantiation)
// //rs_bindings_from_cc/test/golden:templates_source_order_cc needs [//features:wrapper] for MyTemplate<TopLevel> (crate::__CcTemplateInst10MyTemplateI8TopLevelE is a template instantiation))

// Error while generating bindings for type alias 'Alias4':
// Can't generate bindings for Alias4, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:templates_source_order_cc needs [//features:wrapper] for Alias4 (error: Can't generate bindings for MyTemplate<double>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:templates_source_order_cc needs [//features:wrapper] for MyTemplate<double> (crate::__CcTemplateInst10MyTemplateIdE is a template instantiation)
// //rs_bindings_from_cc/test/golden:templates_source_order_cc needs [//features:wrapper] for MyTemplate<double> (crate::__CcTemplateInst10MyTemplateIdE is a template instantiation))

// Error while generating bindings for type alias 'Alias5':
// Can't generate bindings for Alias5, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:templates_source_order_cc needs [//features:wrapper] for Alias5 (error: Can't generate bindings for MyTemplate<bool>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:templates_source_order_cc needs [//features:wrapper] for MyTemplate<bool> (crate::__CcTemplateInst10MyTemplateIbE is a template instantiation)
// //rs_bindings_from_cc/test/golden:templates_source_order_cc needs [//features:wrapper] for MyTemplate<bool> (crate::__CcTemplateInst10MyTemplateIbE is a template instantiation))

// Error while generating bindings for type alias 'Alias6':
// Can't generate bindings for Alias6, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:templates_source_order_cc needs [//features:wrapper] for Alias6 (error: Can't generate bindings for MyTemplate<MyTemplate<TopLevel>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:templates_source_order_cc needs [//features:wrapper] for MyTemplate<MyTemplate<TopLevel>> (crate::__CcTemplateInst10MyTemplateIS_I8TopLevelEE is a template instantiation)
// //rs_bindings_from_cc/test/golden:templates_source_order_cc needs [//features:wrapper] for MyTemplate<MyTemplate<TopLevel>> (crate::__CcTemplateInst10MyTemplateIS_I8TopLevelEE is a template instantiation))

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

    // Error while generating bindings for type alias 'test_namespace_bindings::Alias7':
    // Can't generate bindings for test_namespace_bindings::Alias7, because of missing required features (crubit.rs-features):
    // //rs_bindings_from_cc/test/golden:templates_source_order_cc needs [//features:wrapper] for test_namespace_bindings::Alias7 (error: Can't generate bindings for MyTemplate<char>, because of missing required features (crubit.rs-features):
    // //rs_bindings_from_cc/test/golden:templates_source_order_cc needs [//features:wrapper] for MyTemplate<char> (crate::__CcTemplateInst10MyTemplateIcE is a template instantiation)
    // //rs_bindings_from_cc/test/golden:templates_source_order_cc needs [//features:wrapper] for MyTemplate<char> (crate::__CcTemplateInst10MyTemplateIcE is a template instantiation))

    // Error while generating bindings for type alias 'test_namespace_bindings::Alias8':
    // Can't generate bindings for test_namespace_bindings::Alias8, because of missing required features (crubit.rs-features):
    // //rs_bindings_from_cc/test/golden:templates_source_order_cc needs [//features:wrapper] for test_namespace_bindings::Alias8 (error: Can't generate bindings for MyTemplate<test_namespace_bindings::Inner>, because of missing required features (crubit.rs-features):
    // //rs_bindings_from_cc/test/golden:templates_source_order_cc needs [//features:wrapper] for MyTemplate<test_namespace_bindings::Inner> (crate::__CcTemplateInst10MyTemplateIN23test_namespace_bindings5InnerEE is a template instantiation)
    // //rs_bindings_from_cc/test/golden:templates_source_order_cc needs [//features:wrapper] for MyTemplate<test_namespace_bindings::Inner> (crate::__CcTemplateInst10MyTemplateIN23test_namespace_bindings5InnerEE is a template instantiation))

    // Error while generating bindings for type alias 'test_namespace_bindings::Alias9':
    // Can't generate bindings for test_namespace_bindings::Alias9, because of missing required features (crubit.rs-features):
    // //rs_bindings_from_cc/test/golden:templates_source_order_cc needs [//features:wrapper] for test_namespace_bindings::Alias9 (error: Can't generate bindings for MyTemplate<MyTemplate<test_namespace_bindings::Inner>>, because of missing required features (crubit.rs-features):
    // //rs_bindings_from_cc/test/golden:templates_source_order_cc needs [//features:wrapper] for MyTemplate<MyTemplate<test_namespace_bindings::Inner>> (crate::__CcTemplateInst10MyTemplateIS_IN23test_namespace_bindings5InnerEEE is a template instantiation)
    // //rs_bindings_from_cc/test/golden:templates_source_order_cc needs [//features:wrapper] for MyTemplate<MyTemplate<test_namespace_bindings::Inner>> (crate::__CcTemplateInst10MyTemplateIS_IN23test_namespace_bindings5InnerEEE is a template instantiation))
}

// namespace test_namespace_bindings

// Error while generating bindings for class 'MyTemplate<TopLevel>':
// Can't generate bindings for MyTemplate<TopLevel>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:templates_source_order_cc needs [//features:wrapper] for MyTemplate<TopLevel> (crate::__CcTemplateInst10MyTemplateI8TopLevelE is a template instantiation)
// //rs_bindings_from_cc/test/golden:templates_source_order_cc needs [//features:wrapper] for MyTemplate<TopLevel> (crate::__CcTemplateInst10MyTemplateI8TopLevelE is a template instantiation)

// Error while generating bindings for class 'MyTemplate<test_namespace_bindings::Inner>':
// Can't generate bindings for MyTemplate<test_namespace_bindings::Inner>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:templates_source_order_cc needs [//features:wrapper] for MyTemplate<test_namespace_bindings::Inner> (crate::__CcTemplateInst10MyTemplateIN23test_namespace_bindings5InnerEE is a template instantiation)
// //rs_bindings_from_cc/test/golden:templates_source_order_cc needs [//features:wrapper] for MyTemplate<test_namespace_bindings::Inner> (crate::__CcTemplateInst10MyTemplateIN23test_namespace_bindings5InnerEE is a template instantiation)

// Error while generating bindings for class 'MyTemplate<MyTemplate<TopLevel>>':
// Can't generate bindings for MyTemplate<MyTemplate<TopLevel>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:templates_source_order_cc needs [//features:wrapper] for MyTemplate<MyTemplate<TopLevel>> (crate::__CcTemplateInst10MyTemplateIS_I8TopLevelEE is a template instantiation)
// //rs_bindings_from_cc/test/golden:templates_source_order_cc needs [//features:wrapper] for MyTemplate<MyTemplate<TopLevel>> (crate::__CcTemplateInst10MyTemplateIS_I8TopLevelEE is a template instantiation)

// Error while generating bindings for class 'MyTemplate<MyTemplate<test_namespace_bindings::Inner>>':
// Can't generate bindings for MyTemplate<MyTemplate<test_namespace_bindings::Inner>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:templates_source_order_cc needs [//features:wrapper] for MyTemplate<MyTemplate<test_namespace_bindings::Inner>> (crate::__CcTemplateInst10MyTemplateIS_IN23test_namespace_bindings5InnerEEE is a template instantiation)
// //rs_bindings_from_cc/test/golden:templates_source_order_cc needs [//features:wrapper] for MyTemplate<MyTemplate<test_namespace_bindings::Inner>> (crate::__CcTemplateInst10MyTemplateIS_IN23test_namespace_bindings5InnerEEE is a template instantiation)

// Error while generating bindings for class 'MyTemplate<bool>':
// Can't generate bindings for MyTemplate<bool>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:templates_source_order_cc needs [//features:wrapper] for MyTemplate<bool> (crate::__CcTemplateInst10MyTemplateIbE is a template instantiation)
// //rs_bindings_from_cc/test/golden:templates_source_order_cc needs [//features:wrapper] for MyTemplate<bool> (crate::__CcTemplateInst10MyTemplateIbE is a template instantiation)

// Error while generating bindings for class 'MyTemplate<char>':
// Can't generate bindings for MyTemplate<char>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:templates_source_order_cc needs [//features:wrapper] for MyTemplate<char> (crate::__CcTemplateInst10MyTemplateIcE is a template instantiation)
// //rs_bindings_from_cc/test/golden:templates_source_order_cc needs [//features:wrapper] for MyTemplate<char> (crate::__CcTemplateInst10MyTemplateIcE is a template instantiation)

// Error while generating bindings for class 'MyTemplate<double>':
// Can't generate bindings for MyTemplate<double>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:templates_source_order_cc needs [//features:wrapper] for MyTemplate<double> (crate::__CcTemplateInst10MyTemplateIdE is a template instantiation)
// //rs_bindings_from_cc/test/golden:templates_source_order_cc needs [//features:wrapper] for MyTemplate<double> (crate::__CcTemplateInst10MyTemplateIdE is a template instantiation)

// Error while generating bindings for class 'MyTemplate<float>':
// Can't generate bindings for MyTemplate<float>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:templates_source_order_cc needs [//features:wrapper] for MyTemplate<float> (crate::__CcTemplateInst10MyTemplateIfE is a template instantiation)
// //rs_bindings_from_cc/test/golden:templates_source_order_cc needs [//features:wrapper] for MyTemplate<float> (crate::__CcTemplateInst10MyTemplateIfE is a template instantiation)

// Error while generating bindings for class 'MyTemplate<int>':
// Can't generate bindings for MyTemplate<int>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:templates_source_order_cc needs [//features:wrapper] for MyTemplate<int> (crate::__CcTemplateInst10MyTemplateIiE is a template instantiation)
// //rs_bindings_from_cc/test/golden:templates_source_order_cc needs [//features:wrapper] for MyTemplate<int> (crate::__CcTemplateInst10MyTemplateIiE is a template instantiation)

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
