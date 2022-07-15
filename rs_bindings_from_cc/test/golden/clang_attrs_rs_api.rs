// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for C++ target
// //rs_bindings_from_cc/test/golden:clang_attrs_cc
#![rustfmt::skip]
#![feature(const_ptr_offset_from, custom_inner_attributes, negative_impls, type_alias_impl_trait)]
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
    __non_field_data: [::std::mem::MaybeUninit<u8>; 64],
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
        ::ctor::FnCtor::new(
            move |dest: ::std::pin::Pin<&mut ::std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN18HasCustomAlignmentC1Ev(
                    ::std::pin::Pin::into_inner_unchecked(dest),
                );
            },
        )
    }
}

impl<'b> ::ctor::CtorNew<&'b crate::HasCustomAlignment> for HasCustomAlignment {
    type CtorType = impl ::ctor::Ctor<Output = Self> + 'b;
    #[inline(always)]
    fn ctor_new(args: &'b crate::HasCustomAlignment) -> Self::CtorType {
        let __param_0 = args;
        ::ctor::FnCtor::new(
            move |dest: ::std::pin::Pin<&mut ::std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN18HasCustomAlignmentC1ERKS_(
                    ::std::pin::Pin::into_inner_unchecked(dest),
                    __param_0,
                );
            },
        )
    }
}
impl<'b> ::ctor::CtorNew<(&'b crate::HasCustomAlignment,)> for HasCustomAlignment {
    type CtorType = impl ::ctor::Ctor<Output = Self> + 'b;
    #[inline(always)]
    fn ctor_new(args: (&'b crate::HasCustomAlignment,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'b crate::HasCustomAlignment>>::ctor_new(arg)
    }
}

impl<'b> ::ctor::CtorNew<::ctor::RvalueReference<'b, crate::HasCustomAlignment>>
    for HasCustomAlignment
{
    type CtorType = impl ::ctor::Ctor<Output = Self> + 'b;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'b, crate::HasCustomAlignment>) -> Self::CtorType {
        let __param_0 = args;
        ::ctor::FnCtor::new(
            move |dest: ::std::pin::Pin<&mut ::std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN18HasCustomAlignmentC1EOS_(
                    ::std::pin::Pin::into_inner_unchecked(dest),
                    __param_0,
                );
            },
        )
    }
}
impl<'b> ::ctor::CtorNew<(::ctor::RvalueReference<'b, crate::HasCustomAlignment>,)>
    for HasCustomAlignment
{
    type CtorType = impl ::ctor::Ctor<Output = Self> + 'b;
    #[inline(always)]
    fn ctor_new(args: (::ctor::RvalueReference<'b, crate::HasCustomAlignment>,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ctor::RvalueReference<'b, crate::HasCustomAlignment>>>::ctor_new(
            arg,
        )
    }
}

impl<'b> ::ctor::Assign<&'b crate::HasCustomAlignment> for HasCustomAlignment {
    #[inline(always)]
    fn assign<'a>(self: ::std::pin::Pin<&'a mut Self>, __param_0: &'b crate::HasCustomAlignment) {
        unsafe {
            crate::detail::__rust_thunk___ZN18HasCustomAlignmentaSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::Assign<::ctor::RvalueReference<'b, crate::HasCustomAlignment>>
    for HasCustomAlignment
{
    #[inline(always)]
    fn assign<'a>(
        self: ::std::pin::Pin<&'a mut Self>,
        __param_0: ::ctor::RvalueReference<'b, crate::HasCustomAlignment>,
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
        ::ctor::FnCtor::new(
            move |dest: ::std::pin::Pin<&mut ::std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN27HasFieldWithCustomAlignmentC1Ev(
                    ::std::pin::Pin::into_inner_unchecked(dest),
                );
            },
        )
    }
}

impl<'b> ::ctor::CtorNew<&'b crate::HasFieldWithCustomAlignment> for HasFieldWithCustomAlignment {
    type CtorType = impl ::ctor::Ctor<Output = Self> + 'b;
    #[inline(always)]
    fn ctor_new(args: &'b crate::HasFieldWithCustomAlignment) -> Self::CtorType {
        let __param_0 = args;
        ::ctor::FnCtor::new(
            move |dest: ::std::pin::Pin<&mut ::std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN27HasFieldWithCustomAlignmentC1ERKS_(
                    ::std::pin::Pin::into_inner_unchecked(dest),
                    __param_0,
                );
            },
        )
    }
}
impl<'b> ::ctor::CtorNew<(&'b crate::HasFieldWithCustomAlignment,)>
    for HasFieldWithCustomAlignment
{
    type CtorType = impl ::ctor::Ctor<Output = Self> + 'b;
    #[inline(always)]
    fn ctor_new(args: (&'b crate::HasFieldWithCustomAlignment,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'b crate::HasFieldWithCustomAlignment>>::ctor_new(arg)
    }
}

impl<'b> ::ctor::CtorNew<::ctor::RvalueReference<'b, crate::HasFieldWithCustomAlignment>>
    for HasFieldWithCustomAlignment
{
    type CtorType = impl ::ctor::Ctor<Output = Self> + 'b;
    #[inline(always)]
    fn ctor_new(
        args: ::ctor::RvalueReference<'b, crate::HasFieldWithCustomAlignment>,
    ) -> Self::CtorType {
        let __param_0 = args;
        ::ctor::FnCtor::new(
            move |dest: ::std::pin::Pin<&mut ::std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN27HasFieldWithCustomAlignmentC1EOS_(
                    ::std::pin::Pin::into_inner_unchecked(dest),
                    __param_0,
                );
            },
        )
    }
}
impl<'b> ::ctor::CtorNew<(::ctor::RvalueReference<'b, crate::HasFieldWithCustomAlignment>,)>
    for HasFieldWithCustomAlignment
{
    type CtorType = impl ::ctor::Ctor<Output = Self> + 'b;
    #[inline(always)]
    fn ctor_new(
        args: (::ctor::RvalueReference<'b, crate::HasFieldWithCustomAlignment>,),
    ) -> Self::CtorType {
        let (arg,) = args;
        <Self as::ctor::CtorNew<::ctor::RvalueReference<'b,crate::HasFieldWithCustomAlignment>>>::ctor_new(arg)
    }
}

impl<'b> ::ctor::Assign<&'b crate::HasFieldWithCustomAlignment> for HasFieldWithCustomAlignment {
    #[inline(always)]
    fn assign<'a>(
        self: ::std::pin::Pin<&'a mut Self>,
        __param_0: &'b crate::HasFieldWithCustomAlignment,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN27HasFieldWithCustomAlignmentaSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::Assign<::ctor::RvalueReference<'b, crate::HasFieldWithCustomAlignment>>
    for HasFieldWithCustomAlignment
{
    #[inline(always)]
    fn assign<'a>(
        self: ::std::pin::Pin<&'a mut Self>,
        __param_0: ::ctor::RvalueReference<'b, crate::HasFieldWithCustomAlignment>,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN27HasFieldWithCustomAlignmentaSEOS_(self, __param_0);
        }
    }
}

