// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:templates_cc

#![feature(negative_impls, type_alias_impl_trait)]
#![allow(stable_features)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![deny(warnings)]

// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[derive(Clone, Copy)]
#[repr(C)]
pub struct DifferentScope {
    __non_field_data: [::std::mem::MaybeUninit<u8>; 1],
}
forward_declare::unsafe_define!(forward_declare::symbol!("DifferentScope"), crate::DifferentScope);

impl Default for DifferentScope {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN14DifferentScopeC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b> From<::ctor::RvalueReference<'b, crate::DifferentScope>> for DifferentScope {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, crate::DifferentScope>) -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN14DifferentScopeC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

// rs_bindings_from_cc/test/golden/templates.h;l=10
// Error while generating bindings for item 'DifferentScope::operator=':
// operator= for Unpin types is not yet supported.

// rs_bindings_from_cc/test/golden/templates.h;l=10
// Error while generating bindings for item 'DifferentScope::operator=':
// operator= for Unpin types is not yet supported.

pub mod test_namespace_bindings {
    // rs_bindings_from_cc/test/golden/templates.h;l=14
    // Error while generating bindings for item 'test_namespace_bindings::MyTemplate':
    // Class templates are not supported yet

    pub type MyTypeAlias = crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE;

    pub type OtherTypeAliasInSameTarget =
        crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE;

    #[derive(Clone, Copy)]
    #[repr(C)]
    pub struct TemplateParam {
        __non_field_data: [::std::mem::MaybeUninit<u8>; 1],
    }
    forward_declare::unsafe_define!(
        forward_declare::symbol!("TemplateParam"),
        crate::test_namespace_bindings::TemplateParam
    );

    impl Default for TemplateParam {
        #[inline(always)]
        fn default() -> Self {
            let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk___ZN23test_namespace_bindings13TemplateParamC1Ev(
                    &mut tmp,
                );
                tmp.assume_init()
            }
        }
    }

    impl<'b> From<::ctor::RvalueReference<'b, crate::test_namespace_bindings::TemplateParam>>
        for TemplateParam
    {
        #[inline(always)]
        fn from(
            __param_0: ::ctor::RvalueReference<'b, crate::test_namespace_bindings::TemplateParam>,
        ) -> Self {
            let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk___ZN23test_namespace_bindings13TemplateParamC1EOS0_(
                    &mut tmp, __param_0,
                );
                tmp.assume_init()
            }
        }
    }

    // rs_bindings_from_cc/test/golden/templates.h;l=32
    // Error while generating bindings for item 'TemplateParam::operator=':
    // operator= for Unpin types is not yet supported.

    // rs_bindings_from_cc/test/golden/templates.h;l=32
    // Error while generating bindings for item 'TemplateParam::operator=':
    // operator= for Unpin types is not yet supported.

    pub type TemplateWithStructTemplateParam =
        crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEE;

    pub type ParamFromDifferentScope =
        crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateI14DifferentScopeEE;

    // rs_bindings_from_cc/test/golden/templates.h;l=36
    // Error while generating bindings for item 'test_namespace_bindings::TemplateWithTwoParams':
    // Class templates are not supported yet

    pub type AliasToTemplateWithTwoParams =
        crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIifEE;

    pub type AliasToTemplateOfATemplate =
        crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEE;

    // rs_bindings_from_cc/test/golden/templates.h;l=47
    // Error while generating bindings for item 'test_namespace_bindings::MyStruct':
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

// rs_bindings_from_cc/test/golden/templates.h;l=79
// Error while generating bindings for item 'MyTopLevelTemplate':
// Class templates are not supported yet

pub type TopLevelTemplateWithNonTopLevelParam =
    crate::__CcTemplateInst18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEE;

#[inline(always)]
pub fn processForwardDeclaredSpecialization<'a>(
    i: Option<::std::pin::Pin<&'a mut crate::__CcTemplateInst18MyTopLevelTemplateIiE>>,
) {
    unsafe {
        crate::detail::__rust_thunk___Z36processForwardDeclaredSpecializationP18MyTopLevelTemplateIiE(i)
    }
}

pub mod template_template_params {
    // rs_bindings_from_cc/test/golden/templates.h;l=94
    // Error while generating bindings for item 'template_template_params::Policy':
    // Class templates are not supported yet

    // rs_bindings_from_cc/test/golden/templates.h;l=104
    // Error while generating bindings for item 'template_template_params::MyTemplate':
    // Class templates are not supported yet

    pub type MyTypeAlias =
        crate::__CcTemplateInstN24template_template_params10MyTemplateINS_6PolicyEEE;
}

// namespace template_template_params

pub mod forward_declared_template {
    // rs_bindings_from_cc/test/golden/templates.h;l=122
    // Error while generating bindings for item 'forward_declared_template::ForwardDeclaredTemplate':
    // Class templates are not supported yet

    pub type TypeAliasToForwardDeclaredTemplate =
        crate::__CcTemplateInstN25forward_declared_template23ForwardDeclaredTemplateIiEE;
}

// namespace forward_declared_template

pub mod private_classes {
    #[::ctor::recursively_pinned]
    #[repr(C)]
    pub struct HasPrivateType {
        __non_field_data: [::std::mem::MaybeUninit<u8>; 1],
    }
    forward_declare::unsafe_define!(
        forward_declare::symbol!("HasPrivateType"),
        crate::private_classes::HasPrivateType
    );

