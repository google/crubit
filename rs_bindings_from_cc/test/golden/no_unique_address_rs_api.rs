// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for C++ target
// //rs_bindings_from_cc/test/golden:no_unique_address_cc
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

/// The no_unique_address.h header is present both in
/// rs_bindings_from_cc/test/struct/no_unique_address/ and in
/// rs_bindings_from_cc/test/golden/ because the format provides end-to-end
/// coverage for working accessor functions, while the latter helps manually
/// inspect and verify the expected layout of the generated Rust struct.
#[derive(Clone, Copy)]
#[repr(C, align(4))]
pub struct Struct {
    /// Nobody would ever use a no_unique_address int/char field, this is just
    /// enough to test that the transmute is correct.
    field1: [crate::rust_std::mem::MaybeUninit<u8>; 4],
    field2: [crate::rust_std::mem::MaybeUninit<u8>; 4],
}
forward_declare::unsafe_define!(forward_declare::symbol!("Struct"), crate::Struct);
impl Struct {
    pub fn field1(&self) -> &i32 {
        unsafe { &*(&self.field1 as *const _ as *const i32) }
    }
    pub fn field2(&self) -> &u8 {
        unsafe { &*(&self.field2 as *const _ as *const u8) }
    }
}

impl Default for Struct {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = crate::rust_std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN6StructC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b> From<ctor::RvalueReference<'b, crate::Struct>> for Struct {
    #[inline(always)]
    fn from(__param_0: ctor::RvalueReference<'b, crate::Struct>) -> Self {
        let mut tmp = crate::rust_std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN6StructC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

// rs_bindings_from_cc/test/golden/no_unique_address.h;l=15
// Error while generating bindings for item 'Struct::operator=':
// Bindings for this kind of operator are not supported

// rs_bindings_from_cc/test/golden/no_unique_address.h;l=15
// Error while generating bindings for item 'Struct::operator=':
// Bindings for this kind of operator are not supported

impl Struct {
    #[inline(always)]
    pub fn Make(f1: i32, f2: u8) -> crate::Struct {
        unsafe { crate::detail::__rust_thunk___ZN6Struct4MakeEic(f1, f2) }
    }
}

/// Regression test for b/232418721.  This tests that the offset of `field2` is
/// correct (given its alignment requirements there need to be 3 bytes of padding
/// between `field1` and `field2`).  The verification is mostly done through
/// compile-time assertions of field offsets in the generated Rust code.  Before
/// cl/448287893 `field2` would be incorrectly placed at offset 1.
#[derive(Clone, Copy)]
#[repr(C, align(4))]
pub struct PaddingBetweenFields {
    /// size: 1, alignment: 1 => offset: 0
    pub field1: u8,
    __padding1: [crate::rust_std::mem::MaybeUninit<u8>; 3],
    /// size: 4, alignment: 4 => offset: 4
    field2: [crate::rust_std::mem::MaybeUninit<u8>; 4],
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("PaddingBetweenFields"),
    crate::PaddingBetweenFields
);
impl PaddingBetweenFields {
    pub fn field2(&self) -> &i32 {
        unsafe { &*(&self.field2 as *const _ as *const i32) }
    }
}

impl Default for PaddingBetweenFields {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = crate::rust_std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN20PaddingBetweenFieldsC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b> From<ctor::RvalueReference<'b, crate::PaddingBetweenFields>> for PaddingBetweenFields {
    #[inline(always)]
    fn from(__param_0: ctor::RvalueReference<'b, crate::PaddingBetweenFields>) -> Self {
        let mut tmp = crate::rust_std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN20PaddingBetweenFieldsC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

// rs_bindings_from_cc/test/golden/no_unique_address.h;l=28
// Error while generating bindings for item 'PaddingBetweenFields::operator=':
// Bindings for this kind of operator are not supported

// rs_bindings_from_cc/test/golden/no_unique_address.h;l=28
// Error while generating bindings for item 'PaddingBetweenFields::operator=':
// Bindings for this kind of operator are not supported

impl PaddingBetweenFields {
    #[inline(always)]
    pub fn Make(f1: u8, f2: i32) -> crate::PaddingBetweenFields {
        unsafe { crate::detail::__rust_thunk___ZN20PaddingBetweenFields4MakeEci(f1, f2) }
    }
}

/// Layout properties of FieldInTailPadding_InnerStruct look as follows:
/// - alignment: 4 (because of `inner_int_field`)
/// - dsize (size without padding): 5
///   (4 bytes for `inner_int_field`, 1 byte for `inner_char_field`)
/// - size: 8 (dsize adjusted up to account for alignment)
#[ctor::recursively_pinned(PinnedDrop)]
#[repr(C)]
pub struct FieldInTailPadding_InnerStruct {
    /// size: 4, alignment: 4 => offset: 0
    pub inner_int_field: i32,
    /// size: 1, alignment: 1 => offset: 4
    pub inner_char_field: u8,
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("FieldInTailPadding_InnerStruct"),
    crate::FieldInTailPadding_InnerStruct
);

impl ctor::CtorNew<()> for FieldInTailPadding_InnerStruct {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        ctor::FnCtor::new(
            move |dest: crate::rust_std::pin::Pin<&mut crate::rust_std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN30FieldInTailPadding_InnerStructC1Ev(
                    crate::rust_std::pin::Pin::into_inner_unchecked(dest),
                );
            },
        )
    }
}

impl<'b> ctor::CtorNew<&'b crate::FieldInTailPadding_InnerStruct>
    for FieldInTailPadding_InnerStruct
{
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: &'b crate::FieldInTailPadding_InnerStruct) -> Self::CtorType {
        let __param_0 = args;
        ctor::FnCtor::new(
            move |dest: crate::rust_std::pin::Pin<&mut crate::rust_std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN30FieldInTailPadding_InnerStructC1ERKS_(
                    crate::rust_std::pin::Pin::into_inner_unchecked(dest),
                    __param_0,
                );
            },
        )
    }
}
impl<'b> ctor::CtorNew<(&'b crate::FieldInTailPadding_InnerStruct,)>
    for FieldInTailPadding_InnerStruct
{
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: (&'b crate::FieldInTailPadding_InnerStruct,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ctor::CtorNew<&'b crate::FieldInTailPadding_InnerStruct>>::ctor_new(arg)
    }
}

