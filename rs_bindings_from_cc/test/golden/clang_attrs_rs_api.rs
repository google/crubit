// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:clang_attrs_cc
// Features: experimental, supported

#![rustfmt::skip]
#![feature(custom_inner_attributes, negative_impls, type_alias_impl_trait)]
#![allow(stable_features)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![deny(warnings)]

// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[::ctor::recursively_pinned]
#[repr(C, align(64))]
pub struct HasCustomAlignment {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 64],
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("HasCustomAlignment"),
    crate::HasCustomAlignment
);

impl ::ctor::CtorNew<()> for HasCustomAlignment {
    type CtorType = impl ::ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN18HasCustomAlignmentC1Ev(
                        ::core::pin::Pin::into_inner_unchecked(dest),
                    );
                },
            )
        }
    }
}

impl<'b> ::ctor::CtorNew<&'b Self> for HasCustomAlignment {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: &'b Self) -> Self::CtorType {
        let __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN18HasCustomAlignmentC1ERKS_(
                        ::core::pin::Pin::into_inner_unchecked(dest),
                        __param_0,
                    );
                },
            )
        }
    }
}
impl<'b> ::ctor::CtorNew<(&'b Self,)> for HasCustomAlignment {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: (&'b Self,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'b Self>>::ctor_new(arg)
    }
}

impl<'b> ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>> for HasCustomAlignment {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'b, Self>) -> Self::CtorType {
        let __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN18HasCustomAlignmentC1EOS_(
                        ::core::pin::Pin::into_inner_unchecked(dest),
                        __param_0,
                    );
                },
            )
        }
    }
}
impl<'b> ::ctor::CtorNew<(::ctor::RvalueReference<'b, Self>,)> for HasCustomAlignment {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: (::ctor::RvalueReference<'b, Self>,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>>>::ctor_new(arg)
    }
}

impl<'b> ::ctor::Assign<&'b Self> for HasCustomAlignment {
    #[inline(always)]
    fn assign<'a>(self: ::core::pin::Pin<&'a mut Self>, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN18HasCustomAlignmentaSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::Assign<::ctor::RvalueReference<'b, Self>> for HasCustomAlignment {
    #[inline(always)]
    fn assign<'a>(
        self: ::core::pin::Pin<&'a mut Self>,
        __param_0: ::ctor::RvalueReference<'b, Self>,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN18HasCustomAlignmentaSEOS_(self, __param_0);
        }
    }
}

#[::ctor::recursively_pinned]
#[repr(C)]
pub struct HasFieldWithCustomAlignment {
    pub field: crate::HasCustomAlignment,
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("HasFieldWithCustomAlignment"),
    crate::HasFieldWithCustomAlignment
);

impl ::ctor::CtorNew<()> for HasFieldWithCustomAlignment {
    type CtorType = impl ::ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN27HasFieldWithCustomAlignmentC1Ev(
                        ::core::pin::Pin::into_inner_unchecked(dest),
                    );
                },
            )
        }
    }
}

impl<'b> ::ctor::CtorNew<&'b Self> for HasFieldWithCustomAlignment {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: &'b Self) -> Self::CtorType {
        let __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN27HasFieldWithCustomAlignmentC1ERKS_(
                        ::core::pin::Pin::into_inner_unchecked(dest),
                        __param_0,
                    );
                },
            )
        }
    }
}
impl<'b> ::ctor::CtorNew<(&'b Self,)> for HasFieldWithCustomAlignment {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: (&'b Self,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'b Self>>::ctor_new(arg)
    }
}

impl<'b> ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>> for HasFieldWithCustomAlignment {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'b, Self>) -> Self::CtorType {
        let __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN27HasFieldWithCustomAlignmentC1EOS_(
                        ::core::pin::Pin::into_inner_unchecked(dest),
                        __param_0,
                    );
                },
            )
        }
    }
}
impl<'b> ::ctor::CtorNew<(::ctor::RvalueReference<'b, Self>,)> for HasFieldWithCustomAlignment {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: (::ctor::RvalueReference<'b, Self>,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>>>::ctor_new(arg)
    }
}

