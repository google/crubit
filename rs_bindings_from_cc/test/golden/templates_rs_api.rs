// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:templates_cc

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
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

// Error while generating bindings for constructor 'DifferentScope::DifferentScope':
// Can't generate bindings for DifferentScope::DifferentScope, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:templates_cc needs [//features:experimental] for DifferentScope::DifferentScope (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for constructor 'DifferentScope::DifferentScope':
// Can't generate bindings for DifferentScope::DifferentScope, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:templates_cc needs [//features:experimental] for DifferentScope::DifferentScope (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'DifferentScope::operator=':
// Can't generate bindings for DifferentScope::operator=, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:templates_cc needs [//features:experimental] for DifferentScope::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:templates_cc needs [//features:experimental] for DifferentScope::operator= (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'DifferentScope::operator=':
// Can't generate bindings for DifferentScope::operator=, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:templates_cc needs [//features:experimental] for DifferentScope::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:templates_cc needs [//features:experimental] for DifferentScope::operator= (the type of __param_0 (parameter #1): references are not supported)

pub mod test_namespace_bindings {
    // Error while generating bindings for class 'test_namespace_bindings::MyTemplate':
    // Class templates are not supported yet

    // Error while generating bindings for type alias 'test_namespace_bindings::MyTypeAlias':
    // Can't generate bindings for test_namespace_bindings::MyTypeAlias, because of missing required features (crubit.rs-features):
    // //rs_bindings_from_cc/test/golden:templates_cc needs [//features:wrapper] for test_namespace_bindings::MyTypeAlias (error: Can't generate bindings for test_namespace_bindings::MyTemplate<int>, because of missing required features (crubit.rs-features):
    // //rs_bindings_from_cc/test/golden:templates_cc needs [//features:wrapper] for test_namespace_bindings::MyTemplate<int> (crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE is a template instantiation)
    // //rs_bindings_from_cc/test/golden:templates_cc needs [//features:wrapper] for test_namespace_bindings::MyTemplate<int> (crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE is a template instantiation))

    // Error while generating bindings for type alias 'test_namespace_bindings::OtherTypeAliasInSameTarget':
    // Can't generate bindings for test_namespace_bindings::OtherTypeAliasInSameTarget, because of missing required features (crubit.rs-features):
    // //rs_bindings_from_cc/test/golden:templates_cc needs [//features:wrapper] for test_namespace_bindings::OtherTypeAliasInSameTarget (error: Can't generate bindings for test_namespace_bindings::MyTemplate<int>, because of missing required features (crubit.rs-features):
    // //rs_bindings_from_cc/test/golden:templates_cc needs [//features:wrapper] for test_namespace_bindings::MyTemplate<int> (crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE is a template instantiation)
    // //rs_bindings_from_cc/test/golden:templates_cc needs [//features:wrapper] for test_namespace_bindings::MyTemplate<int> (crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE is a template instantiation))

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

    // Error while generating bindings for constructor 'test_namespace_bindings::TemplateParam::TemplateParam':
    // Can't generate bindings for test_namespace_bindings::TemplateParam::TemplateParam, because of missing required features (crubit.rs-features):
    // //rs_bindings_from_cc/test/golden:templates_cc needs [//features:experimental] for test_namespace_bindings::TemplateParam::TemplateParam (the type of __param_0 (parameter #1): references are not supported)

    // Error while generating bindings for constructor 'test_namespace_bindings::TemplateParam::TemplateParam':
    // Can't generate bindings for test_namespace_bindings::TemplateParam::TemplateParam, because of missing required features (crubit.rs-features):
    // //rs_bindings_from_cc/test/golden:templates_cc needs [//features:experimental] for test_namespace_bindings::TemplateParam::TemplateParam (the type of __param_0 (parameter #1): references are not supported)

    // Error while generating bindings for function 'test_namespace_bindings::TemplateParam::operator=':
    // Can't generate bindings for test_namespace_bindings::TemplateParam::operator=, because of missing required features (crubit.rs-features):
    // //rs_bindings_from_cc/test/golden:templates_cc needs [//features:experimental] for test_namespace_bindings::TemplateParam::operator= (return type: references are not supported)
    // //rs_bindings_from_cc/test/golden:templates_cc needs [//features:experimental] for test_namespace_bindings::TemplateParam::operator= (the type of __param_0 (parameter #1): references are not supported)

