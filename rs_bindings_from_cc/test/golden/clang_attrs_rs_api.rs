// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:clang_attrs_cc
// Features: experimental, supported

#![rustfmt::skip]
#![feature(custom_inner_attributes, negative_impls, register_tool)]
#![allow(stable_features)]
#![no_std]
#![register_tool(__crubit)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code)]
#![deny(warnings)]

#[derive(Clone, Copy)]
#[repr(C, align(64))]
#[__crubit::annotate(cpp_type = "HasCustomAlignment")]
pub struct HasCustomAlignment {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 64],
}
impl !Send for HasCustomAlignment {}
impl !Sync for HasCustomAlignment {}
forward_declare::unsafe_define!(
    forward_declare::symbol!("HasCustomAlignment"),
    crate::HasCustomAlignment
);

impl Default for HasCustomAlignment {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN18HasCustomAlignmentC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b> From<::ctor::RvalueReference<'b, Self>> for HasCustomAlignment {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN18HasCustomAlignmentC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

impl<'b> ::ctor::UnpinAssign<&'b Self> for HasCustomAlignment {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN18HasCustomAlignmentaSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::UnpinAssign<::ctor::RvalueReference<'b, Self>> for HasCustomAlignment {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: ::ctor::RvalueReference<'b, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN18HasCustomAlignmentaSEOS_(self, __param_0);
        }
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
#[__crubit::annotate(cpp_type = "HasFieldWithCustomAlignment")]
pub struct HasFieldWithCustomAlignment {
    pub field: crate::HasCustomAlignment,
}
impl !Send for HasFieldWithCustomAlignment {}
impl !Sync for HasFieldWithCustomAlignment {}
forward_declare::unsafe_define!(
    forward_declare::symbol!("HasFieldWithCustomAlignment"),
    crate::HasFieldWithCustomAlignment
);

impl Default for HasFieldWithCustomAlignment {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN27HasFieldWithCustomAlignmentC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b> From<::ctor::RvalueReference<'b, Self>> for HasFieldWithCustomAlignment {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN27HasFieldWithCustomAlignmentC1EOS_(
                &mut tmp, __param_0,
            );
            tmp.assume_init()
        }
    }
}

impl<'b> ::ctor::UnpinAssign<&'b Self> for HasFieldWithCustomAlignment {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN27HasFieldWithCustomAlignmentaSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::UnpinAssign<::ctor::RvalueReference<'b, Self>> for HasFieldWithCustomAlignment {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: ::ctor::RvalueReference<'b, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN27HasFieldWithCustomAlignmentaSEOS_(self, __param_0);
        }
    }
}

#[derive(Clone, Copy)]
#[repr(C, align(64))]
#[__crubit::annotate(cpp_type = "InheritsFromBaseWithCustomAlignment")]
pub struct InheritsFromBaseWithCustomAlignment {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 64],
}
impl !Send for InheritsFromBaseWithCustomAlignment {}
impl !Sync for InheritsFromBaseWithCustomAlignment {}
forward_declare::unsafe_define!(
    forward_declare::symbol!("InheritsFromBaseWithCustomAlignment"),
    crate::InheritsFromBaseWithCustomAlignment
);

impl Default for InheritsFromBaseWithCustomAlignment {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN35InheritsFromBaseWithCustomAlignmentC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b> From<::ctor::RvalueReference<'b, Self>> for InheritsFromBaseWithCustomAlignment {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN35InheritsFromBaseWithCustomAlignmentC1EOS_(
                &mut tmp, __param_0,
            );
            tmp.assume_init()
        }
    }
}

impl<'b> ::ctor::UnpinAssign<&'b Self> for InheritsFromBaseWithCustomAlignment {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN35InheritsFromBaseWithCustomAlignmentaSERKS_(
                self, __param_0,
            );
        }
    }
}

impl<'b> ::ctor::UnpinAssign<::ctor::RvalueReference<'b, Self>>
    for InheritsFromBaseWithCustomAlignment
{
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: ::ctor::RvalueReference<'b, Self>) {
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

#[derive(Clone, Copy)]
#[repr(C, align(64))]
#[__crubit::annotate(cpp_type = "HasCustomAlignmentWithGnuAttr")]
pub struct HasCustomAlignmentWithGnuAttr {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 64],
}
impl !Send for HasCustomAlignmentWithGnuAttr {}
impl !Sync for HasCustomAlignmentWithGnuAttr {}
forward_declare::unsafe_define!(
    forward_declare::symbol!("HasCustomAlignmentWithGnuAttr"),
    crate::HasCustomAlignmentWithGnuAttr
);

