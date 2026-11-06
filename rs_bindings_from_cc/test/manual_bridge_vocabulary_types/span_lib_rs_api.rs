// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/manual_bridge_vocabulary_types:span_lib

#![rustfmt::skip]
#![feature(cfi_encoding, custom_inner_attributes, impl_trait_in_assoc_type, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![allow(deprecated)]
#![allow(unknown_lints, suspicious_runtime_symbol_definitions)]
#![deny(warnings)]
/// Not Rust-movable: destructor is user-provided; annotate the type with CRUBIT_UNSAFE_BITWISE_RELOCATABLE if a memcpy move is sound
#[::ctor::recursively_pinned(PinnedDrop)]
#[cfi_encoding = "10NonTrivial"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=NonTrivial
pub struct NonTrivial {
    pub num: usize,
}
impl !Send for NonTrivial {}
impl !Sync for NonTrivial {}
unsafe impl ::cxx::ExternType for NonTrivial {
    type Id = ::cxx::type_id!("NonTrivial");
    type Kind = ::cxx::kind::Opaque;
}
forward_declare::unsafe_define!(forward_declare::symbol!("NonTrivial"), crate::NonTrivial);

impl ::ctor::CtorNew<()> for NonTrivial {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                let () = args;
                crate::detail::__rust_thunk___ZN10NonTrivialC1Ev(
                    __crubit_dest as *mut ::core::ffi::c_void,
                );
            })
        }
    }
}

impl<'__param_0> ::ctor::CtorNew<&'__param_0 Self> for NonTrivial {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: &'__param_0 Self) -> Self::CtorType {
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                let mut __param_0 = args;
                crate::detail::__rust_thunk___ZN10NonTrivialC1ERKS_(
                    __crubit_dest as *mut ::core::ffi::c_void,
                    __param_0,
                );
            })
        }
    }
}
impl<'__param_0> ::ctor::CtorNew<(&'__param_0 Self,)> for NonTrivial {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (&'__param_0 Self,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'__param_0 Self>>::ctor_new(arg)
    }
}

impl<'__param_0> ::ctor::Assign<&'__param_0 Self> for NonTrivial {
    #[inline(always)]
    fn assign<'__this>(self: ::core::pin::Pin<&'__this mut Self>, __param_0: &'__param_0 Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN10NonTrivialaSERKS_(self, __param_0);
        }
    }
}

impl ::ctor::PinnedDrop for NonTrivial {
    #[inline(always)]
    unsafe fn pinned_drop<'__this>(self: ::core::pin::Pin<&'__this mut Self>) {
        unsafe { crate::detail::__rust_thunk___ZN10NonTrivialD1Ev(self) }
    }
}

#[inline(always)]
pub fn TruncateSpanMut<'span>(
    span: ::_y__y_third_party_y_absl_y_types_x_span::absl::SpanMut<'span, ::ffi_11::c_int>,
    len: usize,
) -> ::_y__y_third_party_y_absl_y_types_x_span::absl::SpanMut<'span, ::ffi_11::c_int> {
    let mut span = ::core::mem::MaybeUninit::new(span);
    unsafe {
        let mut __crubit_return = ::core::mem::MaybeUninit::<
            ::_y__y_third_party_y_absl_y_types_x_span::absl::SpanMut<'span, ::ffi_11::c_int>,
        >::uninit();
        crate::detail::__rust_thunk___Z15TruncateSpanMutN4absl4SpanIiEEm(
            &raw mut __crubit_return as *mut ::core::ffi::c_void,
            span.as_mut_ptr(),
            len,
        );
        __crubit_return.assume_init()
    }
}

#[inline(always)]
pub fn TruncateSpan<'span>(
    span: ::_y__y_third_party_y_absl_y_types_x_span::absl::Span<'span, ::ffi_11::c_int>,
    len: usize,
) -> ::_y__y_third_party_y_absl_y_types_x_span::absl::Span<'span, ::ffi_11::c_int> {
    let mut span = ::core::mem::MaybeUninit::new(span);
    unsafe {
        let mut __crubit_return = ::core::mem::MaybeUninit::<
            ::_y__y_third_party_y_absl_y_types_x_span::absl::Span<'span, ::ffi_11::c_int>,
        >::uninit();
        crate::detail::__rust_thunk___Z12TruncateSpanN4absl4SpanIKiEEm(
            &raw mut __crubit_return as *mut ::core::ffi::c_void,
            span.as_mut_ptr(),
            len,
        );
        __crubit_return.assume_init()
    }
}