    impl<'b> ::ctor::CtorNew<&'b crate::private_classes::HasPrivateType> for HasPrivateType {
        type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
        #[inline(always)]
        fn ctor_new(args: &'b crate::private_classes::HasPrivateType) -> Self::CtorType {
            let __param_0 = args;
            unsafe {
                ::ctor::FnCtor::new(
                    move |dest: ::std::pin::Pin<
                        &mut ::std::mem::MaybeUninit<crate::private_classes::HasPrivateType>,
                    >| {
                        crate::detail::__rust_thunk___ZN15private_classes14HasPrivateTypeC1ERKS0_(
                            ::std::pin::Pin::into_inner_unchecked(dest),
                            __param_0,
                        );
                    },
                )
            }
        }
    }
    impl<'b> ::ctor::CtorNew<(&'b crate::private_classes::HasPrivateType,)> for HasPrivateType {
        type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
        #[inline(always)]
        fn ctor_new(args: (&'b crate::private_classes::HasPrivateType,)) -> Self::CtorType {
            let (arg,) = args;
            <Self as ::ctor::CtorNew<&'b crate::private_classes::HasPrivateType>>::ctor_new(arg)
        }
    }

    impl<'b> ::ctor::CtorNew<::ctor::RvalueReference<'b, crate::private_classes::HasPrivateType>>
        for HasPrivateType
    {
        type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
        #[inline(always)]
        fn ctor_new(
            args: ::ctor::RvalueReference<'b, crate::private_classes::HasPrivateType>,
        ) -> Self::CtorType {
            let __param_0 = args;
            unsafe {
                ::ctor::FnCtor::new(
                    move |dest: ::std::pin::Pin<
                        &mut ::std::mem::MaybeUninit<crate::private_classes::HasPrivateType>,
                    >| {
                        crate::detail::__rust_thunk___ZN15private_classes14HasPrivateTypeC1EOS0_(
                            ::std::pin::Pin::into_inner_unchecked(dest),
                            __param_0,
                        );
                    },
                )
            }
        }
    }
    impl<'b> ::ctor::CtorNew<(::ctor::RvalueReference<'b, crate::private_classes::HasPrivateType>,)>
        for HasPrivateType
    {
        type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
        #[inline(always)]
        fn ctor_new(
            args: (::ctor::RvalueReference<'b, crate::private_classes::HasPrivateType>,),
        ) -> Self::CtorType {
            let (arg,) = args;
            <Self as ::ctor::CtorNew<
                ::ctor::RvalueReference<'b, crate::private_classes::HasPrivateType>,
            >>::ctor_new(arg)
        }
    }

    impl<'b> ::ctor::Assign<&'b Self> for HasPrivateType {
        #[inline(always)]
        fn assign<'a>(self: ::std::pin::Pin<&'a mut Self>, __param_0: &'b Self) {
            unsafe {
                crate::detail::__rust_thunk___ZN15private_classes14HasPrivateTypeaSERKS0_(
                    self, __param_0,
                );
            }
        }
    }

    impl<'b> ::ctor::Assign<::ctor::RvalueReference<'b, crate::private_classes::HasPrivateType>>
        for HasPrivateType
    {
        #[inline(always)]
        fn assign<'a>(
            self: ::std::pin::Pin<&'a mut Self>,
            __param_0: ::ctor::RvalueReference<'b, crate::private_classes::HasPrivateType>,
        ) {
            unsafe {
                crate::detail::__rust_thunk___ZN15private_classes14HasPrivateTypeaSEOS0_(
                    self, __param_0,
                );
            }
        }
    }
}

// namespace private_classes

// THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_TEMPLATES_H_

#[derive(Clone, Copy)]
#[repr(C)]
pub struct __CcTemplateInstN23test_namespace_bindings10MyTemplateI14DifferentScopeEE {
    __non_field_data: [::std::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) value_: [::std::mem::MaybeUninit<u8>; 1],
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("test_namespace_bindings::MyTemplate<DifferentScope>"),
    crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateI14DifferentScopeEE
);

impl Default for __CcTemplateInstN23test_namespace_bindings10MyTemplateI14DifferentScopeEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN23test_namespace_bindings10MyTemplateI14DifferentScopeEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b>
    From<
        ::ctor::RvalueReference<
            'b,
            crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateI14DifferentScopeEE,
        >,
    > for __CcTemplateInstN23test_namespace_bindings10MyTemplateI14DifferentScopeEE
{
    #[inline(always)]
    fn from(
        __param_0: ::ctor::RvalueReference<
            'b,
            crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateI14DifferentScopeEE,
        >,
    ) -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN23test_namespace_bindings10MyTemplateI14DifferentScopeEC1EOS2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(&mut tmp,__param_0);
            tmp.assume_init()
        }
    }
}

// rs_bindings_from_cc/test/golden/templates.h;l=15
// Error while generating bindings for item 'test_namespace_bindings::MyTemplate<DifferentScope>::operator=':
// operator= for Unpin types is not yet supported.

// rs_bindings_from_cc/test/golden/templates.h;l=15
// Error while generating bindings for item 'test_namespace_bindings::MyTemplate<DifferentScope>::operator=':
// operator= for Unpin types is not yet supported.