    // Error while generating bindings for function 'test_namespace_bindings::TemplateParam::operator=':
    // Can't generate bindings for test_namespace_bindings::TemplateParam::operator=, because of missing required features (crubit.rs-features):
    // //rs_bindings_from_cc/test/golden:templates_cc needs [//features:experimental] for test_namespace_bindings::TemplateParam::operator= (return type: references are not supported)
    // //rs_bindings_from_cc/test/golden:templates_cc needs [//features:experimental] for test_namespace_bindings::TemplateParam::operator= (the type of __param_0 (parameter #1): references are not supported)

    // Error while generating bindings for type alias 'test_namespace_bindings::TemplateWithStructTemplateParam':
    // Can't generate bindings for test_namespace_bindings::TemplateWithStructTemplateParam, because of missing required features (crubit.rs-features):
    // //rs_bindings_from_cc/test/golden:templates_cc needs [//features:wrapper] for test_namespace_bindings::TemplateWithStructTemplateParam (error: Can't generate bindings for test_namespace_bindings::MyTemplate<test_namespace_bindings::TemplateParam>, because of missing required features (crubit.rs-features):
    // //rs_bindings_from_cc/test/golden:templates_cc needs [//features:wrapper] for test_namespace_bindings::MyTemplate<test_namespace_bindings::TemplateParam> (crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEE is a template instantiation)
    // //rs_bindings_from_cc/test/golden:templates_cc needs [//features:wrapper] for test_namespace_bindings::MyTemplate<test_namespace_bindings::TemplateParam> (crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEE is a template instantiation))

    // Error while generating bindings for type alias 'test_namespace_bindings::ParamFromDifferentScope':
    // Can't generate bindings for test_namespace_bindings::ParamFromDifferentScope, because of missing required features (crubit.rs-features):
    // //rs_bindings_from_cc/test/golden:templates_cc needs [//features:wrapper] for test_namespace_bindings::ParamFromDifferentScope (error: Can't generate bindings for test_namespace_bindings::MyTemplate<DifferentScope>, because of missing required features (crubit.rs-features):
    // //rs_bindings_from_cc/test/golden:templates_cc needs [//features:wrapper] for test_namespace_bindings::MyTemplate<DifferentScope> (crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateI14DifferentScopeEE is a template instantiation)
    // //rs_bindings_from_cc/test/golden:templates_cc needs [//features:wrapper] for test_namespace_bindings::MyTemplate<DifferentScope> (crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateI14DifferentScopeEE is a template instantiation))

    // Error while generating bindings for class 'test_namespace_bindings::TemplateWithTwoParams':
    // Class templates are not supported yet

    // Error while generating bindings for type alias 'test_namespace_bindings::AliasToTemplateWithTwoParams':
    // Can't generate bindings for test_namespace_bindings::AliasToTemplateWithTwoParams, because of missing required features (crubit.rs-features):
    // //rs_bindings_from_cc/test/golden:templates_cc needs [//features:wrapper] for test_namespace_bindings::AliasToTemplateWithTwoParams (error: Can't generate bindings for test_namespace_bindings::TemplateWithTwoParams<int, float>, because of missing required features (crubit.rs-features):
    // //rs_bindings_from_cc/test/golden:templates_cc needs [//features:wrapper] for test_namespace_bindings::TemplateWithTwoParams<int, float> (crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIifEE is a template instantiation)
    // //rs_bindings_from_cc/test/golden:templates_cc needs [//features:wrapper] for test_namespace_bindings::TemplateWithTwoParams<int, float> (crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIifEE is a template instantiation))

    // Error while generating bindings for type alias 'test_namespace_bindings::AliasToTemplateOfATemplate':
    // Can't generate bindings for test_namespace_bindings::AliasToTemplateOfATemplate, because of missing required features (crubit.rs-features):
    // //rs_bindings_from_cc/test/golden:templates_cc needs [//features:wrapper] for test_namespace_bindings::AliasToTemplateOfATemplate (error: Can't generate bindings for test_namespace_bindings::TemplateWithTwoParams<test_namespace_bindings::TemplateWithTwoParams<int, int>, int>, because of missing required features (crubit.rs-features):
    // //rs_bindings_from_cc/test/golden:templates_cc needs [//features:wrapper] for test_namespace_bindings::TemplateWithTwoParams<test_namespace_bindings::TemplateWithTwoParams<int, int>, int> (crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEE is a template instantiation)
    // //rs_bindings_from_cc/test/golden:templates_cc needs [//features:wrapper] for test_namespace_bindings::TemplateWithTwoParams<test_namespace_bindings::TemplateWithTwoParams<int, int>, int> (crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEE is a template instantiation))