// rs_bindings_from_cc/test/golden/no_unique_address.h;l=42
// Error while generating bindings for item 'FieldInTailPadding_InnerStruct::operator=':
// Bindings for this kind of operator are not supported

/// User-defined destructor to make this struct non-POD for the purposes of
/// layout.
impl ::ctor::PinnedDrop for FieldInTailPadding_InnerStruct {
    #[inline(always)]
    unsafe fn pinned_drop<'a>(self: crate::rust_std::pin::Pin<&'a mut Self>) {
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
#[ctor::recursively_pinned(PinnedDrop)]
#[repr(C, align(4))]
pub struct FieldInTailPadding {
    __non_field_data: [crate::rust_std::mem::MaybeUninit<u8>; 0],
    inner_struct: [crate::rust_std::mem::MaybeUninit<u8>; 5],
    /// offset: 5 (dsize of `s`).
    pub char_in_tail_padding_of_prev_field: u8,
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("FieldInTailPadding"),
    crate::FieldInTailPadding
);
impl FieldInTailPadding {
    pub fn inner_struct(&self) -> &crate::FieldInTailPadding_InnerStruct {
        unsafe {
            &*(&self.inner_struct as *const _ as *const crate::FieldInTailPadding_InnerStruct)
        }
    }
}

impl<'b> ctor::CtorNew<&'b crate::FieldInTailPadding> for FieldInTailPadding {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: &'b crate::FieldInTailPadding) -> Self::CtorType {
        let __param_0 = args;
        ctor::FnCtor::new(
            move |dest: crate::rust_std::pin::Pin<&mut crate::rust_std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN18FieldInTailPaddingC1ERKS_(
                    crate::rust_std::pin::Pin::into_inner_unchecked(dest),
                    __param_0,
                );
            },
        )
    }
}
impl<'b> ctor::CtorNew<(&'b crate::FieldInTailPadding,)> for FieldInTailPadding {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: (&'b crate::FieldInTailPadding,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ctor::CtorNew<&'b crate::FieldInTailPadding>>::ctor_new(arg)
    }
}