#[inline(always)]
pub fn MakeNonTrivial<'span>(
    span: ::_y__y_third_party_y_absl_y_types_x_span::absl::Span<'span, ::ffi_11::c_int>,
) -> ::ctor::Ctor![crate::NonTrivial] {
    unsafe {
        ::ctor::FnCtor::new(move |__crubit_dest: *mut crate::NonTrivial| {
            let mut span = ::core::mem::MaybeUninit::new(span);
            crate::detail::__rust_thunk___Z14MakeNonTrivialN4absl4SpanIKiEE(
                __crubit_dest as *mut ::core::ffi::c_void,
                span.as_mut_ptr(),
            );
        })
    }
}

/// Cannot easily construct such a span, but it at least receives bindings.
#[inline(always)]
pub fn TruncateSpanNonTrivialMut<'span>(
    span: ::_y__y_third_party_y_absl_y_types_x_span::absl::SpanMut<'span, crate::NonTrivial>,
    len: usize,
) -> ::_y__y_third_party_y_absl_y_types_x_span::absl::SpanMut<'span, crate::NonTrivial> {
    let mut span = ::core::mem::MaybeUninit::new(span);
    unsafe {
        let mut __crubit_return = ::core::mem::MaybeUninit::<
            ::_y__y_third_party_y_absl_y_types_x_span::absl::SpanMut<'span, crate::NonTrivial>,
        >::uninit();
        crate::detail::__rust_thunk___Z25TruncateSpanNonTrivialMutN4absl4SpanI10NonTrivialEEm(
            &raw mut __crubit_return as *mut ::core::ffi::c_void,
            span.as_mut_ptr(),
            len,
        );
        __crubit_return.assume_init()
    }
}

#[inline(always)]
pub fn TruncateSpanNonTrivial<'span>(
    span: ::_y__y_third_party_y_absl_y_types_x_span::absl::Span<'span, crate::NonTrivial>,
    len: usize,
) -> ::_y__y_third_party_y_absl_y_types_x_span::absl::Span<'span, crate::NonTrivial> {
    let mut span = ::core::mem::MaybeUninit::new(span);
    unsafe {
        let mut __crubit_return = ::core::mem::MaybeUninit::<
            ::_y__y_third_party_y_absl_y_types_x_span::absl::Span<'span, crate::NonTrivial>,
        >::uninit();
        crate::detail::__rust_thunk___Z22TruncateSpanNonTrivialN4absl4SpanIK10NonTrivialEEm(
            &raw mut __crubit_return as *mut ::core::ffi::c_void,
            span.as_mut_ptr(),
            len,
        );
        __crubit_return.assume_init()
    }
}

/// # Safety
///
/// To call a function that accepts this type, you must uphold these requirements:
/// * Document why the following public unsafe fields of this type cannot be misused by callee:
///   * `span`: type __CcTemplateInstN4absl4SpanIKiEE has 1 lifetime parameter, but 0 were provided; callers must ensure that arguments have the appropriate lifetime
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "19StructWithSpanField"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=StructWithSpanField
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct StructWithSpanField {
    /// Reason for representing this field as a blob of bytes:
    /// Fields of type `::_y__y_third_party_y_absl_y_types_x_span::absl::Span<'_,::ffi_11::c_int>` with an elided lifetime are not yet supported (b/576904034); annotate the struct with `$lifetime_params("a")` and the field type with `$a` (or `$static`) from `//support:lifetime_annotations`
    pub(crate) span: [::core::mem::MaybeUninit<u8>; 16],
    pub other: ::ffi_11::c_int,
}
impl !Send for StructWithSpanField {}
impl !Sync for StructWithSpanField {}
unsafe impl ::cxx::ExternType for StructWithSpanField {
    type Id = ::cxx::type_id!("StructWithSpanField");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("StructWithSpanField"),
    crate::StructWithSpanField
);

