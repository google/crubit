// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:no_unique_address_cc

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

/// The no_unique_address.h header is present both in
/// rs_bindings_from_cc/test/struct/no_unique_address/ and in
/// rs_bindings_from_cc/test/golden/ because the format provides end-to-end
/// coverage for working accessor functions, while the latter helps manually
/// inspect and verify the expected layout of the generated Rust struct.
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C, align(4))]
///CRUBIT_ANNOTATE: cpp_type=Struct
pub struct Struct {
    /// Nobody would ever use a no_unique_address int/char field, this is just
    /// enough to test that the transmute is correct.
    ///
    /// Reason for representing this field as a blob of bytes:
    /// `[[no_unique_address]]` attribute was present.
    pub(crate) field1: [::core::mem::MaybeUninit<u8>; 4],
    /// Reason for representing this field as a blob of bytes:
    /// `[[no_unique_address]]` attribute was present.
    pub(crate) field2: [::core::mem::MaybeUninit<u8>; 4],
}
impl !Send for Struct {}
impl !Sync for Struct {}
unsafe impl ::cxx::ExternType for Struct {
    type Id = ::cxx::type_id!("Struct");
    type Kind = ::cxx::kind::Trivial;
}
impl Struct {
    #[inline(always)]
    pub fn Make(f1: ::ffi_11::c_int, f2: ::ffi_11::c_char) -> crate::Struct {
        unsafe {
            let mut __return = ::core::mem::MaybeUninit::<Self>::uninit();
            crate::detail::__rust_thunk___ZN6Struct4MakeEic(
                &raw mut __return as *mut ::core::ffi::c_void,
                f1,
                f2,
            );
            __return.assume_init()
        }
    }
}

impl Default for Struct {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN6StructC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

// Error while generating bindings for constructor 'Struct::Struct':
// Can't generate bindings for Struct::Struct, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:no_unique_address_cc needs [//features:experimental] for Struct::Struct (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for constructor 'Struct::Struct':
// Can't generate bindings for Struct::Struct, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:no_unique_address_cc needs [//features:experimental] for Struct::Struct (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'Struct::operator=':
// Can't generate bindings for Struct::operator=, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:no_unique_address_cc needs [//features:experimental] for Struct::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:no_unique_address_cc needs [//features:experimental] for Struct::operator= (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'Struct::operator=':
// Can't generate bindings for Struct::operator=, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:no_unique_address_cc needs [//features:experimental] for Struct::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:no_unique_address_cc needs [//features:experimental] for Struct::operator= (the type of __param_0 (parameter #1): references are not supported)

/// Regression test for b/232418721.  This tests that the offset of `field2` is
/// correct (given its alignment requirements there need to be 3 bytes of padding
/// between `field1` and `field2`).  The verification is mostly done through
/// compile-time assertions of field offsets in the generated Rust code.  Before
/// cl/448287893 `field2` would be incorrectly placed at offset 1.
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C, align(4))]
///CRUBIT_ANNOTATE: cpp_type=PaddingBetweenFields
pub struct PaddingBetweenFields {
    /// size: 1, alignment: 1 => offset: 0
    pub field1: ::ffi_11::c_char,
    __padding1: [::core::mem::MaybeUninit<u8>; 3],
    /// size: 4, alignment: 4 => offset: 4
    ///
    /// Reason for representing this field as a blob of bytes:
    /// `[[no_unique_address]]` attribute was present.
    pub(crate) field2: [::core::mem::MaybeUninit<u8>; 4],
}
impl !Send for PaddingBetweenFields {}
impl !Sync for PaddingBetweenFields {}
unsafe impl ::cxx::ExternType for PaddingBetweenFields {
    type Id = ::cxx::type_id!("PaddingBetweenFields");
    type Kind = ::cxx::kind::Trivial;
}
impl PaddingBetweenFields {
    #[inline(always)]
    pub fn Make(f1: ::ffi_11::c_char, f2: ::ffi_11::c_int) -> crate::PaddingBetweenFields {
        unsafe {
            let mut __return = ::core::mem::MaybeUninit::<Self>::uninit();
            crate::detail::__rust_thunk___ZN20PaddingBetweenFields4MakeEci(
                &raw mut __return as *mut ::core::ffi::c_void,
                f1,
                f2,
            );
            __return.assume_init()
        }
    }
}