    // Error while generating bindings for class 'test_namespace_bindings::MyStruct':
    // Class templates are not supported yet

    // Explicit class template specialization with definition should not be imported
    // unless also instantiated.

    // Explicit class template specialization with definition should be imported
    // even when not instantiated if there is a type alias for it.

    // Error while generating bindings for type alias 'test_namespace_bindings::MyCharStruct':
    // Can't generate bindings for test_namespace_bindings::MyCharStruct, because of missing required features (crubit.rs-features):
    // //rs_bindings_from_cc/test/golden:templates_cc needs [//features:wrapper] for test_namespace_bindings::MyCharStruct (error: Can't generate bindings for test_namespace_bindings::MyStruct<char>, because of missing required features (crubit.rs-features):
    // //rs_bindings_from_cc/test/golden:templates_cc needs [//features:wrapper] for test_namespace_bindings::MyStruct<char> (crate::__CcTemplateInstN23test_namespace_bindings8MyStructIcEE is a template instantiation)
    // //rs_bindings_from_cc/test/golden:templates_cc needs [//features:wrapper] for test_namespace_bindings::MyStruct<char> (crate::__CcTemplateInstN23test_namespace_bindings8MyStructIcEE is a template instantiation))

    // Forward declared explicit class template specialization should be imported
    // so the forward declaration code is generated (`forward_declare!`).

    // Explicit class template instantiation definition is imported similarly to
    // how implicit typedeffed instantiations are.

    // Explicit class template instantiation declaration is not handled (yet?)
    // TODO(b/245467707): Consider handling these as a build speed/ergonomic
    // optimization.
}

// namespace test_namespace_bindings

// Error while generating bindings for class 'MyTopLevelTemplate':
// Class templates are not supported yet

// Error while generating bindings for type alias 'TopLevelTemplateWithNonTopLevelParam':
// Can't generate bindings for TopLevelTemplateWithNonTopLevelParam, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:templates_cc needs [//features:wrapper] for TopLevelTemplateWithNonTopLevelParam (error: Can't generate bindings for MyTopLevelTemplate<test_namespace_bindings::TemplateParam>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:templates_cc needs [//features:wrapper] for MyTopLevelTemplate<test_namespace_bindings::TemplateParam> (crate::__CcTemplateInst18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEE is a template instantiation)
// //rs_bindings_from_cc/test/golden:templates_cc needs [//features:wrapper] for MyTopLevelTemplate<test_namespace_bindings::TemplateParam> (crate::__CcTemplateInst18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEE is a template instantiation))

// Error while generating bindings for function 'processForwardDeclaredSpecialization':
// Can't generate bindings for processForwardDeclaredSpecialization, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:templates_cc needs [//features:wrapper] for processForwardDeclaredSpecialization (the type of i (parameter #0): error: Can't generate bindings for MyTopLevelTemplate<int>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:templates_cc needs [//features:wrapper] for MyTopLevelTemplate<int> (incomplete type))

pub mod template_template_params { // Error while generating bindings for class 'template_template_params::Policy':
                                   // Class templates are not supported yet

    // Error while generating bindings for class 'template_template_params::MyTemplate':
    // Class templates are not supported yet

    // Error while generating bindings for type alias 'template_template_params::MyTypeAlias':
    // Can't generate bindings for template_template_params::MyTypeAlias, because of missing required features (crubit.rs-features):
    // //rs_bindings_from_cc/test/golden:templates_cc needs [//features:wrapper] for template_template_params::MyTypeAlias (error: Can't generate bindings for template_template_params::MyTemplate<template_template_params::Policy>, because of missing required features (crubit.rs-features):
    // //rs_bindings_from_cc/test/golden:templates_cc needs [//features:wrapper] for template_template_params::MyTemplate<template_template_params::Policy> (crate::__CcTemplateInstN24template_template_params10MyTemplateINS_6PolicyEEE is a template instantiation)
    // //rs_bindings_from_cc/test/golden:templates_cc needs [//features:wrapper] for template_template_params::MyTemplate<template_template_params::Policy> (crate::__CcTemplateInstN24template_template_params10MyTemplateINS_6PolicyEEE is a template instantiation))
}

// namespace template_template_params