impl Default for StructWithSpanField {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN19StructWithSpanFieldC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

#[derive(Clone, Copy)]
#[cfi_encoding = "36StructWithLifetimeAnnotatedSpanField"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=StructWithLifetimeAnnotatedSpanField
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct StructWithLifetimeAnnotatedSpanField<'a> {
    pub span: ::_y__y_third_party_y_absl_y_types_x_span::absl::Span<'a, ::ffi_11::c_int>,
    pub other: ::ffi_11::c_int,
    __marker_a: ::core::marker::PhantomData<&'a ()>,
}
impl<'a> !Send for StructWithLifetimeAnnotatedSpanField<'a> {}
impl<'a> !Sync for StructWithLifetimeAnnotatedSpanField<'a> {}
unsafe impl<'a> ::cxx::ExternType for StructWithLifetimeAnnotatedSpanField<'a> {
    type Id = ::cxx::type_id!("StructWithLifetimeAnnotatedSpanField");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("StructWithLifetimeAnnotatedSpanField"),
    crate::StructWithLifetimeAnnotatedSpanField<'_>
);

impl<'a> Default for StructWithLifetimeAnnotatedSpanField<'a> {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN36StructWithLifetimeAnnotatedSpanFieldC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u16reverse_iteratorIP10NonTrivialEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=std :: reverse_iterator < struct NonTrivial *>
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u16reverse_iteratorIP10NonTrivialEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) current: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for __CcTemplateInstNSt3__u16reverse_iteratorIP10NonTrivialEE {}
impl !Sync for __CcTemplateInstNSt3__u16reverse_iteratorIP10NonTrivialEE {}
forward_declare::unsafe_define!(
    forward_declare::symbol!("std :: reverse_iterator < struct NonTrivial *>"),
    crate::__CcTemplateInstNSt3__u16reverse_iteratorIP10NonTrivialEE
);
impl __CcTemplateInstNSt3__u16reverse_iteratorIP10NonTrivialEE {
    #[must_use]
    #[inline(always)]
    pub fn base<'__this>(&'__this self) -> *mut crate::NonTrivial {
        unsafe { self::cc_template_inst_n_st3_u16reverse_iterator_ip10_non_trivial_ee::base(self) }
    }
}

impl Default for __CcTemplateInstNSt3__u16reverse_iteratorIP10NonTrivialEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__cbab09f8__ZNSt3__u16reverse_iteratorIP10NonTrivialEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

/// # Safety
///
/// The caller must ensure that the following unsafe arguments are not misused by the function:
/// * `__x`: raw pointer
impl ::ctor::UnsafeFrom<*mut crate::NonTrivial>
    for __CcTemplateInstNSt3__u16reverse_iteratorIP10NonTrivialEE
{
    #[inline(always)]
    unsafe fn unsafe_from(args: *mut crate::NonTrivial) -> Self {
        let mut __x = args;
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__466dfd43__ZNSt3__u16reverse_iteratorIP10NonTrivialEC1ES2_(
                &raw mut tmp as *mut _,
                __x,
            );
            tmp.assume_init()
        }
    }
}
impl ::ctor::UnsafeCtorNew<*mut crate::NonTrivial>
    for __CcTemplateInstNSt3__u16reverse_iteratorIP10NonTrivialEE
{
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    unsafe fn ctor_new(args: *mut crate::NonTrivial) -> Self::CtorType {
        unsafe { <Self as ::ctor::UnsafeFrom<*mut crate::NonTrivial>>::unsafe_from(args) }
    }
}

