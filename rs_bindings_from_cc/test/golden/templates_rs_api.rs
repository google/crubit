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

    #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
    #[cfi_encoding = "__CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE"]
    #[repr(C, align(4))]
    ///CRUBIT_ANNOTATE: cpp_type=test_namespace_bindings :: MyTemplate < int >
    ///CRUBIT_ANNOTATE: cpp_move_constructible=
    pub struct MyTypeAlias {
        __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
        /// Reason for representing this field as a blob of bytes:
        /// Types of non-public C++ fields can be elided away
        pub(crate) value_: [::core::mem::MaybeUninit<u8>; 4],
    }
    impl !Send for MyTypeAlias {}
    impl !Sync for MyTypeAlias {}
    forward_declare::unsafe_define!(
        forward_declare::symbol!("test_namespace_bindings :: MyTemplate < int >"),
        crate::test_namespace_bindings::MyTypeAlias
    );

    impl Default for MyTypeAlias {
        #[inline(always)]
        fn default() -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk__602f8b9b__ZN23test_namespace_bindings10MyTemplateIiEC1Ev(&raw mut tmp as*mut _);
                tmp.assume_init()
            }
        }
    }

    pub type OtherTypeAliasInSameTarget = crate::test_namespace_bindings::MyTypeAlias;

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

    #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
    #[cfi_encoding = "__CcTemplateInstN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEE"]
    #[repr(C)]
    ///CRUBIT_ANNOTATE: cpp_type=test_namespace_bindings :: MyTemplate < struct test_namespace_bindings :: TemplateParam >
    ///CRUBIT_ANNOTATE: cpp_move_constructible=
    pub struct TemplateWithStructTemplateParam {
        __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
        /// Reason for representing this field as a blob of bytes:
        /// Types of non-public C++ fields can be elided away
        pub(crate) value_: [::core::mem::MaybeUninit<u8>; 1],
    }
    impl !Send for TemplateWithStructTemplateParam {}
    impl !Sync for TemplateWithStructTemplateParam {}
    forward_declare::unsafe_define!(forward_declare::symbol!("test_namespace_bindings :: MyTemplate < struct test_namespace_bindings :: TemplateParam >"),crate::test_namespace_bindings::TemplateWithStructTemplateParam);

    impl Default for TemplateWithStructTemplateParam {
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
    #[cfi_encoding = "__CcTemplateInstN23test_namespace_bindings10MyTemplateI14DifferentScopeEE"]
    #[repr(C)]
    ///CRUBIT_ANNOTATE: cpp_type=test_namespace_bindings :: MyTemplate < struct DifferentScope >
    ///CRUBIT_ANNOTATE: cpp_move_constructible=
    pub struct ParamFromDifferentScope {
        __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
        /// Reason for representing this field as a blob of bytes:
        /// Types of non-public C++ fields can be elided away
        pub(crate) value_: [::core::mem::MaybeUninit<u8>; 1],
    }
    impl !Send for ParamFromDifferentScope {}
    impl !Sync for ParamFromDifferentScope {}
    forward_declare::unsafe_define!(
        forward_declare::symbol!("test_namespace_bindings :: MyTemplate < struct DifferentScope >"),
        crate::test_namespace_bindings::ParamFromDifferentScope
    );

    impl Default for ParamFromDifferentScope {
        #[inline(always)]
        fn default() -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk__602f8b9b__ZN23test_namespace_bindings10MyTemplateI14DifferentScopeEC1Ev(&raw mut tmp as*mut _);
                tmp.assume_init()
            }
        }
    }

    // error: class `test_namespace_bindings::TemplateWithTwoParams` could not be bound
    //   Class templates are not yet supported

    #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
    #[cfi_encoding = "__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIifEE"]
    #[repr(C)]
    ///CRUBIT_ANNOTATE: cpp_type=test_namespace_bindings :: TemplateWithTwoParams < int , float >
    ///CRUBIT_ANNOTATE: cpp_move_constructible=
    pub struct AliasToTemplateWithTwoParams {
        pub value1: ::ffi_11::c_int,
        pub value2: f32,
    }
    impl !Send for AliasToTemplateWithTwoParams {}
    impl !Sync for AliasToTemplateWithTwoParams {}
    forward_declare::unsafe_define!(
        forward_declare::symbol!(
            "test_namespace_bindings :: TemplateWithTwoParams < int , float >"
        ),
        crate::test_namespace_bindings::AliasToTemplateWithTwoParams
    );

    impl Default for AliasToTemplateWithTwoParams {
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
    #[cfi_encoding = "__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEE"]
    #[repr(C)]
    ///CRUBIT_ANNOTATE: cpp_type=test_namespace_bindings :: TemplateWithTwoParams < struct test_namespace_bindings :: TemplateWithTwoParams < int , int >, int >
    ///CRUBIT_ANNOTATE: cpp_move_constructible=
    pub struct AliasToTemplateOfATemplate{ pub value1: crate::zz_template_instantiations::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIiiEE,pub value2: ::ffi_11::c_int, }
    impl !Send for AliasToTemplateOfATemplate {}
    impl !Sync for AliasToTemplateOfATemplate {}
    forward_declare::unsafe_define!(forward_declare::symbol!("test_namespace_bindings :: TemplateWithTwoParams < struct test_namespace_bindings :: TemplateWithTwoParams < int , int >, int >"),crate::test_namespace_bindings::AliasToTemplateOfATemplate);

    impl Default for AliasToTemplateOfATemplate {
        #[inline(always)]
        fn default() -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk__994b0149__ZN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEC1Ev(&raw mut tmp as*mut _);
                tmp.assume_init()
            }
        }
    }

    // error: class `test_namespace_bindings::MyStruct` could not be bound
    //   Class templates are not yet supported

    // Explicit class template specialization with definition should not be imported
    // unless also instantiated.

    // Explicit class template specialization with definition should be imported
    // even when not instantiated if there is a type alias for it.

    /// Explicit class template specialization with definition should be imported
    /// even when not instantiated if there is a type alias for it.
    #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
    #[cfi_encoding = "__CcTemplateInstN23test_namespace_bindings8MyStructIcEE"]
    #[repr(C)]
    ///CRUBIT_ANNOTATE: cpp_type=test_namespace_bindings :: MyStruct < char >
    ///CRUBIT_ANNOTATE: cpp_move_constructible=
    pub struct MyCharStruct {
        __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
    }
    impl !Send for MyCharStruct {}
    impl !Sync for MyCharStruct {}
    forward_declare::unsafe_define!(
        forward_declare::symbol!("test_namespace_bindings :: MyStruct < char >"),
        crate::test_namespace_bindings::MyCharStruct
    );

    impl Default for MyCharStruct {
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

    // Forward declared explicit class template specialization should be imported
    // so the forward declaration code is generated (`forward_declare!`).
}

