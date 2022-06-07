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

// rs_bindings_from_cc/test/golden/templates.h;l=10
// Error while generating bindings for item 'MyTemplate':
// Class templates are not supported yet

pub type MyTypeAlias = crate::__CcTemplateInst10MyTemplateIiE;

pub type OtherTypeAliasInSameTarget = crate::__CcTemplateInst10MyTemplateIiE;

// rs_bindings_from_cc/test/golden/templates.h;l=28
// Error while generating bindings for item 'TemplateWithTwoParams':
// Class templates are not supported yet

pub type AliasToTemplateWithTwoParams = crate::__CcTemplateInst21TemplateWithTwoParamsIifE;

// THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_TEMPLATES_H_

#[ctor::recursively_pinned]
#[repr(C, align(4))]
pub struct __CcTemplateInst10MyTemplateIiE {
    __non_field_data: [crate::rust_std::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) value_: [crate::rust_std::mem::MaybeUninit<u8>; 4],
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("MyTemplate<int>"),
    crate::__CcTemplateInst10MyTemplateIiE
);

impl ctor::CtorNew<()> for __CcTemplateInst10MyTemplateIiE {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        ctor::FnCtor::new(
            move |dest: crate::rust_std::pin::Pin<&mut crate::rust_std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN10MyTemplateIiEC1Ev___third_party_crubit_rs_bindings_from_cc_test_golden_templates_cc(crate::rust_std::pin::Pin::into_inner_unchecked(dest));
            },
        )
    }
}

impl<'b> ctor::CtorNew<&'b crate::__CcTemplateInst10MyTemplateIiE>
    for __CcTemplateInst10MyTemplateIiE
{
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: &'b crate::__CcTemplateInst10MyTemplateIiE) -> Self::CtorType {
        let __param_0 = args;
        ctor::FnCtor::new(
            move |dest: crate::rust_std::pin::Pin<&mut crate::rust_std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN10MyTemplateIiEC1ERKS0____third_party_crubit_rs_bindings_from_cc_test_golden_templates_cc(crate::rust_std::pin::Pin::into_inner_unchecked(dest),__param_0);
            },
        )
    }
}
impl<'b> ctor::CtorNew<(&'b crate::__CcTemplateInst10MyTemplateIiE,)>
    for __CcTemplateInst10MyTemplateIiE
{
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: (&'b crate::__CcTemplateInst10MyTemplateIiE,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ctor::CtorNew<&'b crate::__CcTemplateInst10MyTemplateIiE>>::ctor_new(arg)
    }
}

impl<'b> ctor::CtorNew<ctor::RvalueReference<'b, crate::__CcTemplateInst10MyTemplateIiE>>
    for __CcTemplateInst10MyTemplateIiE
{
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(
        args: ctor::RvalueReference<'b, crate::__CcTemplateInst10MyTemplateIiE>,
    ) -> Self::CtorType {
        let __param_0 = args;
        ctor::FnCtor::new(
            move |dest: crate::rust_std::pin::Pin<&mut crate::rust_std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN10MyTemplateIiEC1EOS0____third_party_crubit_rs_bindings_from_cc_test_golden_templates_cc(crate::rust_std::pin::Pin::into_inner_unchecked(dest),__param_0);
            },
        )
    }
}
impl<'b> ctor::CtorNew<(ctor::RvalueReference<'b, crate::__CcTemplateInst10MyTemplateIiE>,)>
    for __CcTemplateInst10MyTemplateIiE
{
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(
        args: (ctor::RvalueReference<'b, crate::__CcTemplateInst10MyTemplateIiE>,),
    ) -> Self::CtorType {
        let (arg,) = args;
        <Self as ctor::CtorNew<ctor::RvalueReference<'b,crate::__CcTemplateInst10MyTemplateIiE>>>::ctor_new(arg)
    }
}

// rs_bindings_from_cc/test/golden/templates.h;l=11
// Error while generating bindings for item 'MyTemplate<int>::operator=':
// Bindings for this kind of operator are not supported

// rs_bindings_from_cc/test/golden/templates.h;l=11
// Error while generating bindings for item 'MyTemplate<int>::operator=':
// Bindings for this kind of operator are not supported

impl __CcTemplateInst10MyTemplateIiE {
    #[inline(always)]
    pub fn Create(value: i32) -> crate::__CcTemplateInst10MyTemplateIiE {
        unsafe {
            crate::detail::__rust_thunk___ZN10MyTemplateIiE6CreateEi___third_party_crubit_rs_bindings_from_cc_test_golden_templates_cc(value)
        }
    }
}

impl __CcTemplateInst10MyTemplateIiE {
    #[inline(always)]
    pub fn value<'a>(&'a self) -> &'a i32 {
        unsafe {
            crate::detail::__rust_thunk___ZNK10MyTemplateIiE5valueEv___third_party_crubit_rs_bindings_from_cc_test_golden_templates_cc(self)
        }
    }
}