impl<'b> ::ctor::Assign<&'b Self> for HasFieldWithCustomAlignment {
    #[inline(always)]
    fn assign<'a>(self: ::core::pin::Pin<&'a mut Self>, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN27HasFieldWithCustomAlignmentaSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::Assign<::ctor::RvalueReference<'b, Self>> for HasFieldWithCustomAlignment {
    #[inline(always)]
    fn assign<'a>(
        self: ::core::pin::Pin<&'a mut Self>,
        __param_0: ::ctor::RvalueReference<'b, Self>,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN27HasFieldWithCustomAlignmentaSEOS_(self, __param_0);
        }
    }
}

#[::ctor::recursively_pinned]
#[repr(C, align(64))]
pub struct InheritsFromBaseWithCustomAlignment {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 64],
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("InheritsFromBaseWithCustomAlignment"),
    crate::InheritsFromBaseWithCustomAlignment
);

impl ::ctor::CtorNew<()> for InheritsFromBaseWithCustomAlignment {
    type CtorType = impl ::ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN35InheritsFromBaseWithCustomAlignmentC1Ev(
                        ::core::pin::Pin::into_inner_unchecked(dest),
                    );
                },
            )
        }
    }
}

impl<'b> ::ctor::CtorNew<&'b Self> for InheritsFromBaseWithCustomAlignment {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: &'b Self) -> Self::CtorType {
        let __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN35InheritsFromBaseWithCustomAlignmentC1ERKS_(
                        ::core::pin::Pin::into_inner_unchecked(dest),
                        __param_0,
                    );
                },
            )
        }
    }
}
impl<'b> ::ctor::CtorNew<(&'b Self,)> for InheritsFromBaseWithCustomAlignment {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: (&'b Self,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'b Self>>::ctor_new(arg)
    }
}

impl<'b> ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>>
    for InheritsFromBaseWithCustomAlignment
{
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'b, Self>) -> Self::CtorType {
        let __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN35InheritsFromBaseWithCustomAlignmentC1EOS_(
                        ::core::pin::Pin::into_inner_unchecked(dest),
                        __param_0,
                    );
                },
            )
        }
    }
}
impl<'b> ::ctor::CtorNew<(::ctor::RvalueReference<'b, Self>,)>
    for InheritsFromBaseWithCustomAlignment
{
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: (::ctor::RvalueReference<'b, Self>,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>>>::ctor_new(arg)
    }
}

impl<'b> ::ctor::Assign<&'b Self> for InheritsFromBaseWithCustomAlignment {
    #[inline(always)]
    fn assign<'a>(self: ::core::pin::Pin<&'a mut Self>, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN35InheritsFromBaseWithCustomAlignmentaSERKS_(
                self, __param_0,
            );
        }
    }
}

impl<'b> ::ctor::Assign<::ctor::RvalueReference<'b, Self>> for InheritsFromBaseWithCustomAlignment {
    #[inline(always)]
    fn assign<'a>(
        self: ::core::pin::Pin<&'a mut Self>,
        __param_0: ::ctor::RvalueReference<'b, Self>,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN35InheritsFromBaseWithCustomAlignmentaSEOS_(
                self, __param_0,
            );
        }
    }
}

unsafe impl oops::Inherits<crate::HasCustomAlignment>
    for crate::InheritsFromBaseWithCustomAlignment
{
    unsafe fn upcast_ptr(derived: *const Self) -> *const crate::HasCustomAlignment {
        (derived as *const _ as *const u8).offset(0) as *const crate::HasCustomAlignment
    }
}

#[::ctor::recursively_pinned]
#[repr(C, align(64))]
pub struct HasCustomAlignmentWithGnuAttr {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 64],
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("HasCustomAlignmentWithGnuAttr"),
    crate::HasCustomAlignmentWithGnuAttr
);

impl ::ctor::CtorNew<()> for HasCustomAlignmentWithGnuAttr {
    type CtorType = impl ::ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN29HasCustomAlignmentWithGnuAttrC1Ev(
                        ::core::pin::Pin::into_inner_unchecked(dest),
                    );
                },
            )
        }
    }
}

impl<'b> ::ctor::CtorNew<&'b Self> for HasCustomAlignmentWithGnuAttr {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: &'b Self) -> Self::CtorType {
        let __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN29HasCustomAlignmentWithGnuAttrC1ERKS_(
                        ::core::pin::Pin::into_inner_unchecked(dest),
                        __param_0,
                    );
                },
            )
        }
    }
}
impl<'b> ::ctor::CtorNew<(&'b Self,)> for HasCustomAlignmentWithGnuAttr {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: (&'b Self,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'b Self>>::ctor_new(arg)
    }
}