pub mod cc_template_inst_n_st3_u16reverse_iterator_ip10_non_trivial_ee {
    #[must_use]
    ///CRUBIT_ANNOTATE: do_not_bind=
    #[inline(always)]
    pub fn base<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u16reverse_iteratorIP10NonTrivialEE,
    ) -> *mut crate::NonTrivial {
        unsafe {
            crate::detail::__rust_thunk__3ef42fe9__ZNKSt3__u16reverse_iteratorIP10NonTrivialE4baseEv(
                __this,
            )
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u16reverse_iteratorIPK10NonTrivialEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=std :: reverse_iterator < const struct NonTrivial *>
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u16reverse_iteratorIPK10NonTrivialEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) current: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for __CcTemplateInstNSt3__u16reverse_iteratorIPK10NonTrivialEE {}
impl !Sync for __CcTemplateInstNSt3__u16reverse_iteratorIPK10NonTrivialEE {}
forward_declare::unsafe_define!(
    forward_declare::symbol!("std :: reverse_iterator < const struct NonTrivial *>"),
    crate::__CcTemplateInstNSt3__u16reverse_iteratorIPK10NonTrivialEE
);
impl __CcTemplateInstNSt3__u16reverse_iteratorIPK10NonTrivialEE {
    #[must_use]
    #[inline(always)]
    pub fn base<'__this>(&'__this self) -> *const crate::NonTrivial {
        unsafe { self::cc_template_inst_n_st3_u16reverse_iterator_ipk10_non_trivial_ee::base(self) }
    }
}

impl Default for __CcTemplateInstNSt3__u16reverse_iteratorIPK10NonTrivialEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__cbab09f8__ZNSt3__u16reverse_iteratorIPK10NonTrivialEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

/// # Safety
///
/// The caller must ensure that the following unsafe arguments are not misused by the function:
/// * `__x`: raw pointer
impl ::ctor::UnsafeFrom<*const crate::NonTrivial>
    for __CcTemplateInstNSt3__u16reverse_iteratorIPK10NonTrivialEE
{
    #[inline(always)]
    unsafe fn unsafe_from(args: *const crate::NonTrivial) -> Self {
        let mut __x = args;
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__466dfd43__ZNSt3__u16reverse_iteratorIPK10NonTrivialEC1ES3_(
                &raw mut tmp as *mut _,
                __x,
            );
            tmp.assume_init()
        }
    }
}
impl ::ctor::UnsafeCtorNew<*const crate::NonTrivial>
    for __CcTemplateInstNSt3__u16reverse_iteratorIPK10NonTrivialEE
{
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    unsafe fn ctor_new(args: *const crate::NonTrivial) -> Self::CtorType {
        unsafe { <Self as ::ctor::UnsafeFrom<*const crate::NonTrivial>>::unsafe_from(args) }
    }
}

pub mod cc_template_inst_n_st3_u16reverse_iterator_ipk10_non_trivial_ee {
    #[must_use]
    ///CRUBIT_ANNOTATE: do_not_bind=
    #[inline(always)]
    pub fn base<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u16reverse_iteratorIPK10NonTrivialEE,
    ) -> *const crate::NonTrivial {
        unsafe {
            crate::detail::__rust_thunk__3ef42fe9__ZNKSt3__u16reverse_iteratorIPK10NonTrivialE4baseEv(__this)
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u16reverse_iteratorIPKiEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=std :: reverse_iterator < const int *>
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u16reverse_iteratorIPKiEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) current: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for __CcTemplateInstNSt3__u16reverse_iteratorIPKiEE {}
impl !Sync for __CcTemplateInstNSt3__u16reverse_iteratorIPKiEE {}
forward_declare::unsafe_define!(
    forward_declare::symbol!("std :: reverse_iterator < const int *>"),
    crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKiEE
);
impl __CcTemplateInstNSt3__u16reverse_iteratorIPKiEE {
    #[must_use]
    #[inline(always)]
    pub fn base<'__this>(&'__this self) -> *const ::ffi_11::c_int {
        unsafe { self::cc_template_inst_n_st3_u16reverse_iterator_ip_ki_ee::base(self) }
    }
}

impl Default for __CcTemplateInstNSt3__u16reverse_iteratorIPKiEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__cbab09f8__ZNSt3__u16reverse_iteratorIPKiEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