// namespace test_namespace_bindings

// error: class `MyTopLevelTemplate` could not be bound
//   Class templates are not yet supported

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInst18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEE"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=MyTopLevelTemplate < struct test_namespace_bindings :: TemplateParam >
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct TopLevelTemplateWithNonTopLevelParam {
    pub value: crate::test_namespace_bindings::TemplateParam,
}
impl !Send for TopLevelTemplateWithNonTopLevelParam {}
impl !Sync for TopLevelTemplateWithNonTopLevelParam {}
forward_declare::unsafe_define!(
    forward_declare::symbol!(
        "MyTopLevelTemplate < struct test_namespace_bindings :: TemplateParam >"
    ),
    crate::TopLevelTemplateWithNonTopLevelParam
);

impl Default for TopLevelTemplateWithNonTopLevelParam {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__b72784dd__ZN18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEEC1Ev(&raw mut tmp as*mut _);
            tmp.assume_init()
        }
    }
}

/// # Safety
///
/// The caller must ensure that the following unsafe arguments are not misused by the function:
/// * `i`: raw pointer
#[inline(always)]
pub unsafe fn processForwardDeclaredSpecialization(
    i: *mut crate::zz_template_instantiations::__CcTemplateInst18MyTopLevelTemplateIiE,
) {
    unsafe {
        crate::detail::__rust_thunk___Z36processForwardDeclaredSpecializationP18MyTopLevelTemplateIiE(i)
    }
}

pub mod template_template_params {
    // error: class `template_template_params::Policy` could not be bound
    //   Class templates are not yet supported

    // error: class `template_template_params::MyTemplate` could not be bound
    //   Class templates are not yet supported

    #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
    #[cfi_encoding = "__CcTemplateInstN24template_template_params10MyTemplateINS_6PolicyEEE"]
    #[repr(C)]
    ///CRUBIT_ANNOTATE: cpp_type=template_template_params :: MyTemplate < template_template_params :: Policy >
    ///CRUBIT_ANNOTATE: cpp_move_constructible=
    pub struct MyTypeAlias {
        __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
    }
    impl !Send for MyTypeAlias {}
    impl !Sync for MyTypeAlias {}
    forward_declare::unsafe_define!(
        forward_declare::symbol!(
            "template_template_params :: MyTemplate < template_template_params :: Policy >"
        ),
        crate::template_template_params::MyTypeAlias
    );

    impl Default for MyTypeAlias {
        #[inline(always)]
        fn default() -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk__57a95918__ZN24template_template_params10MyTemplateINS_6PolicyEEC1Ev(&raw mut tmp as*mut _);
                tmp.assume_init()
            }
        }
    }
}

// namespace template_template_params