impl<'b> ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>> for HasCustomAlignmentWithGnuAttr {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'b, Self>) -> Self::CtorType {
        let __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN29HasCustomAlignmentWithGnuAttrC1EOS_(
                        ::core::pin::Pin::into_inner_unchecked(dest),
                        __param_0,
                    );
                },
            )
        }
    }
}
impl<'b> ::ctor::CtorNew<(::ctor::RvalueReference<'b, Self>,)> for HasCustomAlignmentWithGnuAttr {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: (::ctor::RvalueReference<'b, Self>,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>>>::ctor_new(arg)
    }
}

impl<'b> ::ctor::Assign<&'b Self> for HasCustomAlignmentWithGnuAttr {
    #[inline(always)]
    fn assign<'a>(self: ::core::pin::Pin<&'a mut Self>, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN29HasCustomAlignmentWithGnuAttraSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::Assign<::ctor::RvalueReference<'b, Self>> for HasCustomAlignmentWithGnuAttr {
    #[inline(always)]
    fn assign<'a>(
        self: ::core::pin::Pin<&'a mut Self>,
        __param_0: ::ctor::RvalueReference<'b, Self>,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN29HasCustomAlignmentWithGnuAttraSEOS_(self, __param_0);
        }
    }
}

pub mod template_with_preferred_name {
    // Error while generating bindings for item 'template_with_preferred_name::SomeTemplate':
    // Class templates are not supported yet

    /// Based on `llvm/include/c++/v1/__fwd/string_view.h` - mimics
    /// definition of the `string_view` type alias.
    pub type SpecializedTypeAlias =
        crate::__CcTemplateInstN28template_with_preferred_name12SomeTemplateIiEE;

    // Based on `llvm/include/c++/v1/string_view` - mimics definition of
    // `basic_string_view` class template (focusing on the attributes related to the
    // preferred name).
}

// namespace template_with_preferred_name

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_CLANG_ATTRS_H_

/// Based on `llvm/include/c++/v1/__fwd/string_view.h` - mimics
/// forward declaration of `basic_string_view` class template.
#[::ctor::recursively_pinned]
#[repr(C)]
pub struct __CcTemplateInstN28template_with_preferred_name12SomeTemplateIiEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("template_with_preferred_name::SomeTemplate<int>"),
    crate::__CcTemplateInstN28template_with_preferred_name12SomeTemplateIiEE
);

impl ::ctor::CtorNew<()> for __CcTemplateInstN28template_with_preferred_name12SomeTemplateIiEE {
    type CtorType = impl ::ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN28template_with_preferred_name12SomeTemplateIiEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3aclang_5fattrs_5fcc(::core::pin::Pin::into_inner_unchecked(dest));
                },
            )
        }
    }
}

impl<'b> ::ctor::CtorNew<&'b Self>
    for __CcTemplateInstN28template_with_preferred_name12SomeTemplateIiEE
{
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: &'b Self) -> Self::CtorType {
        let __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN28template_with_preferred_name12SomeTemplateIiEC1ERKS1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3aclang_5fattrs_5fcc(::core::pin::Pin::into_inner_unchecked(dest),__param_0);
                },
            )
        }
    }
}
impl<'b> ::ctor::CtorNew<(&'b Self,)>
    for __CcTemplateInstN28template_with_preferred_name12SomeTemplateIiEE
{
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: (&'b Self,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'b Self>>::ctor_new(arg)
    }
}

impl<'b> ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>>
    for __CcTemplateInstN28template_with_preferred_name12SomeTemplateIiEE
{
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'b, Self>) -> Self::CtorType {
        let __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN28template_with_preferred_name12SomeTemplateIiEC1EOS1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3aclang_5fattrs_5fcc(::core::pin::Pin::into_inner_unchecked(dest),__param_0);
                },
            )
        }
    }
}
impl<'b> ::ctor::CtorNew<(::ctor::RvalueReference<'b, Self>,)>
    for __CcTemplateInstN28template_with_preferred_name12SomeTemplateIiEE
{
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: (::ctor::RvalueReference<'b, Self>,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>>>::ctor_new(arg)
    }
}