impl __CcTemplateInstN23test_namespace_bindings10MyTemplateI14DifferentScopeEE {
    #[inline(always)]
    pub fn Create(
        value: crate::DifferentScope,
    ) -> crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateI14DifferentScopeEE {
        unsafe {
            crate::detail::__rust_thunk___ZN23test_namespace_bindings10MyTemplateI14DifferentScopeE6CreateES1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(value)
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

#[derive(Clone, Copy)]
#[repr(C)]
pub struct __CcTemplateInstN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEE {
    __non_field_data: [::std::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) value_: [::std::mem::MaybeUninit<u8>; 1],
}
forward_declare::unsafe_define!(
    forward_declare::symbol!(
        "test_namespace_bindings::MyTemplate<test_namespace_bindings::TemplateParam>"
    ),
    crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEE
);

impl Default for __CcTemplateInstN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b>
    From<
        ::ctor::RvalueReference<
            'b,
            crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEE,
        >,
    > for __CcTemplateInstN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEE
{
    #[inline(always)]
    fn from(
        __param_0: ::ctor::RvalueReference<
            'b,
            crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEE,
        >,
    ) -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEC1EOS2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(&mut tmp,__param_0);
            tmp.assume_init()
        }
    }
}

// rs_bindings_from_cc/test/golden/templates.h;l=15
// Error while generating bindings for item 'test_namespace_bindings::MyTemplate<test_namespace_bindings::TemplateParam>::operator=':
// operator= for Unpin types is not yet supported.

// rs_bindings_from_cc/test/golden/templates.h;l=15
// Error while generating bindings for item 'test_namespace_bindings::MyTemplate<test_namespace_bindings::TemplateParam>::operator=':
// operator= for Unpin types is not yet supported.

impl __CcTemplateInstN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEE {
    #[inline(always)]
    pub fn Create(
        value: crate::test_namespace_bindings::TemplateParam,
    ) -> crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEE {
        unsafe {
            crate::detail::__rust_thunk___ZN23test_namespace_bindings10MyTemplateINS_13TemplateParamEE6CreateES1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(value)
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

#[derive(Clone, Copy)]
#[repr(C, align(4))]
pub struct __CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE {
    __non_field_data: [::std::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) value_: [::std::mem::MaybeUninit<u8>; 4],
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("test_namespace_bindings::MyTemplate<int>"),
    crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE
);

impl Default for __CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN23test_namespace_bindings10MyTemplateIiEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b>
    From<
        ::ctor::RvalueReference<
            'b,
            crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE,
        >,
    > for __CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE
{
    #[inline(always)]
    fn from(
        __param_0: ::ctor::RvalueReference<
            'b,
            crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE,
        >,
    ) -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN23test_namespace_bindings10MyTemplateIiEC1EOS1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(&mut tmp,__param_0);
            tmp.assume_init()
        }
    }
}

// rs_bindings_from_cc/test/golden/templates.h;l=15
// Error while generating bindings for item 'test_namespace_bindings::MyTemplate<int>::operator=':
// operator= for Unpin types is not yet supported.

// rs_bindings_from_cc/test/golden/templates.h;l=15
// Error while generating bindings for item 'test_namespace_bindings::MyTemplate<int>::operator=':
// operator= for Unpin types is not yet supported.

impl __CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE {
    #[inline(always)]
    pub fn Create(value: i32) -> crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE {
        unsafe {
            crate::detail::__rust_thunk___ZN23test_namespace_bindings10MyTemplateIiE6CreateEi__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(value)
        }
    }
}

impl __CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE {
    #[inline(always)]
    pub fn value<'a>(&'a self) -> &'a i32 {
        unsafe {
            crate::detail::__rust_thunk___ZNK23test_namespace_bindings10MyTemplateIiE5valueEv__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(self)
        }
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct __CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEE {
    pub value1: crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIiiEE,
    pub value2: i32,
}
forward_declare::unsafe_define!(
    forward_declare::symbol!(
        "test_namespace_bindings::TemplateWithTwoParams<test_namespace_bindings::TemplateWithTwoParams<int, int>, int>"
    ),
    crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEE
);

impl Default for __CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b>
    From<
        ::ctor::RvalueReference<
            'b,
            crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEE,
        >,
    > for __CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEE
{
    #[inline(always)]
    fn from(
        __param_0: ::ctor::RvalueReference<
            'b,
            crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEE,
        >,
    ) -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEC1EOS2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(&mut tmp,__param_0);
            tmp.assume_init()
        }
    }
}

// rs_bindings_from_cc/test/golden/templates.h;l=37
// Error while generating bindings for item 'test_namespace_bindings::TemplateWithTwoParams<test_namespace_bindings::TemplateWithTwoParams<int, int>, int>::operator=':
// operator= for Unpin types is not yet supported.

// rs_bindings_from_cc/test/golden/templates.h;l=37
// Error while generating bindings for item 'test_namespace_bindings::TemplateWithTwoParams<test_namespace_bindings::TemplateWithTwoParams<int, int>, int>::operator=':
// operator= for Unpin types is not yet supported.

#[derive(Clone, Copy)]
#[repr(C)]
pub struct __CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIifEE {
    pub value1: i32,
    pub value2: f32,
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("test_namespace_bindings::TemplateWithTwoParams<int, float>"),
    crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIifEE
);

impl Default for __CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIifEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN23test_namespace_bindings21TemplateWithTwoParamsIifEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b>
    From<
        ::ctor::RvalueReference<
            'b,
            crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIifEE,
        >,
    > for __CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIifEE
{
    #[inline(always)]
    fn from(
        __param_0: ::ctor::RvalueReference<
            'b,
            crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIifEE,
        >,
    ) -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN23test_namespace_bindings21TemplateWithTwoParamsIifEC1EOS1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(&mut tmp,__param_0);
            tmp.assume_init()
        }
    }
}

// rs_bindings_from_cc/test/golden/templates.h;l=37
// Error while generating bindings for item 'test_namespace_bindings::TemplateWithTwoParams<int, float>::operator=':
// operator= for Unpin types is not yet supported.

// rs_bindings_from_cc/test/golden/templates.h;l=37
// Error while generating bindings for item 'test_namespace_bindings::TemplateWithTwoParams<int, float>::operator=':
// operator= for Unpin types is not yet supported.

#[derive(Clone, Copy)]
#[repr(C)]
pub struct __CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIiiEE {
    pub value1: i32,
    pub value2: i32,
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("test_namespace_bindings::TemplateWithTwoParams<int, int>"),
    crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIiiEE
);

impl Default for __CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIiiEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN23test_namespace_bindings21TemplateWithTwoParamsIiiEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b>
    From<
        ::ctor::RvalueReference<
            'b,
            crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIiiEE,
        >,
    > for __CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIiiEE
{
    #[inline(always)]
    fn from(
        __param_0: ::ctor::RvalueReference<
            'b,
            crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIiiEE,
        >,
    ) -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN23test_namespace_bindings21TemplateWithTwoParamsIiiEC1EOS1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(&mut tmp,__param_0);
            tmp.assume_init()
        }
    }
}

// rs_bindings_from_cc/test/golden/templates.h;l=37
// Error while generating bindings for item 'test_namespace_bindings::TemplateWithTwoParams<int, int>::operator=':
// operator= for Unpin types is not yet supported.

// rs_bindings_from_cc/test/golden/templates.h;l=37
// Error while generating bindings for item 'test_namespace_bindings::TemplateWithTwoParams<int, int>::operator=':
// operator= for Unpin types is not yet supported.

/// Explicit class template specialization with definition should be imported
/// even when not instantiated if there is a type alias for it.
#[::ctor::recursively_pinned]
#[repr(C)]
pub struct __CcTemplateInstN23test_namespace_bindings8MyStructIcEE {
    __non_field_data: [::std::mem::MaybeUninit<u8>; 1],
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("test_namespace_bindings::MyStruct<char>"),
    crate::__CcTemplateInstN23test_namespace_bindings8MyStructIcEE
);

impl ::ctor::CtorNew<()> for __CcTemplateInstN23test_namespace_bindings8MyStructIcEE {
    type CtorType = impl ::ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::std::pin::Pin<
                    &mut ::std::mem::MaybeUninit<
                        crate::__CcTemplateInstN23test_namespace_bindings8MyStructIcEE,
                    >,
                >| {
                    crate::detail::__rust_thunk___ZN23test_namespace_bindings8MyStructIcEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(::std::pin::Pin::into_inner_unchecked(dest));
                },
            )
        }
    }
}