impl Default for HasCustomAlignmentWithGnuAttr {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN29HasCustomAlignmentWithGnuAttrC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b> From<::ctor::RvalueReference<'b, Self>> for HasCustomAlignmentWithGnuAttr {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN29HasCustomAlignmentWithGnuAttrC1EOS_(
                &mut tmp, __param_0,
            );
            tmp.assume_init()
        }
    }
}

impl<'b> ::ctor::UnpinAssign<&'b Self> for HasCustomAlignmentWithGnuAttr {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN29HasCustomAlignmentWithGnuAttraSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::UnpinAssign<::ctor::RvalueReference<'b, Self>> for HasCustomAlignmentWithGnuAttr {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: ::ctor::RvalueReference<'b, Self>) {
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

/// Based on `llvm/include/c++/v1/__fwd/string_view.h` - mimics
/// forward declaration of `basic_string_view` class template.
#[derive(Clone, Copy)]
#[repr(C)]
#[__crubit::annotate(cpp_type = "template_with_preferred_name :: SomeTemplate < int >")]
pub struct __CcTemplateInstN28template_with_preferred_name12SomeTemplateIiEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for __CcTemplateInstN28template_with_preferred_name12SomeTemplateIiEE {}
impl !Sync for __CcTemplateInstN28template_with_preferred_name12SomeTemplateIiEE {}
forward_declare::unsafe_define!(
    forward_declare::symbol!("template_with_preferred_name :: SomeTemplate < int >"),
    crate::__CcTemplateInstN28template_with_preferred_name12SomeTemplateIiEE
);

impl Default for __CcTemplateInstN28template_with_preferred_name12SomeTemplateIiEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN28template_with_preferred_name12SomeTemplateIiEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3aclang_5fattrs_5fcc(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b> From<::ctor::RvalueReference<'b, Self>>
    for __CcTemplateInstN28template_with_preferred_name12SomeTemplateIiEE
{
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN28template_with_preferred_name12SomeTemplateIiEC1EOS1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3aclang_5fattrs_5fcc(&mut tmp,__param_0);
            tmp.assume_init()
        }
    }
}

impl<'b> ::ctor::UnpinAssign<&'b Self>
    for __CcTemplateInstN28template_with_preferred_name12SomeTemplateIiEE
{
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN28template_with_preferred_name12SomeTemplateIiEaSERKS1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3aclang_5fattrs_5fcc(self,__param_0);
        }
    }
}

impl<'b> ::ctor::UnpinAssign<::ctor::RvalueReference<'b, Self>>
    for __CcTemplateInstN28template_with_preferred_name12SomeTemplateIiEE
{
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: ::ctor::RvalueReference<'b, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN28template_with_preferred_name12SomeTemplateIiEaSEOS1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3aclang_5fattrs_5fcc(self,__param_0);
        }
    }
}

