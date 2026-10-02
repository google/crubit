// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:templates_cc

#![rustfmt::skip]
#![feature(cfi_encoding, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![allow(deprecated)]
#![allow(unknown_lints, suspicious_runtime_symbol_definitions)]
#![deny(warnings)]
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "14DifferentScope"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=DifferentScope
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct DifferentScope {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for DifferentScope {}
impl !Sync for DifferentScope {}
unsafe impl ::cxx::ExternType for DifferentScope {
    type Id = ::cxx::type_id!("DifferentScope");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(forward_declare::symbol!("DifferentScope"), crate::DifferentScope);

impl Default for DifferentScope {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN14DifferentScopeC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

pub mod test_namespace_bindings {
    // error: class `test_namespace_bindings::MyTemplate` could not be bound
    //   Class templates are not yet supported

    // error: type alias `test_namespace_bindings::MyTypeAlias` could not be bound
    //   template instantiation is not yet supported

    // error: type alias `test_namespace_bindings::OtherTypeAliasInSameTarget` could not be bound
    //   template instantiation is not yet supported

    #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
    #[cfi_encoding = "N23test_namespace_bindings13TemplateParamE"]
    #[repr(C)]
    ///CRUBIT_ANNOTATE: cpp_type=test_namespace_bindings :: TemplateParam
    ///CRUBIT_ANNOTATE: cpp_move_constructible=
    pub struct TemplateParam {
        __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
    }
    impl !Send for TemplateParam {}
    impl !Sync for TemplateParam {}
    unsafe impl ::cxx::ExternType for TemplateParam {
        type Id = ::cxx::type_id!("test_namespace_bindings :: TemplateParam");
        type Kind = ::cxx::kind::Trivial;
    }
    forward_declare::unsafe_define!(
        forward_declare::symbol!("test_namespace_bindings :: TemplateParam"),
        crate::test_namespace_bindings::TemplateParam
    );

    impl Default for TemplateParam {
        #[inline(always)]
        fn default() -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk___ZN23test_namespace_bindings13TemplateParamC1Ev(
                    &raw mut tmp as *mut _,
                );
                tmp.assume_init()
            }
        }
    }

    // error: type alias `test_namespace_bindings::TemplateWithStructTemplateParam` could not be bound
    //   template instantiation is not yet supported

    // error: type alias `test_namespace_bindings::ParamFromDifferentScope` could not be bound
    //   template instantiation is not yet supported

    // error: class `test_namespace_bindings::TemplateWithTwoParams` could not be bound
    //   Class templates are not yet supported

    // error: type alias `test_namespace_bindings::AliasToTemplateWithTwoParams` could not be bound
    //   template instantiation is not yet supported

    // error: type alias `test_namespace_bindings::AliasToTemplateOfATemplate` could not be bound
    //   template instantiation is not yet supported

    // error: class `test_namespace_bindings::MyStruct` could not be bound
    //   Class templates are not yet supported

    // Explicit class template specialization with definition should not be imported
    // unless also instantiated.

    // Explicit class template specialization with definition should be imported
    // even when not instantiated if there is a type alias for it.

    // error: type alias `test_namespace_bindings::MyCharStruct` could not be bound
    //   template instantiation is not yet supported

    // Forward declared explicit class template specialization should be imported
    // so the forward declaration code is generated (`forward_declare!`).
}

// namespace test_namespace_bindings

// error: class `MyTopLevelTemplate` could not be bound
//   Class templates are not yet supported

// error: type alias `TopLevelTemplateWithNonTopLevelParam` could not be bound
//   template instantiation is not yet supported

/// # Safety
///
/// The caller must ensure that the following unsafe arguments are not misused by the function:
/// * `i`: raw pointer
#[inline(always)]
pub unsafe fn processForwardDeclaredSpecialization(
    i: *mut crate::__CcTemplateInst18MyTopLevelTemplateIiE,
) {
    unsafe {
        crate::detail::__rust_thunk___Z36processForwardDeclaredSpecializationP18MyTopLevelTemplateIiE(i)
    }
}

pub mod template_template_params { // error: class `template_template_params::Policy` could not be bound
                                   //   Class templates are not yet supported

    // error: class `template_template_params::MyTemplate` could not be bound
    //   Class templates are not yet supported

    // error: type alias `template_template_params::MyTypeAlias` could not be bound
    //   template instantiation is not yet supported
}

// namespace template_template_params

pub mod forward_declared_template {
    // error: class `forward_declared_template::ForwardDeclaredTemplate` could not be bound
    //   Class templates are not yet supported

    pub type TypeAliasToForwardDeclaredTemplate =
        crate::__CcTemplateInstN25forward_declared_template23ForwardDeclaredTemplateIiEE;
}

// namespace forward_declared_template

pub mod private_classes {
    #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
    #[cfi_encoding = "N15private_classes14HasPrivateTypeE"]
    #[repr(C)]
    ///CRUBIT_ANNOTATE: cpp_type=private_classes :: HasPrivateType
    ///CRUBIT_ANNOTATE: cpp_move_constructible=
    pub struct HasPrivateType {
        __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
    }
    impl !Send for HasPrivateType {}
    impl !Sync for HasPrivateType {}
    unsafe impl ::cxx::ExternType for HasPrivateType {
        type Id = ::cxx::type_id!("private_classes :: HasPrivateType");
        type Kind = ::cxx::kind::Trivial;
    }
    forward_declare::unsafe_define!(
        forward_declare::symbol!("private_classes :: HasPrivateType"),
        crate::private_classes::HasPrivateType
    );
}

// namespace private_classes

// error: class `test_namespace_bindings::MyTemplate<struct DifferentScope>` could not be bound
//   template instantiation is not yet supported

// error: class `test_namespace_bindings::MyTemplate<struct test_namespace_bindings::TemplateParam>` could not be bound
//   template instantiation is not yet supported

// error: class `test_namespace_bindings::MyTemplate<int>` could not be bound
//   template instantiation is not yet supported

// error: struct `test_namespace_bindings::TemplateWithTwoParams<struct test_namespace_bindings::TemplateWithTwoParams<int, int>, int>` could not be bound
//   template instantiation is not yet supported

// error: struct `test_namespace_bindings::TemplateWithTwoParams<int, float>` could not be bound
//   template instantiation is not yet supported

// error: struct `test_namespace_bindings::TemplateWithTwoParams<int, int>` could not be bound
//   template instantiation is not yet supported

// error: struct `test_namespace_bindings::MyStruct<char>` could not be bound
//   template instantiation is not yet supported

// error: struct `MyTopLevelTemplate<struct test_namespace_bindings::TemplateParam>` could not be bound
//   template instantiation is not yet supported

forward_declare::forward_declare!(pub __CcTemplateInst18MyTopLevelTemplateIiE = forward_declare::symbol!("MyTopLevelTemplate < int >"));

// error: class `template_template_params::MyTemplate<template_template_params::Policy>` could not be bound
//   template instantiation is not yet supported

forward_declare::forward_declare!(pub __CcTemplateInstN25forward_declared_template23ForwardDeclaredTemplateIiEE = forward_declare::symbol!("forward_declared_template :: ForwardDeclaredTemplate < int >"));

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN14DifferentScopeC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN23test_namespace_bindings13TemplateParamC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        #[link_name = "_Z36processForwardDeclaredSpecializationP18MyTopLevelTemplateIiE"]
        pub(crate) unsafe fn __rust_thunk___Z36processForwardDeclaredSpecializationP18MyTopLevelTemplateIiE(
            i: *mut crate::__CcTemplateInst18MyTopLevelTemplateIiE,
        );
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::DifferentScope>() == 1);
    assert!(::core::mem::align_of::<crate::DifferentScope>() == 1);
    static_assertions::assert_impl_all!(crate::DifferentScope: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::DifferentScope: Drop);

    assert!(::core::mem::size_of::<crate::test_namespace_bindings::TemplateParam>() == 1);
    assert!(::core::mem::align_of::<crate::test_namespace_bindings::TemplateParam>() == 1);
    static_assertions::assert_impl_all!(crate::test_namespace_bindings::TemplateParam: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::test_namespace_bindings::TemplateParam: Drop);

    assert!(::core::mem::size_of::<crate::private_classes::HasPrivateType>() == 1);
    assert!(::core::mem::align_of::<crate::private_classes::HasPrivateType>() == 1);
    static_assertions::assert_impl_all!(crate::private_classes::HasPrivateType: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::private_classes::HasPrivateType: Drop);
};