impl<'b> ::ctor::CtorNew<&'b crate::__CcTemplateInstN23test_namespace_bindings8MyStructIcEE>
    for __CcTemplateInstN23test_namespace_bindings8MyStructIcEE
{
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(
        args: &'b crate::__CcTemplateInstN23test_namespace_bindings8MyStructIcEE,
    ) -> Self::CtorType {
        let __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::std::pin::Pin<
                    &mut ::std::mem::MaybeUninit<
                        crate::__CcTemplateInstN23test_namespace_bindings8MyStructIcEE,
                    >,
                >| {
                    crate::detail::__rust_thunk___ZN23test_namespace_bindings8MyStructIcEC1ERKS1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(::std::pin::Pin::into_inner_unchecked(dest),__param_0);
                },
            )
        }
    }
}
impl<'b> ::ctor::CtorNew<(&'b crate::__CcTemplateInstN23test_namespace_bindings8MyStructIcEE,)>
    for __CcTemplateInstN23test_namespace_bindings8MyStructIcEE
{
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(
        args: (&'b crate::__CcTemplateInstN23test_namespace_bindings8MyStructIcEE,),
    ) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<
            &'b crate::__CcTemplateInstN23test_namespace_bindings8MyStructIcEE,
        >>::ctor_new(arg)
    }
}

impl<'b>
    ::ctor::CtorNew<
        ::ctor::RvalueReference<'b, crate::__CcTemplateInstN23test_namespace_bindings8MyStructIcEE>,
    > for __CcTemplateInstN23test_namespace_bindings8MyStructIcEE
{
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(
        args: ::ctor::RvalueReference<
            'b,
            crate::__CcTemplateInstN23test_namespace_bindings8MyStructIcEE,
        >,
    ) -> Self::CtorType {
        let __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::std::pin::Pin<
                    &mut ::std::mem::MaybeUninit<
                        crate::__CcTemplateInstN23test_namespace_bindings8MyStructIcEE,
                    >,
                >| {
                    crate::detail::__rust_thunk___ZN23test_namespace_bindings8MyStructIcEC1EOS1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(::std::pin::Pin::into_inner_unchecked(dest),__param_0);
                },
            )
        }
    }
}
impl<'b>
    ::ctor::CtorNew<(
        ::ctor::RvalueReference<'b, crate::__CcTemplateInstN23test_namespace_bindings8MyStructIcEE>,
    )> for __CcTemplateInstN23test_namespace_bindings8MyStructIcEE
{
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(
        args: (
            ::ctor::RvalueReference<
                'b,
                crate::__CcTemplateInstN23test_namespace_bindings8MyStructIcEE,
            >,
        ),
    ) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<
            ::ctor::RvalueReference<
                'b,
                crate::__CcTemplateInstN23test_namespace_bindings8MyStructIcEE,
            >,
        >>::ctor_new(arg)
    }
}

impl<'b> ::ctor::Assign<&'b Self> for __CcTemplateInstN23test_namespace_bindings8MyStructIcEE {
    #[inline(always)]
    fn assign<'a>(self: ::std::pin::Pin<&'a mut Self>, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN23test_namespace_bindings8MyStructIcEaSERKS1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(self,__param_0);
        }
    }
}

impl<'b>
    ::ctor::Assign<
        ::ctor::RvalueReference<'b, crate::__CcTemplateInstN23test_namespace_bindings8MyStructIcEE>,
    > for __CcTemplateInstN23test_namespace_bindings8MyStructIcEE
{
    #[inline(always)]
    fn assign<'a>(
        self: ::std::pin::Pin<&'a mut Self>,
        __param_0: ::ctor::RvalueReference<
            'b,
            crate::__CcTemplateInstN23test_namespace_bindings8MyStructIcEE,
        >,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN23test_namespace_bindings8MyStructIcEaSEOS1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(self,__param_0);
        }
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct __CcTemplateInst18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEE {
    pub value: crate::test_namespace_bindings::TemplateParam,
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("MyTopLevelTemplate<test_namespace_bindings::TemplateParam>"),
    crate::__CcTemplateInst18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEE
);

impl Default for __CcTemplateInst18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b>
    From<
        ::ctor::RvalueReference<
            'b,
            crate::__CcTemplateInst18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEE,
        >,
    > for __CcTemplateInst18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEE
{
    #[inline(always)]
    fn from(
        __param_0: ::ctor::RvalueReference<
            'b,
            crate::__CcTemplateInst18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEE,
        >,
    ) -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEEC1EOS2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(&mut tmp,__param_0);
            tmp.assume_init()
        }
    }
}

// rs_bindings_from_cc/test/golden/templates.h;l=80
// Error while generating bindings for item 'MyTopLevelTemplate<test_namespace_bindings::TemplateParam>::operator=':
// operator= for Unpin types is not yet supported.

// rs_bindings_from_cc/test/golden/templates.h;l=80
// Error while generating bindings for item 'MyTopLevelTemplate<test_namespace_bindings::TemplateParam>::operator=':
// operator= for Unpin types is not yet supported.

forward_declare::forward_declare!(pub __CcTemplateInst18MyTopLevelTemplateIiE = forward_declare::symbol!("__CcTemplateInst18MyTopLevelTemplateIiE"));

#[::ctor::recursively_pinned]
#[repr(C)]
pub struct __CcTemplateInstN24template_template_params10MyTemplateINS_6PolicyEEE {
    __non_field_data: [::std::mem::MaybeUninit<u8>; 1],
}
forward_declare::unsafe_define!(
    forward_declare::symbol!(
        "template_template_params::MyTemplate<template_template_params::Policy>"
    ),
    crate::__CcTemplateInstN24template_template_params10MyTemplateINS_6PolicyEEE
);

impl ::ctor::CtorNew<()> for __CcTemplateInstN24template_template_params10MyTemplateINS_6PolicyEEE {
    type CtorType = impl ::ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(move|dest: ::std::pin::Pin<&mut::std::mem::MaybeUninit<crate::__CcTemplateInstN24template_template_params10MyTemplateINS_6PolicyEEE>>|{ crate::detail::__rust_thunk___ZN24template_template_params10MyTemplateINS_6PolicyEEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(::std::pin::Pin::into_inner_unchecked(dest)); })
        }
    }
}

