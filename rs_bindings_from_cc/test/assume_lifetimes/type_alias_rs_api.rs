// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/assume_lifetimes:type_alias

#![rustfmt::skip]
#![feature(cfi_encoding, custom_inner_attributes, impl_trait_in_assoc_type, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![allow(deprecated)]
#![allow(unknown_lints, suspicious_runtime_symbol_definitions)]
#![deny(warnings)]
#[::ctor::recursively_pinned(PinnedDrop)]
#[cfi_encoding = "13TypeAliasCtor"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=TypeAliasCtor
pub struct TypeAliasCtor {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 1],
}
impl !Send for TypeAliasCtor {}
impl !Sync for TypeAliasCtor {}
unsafe impl ::cxx::ExternType for TypeAliasCtor {
    type Id = ::cxx::type_id!("TypeAliasCtor");
    type Kind = ::cxx::kind::Opaque;
}

impl<'__param_0> ::ctor::CtorNew<&'__param_0 Self> for TypeAliasCtor {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__param_0>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: &'__param_0 Self) -> Self::CtorType {
        let mut __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk___ZN13TypeAliasCtorC1ERKS_(
                    __crubit_dest as *mut ::core::ffi::c_void,
                    __param_0,
                );
            })
        }
    }
}
impl<'__param_0> ::ctor::CtorNew<(&'__param_0 Self,)> for TypeAliasCtor {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__param_0>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (&'__param_0 Self,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'__param_0 Self>>::ctor_new(arg)
    }
}

impl<'__param_0> ::ctor::Assign<&'__param_0 Self> for TypeAliasCtor {
    #[inline(always)]
    fn assign<'__this>(self: ::core::pin::Pin<&'__this mut Self>, __param_0: &'__param_0 Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN13TypeAliasCtoraSERKS_(self, __param_0);
        }
    }
}

impl<'a> ::ctor::CtorNew<::cc_std::std::string_view<'a>> for TypeAliasCtor {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'a>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::cc_std::std::string_view<'a>) -> Self::CtorType {
        let mut a = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk___ZN13TypeAliasCtorC1ENSt3__u17basic_string_viewIcNS0_11char_traitsIcEEEE(__crubit_dest as*mut::core::ffi::c_void,&mut a);
            })
        }
    }
}
impl<'a> ::ctor::CtorNew<(::cc_std::std::string_view<'a>,)> for TypeAliasCtor {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'a>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (::cc_std::std::string_view<'a>,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::cc_std::std::string_view<'a>>>::ctor_new(arg)
    }
}

impl ::ctor::PinnedDrop for TypeAliasCtor {
    #[inline(always)]
    unsafe fn pinned_drop<'__this>(self: ::core::pin::Pin<&'__this mut Self>) {
        unsafe { crate::detail::__rust_thunk___ZN13TypeAliasCtorD1Ev(self) }
    }
}

// error: class `std::basic_string<char32_t, std::char_traits<char32_t>, std::pmr::polymorphic_allocator<char32_t>>` could not be bound
//   incomplete type

// error: class `std::basic_string<char32_t, std::char_traits<char32_t>, std::allocator<char32_t>>` could not be bound
//   incomplete type

// error: class `std::basic_string<char16_t, std::char_traits<char16_t>, std::pmr::polymorphic_allocator<char16_t>>` could not be bound
//   incomplete type

// error: class `std::basic_string<char16_t, std::char_traits<char16_t>, std::allocator<char16_t>>` could not be bound
//   incomplete type

// error: class `std::basic_string<char, std::char_traits<char>, std::pmr::polymorphic_allocator<char>>` could not be bound
//   incomplete type

// error: class `std::basic_string<char, std::char_traits<char>, std::allocator<char>>` could not be bound
//   incomplete type

// error: class `std::basic_ostream<char, std::char_traits<char>>` could not be bound
//   incomplete type

