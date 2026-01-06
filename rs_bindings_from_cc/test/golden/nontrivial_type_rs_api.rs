// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc

#![rustfmt::skip]
#![feature(
    allocator_api,
    cfg_sanitize,
    custom_inner_attributes,
    impl_trait_in_assoc_type,
    negative_impls
)]
#![allow(stable_features)]
#![no_std]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

/// Nontrivial due to (declared, but not yet defined) user-specified constructor
/// and destructor.
///
/// This makes it nontrivial for calls (so not trivially relocatable), as well
/// as specifically giving it a nontrivial move constructor and destructor.
#[::ctor::recursively_pinned(PinnedDrop)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=Nontrivial
pub struct Nontrivial {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    pub field: ::ffi_11::c_int,
}
impl !Send for Nontrivial {}
impl !Sync for Nontrivial {}
unsafe impl ::cxx::ExternType for Nontrivial {
    type Id = ::cxx::type_id!("Nontrivial");
    type Kind = ::cxx::kind::Opaque;
}
impl Nontrivial {
    #[inline(always)]
    pub fn Unqualified<'a>(self: ::core::pin::Pin<&'a mut Self>) {
        unsafe { crate::detail::__rust_thunk___ZN10Nontrivial11UnqualifiedEv(self) }
    }
    #[inline(always)]
    pub fn ConstQualified<'a>(&'a self) {
        unsafe { crate::detail::__rust_thunk___ZNK10Nontrivial14ConstQualifiedEv(self) }
    }
    #[inline(always)]
    pub fn LvalueRefQualified<'a>(self: ::core::pin::Pin<&'a mut Self>) {
        unsafe { crate::detail::__rust_thunk___ZNR10Nontrivial18LvalueRefQualifiedEv(self) }
    }
    #[inline(always)]
    pub fn ConstLvalueRefQualified<'a>(&'a self) {
        unsafe { crate::detail::__rust_thunk___ZNKR10Nontrivial23ConstLvalueRefQualifiedEv(self) }
    }
}

impl ::ctor::CtorNew<()> for Nontrivial {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(move |dest: *mut Self| {
                crate::detail::__rust_thunk___ZN10NontrivialC1Ev(dest as *mut ::core::ffi::c_void);
            })
        }
    }
}

impl ::ctor::CtorNew<::ffi_11::c_int> for Nontrivial {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ffi_11::c_int) -> Self::CtorType {
        let mut field = args;
        unsafe {
            ::ctor::FnCtor::new(move |dest: *mut Self| {
                crate::detail::__rust_thunk___ZN10NontrivialC1Ei(
                    dest as *mut ::core::ffi::c_void,
                    field,
                );
            })
        }
    }
}
impl ::ctor::CtorNew<(::ffi_11::c_int,)> for Nontrivial {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (::ffi_11::c_int,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ffi_11::c_int>>::ctor_new(arg)
    }
}

impl ::ctor::CtorNew<(::ffi_11::c_int, ::ffi_11::c_int)> for Nontrivial {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (::ffi_11::c_int, ::ffi_11::c_int)) -> Self::CtorType {
        let (mut field, mut unused) = args;
        unsafe {
            ::ctor::FnCtor::new(move |dest: *mut Self| {
                crate::detail::__rust_thunk___ZN10NontrivialC1Eii(
                    dest as *mut ::core::ffi::c_void,
                    field,
                    unused,
                );
            })
        }
    }
}

// Error while generating bindings for constructor 'Nontrivial::Nontrivial':
// Can't generate bindings for Nontrivial::Nontrivial, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for Nontrivial::Nontrivial (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for constructor 'Nontrivial::Nontrivial':
// Can't generate bindings for Nontrivial::Nontrivial, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for Nontrivial::Nontrivial (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'Nontrivial::operator=':
// Can't generate bindings for Nontrivial::operator=, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for Nontrivial::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for Nontrivial::operator= (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'Nontrivial::operator=':
// Can't generate bindings for Nontrivial::operator=, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for Nontrivial::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for Nontrivial::operator= (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'Nontrivial::operator=':
// Can't generate bindings for Nontrivial::operator=, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for Nontrivial::operator= (return type: references are not supported)