#[::ctor::recursively_pinned]
#[repr(C, align(64))]
pub struct InheritsFromBaseWithCustomAlignment {
    __non_field_data: [::std::mem::MaybeUninit<u8>; 64],
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
        ::ctor::FnCtor::new(
            move |dest: ::std::pin::Pin<&mut ::std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN35InheritsFromBaseWithCustomAlignmentC1Ev(
                    ::std::pin::Pin::into_inner_unchecked(dest),
                );
            },
        )
    }
}

impl<'b> ::ctor::CtorNew<&'b crate::InheritsFromBaseWithCustomAlignment>
    for InheritsFromBaseWithCustomAlignment
{
    type CtorType = impl ::ctor::Ctor<Output = Self> + 'b;
    #[inline(always)]
    fn ctor_new(args: &'b crate::InheritsFromBaseWithCustomAlignment) -> Self::CtorType {
        let __param_0 = args;
        ::ctor::FnCtor::new(
            move |dest: ::std::pin::Pin<&mut ::std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN35InheritsFromBaseWithCustomAlignmentC1ERKS_(
                    ::std::pin::Pin::into_inner_unchecked(dest),
                    __param_0,
                );
            },
        )
    }
}
impl<'b> ::ctor::CtorNew<(&'b crate::InheritsFromBaseWithCustomAlignment,)>
    for InheritsFromBaseWithCustomAlignment
{
    type CtorType = impl ::ctor::Ctor<Output = Self> + 'b;
    #[inline(always)]
    fn ctor_new(args: (&'b crate::InheritsFromBaseWithCustomAlignment,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'b crate::InheritsFromBaseWithCustomAlignment>>::ctor_new(arg)
    }
}

impl<'b> ::ctor::CtorNew<::ctor::RvalueReference<'b, crate::InheritsFromBaseWithCustomAlignment>>
    for InheritsFromBaseWithCustomAlignment
{
    type CtorType = impl ::ctor::Ctor<Output = Self> + 'b;
    #[inline(always)]
    fn ctor_new(
        args: ::ctor::RvalueReference<'b, crate::InheritsFromBaseWithCustomAlignment>,
    ) -> Self::CtorType {
        let __param_0 = args;
        ::ctor::FnCtor::new(
            move |dest: ::std::pin::Pin<&mut ::std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN35InheritsFromBaseWithCustomAlignmentC1EOS_(
                    ::std::pin::Pin::into_inner_unchecked(dest),
                    __param_0,
                );
            },
        )
    }
}
impl<'b> ::ctor::CtorNew<(::ctor::RvalueReference<'b, crate::InheritsFromBaseWithCustomAlignment>,)>
    for InheritsFromBaseWithCustomAlignment
{
    type CtorType = impl ::ctor::Ctor<Output = Self> + 'b;
    #[inline(always)]
    fn ctor_new(
        args: (::ctor::RvalueReference<'b, crate::InheritsFromBaseWithCustomAlignment>,),
    ) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<
            ::ctor::RvalueReference<'b, crate::InheritsFromBaseWithCustomAlignment>,
        >>::ctor_new(arg)
    }
}