pub mod forward_declared_template {
    // error: class `forward_declared_template::ForwardDeclaredTemplate` could not be bound
    //   Class templates are not yet supported

    pub type TypeAliasToForwardDeclaredTemplate=crate::zz_template_instantiations::__CcTemplateInstN25forward_declared_template23ForwardDeclaredTemplateIiEE;
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

/// Class template instantiations that aren't named by a C++ type alias.
pub mod zz_template_instantiations {
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
    forward_declare::unsafe_define!(forward_declare::symbol!("test_namespace_bindings :: TemplateWithTwoParams < int , int >"),crate::zz_template_instantiations::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIiiEE);

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

    forward_declare::forward_declare!(pub __CcTemplateInst18MyTopLevelTemplateIiE = forward_declare::symbol!("MyTopLevelTemplate < int >"));

    forward_declare::forward_declare!(pub __CcTemplateInstN25forward_declared_template23ForwardDeclaredTemplateIiEE = forward_declare::symbol!("forward_declared_template :: ForwardDeclaredTemplate < int >"));

    #[doc(hidden)]
    pub type __CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE =
        crate::test_namespace_bindings::MyTypeAlias;

    #[doc(hidden)]
    pub type __CcTemplateInstN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEE =
        crate::test_namespace_bindings::TemplateWithStructTemplateParam;

    #[doc(hidden)]
    pub type __CcTemplateInstN23test_namespace_bindings10MyTemplateI14DifferentScopeEE =
        crate::test_namespace_bindings::ParamFromDifferentScope;

    #[doc(hidden)]
    pub type __CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIifEE =
        crate::test_namespace_bindings::AliasToTemplateWithTwoParams;

    #[doc(hidden)]
    pub type __CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEE =
        crate::test_namespace_bindings::AliasToTemplateOfATemplate;

    #[doc(hidden)]
    pub type __CcTemplateInstN23test_namespace_bindings8MyStructIcEE =
        crate::test_namespace_bindings::MyCharStruct;

    #[doc(hidden)]
    pub type __CcTemplateInst18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEE =
        crate::TopLevelTemplateWithNonTopLevelParam;

