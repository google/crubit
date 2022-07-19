// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for C++ target
// //rs_bindings_from_cc/test/golden:templates_cc
#![rustfmt::skip]
#![feature(const_ptr_offset_from, custom_inner_attributes)]
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
    i: Option<::std::pin::Pin<&'a mut crate::__CcTemplateInst18MyTopLevelTemplateIiE>>,
) {
    unsafe {
        crate::detail::__rust_thunk___Z36processForwardDeclaredSpecializationP18MyTopLevelTemplateIiE(i)
    }
}

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
#[repr(C, align(4))]
pub struct __CcTemplateInstN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEE {
    /// Reason for representing this field as a blob of bytes:
    /// Unsupported type 'struct test_namespace_bindings::TemplateWithTwoParams<int, int>': Unsupported type 'struct test_namespace_bindings::TemplateWithTwoParams<int, int>': No generated bindings found for 'TemplateWithTwoParams'
    pub(crate) value1: [::std::mem::MaybeUninit<u8>; 8],
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

// rs_bindings_from_cc/test/golden/templates.h;l=50
// Error while generating bindings for item 'MyTopLevelTemplate<test_namespace_bindings::TemplateParam>::operator=':
// operator= for Unpin types is not yet supported.

// rs_bindings_from_cc/test/golden/templates.h;l=50
// Error while generating bindings for item 'MyTopLevelTemplate<test_namespace_bindings::TemplateParam>::operator=':
// operator= for Unpin types is not yet supported.

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
    memoffset_unstable_const::offset_of!(
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
    memoffset_unstable_const::offset_of!(
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
    memoffset_unstable_const::offset_of!(
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
    memoffset_unstable_const::offset_of!(
        crate::__CcTemplateInst18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEE,
        value
    ) == 0
);