impl ::ctor::Assign<f32> for Nontrivial {
    #[inline(always)]
    fn assign<'a>(self: ::core::pin::Pin<&'a mut Self>, __param_0: f32) {
        unsafe {
            let _ = ::ctor::emplace!(::ctor::FnCtor::new(move |dest: *mut Self| {
                crate::detail::__rust_thunk___ZN10NontrivialaSEf(
                    dest as *mut ::core::ffi::c_void,
                    self,
                    __param_0,
                );
            }));
        }
    }
}

impl ::ctor::PinnedDrop for Nontrivial {
    #[inline(always)]
    unsafe fn pinned_drop<'a>(self: ::core::pin::Pin<&'a mut Self>) {
        crate::detail::__rust_thunk___ZN10NontrivialD1Ev(self)
    }
}

// Error while generating bindings for function 'Nontrivial::RvalueRefQualified':
// Can't generate bindings for Nontrivial::RvalueRefQualified, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for Nontrivial::RvalueRefQualified (the type of __this (parameter #0): references are not supported)

// Error while generating bindings for function 'Nontrivial::ConstRvalueRefQualified':
// Can't generate bindings for Nontrivial::ConstRvalueRefQualified, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for Nontrivial::ConstRvalueRefQualified (the type of __this (parameter #0): references are not supported)

// Error while generating bindings for function 'Nontrivial::operator==':
// Can't generate bindings for Nontrivial::operator==, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for Nontrivial::operator== (the type of rhs (parameter #1): references are not supported)

// Error while generating bindings for function 'Nontrivial::operator!=':
// operator== is present, skipping bindings for operator!=

// Error while generating bindings for function 'Nontrivial::operator<':
// Can't generate bindings for Nontrivial::operator<, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for Nontrivial::operator< (the type of rhs (parameter #1): references are not supported)

// Error while generating bindings for function 'Nontrivial::operator+':
// Can't generate bindings for Nontrivial::operator+, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for Nontrivial::operator+ (the type of rhs (parameter #1): references are not supported)

// Error while generating bindings for function 'Nontrivial::operator+=':
// Compound assignment operators are not supported for non-Unpin types, found ::core::pin::Pin<&'a mut crate::Nontrivial>

/// Nontrivial due to (inline) user-specified constructor and destructor.
///
/// This makes it nontrivial for calls (so not trivially relocatable), as well
/// as specifically giving it a nontrivial move constructor and destructor.
#[::ctor::recursively_pinned(PinnedDrop)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=NontrivialInline
pub struct NontrivialInline {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    pub field: ::ffi_11::c_int,
}
impl !Send for NontrivialInline {}
impl !Sync for NontrivialInline {}
unsafe impl ::cxx::ExternType for NontrivialInline {
    type Id = ::cxx::type_id!("NontrivialInline");
    type Kind = ::cxx::kind::Opaque;
}
impl NontrivialInline {
    #[inline(always)]
    pub fn MemberFunction<'a>(self: ::core::pin::Pin<&'a mut Self>) {
        unsafe { crate::detail::__rust_thunk___ZN16NontrivialInline14MemberFunctionEv(self) }
    }
}

impl ::ctor::CtorNew<()> for NontrivialInline {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(move |dest: *mut Self| {
                crate::detail::__rust_thunk___ZN16NontrivialInlineC1Ev(
                    dest as *mut ::core::ffi::c_void,
                );
            })
        }
    }
}

impl ::ctor::CtorNew<::ffi_11::c_int> for NontrivialInline {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ffi_11::c_int) -> Self::CtorType {
        let mut field = args;
        unsafe {
            ::ctor::FnCtor::new(move |dest: *mut Self| {
                crate::detail::__rust_thunk___ZN16NontrivialInlineC1Ei(
                    dest as *mut ::core::ffi::c_void,
                    field,
                );
            })
        }
    }
}
impl ::ctor::CtorNew<(::ffi_11::c_int,)> for NontrivialInline {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (::ffi_11::c_int,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ffi_11::c_int>>::ctor_new(arg)
    }
}