    #[doc(hidden)]
    pub type __CcTemplateInstN24template_template_params10MyTemplateINS_6PolicyEEE =
        crate::template_template_params::MyTypeAlias;
}
// Keeps the instantiations available at their old path.
#[doc(hidden)]
pub use self::zz_template_instantiations::*;

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN14DifferentScopeC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__602f8b9b__ZN23test_namespace_bindings10MyTemplateIiEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN23test_namespace_bindings13TemplateParamC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__602f8b9b__ZN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__602f8b9b__ZN23test_namespace_bindings10MyTemplateI14DifferentScopeEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__994b0149__ZN23test_namespace_bindings21TemplateWithTwoParamsIifEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__994b0149__ZN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__9d72ecd0__ZN23test_namespace_bindings8MyStructIcEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__b72784dd__ZN18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        #[link_name = "_Z36processForwardDeclaredSpecializationP18MyTopLevelTemplateIiE"]
        pub(crate) unsafe fn __rust_thunk___Z36processForwardDeclaredSpecializationP18MyTopLevelTemplateIiE(
            i: *mut crate::zz_template_instantiations::__CcTemplateInst18MyTopLevelTemplateIiE,
        );
        pub(crate) unsafe fn __rust_thunk__57a95918__ZN24template_template_params10MyTemplateINS_6PolicyEEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__994b0149__ZN23test_namespace_bindings21TemplateWithTwoParamsIiiEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::DifferentScope>() == 1);
    assert!(::core::mem::align_of::<crate::DifferentScope>() == 1);
    static_assertions::assert_impl_all!(crate::DifferentScope: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::DifferentScope: Drop);

    assert!(::core::mem::size_of::<crate::test_namespace_bindings::MyTypeAlias>() == 4);
    assert!(::core::mem::align_of::<crate::test_namespace_bindings::MyTypeAlias>() == 4);
    static_assertions::assert_impl_all!(crate::test_namespace_bindings::MyTypeAlias: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::test_namespace_bindings::MyTypeAlias: Drop);
    assert!(::core::mem::offset_of!(crate::test_namespace_bindings::MyTypeAlias, value_) == 0);
    assert!(::core::mem::size_of::<crate::test_namespace_bindings::TemplateParam>() == 1);
    assert!(::core::mem::align_of::<crate::test_namespace_bindings::TemplateParam>() == 1);
    static_assertions::assert_impl_all!(crate::test_namespace_bindings::TemplateParam: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::test_namespace_bindings::TemplateParam: Drop);

    assert!(
        ::core::mem::size_of::<crate::test_namespace_bindings::TemplateWithStructTemplateParam>()
            == 1
    );
    assert!(
        ::core::mem::align_of::<crate::test_namespace_bindings::TemplateWithStructTemplateParam>()
            == 1
    );
    static_assertions::assert_impl_all!(crate::test_namespace_bindings::TemplateWithStructTemplateParam: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::test_namespace_bindings::TemplateWithStructTemplateParam: Drop);
    assert!(
        ::core::mem::offset_of!(
            crate::test_namespace_bindings::TemplateWithStructTemplateParam,
            value_
        ) == 0
    );
    assert!(::core::mem::size_of::<crate::test_namespace_bindings::ParamFromDifferentScope>() == 1);
    assert!(
        ::core::mem::align_of::<crate::test_namespace_bindings::ParamFromDifferentScope>() == 1
    );
    static_assertions::assert_impl_all!(crate::test_namespace_bindings::ParamFromDifferentScope: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::test_namespace_bindings::ParamFromDifferentScope: Drop);
    assert!(
        ::core::mem::offset_of!(crate::test_namespace_bindings::ParamFromDifferentScope, value_)
            == 0
    );
    assert!(
        ::core::mem::size_of::<crate::test_namespace_bindings::AliasToTemplateWithTwoParams>() == 8
    );
    assert!(
        ::core::mem::align_of::<crate::test_namespace_bindings::AliasToTemplateWithTwoParams>()
            == 4
    );
    static_assertions::assert_impl_all!(crate::test_namespace_bindings::AliasToTemplateWithTwoParams: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::test_namespace_bindings::AliasToTemplateWithTwoParams: Drop);
    assert!(
        ::core::mem::offset_of!(
            crate::test_namespace_bindings::AliasToTemplateWithTwoParams,
            value1
        ) == 0
    );
    assert!(
        ::core::mem::offset_of!(
            crate::test_namespace_bindings::AliasToTemplateWithTwoParams,
            value2
        ) == 4
    );
    assert!(
        ::core::mem::size_of::<crate::test_namespace_bindings::AliasToTemplateOfATemplate>() == 12
    );
    assert!(
        ::core::mem::align_of::<crate::test_namespace_bindings::AliasToTemplateOfATemplate>() == 4
    );
    static_assertions::assert_impl_all!(crate::test_namespace_bindings::AliasToTemplateOfATemplate: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::test_namespace_bindings::AliasToTemplateOfATemplate: Drop);
    assert!(
        ::core::mem::offset_of!(crate::test_namespace_bindings::AliasToTemplateOfATemplate, value1)
            == 0
    );
    assert!(
        ::core::mem::offset_of!(crate::test_namespace_bindings::AliasToTemplateOfATemplate, value2)
            == 8
    );
    assert!(::core::mem::size_of::<crate::test_namespace_bindings::MyCharStruct>() == 1);
    assert!(::core::mem::align_of::<crate::test_namespace_bindings::MyCharStruct>() == 1);
    static_assertions::assert_impl_all!(crate::test_namespace_bindings::MyCharStruct: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::test_namespace_bindings::MyCharStruct: Drop);

    assert!(::core::mem::size_of::<crate::TopLevelTemplateWithNonTopLevelParam>() == 1);
    assert!(::core::mem::align_of::<crate::TopLevelTemplateWithNonTopLevelParam>() == 1);
    static_assertions::assert_impl_all!(crate::TopLevelTemplateWithNonTopLevelParam: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::TopLevelTemplateWithNonTopLevelParam: Drop);
    assert!(::core::mem::offset_of!(crate::TopLevelTemplateWithNonTopLevelParam, value) == 0);
    assert!(::core::mem::size_of::<crate::template_template_params::MyTypeAlias>() == 1);
    assert!(::core::mem::align_of::<crate::template_template_params::MyTypeAlias>() == 1);
    static_assertions::assert_impl_all!(crate::template_template_params::MyTypeAlias: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::template_template_params::MyTypeAlias: Drop);

    assert!(::core::mem::size_of::<crate::private_classes::HasPrivateType>() == 1);
    assert!(::core::mem::align_of::<crate::private_classes::HasPrivateType>() == 1);
    static_assertions::assert_impl_all!(crate::private_classes::HasPrivateType: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::private_classes::HasPrivateType: Drop);

    assert!(::core::mem::size_of::<crate::zz_template_instantiations::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIiiEE>()==8);
    assert!(::core::mem::align_of::<crate::zz_template_instantiations::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIiiEE>()==4);
    static_assertions::assert_impl_all!(crate::zz_template_instantiations::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIiiEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::zz_template_instantiations::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIiiEE: Drop);
    assert!(::core::mem::offset_of!(crate::zz_template_instantiations::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIiiEE,value1)==0);
    assert!(::core::mem::offset_of!(crate::zz_template_instantiations::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIiiEE,value2)==4);
};