pub mod forward_declared_template { // Error while generating bindings for class 'forward_declared_template::ForwardDeclaredTemplate':
                                    // Class templates are not supported yet

    // Error while generating bindings for type alias 'forward_declared_template::TypeAliasToForwardDeclaredTemplate':
    // Can't generate bindings for forward_declared_template::TypeAliasToForwardDeclaredTemplate, because of missing required features (crubit.rs-features):
    // //rs_bindings_from_cc/test/golden:templates_cc needs [//features:wrapper] for forward_declared_template::TypeAliasToForwardDeclaredTemplate (error: Can't generate bindings for forward_declared_template::ForwardDeclaredTemplate<int>, because of missing required features (crubit.rs-features):
    // //rs_bindings_from_cc/test/golden:templates_cc needs [//features:wrapper] for forward_declared_template::ForwardDeclaredTemplate<int> (incomplete type))
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

    // Error while generating bindings for constructor 'private_classes::HasPrivateType::HasPrivateType':
    // Can't generate bindings for private_classes::HasPrivateType::HasPrivateType, because of missing required features (crubit.rs-features):
    // //rs_bindings_from_cc/test/golden:templates_cc needs [//features:experimental] for private_classes::HasPrivateType::HasPrivateType (the type of __param_0 (parameter #1): references are not supported)

    // Error while generating bindings for constructor 'private_classes::HasPrivateType::HasPrivateType':
    // Can't generate bindings for private_classes::HasPrivateType::HasPrivateType, because of missing required features (crubit.rs-features):
    // //rs_bindings_from_cc/test/golden:templates_cc needs [//features:experimental] for private_classes::HasPrivateType::HasPrivateType (the type of __param_0 (parameter #1): references are not supported)

    // Error while generating bindings for function 'private_classes::HasPrivateType::operator=':
    // Can't generate bindings for private_classes::HasPrivateType::operator=, because of missing required features (crubit.rs-features):
    // //rs_bindings_from_cc/test/golden:templates_cc needs [//features:experimental] for private_classes::HasPrivateType::operator= (return type: references are not supported)
    // //rs_bindings_from_cc/test/golden:templates_cc needs [//features:experimental] for private_classes::HasPrivateType::operator= (the type of __param_0 (parameter #1): references are not supported)

    // Error while generating bindings for function 'private_classes::HasPrivateType::operator=':
    // Can't generate bindings for private_classes::HasPrivateType::operator=, because of missing required features (crubit.rs-features):
    // //rs_bindings_from_cc/test/golden:templates_cc needs [//features:experimental] for private_classes::HasPrivateType::operator= (return type: references are not supported)
    // //rs_bindings_from_cc/test/golden:templates_cc needs [//features:experimental] for private_classes::HasPrivateType::operator= (the type of __param_0 (parameter #1): references are not supported)
}

// namespace private_classes

// Error while generating bindings for class 'test_namespace_bindings::MyTemplate<DifferentScope>':
// Can't generate bindings for test_namespace_bindings::MyTemplate<DifferentScope>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:templates_cc needs [//features:wrapper] for test_namespace_bindings::MyTemplate<DifferentScope> (crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateI14DifferentScopeEE is a template instantiation)
// //rs_bindings_from_cc/test/golden:templates_cc needs [//features:wrapper] for test_namespace_bindings::MyTemplate<DifferentScope> (crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateI14DifferentScopeEE is a template instantiation)

// Error while generating bindings for class 'test_namespace_bindings::MyTemplate<test_namespace_bindings::TemplateParam>':
// Can't generate bindings for test_namespace_bindings::MyTemplate<test_namespace_bindings::TemplateParam>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:templates_cc needs [//features:wrapper] for test_namespace_bindings::MyTemplate<test_namespace_bindings::TemplateParam> (crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEE is a template instantiation)
// //rs_bindings_from_cc/test/golden:templates_cc needs [//features:wrapper] for test_namespace_bindings::MyTemplate<test_namespace_bindings::TemplateParam> (crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEE is a template instantiation)

// Error while generating bindings for class 'test_namespace_bindings::MyTemplate<int>':
// Can't generate bindings for test_namespace_bindings::MyTemplate<int>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:templates_cc needs [//features:wrapper] for test_namespace_bindings::MyTemplate<int> (crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE is a template instantiation)
// //rs_bindings_from_cc/test/golden:templates_cc needs [//features:wrapper] for test_namespace_bindings::MyTemplate<int> (crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE is a template instantiation)