impl<'b>
    ::ctor::CtorNew<
        &'b crate::__CcTemplateInstN24template_template_params10MyTemplateINS_6PolicyEEE,
    > for __CcTemplateInstN24template_template_params10MyTemplateINS_6PolicyEEE
{
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(
        args: &'b crate::__CcTemplateInstN24template_template_params10MyTemplateINS_6PolicyEEE,
    ) -> Self::CtorType {
        let __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(move|dest: ::std::pin::Pin<&mut::std::mem::MaybeUninit<crate::__CcTemplateInstN24template_template_params10MyTemplateINS_6PolicyEEE>>|{ crate::detail::__rust_thunk___ZN24template_template_params10MyTemplateINS_6PolicyEEC1ERKS2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(::std::pin::Pin::into_inner_unchecked(dest),__param_0); })
        }
    }
}
impl<'b>
    ::ctor::CtorNew<(
        &'b crate::__CcTemplateInstN24template_template_params10MyTemplateINS_6PolicyEEE,
    )> for __CcTemplateInstN24template_template_params10MyTemplateINS_6PolicyEEE
{
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(
        args: (&'b crate::__CcTemplateInstN24template_template_params10MyTemplateINS_6PolicyEEE,),
    ) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<
            &'b crate::__CcTemplateInstN24template_template_params10MyTemplateINS_6PolicyEEE,
        >>::ctor_new(arg)
    }
}

impl<'b>
    ::ctor::CtorNew<
        ::ctor::RvalueReference<
            'b,
            crate::__CcTemplateInstN24template_template_params10MyTemplateINS_6PolicyEEE,
        >,
    > for __CcTemplateInstN24template_template_params10MyTemplateINS_6PolicyEEE
{
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(
        args: ::ctor::RvalueReference<
            'b,
            crate::__CcTemplateInstN24template_template_params10MyTemplateINS_6PolicyEEE,
        >,
    ) -> Self::CtorType {
        let __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(move|dest: ::std::pin::Pin<&mut::std::mem::MaybeUninit<crate::__CcTemplateInstN24template_template_params10MyTemplateINS_6PolicyEEE>>|{ crate::detail::__rust_thunk___ZN24template_template_params10MyTemplateINS_6PolicyEEC1EOS2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(::std::pin::Pin::into_inner_unchecked(dest),__param_0); })
        }
    }
}
impl<'b>
    ::ctor::CtorNew<(
        ::ctor::RvalueReference<
            'b,
            crate::__CcTemplateInstN24template_template_params10MyTemplateINS_6PolicyEEE,
        >,
    )> for __CcTemplateInstN24template_template_params10MyTemplateINS_6PolicyEEE
{
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(
        args: (
            ::ctor::RvalueReference<
                'b,
                crate::__CcTemplateInstN24template_template_params10MyTemplateINS_6PolicyEEE,
            >,
        ),
    ) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<
            ::ctor::RvalueReference<
                'b,
                crate::__CcTemplateInstN24template_template_params10MyTemplateINS_6PolicyEEE,
            >,
        >>::ctor_new(arg)
    }
}

impl<'b> ::ctor::Assign<&'b Self>
    for __CcTemplateInstN24template_template_params10MyTemplateINS_6PolicyEEE
{
    #[inline(always)]
    fn assign<'a>(self: ::std::pin::Pin<&'a mut Self>, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN24template_template_params10MyTemplateINS_6PolicyEEaSERKS2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(self,__param_0);
        }
    }
}

impl<'b>
    ::ctor::Assign<
        ::ctor::RvalueReference<
            'b,
            crate::__CcTemplateInstN24template_template_params10MyTemplateINS_6PolicyEEE,
        >,
    > for __CcTemplateInstN24template_template_params10MyTemplateINS_6PolicyEEE
{
    #[inline(always)]
    fn assign<'a>(
        self: ::std::pin::Pin<&'a mut Self>,
        __param_0: ::ctor::RvalueReference<
            'b,
            crate::__CcTemplateInstN24template_template_params10MyTemplateINS_6PolicyEEE,
        >,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN24template_template_params10MyTemplateINS_6PolicyEEaSEOS2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(self,__param_0);
        }
    }
}

impl __CcTemplateInstN24template_template_params10MyTemplateINS_6PolicyEEE {
    #[inline(always)]
    pub fn GetPolicy() -> i32 {
        unsafe {
            crate::detail::__rust_thunk___ZN24template_template_params10MyTemplateINS_6PolicyEE9GetPolicyEv__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc()
        }
    }
}

forward_declare::forward_declare!(pub __CcTemplateInstN25forward_declared_template23ForwardDeclaredTemplateIiEE = forward_declare::symbol!("__CcTemplateInstN25forward_declared_template23ForwardDeclaredTemplateIiEE"));

