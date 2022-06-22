// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for C++ target
// //rs_bindings_from_cc/test/golden:templates_cc
#![rustfmt::skip]
#![feature(const_ptr_offset_from, custom_inner_attributes, negative_impls, type_alias_impl_trait)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![deny(warnings)]

use ::std as rust_std;

// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[ctor::recursively_pinned]
#[repr(C)]
pub struct DifferentScope {
    __non_field_data: [crate::rust_std::mem::MaybeUninit<u8>; 1],
}
forward_declare::unsafe_define!(forward_declare::symbol!("DifferentScope"), crate::DifferentScope);

impl ctor::CtorNew<()> for DifferentScope {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        ctor::FnCtor::new(
            move |dest: crate::rust_std::pin::Pin<&mut crate::rust_std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN14DifferentScopeC1Ev(
                    crate::rust_std::pin::Pin::into_inner_unchecked(dest),
                );
            },
        )
    }
}

impl<'b> ctor::CtorNew<&'b crate::DifferentScope> for DifferentScope {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: &'b crate::DifferentScope) -> Self::CtorType {
        let __param_0 = args;
        ctor::FnCtor::new(
            move |dest: crate::rust_std::pin::Pin<&mut crate::rust_std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN14DifferentScopeC1ERKS_(
                    crate::rust_std::pin::Pin::into_inner_unchecked(dest),
                    __param_0,
                );
            },
        )
    }
}
impl<'b> ctor::CtorNew<(&'b crate::DifferentScope,)> for DifferentScope {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: (&'b crate::DifferentScope,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ctor::CtorNew<&'b crate::DifferentScope>>::ctor_new(arg)
    }
}

impl<'b> ctor::CtorNew<ctor::RvalueReference<'b, crate::DifferentScope>> for DifferentScope {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ctor::RvalueReference<'b, crate::DifferentScope>) -> Self::CtorType {
        let __param_0 = args;
        ctor::FnCtor::new(
            move |dest: crate::rust_std::pin::Pin<&mut crate::rust_std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN14DifferentScopeC1EOS_(
                    crate::rust_std::pin::Pin::into_inner_unchecked(dest),
                    __param_0,
                );
            },
        )
    }
}
impl<'b> ctor::CtorNew<(ctor::RvalueReference<'b, crate::DifferentScope>,)> for DifferentScope {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: (ctor::RvalueReference<'b, crate::DifferentScope>,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ctor::CtorNew<ctor::RvalueReference<'b, crate::DifferentScope>>>::ctor_new(arg)
    }
}

impl<'b> ::ctor::Assign<&'b crate::DifferentScope> for DifferentScope {
    #[inline(always)]
    fn assign<'a>(
        self: crate::rust_std::pin::Pin<&'a mut Self>,
        __param_0: &'b crate::DifferentScope,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN14DifferentScopeaSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::Assign<ctor::RvalueReference<'b, crate::DifferentScope>> for DifferentScope {
    #[inline(always)]
    fn assign<'a>(
        self: crate::rust_std::pin::Pin<&'a mut Self>,
        __param_0: ctor::RvalueReference<'b, crate::DifferentScope>,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN14DifferentScopeaSEOS_(self, __param_0);
        }
    }
}

pub mod test_namespace_bindings {
    // rs_bindings_from_cc/test/golden/templates.h;l=14
    // Error while generating bindings for item 'test_namespace_bindings::MyTemplate':
    // Class templates are not supported yet

    pub type MyTypeAlias = crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE;

    pub type OtherTypeAliasInSameTarget =
        crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE;

    #[ctor::recursively_pinned]
    #[repr(C)]
    pub struct TemplateParam {
        __non_field_data: [crate::rust_std::mem::MaybeUninit<u8>; 1],
    }
    forward_declare::unsafe_define!(
        forward_declare::symbol!("TemplateParam"),
        crate::test_namespace_bindings::TemplateParam
    );

    impl ctor::CtorNew<()> for TemplateParam {
        type CtorType = impl ctor::Ctor<Output = Self>;
        #[inline(always)]
        fn ctor_new(args: ()) -> Self::CtorType {
            let () = args;
            ctor::FnCtor::new(
                move |dest: crate::rust_std::pin::Pin<
                    &mut crate::rust_std::mem::MaybeUninit<Self>,
                >| {
                    unsafe {
                        crate::detail::__rust_thunk___ZN23test_namespace_bindings13TemplateParamC1Ev(crate::rust_std::pin::Pin::into_inner_unchecked(dest));
                    }
                },
            )
        }
    }

