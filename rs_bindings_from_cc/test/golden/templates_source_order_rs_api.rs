// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:templates_source_order_cc

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![no_std]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

// Error while generating bindings for item 'MyTemplate':
// Class templates are not supported yet

#[derive(Clone, Copy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=TopLevel
pub struct TopLevel {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for TopLevel {}
impl !Sync for TopLevel {}
forward_declare::unsafe_define!(forward_declare::symbol!("TopLevel"), crate::TopLevel);

impl Default for TopLevel {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN8TopLevelC1Ev(&raw mut tmp as *mut ::core::ffi::c_void);
            tmp.assume_init()
        }
    }
}

impl From<::ctor::RvalueReference<'_, Self>> for TopLevel {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'_, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN8TopLevelC1EOS_(
                &raw mut tmp as *mut ::core::ffi::c_void,
                __param_0,
            );
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<::ctor::RvalueReference<'_, Self>> for TopLevel {
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'_, Self>) -> Self::CtorType {
        <Self as From<::ctor::RvalueReference<'_, Self>>>::from(args)
    }
}

impl ::ctor::UnpinAssign<&Self> for TopLevel {
    #[inline(always)]
    fn unpin_assign(&mut self, __param_0: &Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN8TopLevelaSERKS_(self, __param_0);
        }
    }
}

impl ::ctor::UnpinAssign<::ctor::RvalueReference<'_, Self>> for TopLevel {
    #[inline(always)]
    fn unpin_assign(&mut self, __param_0: ::ctor::RvalueReference<'_, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN8TopLevelaSEOS_(self, __param_0);
        }
    }
}

pub type Alias1 = crate::__CcTemplateInst10MyTemplateIiE;

pub type Alias2 = crate::__CcTemplateInst10MyTemplateIfE;

pub type Alias3 = crate::__CcTemplateInst10MyTemplateI8TopLevelE;

pub type Alias4 = crate::__CcTemplateInst10MyTemplateIdE;

pub type Alias5 = crate::__CcTemplateInst10MyTemplateIbE;

pub type Alias6 = crate::__CcTemplateInst10MyTemplateIS_I8TopLevelEE;

pub mod test_namespace_bindings {
    #[derive(Clone, Copy)]
    #[repr(C)]
    ///CRUBIT_ANNOTATE: cpp_type=test_namespace_bindings :: Inner
    pub struct Inner {
        __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
    }
    impl !Send for Inner {}
    impl !Sync for Inner {}
    forward_declare::unsafe_define!(
        forward_declare::symbol!("test_namespace_bindings :: Inner"),
        crate::test_namespace_bindings::Inner
    );

    impl Default for Inner {
        #[inline(always)]
        fn default() -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk___ZN23test_namespace_bindings5InnerC1Ev(
                    &raw mut tmp as *mut ::core::ffi::c_void,
                );
                tmp.assume_init()
            }
        }
    }

    impl From<::ctor::RvalueReference<'_, Self>> for Inner {
        #[inline(always)]
        fn from(__param_0: ::ctor::RvalueReference<'_, Self>) -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk___ZN23test_namespace_bindings5InnerC1EOS0_(
                    &raw mut tmp as *mut ::core::ffi::c_void,
                    __param_0,
                );
                tmp.assume_init()
            }
        }
    }
    impl ::ctor::CtorNew<::ctor::RvalueReference<'_, Self>> for Inner {
        type CtorType = Self;
        type Error = ::ctor::Infallible;
        #[inline(always)]
        fn ctor_new(args: ::ctor::RvalueReference<'_, Self>) -> Self::CtorType {
            <Self as From<::ctor::RvalueReference<'_, Self>>>::from(args)
        }
    }

    impl ::ctor::UnpinAssign<&Self> for Inner {
        #[inline(always)]
        fn unpin_assign(&mut self, __param_0: &Self) {
            unsafe {
                crate::detail::__rust_thunk___ZN23test_namespace_bindings5InneraSERKS0_(
                    self, __param_0,
                );
            }
        }
    }

    impl ::ctor::UnpinAssign<::ctor::RvalueReference<'_, Self>> for Inner {
        #[inline(always)]
        fn unpin_assign(&mut self, __param_0: ::ctor::RvalueReference<'_, Self>) {
            unsafe {
                crate::detail::__rust_thunk___ZN23test_namespace_bindings5InneraSEOS0_(
                    self, __param_0,
                );
            }
        }
    }

    pub type Alias7 = crate::__CcTemplateInst10MyTemplateIcE;

    pub type Alias8 = crate::__CcTemplateInst10MyTemplateIN23test_namespace_bindings5InnerEE;

    pub type Alias9 = crate::__CcTemplateInst10MyTemplateIS_IN23test_namespace_bindings5InnerEEE;
}

// namespace test_namespace_bindings

#[derive(Clone, Copy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=MyTemplate < TopLevel >
pub struct __CcTemplateInst10MyTemplateI8TopLevelE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) t: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for __CcTemplateInst10MyTemplateI8TopLevelE {}
impl !Sync for __CcTemplateInst10MyTemplateI8TopLevelE {}
forward_declare::unsafe_define!(
    forward_declare::symbol!("MyTemplate < TopLevel >"),
    crate::__CcTemplateInst10MyTemplateI8TopLevelE
);

impl Default for __CcTemplateInst10MyTemplateI8TopLevelE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN10MyTemplateI8TopLevelEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(&raw mut tmp as*mut::core::ffi::c_void);
            tmp.assume_init()
        }
    }
}