impl ::ctor::CtorNew<(::ffi_11::c_int, ::ffi_11::c_int)> for NontrivialInline {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (::ffi_11::c_int, ::ffi_11::c_int)) -> Self::CtorType {
        let (mut field, mut unused) = args;
        unsafe {
            ::ctor::FnCtor::new(move |dest: *mut Self| {
                crate::detail::__rust_thunk___ZN16NontrivialInlineC1Eii(
                    dest as *mut ::core::ffi::c_void,
                    field,
                    unused,
                );
            })
        }
    }
}

// Error while generating bindings for constructor 'NontrivialInline::NontrivialInline':
// Can't generate bindings for NontrivialInline::NontrivialInline, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for NontrivialInline::NontrivialInline (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for constructor 'NontrivialInline::NontrivialInline':
// Can't generate bindings for NontrivialInline::NontrivialInline, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for NontrivialInline::NontrivialInline (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'NontrivialInline::operator=':
// Can't generate bindings for NontrivialInline::operator=, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for NontrivialInline::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for NontrivialInline::operator= (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'NontrivialInline::operator=':
// Can't generate bindings for NontrivialInline::operator=, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for NontrivialInline::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for NontrivialInline::operator= (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'NontrivialInline::operator=':
// Can't generate bindings for NontrivialInline::operator=, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for NontrivialInline::operator= (return type: references are not supported)

impl ::ctor::PinnedDrop for NontrivialInline {
    #[inline(always)]
    unsafe fn pinned_drop<'a>(self: ::core::pin::Pin<&'a mut Self>) {
        crate::detail::__rust_thunk___ZN16NontrivialInlineD1Ev(self)
    }
}

/// Nontrivial due to member variables.
///
/// This changes how the destructor / drop impl work -- instead of calling
/// the destructor for NontrivialMembers, it just calls the destructors for
/// each field.
#[::ctor::recursively_pinned(PinnedDrop)]
#[repr(C, align(4))]
///CRUBIT_ANNOTATE: cpp_type=NontrivialMembers
pub struct NontrivialMembers {
    /// Reason for representing this field as a blob of bytes:
    /// nontrivial fields would be destroyed in the wrong order
    pub(crate) nontrivial_member: [::core::mem::MaybeUninit<u8>; 4],
}
impl !Send for NontrivialMembers {}
impl !Sync for NontrivialMembers {}
unsafe impl ::cxx::ExternType for NontrivialMembers {
    type Id = ::cxx::type_id!("NontrivialMembers");
    type Kind = ::cxx::kind::Opaque;
}

impl ::ctor::CtorNew<()> for NontrivialMembers {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(move |dest: *mut Self| {
                crate::detail::__rust_thunk___ZN17NontrivialMembersC1Ev(
                    dest as *mut ::core::ffi::c_void,
                );
            })
        }
    }
}

// Error while generating bindings for constructor 'NontrivialMembers::NontrivialMembers':
// Can't generate bindings for NontrivialMembers::NontrivialMembers, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for NontrivialMembers::NontrivialMembers (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for constructor 'NontrivialMembers::NontrivialMembers':
// Can't generate bindings for NontrivialMembers::NontrivialMembers, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for NontrivialMembers::NontrivialMembers (the type of __param_0 (parameter #1): references are not supported)

impl ::ctor::PinnedDrop for NontrivialMembers {
    #[inline(always)]
    unsafe fn pinned_drop<'a>(self: ::core::pin::Pin<&'a mut Self>) {
        crate::detail::__rust_thunk___ZN17NontrivialMembersD1Ev(self)
    }
}

// Error while generating bindings for function 'NontrivialMembers::operator=':
// Can't generate bindings for NontrivialMembers::operator=, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for NontrivialMembers::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for NontrivialMembers::operator= (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'NontrivialMembers::operator=':
// Can't generate bindings for NontrivialMembers::operator=, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for NontrivialMembers::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for NontrivialMembers::operator= (the type of __param_0 (parameter #1): references are not supported)

