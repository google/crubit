// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:templates_cc

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
forward_declare::unsafe_define!(forward_declare::symbol!("DifferentScope"), crate::DifferentScope);

impl Default for DifferentScope {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN14DifferentScopeC1Ev(
                &raw mut tmp as *mut ::core::ffi::c_void,
            );
            tmp.assume_init()
        }
    }
}

pub mod test_namespace_bindings {
    // Error while generating bindings for class 'test_namespace_bindings::MyTemplate':
    // Class templates are not supported yet

    pub type MyTypeAlias = crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE;

    pub type OtherTypeAliasInSameTarget =
        crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE;

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
                    &raw mut tmp as *mut ::core::ffi::c_void,
                );
                tmp.assume_init()
            }
        }
    }

    pub type TemplateWithStructTemplateParam =
        crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEE;

    pub type ParamFromDifferentScope =
        crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateI14DifferentScopeEE;

    // Error while generating bindings for class 'test_namespace_bindings::TemplateWithTwoParams':
    // Class templates are not supported yet

    pub type AliasToTemplateWithTwoParams =
        crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIifEE;

    pub type AliasToTemplateOfATemplate =
        crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEE;

    // Error while generating bindings for class 'test_namespace_bindings::MyStruct':
    // Class templates are not supported yet

    // Explicit class template specialization with definition should not be imported
    // unless also instantiated.

    // Explicit class template specialization with definition should be imported
    // even when not instantiated if there is a type alias for it.

    pub type MyCharStruct = crate::__CcTemplateInstN23test_namespace_bindings8MyStructIcEE;

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

pub type TopLevelTemplateWithNonTopLevelParam =
    crate::__CcTemplateInst18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEE;

#[inline(always)]
pub fn processForwardDeclaredSpecialization<'a>(
    i: Option<::core::pin::Pin<&'a mut crate::__CcTemplateInst18MyTopLevelTemplateIiE>>,
) {
    unsafe {
        crate::detail::__rust_thunk___Z36processForwardDeclaredSpecializationP18MyTopLevelTemplateIiE(i)
    }
}

pub mod template_template_params {
    // Error while generating bindings for class 'template_template_params::Policy':
    // Class templates are not supported yet

    // Error while generating bindings for class 'template_template_params::MyTemplate':
    // Class templates are not supported yet

    pub type MyTypeAlias =
        crate::__CcTemplateInstN24template_template_params10MyTemplateINS_6PolicyEEE;
}

// namespace template_template_params

pub mod forward_declared_template {
    // Error while generating bindings for class 'forward_declared_template::ForwardDeclaredTemplate':
    // Class templates are not supported yet

    pub type TypeAliasToForwardDeclaredTemplate =
        crate::__CcTemplateInstN25forward_declared_template23ForwardDeclaredTemplateIiEE;
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
    forward_declare::unsafe_define!(
        forward_declare::symbol!("private_classes :: HasPrivateType"),
        crate::private_classes::HasPrivateType
    );
}

// namespace private_classes

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=test_namespace_bindings :: MyTemplate < DifferentScope >
pub struct __CcTemplateInstN23test_namespace_bindings10MyTemplateI14DifferentScopeEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) value_: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for __CcTemplateInstN23test_namespace_bindings10MyTemplateI14DifferentScopeEE {}
impl !Sync for __CcTemplateInstN23test_namespace_bindings10MyTemplateI14DifferentScopeEE {}
forward_declare::unsafe_define!(
    forward_declare::symbol!("test_namespace_bindings :: MyTemplate < DifferentScope >"),
    crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateI14DifferentScopeEE
);

impl Default for __CcTemplateInstN23test_namespace_bindings10MyTemplateI14DifferentScopeEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN23test_namespace_bindings10MyTemplateI14DifferentScopeEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(&raw mut tmp as*mut::core::ffi::c_void);
            tmp.assume_init()
        }
    }
}

impl __CcTemplateInstN23test_namespace_bindings10MyTemplateI14DifferentScopeEE {
    #[inline(always)]
    pub fn Create(
        mut value: crate::DifferentScope,
    ) -> crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateI14DifferentScopeEE {
        unsafe {
            let mut __return = ::core::mem::MaybeUninit::<Self>::uninit();
            crate::detail::__rust_thunk___ZN23test_namespace_bindings10MyTemplateI14DifferentScopeE6CreateES1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(&raw mut __return as*mut::core::ffi::c_void,&mut value);
            __return.assume_init()
        }
    }
}