/// # Safety
///
/// The caller must ensure that the following unsafe arguments are not misused by the function:
/// * `__x`: raw pointer
impl ::ctor::UnsafeFrom<*const ::ffi_11::c_int>
    for __CcTemplateInstNSt3__u16reverse_iteratorIPKiEE
{
    #[inline(always)]
    unsafe fn unsafe_from(args: *const ::ffi_11::c_int) -> Self {
        let mut __x = args;
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__466dfd43__ZNSt3__u16reverse_iteratorIPKiEC1ES2_(
                &raw mut tmp as *mut _,
                __x,
            );
            tmp.assume_init()
        }
    }
}
impl ::ctor::UnsafeCtorNew<*const ::ffi_11::c_int>
    for __CcTemplateInstNSt3__u16reverse_iteratorIPKiEE
{
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    unsafe fn ctor_new(args: *const ::ffi_11::c_int) -> Self::CtorType {
        unsafe { <Self as ::ctor::UnsafeFrom<*const ::ffi_11::c_int>>::unsafe_from(args) }
    }
}

pub mod cc_template_inst_n_st3_u16reverse_iterator_ip_ki_ee {
    #[must_use]
    ///CRUBIT_ANNOTATE: do_not_bind=
    #[inline(always)]
    pub fn base<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKiEE,
    ) -> *const ::ffi_11::c_int {
        unsafe {
            crate::detail::__rust_thunk__3ef42fe9__ZNKSt3__u16reverse_iteratorIPKiE4baseEv(__this)
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u16reverse_iteratorIPiEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=std :: reverse_iterator < int *>
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u16reverse_iteratorIPiEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) current: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for __CcTemplateInstNSt3__u16reverse_iteratorIPiEE {}
impl !Sync for __CcTemplateInstNSt3__u16reverse_iteratorIPiEE {}
forward_declare::unsafe_define!(
    forward_declare::symbol!("std :: reverse_iterator < int *>"),
    crate::__CcTemplateInstNSt3__u16reverse_iteratorIPiEE
);
impl __CcTemplateInstNSt3__u16reverse_iteratorIPiEE {
    #[must_use]
    #[inline(always)]
    pub fn base<'__this>(&'__this self) -> *mut ::ffi_11::c_int {
        unsafe { self::cc_template_inst_n_st3_u16reverse_iterator_i_pi_ee::base(self) }
    }
}

impl Default for __CcTemplateInstNSt3__u16reverse_iteratorIPiEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__cbab09f8__ZNSt3__u16reverse_iteratorIPiEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

/// # Safety
///
/// The caller must ensure that the following unsafe arguments are not misused by the function:
/// * `__x`: raw pointer
impl ::ctor::UnsafeFrom<*mut ::ffi_11::c_int> for __CcTemplateInstNSt3__u16reverse_iteratorIPiEE {
    #[inline(always)]
    unsafe fn unsafe_from(args: *mut ::ffi_11::c_int) -> Self {
        let mut __x = args;
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__466dfd43__ZNSt3__u16reverse_iteratorIPiEC1ES1_(
                &raw mut tmp as *mut _,
                __x,
            );
            tmp.assume_init()
        }
    }
}
impl ::ctor::UnsafeCtorNew<*mut ::ffi_11::c_int>
    for __CcTemplateInstNSt3__u16reverse_iteratorIPiEE
{
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    unsafe fn ctor_new(args: *mut ::ffi_11::c_int) -> Self::CtorType {
        unsafe { <Self as ::ctor::UnsafeFrom<*mut ::ffi_11::c_int>>::unsafe_from(args) }
    }
}