impl<'b> ::ctor::Assign<&'b Self>
    for __CcTemplateInstN28template_with_preferred_name12SomeTemplateIiEE
{
    #[inline(always)]
    fn assign<'a>(self: ::core::pin::Pin<&'a mut Self>, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN28template_with_preferred_name12SomeTemplateIiEaSERKS1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3aclang_5fattrs_5fcc(self,__param_0);
        }
    }
}

impl<'b> ::ctor::Assign<::ctor::RvalueReference<'b, Self>>
    for __CcTemplateInstN28template_with_preferred_name12SomeTemplateIiEE
{
    #[inline(always)]
    fn assign<'a>(
        self: ::core::pin::Pin<&'a mut Self>,
        __param_0: ::ctor::RvalueReference<'b, Self>,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN28template_with_preferred_name12SomeTemplateIiEaSEOS1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3aclang_5fattrs_5fcc(self,__param_0);
        }
    }
}

impl __CcTemplateInstN28template_with_preferred_name12SomeTemplateIiEE {
    #[inline(always)]
    pub fn foo<'a>(self: ::core::pin::Pin<&'a mut Self>) -> i32 {
        unsafe {
            crate::detail::__rust_thunk___ZN28template_with_preferred_name12SomeTemplateIiE3fooEv__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3aclang_5fattrs_5fcc(self)
        }
    }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    extern "C" {
        pub(crate) fn __rust_thunk___ZN18HasCustomAlignmentC1Ev<'a>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::HasCustomAlignment>,
        );
        pub(crate) fn __rust_thunk___ZN18HasCustomAlignmentC1ERKS_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::HasCustomAlignment>,
            __param_0: &'b crate::HasCustomAlignment,
        );
        pub(crate) fn __rust_thunk___ZN18HasCustomAlignmentC1EOS_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::HasCustomAlignment>,
            __param_0: ::ctor::RvalueReference<'b, crate::HasCustomAlignment>,
        );
        pub(crate) fn __rust_thunk___ZN18HasCustomAlignmentaSERKS_<'a, 'b>(
            __this: ::core::pin::Pin<&'a mut crate::HasCustomAlignment>,
            __param_0: &'b crate::HasCustomAlignment,
        ) -> ::core::pin::Pin<&'a mut crate::HasCustomAlignment>;
        pub(crate) fn __rust_thunk___ZN18HasCustomAlignmentaSEOS_<'a, 'b>(
            __this: ::core::pin::Pin<&'a mut crate::HasCustomAlignment>,
            __param_0: ::ctor::RvalueReference<'b, crate::HasCustomAlignment>,
        ) -> ::core::pin::Pin<&'a mut crate::HasCustomAlignment>;
        pub(crate) fn __rust_thunk___ZN27HasFieldWithCustomAlignmentC1Ev<'a>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::HasFieldWithCustomAlignment>,
        );
        pub(crate) fn __rust_thunk___ZN27HasFieldWithCustomAlignmentC1ERKS_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::HasFieldWithCustomAlignment>,
            __param_0: &'b crate::HasFieldWithCustomAlignment,
        );
        pub(crate) fn __rust_thunk___ZN27HasFieldWithCustomAlignmentC1EOS_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::HasFieldWithCustomAlignment>,
            __param_0: ::ctor::RvalueReference<'b, crate::HasFieldWithCustomAlignment>,
        );
        pub(crate) fn __rust_thunk___ZN27HasFieldWithCustomAlignmentaSERKS_<'a, 'b>(
            __this: ::core::pin::Pin<&'a mut crate::HasFieldWithCustomAlignment>,
            __param_0: &'b crate::HasFieldWithCustomAlignment,
        ) -> ::core::pin::Pin<&'a mut crate::HasFieldWithCustomAlignment>;
        pub(crate) fn __rust_thunk___ZN27HasFieldWithCustomAlignmentaSEOS_<'a, 'b>(
            __this: ::core::pin::Pin<&'a mut crate::HasFieldWithCustomAlignment>,
            __param_0: ::ctor::RvalueReference<'b, crate::HasFieldWithCustomAlignment>,
        ) -> ::core::pin::Pin<&'a mut crate::HasFieldWithCustomAlignment>;
        pub(crate) fn __rust_thunk___ZN35InheritsFromBaseWithCustomAlignmentC1Ev<'a>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::InheritsFromBaseWithCustomAlignment>,
        );
        pub(crate) fn __rust_thunk___ZN35InheritsFromBaseWithCustomAlignmentC1ERKS_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::InheritsFromBaseWithCustomAlignment>,
            __param_0: &'b crate::InheritsFromBaseWithCustomAlignment,
        );
        pub(crate) fn __rust_thunk___ZN35InheritsFromBaseWithCustomAlignmentC1EOS_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::InheritsFromBaseWithCustomAlignment>,
            __param_0: ::ctor::RvalueReference<'b, crate::InheritsFromBaseWithCustomAlignment>,
        );
        pub(crate) fn __rust_thunk___ZN35InheritsFromBaseWithCustomAlignmentaSERKS_<'a, 'b>(
            __this: ::core::pin::Pin<&'a mut crate::InheritsFromBaseWithCustomAlignment>,
            __param_0: &'b crate::InheritsFromBaseWithCustomAlignment,
        ) -> ::core::pin::Pin<&'a mut crate::InheritsFromBaseWithCustomAlignment>;
        pub(crate) fn __rust_thunk___ZN35InheritsFromBaseWithCustomAlignmentaSEOS_<'a, 'b>(
            __this: ::core::pin::Pin<&'a mut crate::InheritsFromBaseWithCustomAlignment>,
            __param_0: ::ctor::RvalueReference<'b, crate::InheritsFromBaseWithCustomAlignment>,
        ) -> ::core::pin::Pin<&'a mut crate::InheritsFromBaseWithCustomAlignment>;
        pub(crate) fn __rust_thunk___ZN29HasCustomAlignmentWithGnuAttrC1Ev<'a>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::HasCustomAlignmentWithGnuAttr>,
        );
        pub(crate) fn __rust_thunk___ZN29HasCustomAlignmentWithGnuAttrC1ERKS_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::HasCustomAlignmentWithGnuAttr>,
            __param_0: &'b crate::HasCustomAlignmentWithGnuAttr,
        );
        pub(crate) fn __rust_thunk___ZN29HasCustomAlignmentWithGnuAttrC1EOS_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::HasCustomAlignmentWithGnuAttr>,
            __param_0: ::ctor::RvalueReference<'b, crate::HasCustomAlignmentWithGnuAttr>,
        );
        pub(crate) fn __rust_thunk___ZN29HasCustomAlignmentWithGnuAttraSERKS_<'a, 'b>(
            __this: ::core::pin::Pin<&'a mut crate::HasCustomAlignmentWithGnuAttr>,
            __param_0: &'b crate::HasCustomAlignmentWithGnuAttr,
        ) -> ::core::pin::Pin<&'a mut crate::HasCustomAlignmentWithGnuAttr>;
        pub(crate) fn __rust_thunk___ZN29HasCustomAlignmentWithGnuAttraSEOS_<'a, 'b>(
            __this: ::core::pin::Pin<&'a mut crate::HasCustomAlignmentWithGnuAttr>,
            __param_0: ::ctor::RvalueReference<'b, crate::HasCustomAlignmentWithGnuAttr>,
        ) -> ::core::pin::Pin<&'a mut crate::HasCustomAlignmentWithGnuAttr>;
        pub(crate) fn __rust_thunk___ZN28template_with_preferred_name12SomeTemplateIiEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3aclang_5fattrs_5fcc<
            'a,
        >(
            __this: &'a mut ::core::mem::MaybeUninit<
                crate::__CcTemplateInstN28template_with_preferred_name12SomeTemplateIiEE,
            >,
        );
        pub(crate) fn __rust_thunk___ZN28template_with_preferred_name12SomeTemplateIiEC1ERKS1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3aclang_5fattrs_5fcc<
            'a,
            'b,
        >(
            __this: &'a mut ::core::mem::MaybeUninit<
                crate::__CcTemplateInstN28template_with_preferred_name12SomeTemplateIiEE,
            >,
            __param_0: &'b crate::__CcTemplateInstN28template_with_preferred_name12SomeTemplateIiEE,
        );
        pub(crate) fn __rust_thunk___ZN28template_with_preferred_name12SomeTemplateIiEC1EOS1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3aclang_5fattrs_5fcc<
            'a,
            'b,
        >(
            __this: &'a mut ::core::mem::MaybeUninit<
                crate::__CcTemplateInstN28template_with_preferred_name12SomeTemplateIiEE,
            >,
            __param_0: ::ctor::RvalueReference<
                'b,
                crate::__CcTemplateInstN28template_with_preferred_name12SomeTemplateIiEE,
            >,
        );
        pub(crate) fn __rust_thunk___ZN28template_with_preferred_name12SomeTemplateIiEaSERKS1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3aclang_5fattrs_5fcc<
            'a,
            'b,
        >(
            __this: ::core::pin::Pin<
                &'a mut crate::__CcTemplateInstN28template_with_preferred_name12SomeTemplateIiEE,
            >,
            __param_0: &'b crate::__CcTemplateInstN28template_with_preferred_name12SomeTemplateIiEE,
        ) -> ::core::pin::Pin<
            &'a mut crate::__CcTemplateInstN28template_with_preferred_name12SomeTemplateIiEE,
        >;
        pub(crate) fn __rust_thunk___ZN28template_with_preferred_name12SomeTemplateIiEaSEOS1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3aclang_5fattrs_5fcc<
            'a,
            'b,
        >(
            __this: ::core::pin::Pin<
                &'a mut crate::__CcTemplateInstN28template_with_preferred_name12SomeTemplateIiEE,
            >,
            __param_0: ::ctor::RvalueReference<
                'b,
                crate::__CcTemplateInstN28template_with_preferred_name12SomeTemplateIiEE,
            >,
        ) -> ::core::pin::Pin<
            &'a mut crate::__CcTemplateInstN28template_with_preferred_name12SomeTemplateIiEE,
        >;
        pub(crate) fn __rust_thunk___ZN28template_with_preferred_name12SomeTemplateIiE3fooEv__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3aclang_5fattrs_5fcc<
            'a,
        >(
            __this: ::core::pin::Pin<
                &'a mut crate::__CcTemplateInstN28template_with_preferred_name12SomeTemplateIiEE,
            >,
        ) -> i32;
    }
}