// error: class `std::basic_ostream<wchar_t, std::char_traits<wchar_t>>` could not be bound
//   incomplete type

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=std :: basic_string_view < char32_t , std :: char_traits < char32_t >>
pub struct __CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) __data_: [::core::mem::MaybeUninit<u8>; 8],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) __size_: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for __CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE {}
impl !Sync for __CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE {}
impl __CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE {
    #[must_use]
    #[inline(always)]
    pub fn begin<'__this>(&'__this self) -> *const u32 {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_di_ns_11char_traits_i_di_eeee::begin(
                self,
            )
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn end<'__this>(&'__this self) -> *const u32 {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_di_ns_11char_traits_i_di_eeee::end(
                self,
            )
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn cbegin<'__this>(&'__this self) -> *const u32 {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_di_ns_11char_traits_i_di_eeee::cbegin(self)
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn cend<'__this>(&'__this self) -> *const u32 {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_di_ns_11char_traits_i_di_eeee::cend(
                self,
            )
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn rbegin<'__this>(
        &'__this self,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDiEE {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_di_ns_11char_traits_i_di_eeee::rbegin(self)
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn rend<'__this>(&'__this self) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDiEE {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_di_ns_11char_traits_i_di_eeee::rend(
                self,
            )
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn crbegin<'__this>(
        &'__this self,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDiEE {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_di_ns_11char_traits_i_di_eeee::crbegin(self)
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn crend<'__this>(
        &'__this self,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDiEE {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_di_ns_11char_traits_i_di_eeee::crend(
                self,
            )
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn size<'__this>(&'__this self) -> usize {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_di_ns_11char_traits_i_di_eeee::size(
                self,
            )
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn length<'__this>(&'__this self) -> usize {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_di_ns_11char_traits_i_di_eeee::length(self)
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn max_size<'__this>(&'__this self) -> usize {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_di_ns_11char_traits_i_di_eeee::max_size(self)
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn empty<'__this>(&'__this self) -> bool {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_di_ns_11char_traits_i_di_eeee::empty(
                self,
            )
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn at<'__this>(&'__this self, __pos: usize) -> ::cref::CRef<'__this, u32> {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_di_ns_11char_traits_i_di_eeee::at(
                self, __pos,
            )
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn front<'__this>(&'__this self) -> ::cref::CRef<'__this, u32> {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_di_ns_11char_traits_i_di_eeee::front(
                self,
            )
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn back<'__this>(&'__this self) -> ::cref::CRef<'__this, u32> {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_di_ns_11char_traits_i_di_eeee::back(
                self,
            )
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn data<'__this>(&'__this self) -> *const u32 {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_di_ns_11char_traits_i_di_eeee::data(
                self,
            )
        }
    }
    #[inline(always)]
    pub fn remove_prefix<'__this>(&'__this mut self, __n: usize) {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_di_ns_11char_traits_i_di_eeee::remove_prefix(self,__n)
        }
    }
    #[inline(always)]
    pub fn remove_suffix<'__this>(&'__this mut self, __n: usize) {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_di_ns_11char_traits_i_di_eeee::remove_suffix(self,__n)
        }
    }
    #[inline(always)]
    pub fn swap<'__other, '__this>(&'__this mut self, __other: &'__other mut Self) {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_di_ns_11char_traits_i_di_eeee::swap(
                self, __other,
            )
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__s`: raw pointer
    #[inline(always)]
    pub unsafe fn copy<'__this>(&'__this self, __s: *mut u32, __n: usize, __pos: usize) -> usize {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_di_ns_11char_traits_i_di_eeee::copy(
                self, __s, __n, __pos,
            )
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn substr<'__this>(
        &'__this self,
        __pos: usize,
        __n: usize,
    ) -> crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_di_ns_11char_traits_i_di_eeee::substr(self,__pos,__n)
        }
    }
}

impl Default for __CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__e3237267__ZNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEC1Ev(&raw mut tmp as*mut _);
            tmp.assume_init()
        }
    }
}

impl ::ctor::UnsafeFrom<*const u32>
    for __CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE
{
    #[inline(always)]
    unsafe fn unsafe_from(args: *const u32) -> Self {
        let mut __s = args;
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__35143324__ZNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEC1EPKDi(&raw mut tmp as*mut _,__s);
            tmp.assume_init()
        }
    }
}
impl ::ctor::UnsafeCtorNew<*const u32>
    for __CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE
{
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    unsafe fn ctor_new(args: *const u32) -> Self::CtorType {
        unsafe { <Self as ::ctor::UnsafeFrom<*const u32>>::unsafe_from(args) }
    }
}

impl ::operator::CcIndex<usize>
    for __CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE
{
    type Output<'ctnr> = &'ctnr u32;
    #[inline(always)]
    fn cc_index<'ctnr>(&'ctnr self, __pos: usize) -> Self::Output<'ctnr> {
        unsafe {
            crate::detail::__rust_thunk__6e80e7d7__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEixEm(self,__pos)
        }
    }
}
impl ::core::ops::Index<usize>
    for __CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE
{
    type Output = u32;
    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        ::operator::CcIndex::cc_index(self, index)
    }
}

pub mod cc_template_inst_n_st3_u17basic_string_view_i_di_ns_11char_traits_i_di_eeee {
    #[must_use]
    #[inline(always)]
    pub(crate) fn begin<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
    ) -> *const u32 {
        unsafe {
            crate::detail::__rust_thunk__d87a5768__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE5beginEv(__this)
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn end<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
    ) -> *const u32 {
        unsafe {
            crate::detail::__rust_thunk__9cd7c456__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE3endEv(__this)
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn cbegin<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
    ) -> *const u32 {
        unsafe {
            crate::detail::__rust_thunk__ba84366a__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE6cbeginEv(__this)
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn cend<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
    ) -> *const u32 {
        unsafe {
            crate::detail::__rust_thunk__98fd39f0__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE4cendEv(__this)
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn rbegin<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDiEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDiEE,
            >::uninit();
            crate::detail::__rust_thunk__e474ba02__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE6rbeginEv(&raw mut __crubit_return as*mut::core::ffi::c_void,__this);
            __crubit_return.assume_init()
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn rend<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDiEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDiEE,
            >::uninit();
            crate::detail::__rust_thunk__45754841__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE4rendEv(&raw mut __crubit_return as*mut::core::ffi::c_void,__this);
            __crubit_return.assume_init()
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn crbegin<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDiEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDiEE,
            >::uninit();
            crate::detail::__rust_thunk__572dcabe__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE7crbeginEv(&raw mut __crubit_return as*mut::core::ffi::c_void,__this);
            __crubit_return.assume_init()
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn crend<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDiEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDiEE,
            >::uninit();
            crate::detail::__rust_thunk__8b827fea__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE5crendEv(&raw mut __crubit_return as*mut::core::ffi::c_void,__this);
            __crubit_return.assume_init()
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn size<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
    ) -> usize {
        unsafe {
            crate::detail::__rust_thunk__4fcc8cae__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE4sizeEv(__this)
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn length<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
    ) -> usize {
        unsafe {
            crate::detail::__rust_thunk__a1b282c6__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE6lengthEv(__this)
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn max_size<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
    ) -> usize {
        unsafe {
            crate::detail::__rust_thunk__f9699d7a__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE8max_sizeEv(__this)
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn empty<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
    ) -> bool {
        unsafe {
            crate::detail::__rust_thunk__1e821d1e__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE5emptyEv(__this)
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn at<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
        __pos: usize,
    ) -> ::cref::CRef<'__this, u32> {
        unsafe {
            crate::detail::__rust_thunk__ab69815c__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE2atEm(__this,__pos)
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn front<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
    ) -> ::cref::CRef<'__this, u32> {
        unsafe {
            crate::detail::__rust_thunk__9dc1d8ab__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE5frontEv(__this)
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn back<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
    ) -> ::cref::CRef<'__this, u32> {
        unsafe {
            crate::detail::__rust_thunk__20abdabc__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE4backEv(__this)
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn data<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
    ) -> *const u32 {
        unsafe {
            crate::detail::__rust_thunk__659db6d3__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE4dataEv(__this)
        }
    }
    #[inline(always)]
    pub(crate) fn remove_prefix<'__this>(
        __this: &'__this mut crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
        __n: usize,
    ) {
        unsafe {
            crate::detail::__rust_thunk__06c1d1b6__ZNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE13remove_prefixEm(__this,__n)
        }
    }
    #[inline(always)]
    pub(crate) fn remove_suffix<'__this>(
        __this: &'__this mut crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
        __n: usize,
    ) {
        unsafe {
            crate::detail::__rust_thunk__a7391000__ZNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE13remove_suffixEm(__this,__n)
        }
    }
    #[inline(always)]
    pub(crate) fn swap<'__other, '__this>(
        __this: &'__this mut crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
        __other: &'__other mut crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
    ) {
        unsafe {
            crate::detail::__rust_thunk__f1d195d0__ZNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE4swapERS3_(__this,__other)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__s`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn copy<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
        __s: *mut u32,
        __n: usize,
        __pos: usize,
    ) -> usize {
        unsafe {
            crate::detail::__rust_thunk__63a9d1e0__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE4copyEPDimm(__this,__s,__n,__pos)
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn substr<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
        __pos: usize,
        __n: usize,
    ) -> crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
            >::uninit();
            crate::detail::__rust_thunk__86ba0b92__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE6substrEmm(&raw mut __crubit_return as*mut::core::ffi::c_void,__this,__pos,__n);
            __crubit_return.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=std :: basic_string_view < char16_t , std :: char_traits < char16_t >>
pub struct __CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) __data_: [::core::mem::MaybeUninit<u8>; 8],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) __size_: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for __CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE {}
impl !Sync for __CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE {}
impl __CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE {
    #[must_use]
    #[inline(always)]
    pub fn begin<'__this>(&'__this self) -> *const u16 {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_ds_ns_11char_traits_i_ds_eeee::begin(
                self,
            )
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn end<'__this>(&'__this self) -> *const u16 {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_ds_ns_11char_traits_i_ds_eeee::end(
                self,
            )
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn cbegin<'__this>(&'__this self) -> *const u16 {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_ds_ns_11char_traits_i_ds_eeee::cbegin(self)
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn cend<'__this>(&'__this self) -> *const u16 {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_ds_ns_11char_traits_i_ds_eeee::cend(
                self,
            )
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn rbegin<'__this>(
        &'__this self,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDsEE {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_ds_ns_11char_traits_i_ds_eeee::rbegin(self)
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn rend<'__this>(&'__this self) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDsEE {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_ds_ns_11char_traits_i_ds_eeee::rend(
                self,
            )
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn crbegin<'__this>(
        &'__this self,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDsEE {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_ds_ns_11char_traits_i_ds_eeee::crbegin(self)
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn crend<'__this>(
        &'__this self,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDsEE {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_ds_ns_11char_traits_i_ds_eeee::crend(
                self,
            )
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn size<'__this>(&'__this self) -> usize {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_ds_ns_11char_traits_i_ds_eeee::size(
                self,
            )
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn length<'__this>(&'__this self) -> usize {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_ds_ns_11char_traits_i_ds_eeee::length(self)
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn max_size<'__this>(&'__this self) -> usize {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_ds_ns_11char_traits_i_ds_eeee::max_size(self)
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn empty<'__this>(&'__this self) -> bool {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_ds_ns_11char_traits_i_ds_eeee::empty(
                self,
            )
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn at<'__this>(&'__this self, __pos: usize) -> ::cref::CRef<'__this, u16> {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_ds_ns_11char_traits_i_ds_eeee::at(
                self, __pos,
            )
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn front<'__this>(&'__this self) -> ::cref::CRef<'__this, u16> {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_ds_ns_11char_traits_i_ds_eeee::front(
                self,
            )
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn back<'__this>(&'__this self) -> ::cref::CRef<'__this, u16> {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_ds_ns_11char_traits_i_ds_eeee::back(
                self,
            )
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn data<'__this>(&'__this self) -> *const u16 {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_ds_ns_11char_traits_i_ds_eeee::data(
                self,
            )
        }
    }
    #[inline(always)]
    pub fn remove_prefix<'__this>(&'__this mut self, __n: usize) {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_ds_ns_11char_traits_i_ds_eeee::remove_prefix(self,__n)
        }
    }
    #[inline(always)]
    pub fn remove_suffix<'__this>(&'__this mut self, __n: usize) {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_ds_ns_11char_traits_i_ds_eeee::remove_suffix(self,__n)
        }
    }
    #[inline(always)]
    pub fn swap<'__other, '__this>(&'__this mut self, __other: &'__other mut Self) {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_ds_ns_11char_traits_i_ds_eeee::swap(
                self, __other,
            )
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__s`: raw pointer
    #[inline(always)]
    pub unsafe fn copy<'__this>(&'__this self, __s: *mut u16, __n: usize, __pos: usize) -> usize {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_ds_ns_11char_traits_i_ds_eeee::copy(
                self, __s, __n, __pos,
            )
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn substr<'__this>(
        &'__this self,
        __pos: usize,
        __n: usize,
    ) -> crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_ds_ns_11char_traits_i_ds_eeee::substr(self,__pos,__n)
        }
    }
}

impl Default for __CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__e3237267__ZNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEC1Ev(&raw mut tmp as*mut _);
            tmp.assume_init()
        }
    }
}

impl ::ctor::UnsafeFrom<*const u16>
    for __CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE
{
    #[inline(always)]
    unsafe fn unsafe_from(args: *const u16) -> Self {
        let mut __s = args;
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__35143324__ZNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEC1EPKDs(&raw mut tmp as*mut _,__s);
            tmp.assume_init()
        }
    }
}
impl ::ctor::UnsafeCtorNew<*const u16>
    for __CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE
{
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    unsafe fn ctor_new(args: *const u16) -> Self::CtorType {
        unsafe { <Self as ::ctor::UnsafeFrom<*const u16>>::unsafe_from(args) }
    }
}

impl ::operator::CcIndex<usize>
    for __CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE
{
    type Output<'ctnr> = &'ctnr u16;
    #[inline(always)]
    fn cc_index<'ctnr>(&'ctnr self, __pos: usize) -> Self::Output<'ctnr> {
        unsafe {
            crate::detail::__rust_thunk__6e80e7d7__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEixEm(self,__pos)
        }
    }
}
impl ::core::ops::Index<usize>
    for __CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE
{
    type Output = u16;
    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        ::operator::CcIndex::cc_index(self, index)
    }
}

pub mod cc_template_inst_n_st3_u17basic_string_view_i_ds_ns_11char_traits_i_ds_eeee {
    #[must_use]
    #[inline(always)]
    pub(crate) fn begin<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
    ) -> *const u16 {
        unsafe {
            crate::detail::__rust_thunk__d87a5768__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE5beginEv(__this)
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn end<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
    ) -> *const u16 {
        unsafe {
            crate::detail::__rust_thunk__9cd7c456__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE3endEv(__this)
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn cbegin<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
    ) -> *const u16 {
        unsafe {
            crate::detail::__rust_thunk__ba84366a__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE6cbeginEv(__this)
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn cend<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
    ) -> *const u16 {
        unsafe {
            crate::detail::__rust_thunk__98fd39f0__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE4cendEv(__this)
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn rbegin<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDsEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDsEE,
            >::uninit();
            crate::detail::__rust_thunk__e474ba02__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE6rbeginEv(&raw mut __crubit_return as*mut::core::ffi::c_void,__this);
            __crubit_return.assume_init()
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn rend<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDsEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDsEE,
            >::uninit();
            crate::detail::__rust_thunk__45754841__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE4rendEv(&raw mut __crubit_return as*mut::core::ffi::c_void,__this);
            __crubit_return.assume_init()
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn crbegin<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDsEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDsEE,
            >::uninit();
            crate::detail::__rust_thunk__572dcabe__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE7crbeginEv(&raw mut __crubit_return as*mut::core::ffi::c_void,__this);
            __crubit_return.assume_init()
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn crend<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDsEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDsEE,
            >::uninit();
            crate::detail::__rust_thunk__8b827fea__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE5crendEv(&raw mut __crubit_return as*mut::core::ffi::c_void,__this);
            __crubit_return.assume_init()
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn size<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
    ) -> usize {
        unsafe {
            crate::detail::__rust_thunk__4fcc8cae__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE4sizeEv(__this)
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn length<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
    ) -> usize {
        unsafe {
            crate::detail::__rust_thunk__a1b282c6__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE6lengthEv(__this)
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn max_size<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
    ) -> usize {
        unsafe {
            crate::detail::__rust_thunk__f9699d7a__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE8max_sizeEv(__this)
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn empty<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
    ) -> bool {
        unsafe {
            crate::detail::__rust_thunk__1e821d1e__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE5emptyEv(__this)
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn at<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
        __pos: usize,
    ) -> ::cref::CRef<'__this, u16> {
        unsafe {
            crate::detail::__rust_thunk__ab69815c__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE2atEm(__this,__pos)
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn front<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
    ) -> ::cref::CRef<'__this, u16> {
        unsafe {
            crate::detail::__rust_thunk__9dc1d8ab__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE5frontEv(__this)
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn back<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
    ) -> ::cref::CRef<'__this, u16> {
        unsafe {
            crate::detail::__rust_thunk__20abdabc__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE4backEv(__this)
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn data<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
    ) -> *const u16 {
        unsafe {
            crate::detail::__rust_thunk__659db6d3__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE4dataEv(__this)
        }
    }
    #[inline(always)]
    pub(crate) fn remove_prefix<'__this>(
        __this: &'__this mut crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
        __n: usize,
    ) {
        unsafe {
            crate::detail::__rust_thunk__06c1d1b6__ZNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE13remove_prefixEm(__this,__n)
        }
    }
    #[inline(always)]
    pub(crate) fn remove_suffix<'__this>(
        __this: &'__this mut crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
        __n: usize,
    ) {
        unsafe {
            crate::detail::__rust_thunk__a7391000__ZNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE13remove_suffixEm(__this,__n)
        }
    }
    #[inline(always)]
    pub(crate) fn swap<'__other, '__this>(
        __this: &'__this mut crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
        __other: &'__other mut crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
    ) {
        unsafe {
            crate::detail::__rust_thunk__f1d195d0__ZNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE4swapERS3_(__this,__other)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__s`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn copy<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
        __s: *mut u16,
        __n: usize,
        __pos: usize,
    ) -> usize {
        unsafe {
            crate::detail::__rust_thunk__63a9d1e0__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE4copyEPDsmm(__this,__s,__n,__pos)
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn substr<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
        __pos: usize,
        __n: usize,
    ) -> crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
            >::uninit();
            crate::detail::__rust_thunk__86ba0b92__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE6substrEmm(&raw mut __crubit_return as*mut::core::ffi::c_void,__this,__pos,__n);
            __crubit_return.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=std :: basic_string_view < char8_t , std :: char_traits < char8_t >>
pub struct __CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) __data_: [::core::mem::MaybeUninit<u8>; 8],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) __size_: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for __CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE {}
impl !Sync for __CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE {}
impl __CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE {
    #[must_use]
    #[inline(always)]
    pub fn rbegin<'__this>(
        &'__this self,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDuEE {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_du_ns_11char_traits_i_du_eeee::rbegin(self)
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn rend<'__this>(&'__this self) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDuEE {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_du_ns_11char_traits_i_du_eeee::rend(
                self,
            )
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn crbegin<'__this>(
        &'__this self,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDuEE {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_du_ns_11char_traits_i_du_eeee::crbegin(self)
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn crend<'__this>(
        &'__this self,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDuEE {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_du_ns_11char_traits_i_du_eeee::crend(
                self,
            )
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn size<'__this>(&'__this self) -> usize {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_du_ns_11char_traits_i_du_eeee::size(
                self,
            )
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn length<'__this>(&'__this self) -> usize {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_du_ns_11char_traits_i_du_eeee::length(self)
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn max_size<'__this>(&'__this self) -> usize {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_du_ns_11char_traits_i_du_eeee::max_size(self)
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn empty<'__this>(&'__this self) -> bool {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_du_ns_11char_traits_i_du_eeee::empty(
                self,
            )
        }
    }
    #[inline(always)]
    pub fn remove_prefix<'__this>(&'__this mut self, __n: usize) {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_du_ns_11char_traits_i_du_eeee::remove_prefix(self,__n)
        }
    }
    #[inline(always)]
    pub fn remove_suffix<'__this>(&'__this mut self, __n: usize) {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_du_ns_11char_traits_i_du_eeee::remove_suffix(self,__n)
        }
    }
    #[inline(always)]
    pub fn swap<'__other, '__this>(&'__this mut self, __other: &'__other mut Self) {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_du_ns_11char_traits_i_du_eeee::swap(
                self, __other,
            )
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn substr<'__this>(
        &'__this self,
        __pos: usize,
        __n: usize,
    ) -> crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_du_ns_11char_traits_i_du_eeee::substr(self,__pos,__n)
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn find<'__this>(&'__this self, mut __s: Self, __pos: usize) -> usize {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_du_ns_11char_traits_i_du_eeee::find(
                self, __s, __pos,
            )
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn rfind<'__this>(&'__this self, mut __s: Self, __pos: usize) -> usize {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_du_ns_11char_traits_i_du_eeee::rfind(
                self, __s, __pos,
            )
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn find_first_of<'__this>(&'__this self, mut __s: Self, __pos: usize) -> usize {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_du_ns_11char_traits_i_du_eeee::find_first_of(self,__s,__pos)
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn find_last_of<'__this>(&'__this self, mut __s: Self, __pos: usize) -> usize {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_du_ns_11char_traits_i_du_eeee::find_last_of(self,__s,__pos)
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn find_first_not_of<'__this>(&'__this self, mut __s: Self, __pos: usize) -> usize {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_du_ns_11char_traits_i_du_eeee::find_first_not_of(self,__s,__pos)
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn find_last_not_of<'__this>(&'__this self, mut __s: Self, __pos: usize) -> usize {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_du_ns_11char_traits_i_du_eeee::find_last_not_of(self,__s,__pos)
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn starts_with<'__this>(&'__this self, mut __s: Self) -> bool {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_du_ns_11char_traits_i_du_eeee::starts_with(self,__s)
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn ends_with<'__this>(&'__this self, mut __s: Self) -> bool {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_du_ns_11char_traits_i_du_eeee::ends_with(self,__s)
        }
    }
}