mod detail {
    #[allow(unused_imports)]
    use super::*;
    extern "C" {
        pub(crate) fn __rust_thunk___ZN14DifferentScopeC1Ev<'a>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::DifferentScope>,
        );
        pub(crate) fn __rust_thunk___ZN14DifferentScopeC1EOS_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::DifferentScope>,
            __param_0: ::ctor::RvalueReference<'b, crate::DifferentScope>,
        );
        pub(crate) fn __rust_thunk___ZN23test_namespace_bindings13TemplateParamC1Ev<'a>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::test_namespace_bindings::TemplateParam>,
        );
        pub(crate) fn __rust_thunk___ZN23test_namespace_bindings13TemplateParamC1EOS0_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::test_namespace_bindings::TemplateParam>,
            __param_0: ::ctor::RvalueReference<'b, crate::test_namespace_bindings::TemplateParam>,
        );
        #[link_name = "_Z36processForwardDeclaredSpecializationP18MyTopLevelTemplateIiE"]
        pub(crate) fn __rust_thunk___Z36processForwardDeclaredSpecializationP18MyTopLevelTemplateIiE<
            'a,
        >(
            i: Option<::std::pin::Pin<&'a mut crate::__CcTemplateInst18MyTopLevelTemplateIiE>>,
        );
        pub(crate) fn __rust_thunk___ZN15private_classes14HasPrivateTypeC1ERKS0_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::private_classes::HasPrivateType>,
            __param_0: &'b crate::private_classes::HasPrivateType,
        );
        pub(crate) fn __rust_thunk___ZN15private_classes14HasPrivateTypeC1EOS0_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::private_classes::HasPrivateType>,
            __param_0: ::ctor::RvalueReference<'b, crate::private_classes::HasPrivateType>,
        );
        pub(crate) fn __rust_thunk___ZN15private_classes14HasPrivateTypeaSERKS0_<'a, 'b>(
            __this: ::std::pin::Pin<&'a mut crate::private_classes::HasPrivateType>,
            __param_0: &'b crate::private_classes::HasPrivateType,
        ) -> ::std::pin::Pin<&'a mut crate::private_classes::HasPrivateType>;
        pub(crate) fn __rust_thunk___ZN15private_classes14HasPrivateTypeaSEOS0_<'a, 'b>(
            __this: ::std::pin::Pin<&'a mut crate::private_classes::HasPrivateType>,
            __param_0: ::ctor::RvalueReference<'b, crate::private_classes::HasPrivateType>,
        ) -> ::std::pin::Pin<&'a mut crate::private_classes::HasPrivateType>;
        pub(crate) fn __rust_thunk___ZN23test_namespace_bindings10MyTemplateI14DifferentScopeEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc<
            'a,
        >(
            __this: &'a mut ::std::mem::MaybeUninit<
                crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateI14DifferentScopeEE,
            >,
        );
        pub(crate) fn __rust_thunk___ZN23test_namespace_bindings10MyTemplateI14DifferentScopeEC1EOS2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc<
            'a,
            'b,
        >(
            __this: &'a mut ::std::mem::MaybeUninit<
                crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateI14DifferentScopeEE,
            >,
            __param_0: ::ctor::RvalueReference<
                'b,
                crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateI14DifferentScopeEE,
            >,
        );
        pub(crate) fn __rust_thunk___ZN23test_namespace_bindings10MyTemplateI14DifferentScopeE6CreateES1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(
            value: crate::DifferentScope,
        ) -> crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateI14DifferentScopeEE;
        pub(crate) fn __rust_thunk___ZNK23test_namespace_bindings10MyTemplateI14DifferentScopeE5valueEv__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc<
            'a,
        >(
            __this:&'a crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateI14DifferentScopeEE,
        ) -> &'a crate::DifferentScope;
        pub(crate) fn __rust_thunk___ZN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc<
            'a,
        >(
            __this: &'a mut ::std::mem::MaybeUninit<
                crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEE,
            >,
        );
        pub(crate) fn __rust_thunk___ZN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEC1EOS2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc<
            'a,
            'b,
        >(
            __this: &'a mut ::std::mem::MaybeUninit<
                crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEE,
            >,
            __param_0: ::ctor::RvalueReference<
                'b,
                crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEE,
            >,
        );
        pub(crate) fn __rust_thunk___ZN23test_namespace_bindings10MyTemplateINS_13TemplateParamEE6CreateES1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(
            value: crate::test_namespace_bindings::TemplateParam,
        ) -> crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEE;
        pub(crate) fn __rust_thunk___ZNK23test_namespace_bindings10MyTemplateINS_13TemplateParamEE5valueEv__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc<
            'a,
        >(
            __this:&'a crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEE,
        ) -> &'a crate::test_namespace_bindings::TemplateParam;
        pub(crate) fn __rust_thunk___ZN23test_namespace_bindings10MyTemplateIiEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc<
            'a,
        >(
            __this: &'a mut ::std::mem::MaybeUninit<
                crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE,
            >,
        );
        pub(crate) fn __rust_thunk___ZN23test_namespace_bindings10MyTemplateIiEC1EOS1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc<
            'a,
            'b,
        >(
            __this: &'a mut ::std::mem::MaybeUninit<
                crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE,
            >,
            __param_0: ::ctor::RvalueReference<
                'b,
                crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE,
            >,
        );
        pub(crate) fn __rust_thunk___ZN23test_namespace_bindings10MyTemplateIiE6CreateEi__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(
            value: i32,
        ) -> crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE;
        pub(crate) fn __rust_thunk___ZNK23test_namespace_bindings10MyTemplateIiE5valueEv__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc<
            'a,
        >(
            __this: &'a crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE,
        ) -> &'a i32;
        pub(crate) fn __rust_thunk___ZN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc<
            'a,
        >(
            __this:&'a mut::std::mem::MaybeUninit<crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEE>,
        );
        pub(crate) fn __rust_thunk___ZN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEC1EOS2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc<
            'a,
            'b,
        >(
            __this:&'a mut::std::mem::MaybeUninit<crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEE>,
            __param_0: ::ctor::RvalueReference<'b,crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEE>,
        );
        pub(crate) fn __rust_thunk___ZN23test_namespace_bindings21TemplateWithTwoParamsIifEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc<
            'a,
        >(
            __this: &'a mut ::std::mem::MaybeUninit<
                crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIifEE,
            >,
        );
        pub(crate) fn __rust_thunk___ZN23test_namespace_bindings21TemplateWithTwoParamsIifEC1EOS1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc<
            'a,
            'b,
        >(
            __this: &'a mut ::std::mem::MaybeUninit<
                crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIifEE,
            >,
            __param_0: ::ctor::RvalueReference<
                'b,
                crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIifEE,
            >,
        );
        pub(crate) fn __rust_thunk___ZN23test_namespace_bindings21TemplateWithTwoParamsIiiEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc<
            'a,
        >(
            __this: &'a mut ::std::mem::MaybeUninit<
                crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIiiEE,
            >,
        );
        pub(crate) fn __rust_thunk___ZN23test_namespace_bindings21TemplateWithTwoParamsIiiEC1EOS1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc<
            'a,
            'b,
        >(
            __this: &'a mut ::std::mem::MaybeUninit<
                crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIiiEE,
            >,
            __param_0: ::ctor::RvalueReference<
                'b,
                crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIiiEE,
            >,
        );
        pub(crate) fn __rust_thunk___ZN23test_namespace_bindings8MyStructIcEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc<
            'a,
        >(
            __this: &'a mut ::std::mem::MaybeUninit<
                crate::__CcTemplateInstN23test_namespace_bindings8MyStructIcEE,
            >,
        );
        pub(crate) fn __rust_thunk___ZN23test_namespace_bindings8MyStructIcEC1ERKS1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc<
            'a,
            'b,
        >(
            __this: &'a mut ::std::mem::MaybeUninit<
                crate::__CcTemplateInstN23test_namespace_bindings8MyStructIcEE,
            >,
            __param_0: &'b crate::__CcTemplateInstN23test_namespace_bindings8MyStructIcEE,
        );
        pub(crate) fn __rust_thunk___ZN23test_namespace_bindings8MyStructIcEC1EOS1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc<
            'a,
            'b,
        >(
            __this: &'a mut ::std::mem::MaybeUninit<
                crate::__CcTemplateInstN23test_namespace_bindings8MyStructIcEE,
            >,
            __param_0: ::ctor::RvalueReference<
                'b,
                crate::__CcTemplateInstN23test_namespace_bindings8MyStructIcEE,
            >,
        );
        pub(crate) fn __rust_thunk___ZN23test_namespace_bindings8MyStructIcEaSERKS1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc<
            'a,
            'b,
        >(
            __this: ::std::pin::Pin<
                &'a mut crate::__CcTemplateInstN23test_namespace_bindings8MyStructIcEE,
            >,
            __param_0: &'b crate::__CcTemplateInstN23test_namespace_bindings8MyStructIcEE,
        ) -> ::std::pin::Pin<&'a mut crate::__CcTemplateInstN23test_namespace_bindings8MyStructIcEE>;
        pub(crate) fn __rust_thunk___ZN23test_namespace_bindings8MyStructIcEaSEOS1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc<
            'a,
            'b,
        >(
            __this: ::std::pin::Pin<
                &'a mut crate::__CcTemplateInstN23test_namespace_bindings8MyStructIcEE,
            >,
            __param_0: ::ctor::RvalueReference<
                'b,
                crate::__CcTemplateInstN23test_namespace_bindings8MyStructIcEE,
            >,
        ) -> ::std::pin::Pin<&'a mut crate::__CcTemplateInstN23test_namespace_bindings8MyStructIcEE>;
        pub(crate) fn __rust_thunk___ZN18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc<
            'a,
        >(
            __this:&'a mut::std::mem::MaybeUninit<crate::__CcTemplateInst18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEE>,
        );
        pub(crate) fn __rust_thunk___ZN18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEEC1EOS2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc<
            'a,
            'b,
        >(
            __this:&'a mut::std::mem::MaybeUninit<crate::__CcTemplateInst18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEE>,
            __param_0: ::ctor::RvalueReference<'b,crate::__CcTemplateInst18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEE>,
        );
        pub(crate) fn __rust_thunk___ZN24template_template_params10MyTemplateINS_6PolicyEEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc<
            'a,
        >(
            __this: &'a mut ::std::mem::MaybeUninit<
                crate::__CcTemplateInstN24template_template_params10MyTemplateINS_6PolicyEEE,
            >,
        );
        pub(crate) fn __rust_thunk___ZN24template_template_params10MyTemplateINS_6PolicyEEC1ERKS2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc<
            'a,
            'b,
        >(
            __this: &'a mut ::std::mem::MaybeUninit<
                crate::__CcTemplateInstN24template_template_params10MyTemplateINS_6PolicyEEE,
            >,
            __param_0:&'b crate::__CcTemplateInstN24template_template_params10MyTemplateINS_6PolicyEEE,
        );
        pub(crate) fn __rust_thunk___ZN24template_template_params10MyTemplateINS_6PolicyEEC1EOS2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc<
            'a,
            'b,
        >(
            __this: &'a mut ::std::mem::MaybeUninit<
                crate::__CcTemplateInstN24template_template_params10MyTemplateINS_6PolicyEEE,
            >,
            __param_0: ::ctor::RvalueReference<
                'b,
                crate::__CcTemplateInstN24template_template_params10MyTemplateINS_6PolicyEEE,
            >,
        );
        pub(crate) fn __rust_thunk___ZN24template_template_params10MyTemplateINS_6PolicyEEaSERKS2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc<
            'a,
            'b,
        >(
            __this: ::std::pin::Pin<&'a mut crate::__CcTemplateInstN24template_template_params10MyTemplateINS_6PolicyEEE>,
            __param_0:&'b crate::__CcTemplateInstN24template_template_params10MyTemplateINS_6PolicyEEE,
        ) -> ::std::pin::Pin<
            &'a mut crate::__CcTemplateInstN24template_template_params10MyTemplateINS_6PolicyEEE,
        >;
        pub(crate) fn __rust_thunk___ZN24template_template_params10MyTemplateINS_6PolicyEEaSEOS2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc<
            'a,
            'b,
        >(
            __this: ::std::pin::Pin<&'a mut crate::__CcTemplateInstN24template_template_params10MyTemplateINS_6PolicyEEE>,
            __param_0: ::ctor::RvalueReference<
                'b,
                crate::__CcTemplateInstN24template_template_params10MyTemplateINS_6PolicyEEE,
            >,
        ) -> ::std::pin::Pin<
            &'a mut crate::__CcTemplateInstN24template_template_params10MyTemplateINS_6PolicyEEE,
        >;
        pub(crate) fn __rust_thunk___ZN24template_template_params10MyTemplateINS_6PolicyEE9GetPolicyEv__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc()
        -> i32;
    }
}