impl<'b> ::ctor::Assign<&'b crate::InheritsFromBaseWithCustomAlignment>
    for InheritsFromBaseWithCustomAlignment
{
    #[inline(always)]
    fn assign<'a>(
        self: ::std::pin::Pin<&'a mut Self>,
        __param_0: &'b crate::InheritsFromBaseWithCustomAlignment,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN35InheritsFromBaseWithCustomAlignmentaSERKS_(
                self, __param_0,
            );
        }
    }
}

impl<'b> ::ctor::Assign<::ctor::RvalueReference<'b, crate::InheritsFromBaseWithCustomAlignment>>
    for InheritsFromBaseWithCustomAlignment
{
    #[inline(always)]
    fn assign<'a>(
        self: ::std::pin::Pin<&'a mut Self>,
        __param_0: ::ctor::RvalueReference<'b, crate::InheritsFromBaseWithCustomAlignment>,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN35InheritsFromBaseWithCustomAlignmentaSEOS_(
                self, __param_0,
            );
        }
    }
}

unsafe impl oops::Inherits<crate::HasCustomAlignment> for InheritsFromBaseWithCustomAlignment {
    unsafe fn upcast_ptr(derived: *const Self) -> *const crate::HasCustomAlignment {
        (derived as *const _ as *const u8).offset(0) as *const crate::HasCustomAlignment
    }
}

#[::ctor::recursively_pinned]
#[repr(C, align(64))]
pub struct HasCustomAlignmentWithGnuAttr {
    __non_field_data: [::std::mem::MaybeUninit<u8>; 64],
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
        ::ctor::FnCtor::new(
            move |dest: ::std::pin::Pin<&mut ::std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN29HasCustomAlignmentWithGnuAttrC1Ev(
                    ::std::pin::Pin::into_inner_unchecked(dest),
                );
            },
        )
    }
}