// Error while generating bindings for struct 'test_namespace_bindings::TemplateWithTwoParams<test_namespace_bindings::TemplateWithTwoParams<int, int>, int>':
// Can't generate bindings for test_namespace_bindings::TemplateWithTwoParams<test_namespace_bindings::TemplateWithTwoParams<int, int>, int>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:templates_cc needs [//features:wrapper] for test_namespace_bindings::TemplateWithTwoParams<test_namespace_bindings::TemplateWithTwoParams<int, int>, int> (crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEE is a template instantiation)
// //rs_bindings_from_cc/test/golden:templates_cc needs [//features:wrapper] for test_namespace_bindings::TemplateWithTwoParams<test_namespace_bindings::TemplateWithTwoParams<int, int>, int> (crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEE is a template instantiation)

// Error while generating bindings for struct 'test_namespace_bindings::TemplateWithTwoParams<int, float>':
// Can't generate bindings for test_namespace_bindings::TemplateWithTwoParams<int, float>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:templates_cc needs [//features:wrapper] for test_namespace_bindings::TemplateWithTwoParams<int, float> (crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIifEE is a template instantiation)
// //rs_bindings_from_cc/test/golden:templates_cc needs [//features:wrapper] for test_namespace_bindings::TemplateWithTwoParams<int, float> (crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIifEE is a template instantiation)

// Error while generating bindings for struct 'test_namespace_bindings::TemplateWithTwoParams<int, int>':
// Can't generate bindings for test_namespace_bindings::TemplateWithTwoParams<int, int>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:templates_cc needs [//features:wrapper] for test_namespace_bindings::TemplateWithTwoParams<int, int> (crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIiiEE is a template instantiation)
// //rs_bindings_from_cc/test/golden:templates_cc needs [//features:wrapper] for test_namespace_bindings::TemplateWithTwoParams<int, int> (crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIiiEE is a template instantiation)

// Error while generating bindings for struct 'test_namespace_bindings::MyStruct<char>':
// Can't generate bindings for test_namespace_bindings::MyStruct<char>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:templates_cc needs [//features:wrapper] for test_namespace_bindings::MyStruct<char> (crate::__CcTemplateInstN23test_namespace_bindings8MyStructIcEE is a template instantiation)
// //rs_bindings_from_cc/test/golden:templates_cc needs [//features:wrapper] for test_namespace_bindings::MyStruct<char> (crate::__CcTemplateInstN23test_namespace_bindings8MyStructIcEE is a template instantiation)

// Error while generating bindings for struct 'MyTopLevelTemplate<test_namespace_bindings::TemplateParam>':
// Can't generate bindings for MyTopLevelTemplate<test_namespace_bindings::TemplateParam>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:templates_cc needs [//features:wrapper] for MyTopLevelTemplate<test_namespace_bindings::TemplateParam> (crate::__CcTemplateInst18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEE is a template instantiation)
// //rs_bindings_from_cc/test/golden:templates_cc needs [//features:wrapper] for MyTopLevelTemplate<test_namespace_bindings::TemplateParam> (crate::__CcTemplateInst18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEE is a template instantiation)

// Error while generating bindings for struct 'MyTopLevelTemplate<int>':
// Can't generate bindings for MyTopLevelTemplate<int>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:templates_cc needs [//features:wrapper] for MyTopLevelTemplate<int> (incomplete type)

// Error while generating bindings for class 'template_template_params::MyTemplate<template_template_params::Policy>':
// Can't generate bindings for template_template_params::MyTemplate<template_template_params::Policy>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:templates_cc needs [//features:wrapper] for template_template_params::MyTemplate<template_template_params::Policy> (crate::__CcTemplateInstN24template_template_params10MyTemplateINS_6PolicyEEE is a template instantiation)
// //rs_bindings_from_cc/test/golden:templates_cc needs [//features:wrapper] for template_template_params::MyTemplate<template_template_params::Policy> (crate::__CcTemplateInstN24template_template_params10MyTemplateINS_6PolicyEEE is a template instantiation)

// Error while generating bindings for class 'forward_declared_template::ForwardDeclaredTemplate<int>':
// Can't generate bindings for forward_declared_template::ForwardDeclaredTemplate<int>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:templates_cc needs [//features:wrapper] for forward_declared_template::ForwardDeclaredTemplate<int> (incomplete type)

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