    impl<'b> ctor::CtorNew<&'b crate::test_namespace_bindings::TemplateParam> for TemplateParam {
        type CtorType = impl ctor::Ctor<Output = Self>;
        #[inline(always)]
        fn ctor_new(args: &'b crate::test_namespace_bindings::TemplateParam) -> Self::CtorType {
            let __param_0 = args;
            ctor::FnCtor::new(
                move |dest: crate::rust_std::pin::Pin<
                    &mut crate::rust_std::mem::MaybeUninit<Self>,
                >| {
                    unsafe {
                        crate::detail::__rust_thunk___ZN23test_namespace_bindings13TemplateParamC1ERKS0_(crate::rust_std::pin::Pin::into_inner_unchecked(dest),__param_0);
                    }
                },
            )
        }
    }
    impl<'b> ctor::CtorNew<(&'b crate::test_namespace_bindings::TemplateParam,)> for TemplateParam {
        type CtorType = impl ctor::Ctor<Output = Self>;
        #[inline(always)]
        fn ctor_new(args: (&'b crate::test_namespace_bindings::TemplateParam,)) -> Self::CtorType {
            let (arg,) = args;
            <Self as ctor::CtorNew<&'b crate::test_namespace_bindings::TemplateParam>>::ctor_new(
                arg,
            )
        }
    }

    impl<'b> ctor::CtorNew<ctor::RvalueReference<'b, crate::test_namespace_bindings::TemplateParam>>
        for TemplateParam
    {
        type CtorType = impl ctor::Ctor<Output = Self>;
        #[inline(always)]
        fn ctor_new(
            args: ctor::RvalueReference<'b, crate::test_namespace_bindings::TemplateParam>,
        ) -> Self::CtorType {
            let __param_0 = args;
            ctor::FnCtor::new(
                move |dest: crate::rust_std::pin::Pin<
                    &mut crate::rust_std::mem::MaybeUninit<Self>,
                >| {
                    unsafe {
                        crate::detail::__rust_thunk___ZN23test_namespace_bindings13TemplateParamC1EOS0_(crate::rust_std::pin::Pin::into_inner_unchecked(dest),__param_0);
                    }
                },
            )
        }
    }
    impl<'b>
        ctor::CtorNew<(ctor::RvalueReference<'b, crate::test_namespace_bindings::TemplateParam>,)>
        for TemplateParam
    {
        type CtorType = impl ctor::Ctor<Output = Self>;
        #[inline(always)]
        fn ctor_new(
            args: (ctor::RvalueReference<'b, crate::test_namespace_bindings::TemplateParam>,),
        ) -> Self::CtorType {
            let (arg,) = args;
            <Self as ctor::CtorNew<
                ctor::RvalueReference<'b, crate::test_namespace_bindings::TemplateParam>,
            >>::ctor_new(arg)
        }
    }

    impl<'b> ::ctor::Assign<&'b crate::test_namespace_bindings::TemplateParam> for TemplateParam {
        #[inline(always)]
        fn assign<'a>(
            self: crate::rust_std::pin::Pin<&'a mut Self>,
            __param_0: &'b crate::test_namespace_bindings::TemplateParam,
        ) {
            unsafe {
                crate::detail::__rust_thunk___ZN23test_namespace_bindings13TemplateParamaSERKS0_(
                    self, __param_0,
                );
            }
        }
    }

    impl<'b>
        ::ctor::Assign<ctor::RvalueReference<'b, crate::test_namespace_bindings::TemplateParam>>
        for TemplateParam
    {
        #[inline(always)]
        fn assign<'a>(
            self: crate::rust_std::pin::Pin<&'a mut Self>,
            __param_0: ctor::RvalueReference<'b, crate::test_namespace_bindings::TemplateParam>,
        ) {
            unsafe {
                crate::detail::__rust_thunk___ZN23test_namespace_bindings13TemplateParamaSEOS0_(
                    self, __param_0,
                );
            }
        }
    }

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
}

// namespace test_namespace_bindings

// rs_bindings_from_cc/test/golden/templates.h;l=49
// Error while generating bindings for item 'MyTopLevelTemplate':
// Class templates are not supported yet

pub type TopLevelTemplateWithNonTopLevelParam =
    crate::__CcTemplateInst18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEE;

forward_declare::forward_declare!(pub __CcTemplateInst18MyTopLevelTemplateIiE = forward_declare::symbol!("__CcTemplateInst18MyTopLevelTemplateIiE"));

#[inline(always)]
pub fn processForwardDeclaredSpecialization<'a>(
    i: Option<crate::rust_std::pin::Pin<&'a mut crate::__CcTemplateInst18MyTopLevelTemplateIiE>>,
) {
    unsafe {
        crate::detail::__rust_thunk___Z36processForwardDeclaredSpecializationP18MyTopLevelTemplateIiE(i)
    }
}

// THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_TEMPLATES_H_

#[ctor::recursively_pinned]
#[repr(C)]
pub struct __CcTemplateInstN23test_namespace_bindings10MyTemplateI14DifferentScopeEE {
    __non_field_data: [crate::rust_std::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) value_: [crate::rust_std::mem::MaybeUninit<u8>; 1],
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("test_namespace_bindings::MyTemplate<DifferentScope>"),
    crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateI14DifferentScopeEE
);

impl ctor::CtorNew<()>
    for __CcTemplateInstN23test_namespace_bindings10MyTemplateI14DifferentScopeEE
{
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        ctor::FnCtor::new(
            move |dest: crate::rust_std::pin::Pin<&mut crate::rust_std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN23test_namespace_bindings10MyTemplateI14DifferentScopeEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(crate::rust_std::pin::Pin::into_inner_unchecked(dest));
            },
        )
    }
}

impl<'b>
    ctor::CtorNew<
        &'b crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateI14DifferentScopeEE,
    > for __CcTemplateInstN23test_namespace_bindings10MyTemplateI14DifferentScopeEE
{
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(
        args: &'b crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateI14DifferentScopeEE,
    ) -> Self::CtorType {
        let __param_0 = args;
        ctor::FnCtor::new(
            move |dest: crate::rust_std::pin::Pin<&mut crate::rust_std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN23test_namespace_bindings10MyTemplateI14DifferentScopeEC1ERKS2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(crate::rust_std::pin::Pin::into_inner_unchecked(dest),__param_0);
            },
        )
    }
}
impl<'b>
    ctor::CtorNew<(
        &'b crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateI14DifferentScopeEE,
    )> for __CcTemplateInstN23test_namespace_bindings10MyTemplateI14DifferentScopeEE
{
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(
        args: (
            &'b crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateI14DifferentScopeEE,
        ),
    ) -> Self::CtorType {
        let (arg,) = args;
        <Self as ctor::CtorNew<
            &'b crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateI14DifferentScopeEE,
        >>::ctor_new(arg)
    }
}