const _: () = assert!(::std::mem::size_of::<Option<&i32>>() == ::std::mem::size_of::<&i32>());

const _: () = assert!(::std::mem::size_of::<crate::DifferentScope>() == 1);
const _: () = assert!(::std::mem::align_of::<crate::DifferentScope>() == 1);
const _: () = {
    static_assertions::assert_impl_all!(crate::DifferentScope: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::DifferentScope: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::DifferentScope: Drop);
};

const _: () = assert!(::std::mem::size_of::<crate::test_namespace_bindings::TemplateParam>() == 1);
const _: () = assert!(::std::mem::align_of::<crate::test_namespace_bindings::TemplateParam>() == 1);
const _: () = {
    static_assertions::assert_impl_all!(crate::test_namespace_bindings::TemplateParam: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::test_namespace_bindings::TemplateParam: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::test_namespace_bindings::TemplateParam: Drop);
};

const _: () = assert!(::std::mem::size_of::<crate::private_classes::HasPrivateType>() == 1);
const _: () = assert!(::std::mem::align_of::<crate::private_classes::HasPrivateType>() == 1);
const _: () = {
    static_assertions::assert_not_impl_any!(crate::private_classes::HasPrivateType: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::private_classes::HasPrivateType: Drop);
};

const _: () = assert!(
    ::std::mem::size_of::<
        crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateI14DifferentScopeEE,
    >() == 1
);
const _: () = assert!(
    ::std::mem::align_of::<
        crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateI14DifferentScopeEE,
    >() == 1
);
const _: () = {
    static_assertions::assert_impl_all!(
        crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateI14DifferentScopeEE: Clone
    );
};
const _: () = {
    static_assertions::assert_impl_all!(
        crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateI14DifferentScopeEE: Copy
    );
};
const _: () = {
    static_assertions::assert_not_impl_any!(
        crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateI14DifferentScopeEE: Drop
    );
};
const _: () = assert!(
    memoffset::offset_of!(
        crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateI14DifferentScopeEE,
        value_
    ) == 0
);