#[ctor::recursively_pinned]
#[repr(C)]
pub struct __CcTemplateInst21TemplateWithTwoParamsIifE {
    pub value1: i32,
    pub value2: f32,
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("TemplateWithTwoParams<int, float>"),
    crate::__CcTemplateInst21TemplateWithTwoParamsIifE
);

impl ctor::CtorNew<()> for __CcTemplateInst21TemplateWithTwoParamsIifE {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        ctor::FnCtor::new(
            move |dest: crate::rust_std::pin::Pin<&mut crate::rust_std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN21TemplateWithTwoParamsIifEC1Ev___third_party_crubit_rs_bindings_from_cc_test_golden_templates_cc(crate::rust_std::pin::Pin::into_inner_unchecked(dest));
            },
        )
    }
}

impl<'b> ctor::CtorNew<&'b crate::__CcTemplateInst21TemplateWithTwoParamsIifE>
    for __CcTemplateInst21TemplateWithTwoParamsIifE
{
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: &'b crate::__CcTemplateInst21TemplateWithTwoParamsIifE) -> Self::CtorType {
        let __param_0 = args;
        ctor::FnCtor::new(
            move |dest: crate::rust_std::pin::Pin<&mut crate::rust_std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN21TemplateWithTwoParamsIifEC1ERKS0____third_party_crubit_rs_bindings_from_cc_test_golden_templates_cc(crate::rust_std::pin::Pin::into_inner_unchecked(dest),__param_0);
            },
        )
    }
}
impl<'b> ctor::CtorNew<(&'b crate::__CcTemplateInst21TemplateWithTwoParamsIifE,)>
    for __CcTemplateInst21TemplateWithTwoParamsIifE
{
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: (&'b crate::__CcTemplateInst21TemplateWithTwoParamsIifE,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ctor::CtorNew<&'b crate::__CcTemplateInst21TemplateWithTwoParamsIifE>>::ctor_new(
            arg,
        )
    }
}