impl<'b>
    ctor::CtorNew<
        ctor::RvalueReference<
            'b,
            crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateI14DifferentScopeEE,
        >,
    > for __CcTemplateInstN23test_namespace_bindings10MyTemplateI14DifferentScopeEE
{
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(
        args: ctor::RvalueReference<
            'b,
            crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateI14DifferentScopeEE,
        >,
    ) -> Self::CtorType {
        let __param_0 = args;
        ctor::FnCtor::new(
            move |dest: crate::rust_std::pin::Pin<&mut crate::rust_std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN23test_namespace_bindings10MyTemplateI14DifferentScopeEC1EOS2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(crate::rust_std::pin::Pin::into_inner_unchecked(dest),__param_0);
            },
        )
    }
}
impl<'b>
    ctor::CtorNew<(
        ctor::RvalueReference<
            'b,
            crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateI14DifferentScopeEE,
        >,
    )> for __CcTemplateInstN23test_namespace_bindings10MyTemplateI14DifferentScopeEE
{
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(
        args: (
            ctor::RvalueReference<
                'b,
                crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateI14DifferentScopeEE,
            >,
        ),
    ) -> Self::CtorType {
        let (arg,) = args;
        <Self as ctor::CtorNew<
            ctor::RvalueReference<
                'b,
                crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateI14DifferentScopeEE,
            >,
        >>::ctor_new(arg)
    }
}

impl<'b>
    ::ctor::Assign<
        &'b crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateI14DifferentScopeEE,
    > for __CcTemplateInstN23test_namespace_bindings10MyTemplateI14DifferentScopeEE
{
    #[inline(always)]
    fn assign<'a>(
        self: crate::rust_std::pin::Pin<&'a mut Self>,
        __param_0:&'b crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateI14DifferentScopeEE,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN23test_namespace_bindings10MyTemplateI14DifferentScopeEaSERKS2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(self,__param_0);
        }
    }
}

impl<'b>
    ::ctor::Assign<
        ctor::RvalueReference<
            'b,
            crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateI14DifferentScopeEE,
        >,
    > for __CcTemplateInstN23test_namespace_bindings10MyTemplateI14DifferentScopeEE
{
    #[inline(always)]
    fn assign<'a>(
        self: crate::rust_std::pin::Pin<&'a mut Self>,
        __param_0: ctor::RvalueReference<
            'b,
            crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateI14DifferentScopeEE,
        >,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN23test_namespace_bindings10MyTemplateI14DifferentScopeEaSEOS2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(self,__param_0);
        }
    }
}

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

#[ctor::recursively_pinned]
#[repr(C)]
pub struct __CcTemplateInstN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEE {
    __non_field_data: [crate::rust_std::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) value_: [crate::rust_std::mem::MaybeUninit<u8>; 1],
}
forward_declare::unsafe_define!(
    forward_declare::symbol!(
        "test_namespace_bindings::MyTemplate<test_namespace_bindings::TemplateParam>"
    ),
    crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEE
);

impl ctor::CtorNew<()>
    for __CcTemplateInstN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEE
{
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        ctor::FnCtor::new(
            move |dest: crate::rust_std::pin::Pin<&mut crate::rust_std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(crate::rust_std::pin::Pin::into_inner_unchecked(dest));
            },
        )
    }
}

impl<'b>
    ctor::CtorNew<
        &'b crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEE,
    > for __CcTemplateInstN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEE
{
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(
        args:&'b crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEE,
    ) -> Self::CtorType {
        let __param_0 = args;
        ctor::FnCtor::new(
            move |dest: crate::rust_std::pin::Pin<&mut crate::rust_std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEC1ERKS2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(crate::rust_std::pin::Pin::into_inner_unchecked(dest),__param_0);
            },
        )
    }
}
impl<'b>
    ctor::CtorNew<(
        &'b crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEE,
    )> for __CcTemplateInstN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEE
{
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(
        args: (
            &'b crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEE,
        ),
    ) -> Self::CtorType {
        let (arg,) = args;
        <Self as ctor::CtorNew<
            &'b crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEE,
        >>::ctor_new(arg)
    }
}

impl<'b>
    ctor::CtorNew<
        ctor::RvalueReference<
            'b,
            crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEE,
        >,
    > for __CcTemplateInstN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEE
{
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(
        args: ctor::RvalueReference<
            'b,
            crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEE,
        >,
    ) -> Self::CtorType {
        let __param_0 = args;
        ctor::FnCtor::new(
            move |dest: crate::rust_std::pin::Pin<&mut crate::rust_std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEC1EOS2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(crate::rust_std::pin::Pin::into_inner_unchecked(dest),__param_0);
            },
        )
    }
}
impl<'b>
    ctor::CtorNew<(
        ctor::RvalueReference<
            'b,
            crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEE,
        >,
    )> for __CcTemplateInstN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEE
{
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(
        args: (
            ctor::RvalueReference<
                'b,
                crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEE,
            >,
        ),
    ) -> Self::CtorType {
        let (arg,) = args;
        <Self as ctor::CtorNew<
            ctor::RvalueReference<
                'b,
                crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEE,
            >,
        >>::ctor_new(arg)
    }
}

impl<'b>
    ::ctor::Assign<
        &'b crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEE,
    > for __CcTemplateInstN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEE
{
    #[inline(always)]
    fn assign<'a>(
        self: crate::rust_std::pin::Pin<&'a mut Self>,
        __param_0:&'b crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEE,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEaSERKS2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(self,__param_0);
        }
    }
}

impl<'b>
    ::ctor::Assign<
        ctor::RvalueReference<
            'b,
            crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEE,
        >,
    > for __CcTemplateInstN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEE
{
    #[inline(always)]
    fn assign<'a>(
        self: crate::rust_std::pin::Pin<&'a mut Self>,
        __param_0: ctor::RvalueReference<
            'b,
            crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEE,
        >,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEaSEOS2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(self,__param_0);
        }
    }
}

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

#[ctor::recursively_pinned]
#[repr(C, align(4))]
pub struct __CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE {
    __non_field_data: [crate::rust_std::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) value_: [crate::rust_std::mem::MaybeUninit<u8>; 4],
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("test_namespace_bindings::MyTemplate<int>"),
    crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE
);

impl ctor::CtorNew<()> for __CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        ctor::FnCtor::new(
            move |dest: crate::rust_std::pin::Pin<&mut crate::rust_std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN23test_namespace_bindings10MyTemplateIiEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(crate::rust_std::pin::Pin::into_inner_unchecked(dest));
            },
        )
    }
}