impl From<::ctor::RvalueReference<'_, Self>> for __CcTemplateInst10MyTemplateI8TopLevelE {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'_, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN10MyTemplateI8TopLevelEC1EOS1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(&raw mut tmp as*mut::core::ffi::c_void,__param_0);
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<::ctor::RvalueReference<'_, Self>>
    for __CcTemplateInst10MyTemplateI8TopLevelE
{
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'_, Self>) -> Self::CtorType {
        <Self as From<::ctor::RvalueReference<'_, Self>>>::from(args)
    }
}

impl ::ctor::UnpinAssign<&Self> for __CcTemplateInst10MyTemplateI8TopLevelE {
    #[inline(always)]
    fn unpin_assign(&mut self, __param_0: &Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN10MyTemplateI8TopLevelEaSERKS1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(self,__param_0);
        }
    }
}

impl ::ctor::UnpinAssign<::ctor::RvalueReference<'_, Self>>
    for __CcTemplateInst10MyTemplateI8TopLevelE
{
    #[inline(always)]
    fn unpin_assign(&mut self, __param_0: ::ctor::RvalueReference<'_, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN10MyTemplateI8TopLevelEaSEOS1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(self,__param_0);
        }
    }
}

impl __CcTemplateInst10MyTemplateI8TopLevelE {
    #[inline(always)]
    pub unsafe fn processT(__this: *mut Self, mut t: crate::TopLevel) {
        crate::detail::__rust_thunk___ZN10MyTemplateI8TopLevelE8processTES0___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(__this,&mut t)
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=MyTemplate < test_namespace_bindings :: Inner >
pub struct __CcTemplateInst10MyTemplateIN23test_namespace_bindings5InnerEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) t: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for __CcTemplateInst10MyTemplateIN23test_namespace_bindings5InnerEE {}
impl !Sync for __CcTemplateInst10MyTemplateIN23test_namespace_bindings5InnerEE {}
forward_declare::unsafe_define!(
    forward_declare::symbol!("MyTemplate < test_namespace_bindings :: Inner >"),
    crate::__CcTemplateInst10MyTemplateIN23test_namespace_bindings5InnerEE
);

impl Default for __CcTemplateInst10MyTemplateIN23test_namespace_bindings5InnerEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN10MyTemplateIN23test_namespace_bindings5InnerEEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(&raw mut tmp as*mut::core::ffi::c_void);
            tmp.assume_init()
        }
    }
}

impl From<::ctor::RvalueReference<'_, Self>>
    for __CcTemplateInst10MyTemplateIN23test_namespace_bindings5InnerEE
{
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'_, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN10MyTemplateIN23test_namespace_bindings5InnerEEC1EOS2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(&raw mut tmp as*mut::core::ffi::c_void,__param_0);
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<::ctor::RvalueReference<'_, Self>>
    for __CcTemplateInst10MyTemplateIN23test_namespace_bindings5InnerEE
{
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'_, Self>) -> Self::CtorType {
        <Self as From<::ctor::RvalueReference<'_, Self>>>::from(args)
    }
}

impl ::ctor::UnpinAssign<&Self>
    for __CcTemplateInst10MyTemplateIN23test_namespace_bindings5InnerEE
{
    #[inline(always)]
    fn unpin_assign(&mut self, __param_0: &Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN10MyTemplateIN23test_namespace_bindings5InnerEEaSERKS2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(self,__param_0);
        }
    }
}

impl ::ctor::UnpinAssign<::ctor::RvalueReference<'_, Self>>
    for __CcTemplateInst10MyTemplateIN23test_namespace_bindings5InnerEE
{
    #[inline(always)]
    fn unpin_assign(&mut self, __param_0: ::ctor::RvalueReference<'_, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN10MyTemplateIN23test_namespace_bindings5InnerEEaSEOS2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(self,__param_0);
        }
    }
}

impl __CcTemplateInst10MyTemplateIN23test_namespace_bindings5InnerEE {
    #[inline(always)]
    pub unsafe fn processT(__this: *mut Self, mut t: crate::test_namespace_bindings::Inner) {
        crate::detail::__rust_thunk___ZN10MyTemplateIN23test_namespace_bindings5InnerEE8processTES1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(__this,&mut t)
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=MyTemplate < MyTemplate < TopLevel >>
pub struct __CcTemplateInst10MyTemplateIS_I8TopLevelEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) t: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for __CcTemplateInst10MyTemplateIS_I8TopLevelEE {}
impl !Sync for __CcTemplateInst10MyTemplateIS_I8TopLevelEE {}
forward_declare::unsafe_define!(
    forward_declare::symbol!("MyTemplate < MyTemplate < TopLevel >>"),
    crate::__CcTemplateInst10MyTemplateIS_I8TopLevelEE
);

impl Default for __CcTemplateInst10MyTemplateIS_I8TopLevelEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN10MyTemplateIS_I8TopLevelEEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(&raw mut tmp as*mut::core::ffi::c_void);
            tmp.assume_init()
        }
    }
}

impl From<::ctor::RvalueReference<'_, Self>> for __CcTemplateInst10MyTemplateIS_I8TopLevelEE {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'_, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN10MyTemplateIS_I8TopLevelEEC1EOS2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(&raw mut tmp as*mut::core::ffi::c_void,__param_0);
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<::ctor::RvalueReference<'_, Self>>
    for __CcTemplateInst10MyTemplateIS_I8TopLevelEE
{
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'_, Self>) -> Self::CtorType {
        <Self as From<::ctor::RvalueReference<'_, Self>>>::from(args)
    }
}

impl ::ctor::UnpinAssign<&Self> for __CcTemplateInst10MyTemplateIS_I8TopLevelEE {
    #[inline(always)]
    fn unpin_assign(&mut self, __param_0: &Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN10MyTemplateIS_I8TopLevelEEaSERKS2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(self,__param_0);
        }
    }
}