/// Nontrivial, but trivially relocatable and final (and therefore Unpin).
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=NontrivialUnpin
pub struct NontrivialUnpin {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    pub field: ::ffi_11::c_int,
}
impl !Send for NontrivialUnpin {}
impl !Sync for NontrivialUnpin {}
unsafe impl ::cxx::ExternType for NontrivialUnpin {
    type Id = ::cxx::type_id!("NontrivialUnpin");
    type Kind = ::cxx::kind::Trivial;
}
impl NontrivialUnpin {
    #[inline(always)]
    pub fn MemberFunction<'a>(&'a mut self) {
        unsafe { crate::detail::__rust_thunk___ZN15NontrivialUnpin14MemberFunctionEv(self) }
    }
}

impl Default for NontrivialUnpin {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN15NontrivialUnpinC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

impl From<::ffi_11::c_int> for NontrivialUnpin {
    #[inline(always)]
    fn from(field: ::ffi_11::c_int) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN15NontrivialUnpinC1Ei(&raw mut tmp as *mut _, field);
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<::ffi_11::c_int> for NontrivialUnpin {
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ffi_11::c_int) -> Self::CtorType {
        <Self as From<::ffi_11::c_int>>::from(args)
    }
}

// Error while generating bindings for constructor 'NontrivialUnpin::NontrivialUnpin':
// Constructors with more than one parameter are not yet supported. See b/216648347.

// Error while generating bindings for constructor 'NontrivialUnpin::NontrivialUnpin':
// Can't generate bindings for NontrivialUnpin::NontrivialUnpin, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for NontrivialUnpin::NontrivialUnpin (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for constructor 'NontrivialUnpin::NontrivialUnpin':
// Can't generate bindings for NontrivialUnpin::NontrivialUnpin, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for NontrivialUnpin::NontrivialUnpin (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for constructor 'NontrivialUnpin::NontrivialUnpin':
// Can't generate bindings for NontrivialUnpin::NontrivialUnpin, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for NontrivialUnpin::NontrivialUnpin (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'NontrivialUnpin::operator=':
// Can't generate bindings for NontrivialUnpin::operator=, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for NontrivialUnpin::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for NontrivialUnpin::operator= (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'NontrivialUnpin::operator=':
// Can't generate bindings for NontrivialUnpin::operator=, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for NontrivialUnpin::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for NontrivialUnpin::operator= (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'NontrivialUnpin::operator=':
// Can't generate bindings for NontrivialUnpin::operator=, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for NontrivialUnpin::operator= (return type: references are not supported)

impl Drop for NontrivialUnpin {
    #[inline(always)]
    fn drop<'a>(&'a mut self) {
        unsafe { crate::detail::__rust_thunk___ZN15NontrivialUnpinD1Ev(self) }
    }
}

#[inline(always)]
pub fn TakesByValue(
    nontrivial: impl ::ctor::Ctor<Output = crate::Nontrivial, Error = ::ctor::Infallible>,
) -> impl ::ctor::Ctor<Output = crate::Nontrivial, Error = ::ctor::Infallible> {
    unsafe {
        ::ctor::FnCtor::new(move |dest: *mut crate::Nontrivial| {
            crate::detail::__rust_thunk___Z12TakesByValue10Nontrivial(
                dest as *mut ::core::ffi::c_void,
                ::core::pin::Pin::into_inner_unchecked(::ctor::emplace!(nontrivial)),
            );
        })
    }
}

#[inline(always)]
pub fn TakesByValueInline(
    nontrivial: impl ::ctor::Ctor<Output = crate::NontrivialInline, Error = ::ctor::Infallible>,
) -> impl ::ctor::Ctor<Output = crate::NontrivialInline, Error = ::ctor::Infallible> {
    unsafe {
        ::ctor::FnCtor::new(move |dest: *mut crate::NontrivialInline| {
            crate::detail::__rust_thunk___Z18TakesByValueInline16NontrivialInline(
                dest as *mut ::core::ffi::c_void,
                ::core::pin::Pin::into_inner_unchecked(::ctor::emplace!(nontrivial)),
            );
        })
    }
}