impl<'b> ctor::CtorNew<ctor::RvalueReference<'b, crate::FieldInTailPadding>>
    for FieldInTailPadding
{
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ctor::RvalueReference<'b, crate::FieldInTailPadding>) -> Self::CtorType {
        let __param_0 = args;
        ctor::FnCtor::new(
            move |dest: crate::rust_std::pin::Pin<&mut crate::rust_std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN18FieldInTailPaddingC1EOS_(
                    crate::rust_std::pin::Pin::into_inner_unchecked(dest),
                    __param_0,
                );
            },
        )
    }
}
impl<'b> ctor::CtorNew<(ctor::RvalueReference<'b, crate::FieldInTailPadding>,)>
    for FieldInTailPadding
{
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: (ctor::RvalueReference<'b, crate::FieldInTailPadding>,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ctor::CtorNew<ctor::RvalueReference<'b, crate::FieldInTailPadding>>>::ctor_new(arg)
    }
}

impl ::ctor::PinnedDrop for FieldInTailPadding {
    #[inline(always)]
    unsafe fn pinned_drop<'a>(self: crate::rust_std::pin::Pin<&'a mut Self>) {
        crate::detail::__rust_thunk___ZN18FieldInTailPaddingD1Ev(self)
    }
}

// rs_bindings_from_cc/test/golden/no_unique_address.h;l=59
// Error while generating bindings for item 'FieldInTailPadding::operator=':
// Bindings for this kind of operator are not supported

// rs_bindings_from_cc/test/golden/no_unique_address.h;l=59
// Error while generating bindings for item 'FieldInTailPadding::operator=':
// Bindings for this kind of operator are not supported

impl ctor::CtorNew<(i32, u8, u8)> for FieldInTailPadding {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: (i32, u8, u8)) -> Self::CtorType {
        let (inner_int, inner_char, outer_char) = args;
        ctor::FnCtor::new(
            move |dest: crate::rust_std::pin::Pin<&mut crate::rust_std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN18FieldInTailPaddingC1Eicc(
                    crate::rust_std::pin::Pin::into_inner_unchecked(dest),
                    inner_int,
                    inner_char,
                    outer_char,
                );
            },
        )
    }
}

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_NO_UNIQUE_ADDRESS_NO_UNIQUE_ADDRESS_H_

mod detail {
    #[allow(unused_imports)]
    use super::*;
    extern "C" {
        pub(crate) fn __rust_thunk___ZN6StructC1Ev<'a>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::Struct>,
        );
        pub(crate) fn __rust_thunk___ZN6StructC1EOS_<'a, 'b>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::Struct>,
            __param_0: ctor::RvalueReference<'b, crate::Struct>,
        );
        pub(crate) fn __rust_thunk___ZN6Struct4MakeEic(f1: i32, f2: u8) -> crate::Struct;
        pub(crate) fn __rust_thunk___ZN20PaddingBetweenFieldsC1Ev<'a>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::PaddingBetweenFields>,
        );
        pub(crate) fn __rust_thunk___ZN20PaddingBetweenFieldsC1EOS_<'a, 'b>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::PaddingBetweenFields>,
            __param_0: ctor::RvalueReference<'b, crate::PaddingBetweenFields>,
        );
        pub(crate) fn __rust_thunk___ZN20PaddingBetweenFields4MakeEci(
            f1: u8,
            f2: i32,
        ) -> crate::PaddingBetweenFields;
        pub(crate) fn __rust_thunk___ZN30FieldInTailPadding_InnerStructC1Ev<'a>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<
                crate::FieldInTailPadding_InnerStruct,
            >,
        );
        pub(crate) fn __rust_thunk___ZN30FieldInTailPadding_InnerStructC1ERKS_<'a, 'b>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<
                crate::FieldInTailPadding_InnerStruct,
            >,
            __param_0: &'b crate::FieldInTailPadding_InnerStruct,
        );
        pub(crate) fn __rust_thunk___ZN30FieldInTailPadding_InnerStructD1Ev<'a>(
            __this: crate::rust_std::pin::Pin<&'a mut crate::FieldInTailPadding_InnerStruct>,
        );
        pub(crate) fn __rust_thunk___ZN18FieldInTailPaddingC1ERKS_<'a, 'b>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::FieldInTailPadding>,
            __param_0: &'b crate::FieldInTailPadding,
        );
        pub(crate) fn __rust_thunk___ZN18FieldInTailPaddingC1EOS_<'a, 'b>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::FieldInTailPadding>,
            __param_0: ctor::RvalueReference<'b, crate::FieldInTailPadding>,
        );
        pub(crate) fn __rust_thunk___ZN18FieldInTailPaddingD1Ev<'a>(
            __this: crate::rust_std::pin::Pin<&'a mut crate::FieldInTailPadding>,
        );
        pub(crate) fn __rust_thunk___ZN18FieldInTailPaddingC1Eicc<'a>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::FieldInTailPadding>,
            inner_int: i32,
            inner_char: u8,
            outer_char: u8,
        );
    }
}