impl ::ctor::UnpinAssign<::ctor::RvalueReference<'_, Self>>
    for __CcTemplateInst10MyTemplateIS_I8TopLevelEE
{
    #[inline(always)]
    fn unpin_assign(&mut self, __param_0: ::ctor::RvalueReference<'_, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN10MyTemplateIS_I8TopLevelEEaSEOS2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(self,__param_0);
        }
    }
}

impl __CcTemplateInst10MyTemplateIS_I8TopLevelEE {
    #[inline(always)]
    pub unsafe fn processT(
        __this: *mut Self,
        mut t: crate::__CcTemplateInst10MyTemplateI8TopLevelE,
    ) {
        crate::detail::__rust_thunk___ZN10MyTemplateIS_I8TopLevelEE8processTES1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(__this,&mut t)
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=MyTemplate < MyTemplate < test_namespace_bindings :: Inner >>
pub struct __CcTemplateInst10MyTemplateIS_IN23test_namespace_bindings5InnerEEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) t: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for __CcTemplateInst10MyTemplateIS_IN23test_namespace_bindings5InnerEEE {}
impl !Sync for __CcTemplateInst10MyTemplateIS_IN23test_namespace_bindings5InnerEEE {}
forward_declare::unsafe_define!(
    forward_declare::symbol!("MyTemplate < MyTemplate < test_namespace_bindings :: Inner >>"),
    crate::__CcTemplateInst10MyTemplateIS_IN23test_namespace_bindings5InnerEEE
);

impl Default for __CcTemplateInst10MyTemplateIS_IN23test_namespace_bindings5InnerEEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN10MyTemplateIS_IN23test_namespace_bindings5InnerEEEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(&raw mut tmp as*mut::core::ffi::c_void);
            tmp.assume_init()
        }
    }
}

impl From<::ctor::RvalueReference<'_, Self>>
    for __CcTemplateInst10MyTemplateIS_IN23test_namespace_bindings5InnerEEE
{
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'_, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN10MyTemplateIS_IN23test_namespace_bindings5InnerEEEC1EOS3___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(&raw mut tmp as*mut::core::ffi::c_void,__param_0);
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<::ctor::RvalueReference<'_, Self>>
    for __CcTemplateInst10MyTemplateIS_IN23test_namespace_bindings5InnerEEE
{
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'_, Self>) -> Self::CtorType {
        <Self as From<::ctor::RvalueReference<'_, Self>>>::from(args)
    }
}

impl ::ctor::UnpinAssign<&Self>
    for __CcTemplateInst10MyTemplateIS_IN23test_namespace_bindings5InnerEEE
{
    #[inline(always)]
    fn unpin_assign(&mut self, __param_0: &Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN10MyTemplateIS_IN23test_namespace_bindings5InnerEEEaSERKS3___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(self,__param_0);
        }
    }
}

impl ::ctor::UnpinAssign<::ctor::RvalueReference<'_, Self>>
    for __CcTemplateInst10MyTemplateIS_IN23test_namespace_bindings5InnerEEE
{
    #[inline(always)]
    fn unpin_assign(&mut self, __param_0: ::ctor::RvalueReference<'_, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN10MyTemplateIS_IN23test_namespace_bindings5InnerEEEaSEOS3___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(self,__param_0);
        }
    }
}

impl __CcTemplateInst10MyTemplateIS_IN23test_namespace_bindings5InnerEEE {
    #[inline(always)]
    pub unsafe fn processT(
        __this: *mut Self,
        mut t: crate::__CcTemplateInst10MyTemplateIN23test_namespace_bindings5InnerEE,
    ) {
        crate::detail::__rust_thunk___ZN10MyTemplateIS_IN23test_namespace_bindings5InnerEEE8processTES2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(__this,&mut t)
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=MyTemplate < bool >
pub struct __CcTemplateInst10MyTemplateIbE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) t: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for __CcTemplateInst10MyTemplateIbE {}
impl !Sync for __CcTemplateInst10MyTemplateIbE {}
forward_declare::unsafe_define!(
    forward_declare::symbol!("MyTemplate < bool >"),
    crate::__CcTemplateInst10MyTemplateIbE
);

impl Default for __CcTemplateInst10MyTemplateIbE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN10MyTemplateIbEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(&raw mut tmp as*mut::core::ffi::c_void);
            tmp.assume_init()
        }
    }
}

impl From<::ctor::RvalueReference<'_, Self>> for __CcTemplateInst10MyTemplateIbE {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'_, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN10MyTemplateIbEC1EOS0___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(&raw mut tmp as*mut::core::ffi::c_void,__param_0);
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<::ctor::RvalueReference<'_, Self>> for __CcTemplateInst10MyTemplateIbE {
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'_, Self>) -> Self::CtorType {
        <Self as From<::ctor::RvalueReference<'_, Self>>>::from(args)
    }
}

impl ::ctor::UnpinAssign<&Self> for __CcTemplateInst10MyTemplateIbE {
    #[inline(always)]
    fn unpin_assign(&mut self, __param_0: &Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN10MyTemplateIbEaSERKS0___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(self,__param_0);
        }
    }
}

impl ::ctor::UnpinAssign<::ctor::RvalueReference<'_, Self>> for __CcTemplateInst10MyTemplateIbE {
    #[inline(always)]
    fn unpin_assign(&mut self, __param_0: ::ctor::RvalueReference<'_, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN10MyTemplateIbEaSEOS0___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(self,__param_0);
        }
    }
}