impl Default for PaddingBetweenFields {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN20PaddingBetweenFieldsC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

// Error while generating bindings for constructor 'PaddingBetweenFields::PaddingBetweenFields':
// Can't generate bindings for PaddingBetweenFields::PaddingBetweenFields, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:no_unique_address_cc needs [//features:experimental] for PaddingBetweenFields::PaddingBetweenFields (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for constructor 'PaddingBetweenFields::PaddingBetweenFields':
// Can't generate bindings for PaddingBetweenFields::PaddingBetweenFields, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:no_unique_address_cc needs [//features:experimental] for PaddingBetweenFields::PaddingBetweenFields (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'PaddingBetweenFields::operator=':
// Can't generate bindings for PaddingBetweenFields::operator=, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:no_unique_address_cc needs [//features:experimental] for PaddingBetweenFields::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:no_unique_address_cc needs [//features:experimental] for PaddingBetweenFields::operator= (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'PaddingBetweenFields::operator=':
// Can't generate bindings for PaddingBetweenFields::operator=, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:no_unique_address_cc needs [//features:experimental] for PaddingBetweenFields::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:no_unique_address_cc needs [//features:experimental] for PaddingBetweenFields::operator= (the type of __param_0 (parameter #1): references are not supported)

/// Layout properties of FieldInTailPadding_InnerStruct look as follows:
/// - alignment: 4 (because of `inner_int_field`)
/// - dsize (size without padding): 5
///   (4 bytes for `inner_int_field`, 1 byte for `inner_char_field`)
/// - size: 8 (dsize adjusted up to account for alignment)
#[::ctor::recursively_pinned(PinnedDrop)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=FieldInTailPadding_InnerStruct
pub struct FieldInTailPadding_InnerStruct {
    /// size: 4, alignment: 4 => offset: 0
    pub inner_int_field: ::ffi_11::c_int,
    /// size: 1, alignment: 1 => offset: 4
    pub inner_char_field: ::ffi_11::c_char,
}
impl !Send for FieldInTailPadding_InnerStruct {}
impl !Sync for FieldInTailPadding_InnerStruct {}
unsafe impl ::cxx::ExternType for FieldInTailPadding_InnerStruct {
    type Id = ::cxx::type_id!("FieldInTailPadding_InnerStruct");
    type Kind = ::cxx::kind::Opaque;
}

impl ::ctor::CtorNew<()> for FieldInTailPadding_InnerStruct {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(move |dest: *mut Self| {
                crate::detail::__rust_thunk___ZN30FieldInTailPadding_InnerStructC1Ev(
                    dest as *mut ::core::ffi::c_void,
                );
            })
        }
    }
}

// Error while generating bindings for constructor 'FieldInTailPadding_InnerStruct::FieldInTailPadding_InnerStruct':
// Can't generate bindings for FieldInTailPadding_InnerStruct::FieldInTailPadding_InnerStruct, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:no_unique_address_cc needs [//features:experimental] for FieldInTailPadding_InnerStruct::FieldInTailPadding_InnerStruct (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'FieldInTailPadding_InnerStruct::operator=':
// Can't generate bindings for FieldInTailPadding_InnerStruct::operator=, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:no_unique_address_cc needs [//features:experimental] for FieldInTailPadding_InnerStruct::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:no_unique_address_cc needs [//features:experimental] for FieldInTailPadding_InnerStruct::operator= (the type of __param_0 (parameter #1): references are not supported)

/// User-defined destructor to make this struct non-POD for the purposes of
/// layout.
impl ::ctor::PinnedDrop for FieldInTailPadding_InnerStruct {
    #[inline(always)]
    unsafe fn pinned_drop<'a>(self: ::core::pin::Pin<&'a mut Self>) {
        crate::detail::__rust_thunk___ZN30FieldInTailPadding_InnerStructD1Ev(self)
    }
}

/// Regression test against b/232418721#comment7.  This tests that the offset of
/// `char_in_tail_padding_of_prev_field`` is correct - because of
/// `no_unique_address` this field should be laid out inside the tail padding of
/// `inner_struct` (i.e. offset of `char_in_tail_padding_of_prev_field`` should
/// be 5 = dsize of `s` rather than 8 = size of `s`).  The verification is mostly
/// done through compile-time assertions of field offsets in the generated Rust
/// code.  The initial alignment-based fix idea for b/232418721 would incorrectly
/// put `char_in_tail_padding_of_prev_field` at offset 8.
#[::ctor::recursively_pinned(PinnedDrop)]
#[repr(C, align(4))]
///CRUBIT_ANNOTATE: cpp_type=FieldInTailPadding
pub struct FieldInTailPadding {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// `[[no_unique_address]]` attribute was present.
    pub(crate) inner_struct: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 5],
    /// offset: 5 (dsize of `s`).
    pub char_in_tail_padding_of_prev_field: ::ffi_11::c_char,
}
impl !Send for FieldInTailPadding {}
impl !Sync for FieldInTailPadding {}
unsafe impl ::cxx::ExternType for FieldInTailPadding {
    type Id = ::cxx::type_id!("FieldInTailPadding");
    type Kind = ::cxx::kind::Opaque;
}

// Error while generating bindings for constructor 'FieldInTailPadding::FieldInTailPadding':
// Can't generate bindings for FieldInTailPadding::FieldInTailPadding, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:no_unique_address_cc needs [//features:experimental] for FieldInTailPadding::FieldInTailPadding (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for constructor 'FieldInTailPadding::FieldInTailPadding':
// Can't generate bindings for FieldInTailPadding::FieldInTailPadding, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:no_unique_address_cc needs [//features:experimental] for FieldInTailPadding::FieldInTailPadding (the type of __param_0 (parameter #1): references are not supported)