#[inline(always)]
pub fn TakesByValueUnpin(mut nontrivial: crate::NontrivialUnpin) -> crate::NontrivialUnpin {
    unsafe {
        let mut __return = ::core::mem::MaybeUninit::<crate::NontrivialUnpin>::uninit();
        crate::detail::__rust_thunk___Z17TakesByValueUnpin15NontrivialUnpin(
            &raw mut __return as *mut ::core::ffi::c_void,
            &mut nontrivial,
        );
        __return.assume_init()
    }
}

// Error while generating bindings for function 'TakesByReference':
// Can't generate bindings for TakesByReference, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for TakesByReference (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for TakesByReference (the type of nontrivial (parameter #0): references are not supported)

// Error while generating bindings for function 'TakesUnpinByReference':
// Can't generate bindings for TakesUnpinByReference, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for TakesUnpinByReference (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for TakesUnpinByReference (the type of nontrivial (parameter #0): references are not supported)

// Error while generating bindings for function 'TakesByConstReference':
// Can't generate bindings for TakesByConstReference, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for TakesByConstReference (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for TakesByConstReference (the type of nontrivial (parameter #0): references are not supported)

// Error while generating bindings for function 'TakesUnpinByConstReference':
// Can't generate bindings for TakesUnpinByConstReference, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for TakesUnpinByConstReference (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for TakesUnpinByConstReference (the type of nontrivial (parameter #0): references are not supported)

// Error while generating bindings for function 'TakesByRvalueReference':
// Can't generate bindings for TakesByRvalueReference, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for TakesByRvalueReference (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for TakesByRvalueReference (the type of nontrivial (parameter #0): references are not supported)

// Error while generating bindings for function 'TakesUnpinByRvalueReference':
// Can't generate bindings for TakesUnpinByRvalueReference, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for TakesUnpinByRvalueReference (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for TakesUnpinByRvalueReference (the type of nontrivial (parameter #0): references are not supported)

// Error while generating bindings for function 'TakesByConstRvalueReference':
// Can't generate bindings for TakesByConstRvalueReference, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for TakesByConstRvalueReference (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for TakesByConstRvalueReference (the type of nontrivial (parameter #0): references are not supported)

// Error while generating bindings for function 'TakesUnpinByConstRvalueReference':
// Can't generate bindings for TakesUnpinByConstRvalueReference, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for TakesUnpinByConstRvalueReference (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for TakesUnpinByConstRvalueReference (the type of nontrivial (parameter #0): references are not supported)

/// Finally, testing for strange by-value APIs.
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=NontrivialByValue
pub struct NontrivialByValue {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for NontrivialByValue {}
impl !Sync for NontrivialByValue {}
unsafe impl ::cxx::ExternType for NontrivialByValue {
    type Id = ::cxx::type_id!("NontrivialByValue");
    type Kind = ::cxx::kind::Trivial;
}

// Error while generating bindings for constructor 'NontrivialByValue::NontrivialByValue':
// Can't generate bindings for NontrivialByValue::NontrivialByValue, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for NontrivialByValue::NontrivialByValue (the type of other (parameter #1): references are not supported)

// Error while generating bindings for constructor 'NontrivialByValue::NontrivialByValue':
// Can't generate bindings for NontrivialByValue::NontrivialByValue, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for NontrivialByValue::NontrivialByValue (the type of other (parameter #1): references are not supported)

// Error while generating bindings for function 'NontrivialByValue::operator=':
// Can't generate bindings for NontrivialByValue::operator=, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for NontrivialByValue::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for NontrivialByValue::operator= (the type of other (parameter #1): references are not supported)

// Error while generating bindings for function 'NontrivialByValue::operator=':
// Can't generate bindings for NontrivialByValue::operator=, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for NontrivialByValue::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for NontrivialByValue::operator= (the type of other (parameter #1): references are not supported)

impl<'other> ::ctor::UnpinAssign<::ctor::RvalueReference<'other, crate::Nontrivial>>
    for NontrivialByValue
{
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, other: ::ctor::RvalueReference<'other, crate::Nontrivial>) {
        unsafe {
            let mut __return = ::core::mem::MaybeUninit::<Self>::uninit();
            crate::detail::__rust_thunk___ZN17NontrivialByValueaSE10Nontrivial(
                &raw mut __return as *mut ::core::ffi::c_void,
                self,
                other,
            );
            __return.assume_init();
        }
    }
}