impl __CcTemplateInstN23test_namespace_bindings10MyTemplateI14DifferentScopeEE {
    #[inline(always)]
    pub fn value<'a>(&'a self) -> &'a crate::DifferentScope {
        unsafe {
            crate::detail::__rust_thunk___ZNK23test_namespace_bindings10MyTemplateI14DifferentScopeE5valueEv__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(self)
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=test_namespace_bindings :: MyTemplate < test_namespace_bindings :: TemplateParam >
pub struct __CcTemplateInstN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) value_: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for __CcTemplateInstN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEE {}
impl !Sync for __CcTemplateInstN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEE {}
forward_declare::unsafe_define!(
    forward_declare::symbol!(
        "test_namespace_bindings :: MyTemplate < test_namespace_bindings :: TemplateParam >"
    ),
    crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEE
);

impl Default for __CcTemplateInstN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(&raw mut tmp as*mut::core::ffi::c_void);
            tmp.assume_init()
        }
    }
}

impl __CcTemplateInstN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEE {
    #[inline(always)]
    pub fn Create(
        mut value: crate::test_namespace_bindings::TemplateParam,
    ) -> crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEE {
        unsafe {
            let mut __return = ::core::mem::MaybeUninit::<Self>::uninit();
            crate::detail::__rust_thunk___ZN23test_namespace_bindings10MyTemplateINS_13TemplateParamEE6CreateES1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(&raw mut __return as*mut::core::ffi::c_void,&mut value);
            __return.assume_init()
        }
    }
}

impl __CcTemplateInstN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEE {
    #[inline(always)]
    pub fn value<'a>(&'a self) -> &'a crate::test_namespace_bindings::TemplateParam {
        unsafe {
            crate::detail::__rust_thunk___ZNK23test_namespace_bindings10MyTemplateINS_13TemplateParamEE5valueEv__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(self)
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C, align(4))]
///CRUBIT_ANNOTATE: cpp_type=test_namespace_bindings :: MyTemplate < int >
pub struct __CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) value_: [::core::mem::MaybeUninit<u8>; 4],
}
impl !Send for __CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE {}
impl !Sync for __CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE {}
forward_declare::unsafe_define!(
    forward_declare::symbol!("test_namespace_bindings :: MyTemplate < int >"),
    crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE
);

impl Default for __CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN23test_namespace_bindings10MyTemplateIiEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(&raw mut tmp as*mut::core::ffi::c_void);
            tmp.assume_init()
        }
    }
}

impl __CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE {
    #[inline(always)]
    pub fn Create(
        value: ::core::ffi::c_int,
    ) -> crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE {
        unsafe {
            let mut __return = ::core::mem::MaybeUninit::<Self>::uninit();
            crate::detail::__rust_thunk___ZN23test_namespace_bindings10MyTemplateIiE6CreateEi__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(&raw mut __return as*mut::core::ffi::c_void,value);
            __return.assume_init()
        }
    }
}

impl __CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE {
    #[inline(always)]
    pub fn value<'a>(&'a self) -> &'a ::core::ffi::c_int {
        unsafe {
            crate::detail::__rust_thunk___ZNK23test_namespace_bindings10MyTemplateIiE5valueEv__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(self)
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=test_namespace_bindings :: TemplateWithTwoParams < test_namespace_bindings :: TemplateWithTwoParams < int , int >, int >
pub struct __CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEE {
    pub value1: crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIiiEE,
    pub value2: ::core::ffi::c_int,
}
impl !Send for __CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEE {}
impl !Sync for __CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEE {}
forward_declare::unsafe_define!(forward_declare::symbol!("test_namespace_bindings :: TemplateWithTwoParams < test_namespace_bindings :: TemplateWithTwoParams < int , int >, int >"),crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEE);

impl Default for __CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(&raw mut tmp as*mut::core::ffi::c_void);
            tmp.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=test_namespace_bindings :: TemplateWithTwoParams < int , float >
pub struct __CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIifEE {
    pub value1: ::core::ffi::c_int,
    pub value2: f32,
}
impl !Send for __CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIifEE {}
impl !Sync for __CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIifEE {}
forward_declare::unsafe_define!(
    forward_declare::symbol!("test_namespace_bindings :: TemplateWithTwoParams < int , float >"),
    crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIifEE
);

impl Default for __CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIifEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN23test_namespace_bindings21TemplateWithTwoParamsIifEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(&raw mut tmp as*mut::core::ffi::c_void);
            tmp.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=test_namespace_bindings :: TemplateWithTwoParams < int , int >
pub struct __CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIiiEE {
    pub value1: ::core::ffi::c_int,
    pub value2: ::core::ffi::c_int,
}
impl !Send for __CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIiiEE {}
impl !Sync for __CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIiiEE {}
forward_declare::unsafe_define!(
    forward_declare::symbol!("test_namespace_bindings :: TemplateWithTwoParams < int , int >"),
    crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIiiEE
);

impl Default for __CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIiiEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN23test_namespace_bindings21TemplateWithTwoParamsIiiEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(&raw mut tmp as*mut::core::ffi::c_void);
            tmp.assume_init()
        }
    }
}