impl<'b> ::ctor::CtorNew<&'b crate::HasCustomAlignmentWithGnuAttr>
    for HasCustomAlignmentWithGnuAttr
{
    type CtorType = impl ::ctor::Ctor<Output = Self> + 'b;
    #[inline(always)]
    fn ctor_new(args: &'b crate::HasCustomAlignmentWithGnuAttr) -> Self::CtorType {
        let __param_0 = args;
        ::ctor::FnCtor::new(
            move |dest: ::std::pin::Pin<&mut ::std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN29HasCustomAlignmentWithGnuAttrC1ERKS_(
                    ::std::pin::Pin::into_inner_unchecked(dest),
                    __param_0,
                );
            },
        )
    }
}
impl<'b> ::ctor::CtorNew<(&'b crate::HasCustomAlignmentWithGnuAttr,)>
    for HasCustomAlignmentWithGnuAttr
{
    type CtorType = impl ::ctor::Ctor<Output = Self> + 'b;
    #[inline(always)]
    fn ctor_new(args: (&'b crate::HasCustomAlignmentWithGnuAttr,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'b crate::HasCustomAlignmentWithGnuAttr>>::ctor_new(arg)
    }
}

impl<'b> ::ctor::CtorNew<::ctor::RvalueReference<'b, crate::HasCustomAlignmentWithGnuAttr>>
    for HasCustomAlignmentWithGnuAttr
{
    type CtorType = impl ::ctor::Ctor<Output = Self> + 'b;
    #[inline(always)]
    fn ctor_new(
        args: ::ctor::RvalueReference<'b, crate::HasCustomAlignmentWithGnuAttr>,
    ) -> Self::CtorType {
        let __param_0 = args;
        ::ctor::FnCtor::new(
            move |dest: ::std::pin::Pin<&mut ::std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN29HasCustomAlignmentWithGnuAttrC1EOS_(
                    ::std::pin::Pin::into_inner_unchecked(dest),
                    __param_0,
                );
            },
        )
    }
}
impl<'b> ::ctor::CtorNew<(::ctor::RvalueReference<'b, crate::HasCustomAlignmentWithGnuAttr>,)>
    for HasCustomAlignmentWithGnuAttr
{
    type CtorType = impl ::ctor::Ctor<Output = Self> + 'b;
    #[inline(always)]
    fn ctor_new(
        args: (::ctor::RvalueReference<'b, crate::HasCustomAlignmentWithGnuAttr>,),
    ) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<
            ::ctor::RvalueReference<'b, crate::HasCustomAlignmentWithGnuAttr>,
        >>::ctor_new(arg)
    }
}

impl<'b> ::ctor::Assign<&'b crate::HasCustomAlignmentWithGnuAttr>
    for HasCustomAlignmentWithGnuAttr
{
    #[inline(always)]
    fn assign<'a>(
        self: ::std::pin::Pin<&'a mut Self>,
        __param_0: &'b crate::HasCustomAlignmentWithGnuAttr,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN29HasCustomAlignmentWithGnuAttraSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::Assign<::ctor::RvalueReference<'b, crate::HasCustomAlignmentWithGnuAttr>>
    for HasCustomAlignmentWithGnuAttr
{
    #[inline(always)]
    fn assign<'a>(
        self: ::std::pin::Pin<&'a mut Self>,
        __param_0: ::ctor::RvalueReference<'b, crate::HasCustomAlignmentWithGnuAttr>,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN29HasCustomAlignmentWithGnuAttraSEOS_(self, __param_0);
        }
    }
}

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_CLANG_ATTRS_H_