impl __CcTemplateInst10MyTemplateIbE {
    #[inline(always)]
    pub unsafe fn processT(__this: *mut Self, t: bool) {
        crate::detail::__rust_thunk___ZN10MyTemplateIbE8processTEb__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(__this,t)
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=MyTemplate < char >
pub struct __CcTemplateInst10MyTemplateIcE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) t: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for __CcTemplateInst10MyTemplateIcE {}
impl !Sync for __CcTemplateInst10MyTemplateIcE {}
forward_declare::unsafe_define!(
    forward_declare::symbol!("MyTemplate < char >"),
    crate::__CcTemplateInst10MyTemplateIcE
);

impl Default for __CcTemplateInst10MyTemplateIcE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN10MyTemplateIcEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(&raw mut tmp as*mut::core::ffi::c_void);
            tmp.assume_init()
        }
    }
}

impl From<::ctor::RvalueReference<'_, Self>> for __CcTemplateInst10MyTemplateIcE {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'_, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN10MyTemplateIcEC1EOS0___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(&raw mut tmp as*mut::core::ffi::c_void,__param_0);
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<::ctor::RvalueReference<'_, Self>> for __CcTemplateInst10MyTemplateIcE {
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'_, Self>) -> Self::CtorType {
        <Self as From<::ctor::RvalueReference<'_, Self>>>::from(args)
    }
}

impl ::ctor::UnpinAssign<&Self> for __CcTemplateInst10MyTemplateIcE {
    #[inline(always)]
    fn unpin_assign(&mut self, __param_0: &Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN10MyTemplateIcEaSERKS0___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(self,__param_0);
        }
    }
}

impl ::ctor::UnpinAssign<::ctor::RvalueReference<'_, Self>> for __CcTemplateInst10MyTemplateIcE {
    #[inline(always)]
    fn unpin_assign(&mut self, __param_0: ::ctor::RvalueReference<'_, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN10MyTemplateIcEaSEOS0___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(self,__param_0);
        }
    }
}

impl __CcTemplateInst10MyTemplateIcE {
    #[inline(always)]
    pub unsafe fn processT(__this: *mut Self, t: ::core::ffi::c_char) {
        crate::detail::__rust_thunk___ZN10MyTemplateIcE8processTEc__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(__this,t)
    }
}

#[derive(Clone, Copy)]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=MyTemplate < double >
pub struct __CcTemplateInst10MyTemplateIdE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) t: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for __CcTemplateInst10MyTemplateIdE {}
impl !Sync for __CcTemplateInst10MyTemplateIdE {}
forward_declare::unsafe_define!(
    forward_declare::symbol!("MyTemplate < double >"),
    crate::__CcTemplateInst10MyTemplateIdE
);

impl Default for __CcTemplateInst10MyTemplateIdE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN10MyTemplateIdEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(&raw mut tmp as*mut::core::ffi::c_void);
            tmp.assume_init()
        }
    }
}

impl From<::ctor::RvalueReference<'_, Self>> for __CcTemplateInst10MyTemplateIdE {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'_, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN10MyTemplateIdEC1EOS0___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(&raw mut tmp as*mut::core::ffi::c_void,__param_0);
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<::ctor::RvalueReference<'_, Self>> for __CcTemplateInst10MyTemplateIdE {
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'_, Self>) -> Self::CtorType {
        <Self as From<::ctor::RvalueReference<'_, Self>>>::from(args)
    }
}

impl ::ctor::UnpinAssign<&Self> for __CcTemplateInst10MyTemplateIdE {
    #[inline(always)]
    fn unpin_assign(&mut self, __param_0: &Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN10MyTemplateIdEaSERKS0___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(self,__param_0);
        }
    }
}

impl ::ctor::UnpinAssign<::ctor::RvalueReference<'_, Self>> for __CcTemplateInst10MyTemplateIdE {
    #[inline(always)]
    fn unpin_assign(&mut self, __param_0: ::ctor::RvalueReference<'_, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN10MyTemplateIdEaSEOS0___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(self,__param_0);
        }
    }
}

impl __CcTemplateInst10MyTemplateIdE {
    #[inline(always)]
    pub unsafe fn processT(__this: *mut Self, t: f64) {
        crate::detail::__rust_thunk___ZN10MyTemplateIdE8processTEd__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(__this,t)
    }
}

#[derive(Clone, Copy)]
#[repr(C, align(4))]
///CRUBIT_ANNOTATE: cpp_type=MyTemplate < float >
pub struct __CcTemplateInst10MyTemplateIfE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) t: [::core::mem::MaybeUninit<u8>; 4],
}
impl !Send for __CcTemplateInst10MyTemplateIfE {}
impl !Sync for __CcTemplateInst10MyTemplateIfE {}
forward_declare::unsafe_define!(
    forward_declare::symbol!("MyTemplate < float >"),
    crate::__CcTemplateInst10MyTemplateIfE
);

impl Default for __CcTemplateInst10MyTemplateIfE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN10MyTemplateIfEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(&raw mut tmp as*mut::core::ffi::c_void);
            tmp.assume_init()
        }
    }
}

impl From<::ctor::RvalueReference<'_, Self>> for __CcTemplateInst10MyTemplateIfE {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'_, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN10MyTemplateIfEC1EOS0___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(&raw mut tmp as*mut::core::ffi::c_void,__param_0);
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<::ctor::RvalueReference<'_, Self>> for __CcTemplateInst10MyTemplateIfE {
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'_, Self>) -> Self::CtorType {
        <Self as From<::ctor::RvalueReference<'_, Self>>>::from(args)
    }
}

impl ::ctor::UnpinAssign<&Self> for __CcTemplateInst10MyTemplateIfE {
    #[inline(always)]
    fn unpin_assign(&mut self, __param_0: &Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN10MyTemplateIfEaSERKS0___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(self,__param_0);
        }
    }
}