#[diagnostic::on_unimplemented(
    message = "binding generation for function failed\nExpected first operator== param reference to be immutable, but found mutable reference: &'a mut crate::NontrivialByValue\ncomparison operator return type must be `bool`, found: crate::NontrivialByValue"
)]
pub trait BindingFailedFor_ZN17NontrivialByValueeqES_ {}
impl<'error> PartialEq for NontrivialByValue
where
    &'error (): BindingFailedFor_ZN17NontrivialByValueeqES_,
{
    #[inline(always)]
    fn eq<'a>(&'a self, other: &Self) -> bool {
        #![allow(unused_variables)]
        unreachable!(
            "This impl can never be instantiated. \
                    If this message appears at runtime, please report a crubit.rs-bug."
        )
    }
}

#[::ctor::recursively_pinned(PinnedDrop)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=Nonmovable
pub struct Nonmovable {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for Nonmovable {}
impl !Sync for Nonmovable {}
unsafe impl ::cxx::ExternType for Nonmovable {
    type Id = ::cxx::type_id!("Nonmovable");
    type Kind = ::cxx::kind::Opaque;
}
impl Nonmovable {
    #[inline(always)]
    pub fn MemberFunction<'a>(self: ::core::pin::Pin<&'a mut Self>) {
        unsafe { crate::detail::__rust_thunk___ZN10Nonmovable14MemberFunctionEv(self) }
    }
}

impl ::ctor::CtorNew<()> for Nonmovable {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(move |dest: *mut Self| {
                crate::detail::__rust_thunk___ZN10NonmovableC1Ev(dest as *mut ::core::ffi::c_void);
            })
        }
    }
}

impl ::ctor::PinnedDrop for Nonmovable {
    #[inline(always)]
    unsafe fn pinned_drop<'a>(self: ::core::pin::Pin<&'a mut Self>) {
        crate::detail::__rust_thunk___ZN10NonmovableD1Ev(self)
    }
}

#[diagnostic::on_unimplemented(
    message = "binding generation for function failed\nNon-movable, non-trivial_abi type 'crate::Nonmovable' is not supported by value as parameter #0"
)]
pub trait BindingFailedFor_Z22TakesNonmovableByValue10Nonmovable {}
#[inline(always)]
pub fn TakesNonmovableByValue<'error>(
    nonmovable: impl ::ctor::Ctor<Output = crate::Nonmovable, Error = ::ctor::Infallible>,
) where
    &'error (): BindingFailedFor_Z22TakesNonmovableByValue10Nonmovable,
{
    #![allow(unused_variables)]
    unreachable!(
        "This impl can never be instantiated. \
                    If this message appears at runtime, please report a crubit.rs-bug."
    )
}