pub mod cc_template_inst_n_st3_u16reverse_iterator_i_pi_ee {
    #[must_use]
    ///CRUBIT_ANNOTATE: do_not_bind=
    #[inline(always)]
    pub fn base<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u16reverse_iteratorIPiEE,
    ) -> *mut ::ffi_11::c_int {
        unsafe {
            crate::detail::__rust_thunk__3ef42fe9__ZNKSt3__u16reverse_iteratorIPiE4baseEv(__this)
        }
    }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN10NonTrivialC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN10NonTrivialC1ERKS_<'__param_0>(
            __this: *mut ::core::ffi::c_void,
            __param_0: &'__param_0 crate::NonTrivial,
        );
        pub(crate) unsafe fn __rust_thunk___ZN10NonTrivialaSERKS_<'__param_0, '__this>(
            __this: ::core::pin::Pin<&'__this mut crate::NonTrivial>,
            __param_0: &'__param_0 crate::NonTrivial,
        ) -> ::core::pin::Pin<&'__this mut crate::NonTrivial>;
        pub(crate) unsafe fn __rust_thunk___ZN10NonTrivialD1Ev<'__this>(
            __this: ::core::pin::Pin<&'__this mut crate::NonTrivial>,
        );
        pub(crate) unsafe fn __rust_thunk___Z15TruncateSpanMutN4absl4SpanIiEEm<'span>(
            __return: *mut ::core::ffi::c_void,
            span: *mut ::_y__y_third_party_y_absl_y_types_x_span::absl::SpanMut<
                'span,
                ::ffi_11::c_int,
            >,
            len: usize,
        );
        pub(crate) unsafe fn __rust_thunk___Z12TruncateSpanN4absl4SpanIKiEEm<'span>(
            __return: *mut ::core::ffi::c_void,
            span: *mut ::_y__y_third_party_y_absl_y_types_x_span::absl::Span<
                'span,
                ::ffi_11::c_int,
            >,
            len: usize,
        );
        pub(crate) unsafe fn __rust_thunk___Z14MakeNonTrivialN4absl4SpanIKiEE<'span>(
            __return: *mut ::core::ffi::c_void,
            span: *mut ::_y__y_third_party_y_absl_y_types_x_span::absl::Span<
                'span,
                ::ffi_11::c_int,
            >,
        );
        pub(crate) unsafe fn __rust_thunk___Z25TruncateSpanNonTrivialMutN4absl4SpanI10NonTrivialEEm<
            'span,
        >(
            __return: *mut ::core::ffi::c_void,
            span: *mut ::_y__y_third_party_y_absl_y_types_x_span::absl::SpanMut<
                'span,
                crate::NonTrivial,
            >,
            len: usize,
        );
        pub(crate) unsafe fn __rust_thunk___Z22TruncateSpanNonTrivialN4absl4SpanIK10NonTrivialEEm<
            'span,
        >(
            __return: *mut ::core::ffi::c_void,
            span: *mut ::_y__y_third_party_y_absl_y_types_x_span::absl::Span<
                'span,
                crate::NonTrivial,
            >,
            len: usize,
        );
        pub(crate) unsafe fn __rust_thunk___ZN19StructWithSpanFieldC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN36StructWithLifetimeAnnotatedSpanFieldC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__cbab09f8__ZNSt3__u16reverse_iteratorIP10NonTrivialEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__466dfd43__ZNSt3__u16reverse_iteratorIP10NonTrivialEC1ES2_(
            __this: *mut ::core::ffi::c_void,
            __x: *mut crate::NonTrivial,
        );
        pub(crate) unsafe fn __rust_thunk__3ef42fe9__ZNKSt3__u16reverse_iteratorIP10NonTrivialE4baseEv<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u16reverse_iteratorIP10NonTrivialEE,
        ) -> *mut crate::NonTrivial;
        pub(crate) unsafe fn __rust_thunk__cbab09f8__ZNSt3__u16reverse_iteratorIPK10NonTrivialEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__466dfd43__ZNSt3__u16reverse_iteratorIPK10NonTrivialEC1ES3_(
            __this: *mut ::core::ffi::c_void,
            __x: *const crate::NonTrivial,
        );
        pub(crate) unsafe fn __rust_thunk__3ef42fe9__ZNKSt3__u16reverse_iteratorIPK10NonTrivialE4baseEv<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u16reverse_iteratorIPK10NonTrivialEE,
        ) -> *const crate::NonTrivial;
        pub(crate) unsafe fn __rust_thunk__cbab09f8__ZNSt3__u16reverse_iteratorIPKiEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__466dfd43__ZNSt3__u16reverse_iteratorIPKiEC1ES2_(
            __this: *mut ::core::ffi::c_void,
            __x: *const ::ffi_11::c_int,
        );
        pub(crate) unsafe fn __rust_thunk__3ef42fe9__ZNKSt3__u16reverse_iteratorIPKiE4baseEv<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKiEE,
        ) -> *const ::ffi_11::c_int;
        pub(crate) unsafe fn __rust_thunk__cbab09f8__ZNSt3__u16reverse_iteratorIPiEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__466dfd43__ZNSt3__u16reverse_iteratorIPiEC1ES1_(
            __this: *mut ::core::ffi::c_void,
            __x: *mut ::ffi_11::c_int,
        );
        pub(crate) unsafe fn __rust_thunk__3ef42fe9__ZNKSt3__u16reverse_iteratorIPiE4baseEv<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u16reverse_iteratorIPiEE,
        ) -> *mut ::ffi_11::c_int;
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::NonTrivial>() == 8);
    assert!(::core::mem::align_of::<crate::NonTrivial>() == 8);
    static_assertions::assert_impl_all!(crate::NonTrivial: Drop);
    static_assertions::assert_not_impl_any!(crate::NonTrivial: Copy);
    assert!(::core::mem::offset_of!(crate::NonTrivial, num) == 0);
    static_assertions::assert_impl_all!(usize: Copy);
    assert!(::core::mem::size_of::<crate::StructWithSpanField>() == 24);
    assert!(::core::mem::align_of::<crate::StructWithSpanField>() == 8);
    static_assertions::assert_impl_all!(crate::StructWithSpanField: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::StructWithSpanField: Drop);
    assert!(::core::mem::offset_of!(crate::StructWithSpanField, span) == 0);
    assert!(::core::mem::offset_of!(crate::StructWithSpanField, other) == 16);
    assert!(::core::mem::size_of::<crate::StructWithLifetimeAnnotatedSpanField>() == 24);
    assert!(::core::mem::align_of::<crate::StructWithLifetimeAnnotatedSpanField>() == 8);
    static_assertions::assert_impl_all!(crate::StructWithLifetimeAnnotatedSpanField<'static>: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::StructWithLifetimeAnnotatedSpanField<'static>: Drop);
    assert!(::core::mem::offset_of!(crate::StructWithLifetimeAnnotatedSpanField, span) == 0);
    assert!(::core::mem::offset_of!(crate::StructWithLifetimeAnnotatedSpanField, other) == 16);
    assert!(
        ::core::mem::size_of::<crate::__CcTemplateInstNSt3__u16reverse_iteratorIP10NonTrivialEE>()
            == 8
    );
    assert!(
        ::core::mem::align_of::<crate::__CcTemplateInstNSt3__u16reverse_iteratorIP10NonTrivialEE>()
            == 8
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u16reverse_iteratorIP10NonTrivialEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u16reverse_iteratorIP10NonTrivialEE: Drop);
    assert!(
        ::core::mem::offset_of!(
            crate::__CcTemplateInstNSt3__u16reverse_iteratorIP10NonTrivialEE,
            current
        ) == 0
    );
    assert!(
        ::core::mem::size_of::<crate::__CcTemplateInstNSt3__u16reverse_iteratorIPK10NonTrivialEE>()
            == 8
    );
    assert!(
        ::core::mem::align_of::<crate::__CcTemplateInstNSt3__u16reverse_iteratorIPK10NonTrivialEE>(
        ) == 8
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u16reverse_iteratorIPK10NonTrivialEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u16reverse_iteratorIPK10NonTrivialEE: Drop);
    assert!(
        ::core::mem::offset_of!(
            crate::__CcTemplateInstNSt3__u16reverse_iteratorIPK10NonTrivialEE,
            current
        ) == 0
    );
    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKiEE>() == 8);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKiEE>() == 8);
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKiEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKiEE: Drop);
    assert!(
        ::core::mem::offset_of!(crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKiEE, current)
            == 0
    );
    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u16reverse_iteratorIPiEE>() == 8);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u16reverse_iteratorIPiEE>() == 8);
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u16reverse_iteratorIPiEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u16reverse_iteratorIPiEE: Drop);
    assert!(
        ::core::mem::offset_of!(crate::__CcTemplateInstNSt3__u16reverse_iteratorIPiEE, current)
            == 0
    );
};