impl ::ctor::UnpinAssign<::ctor::RvalueReference<'_, Self>> for __CcTemplateInst10MyTemplateIfE {
    #[inline(always)]
    fn unpin_assign(&mut self, __param_0: ::ctor::RvalueReference<'_, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN10MyTemplateIfEaSEOS0___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(self,__param_0);
        }
    }
}

impl __CcTemplateInst10MyTemplateIfE {
    #[inline(always)]
    pub unsafe fn processT(__this: *mut Self, t: f32) {
        crate::detail::__rust_thunk___ZN10MyTemplateIfE8processTEf__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(__this,t)
    }
}

#[derive(Clone, Copy)]
#[repr(C, align(4))]
///CRUBIT_ANNOTATE: cpp_type=MyTemplate < int >
pub struct __CcTemplateInst10MyTemplateIiE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) t: [::core::mem::MaybeUninit<u8>; 4],
}
impl !Send for __CcTemplateInst10MyTemplateIiE {}
impl !Sync for __CcTemplateInst10MyTemplateIiE {}
forward_declare::unsafe_define!(
    forward_declare::symbol!("MyTemplate < int >"),
    crate::__CcTemplateInst10MyTemplateIiE
);

impl Default for __CcTemplateInst10MyTemplateIiE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN10MyTemplateIiEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(&raw mut tmp as*mut::core::ffi::c_void);
            tmp.assume_init()
        }
    }
}

impl From<::ctor::RvalueReference<'_, Self>> for __CcTemplateInst10MyTemplateIiE {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'_, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN10MyTemplateIiEC1EOS0___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(&raw mut tmp as*mut::core::ffi::c_void,__param_0);
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<::ctor::RvalueReference<'_, Self>> for __CcTemplateInst10MyTemplateIiE {
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'_, Self>) -> Self::CtorType {
        <Self as From<::ctor::RvalueReference<'_, Self>>>::from(args)
    }
}

impl ::ctor::UnpinAssign<&Self> for __CcTemplateInst10MyTemplateIiE {
    #[inline(always)]
    fn unpin_assign(&mut self, __param_0: &Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN10MyTemplateIiEaSERKS0___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(self,__param_0);
        }
    }
}

impl ::ctor::UnpinAssign<::ctor::RvalueReference<'_, Self>> for __CcTemplateInst10MyTemplateIiE {
    #[inline(always)]
    fn unpin_assign(&mut self, __param_0: ::ctor::RvalueReference<'_, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN10MyTemplateIiEaSEOS0___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(self,__param_0);
        }
    }
}