#[inline(always)]
pub fn ReturnsNonmovableByValue(
) -> impl ::ctor::Ctor<Output = crate::Nonmovable, Error = ::ctor::Infallible> {
    unsafe {
        ::ctor::FnCtor::new(move |dest: *mut crate::Nonmovable| {
            crate::detail::__rust_thunk___Z24ReturnsNonmovableByValuev(
                dest as *mut ::core::ffi::c_void,
            );
        })
    }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        #[link_name = "_ZN10NontrivialC1Ev"]
        pub(crate) unsafe fn __rust_thunk___ZN10NontrivialC1Ev(__this: *mut ::core::ffi::c_void);
        #[link_name = "_ZN10NontrivialC1Ei"]
        pub(crate) unsafe fn __rust_thunk___ZN10NontrivialC1Ei(
            __this: *mut ::core::ffi::c_void,
            field: ::ffi_11::c_int,
        );
        #[link_name = "_ZN10NontrivialC1Eii"]
        pub(crate) unsafe fn __rust_thunk___ZN10NontrivialC1Eii(
            __this: *mut ::core::ffi::c_void,
            field: ::ffi_11::c_int,
            unused: ::ffi_11::c_int,
        );
        pub(crate) unsafe fn __rust_thunk___ZN10NontrivialaSEf<'a>(
            __return: *mut ::core::ffi::c_void,
            __this: ::core::pin::Pin<&'a mut crate::Nontrivial>,
            __param_0: f32,
        );
        #[link_name = "_ZN10NontrivialD1Ev"]
        pub(crate) unsafe fn __rust_thunk___ZN10NontrivialD1Ev<'a>(
            __this: ::core::pin::Pin<&'a mut crate::Nontrivial>,
        );
        #[link_name = "_ZN10Nontrivial11UnqualifiedEv"]
        pub(crate) unsafe fn __rust_thunk___ZN10Nontrivial11UnqualifiedEv<'a>(
            __this: ::core::pin::Pin<&'a mut crate::Nontrivial>,
        );
        #[link_name = "_ZNK10Nontrivial14ConstQualifiedEv"]
        pub(crate) unsafe fn __rust_thunk___ZNK10Nontrivial14ConstQualifiedEv<'a>(
            __this: &'a crate::Nontrivial,
        );
        #[link_name = "_ZNR10Nontrivial18LvalueRefQualifiedEv"]
        pub(crate) unsafe fn __rust_thunk___ZNR10Nontrivial18LvalueRefQualifiedEv<'a>(
            __this: ::core::pin::Pin<&'a mut crate::Nontrivial>,
        );
        #[link_name = "_ZNKR10Nontrivial23ConstLvalueRefQualifiedEv"]
        pub(crate) unsafe fn __rust_thunk___ZNKR10Nontrivial23ConstLvalueRefQualifiedEv<'a>(
            __this: &'a crate::Nontrivial,
        );
        pub(crate) unsafe fn __rust_thunk___ZN16NontrivialInlineC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN16NontrivialInlineC1Ei(
            __this: *mut ::core::ffi::c_void,
            field: ::ffi_11::c_int,
        );
        pub(crate) unsafe fn __rust_thunk___ZN16NontrivialInlineC1Eii(
            __this: *mut ::core::ffi::c_void,
            field: ::ffi_11::c_int,
            unused: ::ffi_11::c_int,
        );
        pub(crate) unsafe fn __rust_thunk___ZN16NontrivialInlineD1Ev<'a>(
            __this: ::core::pin::Pin<&'a mut crate::NontrivialInline>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN16NontrivialInline14MemberFunctionEv<'a>(
            __this: ::core::pin::Pin<&'a mut crate::NontrivialInline>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN17NontrivialMembersC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN17NontrivialMembersD1Ev<'a>(
            __this: ::core::pin::Pin<&'a mut crate::NontrivialMembers>,
        );
        #[link_name = "_ZN15NontrivialUnpinC1Ev"]
        pub(crate) unsafe fn __rust_thunk___ZN15NontrivialUnpinC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        #[link_name = "_ZN15NontrivialUnpinC1Ei"]
        pub(crate) unsafe fn __rust_thunk___ZN15NontrivialUnpinC1Ei(
            __this: *mut ::core::ffi::c_void,
            field: ::ffi_11::c_int,
        );
        #[link_name = "_ZN15NontrivialUnpinD1Ev"]
        pub(crate) unsafe fn __rust_thunk___ZN15NontrivialUnpinD1Ev<'a>(
            __this: &'a mut crate::NontrivialUnpin,
        );
        #[link_name = "_ZN15NontrivialUnpin14MemberFunctionEv"]
        pub(crate) unsafe fn __rust_thunk___ZN15NontrivialUnpin14MemberFunctionEv<'a>(
            __this: &'a mut crate::NontrivialUnpin,
        );
        pub(crate) unsafe fn __rust_thunk___Z12TakesByValue10Nontrivial(
            __return: *mut ::core::ffi::c_void,
            nontrivial: &mut crate::Nontrivial,
        );
        pub(crate) unsafe fn __rust_thunk___Z18TakesByValueInline16NontrivialInline(
            __return: *mut ::core::ffi::c_void,
            nontrivial: &mut crate::NontrivialInline,
        );
        pub(crate) unsafe fn __rust_thunk___Z17TakesByValueUnpin15NontrivialUnpin(
            __return: *mut ::core::ffi::c_void,
            nontrivial: &mut crate::NontrivialUnpin,
        );
        pub(crate) unsafe fn __rust_thunk___ZN17NontrivialByValueaSE10Nontrivial<'a, 'other>(
            __return: *mut ::core::ffi::c_void,
            __this: &'a mut crate::NontrivialByValue,
            other: ::ctor::RvalueReference<'other, crate::Nontrivial>,
        );
        #[link_name = "_ZN10NonmovableC1Ev"]
        pub(crate) unsafe fn __rust_thunk___ZN10NonmovableC1Ev(__this: *mut ::core::ffi::c_void);
        #[link_name = "_ZN10NonmovableD1Ev"]
        pub(crate) unsafe fn __rust_thunk___ZN10NonmovableD1Ev<'a>(
            __this: ::core::pin::Pin<&'a mut crate::Nonmovable>,
        );
        #[link_name = "_ZN10Nonmovable14MemberFunctionEv"]
        pub(crate) unsafe fn __rust_thunk___ZN10Nonmovable14MemberFunctionEv<'a>(
            __this: ::core::pin::Pin<&'a mut crate::Nonmovable>,
        );
        pub(crate) unsafe fn __rust_thunk___Z24ReturnsNonmovableByValuev(
            __return: *mut ::core::ffi::c_void,
        );
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::Nontrivial>() == 4);
    assert!(::core::mem::align_of::<crate::Nontrivial>() == 4);
    static_assertions::assert_impl_all!(crate::Nontrivial: Drop);
    static_assertions::assert_not_impl_any!(crate::Nontrivial: Copy);
    assert!(::core::mem::offset_of!(crate::Nontrivial, field) == 0);
    static_assertions::assert_impl_all!(::ffi_11::c_int: Copy);
    assert!(::core::mem::size_of::<crate::NontrivialInline>() == 4);
    assert!(::core::mem::align_of::<crate::NontrivialInline>() == 4);
    static_assertions::assert_impl_all!(crate::NontrivialInline: Drop);
    static_assertions::assert_not_impl_any!(crate::NontrivialInline: Copy);
    assert!(::core::mem::offset_of!(crate::NontrivialInline, field) == 0);
    static_assertions::assert_impl_all!(::ffi_11::c_int: Copy);
    assert!(::core::mem::size_of::<crate::NontrivialMembers>() == 4);
    assert!(::core::mem::align_of::<crate::NontrivialMembers>() == 4);
    static_assertions::assert_impl_all!(crate::NontrivialMembers: Drop);
    static_assertions::assert_not_impl_any!(crate::NontrivialMembers: Copy);
    assert!(::core::mem::offset_of!(crate::NontrivialMembers, nontrivial_member) == 0);
    assert!(::core::mem::size_of::<crate::NontrivialUnpin>() == 4);
    assert!(::core::mem::align_of::<crate::NontrivialUnpin>() == 4);
    static_assertions::assert_impl_all!(crate::NontrivialUnpin: Drop);
    static_assertions::assert_not_impl_any!(crate::NontrivialUnpin: Copy);
    assert!(::core::mem::offset_of!(crate::NontrivialUnpin, field) == 0);
    static_assertions::assert_impl_all!(::ffi_11::c_int: Copy);
    assert!(::core::mem::size_of::<crate::NontrivialByValue>() == 1);
    assert!(::core::mem::align_of::<crate::NontrivialByValue>() == 1);
    static_assertions::assert_impl_all!(crate::NontrivialByValue: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::NontrivialByValue: Drop);

    assert!(::core::mem::size_of::<crate::Nonmovable>() == 1);
    assert!(::core::mem::align_of::<crate::Nonmovable>() == 1);
    static_assertions::assert_impl_all!(crate::Nonmovable: Drop);
    static_assertions::assert_not_impl_any!(crate::Nonmovable: Copy);
};