impl<'b> ctor::CtorNew<&'b crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE>
    for __CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE
{
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(
        args: &'b crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE,
    ) -> Self::CtorType {
        let __param_0 = args;
        ctor::FnCtor::new(
            move |dest: crate::rust_std::pin::Pin<&mut crate::rust_std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN23test_namespace_bindings10MyTemplateIiEC1ERKS1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(crate::rust_std::pin::Pin::into_inner_unchecked(dest),__param_0);
            },
        )
    }
}
impl<'b> ctor::CtorNew<(&'b crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE,)>
    for __CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE
{
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(
        args: (&'b crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE,),
    ) -> Self::CtorType {
        let (arg,) = args;
        <Self as ctor::CtorNew<
            &'b crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE,
        >>::ctor_new(arg)
    }
}

impl<'b>
    ctor::CtorNew<
        ctor::RvalueReference<
            'b,
            crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE,
        >,
    > for __CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE
{
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(
        args: ctor::RvalueReference<
            'b,
            crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE,
        >,
    ) -> Self::CtorType {
        let __param_0 = args;
        ctor::FnCtor::new(
            move |dest: crate::rust_std::pin::Pin<&mut crate::rust_std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN23test_namespace_bindings10MyTemplateIiEC1EOS1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(crate::rust_std::pin::Pin::into_inner_unchecked(dest),__param_0);
            },
        )
    }
}
impl<'b>
    ctor::CtorNew<(
        ctor::RvalueReference<
            'b,
            crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE,
        >,
    )> for __CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE
{
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(
        args: (
            ctor::RvalueReference<
                'b,
                crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE,
            >,
        ),
    ) -> Self::CtorType {
        let (arg,) = args;
        <Self as ctor::CtorNew<
            ctor::RvalueReference<
                'b,
                crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE,
            >,
        >>::ctor_new(arg)
    }
}

impl<'b> ::ctor::Assign<&'b crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE>
    for __CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE
{
    #[inline(always)]
    fn assign<'a>(
        self: crate::rust_std::pin::Pin<&'a mut Self>,
        __param_0: &'b crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN23test_namespace_bindings10MyTemplateIiEaSERKS1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(self,__param_0);
        }
    }
}

impl<'b>
    ::ctor::Assign<
        ctor::RvalueReference<
            'b,
            crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE,
        >,
    > for __CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE
{
    #[inline(always)]
    fn assign<'a>(
        self: crate::rust_std::pin::Pin<&'a mut Self>,
        __param_0: ctor::RvalueReference<
            'b,
            crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE,
        >,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN23test_namespace_bindings10MyTemplateIiEaSEOS1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(self,__param_0);
        }
    }
}

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

#[ctor::recursively_pinned]
#[repr(C, align(4))]
pub struct __CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEE {
    /// Reason for representing this field as a blob of bytes:
    /// Unsupported type 'struct test_namespace_bindings::TemplateWithTwoParams<int, int>': Unsupported type 'struct test_namespace_bindings::TemplateWithTwoParams<int, int>': No generated bindings found for 'TemplateWithTwoParams'
    pub(crate) value1: [crate::rust_std::mem::MaybeUninit<u8>; 8],
    pub value2: i32,
}
forward_declare::unsafe_define!(
    forward_declare::symbol!(
        "test_namespace_bindings::TemplateWithTwoParams<test_namespace_bindings::TemplateWithTwoParams<int, int>, int>"
    ),
    crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEE
);

impl ctor::CtorNew<()>
    for __CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEE
{
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        ctor::FnCtor::new(
            move |dest: crate::rust_std::pin::Pin<&mut crate::rust_std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(crate::rust_std::pin::Pin::into_inner_unchecked(dest));
            },
        )
    }
}

impl<'b>
    ctor::CtorNew<
        &'b crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEE,
    > for __CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEE
{
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(
        args:&'b crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEE,
    ) -> Self::CtorType {
        let __param_0 = args;
        ctor::FnCtor::new(
            move |dest: crate::rust_std::pin::Pin<&mut crate::rust_std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEC1ERKS2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(crate::rust_std::pin::Pin::into_inner_unchecked(dest),__param_0);
            },
        )
    }
}
impl<'b>
    ctor::CtorNew<(
        &'b crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEE,
    )> for __CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEE
{
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(
        args:(&'b crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEE,),
    ) -> Self::CtorType {
        let (arg,) = args;
        <Self as ctor::CtorNew<&'b crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEE>>::ctor_new(arg)
    }
}

impl<'b>
    ctor::CtorNew<
        ctor::RvalueReference<
            'b,
            crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEE,
        >,
    > for __CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEE
{
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(
        args: ctor::RvalueReference<
            'b,
            crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEE,
        >,
    ) -> Self::CtorType {
        let __param_0 = args;
        ctor::FnCtor::new(
            move |dest: crate::rust_std::pin::Pin<&mut crate::rust_std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEC1EOS2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(crate::rust_std::pin::Pin::into_inner_unchecked(dest),__param_0);
            },
        )
    }
}
impl<'b>
    ctor::CtorNew<(
        ctor::RvalueReference<
            'b,
            crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEE,
        >,
    )> for __CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEE
{
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(
        args:(ctor::RvalueReference<'b,crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEE>,),
    ) -> Self::CtorType {
        let (arg,) = args;
        <Self as ctor::CtorNew<ctor::RvalueReference<'b,crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEE>>>::ctor_new(arg)
    }
}

impl<'b>
    ::ctor::Assign<
        &'b crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEE,
    > for __CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEE
{
    #[inline(always)]
    fn assign<'a>(
        self: crate::rust_std::pin::Pin<&'a mut Self>,
        __param_0:&'b crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEE,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEaSERKS2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(self,__param_0);
        }
    }
}

impl<'b>
    ::ctor::Assign<
        ctor::RvalueReference<
            'b,
            crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEE,
        >,
    > for __CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEE
{
    #[inline(always)]
    fn assign<'a>(
        self: crate::rust_std::pin::Pin<&'a mut Self>,
        __param_0: ctor::RvalueReference<
            'b,
            crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEE,
        >,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEaSEOS2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(self,__param_0);
        }
    }
}

