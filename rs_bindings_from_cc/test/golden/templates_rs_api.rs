// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:templates_cc

#![rustfmt::skip]
#![feature(custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![deny(warnings)]

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=DifferentScope
pub struct DifferentScope {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for DifferentScope {}
impl !Sync for DifferentScope {}
unsafe impl ::cxx::ExternType for DifferentScope {
    type Id = ::cxx::type_id!("DifferentScope");
    type Kind = ::cxx::kind::Trivial;
}

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
    //   template instantiation is not yet supported

    // error: type alias `test_namespace_bindings::OtherTypeAliasInSameTarget` could not be bound
    //   template instantiation is not yet supported
    //   template instantiation is not yet supported

    #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
    #[repr(C)]
    ///CRUBIT_ANNOTATE: cpp_type=test_namespace_bindings :: TemplateParam
    pub struct TemplateParam {
        __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
    }
    impl !Send for TemplateParam {}
    impl !Sync for TemplateParam {}
    unsafe impl ::cxx::ExternType for TemplateParam {
        type Id = ::cxx::type_id!("test_namespace_bindings :: TemplateParam");
        type Kind = ::cxx::kind::Trivial;
    }

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
    //   template instantiation is not yet supported

    // error: type alias `test_namespace_bindings::ParamFromDifferentScope` could not be bound
    //   template instantiation is not yet supported
    //   template instantiation is not yet supported

    // error: class `test_namespace_bindings::TemplateWithTwoParams` could not be bound
    //   Class templates are not yet supported

    // error: type alias `test_namespace_bindings::AliasToTemplateWithTwoParams` could not be bound
    //   template instantiation is not yet supported
    //   template instantiation is not yet supported

    // error: type alias `test_namespace_bindings::AliasToTemplateOfATemplate` could not be bound
    //   template instantiation is not yet supported
    //   template instantiation is not yet supported

    // error: class `test_namespace_bindings::MyStruct` could not be bound
    //   Class templates are not yet supported

    // Explicit class template specialization with definition should not be imported
    // unless also instantiated.

    // Explicit class template specialization with definition should be imported
    // even when not instantiated if there is a type alias for it.

    // error: type alias `test_namespace_bindings::MyCharStruct` could not be bound
    //   template instantiation is not yet supported
    //   template instantiation is not yet supported

    // Forward declared explicit class template specialization should be imported
    // so the forward declaration code is generated (`forward_declare!`).

    // Explicit class template instantiation definition is imported similarly to
    // how implicit typedeffed instantiations are.

    // Explicit class template instantiation declaration is not handled (yet?)
    // TODO(b/245467707): Consider handling these as a build speed/ergonomic
    // optimization.
}

// namespace test_namespace_bindings

// error: class `MyTopLevelTemplate` could not be bound
//   Class templates are not yet supported

// error: type alias `TopLevelTemplateWithNonTopLevelParam` could not be bound
//   template instantiation is not yet supported
//   template instantiation is not yet supported

// error: function `processForwardDeclaredSpecialization` could not be bound
//   Unsupported parameter #0 (i): incomplete type

pub mod template_template_params { // error: class `template_template_params::Policy` could not be bound
                                   //   Class templates are not yet supported

    // error: class `template_template_params::MyTemplate` could not be bound
    //   Class templates are not yet supported

    // error: type alias `template_template_params::MyTypeAlias` could not be bound
    //   template instantiation is not yet supported
    //   template instantiation is not yet supported
}

// namespace template_template_params

pub mod forward_declared_template { // error: class `forward_declared_template::ForwardDeclaredTemplate` could not be bound
                                    //   Class templates are not yet supported

    // error: type alias `forward_declared_template::TypeAliasToForwardDeclaredTemplate` could not be bound
    //   incomplete type
}

// namespace forward_declared_template

pub mod private_classes {
    #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
    #[repr(C)]
    ///CRUBIT_ANNOTATE: cpp_type=private_classes :: HasPrivateType
    pub struct HasPrivateType {
        __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
    }
    impl !Send for HasPrivateType {}
    impl !Sync for HasPrivateType {}
    unsafe impl ::cxx::ExternType for HasPrivateType {
        type Id = ::cxx::type_id!("private_classes :: HasPrivateType");
        type Kind = ::cxx::kind::Trivial;
    }
}

// namespace private_classes

// error: class `test_namespace_bindings::MyTemplate<DifferentScope>` could not be bound
//   template instantiation is not yet supported
//   template instantiation is not yet supported

// error: class `test_namespace_bindings::MyTemplate<test_namespace_bindings::TemplateParam>` could not be bound
//   template instantiation is not yet supported
//   template instantiation is not yet supported

// error: class `test_namespace_bindings::MyTemplate<int>` could not be bound
//   template instantiation is not yet supported
//   template instantiation is not yet supported

// error: struct `test_namespace_bindings::TemplateWithTwoParams<test_namespace_bindings::TemplateWithTwoParams<int, int>, int>` could not be bound
//   template instantiation is not yet supported
//   template instantiation is not yet supported

// error: struct `test_namespace_bindings::TemplateWithTwoParams<int, float>` could not be bound
//   template instantiation is not yet supported
//   template instantiation is not yet supported

// error: struct `test_namespace_bindings::TemplateWithTwoParams<int, int>` could not be bound
//   template instantiation is not yet supported
//   template instantiation is not yet supported

// error: struct `test_namespace_bindings::MyStruct<char>` could not be bound
//   template instantiation is not yet supported
//   template instantiation is not yet supported

// error: struct `MyTopLevelTemplate<test_namespace_bindings::TemplateParam>` could not be bound
//   template instantiation is not yet supported
//   template instantiation is not yet supported

// error: struct `MyTopLevelTemplate<int>` could not be bound
//   incomplete type

// error: class `template_template_params::MyTemplate<template_template_params::Policy>` could not be bound
//   template instantiation is not yet supported
//   template instantiation is not yet supported

// error: class `forward_declared_template::ForwardDeclaredTemplate<int>` could not be bound
//   incomplete type

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