/// Explicit class template specialization with definition should be imported
/// even when not instantiated if there is a type alias for it.
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=test_namespace_bindings :: MyStruct < char >
pub struct __CcTemplateInstN23test_namespace_bindings8MyStructIcEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for __CcTemplateInstN23test_namespace_bindings8MyStructIcEE {}
impl !Sync for __CcTemplateInstN23test_namespace_bindings8MyStructIcEE {}
forward_declare::unsafe_define!(
    forward_declare::symbol!("test_namespace_bindings :: MyStruct < char >"),
    crate::__CcTemplateInstN23test_namespace_bindings8MyStructIcEE
);

impl Default for __CcTemplateInstN23test_namespace_bindings8MyStructIcEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN23test_namespace_bindings8MyStructIcEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(&raw mut tmp as*mut::core::ffi::c_void);
            tmp.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=MyTopLevelTemplate < test_namespace_bindings :: TemplateParam >
pub struct __CcTemplateInst18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEE {
    pub value: crate::test_namespace_bindings::TemplateParam,
}
impl !Send for __CcTemplateInst18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEE {}
impl !Sync for __CcTemplateInst18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEE {}
forward_declare::unsafe_define!(
    forward_declare::symbol!("MyTopLevelTemplate < test_namespace_bindings :: TemplateParam >"),
    crate::__CcTemplateInst18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEE
);

impl Default for __CcTemplateInst18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(&raw mut tmp as*mut::core::ffi::c_void);
            tmp.assume_init()
        }
    }
}

forward_declare::forward_declare!(pub __CcTemplateInst18MyTopLevelTemplateIiE = forward_declare::symbol!("MyTopLevelTemplate < int >"));

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=template_template_params :: MyTemplate < template_template_params :: Policy >
pub struct __CcTemplateInstN24template_template_params10MyTemplateINS_6PolicyEEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for __CcTemplateInstN24template_template_params10MyTemplateINS_6PolicyEEE {}
impl !Sync for __CcTemplateInstN24template_template_params10MyTemplateINS_6PolicyEEE {}
forward_declare::unsafe_define!(
    forward_declare::symbol!(
        "template_template_params :: MyTemplate < template_template_params :: Policy >"
    ),
    crate::__CcTemplateInstN24template_template_params10MyTemplateINS_6PolicyEEE
);

impl Default for __CcTemplateInstN24template_template_params10MyTemplateINS_6PolicyEEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN24template_template_params10MyTemplateINS_6PolicyEEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(&raw mut tmp as*mut::core::ffi::c_void);
            tmp.assume_init()
        }
    }
}

