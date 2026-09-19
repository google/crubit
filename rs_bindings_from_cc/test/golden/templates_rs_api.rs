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

    pub type MyTypeAlias = crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE;

    pub type OtherTypeAliasInSameTarget =
        crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE;

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

    pub type TemplateWithStructTemplateParam =
        crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEE;

    pub type ParamFromDifferentScope =
        crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateI14DifferentScopeEE;

    // error: class `test_namespace_bindings::TemplateWithTwoParams` could not be bound
    //   Class templates are not yet supported

    pub type AliasToTemplateWithTwoParams =
        crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIifEE;

    pub type AliasToTemplateOfATemplate =
        crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEE;

    // error: class `test_namespace_bindings::MyStruct` could not be bound
    //   Class templates are not yet supported

    // Explicit class template specialization with definition should not be imported
    // unless also instantiated.

    // Explicit class template specialization with definition should be imported
    // even when not instantiated if there is a type alias for it.

    pub type MyCharStruct = crate::__CcTemplateInstN23test_namespace_bindings8MyStructIcEE;

    // Forward declared explicit class template specialization should be imported
    // so the forward declaration code is generated (`forward_declare!`).
}

// namespace test_namespace_bindings

// error: class `MyTopLevelTemplate` could not be bound
//   Class templates are not yet supported

pub type TopLevelTemplateWithNonTopLevelParam =
    crate::__CcTemplateInst18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEE;

// error: function `processForwardDeclaredSpecialization` could not be bound
//   Unsupported parameter type `MyTopLevelTemplate<int>* i`:
//     incomplete type

pub mod template_template_params {
    // error: class `template_template_params::Policy` could not be bound
    //   Class templates are not yet supported

    // error: class `template_template_params::MyTemplate` could not be bound
    //   Class templates are not yet supported

    pub type MyTypeAlias =
        crate::__CcTemplateInstN24template_template_params10MyTemplateINS_6PolicyEEE;
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
}

// namespace private_classes

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstN23test_namespace_bindings10MyTemplateI14DifferentScopeEE"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=test_namespace_bindings :: MyTemplate < DifferentScope >
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstN23test_namespace_bindings10MyTemplateI14DifferentScopeEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) value_: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for __CcTemplateInstN23test_namespace_bindings10MyTemplateI14DifferentScopeEE {}
impl !Sync for __CcTemplateInstN23test_namespace_bindings10MyTemplateI14DifferentScopeEE {}

impl Default for __CcTemplateInstN23test_namespace_bindings10MyTemplateI14DifferentScopeEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__602f8b9b__ZN23test_namespace_bindings10MyTemplateI14DifferentScopeEC1Ev(&raw mut tmp as*mut _);
            tmp.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEE"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=test_namespace_bindings :: MyTemplate < test_namespace_bindings :: TemplateParam >
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) value_: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for __CcTemplateInstN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEE {}
impl !Sync for __CcTemplateInstN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEE {}

impl Default for __CcTemplateInstN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__602f8b9b__ZN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEC1Ev(&raw mut tmp as*mut _);
            tmp.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE"]
#[repr(C, align(4))]
///CRUBIT_ANNOTATE: cpp_type=test_namespace_bindings :: MyTemplate < int >
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) value_: [::core::mem::MaybeUninit<u8>; 4],
}
impl !Send for __CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE {}
impl !Sync for __CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE {}

impl Default for __CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__602f8b9b__ZN23test_namespace_bindings10MyTemplateIiEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEE"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=test_namespace_bindings :: TemplateWithTwoParams < test_namespace_bindings :: TemplateWithTwoParams < int , int >, int >
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEE {
    pub value1: crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIiiEE,
    pub value2: ::ffi_11::c_int,
}
impl !Send for __CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEE {}
impl !Sync for __CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEE {}

impl Default for __CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__994b0149__ZN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEC1Ev(&raw mut tmp as*mut _);
            tmp.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIifEE"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=test_namespace_bindings :: TemplateWithTwoParams < int , float >
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIifEE {
    pub value1: ::ffi_11::c_int,
    pub value2: f32,
}
impl !Send for __CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIifEE {}
impl !Sync for __CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIifEE {}

impl Default for __CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIifEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__994b0149__ZN23test_namespace_bindings21TemplateWithTwoParamsIifEC1Ev(&raw mut tmp as*mut _);
            tmp.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIiiEE"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=test_namespace_bindings :: TemplateWithTwoParams < int , int >
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIiiEE {
    pub value1: ::ffi_11::c_int,
    pub value2: ::ffi_11::c_int,
}
impl !Send for __CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIiiEE {}
impl !Sync for __CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIiiEE {}

impl Default for __CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIiiEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__994b0149__ZN23test_namespace_bindings21TemplateWithTwoParamsIiiEC1Ev(&raw mut tmp as*mut _);
            tmp.assume_init()
        }
    }
}

/// Explicit class template specialization with definition should be imported
/// even when not instantiated if there is a type alias for it.
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstN23test_namespace_bindings8MyStructIcEE"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=test_namespace_bindings :: MyStruct < char >
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstN23test_namespace_bindings8MyStructIcEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for __CcTemplateInstN23test_namespace_bindings8MyStructIcEE {}
impl !Sync for __CcTemplateInstN23test_namespace_bindings8MyStructIcEE {}

impl Default for __CcTemplateInstN23test_namespace_bindings8MyStructIcEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__9d72ecd0__ZN23test_namespace_bindings8MyStructIcEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInst18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEE"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=MyTopLevelTemplate < test_namespace_bindings :: TemplateParam >
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInst18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEE {
    pub value: crate::test_namespace_bindings::TemplateParam,
}
impl !Send for __CcTemplateInst18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEE {}
impl !Sync for __CcTemplateInst18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEE {}

impl Default for __CcTemplateInst18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__b72784dd__ZN18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEEC1Ev(&raw mut tmp as*mut _);
            tmp.assume_init()
        }
    }
}

// error: struct `MyTopLevelTemplate<int>` could not be bound
//   incomplete type

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstN24template_template_params10MyTemplateINS_6PolicyEEE"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=template_template_params :: MyTemplate < template_template_params :: Policy >
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstN24template_template_params10MyTemplateINS_6PolicyEEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for __CcTemplateInstN24template_template_params10MyTemplateINS_6PolicyEEE {}
impl !Sync for __CcTemplateInstN24template_template_params10MyTemplateINS_6PolicyEEE {}

impl Default for __CcTemplateInstN24template_template_params10MyTemplateINS_6PolicyEEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__57a95918__ZN24template_template_params10MyTemplateINS_6PolicyEEC1Ev(&raw mut tmp as*mut _);
            tmp.assume_init()
        }
    }
}

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
        pub(crate) unsafe fn __rust_thunk__602f8b9b__ZN23test_namespace_bindings10MyTemplateI14DifferentScopeEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__602f8b9b__ZN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__602f8b9b__ZN23test_namespace_bindings10MyTemplateIiEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__994b0149__ZN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__994b0149__ZN23test_namespace_bindings21TemplateWithTwoParamsIifEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__994b0149__ZN23test_namespace_bindings21TemplateWithTwoParamsIiiEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__9d72ecd0__ZN23test_namespace_bindings8MyStructIcEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__b72784dd__ZN18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__57a95918__ZN24template_template_params10MyTemplateINS_6PolicyEEC1Ev(
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