mod detail {
    #[allow(unused_imports)]
    use super::*;
    extern "C" {
        pub(crate) fn __rust_thunk___ZN18HasCustomAlignmentC1Ev<'a>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::HasCustomAlignment>,
        );
        pub(crate) fn __rust_thunk___ZN18HasCustomAlignmentC1ERKS_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::HasCustomAlignment>,
            __param_0: &'b crate::HasCustomAlignment,
        );
        pub(crate) fn __rust_thunk___ZN18HasCustomAlignmentC1EOS_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::HasCustomAlignment>,
            __param_0: ::ctor::RvalueReference<'b, crate::HasCustomAlignment>,
        );
        pub(crate) fn __rust_thunk___ZN18HasCustomAlignmentaSERKS_<'a, 'b>(
            __this: ::std::pin::Pin<&'a mut crate::HasCustomAlignment>,
            __param_0: &'b crate::HasCustomAlignment,
        ) -> ::std::pin::Pin<&'a mut crate::HasCustomAlignment>;
        pub(crate) fn __rust_thunk___ZN18HasCustomAlignmentaSEOS_<'a, 'b>(
            __this: ::std::pin::Pin<&'a mut crate::HasCustomAlignment>,
            __param_0: ::ctor::RvalueReference<'b, crate::HasCustomAlignment>,
        ) -> ::std::pin::Pin<&'a mut crate::HasCustomAlignment>;
        pub(crate) fn __rust_thunk___ZN27HasFieldWithCustomAlignmentC1Ev<'a>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::HasFieldWithCustomAlignment>,
        );
        pub(crate) fn __rust_thunk___ZN27HasFieldWithCustomAlignmentC1ERKS_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::HasFieldWithCustomAlignment>,
            __param_0: &'b crate::HasFieldWithCustomAlignment,
        );
        pub(crate) fn __rust_thunk___ZN27HasFieldWithCustomAlignmentC1EOS_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::HasFieldWithCustomAlignment>,
            __param_0: ::ctor::RvalueReference<'b, crate::HasFieldWithCustomAlignment>,
        );
        pub(crate) fn __rust_thunk___ZN27HasFieldWithCustomAlignmentaSERKS_<'a, 'b>(
            __this: ::std::pin::Pin<&'a mut crate::HasFieldWithCustomAlignment>,
            __param_0: &'b crate::HasFieldWithCustomAlignment,
        ) -> ::std::pin::Pin<&'a mut crate::HasFieldWithCustomAlignment>;
        pub(crate) fn __rust_thunk___ZN27HasFieldWithCustomAlignmentaSEOS_<'a, 'b>(
            __this: ::std::pin::Pin<&'a mut crate::HasFieldWithCustomAlignment>,
            __param_0: ::ctor::RvalueReference<'b, crate::HasFieldWithCustomAlignment>,
        ) -> ::std::pin::Pin<&'a mut crate::HasFieldWithCustomAlignment>;
        pub(crate) fn __rust_thunk___ZN35InheritsFromBaseWithCustomAlignmentC1Ev<'a>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::InheritsFromBaseWithCustomAlignment>,
        );
        pub(crate) fn __rust_thunk___ZN35InheritsFromBaseWithCustomAlignmentC1ERKS_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::InheritsFromBaseWithCustomAlignment>,
            __param_0: &'b crate::InheritsFromBaseWithCustomAlignment,
        );
        pub(crate) fn __rust_thunk___ZN35InheritsFromBaseWithCustomAlignmentC1EOS_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::InheritsFromBaseWithCustomAlignment>,
            __param_0: ::ctor::RvalueReference<'b, crate::InheritsFromBaseWithCustomAlignment>,
        );
        pub(crate) fn __rust_thunk___ZN35InheritsFromBaseWithCustomAlignmentaSERKS_<'a, 'b>(
            __this: ::std::pin::Pin<&'a mut crate::InheritsFromBaseWithCustomAlignment>,
            __param_0: &'b crate::InheritsFromBaseWithCustomAlignment,
        ) -> ::std::pin::Pin<&'a mut crate::InheritsFromBaseWithCustomAlignment>;
        pub(crate) fn __rust_thunk___ZN35InheritsFromBaseWithCustomAlignmentaSEOS_<'a, 'b>(
            __this: ::std::pin::Pin<&'a mut crate::InheritsFromBaseWithCustomAlignment>,
            __param_0: ::ctor::RvalueReference<'b, crate::InheritsFromBaseWithCustomAlignment>,
        ) -> ::std::pin::Pin<&'a mut crate::InheritsFromBaseWithCustomAlignment>;
        pub(crate) fn __rust_thunk___ZN29HasCustomAlignmentWithGnuAttrC1Ev<'a>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::HasCustomAlignmentWithGnuAttr>,
        );
        pub(crate) fn __rust_thunk___ZN29HasCustomAlignmentWithGnuAttrC1ERKS_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::HasCustomAlignmentWithGnuAttr>,
            __param_0: &'b crate::HasCustomAlignmentWithGnuAttr,
        );
        pub(crate) fn __rust_thunk___ZN29HasCustomAlignmentWithGnuAttrC1EOS_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::HasCustomAlignmentWithGnuAttr>,
            __param_0: ::ctor::RvalueReference<'b, crate::HasCustomAlignmentWithGnuAttr>,
        );
        pub(crate) fn __rust_thunk___ZN29HasCustomAlignmentWithGnuAttraSERKS_<'a, 'b>(
            __this: ::std::pin::Pin<&'a mut crate::HasCustomAlignmentWithGnuAttr>,
            __param_0: &'b crate::HasCustomAlignmentWithGnuAttr,
        ) -> ::std::pin::Pin<&'a mut crate::HasCustomAlignmentWithGnuAttr>;
        pub(crate) fn __rust_thunk___ZN29HasCustomAlignmentWithGnuAttraSEOS_<'a, 'b>(
            __this: ::std::pin::Pin<&'a mut crate::HasCustomAlignmentWithGnuAttr>,
            __param_0: ::ctor::RvalueReference<'b, crate::HasCustomAlignmentWithGnuAttr>,
        ) -> ::std::pin::Pin<&'a mut crate::HasCustomAlignmentWithGnuAttr>;
    }
}