impl __CcTemplateInstN28template_with_preferred_name12SomeTemplateIiEE {
    #[inline(always)]
    pub fn foo<'a>(&'a mut self) -> ::core::ffi::c_int {
        unsafe {
            crate::detail::__rust_thunk___ZN28template_with_preferred_name12SomeTemplateIiE3fooEv__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3aclang_5fattrs_5fcc(self)
        }
    }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN18HasCustomAlignmentC1Ev<'a>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::HasCustomAlignment>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN18HasCustomAlignmentC1EOS_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::HasCustomAlignment>,
            __param_0: ::ctor::RvalueReference<'b, crate::HasCustomAlignment>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN18HasCustomAlignmentaSERKS_<'a, 'b>(
            __this: &'a mut crate::HasCustomAlignment,
            __param_0: &'b crate::HasCustomAlignment,
        ) -> &'a mut crate::HasCustomAlignment;
        pub(crate) unsafe fn __rust_thunk___ZN18HasCustomAlignmentaSEOS_<'a, 'b>(
            __this: &'a mut crate::HasCustomAlignment,
            __param_0: ::ctor::RvalueReference<'b, crate::HasCustomAlignment>,
        ) -> &'a mut crate::HasCustomAlignment;
        pub(crate) unsafe fn __rust_thunk___ZN27HasFieldWithCustomAlignmentC1Ev<'a>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::HasFieldWithCustomAlignment>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN27HasFieldWithCustomAlignmentC1EOS_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::HasFieldWithCustomAlignment>,
            __param_0: ::ctor::RvalueReference<'b, crate::HasFieldWithCustomAlignment>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN27HasFieldWithCustomAlignmentaSERKS_<'a, 'b>(
            __this: &'a mut crate::HasFieldWithCustomAlignment,
            __param_0: &'b crate::HasFieldWithCustomAlignment,
        ) -> &'a mut crate::HasFieldWithCustomAlignment;
        pub(crate) unsafe fn __rust_thunk___ZN27HasFieldWithCustomAlignmentaSEOS_<'a, 'b>(
            __this: &'a mut crate::HasFieldWithCustomAlignment,
            __param_0: ::ctor::RvalueReference<'b, crate::HasFieldWithCustomAlignment>,
        ) -> &'a mut crate::HasFieldWithCustomAlignment;
        pub(crate) unsafe fn __rust_thunk___ZN35InheritsFromBaseWithCustomAlignmentC1Ev<'a>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::InheritsFromBaseWithCustomAlignment>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN35InheritsFromBaseWithCustomAlignmentC1EOS_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::InheritsFromBaseWithCustomAlignment>,
            __param_0: ::ctor::RvalueReference<'b, crate::InheritsFromBaseWithCustomAlignment>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN35InheritsFromBaseWithCustomAlignmentaSERKS_<'a, 'b>(
            __this: &'a mut crate::InheritsFromBaseWithCustomAlignment,
            __param_0: &'b crate::InheritsFromBaseWithCustomAlignment,
        ) -> &'a mut crate::InheritsFromBaseWithCustomAlignment;
        pub(crate) unsafe fn __rust_thunk___ZN35InheritsFromBaseWithCustomAlignmentaSEOS_<'a, 'b>(
            __this: &'a mut crate::InheritsFromBaseWithCustomAlignment,
            __param_0: ::ctor::RvalueReference<'b, crate::InheritsFromBaseWithCustomAlignment>,
        ) -> &'a mut crate::InheritsFromBaseWithCustomAlignment;
        pub(crate) unsafe fn __rust_thunk___ZN29HasCustomAlignmentWithGnuAttrC1Ev<'a>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::HasCustomAlignmentWithGnuAttr>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN29HasCustomAlignmentWithGnuAttrC1EOS_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::HasCustomAlignmentWithGnuAttr>,
            __param_0: ::ctor::RvalueReference<'b, crate::HasCustomAlignmentWithGnuAttr>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN29HasCustomAlignmentWithGnuAttraSERKS_<'a, 'b>(
            __this: &'a mut crate::HasCustomAlignmentWithGnuAttr,
            __param_0: &'b crate::HasCustomAlignmentWithGnuAttr,
        ) -> &'a mut crate::HasCustomAlignmentWithGnuAttr;
        pub(crate) unsafe fn __rust_thunk___ZN29HasCustomAlignmentWithGnuAttraSEOS_<'a, 'b>(
            __this: &'a mut crate::HasCustomAlignmentWithGnuAttr,
            __param_0: ::ctor::RvalueReference<'b, crate::HasCustomAlignmentWithGnuAttr>,
        ) -> &'a mut crate::HasCustomAlignmentWithGnuAttr;
        pub(crate) unsafe fn __rust_thunk___ZN28template_with_preferred_name12SomeTemplateIiEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3aclang_5fattrs_5fcc<
            'a,
        >(
            __this: &'a mut ::core::mem::MaybeUninit<
                crate::__CcTemplateInstN28template_with_preferred_name12SomeTemplateIiEE,
            >,
        );
        pub(crate) unsafe fn __rust_thunk___ZN28template_with_preferred_name12SomeTemplateIiEC1EOS1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3aclang_5fattrs_5fcc<
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
        pub(crate) unsafe fn __rust_thunk___ZN28template_with_preferred_name12SomeTemplateIiEaSERKS1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3aclang_5fattrs_5fcc<
            'a,
            'b,
        >(
            __this: &'a mut crate::__CcTemplateInstN28template_with_preferred_name12SomeTemplateIiEE,
            __param_0: &'b crate::__CcTemplateInstN28template_with_preferred_name12SomeTemplateIiEE,
        ) -> &'a mut crate::__CcTemplateInstN28template_with_preferred_name12SomeTemplateIiEE;
        pub(crate) unsafe fn __rust_thunk___ZN28template_with_preferred_name12SomeTemplateIiEaSEOS1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3aclang_5fattrs_5fcc<
            'a,
            'b,
        >(
            __this: &'a mut crate::__CcTemplateInstN28template_with_preferred_name12SomeTemplateIiEE,
            __param_0: ::ctor::RvalueReference<
                'b,
                crate::__CcTemplateInstN28template_with_preferred_name12SomeTemplateIiEE,
            >,
        ) -> &'a mut crate::__CcTemplateInstN28template_with_preferred_name12SomeTemplateIiEE;
        pub(crate) unsafe fn __rust_thunk___ZN28template_with_preferred_name12SomeTemplateIiE3fooEv__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3aclang_5fattrs_5fcc<
            'a,
        >(
            __this: &'a mut crate::__CcTemplateInstN28template_with_preferred_name12SomeTemplateIiEE,
        ) -> ::core::ffi::c_int;
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::HasCustomAlignment>() == 64);
    assert!(::core::mem::align_of::<crate::HasCustomAlignment>() == 64);
    static_assertions::assert_impl_all!(crate::HasCustomAlignment: Clone);
    static_assertions::assert_impl_all!(crate::HasCustomAlignment: Copy);
    static_assertions::assert_not_impl_any!(crate::HasCustomAlignment: Drop);

    assert!(::core::mem::size_of::<crate::HasFieldWithCustomAlignment>() == 64);
    assert!(::core::mem::align_of::<crate::HasFieldWithCustomAlignment>() == 64);
    static_assertions::assert_impl_all!(crate::HasFieldWithCustomAlignment: Clone);
    static_assertions::assert_impl_all!(crate::HasFieldWithCustomAlignment: Copy);
    static_assertions::assert_not_impl_any!(crate::HasFieldWithCustomAlignment: Drop);
    assert!(::core::mem::offset_of!(crate::HasFieldWithCustomAlignment, field) == 0);

    assert!(::core::mem::size_of::<crate::InheritsFromBaseWithCustomAlignment>() == 64);
    assert!(::core::mem::align_of::<crate::InheritsFromBaseWithCustomAlignment>() == 64);
    static_assertions::assert_impl_all!(crate::InheritsFromBaseWithCustomAlignment: Clone);
    static_assertions::assert_impl_all!(crate::InheritsFromBaseWithCustomAlignment: Copy);
    static_assertions::assert_not_impl_any!(crate::InheritsFromBaseWithCustomAlignment: Drop);

    assert!(::core::mem::size_of::<crate::HasCustomAlignmentWithGnuAttr>() == 64);
    assert!(::core::mem::align_of::<crate::HasCustomAlignmentWithGnuAttr>() == 64);
    static_assertions::assert_impl_all!(crate::HasCustomAlignmentWithGnuAttr: Clone);
    static_assertions::assert_impl_all!(crate::HasCustomAlignmentWithGnuAttr: Copy);
    static_assertions::assert_not_impl_any!(crate::HasCustomAlignmentWithGnuAttr: Drop);

    assert!(
        ::core::mem::size_of::<
            crate::__CcTemplateInstN28template_with_preferred_name12SomeTemplateIiEE,
        >() == 1
    );
    assert!(
        ::core::mem::align_of::<
            crate::__CcTemplateInstN28template_with_preferred_name12SomeTemplateIiEE,
        >() == 1
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstN28template_with_preferred_name12SomeTemplateIiEE: Clone);
    static_assertions::assert_impl_all!(crate::__CcTemplateInstN28template_with_preferred_name12SomeTemplateIiEE: Copy);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstN28template_with_preferred_name12SomeTemplateIiEE: Drop);
};