#[ctor::recursively_pinned]
#[repr(C)]
pub struct __CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIifEE {
    pub value1: i32,
    pub value2: f32,
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("test_namespace_bindings::TemplateWithTwoParams<int, float>"),
    crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIifEE
);

impl ctor::CtorNew<()> for __CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIifEE {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        ctor::FnCtor::new(
            move |dest: crate::rust_std::pin::Pin<&mut crate::rust_std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN23test_namespace_bindings21TemplateWithTwoParamsIifEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(crate::rust_std::pin::Pin::into_inner_unchecked(dest));
            },
        )
    }
}

impl<'b>
    ctor::CtorNew<&'b crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIifEE>
    for __CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIifEE
{
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(
        args: &'b crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIifEE,
    ) -> Self::CtorType {
        let __param_0 = args;
        ctor::FnCtor::new(
            move |dest: crate::rust_std::pin::Pin<&mut crate::rust_std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN23test_namespace_bindings21TemplateWithTwoParamsIifEC1ERKS1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(crate::rust_std::pin::Pin::into_inner_unchecked(dest),__param_0);
            },
        )
    }
}
impl<'b>
    ctor::CtorNew<(
        &'b crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIifEE,
    )> for __CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIifEE
{
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(
        args: (&'b crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIifEE,),
    ) -> Self::CtorType {
        let (arg,) = args;
        <Self as ctor::CtorNew<
            &'b crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIifEE,
        >>::ctor_new(arg)
    }
}

impl<'b>
    ctor::CtorNew<
        ctor::RvalueReference<
            'b,
            crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIifEE,
        >,
    > for __CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIifEE
{
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(
        args: ctor::RvalueReference<
            'b,
            crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIifEE,
        >,
    ) -> Self::CtorType {
        let __param_0 = args;
        ctor::FnCtor::new(
            move |dest: crate::rust_std::pin::Pin<&mut crate::rust_std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN23test_namespace_bindings21TemplateWithTwoParamsIifEC1EOS1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(crate::rust_std::pin::Pin::into_inner_unchecked(dest),__param_0);
            },
        )
    }
}
impl<'b>
    ctor::CtorNew<(
        ctor::RvalueReference<
            'b,
            crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIifEE,
        >,
    )> for __CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIifEE
{
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(
        args: (
            ctor::RvalueReference<
                'b,
                crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIifEE,
            >,
        ),
    ) -> Self::CtorType {
        let (arg,) = args;
        <Self as ctor::CtorNew<
            ctor::RvalueReference<
                'b,
                crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIifEE,
            >,
        >>::ctor_new(arg)
    }
}

impl<'b>
    ::ctor::Assign<
        &'b crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIifEE,
    > for __CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIifEE
{
    #[inline(always)]
    fn assign<'a>(
        self: crate::rust_std::pin::Pin<&'a mut Self>,
        __param_0:&'b crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIifEE,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN23test_namespace_bindings21TemplateWithTwoParamsIifEaSERKS1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(self,__param_0);
        }
    }
}

impl<'b>
    ::ctor::Assign<
        ctor::RvalueReference<
            'b,
            crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIifEE,
        >,
    > for __CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIifEE
{
    #[inline(always)]
    fn assign<'a>(
        self: crate::rust_std::pin::Pin<&'a mut Self>,
        __param_0: ctor::RvalueReference<
            'b,
            crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIifEE,
        >,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN23test_namespace_bindings21TemplateWithTwoParamsIifEaSEOS1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(self,__param_0);
        }
    }
}

#[ctor::recursively_pinned]
#[repr(C)]
pub struct __CcTemplateInst18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEE {
    pub value: crate::test_namespace_bindings::TemplateParam,
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("MyTopLevelTemplate<test_namespace_bindings::TemplateParam>"),
    crate::__CcTemplateInst18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEE
);

impl ctor::CtorNew<()>
    for __CcTemplateInst18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEE
{
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        ctor::FnCtor::new(
            move |dest: crate::rust_std::pin::Pin<&mut crate::rust_std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(crate::rust_std::pin::Pin::into_inner_unchecked(dest));
            },
        )
    }
}

impl<'b>
    ctor::CtorNew<
        &'b crate::__CcTemplateInst18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEE,
    > for __CcTemplateInst18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEE
{
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(
        args:&'b crate::__CcTemplateInst18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEE,
    ) -> Self::CtorType {
        let __param_0 = args;
        ctor::FnCtor::new(
            move |dest: crate::rust_std::pin::Pin<&mut crate::rust_std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEEC1ERKS2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(crate::rust_std::pin::Pin::into_inner_unchecked(dest),__param_0);
            },
        )
    }
}
impl<'b>
    ctor::CtorNew<(
        &'b crate::__CcTemplateInst18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEE,
    )> for __CcTemplateInst18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEE
{
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(
        args:(&'b crate::__CcTemplateInst18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEE,),
    ) -> Self::CtorType {
        let (arg,) = args;
        <Self as ctor::CtorNew<&'b crate::__CcTemplateInst18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEE>>::ctor_new(arg)
    }
}

impl<'b>
    ctor::CtorNew<
        ctor::RvalueReference<
            'b,
            crate::__CcTemplateInst18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEE,
        >,
    > for __CcTemplateInst18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEE
{
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(
        args: ctor::RvalueReference<
            'b,
            crate::__CcTemplateInst18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEE,
        >,
    ) -> Self::CtorType {
        let __param_0 = args;
        ctor::FnCtor::new(
            move |dest: crate::rust_std::pin::Pin<&mut crate::rust_std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEEC1EOS2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(crate::rust_std::pin::Pin::into_inner_unchecked(dest),__param_0);
            },
        )
    }
}
impl<'b>
    ctor::CtorNew<(
        ctor::RvalueReference<
            'b,
            crate::__CcTemplateInst18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEE,
        >,
    )> for __CcTemplateInst18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEE
{
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(
        args:(ctor::RvalueReference<'b,crate::__CcTemplateInst18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEE>,),
    ) -> Self::CtorType {
        let (arg,) = args;
        <Self as ctor::CtorNew<ctor::RvalueReference<'b,crate::__CcTemplateInst18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEE>>>::ctor_new(arg)
    }
}

impl<'b>
    ::ctor::Assign<
        &'b crate::__CcTemplateInst18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEE,
    > for __CcTemplateInst18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEE
{
    #[inline(always)]
    fn assign<'a>(
        self: crate::rust_std::pin::Pin<&'a mut Self>,
        __param_0:&'b crate::__CcTemplateInst18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEE,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEEaSERKS2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(self,__param_0);
        }
    }
}