impl<'b>
    ctor::CtorNew<ctor::RvalueReference<'b, crate::__CcTemplateInst21TemplateWithTwoParamsIifE>>
    for __CcTemplateInst21TemplateWithTwoParamsIifE
{
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(
        args: ctor::RvalueReference<'b, crate::__CcTemplateInst21TemplateWithTwoParamsIifE>,
    ) -> Self::CtorType {
        let __param_0 = args;
        ctor::FnCtor::new(
            move |dest: crate::rust_std::pin::Pin<&mut crate::rust_std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN21TemplateWithTwoParamsIifEC1EOS0____third_party_crubit_rs_bindings_from_cc_test_golden_templates_cc(crate::rust_std::pin::Pin::into_inner_unchecked(dest),__param_0);
            },
        )
    }
}
impl<'b>
    ctor::CtorNew<(ctor::RvalueReference<'b, crate::__CcTemplateInst21TemplateWithTwoParamsIifE>,)>
    for __CcTemplateInst21TemplateWithTwoParamsIifE
{
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(
        args: (ctor::RvalueReference<'b, crate::__CcTemplateInst21TemplateWithTwoParamsIifE>,),
    ) -> Self::CtorType {
        let (arg,) = args;
        <Self as ctor::CtorNew<
            ctor::RvalueReference<'b, crate::__CcTemplateInst21TemplateWithTwoParamsIifE>,
        >>::ctor_new(arg)
    }
}

// rs_bindings_from_cc/test/golden/templates.h;l=29
// Error while generating bindings for item 'TemplateWithTwoParams<int, float>::operator=':
// Bindings for this kind of operator are not supported

// rs_bindings_from_cc/test/golden/templates.h;l=29
// Error while generating bindings for item 'TemplateWithTwoParams<int, float>::operator=':
// Bindings for this kind of operator are not supported

mod detail {
    #[allow(unused_imports)]
    use super::*;
    extern "C" {
        pub(crate) fn __rust_thunk___ZN10MyTemplateIiEC1Ev___third_party_crubit_rs_bindings_from_cc_test_golden_templates_cc<
            'a,
        >(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<
                crate::__CcTemplateInst10MyTemplateIiE,
            >,
        );
        pub(crate) fn __rust_thunk___ZN10MyTemplateIiEC1ERKS0____third_party_crubit_rs_bindings_from_cc_test_golden_templates_cc<
            'a,
            'b,
        >(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<
                crate::__CcTemplateInst10MyTemplateIiE,
            >,
            __param_0: &'b crate::__CcTemplateInst10MyTemplateIiE,
        );
        pub(crate) fn __rust_thunk___ZN10MyTemplateIiEC1EOS0____third_party_crubit_rs_bindings_from_cc_test_golden_templates_cc<
            'a,
            'b,
        >(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<
                crate::__CcTemplateInst10MyTemplateIiE,
            >,
            __param_0: ctor::RvalueReference<'b, crate::__CcTemplateInst10MyTemplateIiE>,
        );
        pub(crate) fn __rust_thunk___ZN10MyTemplateIiE6CreateEi___third_party_crubit_rs_bindings_from_cc_test_golden_templates_cc(
            value: i32,
        ) -> crate::__CcTemplateInst10MyTemplateIiE;
        pub(crate) fn __rust_thunk___ZNK10MyTemplateIiE5valueEv___third_party_crubit_rs_bindings_from_cc_test_golden_templates_cc<
            'a,
        >(
            __this: &'a crate::__CcTemplateInst10MyTemplateIiE,
        ) -> &'a i32;
        pub(crate) fn __rust_thunk___ZN21TemplateWithTwoParamsIifEC1Ev___third_party_crubit_rs_bindings_from_cc_test_golden_templates_cc<
            'a,
        >(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<
                crate::__CcTemplateInst21TemplateWithTwoParamsIifE,
            >,
        );
        pub(crate) fn __rust_thunk___ZN21TemplateWithTwoParamsIifEC1ERKS0____third_party_crubit_rs_bindings_from_cc_test_golden_templates_cc<
            'a,
            'b,
        >(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<
                crate::__CcTemplateInst21TemplateWithTwoParamsIifE,
            >,
            __param_0: &'b crate::__CcTemplateInst21TemplateWithTwoParamsIifE,
        );
        pub(crate) fn __rust_thunk___ZN21TemplateWithTwoParamsIifEC1EOS0____third_party_crubit_rs_bindings_from_cc_test_golden_templates_cc<
            'a,
            'b,
        >(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<
                crate::__CcTemplateInst21TemplateWithTwoParamsIifE,
            >,
            __param_0: ctor::RvalueReference<
                'b,
                crate::__CcTemplateInst21TemplateWithTwoParamsIifE,
            >,
        );
    }
}

const _: () = assert!(rust_std::mem::size_of::<Option<&i32>>() == rust_std::mem::size_of::<&i32>());

const _: () = assert!(rust_std::mem::size_of::<crate::__CcTemplateInst10MyTemplateIiE>() == 4);
const _: () = assert!(rust_std::mem::align_of::<crate::__CcTemplateInst10MyTemplateIiE>() == 4);
const _: () = {
    static_assertions::assert_not_impl_all!(crate::__CcTemplateInst10MyTemplateIiE: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_all!(crate::__CcTemplateInst10MyTemplateIiE: Drop);
};
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::__CcTemplateInst10MyTemplateIiE, value_) == 0
);

const _: () =
    assert!(rust_std::mem::size_of::<crate::__CcTemplateInst21TemplateWithTwoParamsIifE>() == 8);
const _: () =
    assert!(rust_std::mem::align_of::<crate::__CcTemplateInst21TemplateWithTwoParamsIifE>() == 4);
const _: () = {
    static_assertions::assert_not_impl_all!(
        crate::__CcTemplateInst21TemplateWithTwoParamsIifE: Copy
    );
};
const _: () = {
    static_assertions::assert_not_impl_all!(
        crate::__CcTemplateInst21TemplateWithTwoParamsIifE: Drop
    );
};
const _: () = assert!(
    memoffset_unstable_const::offset_of!(
        crate::__CcTemplateInst21TemplateWithTwoParamsIifE,
        value1
    ) == 0
);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(
        crate::__CcTemplateInst21TemplateWithTwoParamsIifE,
        value2
    ) == 4
);