impl Default for __CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__e3237267__ZNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEC1Ev(&raw mut tmp as*mut _);
            tmp.assume_init()
        }
    }
}

pub mod cc_template_inst_n_st3_u17basic_string_view_i_du_ns_11char_traits_i_du_eeee {
    #[must_use]
    #[inline(always)]
    pub(crate) fn rbegin<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDuEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDuEE,
            >::uninit();
            crate::detail::__rust_thunk__e474ba02__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE6rbeginEv(&raw mut __crubit_return as*mut::core::ffi::c_void,__this);
            __crubit_return.assume_init()
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn rend<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDuEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDuEE,
            >::uninit();
            crate::detail::__rust_thunk__45754841__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE4rendEv(&raw mut __crubit_return as*mut::core::ffi::c_void,__this);
            __crubit_return.assume_init()
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn crbegin<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDuEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDuEE,
            >::uninit();
            crate::detail::__rust_thunk__572dcabe__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE7crbeginEv(&raw mut __crubit_return as*mut::core::ffi::c_void,__this);
            __crubit_return.assume_init()
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn crend<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDuEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDuEE,
            >::uninit();
            crate::detail::__rust_thunk__8b827fea__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE5crendEv(&raw mut __crubit_return as*mut::core::ffi::c_void,__this);
            __crubit_return.assume_init()
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn size<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
    ) -> usize {
        unsafe {
            crate::detail::__rust_thunk__4fcc8cae__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE4sizeEv(__this)
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn length<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
    ) -> usize {
        unsafe {
            crate::detail::__rust_thunk__a1b282c6__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE6lengthEv(__this)
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn max_size<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
    ) -> usize {
        unsafe {
            crate::detail::__rust_thunk__f9699d7a__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE8max_sizeEv(__this)
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn empty<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
    ) -> bool {
        unsafe {
            crate::detail::__rust_thunk__1e821d1e__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE5emptyEv(__this)
        }
    }
    #[inline(always)]
    pub(crate) fn remove_prefix<'__this>(
        __this: &'__this mut crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
        __n: usize,
    ) {
        unsafe {
            crate::detail::__rust_thunk__06c1d1b6__ZNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE13remove_prefixEm(__this,__n)
        }
    }
    #[inline(always)]
    pub(crate) fn remove_suffix<'__this>(
        __this: &'__this mut crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
        __n: usize,
    ) {
        unsafe {
            crate::detail::__rust_thunk__a7391000__ZNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE13remove_suffixEm(__this,__n)
        }
    }
    #[inline(always)]
    pub(crate) fn swap<'__other, '__this>(
        __this: &'__this mut crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
        __other: &'__other mut crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
    ) {
        unsafe {
            crate::detail::__rust_thunk__f1d195d0__ZNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE4swapERS3_(__this,__other)
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn substr<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
        __pos: usize,
        __n: usize,
    ) -> crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
            >::uninit();
            crate::detail::__rust_thunk__86ba0b92__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE6substrEmm(&raw mut __crubit_return as*mut::core::ffi::c_void,__this,__pos,__n);
            __crubit_return.assume_init()
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn find<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
        mut __s: crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
        __pos: usize,
    ) -> usize {
        unsafe {
            crate::detail::__rust_thunk__654b550f__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE4findES3_m(__this,&mut __s,__pos)
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn rfind<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
        mut __s: crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
        __pos: usize,
    ) -> usize {
        unsafe {
            crate::detail::__rust_thunk__dbd88331__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE5rfindES3_m(__this,&mut __s,__pos)
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn find_first_of<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
        mut __s: crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
        __pos: usize,
    ) -> usize {
        unsafe {
            crate::detail::__rust_thunk__3ba36d97__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE13find_first_ofES3_m(__this,&mut __s,__pos)
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn find_last_of<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
        mut __s: crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
        __pos: usize,
    ) -> usize {
        unsafe {
            crate::detail::__rust_thunk__b8a8a87c__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE12find_last_ofES3_m(__this,&mut __s,__pos)
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn find_first_not_of<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
        mut __s: crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
        __pos: usize,
    ) -> usize {
        unsafe {
            crate::detail::__rust_thunk__a6fd6d60__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE17find_first_not_ofES3_m(__this,&mut __s,__pos)
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn find_last_not_of<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
        mut __s: crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
        __pos: usize,
    ) -> usize {
        unsafe {
            crate::detail::__rust_thunk__27e3987c__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE16find_last_not_ofES3_m(__this,&mut __s,__pos)
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn starts_with<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
        mut __s: crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
    ) -> bool {
        unsafe {
            crate::detail::__rust_thunk__1f379720__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE11starts_withES3_(__this,&mut __s)
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn ends_with<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
        mut __s: crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
    ) -> bool {
        unsafe {
            crate::detail::__rust_thunk__6beb84a4__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE9ends_withES3_(__this,&mut __s)
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u16reverse_iteratorIPKDiEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=std :: reverse_iterator < const char32_t *>
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u16reverse_iteratorIPKDiEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) current: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for __CcTemplateInstNSt3__u16reverse_iteratorIPKDiEE {}
impl !Sync for __CcTemplateInstNSt3__u16reverse_iteratorIPKDiEE {}
impl __CcTemplateInstNSt3__u16reverse_iteratorIPKDiEE {
    #[must_use]
    #[inline(always)]
    pub fn base<'__this>(&'__this self) -> *const u32 {
        unsafe { self::cc_template_inst_n_st3_u16reverse_iterator_ipk_di_ee::base(self) }
    }
}

impl Default for __CcTemplateInstNSt3__u16reverse_iteratorIPKDiEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__63d556e1__ZNSt3__u16reverse_iteratorIPKDiEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

impl ::ctor::UnsafeFrom<*const u32> for __CcTemplateInstNSt3__u16reverse_iteratorIPKDiEE {
    #[inline(always)]
    unsafe fn unsafe_from(args: *const u32) -> Self {
        let mut __x = args;
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__9cfa002e__ZNSt3__u16reverse_iteratorIPKDiEC1ES2_(
                &raw mut tmp as *mut _,
                __x,
            );
            tmp.assume_init()
        }
    }
}
impl ::ctor::UnsafeCtorNew<*const u32> for __CcTemplateInstNSt3__u16reverse_iteratorIPKDiEE {
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    unsafe fn ctor_new(args: *const u32) -> Self::CtorType {
        unsafe { <Self as ::ctor::UnsafeFrom<*const u32>>::unsafe_from(args) }
    }
}

pub mod cc_template_inst_n_st3_u16reverse_iterator_ipk_di_ee {
    #[must_use]
    #[inline(always)]
    pub(crate) fn base<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDiEE,
    ) -> *const u32 {
        unsafe {
            crate::detail::__rust_thunk__1b6af95a__ZNKSt3__u16reverse_iteratorIPKDiE4baseEv(__this)
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u16reverse_iteratorIPKDsEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=std :: reverse_iterator < const char16_t *>
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u16reverse_iteratorIPKDsEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) current: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for __CcTemplateInstNSt3__u16reverse_iteratorIPKDsEE {}
impl !Sync for __CcTemplateInstNSt3__u16reverse_iteratorIPKDsEE {}
impl __CcTemplateInstNSt3__u16reverse_iteratorIPKDsEE {
    #[must_use]
    #[inline(always)]
    pub fn base<'__this>(&'__this self) -> *const u16 {
        unsafe { self::cc_template_inst_n_st3_u16reverse_iterator_ipk_ds_ee::base(self) }
    }
}

impl Default for __CcTemplateInstNSt3__u16reverse_iteratorIPKDsEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__63d556e1__ZNSt3__u16reverse_iteratorIPKDsEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

impl ::ctor::UnsafeFrom<*const u16> for __CcTemplateInstNSt3__u16reverse_iteratorIPKDsEE {
    #[inline(always)]
    unsafe fn unsafe_from(args: *const u16) -> Self {
        let mut __x = args;
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__9cfa002e__ZNSt3__u16reverse_iteratorIPKDsEC1ES2_(
                &raw mut tmp as *mut _,
                __x,
            );
            tmp.assume_init()
        }
    }
}
impl ::ctor::UnsafeCtorNew<*const u16> for __CcTemplateInstNSt3__u16reverse_iteratorIPKDsEE {
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    unsafe fn ctor_new(args: *const u16) -> Self::CtorType {
        unsafe { <Self as ::ctor::UnsafeFrom<*const u16>>::unsafe_from(args) }
    }
}

pub mod cc_template_inst_n_st3_u16reverse_iterator_ipk_ds_ee {
    #[must_use]
    #[inline(always)]
    pub(crate) fn base<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDsEE,
    ) -> *const u16 {
        unsafe {
            crate::detail::__rust_thunk__1b6af95a__ZNKSt3__u16reverse_iteratorIPKDsE4baseEv(__this)
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u16reverse_iteratorIPKDuEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=std :: reverse_iterator < const char8_t *>
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u16reverse_iteratorIPKDuEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) current: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for __CcTemplateInstNSt3__u16reverse_iteratorIPKDuEE {}
impl !Sync for __CcTemplateInstNSt3__u16reverse_iteratorIPKDuEE {}

impl Default for __CcTemplateInstNSt3__u16reverse_iteratorIPKDuEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__63d556e1__ZNSt3__u16reverse_iteratorIPKDuEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u16reverse_iteratorIPKcEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=std :: reverse_iterator < const char *>
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u16reverse_iteratorIPKcEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) current: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for __CcTemplateInstNSt3__u16reverse_iteratorIPKcEE {}
impl !Sync for __CcTemplateInstNSt3__u16reverse_iteratorIPKcEE {}
impl __CcTemplateInstNSt3__u16reverse_iteratorIPKcEE {
    #[must_use]
    #[inline(always)]
    pub fn base<'__this>(&'__this self) -> *const ::ffi_11::c_char {
        unsafe { self::cc_template_inst_n_st3_u16reverse_iterator_ip_kc_ee::base(self) }
    }
}

impl Default for __CcTemplateInstNSt3__u16reverse_iteratorIPKcEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__63d556e1__ZNSt3__u16reverse_iteratorIPKcEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

impl ::ctor::UnsafeFrom<*const ::ffi_11::c_char>
    for __CcTemplateInstNSt3__u16reverse_iteratorIPKcEE
{
    #[inline(always)]
    unsafe fn unsafe_from(args: *const ::ffi_11::c_char) -> Self {
        let mut __x = args;
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__9cfa002e__ZNSt3__u16reverse_iteratorIPKcEC1ES2_(
                &raw mut tmp as *mut _,
                __x,
            );
            tmp.assume_init()
        }
    }
}
impl ::ctor::UnsafeCtorNew<*const ::ffi_11::c_char>
    for __CcTemplateInstNSt3__u16reverse_iteratorIPKcEE
{
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    unsafe fn ctor_new(args: *const ::ffi_11::c_char) -> Self::CtorType {
        unsafe { <Self as ::ctor::UnsafeFrom<*const ::ffi_11::c_char>>::unsafe_from(args) }
    }
}

pub mod cc_template_inst_n_st3_u16reverse_iterator_ip_kc_ee {
    #[must_use]
    #[inline(always)]
    pub(crate) fn base<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKcEE,
    ) -> *const ::ffi_11::c_char {
        unsafe {
            crate::detail::__rust_thunk__1b6af95a__ZNKSt3__u16reverse_iteratorIPKcE4baseEv(__this)
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u16reverse_iteratorIPKwEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=std :: reverse_iterator < const wchar_t *>
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u16reverse_iteratorIPKwEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) current: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for __CcTemplateInstNSt3__u16reverse_iteratorIPKwEE {}
impl !Sync for __CcTemplateInstNSt3__u16reverse_iteratorIPKwEE {}

impl Default for __CcTemplateInstNSt3__u16reverse_iteratorIPKwEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__63d556e1__ZNSt3__u16reverse_iteratorIPKwEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

// error: class `std::basic_filebuf<char, std::char_traits<char>>` could not be bound
//   incomplete type

// error: class `std::basic_filebuf<wchar_t, std::char_traits<wchar_t>>` could not be bound
//   incomplete type

// error: class `std::basic_ifstream<char, std::char_traits<char>>` could not be bound
//   incomplete type

// error: class `std::basic_ifstream<wchar_t, std::char_traits<wchar_t>>` could not be bound
//   incomplete type

// error: class `std::basic_ofstream<char, std::char_traits<char>>` could not be bound
//   incomplete type

// error: class `std::basic_ofstream<wchar_t, std::char_traits<wchar_t>>` could not be bound
//   incomplete type

// error: class `std::basic_fstream<char, std::char_traits<char>>` could not be bound
//   incomplete type

// error: class `std::basic_fstream<wchar_t, std::char_traits<wchar_t>>` could not be bound
//   incomplete type

// error: class `std::basic_ios<char, std::char_traits<char>>` could not be bound
//   incomplete type

// error: class `std::basic_ios<wchar_t, std::char_traits<wchar_t>>` could not be bound
//   incomplete type

// error: class `std::basic_istream<char, std::char_traits<char>>` could not be bound
//   incomplete type

// error: class `std::basic_istream<wchar_t, std::char_traits<wchar_t>>` could not be bound
//   incomplete type

// error: class `std::basic_iostream<char, std::char_traits<char>>` could not be bound
//   incomplete type

// error: class `std::basic_iostream<wchar_t, std::char_traits<wchar_t>>` could not be bound
//   incomplete type

// error: class `std::basic_stringbuf<char, std::char_traits<char>, std::allocator<char>>` could not be bound
//   incomplete type

// error: class `std::basic_stringbuf<wchar_t, std::char_traits<wchar_t>, std::allocator<wchar_t>>` could not be bound
//   incomplete type

// error: class `std::basic_istringstream<char, std::char_traits<char>, std::allocator<char>>` could not be bound
//   incomplete type

// error: class `std::basic_istringstream<wchar_t, std::char_traits<wchar_t>, std::allocator<wchar_t>>` could not be bound
//   incomplete type

// error: class `std::basic_ostringstream<char, std::char_traits<char>, std::allocator<char>>` could not be bound
//   incomplete type

// error: class `std::basic_ostringstream<wchar_t, std::char_traits<wchar_t>, std::allocator<wchar_t>>` could not be bound
//   incomplete type

// error: class `std::basic_stringstream<char, std::char_traits<char>, std::allocator<char>>` could not be bound
//   incomplete type

// error: class `std::basic_stringstream<wchar_t, std::char_traits<wchar_t>, std::allocator<wchar_t>>` could not be bound
//   incomplete type

// error: class `std::basic_streambuf<char, std::char_traits<char>>` could not be bound
//   incomplete type

// error: class `std::basic_streambuf<wchar_t, std::char_traits<wchar_t>>` could not be bound
//   incomplete type

// error: class `std::fpos<__mbstate_t>` could not be bound
//   incomplete type

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN13TypeAliasCtorC1ERKS_<'__param_0>(
            __this: *mut ::core::ffi::c_void,
            __param_0: &'__param_0 crate::TypeAliasCtor,
        );
        pub(crate) unsafe fn __rust_thunk___ZN13TypeAliasCtoraSERKS_<'__param_0, '__this>(
            __this: ::core::pin::Pin<&'__this mut crate::TypeAliasCtor>,
            __param_0: &'__param_0 crate::TypeAliasCtor,
        ) -> ::core::pin::Pin<&'__this mut crate::TypeAliasCtor>;
        pub(crate) unsafe fn __rust_thunk___ZN13TypeAliasCtorC1ENSt3__u17basic_string_viewIcNS0_11char_traitsIcEEEE<
            'a,
        >(
            __this: *mut ::core::ffi::c_void,
            a: &mut ::cc_std::std::string_view<'a>,
        );
        #[link_name = "_ZN13TypeAliasCtorD1Ev"]
        pub(crate) unsafe fn __rust_thunk___ZN13TypeAliasCtorD1Ev<'__this>(
            __this: ::core::pin::Pin<&'__this mut crate::TypeAliasCtor>,
        );
        pub(crate) unsafe fn __rust_thunk__e3237267__ZNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__35143324__ZNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEC1EPKDi(
            __this: *mut ::core::ffi::c_void,
            __s: *const u32,
        );
        pub(crate) unsafe fn __rust_thunk__d87a5768__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE5beginEv<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
        ) -> *const u32;
        pub(crate) unsafe fn __rust_thunk__9cd7c456__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE3endEv<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
        ) -> *const u32;
        pub(crate) unsafe fn __rust_thunk__ba84366a__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE6cbeginEv<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
        ) -> *const u32;
        pub(crate) unsafe fn __rust_thunk__98fd39f0__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE4cendEv<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
        ) -> *const u32;
        pub(crate) unsafe fn __rust_thunk__e474ba02__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE6rbeginEv<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__45754841__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE4rendEv<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__572dcabe__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE7crbeginEv<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__8b827fea__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE5crendEv<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__4fcc8cae__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE4sizeEv<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
        ) -> usize;
        pub(crate) unsafe fn __rust_thunk__a1b282c6__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE6lengthEv<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
        ) -> usize;
        pub(crate) unsafe fn __rust_thunk__f9699d7a__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE8max_sizeEv<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
        ) -> usize;
        pub(crate) unsafe fn __rust_thunk__1e821d1e__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE5emptyEv<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
        ) -> bool;
        pub(crate) unsafe fn __rust_thunk__6e80e7d7__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEixEm<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
            __pos: usize,
        ) -> &'__this u32;
        pub(crate) unsafe fn __rust_thunk__ab69815c__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE2atEm<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
            __pos: usize,
        ) -> ::cref::CRef<'__this, u32>;
        pub(crate) unsafe fn __rust_thunk__9dc1d8ab__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE5frontEv<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
        ) -> ::cref::CRef<'__this, u32>;
        pub(crate) unsafe fn __rust_thunk__20abdabc__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE4backEv<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
        ) -> ::cref::CRef<'__this, u32>;
        pub(crate) unsafe fn __rust_thunk__659db6d3__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE4dataEv<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
        ) -> *const u32;
        pub(crate) unsafe fn __rust_thunk__06c1d1b6__ZNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE13remove_prefixEm<
            '__this,
        >(
            __this: &'__this mut crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
            __n: usize,
        );
        pub(crate) unsafe fn __rust_thunk__a7391000__ZNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE13remove_suffixEm<
            '__this,
        >(
            __this: &'__this mut crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
            __n: usize,
        );
        pub(crate) unsafe fn __rust_thunk__f1d195d0__ZNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE4swapERS3_<
            '__other,
            '__this,
        >(
            __this: &'__this mut crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
            __other: &'__other mut crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__63a9d1e0__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE4copyEPDimm<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
            __s: *mut u32,
            __n: usize,
            __pos: usize,
        ) -> usize;
        pub(crate) unsafe fn __rust_thunk__86ba0b92__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE6substrEmm<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
            __pos: usize,
            __n: usize,
        );
        pub(crate) unsafe fn __rust_thunk__e3237267__ZNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__35143324__ZNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEC1EPKDs(
            __this: *mut ::core::ffi::c_void,
            __s: *const u16,
        );
        pub(crate) unsafe fn __rust_thunk__d87a5768__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE5beginEv<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
        ) -> *const u16;
        pub(crate) unsafe fn __rust_thunk__9cd7c456__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE3endEv<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
        ) -> *const u16;
        pub(crate) unsafe fn __rust_thunk__ba84366a__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE6cbeginEv<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
        ) -> *const u16;
        pub(crate) unsafe fn __rust_thunk__98fd39f0__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE4cendEv<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
        ) -> *const u16;
        pub(crate) unsafe fn __rust_thunk__e474ba02__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE6rbeginEv<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__45754841__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE4rendEv<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__572dcabe__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE7crbeginEv<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__8b827fea__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE5crendEv<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__4fcc8cae__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE4sizeEv<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
        ) -> usize;
        pub(crate) unsafe fn __rust_thunk__a1b282c6__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE6lengthEv<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
        ) -> usize;
        pub(crate) unsafe fn __rust_thunk__f9699d7a__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE8max_sizeEv<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
        ) -> usize;
        pub(crate) unsafe fn __rust_thunk__1e821d1e__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE5emptyEv<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
        ) -> bool;
        pub(crate) unsafe fn __rust_thunk__6e80e7d7__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEixEm<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
            __pos: usize,
        ) -> &'__this u16;
        pub(crate) unsafe fn __rust_thunk__ab69815c__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE2atEm<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
            __pos: usize,
        ) -> ::cref::CRef<'__this, u16>;
        pub(crate) unsafe fn __rust_thunk__9dc1d8ab__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE5frontEv<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
        ) -> ::cref::CRef<'__this, u16>;
        pub(crate) unsafe fn __rust_thunk__20abdabc__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE4backEv<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
        ) -> ::cref::CRef<'__this, u16>;
        pub(crate) unsafe fn __rust_thunk__659db6d3__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE4dataEv<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
        ) -> *const u16;
        pub(crate) unsafe fn __rust_thunk__06c1d1b6__ZNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE13remove_prefixEm<
            '__this,
        >(
            __this: &'__this mut crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
            __n: usize,
        );
        pub(crate) unsafe fn __rust_thunk__a7391000__ZNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE13remove_suffixEm<
            '__this,
        >(
            __this: &'__this mut crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
            __n: usize,
        );
        pub(crate) unsafe fn __rust_thunk__f1d195d0__ZNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE4swapERS3_<
            '__other,
            '__this,
        >(
            __this: &'__this mut crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
            __other: &'__other mut crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__63a9d1e0__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE4copyEPDsmm<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
            __s: *mut u16,
            __n: usize,
            __pos: usize,
        ) -> usize;
        pub(crate) unsafe fn __rust_thunk__86ba0b92__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE6substrEmm<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
            __pos: usize,
            __n: usize,
        );
        pub(crate) unsafe fn __rust_thunk__e3237267__ZNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__e474ba02__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE6rbeginEv<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__45754841__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE4rendEv<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__572dcabe__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE7crbeginEv<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__8b827fea__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE5crendEv<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__4fcc8cae__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE4sizeEv<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
        ) -> usize;
        pub(crate) unsafe fn __rust_thunk__a1b282c6__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE6lengthEv<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
        ) -> usize;
        pub(crate) unsafe fn __rust_thunk__f9699d7a__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE8max_sizeEv<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
        ) -> usize;
        pub(crate) unsafe fn __rust_thunk__1e821d1e__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE5emptyEv<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
        ) -> bool;
        pub(crate) unsafe fn __rust_thunk__06c1d1b6__ZNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE13remove_prefixEm<
            '__this,
        >(
            __this: &'__this mut crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
            __n: usize,
        );
        pub(crate) unsafe fn __rust_thunk__a7391000__ZNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE13remove_suffixEm<
            '__this,
        >(
            __this: &'__this mut crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
            __n: usize,
        );
        pub(crate) unsafe fn __rust_thunk__f1d195d0__ZNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE4swapERS3_<
            '__other,
            '__this,
        >(
            __this: &'__this mut crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
            __other: &'__other mut crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__86ba0b92__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE6substrEmm<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
            __pos: usize,
            __n: usize,
        );
        pub(crate) unsafe fn __rust_thunk__654b550f__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE4findES3_m<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
            __s: &mut crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
            __pos: usize,
        ) -> usize;
        pub(crate) unsafe fn __rust_thunk__dbd88331__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE5rfindES3_m<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
            __s: &mut crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
            __pos: usize,
        ) -> usize;
        pub(crate) unsafe fn __rust_thunk__3ba36d97__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE13find_first_ofES3_m<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
            __s: &mut crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
            __pos: usize,
        ) -> usize;
        pub(crate) unsafe fn __rust_thunk__b8a8a87c__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE12find_last_ofES3_m<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
            __s: &mut crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
            __pos: usize,
        ) -> usize;
        pub(crate) unsafe fn __rust_thunk__a6fd6d60__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE17find_first_not_ofES3_m<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
            __s: &mut crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
            __pos: usize,
        ) -> usize;
        pub(crate) unsafe fn __rust_thunk__27e3987c__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE16find_last_not_ofES3_m<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
            __s: &mut crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
            __pos: usize,
        ) -> usize;
        pub(crate) unsafe fn __rust_thunk__1f379720__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE11starts_withES3_<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
            __s: &mut crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
        ) -> bool;
        pub(crate) unsafe fn __rust_thunk__6beb84a4__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE9ends_withES3_<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
            __s: &mut crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
        ) -> bool;
        pub(crate) unsafe fn __rust_thunk__63d556e1__ZNSt3__u16reverse_iteratorIPKDiEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__9cfa002e__ZNSt3__u16reverse_iteratorIPKDiEC1ES2_(
            __this: *mut ::core::ffi::c_void,
            __x: *const u32,
        );
        pub(crate) unsafe fn __rust_thunk__1b6af95a__ZNKSt3__u16reverse_iteratorIPKDiE4baseEv<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDiEE,
        ) -> *const u32;
        pub(crate) unsafe fn __rust_thunk__63d556e1__ZNSt3__u16reverse_iteratorIPKDsEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__9cfa002e__ZNSt3__u16reverse_iteratorIPKDsEC1ES2_(
            __this: *mut ::core::ffi::c_void,
            __x: *const u16,
        );
        pub(crate) unsafe fn __rust_thunk__1b6af95a__ZNKSt3__u16reverse_iteratorIPKDsE4baseEv<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDsEE,
        ) -> *const u16;
        pub(crate) unsafe fn __rust_thunk__63d556e1__ZNSt3__u16reverse_iteratorIPKDuEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__63d556e1__ZNSt3__u16reverse_iteratorIPKcEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__9cfa002e__ZNSt3__u16reverse_iteratorIPKcEC1ES2_(
            __this: *mut ::core::ffi::c_void,
            __x: *const ::ffi_11::c_char,
        );
        pub(crate) unsafe fn __rust_thunk__1b6af95a__ZNKSt3__u16reverse_iteratorIPKcE4baseEv<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKcEE,
        ) -> *const ::ffi_11::c_char;
        pub(crate) unsafe fn __rust_thunk__63d556e1__ZNSt3__u16reverse_iteratorIPKwEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::TypeAliasCtor>() == 1);
    assert!(::core::mem::align_of::<crate::TypeAliasCtor>() == 1);
    static_assertions::assert_impl_all!(crate::TypeAliasCtor: Drop);
    static_assertions::assert_not_impl_any!(crate::TypeAliasCtor: Copy);

    assert!(
        ::core::mem::size_of::<
            crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
        >() == 16
    );
    assert!(
        ::core::mem::align_of::<
            crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
        >() == 8
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE: Drop);
    assert!(
        ::core::mem::offset_of!(
            crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
            __data_
        ) == 0
    );
    assert!(
        ::core::mem::offset_of!(
            crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
            __size_
        ) == 8
    );
    assert!(
        ::core::mem::size_of::<
            crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
        >() == 16
    );
    assert!(
        ::core::mem::align_of::<
            crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
        >() == 8
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE: Drop);
    assert!(
        ::core::mem::offset_of!(
            crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
            __data_
        ) == 0
    );
    assert!(
        ::core::mem::offset_of!(
            crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
            __size_
        ) == 8
    );
    assert!(
        ::core::mem::size_of::<
            crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
        >() == 16
    );
    assert!(
        ::core::mem::align_of::<
            crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
        >() == 8
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE: Drop);
    assert!(
        ::core::mem::offset_of!(
            crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
            __data_
        ) == 0
    );
    assert!(
        ::core::mem::offset_of!(
            crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
            __size_
        ) == 8
    );
    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDiEE>() == 8);
    assert!(
        ::core::mem::align_of::<crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDiEE>() == 8
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDiEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDiEE: Drop);
    assert!(
        ::core::mem::offset_of!(crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDiEE, current)
            == 0
    );
    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDsEE>() == 8);
    assert!(
        ::core::mem::align_of::<crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDsEE>() == 8
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDsEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDsEE: Drop);
    assert!(
        ::core::mem::offset_of!(crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDsEE, current)
            == 0
    );
    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDuEE>() == 8);
    assert!(
        ::core::mem::align_of::<crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDuEE>() == 8
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDuEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDuEE: Drop);
    assert!(
        ::core::mem::offset_of!(crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDuEE, current)
            == 0
    );
    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKcEE>() == 8);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKcEE>() == 8);
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKcEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKcEE: Drop);
    assert!(
        ::core::mem::offset_of!(crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKcEE, current)
            == 0
    );
    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKwEE>() == 8);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKwEE>() == 8);
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKwEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKwEE: Drop);
    assert!(
        ::core::mem::offset_of!(crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKwEE, current)
            == 0
    );
};