impl<'b>
    ::ctor::Assign<
        ctor::RvalueReference<
            'b,
            crate::__CcTemplateInst18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEE,
        >,
    > for __CcTemplateInst18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEE
{
    #[inline(always)]
    fn assign<'a>(
        self: crate::rust_std::pin::Pin<&'a mut Self>,
        __param_0: ctor::RvalueReference<
            'b,
            crate::__CcTemplateInst18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEE,
        >,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEEaSEOS2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(self,__param_0);
        }
    }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    extern "C" {
        pub(crate) fn __rust_thunk___ZN14DifferentScopeC1Ev<'a>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::DifferentScope>,
        );
        pub(crate) fn __rust_thunk___ZN14DifferentScopeC1ERKS_<'a, 'b>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::DifferentScope>,
            __param_0: &'b crate::DifferentScope,
        );
        pub(crate) fn __rust_thunk___ZN14DifferentScopeC1EOS_<'a, 'b>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::DifferentScope>,
            __param_0: ctor::RvalueReference<'b, crate::DifferentScope>,
        );
        pub(crate) fn __rust_thunk___ZN14DifferentScopeaSERKS_<'a, 'b>(
            __this: crate::rust_std::pin::Pin<&'a mut crate::DifferentScope>,
            __param_0: &'b crate::DifferentScope,
        ) -> crate::rust_std::pin::Pin<&'a mut crate::DifferentScope>;
        pub(crate) fn __rust_thunk___ZN14DifferentScopeaSEOS_<'a, 'b>(
            __this: crate::rust_std::pin::Pin<&'a mut crate::DifferentScope>,
            __param_0: ctor::RvalueReference<'b, crate::DifferentScope>,
        ) -> crate::rust_std::pin::Pin<&'a mut crate::DifferentScope>;
        pub(crate) fn __rust_thunk___ZN23test_namespace_bindings13TemplateParamC1Ev<'a>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<
                crate::test_namespace_bindings::TemplateParam,
            >,
        );
        pub(crate) fn __rust_thunk___ZN23test_namespace_bindings13TemplateParamC1ERKS0_<'a, 'b>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<
                crate::test_namespace_bindings::TemplateParam,
            >,
            __param_0: &'b crate::test_namespace_bindings::TemplateParam,
        );
        pub(crate) fn __rust_thunk___ZN23test_namespace_bindings13TemplateParamC1EOS0_<'a, 'b>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<
                crate::test_namespace_bindings::TemplateParam,
            >,
            __param_0: ctor::RvalueReference<'b, crate::test_namespace_bindings::TemplateParam>,
        );
        pub(crate) fn __rust_thunk___ZN23test_namespace_bindings13TemplateParamaSERKS0_<'a, 'b>(
            __this: crate::rust_std::pin::Pin<
                &'a mut crate::test_namespace_bindings::TemplateParam,
            >,
            __param_0: &'b crate::test_namespace_bindings::TemplateParam,
        ) -> crate::rust_std::pin::Pin<&'a mut crate::test_namespace_bindings::TemplateParam>;
        pub(crate) fn __rust_thunk___ZN23test_namespace_bindings13TemplateParamaSEOS0_<'a, 'b>(
            __this: crate::rust_std::pin::Pin<
                &'a mut crate::test_namespace_bindings::TemplateParam,
            >,
            __param_0: ctor::RvalueReference<'b, crate::test_namespace_bindings::TemplateParam>,
        ) -> crate::rust_std::pin::Pin<&'a mut crate::test_namespace_bindings::TemplateParam>;
        #[link_name = "_Z36processForwardDeclaredSpecializationP18MyTopLevelTemplateIiE"]
        pub(crate) fn __rust_thunk___Z36processForwardDeclaredSpecializationP18MyTopLevelTemplateIiE<
            'a,
        >(
            i: Option<
                crate::rust_std::pin::Pin<&'a mut crate::__CcTemplateInst18MyTopLevelTemplateIiE>,
            >,
        );
        pub(crate) fn __rust_thunk___ZN23test_namespace_bindings10MyTemplateI14DifferentScopeEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc<
            'a,
        >(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<
                crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateI14DifferentScopeEE,
            >,
        );
        pub(crate) fn __rust_thunk___ZN23test_namespace_bindings10MyTemplateI14DifferentScopeEC1ERKS2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc<
            'a,
            'b,
        >(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<
                crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateI14DifferentScopeEE,
            >,
            __param_0:&'b crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateI14DifferentScopeEE,
        );
        pub(crate) fn __rust_thunk___ZN23test_namespace_bindings10MyTemplateI14DifferentScopeEC1EOS2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc<
            'a,
            'b,
        >(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<
                crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateI14DifferentScopeEE,
            >,
            __param_0: ctor::RvalueReference<
                'b,
                crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateI14DifferentScopeEE,
            >,
        );
        pub(crate)fn __rust_thunk___ZN23test_namespace_bindings10MyTemplateI14DifferentScopeEaSERKS2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc<'a,'b>(__this:crate::rust_std::pin::Pin<&'a mut crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateI14DifferentScopeEE>,__param_0:&'b crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateI14DifferentScopeEE)->crate::rust_std::pin::Pin<&'a mut crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateI14DifferentScopeEE>;
        pub(crate)fn __rust_thunk___ZN23test_namespace_bindings10MyTemplateI14DifferentScopeEaSEOS2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc<'a,'b>(__this:crate::rust_std::pin::Pin<&'a mut crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateI14DifferentScopeEE>,__param_0:ctor::RvalueReference<'b,crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateI14DifferentScopeEE>)->crate::rust_std::pin::Pin<&'a mut crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateI14DifferentScopeEE>;
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
            __this: &'a mut crate::rust_std::mem::MaybeUninit<
                crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEE,
            >,
        );
        pub(crate) fn __rust_thunk___ZN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEC1ERKS2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc<
            'a,
            'b,
        >(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<
                crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEE,
            >,
            __param_0:&'b crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEE,
        );
        pub(crate) fn __rust_thunk___ZN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEC1EOS2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc<
            'a,
            'b,
        >(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<
                crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEE,
            >,
            __param_0: ctor::RvalueReference<
                'b,
                crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEE,
            >,
        );
        pub(crate)fn __rust_thunk___ZN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEaSERKS2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc<'a,'b>(__this:crate::rust_std::pin::Pin<&'a mut crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEE>,__param_0:&'b crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEE)->crate::rust_std::pin::Pin<&'a mut crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEE>;
        pub(crate)fn __rust_thunk___ZN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEaSEOS2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc<'a,'b>(__this:crate::rust_std::pin::Pin<&'a mut crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEE>,__param_0:ctor::RvalueReference<'b,crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEE>)->crate::rust_std::pin::Pin<&'a mut crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEE>;
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
            __this: &'a mut crate::rust_std::mem::MaybeUninit<
                crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE,
            >,
        );
        pub(crate) fn __rust_thunk___ZN23test_namespace_bindings10MyTemplateIiEC1ERKS1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc<
            'a,
            'b,
        >(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<
                crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE,
            >,
            __param_0: &'b crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE,
        );
        pub(crate) fn __rust_thunk___ZN23test_namespace_bindings10MyTemplateIiEC1EOS1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc<
            'a,
            'b,
        >(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<
                crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE,
            >,
            __param_0: ctor::RvalueReference<
                'b,
                crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE,
            >,
        );
        pub(crate) fn __rust_thunk___ZN23test_namespace_bindings10MyTemplateIiEaSERKS1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc<
            'a,
            'b,
        >(
            __this: crate::rust_std::pin::Pin<
                &'a mut crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE,
            >,
            __param_0: &'b crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE,
        ) -> crate::rust_std::pin::Pin<
            &'a mut crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE,
        >;
        pub(crate) fn __rust_thunk___ZN23test_namespace_bindings10MyTemplateIiEaSEOS1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc<
            'a,
            'b,
        >(
            __this: crate::rust_std::pin::Pin<
                &'a mut crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE,
            >,
            __param_0: ctor::RvalueReference<
                'b,
                crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE,
            >,
        ) -> crate::rust_std::pin::Pin<
            &'a mut crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE,
        >;
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
            __this:&'a mut crate::rust_std::mem::MaybeUninit<crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEE>,
        );
        pub(crate) fn __rust_thunk___ZN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEC1ERKS2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc<
            'a,
            'b,
        >(
            __this:&'a mut crate::rust_std::mem::MaybeUninit<crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEE>,
            __param_0:&'b crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEE,
        );
        pub(crate) fn __rust_thunk___ZN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEC1EOS2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc<
            'a,
            'b,
        >(
            __this:&'a mut crate::rust_std::mem::MaybeUninit<crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEE>,
            __param_0:ctor::RvalueReference<'b,crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEE>,
        );
        pub(crate)fn __rust_thunk___ZN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEaSERKS2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc<'a,'b>(__this:crate::rust_std::pin::Pin<&'a mut crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEE>,__param_0:&'b crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEE)->crate::rust_std::pin::Pin<&'a mut crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEE>;
        pub(crate)fn __rust_thunk___ZN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEaSEOS2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc<'a,'b>(__this:crate::rust_std::pin::Pin<&'a mut crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEE>,__param_0:ctor::RvalueReference<'b,crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEE>)->crate::rust_std::pin::Pin<&'a mut crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEE>;
        pub(crate) fn __rust_thunk___ZN23test_namespace_bindings21TemplateWithTwoParamsIifEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc<
            'a,
        >(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<
                crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIifEE,
            >,
        );
        pub(crate) fn __rust_thunk___ZN23test_namespace_bindings21TemplateWithTwoParamsIifEC1ERKS1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc<
            'a,
            'b,
        >(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<
                crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIifEE,
            >,
            __param_0:&'b crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIifEE,
        );
        pub(crate) fn __rust_thunk___ZN23test_namespace_bindings21TemplateWithTwoParamsIifEC1EOS1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc<
            'a,
            'b,
        >(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<
                crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIifEE,
            >,
            __param_0: ctor::RvalueReference<
                'b,
                crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIifEE,
            >,
        );
        pub(crate) fn __rust_thunk___ZN23test_namespace_bindings21TemplateWithTwoParamsIifEaSERKS1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc<
            'a,
            'b,
        >(
            __this:crate::rust_std::pin::Pin<&'a mut crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIifEE>,
            __param_0:&'b crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIifEE,
        ) -> crate::rust_std::pin::Pin<
            &'a mut crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIifEE,
        >;
        pub(crate) fn __rust_thunk___ZN23test_namespace_bindings21TemplateWithTwoParamsIifEaSEOS1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc<
            'a,
            'b,
        >(
            __this:crate::rust_std::pin::Pin<&'a mut crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIifEE>,
            __param_0: ctor::RvalueReference<
                'b,
                crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIifEE,
            >,
        ) -> crate::rust_std::pin::Pin<
            &'a mut crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIifEE,
        >;
        pub(crate) fn __rust_thunk___ZN18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc<
            'a,
        >(
            __this:&'a mut crate::rust_std::mem::MaybeUninit<crate::__CcTemplateInst18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEE>,
        );
        pub(crate) fn __rust_thunk___ZN18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEEC1ERKS2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc<
            'a,
            'b,
        >(
            __this:&'a mut crate::rust_std::mem::MaybeUninit<crate::__CcTemplateInst18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEE>,
            __param_0:&'b crate::__CcTemplateInst18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEE,
        );
        pub(crate) fn __rust_thunk___ZN18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEEC1EOS2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc<
            'a,
            'b,
        >(
            __this:&'a mut crate::rust_std::mem::MaybeUninit<crate::__CcTemplateInst18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEE>,
            __param_0:ctor::RvalueReference<'b,crate::__CcTemplateInst18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEE>,
        );
        pub(crate)fn __rust_thunk___ZN18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEEaSERKS2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc<'a,'b>(__this:crate::rust_std::pin::Pin<&'a mut crate::__CcTemplateInst18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEE>,__param_0:&'b crate::__CcTemplateInst18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEE)->crate::rust_std::pin::Pin<&'a mut crate::__CcTemplateInst18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEE>;
        pub(crate)fn __rust_thunk___ZN18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEEaSEOS2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc<'a,'b>(__this:crate::rust_std::pin::Pin<&'a mut crate::__CcTemplateInst18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEE>,__param_0:ctor::RvalueReference<'b,crate::__CcTemplateInst18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEE>)->crate::rust_std::pin::Pin<&'a mut crate::__CcTemplateInst18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEE>;
    }
}