impl __CcTemplateInst10MyTemplateIiE {
    #[inline(always)]
    pub unsafe fn processT(__this: *mut Self, t: ::core::ffi::c_int) {
        crate::detail::__rust_thunk___ZN10MyTemplateIiE8processTEi__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(__this,t)
    }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN8TopLevelC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN8TopLevelC1EOS_(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'_, crate::TopLevel>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN8TopLevelaSERKS_<'__return_lifetime>(
            __this: &mut crate::TopLevel,
            __param_0: &crate::TopLevel,
        ) -> &'__return_lifetime mut crate::TopLevel;
        pub(crate) unsafe fn __rust_thunk___ZN8TopLevelaSEOS_<'__return_lifetime>(
            __this: &mut crate::TopLevel,
            __param_0: ::ctor::RvalueReference<'_, crate::TopLevel>,
        ) -> &'__return_lifetime mut crate::TopLevel;
        pub(crate) unsafe fn __rust_thunk___ZN23test_namespace_bindings5InnerC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN23test_namespace_bindings5InnerC1EOS0_(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'_, crate::test_namespace_bindings::Inner>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN23test_namespace_bindings5InneraSERKS0_<
            '__return_lifetime,
        >(
            __this: &mut crate::test_namespace_bindings::Inner,
            __param_0: &crate::test_namespace_bindings::Inner,
        ) -> &'__return_lifetime mut crate::test_namespace_bindings::Inner;
        pub(crate) unsafe fn __rust_thunk___ZN23test_namespace_bindings5InneraSEOS0_<
            '__return_lifetime,
        >(
            __this: &mut crate::test_namespace_bindings::Inner,
            __param_0: ::ctor::RvalueReference<'_, crate::test_namespace_bindings::Inner>,
        ) -> &'__return_lifetime mut crate::test_namespace_bindings::Inner;
        pub(crate) unsafe fn __rust_thunk___ZN10MyTemplateI8TopLevelEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN10MyTemplateI8TopLevelEC1EOS1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'_, crate::__CcTemplateInst10MyTemplateI8TopLevelE>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN10MyTemplateI8TopLevelEaSERKS1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc<
            '__return_lifetime,
        >(
            __this: &mut crate::__CcTemplateInst10MyTemplateI8TopLevelE,
            __param_0: &crate::__CcTemplateInst10MyTemplateI8TopLevelE,
        ) -> &'__return_lifetime mut crate::__CcTemplateInst10MyTemplateI8TopLevelE;
        pub(crate) unsafe fn __rust_thunk___ZN10MyTemplateI8TopLevelEaSEOS1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc<
            '__return_lifetime,
        >(
            __this: &mut crate::__CcTemplateInst10MyTemplateI8TopLevelE,
            __param_0: ::ctor::RvalueReference<'_, crate::__CcTemplateInst10MyTemplateI8TopLevelE>,
        ) -> &'__return_lifetime mut crate::__CcTemplateInst10MyTemplateI8TopLevelE;
        pub(crate) unsafe fn __rust_thunk___ZN10MyTemplateI8TopLevelE8processTES0___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
            __this: *mut crate::__CcTemplateInst10MyTemplateI8TopLevelE,
            t: &mut crate::TopLevel,
        );
        pub(crate) unsafe fn __rust_thunk___ZN10MyTemplateIN23test_namespace_bindings5InnerEEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN10MyTemplateIN23test_namespace_bindings5InnerEEC1EOS2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<
                '_,
                crate::__CcTemplateInst10MyTemplateIN23test_namespace_bindings5InnerEE,
            >,
        );
        pub(crate)unsafe fn __rust_thunk___ZN10MyTemplateIN23test_namespace_bindings5InnerEEaSERKS2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc<'__return_lifetime>(__this: &mut crate::__CcTemplateInst10MyTemplateIN23test_namespace_bindings5InnerEE,__param_0: &crate::__CcTemplateInst10MyTemplateIN23test_namespace_bindings5InnerEE)->&'__return_lifetime mut crate::__CcTemplateInst10MyTemplateIN23test_namespace_bindings5InnerEE;
        pub(crate)unsafe fn __rust_thunk___ZN10MyTemplateIN23test_namespace_bindings5InnerEEaSEOS2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc<'__return_lifetime>(__this: &mut crate::__CcTemplateInst10MyTemplateIN23test_namespace_bindings5InnerEE,__param_0: ::ctor::RvalueReference<'_,crate::__CcTemplateInst10MyTemplateIN23test_namespace_bindings5InnerEE>)->&'__return_lifetime mut crate::__CcTemplateInst10MyTemplateIN23test_namespace_bindings5InnerEE;
        pub(crate) unsafe fn __rust_thunk___ZN10MyTemplateIN23test_namespace_bindings5InnerEE8processTES1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
            __this: *mut crate::__CcTemplateInst10MyTemplateIN23test_namespace_bindings5InnerEE,
            t: &mut crate::test_namespace_bindings::Inner,
        );
        pub(crate) unsafe fn __rust_thunk___ZN10MyTemplateIS_I8TopLevelEEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN10MyTemplateIS_I8TopLevelEEC1EOS2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<
                '_,
                crate::__CcTemplateInst10MyTemplateIS_I8TopLevelEE,
            >,
        );
        pub(crate) unsafe fn __rust_thunk___ZN10MyTemplateIS_I8TopLevelEEaSERKS2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc<
            '__return_lifetime,
        >(
            __this: &mut crate::__CcTemplateInst10MyTemplateIS_I8TopLevelEE,
            __param_0: &crate::__CcTemplateInst10MyTemplateIS_I8TopLevelEE,
        ) -> &'__return_lifetime mut crate::__CcTemplateInst10MyTemplateIS_I8TopLevelEE;
        pub(crate) unsafe fn __rust_thunk___ZN10MyTemplateIS_I8TopLevelEEaSEOS2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc<
            '__return_lifetime,
        >(
            __this: &mut crate::__CcTemplateInst10MyTemplateIS_I8TopLevelEE,
            __param_0: ::ctor::RvalueReference<
                '_,
                crate::__CcTemplateInst10MyTemplateIS_I8TopLevelEE,
            >,
        ) -> &'__return_lifetime mut crate::__CcTemplateInst10MyTemplateIS_I8TopLevelEE;
        pub(crate) unsafe fn __rust_thunk___ZN10MyTemplateIS_I8TopLevelEE8processTES1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
            __this: *mut crate::__CcTemplateInst10MyTemplateIS_I8TopLevelEE,
            t: &mut crate::__CcTemplateInst10MyTemplateI8TopLevelE,
        );
        pub(crate) unsafe fn __rust_thunk___ZN10MyTemplateIS_IN23test_namespace_bindings5InnerEEEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN10MyTemplateIS_IN23test_namespace_bindings5InnerEEEC1EOS3___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<
                '_,
                crate::__CcTemplateInst10MyTemplateIS_IN23test_namespace_bindings5InnerEEE,
            >,
        );
        pub(crate)unsafe fn __rust_thunk___ZN10MyTemplateIS_IN23test_namespace_bindings5InnerEEEaSERKS3___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc<'__return_lifetime>(__this: &mut crate::__CcTemplateInst10MyTemplateIS_IN23test_namespace_bindings5InnerEEE,__param_0: &crate::__CcTemplateInst10MyTemplateIS_IN23test_namespace_bindings5InnerEEE)->&'__return_lifetime mut crate::__CcTemplateInst10MyTemplateIS_IN23test_namespace_bindings5InnerEEE;
        pub(crate)unsafe fn __rust_thunk___ZN10MyTemplateIS_IN23test_namespace_bindings5InnerEEEaSEOS3___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc<'__return_lifetime>(__this: &mut crate::__CcTemplateInst10MyTemplateIS_IN23test_namespace_bindings5InnerEEE,__param_0: ::ctor::RvalueReference<'_,crate::__CcTemplateInst10MyTemplateIS_IN23test_namespace_bindings5InnerEEE>)->&'__return_lifetime mut crate::__CcTemplateInst10MyTemplateIS_IN23test_namespace_bindings5InnerEEE;
        pub(crate) unsafe fn __rust_thunk___ZN10MyTemplateIS_IN23test_namespace_bindings5InnerEEE8processTES2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
            __this: *mut crate::__CcTemplateInst10MyTemplateIS_IN23test_namespace_bindings5InnerEEE,
            t: &mut crate::__CcTemplateInst10MyTemplateIN23test_namespace_bindings5InnerEE,
        );
        pub(crate) unsafe fn __rust_thunk___ZN10MyTemplateIbEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN10MyTemplateIbEC1EOS0___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'_, crate::__CcTemplateInst10MyTemplateIbE>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN10MyTemplateIbEaSERKS0___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc<
            '__return_lifetime,
        >(
            __this: &mut crate::__CcTemplateInst10MyTemplateIbE,
            __param_0: &crate::__CcTemplateInst10MyTemplateIbE,
        ) -> &'__return_lifetime mut crate::__CcTemplateInst10MyTemplateIbE;
        pub(crate) unsafe fn __rust_thunk___ZN10MyTemplateIbEaSEOS0___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc<
            '__return_lifetime,
        >(
            __this: &mut crate::__CcTemplateInst10MyTemplateIbE,
            __param_0: ::ctor::RvalueReference<'_, crate::__CcTemplateInst10MyTemplateIbE>,
        ) -> &'__return_lifetime mut crate::__CcTemplateInst10MyTemplateIbE;
        pub(crate) unsafe fn __rust_thunk___ZN10MyTemplateIbE8processTEb__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
            __this: *mut crate::__CcTemplateInst10MyTemplateIbE,
            t: bool,
        );
        pub(crate) unsafe fn __rust_thunk___ZN10MyTemplateIcEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN10MyTemplateIcEC1EOS0___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'_, crate::__CcTemplateInst10MyTemplateIcE>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN10MyTemplateIcEaSERKS0___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc<
            '__return_lifetime,
        >(
            __this: &mut crate::__CcTemplateInst10MyTemplateIcE,
            __param_0: &crate::__CcTemplateInst10MyTemplateIcE,
        ) -> &'__return_lifetime mut crate::__CcTemplateInst10MyTemplateIcE;
        pub(crate) unsafe fn __rust_thunk___ZN10MyTemplateIcEaSEOS0___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc<
            '__return_lifetime,
        >(
            __this: &mut crate::__CcTemplateInst10MyTemplateIcE,
            __param_0: ::ctor::RvalueReference<'_, crate::__CcTemplateInst10MyTemplateIcE>,
        ) -> &'__return_lifetime mut crate::__CcTemplateInst10MyTemplateIcE;
        pub(crate) unsafe fn __rust_thunk___ZN10MyTemplateIcE8processTEc__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
            __this: *mut crate::__CcTemplateInst10MyTemplateIcE,
            t: ::core::ffi::c_char,
        );
        pub(crate) unsafe fn __rust_thunk___ZN10MyTemplateIdEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN10MyTemplateIdEC1EOS0___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'_, crate::__CcTemplateInst10MyTemplateIdE>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN10MyTemplateIdEaSERKS0___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc<
            '__return_lifetime,
        >(
            __this: &mut crate::__CcTemplateInst10MyTemplateIdE,
            __param_0: &crate::__CcTemplateInst10MyTemplateIdE,
        ) -> &'__return_lifetime mut crate::__CcTemplateInst10MyTemplateIdE;
        pub(crate) unsafe fn __rust_thunk___ZN10MyTemplateIdEaSEOS0___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc<
            '__return_lifetime,
        >(
            __this: &mut crate::__CcTemplateInst10MyTemplateIdE,
            __param_0: ::ctor::RvalueReference<'_, crate::__CcTemplateInst10MyTemplateIdE>,
        ) -> &'__return_lifetime mut crate::__CcTemplateInst10MyTemplateIdE;
        pub(crate) unsafe fn __rust_thunk___ZN10MyTemplateIdE8processTEd__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
            __this: *mut crate::__CcTemplateInst10MyTemplateIdE,
            t: f64,
        );
        pub(crate) unsafe fn __rust_thunk___ZN10MyTemplateIfEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN10MyTemplateIfEC1EOS0___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'_, crate::__CcTemplateInst10MyTemplateIfE>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN10MyTemplateIfEaSERKS0___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc<
            '__return_lifetime,
        >(
            __this: &mut crate::__CcTemplateInst10MyTemplateIfE,
            __param_0: &crate::__CcTemplateInst10MyTemplateIfE,
        ) -> &'__return_lifetime mut crate::__CcTemplateInst10MyTemplateIfE;
        pub(crate) unsafe fn __rust_thunk___ZN10MyTemplateIfEaSEOS0___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc<
            '__return_lifetime,
        >(
            __this: &mut crate::__CcTemplateInst10MyTemplateIfE,
            __param_0: ::ctor::RvalueReference<'_, crate::__CcTemplateInst10MyTemplateIfE>,
        ) -> &'__return_lifetime mut crate::__CcTemplateInst10MyTemplateIfE;
        pub(crate) unsafe fn __rust_thunk___ZN10MyTemplateIfE8processTEf__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
            __this: *mut crate::__CcTemplateInst10MyTemplateIfE,
            t: f32,
        );
        pub(crate) unsafe fn __rust_thunk___ZN10MyTemplateIiEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN10MyTemplateIiEC1EOS0___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'_, crate::__CcTemplateInst10MyTemplateIiE>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN10MyTemplateIiEaSERKS0___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc<
            '__return_lifetime,
        >(
            __this: &mut crate::__CcTemplateInst10MyTemplateIiE,
            __param_0: &crate::__CcTemplateInst10MyTemplateIiE,
        ) -> &'__return_lifetime mut crate::__CcTemplateInst10MyTemplateIiE;
        pub(crate) unsafe fn __rust_thunk___ZN10MyTemplateIiEaSEOS0___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc<
            '__return_lifetime,
        >(
            __this: &mut crate::__CcTemplateInst10MyTemplateIiE,
            __param_0: ::ctor::RvalueReference<'_, crate::__CcTemplateInst10MyTemplateIiE>,
        ) -> &'__return_lifetime mut crate::__CcTemplateInst10MyTemplateIiE;
        pub(crate) unsafe fn __rust_thunk___ZN10MyTemplateIiE8processTEi__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
            __this: *mut crate::__CcTemplateInst10MyTemplateIiE,
            t: ::core::ffi::c_int,
        );
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::TopLevel>() == 1);
    assert!(::core::mem::align_of::<crate::TopLevel>() == 1);
    static_assertions::assert_impl_all!(crate::TopLevel: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::TopLevel: Drop);

    assert!(::core::mem::size_of::<crate::test_namespace_bindings::Inner>() == 1);
    assert!(::core::mem::align_of::<crate::test_namespace_bindings::Inner>() == 1);
    static_assertions::assert_impl_all!(crate::test_namespace_bindings::Inner: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::test_namespace_bindings::Inner: Drop);

    assert!(::core::mem::size_of::<crate::__CcTemplateInst10MyTemplateI8TopLevelE>() == 1);
    assert!(::core::mem::align_of::<crate::__CcTemplateInst10MyTemplateI8TopLevelE>() == 1);
    static_assertions::assert_impl_all!(crate::__CcTemplateInst10MyTemplateI8TopLevelE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInst10MyTemplateI8TopLevelE: Drop);
    assert!(::core::mem::offset_of!(crate::__CcTemplateInst10MyTemplateI8TopLevelE, t) == 0);
    assert!(
        ::core::mem::size_of::<
            crate::__CcTemplateInst10MyTemplateIN23test_namespace_bindings5InnerEE,
        >() == 1
    );
    assert!(
        ::core::mem::align_of::<
            crate::__CcTemplateInst10MyTemplateIN23test_namespace_bindings5InnerEE,
        >() == 1
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInst10MyTemplateIN23test_namespace_bindings5InnerEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInst10MyTemplateIN23test_namespace_bindings5InnerEE: Drop);
    assert!(
        ::core::mem::offset_of!(
            crate::__CcTemplateInst10MyTemplateIN23test_namespace_bindings5InnerEE,
            t
        ) == 0
    );
    assert!(::core::mem::size_of::<crate::__CcTemplateInst10MyTemplateIS_I8TopLevelEE>() == 1);
    assert!(::core::mem::align_of::<crate::__CcTemplateInst10MyTemplateIS_I8TopLevelEE>() == 1);
    static_assertions::assert_impl_all!(crate::__CcTemplateInst10MyTemplateIS_I8TopLevelEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInst10MyTemplateIS_I8TopLevelEE: Drop);
    assert!(::core::mem::offset_of!(crate::__CcTemplateInst10MyTemplateIS_I8TopLevelEE, t) == 0);
    assert!(
        ::core::mem::size_of::<
            crate::__CcTemplateInst10MyTemplateIS_IN23test_namespace_bindings5InnerEEE,
        >() == 1
    );
    assert!(
        ::core::mem::align_of::<
            crate::__CcTemplateInst10MyTemplateIS_IN23test_namespace_bindings5InnerEEE,
        >() == 1
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInst10MyTemplateIS_IN23test_namespace_bindings5InnerEEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInst10MyTemplateIS_IN23test_namespace_bindings5InnerEEE: Drop);
    assert!(
        ::core::mem::offset_of!(
            crate::__CcTemplateInst10MyTemplateIS_IN23test_namespace_bindings5InnerEEE,
            t
        ) == 0
    );
    assert!(::core::mem::size_of::<crate::__CcTemplateInst10MyTemplateIbE>() == 1);
    assert!(::core::mem::align_of::<crate::__CcTemplateInst10MyTemplateIbE>() == 1);
    static_assertions::assert_impl_all!(crate::__CcTemplateInst10MyTemplateIbE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInst10MyTemplateIbE: Drop);
    assert!(::core::mem::offset_of!(crate::__CcTemplateInst10MyTemplateIbE, t) == 0);
    assert!(::core::mem::size_of::<crate::__CcTemplateInst10MyTemplateIcE>() == 1);
    assert!(::core::mem::align_of::<crate::__CcTemplateInst10MyTemplateIcE>() == 1);
    static_assertions::assert_impl_all!(crate::__CcTemplateInst10MyTemplateIcE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInst10MyTemplateIcE: Drop);
    assert!(::core::mem::offset_of!(crate::__CcTemplateInst10MyTemplateIcE, t) == 0);
    assert!(::core::mem::size_of::<crate::__CcTemplateInst10MyTemplateIdE>() == 8);
    assert!(::core::mem::align_of::<crate::__CcTemplateInst10MyTemplateIdE>() == 8);
    static_assertions::assert_impl_all!(crate::__CcTemplateInst10MyTemplateIdE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInst10MyTemplateIdE: Drop);
    assert!(::core::mem::offset_of!(crate::__CcTemplateInst10MyTemplateIdE, t) == 0);
    assert!(::core::mem::size_of::<crate::__CcTemplateInst10MyTemplateIfE>() == 4);
    assert!(::core::mem::align_of::<crate::__CcTemplateInst10MyTemplateIfE>() == 4);
    static_assertions::assert_impl_all!(crate::__CcTemplateInst10MyTemplateIfE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInst10MyTemplateIfE: Drop);
    assert!(::core::mem::offset_of!(crate::__CcTemplateInst10MyTemplateIfE, t) == 0);
    assert!(::core::mem::size_of::<crate::__CcTemplateInst10MyTemplateIiE>() == 4);
    assert!(::core::mem::align_of::<crate::__CcTemplateInst10MyTemplateIiE>() == 4);
    static_assertions::assert_impl_all!(crate::__CcTemplateInst10MyTemplateIiE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInst10MyTemplateIiE: Drop);
    assert!(::core::mem::offset_of!(crate::__CcTemplateInst10MyTemplateIiE, t) == 0);
};