const _: () = assert!(
    ::std::mem::size_of::<
        crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEE,
    >() == 1
);
const _: () = assert!(
    ::std::mem::align_of::<
        crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEE,
    >() == 1
);
const _: () = {
    static_assertions::assert_impl_all!(
        crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEE: Clone
    );
};
const _: () = {
    static_assertions::assert_impl_all!(
        crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEE: Copy
    );
};
const _: () = {
    static_assertions::assert_not_impl_any!(
        crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEE: Drop
    );
};
const _: () = assert!(
    memoffset::offset_of!(
        crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEE,
        value_
    ) == 0
);

const _: () = assert!(
    ::std::mem::size_of::<crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE>() == 4
);
const _: () = assert!(
    ::std::mem::align_of::<crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE>()
        == 4
);
const _: () = {
    static_assertions::assert_impl_all!(
        crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE: Clone
    );
};
const _: () = {
    static_assertions::assert_impl_all!(
        crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE: Copy
    );
};
const _: () = {
    static_assertions::assert_not_impl_any!(
        crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE: Drop
    );
};
const _: () = assert!(
    memoffset::offset_of!(
        crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE,
        value_
    ) == 0
);

const _: () = assert!(
    ::std::mem::size_of::<
        crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEE,
    >() == 12
);
const _: () = assert!(
    ::std::mem::align_of::<
        crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEE,
    >() == 4
);
const _: () = {
    static_assertions::assert_impl_all!(
        crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEE:
            Clone
    );
};
const _: () = {
    static_assertions::assert_impl_all!(
        crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEE: Copy
    );
};
const _: () = {
    static_assertions::assert_not_impl_any!(
        crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEE: Drop
    );
};
const _: () = assert!(
    memoffset::offset_of!(
        crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEE,
        value1
    ) == 0
);
const _: () = assert!(
    memoffset::offset_of!(
        crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEE,
        value2
    ) == 8
);

const _: () = assert!(
    ::std::mem::size_of::<
        crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIifEE,
    >() == 8
);
const _: () = assert!(
    ::std::mem::align_of::<
        crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIifEE,
    >() == 4
);
const _: () = {
    static_assertions::assert_impl_all!(
        crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIifEE: Clone
    );
};
const _: () = {
    static_assertions::assert_impl_all!(
        crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIifEE: Copy
    );
};
const _: () = {
    static_assertions::assert_not_impl_any!(
        crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIifEE: Drop
    );
};
const _: () = assert!(
    memoffset::offset_of!(
        crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIifEE,
        value1
    ) == 0
);
const _: () = assert!(
    memoffset::offset_of!(
        crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIifEE,
        value2
    ) == 4
);

const _: () = assert!(
    ::std::mem::size_of::<
        crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIiiEE,
    >() == 8
);
const _: () = assert!(
    ::std::mem::align_of::<
        crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIiiEE,
    >() == 4
);
const _: () = {
    static_assertions::assert_impl_all!(
        crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIiiEE: Clone
    );
};
const _: () = {
    static_assertions::assert_impl_all!(
        crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIiiEE: Copy
    );
};
const _: () = {
    static_assertions::assert_not_impl_any!(
        crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIiiEE: Drop
    );
};
const _: () = assert!(
    memoffset::offset_of!(
        crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIiiEE,
        value1
    ) == 0
);
const _: () = assert!(
    memoffset::offset_of!(
        crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIiiEE,
        value2
    ) == 4
);

const _: () = assert!(
    ::std::mem::size_of::<crate::__CcTemplateInstN23test_namespace_bindings8MyStructIcEE>() == 1
);
const _: () = assert!(
    ::std::mem::align_of::<crate::__CcTemplateInstN23test_namespace_bindings8MyStructIcEE>() == 1
);
const _: () = {
    static_assertions::assert_not_impl_any!(
        crate::__CcTemplateInstN23test_namespace_bindings8MyStructIcEE: Copy
    );
};
const _: () = {
    static_assertions::assert_not_impl_any!(
        crate::__CcTemplateInstN23test_namespace_bindings8MyStructIcEE: Drop
    );
};

const _: () = assert!(
    ::std::mem::size_of::<
        crate::__CcTemplateInst18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEE,
    >() == 1
);
const _: () = assert!(
    ::std::mem::align_of::<
        crate::__CcTemplateInst18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEE,
    >() == 1
);
const _: () = {
    static_assertions::assert_impl_all!(
        crate::__CcTemplateInst18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEE:
            Clone
    );
};
const _: () = {
    static_assertions::assert_impl_all!(
        crate::__CcTemplateInst18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEE:
            Copy
    );
};
const _: () = {
    static_assertions::assert_not_impl_any!(
        crate::__CcTemplateInst18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEE:
            Drop
    );
};
const _: () = assert!(
    memoffset::offset_of!(
        crate::__CcTemplateInst18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEE,
        value
    ) == 0
);

const _: () = assert!(
    ::std::mem::size_of::<
        crate::__CcTemplateInstN24template_template_params10MyTemplateINS_6PolicyEEE,
    >() == 1
);
const _: () = assert!(
    ::std::mem::align_of::<
        crate::__CcTemplateInstN24template_template_params10MyTemplateINS_6PolicyEEE,
    >() == 1
);
const _: () = {
    static_assertions::assert_not_impl_any!(
        crate::__CcTemplateInstN24template_template_params10MyTemplateINS_6PolicyEEE: Copy
    );
};
const _: () = {
    static_assertions::assert_not_impl_any!(
        crate::__CcTemplateInstN24template_template_params10MyTemplateINS_6PolicyEEE: Drop
    );
};