const _: () = assert!(rust_std::mem::size_of::<Option<&i32>>() == rust_std::mem::size_of::<&i32>());

const _: () = assert!(rust_std::mem::size_of::<crate::DifferentScope>() == 1);
const _: () = assert!(rust_std::mem::align_of::<crate::DifferentScope>() == 1);
const _: () = {
    static_assertions::assert_not_impl_all!(crate::DifferentScope: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_all!(crate::DifferentScope: Drop);
};

const _: () =
    assert!(rust_std::mem::size_of::<crate::test_namespace_bindings::TemplateParam>() == 1);
const _: () =
    assert!(rust_std::mem::align_of::<crate::test_namespace_bindings::TemplateParam>() == 1);
const _: () = {
    static_assertions::assert_not_impl_all!(crate::test_namespace_bindings::TemplateParam: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_all!(crate::test_namespace_bindings::TemplateParam: Drop);
};

const _: () = assert!(
    rust_std::mem::size_of::<
        crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateI14DifferentScopeEE,
    >() == 1
);
const _: () = assert!(
    rust_std::mem::align_of::<
        crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateI14DifferentScopeEE,
    >() == 1
);
const _: () = {
    static_assertions::assert_not_impl_all!(
        crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateI14DifferentScopeEE: Copy
    );
};
const _: () = {
    static_assertions::assert_not_impl_all!(
        crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateI14DifferentScopeEE: Drop
    );
};
const _: () = assert!(
    memoffset_unstable_const::offset_of!(
        crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateI14DifferentScopeEE,
        value_
    ) == 0
);

const _: () = assert!(
    rust_std::mem::size_of::<
        crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEE,
    >() == 1
);
const _: () = assert!(
    rust_std::mem::align_of::<
        crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEE,
    >() == 1
);
const _: () = {
    static_assertions::assert_not_impl_all!(
        crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEE: Copy
    );
};
const _: () = {
    static_assertions::assert_not_impl_all!(
        crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEE: Drop
    );
};
const _: () = assert!(
    memoffset_unstable_const::offset_of!(
        crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEE,
        value_
    ) == 0
);

const _: () = assert!(
    rust_std::mem::size_of::<crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE>()
        == 4
);
const _: () = assert!(
    rust_std::mem::align_of::<crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE>()
        == 4
);
const _: () = {
    static_assertions::assert_not_impl_all!(
        crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE: Copy
    );
};
const _: () = {
    static_assertions::assert_not_impl_all!(
        crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE: Drop
    );
};
const _: () = assert!(
    memoffset_unstable_const::offset_of!(
        crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE,
        value_
    ) == 0
);

const _: () = assert!(
    rust_std::mem::size_of::<
        crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEE,
    >() == 12
);
const _: () = assert!(
    rust_std::mem::align_of::<
        crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEE,
    >() == 4
);
const _: () = {
    static_assertions::assert_not_impl_all!(
        crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEE: Copy
    );
};
const _: () = {
    static_assertions::assert_not_impl_all!(
        crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEE: Drop
    );
};
const _: () = assert!(
    memoffset_unstable_const::offset_of!(
        crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEE,
        value1
    ) == 0
);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(
        crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEE,
        value2
    ) == 8
);

const _: () = assert!(
    rust_std::mem::size_of::<
        crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIifEE,
    >() == 8
);
const _: () = assert!(
    rust_std::mem::align_of::<
        crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIifEE,
    >() == 4
);
const _: () = {
    static_assertions::assert_not_impl_all!(
        crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIifEE: Copy
    );
};
const _: () = {
    static_assertions::assert_not_impl_all!(
        crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIifEE: Drop
    );
};
const _: () = assert!(
    memoffset_unstable_const::offset_of!(
        crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIifEE,
        value1
    ) == 0
);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(
        crate::__CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsIifEE,
        value2
    ) == 4
);

const _: () = assert!(
    rust_std::mem::size_of::<
        crate::__CcTemplateInst18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEE,
    >() == 1
);
const _: () = assert!(
    rust_std::mem::align_of::<
        crate::__CcTemplateInst18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEE,
    >() == 1
);
const _: () = {
    static_assertions::assert_not_impl_all!(
        crate::__CcTemplateInst18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEE:
            Copy
    );
};
const _: () = {
    static_assertions::assert_not_impl_all!(
        crate::__CcTemplateInst18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEE:
            Drop
    );
};
const _: () = assert!(
    memoffset_unstable_const::offset_of!(
        crate::__CcTemplateInst18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEE,
        value
    ) == 0
);