impl ::ctor::PinnedDrop for FieldInTailPadding {
    #[inline(always)]
    unsafe fn pinned_drop<'a>(self: ::core::pin::Pin<&'a mut Self>) {
        crate::detail::__rust_thunk___ZN18FieldInTailPaddingD1Ev(self)
    }
}

// Error while generating bindings for function 'FieldInTailPadding::operator=':
// Can't generate bindings for FieldInTailPadding::operator=, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:no_unique_address_cc needs [//features:experimental] for FieldInTailPadding::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:no_unique_address_cc needs [//features:experimental] for FieldInTailPadding::operator= (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'FieldInTailPadding::operator=':
// Can't generate bindings for FieldInTailPadding::operator=, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:no_unique_address_cc needs [//features:experimental] for FieldInTailPadding::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:no_unique_address_cc needs [//features:experimental] for FieldInTailPadding::operator= (the type of __param_0 (parameter #1): references are not supported)

impl ::ctor::CtorNew<(::ffi_11::c_int, ::ffi_11::c_char, ::ffi_11::c_char)> for FieldInTailPadding {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (::ffi_11::c_int, ::ffi_11::c_char, ::ffi_11::c_char)) -> Self::CtorType {
        let (mut inner_int, mut inner_char, mut outer_char) = args;
        unsafe {
            ::ctor::FnCtor::new(move |dest: *mut Self| {
                crate::detail::__rust_thunk___ZN18FieldInTailPaddingC1Eicc(
                    dest as *mut ::core::ffi::c_void,
                    inner_int,
                    inner_char,
                    outer_char,
                );
            })
        }
    }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN6StructC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN6Struct4MakeEic(
            __return: *mut ::core::ffi::c_void,
            f1: ::ffi_11::c_int,
            f2: ::ffi_11::c_char,
        );
        pub(crate) unsafe fn __rust_thunk___ZN20PaddingBetweenFieldsC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN20PaddingBetweenFields4MakeEci(
            __return: *mut ::core::ffi::c_void,
            f1: ::ffi_11::c_char,
            f2: ::ffi_11::c_int,
        );
        pub(crate) unsafe fn __rust_thunk___ZN30FieldInTailPadding_InnerStructC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN30FieldInTailPadding_InnerStructD1Ev<'a>(
            __this: ::core::pin::Pin<&'a mut crate::FieldInTailPadding_InnerStruct>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN18FieldInTailPaddingD1Ev<'a>(
            __this: ::core::pin::Pin<&'a mut crate::FieldInTailPadding>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN18FieldInTailPaddingC1Eicc(
            __this: *mut ::core::ffi::c_void,
            inner_int: ::ffi_11::c_int,
            inner_char: ::ffi_11::c_char,
            outer_char: ::ffi_11::c_char,
        );
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::Struct>() == 8);
    assert!(::core::mem::align_of::<crate::Struct>() == 4);
    static_assertions::assert_impl_all!(crate::Struct: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::Struct: Drop);
    assert!(::core::mem::offset_of!(crate::Struct, field1) == 0);
    assert!(::core::mem::offset_of!(crate::Struct, field2) == 4);
    assert!(::core::mem::size_of::<crate::PaddingBetweenFields>() == 8);
    assert!(::core::mem::align_of::<crate::PaddingBetweenFields>() == 4);
    static_assertions::assert_impl_all!(crate::PaddingBetweenFields: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::PaddingBetweenFields: Drop);
    assert!(::core::mem::offset_of!(crate::PaddingBetweenFields, field1) == 0);
    assert!(::core::mem::offset_of!(crate::PaddingBetweenFields, field2) == 4);
    assert!(::core::mem::size_of::<crate::FieldInTailPadding_InnerStruct>() == 8);
    assert!(::core::mem::align_of::<crate::FieldInTailPadding_InnerStruct>() == 4);
    static_assertions::assert_impl_all!(crate::FieldInTailPadding_InnerStruct: Drop);
    static_assertions::assert_not_impl_any!(crate::FieldInTailPadding_InnerStruct: Copy);
    assert!(::core::mem::offset_of!(crate::FieldInTailPadding_InnerStruct, inner_int_field) == 0);
    assert!(::core::mem::offset_of!(crate::FieldInTailPadding_InnerStruct, inner_char_field) == 4);
    static_assertions::assert_impl_all!(::ffi_11::c_int: Copy);
    static_assertions::assert_impl_all!(::ffi_11::c_char: Copy);
    assert!(::core::mem::size_of::<crate::FieldInTailPadding>() == 8);
    assert!(::core::mem::align_of::<crate::FieldInTailPadding>() == 4);
    static_assertions::assert_impl_all!(crate::FieldInTailPadding: Drop);
    static_assertions::assert_not_impl_any!(crate::FieldInTailPadding: Copy);
    assert!(::core::mem::offset_of!(crate::FieldInTailPadding, inner_struct) == 0);
    assert!(
        ::core::mem::offset_of!(crate::FieldInTailPadding, char_in_tail_padding_of_prev_field) == 5
    );
    static_assertions::assert_impl_all!(::ffi_11::c_char: Copy);
};