const _: () = assert!(::core::mem::size_of::<Option<&i32>>() == ::core::mem::size_of::<&i32>());

const _: () = assert!(::core::mem::size_of::<crate::HasCustomAlignment>() == 64);
const _: () = assert!(::core::mem::align_of::<crate::HasCustomAlignment>() == 64);
const _: () = {
    static_assertions::assert_not_impl_any!(crate::HasCustomAlignment: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::HasCustomAlignment: Drop);
};

const _: () = assert!(::core::mem::size_of::<crate::HasFieldWithCustomAlignment>() == 64);
const _: () = assert!(::core::mem::align_of::<crate::HasFieldWithCustomAlignment>() == 64);
const _: () = {
    static_assertions::assert_not_impl_any!(crate::HasFieldWithCustomAlignment: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::HasFieldWithCustomAlignment: Drop);
};
const _: () = assert!(memoffset::offset_of!(crate::HasFieldWithCustomAlignment, field) == 0);

const _: () = assert!(::core::mem::size_of::<crate::InheritsFromBaseWithCustomAlignment>() == 64);
const _: () = assert!(::core::mem::align_of::<crate::InheritsFromBaseWithCustomAlignment>() == 64);
const _: () = {
    static_assertions::assert_not_impl_any!(crate::InheritsFromBaseWithCustomAlignment: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::InheritsFromBaseWithCustomAlignment: Drop);
};

const _: () = assert!(::core::mem::size_of::<crate::HasCustomAlignmentWithGnuAttr>() == 64);
const _: () = assert!(::core::mem::align_of::<crate::HasCustomAlignmentWithGnuAttr>() == 64);
const _: () = {
    static_assertions::assert_not_impl_any!(crate::HasCustomAlignmentWithGnuAttr: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::HasCustomAlignmentWithGnuAttr: Drop);
};

const _: () = assert!(
    ::core::mem::size_of::<crate::__CcTemplateInstN28template_with_preferred_name12SomeTemplateIiEE>(
    ) == 1
);
const _: () = assert!(
    ::core::mem::align_of::<crate::__CcTemplateInstN28template_with_preferred_name12SomeTemplateIiEE>(
    ) == 1
);
const _: () = {
    static_assertions::assert_not_impl_any!(
        crate::__CcTemplateInstN28template_with_preferred_name12SomeTemplateIiEE: Copy
    );
};
const _: () = {
    static_assertions::assert_not_impl_any!(
        crate::__CcTemplateInstN28template_with_preferred_name12SomeTemplateIiEE: Drop
    );
};