impl __CcTemplateInstN24template_template_params10MyTemplateINS_6PolicyEEE {
    #[inline(always)]
    pub fn GetPolicy() -> ::core::ffi::c_int {
        unsafe {
            crate::detail::__rust_thunk___ZN24template_template_params10MyTemplateINS_6PolicyEE9GetPolicyEv__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc()
        }
    }
}

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
        pub(crate) unsafe fn __rust_thunk___Z36processForwardDeclaredSpecializationP18MyTopLevelTemplateIiE<
            'a,
        >(
            i: Option<::core::pin::Pin<&'a mut crate::__CcTemplateInst18MyTopLevelTemplateIiE>>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN23test_namespace_bindings10MyTemplateI14DifferentScopeEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN23test_namespace_bindings10MyTemplateI14DifferentScopeE6CreateES1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(
            __return: *mut ::core::ffi::c_void,
            value: &mut crate::DifferentScope,
        );
        pub(crate) unsafe fn __rust_thunk___ZNK23test_namespace_bindings10MyTemplateI14DifferentScopeE5valueEv__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc<
            'a,
        >(
            __this: &'a crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateI14DifferentScopeEE,
        ) -> &'a crate::DifferentScope;
        pub(crate) unsafe fn __rust_thunk___ZN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN23test_namespace_bindings10MyTemplateINS_13TemplateParamEE6CreateES1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(
            __return: *mut ::core::ffi::c_void,
            value: &mut crate::test_namespace_bindings::TemplateParam,
        );
        pub(crate) unsafe fn __rust_thunk___ZNK23test_namespace_bindings10MyTemplateINS_13TemplateParamEE5valueEv__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc<
            'a,
        >(
            __this: &'a crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEE,
        ) -> &'a crate::test_namespace_bindings::TemplateParam;
        pub(crate) unsafe fn __rust_thunk___ZN23test_namespace_bindings10MyTemplateIiEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN23test_namespace_bindings10MyTemplateIiE6CreateEi__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(
            __return: *mut ::core::ffi::c_void,
            value: ::core::ffi::c_int,
        );
        pub(crate) unsafe fn __rust_thunk___ZNK23test_namespace_bindings10MyTemplateIiE5valueEv__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc<
            'a,
        >(
            __this: &'a crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE,
        ) -> &'a ::core::ffi::c_int;
        pub(crate) unsafe fn __rust_thunk___ZN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN23test_namespace_bindings21TemplateWithTwoParamsIifEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN23test_namespace_bindings21TemplateWithTwoParamsIiiEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN23test_namespace_bindings8MyStructIcEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN24template_template_params10MyTemplateINS_6PolicyEEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN24template_template_params10MyTemplateINS_6PolicyEE9GetPolicyEv__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(
        ) -> ::core::ffi::c_int;
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

    assert!(
        ::core::mem::size_of::<
            crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateI14DifferentScopeEE,
        >() == 1
    );
    assert!(
        ::core::mem::align_of::<
            crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateI14DifferentScopeEE,
        >() == 1
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateI14DifferentScopeEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateI14DifferentScopeEE: Drop);
    assert!(
        ::core::mem::offset_of!(
            crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateI14DifferentScopeEE,
            value_
        ) == 0
    );
    assert!(
        ::core::mem::size_of::<
            crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEE,
        >() == 1
    );
    assert!(
        ::core::mem::align_of::<
            crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEE,
        >() == 1
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEE: Drop);
    assert!(
        ::core::mem::offset_of!(
            crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEE,
            value_
        ) == 0
    );
    assert!(
        ::core::mem::size_of::<crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE>()
            == 4
    );
    assert!(
        ::core::mem::align_of::<crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE>(
        ) == 4
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE: Drop);
    assert!(
        ::core::mem::offset_of!(
            crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE,
            value_
        ) == 0
    );
    assert!(
        ::core::mem::size_of::<
            crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEE,
        >() == 12
    );
    assert!(
        ::core::mem::align_of::<
            crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEE,
        >() == 4
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEE: Drop);
    assert!(
        ::core::mem::offset_of!(
            crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEE,
            value1
        ) == 0
    );
    assert!(
        ::core::mem::offset_of!(
            crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEE,
            value2
        ) == 8
    );
    assert!(
        ::core::mem::size_of::<
            crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIifEE,
        >() == 8
    );
    assert!(
        ::core::mem::align_of::<
            crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIifEE,
        >() == 4
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIifEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIifEE: Drop);
    assert!(
        ::core::mem::offset_of!(
            crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIifEE,
            value1
        ) == 0
    );
    assert!(
        ::core::mem::offset_of!(
            crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIifEE,
            value2
        ) == 4
    );
    assert!(
        ::core::mem::size_of::<
            crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIiiEE,
        >() == 8
    );
    assert!(
        ::core::mem::align_of::<
            crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIiiEE,
        >() == 4
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIiiEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIiiEE: Drop);
    assert!(
        ::core::mem::offset_of!(
            crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIiiEE,
            value1
        ) == 0
    );
    assert!(
        ::core::mem::offset_of!(
            crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIiiEE,
            value2
        ) == 4
    );
    assert!(
        ::core::mem::size_of::<crate::__CcTemplateInstN23test_namespace_bindings8MyStructIcEE>()
            == 1
    );
    assert!(
        ::core::mem::align_of::<crate::__CcTemplateInstN23test_namespace_bindings8MyStructIcEE>()
            == 1
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstN23test_namespace_bindings8MyStructIcEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstN23test_namespace_bindings8MyStructIcEE: Drop);

    assert!(
        ::core::mem::size_of::<
            crate::__CcTemplateInst18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEE,
        >() == 1
    );
    assert!(
        ::core::mem::align_of::<
            crate::__CcTemplateInst18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEE,
        >() == 1
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInst18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInst18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEE: Drop);
    assert!(
        ::core::mem::offset_of!(
            crate::__CcTemplateInst18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEE,
            value
        ) == 0
    );
    assert!(
        ::core::mem::size_of::<
            crate::__CcTemplateInstN24template_template_params10MyTemplateINS_6PolicyEEE,
        >() == 1
    );
    assert!(
        ::core::mem::align_of::<
            crate::__CcTemplateInstN24template_template_params10MyTemplateINS_6PolicyEEE,
        >() == 1
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstN24template_template_params10MyTemplateINS_6PolicyEEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstN24template_template_params10MyTemplateINS_6PolicyEEE: Drop);
};