const _: () = assert!(::std::mem::size_of::<Option<&i32>>() == ::std::mem::size_of::<&i32>());

const _: () = assert!(::std::mem::size_of::<crate::HasCustomAlignment>() == 64);
const _: () = assert!(::std::mem::align_of::<crate::HasCustomAlignment>() == 64);
const _: () = {
    static_assertions::assert_not_impl_any!(crate::HasCustomAlignment: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::HasCustomAlignment: Drop);
};

const _: () = assert!(::std::mem::size_of::<crate::HasFieldWithCustomAlignment>() == 64);
const _: () = assert!(::std::mem::align_of::<crate::HasFieldWithCustomAlignment>() == 64);
const _: () = {
    static_assertions::assert_not_impl_any!(crate::HasFieldWithCustomAlignment: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::HasFieldWithCustomAlignment: Drop);
};
const _: () =
    assert!(memoffset_unstable_const::offset_of!(crate::HasFieldWithCustomAlignment, field) == 0);

const _: () = assert!(::std::mem::size_of::<crate::InheritsFromBaseWithCustomAlignment>() == 64);
const _: () = assert!(::std::mem::align_of::<crate::InheritsFromBaseWithCustomAlignment>() == 64);
const _: () = {
    static_assertions::assert_not_impl_any!(crate::InheritsFromBaseWithCustomAlignment: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::InheritsFromBaseWithCustomAlignment: Drop);
};

const _: () = assert!(::std::mem::size_of::<crate::HasCustomAlignmentWithGnuAttr>() == 64);
const _: () = assert!(::std::mem::align_of::<crate::HasCustomAlignmentWithGnuAttr>() == 64);
const _: () = {
    static_assertions::assert_not_impl_any!(crate::HasCustomAlignmentWithGnuAttr: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::HasCustomAlignmentWithGnuAttr: Drop);
};