const _: () = assert!(rust_std::mem::size_of::<Option<&i32>>() == rust_std::mem::size_of::<&i32>());

const _: () = assert!(rust_std::mem::size_of::<crate::Struct>() == 8);
const _: () = assert!(rust_std::mem::align_of::<crate::Struct>() == 4);
const _: () = {
    static_assertions::assert_impl_all!(crate::Struct: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::Struct: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_all!(crate::Struct: Drop);
};
const _: () = assert!(memoffset_unstable_const::offset_of!(crate::Struct, field1) * 8 == 0);
const _: () = assert!(memoffset_unstable_const::offset_of!(crate::Struct, field2) * 8 == 32);

const _: () = assert!(rust_std::mem::size_of::<crate::PaddingBetweenFields>() == 8);
const _: () = assert!(rust_std::mem::align_of::<crate::PaddingBetweenFields>() == 4);
const _: () = {
    static_assertions::assert_impl_all!(crate::PaddingBetweenFields: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::PaddingBetweenFields: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_all!(crate::PaddingBetweenFields: Drop);
};
const _: () =
    assert!(memoffset_unstable_const::offset_of!(crate::PaddingBetweenFields, field1) * 8 == 0);
const _: () =
    assert!(memoffset_unstable_const::offset_of!(crate::PaddingBetweenFields, field2) * 8 == 32);

const _: () = assert!(rust_std::mem::size_of::<crate::FieldInTailPadding_InnerStruct>() == 8);
const _: () = assert!(rust_std::mem::align_of::<crate::FieldInTailPadding_InnerStruct>() == 4);
const _: () = {
    static_assertions::assert_not_impl_all!(crate::FieldInTailPadding_InnerStruct: Copy);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::FieldInTailPadding_InnerStruct: Drop);
};
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldInTailPadding_InnerStruct, inner_int_field)
        * 8
        == 0
);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldInTailPadding_InnerStruct, inner_char_field)
        * 8
        == 32
);
const _: () = {
    static_assertions::assert_impl_all!(i32: Copy);
};
const _: () = {
    static_assertions::assert_impl_all!(u8: Copy);
};

const _: () = assert!(rust_std::mem::size_of::<crate::FieldInTailPadding>() == 8);
const _: () = assert!(rust_std::mem::align_of::<crate::FieldInTailPadding>() == 4);
const _: () = {
    static_assertions::assert_not_impl_all!(crate::FieldInTailPadding: Copy);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::FieldInTailPadding: Drop);
};
const _: () =
    assert!(memoffset_unstable_const::offset_of!(crate::FieldInTailPadding, inner_struct) * 8 == 0);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(
        crate::FieldInTailPadding,
        char_in_tail_padding_of_prev_field
    ) * 8
        == 40
);
const _: () = {
    static_assertions::assert_impl_all!(u8: Copy);
};
