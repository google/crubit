// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for C++ target
// //rs_bindings_from_cc/test/golden:unions_cc
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

#[derive(Clone, Copy)]
#[repr(C)]
pub union EmptyUnion {
    __non_field_data: [crate::rust_std::mem::MaybeUninit<u8>; 1],
}
forward_declare::unsafe_define!(forward_declare::symbol!("EmptyUnion"), crate::EmptyUnion);

impl Default for EmptyUnion {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = crate::rust_std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN10EmptyUnionC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b> From<ctor::RvalueReference<'b, crate::EmptyUnion>> for EmptyUnion {
    #[inline(always)]
    fn from(__param_0: ctor::RvalueReference<'b, crate::EmptyUnion>) -> Self {
        let mut tmp = crate::rust_std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN10EmptyUnionC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

// rs_bindings_from_cc/test/golden/unions.h;l=10
// Error while generating bindings for item 'EmptyUnion::operator=':
// Bindings for this kind of operator are not supported

// rs_bindings_from_cc/test/golden/unions.h;l=10
// Error while generating bindings for item 'EmptyUnion::operator=':
// Bindings for this kind of operator are not supported

#[ctor::recursively_pinned]
#[repr(C)]
pub struct Nontrivial {
    __non_field_data: [crate::rust_std::mem::MaybeUninit<u8>; 0],
    pub field: i32,
}
forward_declare::unsafe_define!(forward_declare::symbol!("Nontrivial"), crate::Nontrivial);

impl ctor::CtorNew<()> for Nontrivial {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        ctor::FnCtor::new(
            move |dest: crate::rust_std::pin::Pin<&mut crate::rust_std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN10NontrivialC1Ev(
                    crate::rust_std::pin::Pin::into_inner_unchecked(dest),
                );
            },
        )
    }
}

impl<'b> ctor::CtorNew<ctor::RvalueReference<'b, crate::Nontrivial>> for Nontrivial {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ctor::RvalueReference<'b, crate::Nontrivial>) -> Self::CtorType {
        let __param_0 = args;
        ctor::FnCtor::new(
            move |dest: crate::rust_std::pin::Pin<&mut crate::rust_std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN10NontrivialC1EOS_(
                    crate::rust_std::pin::Pin::into_inner_unchecked(dest),
                    __param_0,
                );
            },
        )
    }
}
impl<'b> ctor::CtorNew<(ctor::RvalueReference<'b, crate::Nontrivial>,)> for Nontrivial {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: (ctor::RvalueReference<'b, crate::Nontrivial>,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ctor::CtorNew<ctor::RvalueReference<'b, crate::Nontrivial>>>::ctor_new(arg)
    }
}

#[ctor::recursively_pinned(PinnedDrop)]
#[repr(C)]
pub struct TriviallyCopyableButNontriviallyDestructible {
    __non_field_data: [crate::rust_std::mem::MaybeUninit<u8>; 1],
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("TriviallyCopyableButNontriviallyDestructible"),
    crate::TriviallyCopyableButNontriviallyDestructible
);

// rs_bindings_from_cc/test/golden/unions.h;l=19
// Error while generating bindings for item 'TriviallyCopyableButNontriviallyDestructible::operator=':
// Bindings for this kind of operator are not supported

impl<'b> ctor::CtorNew<&'b crate::TriviallyCopyableButNontriviallyDestructible>
    for TriviallyCopyableButNontriviallyDestructible
{
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: &'b crate::TriviallyCopyableButNontriviallyDestructible) -> Self::CtorType {
        let __param_0 = args;
        ctor::FnCtor::new(
            move |dest: crate::rust_std::pin::Pin<&mut crate::rust_std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN44TriviallyCopyableButNontriviallyDestructibleC1ERKS_(crate::rust_std::pin::Pin::into_inner_unchecked(dest),__param_0);
            },
        )
    }
}
impl<'b> ctor::CtorNew<(&'b crate::TriviallyCopyableButNontriviallyDestructible,)>
    for TriviallyCopyableButNontriviallyDestructible
{
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(
        args: (&'b crate::TriviallyCopyableButNontriviallyDestructible,),
    ) -> Self::CtorType {
        let (arg,) = args;
        <Self as ctor::CtorNew<&'b crate::TriviallyCopyableButNontriviallyDestructible>>::ctor_new(
            arg,
        )
    }
}

impl ::ctor::PinnedDrop for TriviallyCopyableButNontriviallyDestructible {
    #[inline(always)]
    unsafe fn pinned_drop<'a>(self: crate::rust_std::pin::Pin<&'a mut Self>) {
        crate::detail::__rust_thunk___ZN44TriviallyCopyableButNontriviallyDestructibleD1Ev(self)
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub union NonEmptyUnion {
    pub bool_field: bool,
    pub char_field: u8,
    pub int_field: i32,
    pub long_long_field: i64,
}
forward_declare::unsafe_define!(forward_declare::symbol!("NonEmptyUnion"), crate::NonEmptyUnion);

impl Default for NonEmptyUnion {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = crate::rust_std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN13NonEmptyUnionC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b> From<ctor::RvalueReference<'b, crate::NonEmptyUnion>> for NonEmptyUnion {
    #[inline(always)]
    fn from(__param_0: ctor::RvalueReference<'b, crate::NonEmptyUnion>) -> Self {
        let mut tmp = crate::rust_std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN13NonEmptyUnionC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

// rs_bindings_from_cc/test/golden/unions.h;l=25
// Error while generating bindings for item 'NonEmptyUnion::operator=':
// Bindings for this kind of operator are not supported

// rs_bindings_from_cc/test/golden/unions.h;l=25
// Error while generating bindings for item 'NonEmptyUnion::operator=':
// Bindings for this kind of operator are not supported

#[ctor::recursively_pinned]
#[repr(C)]
pub union NonCopyUnion {
    pub trivial_member: bool,
    pub nontrivial_member: crate::rust_std::mem::ManuallyDrop<crate::Nontrivial>,
}
forward_declare::unsafe_define!(forward_declare::symbol!("NonCopyUnion"), crate::NonCopyUnion);

#[repr(C)]
pub union NonCopyUnion2 {
    pub trivial_member: bool,
    pub nontrivial_member:
        crate::rust_std::mem::ManuallyDrop<crate::TriviallyCopyableButNontriviallyDestructible>,
}
forward_declare::unsafe_define!(forward_declare::symbol!("NonCopyUnion2"), crate::NonCopyUnion2);

impl Clone for NonCopyUnion2 {
    #[inline(always)]
    fn clone<'b>(&'b self) -> Self {
        let mut tmp = crate::rust_std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN13NonCopyUnion2C1ERKS_(&mut tmp, self);
            tmp.assume_init()
        }
    }
}

impl<'b> From<ctor::RvalueReference<'b, crate::NonCopyUnion2>> for NonCopyUnion2 {
    #[inline(always)]
    fn from(__param_0: ctor::RvalueReference<'b, crate::NonCopyUnion2>) -> Self {
        let mut tmp = crate::rust_std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN13NonCopyUnion2C1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

// rs_bindings_from_cc/test/golden/unions.h;l=37
// Error while generating bindings for item 'NonCopyUnion2::operator=':
// Bindings for this kind of operator are not supported

// rs_bindings_from_cc/test/golden/unions.h;l=37
// Error while generating bindings for item 'NonCopyUnion2::operator=':
// Bindings for this kind of operator are not supported

#[derive(Clone, Copy)]
#[repr(C)]
pub union UnionWithOpaqueField {
    /// Reason for representing this field as a blob of bytes:
    /// Unsupported type 'char[42]': Unsupported clang::Type class 'ConstantArray'
    constant_array_field_not_yet_supported: [crate::rust_std::mem::MaybeUninit<u8>; 42],
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("UnionWithOpaqueField"),
    crate::UnionWithOpaqueField
);

impl Default for UnionWithOpaqueField {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = crate::rust_std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN20UnionWithOpaqueFieldC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b> From<ctor::RvalueReference<'b, crate::UnionWithOpaqueField>> for UnionWithOpaqueField {
    #[inline(always)]
    fn from(__param_0: ctor::RvalueReference<'b, crate::UnionWithOpaqueField>) -> Self {
        let mut tmp = crate::rust_std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN20UnionWithOpaqueFieldC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

// rs_bindings_from_cc/test/golden/unions.h;l=42
// Error while generating bindings for item 'UnionWithOpaqueField::operator=':
// Bindings for this kind of operator are not supported

// rs_bindings_from_cc/test/golden/unions.h;l=42
// Error while generating bindings for item 'UnionWithOpaqueField::operator=':
// Bindings for this kind of operator are not supported

// THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_UNIONS_H_

mod detail {
    #[allow(unused_imports)]
    use super::*;
    extern "C" {
        pub(crate) fn __rust_thunk___ZN10EmptyUnionC1Ev<'a>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::EmptyUnion>,
        );
        pub(crate) fn __rust_thunk___ZN10EmptyUnionC1EOS_<'a, 'b>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::EmptyUnion>,
            __param_0: ctor::RvalueReference<'b, crate::EmptyUnion>,
        );
        #[link_name = "_ZN10NontrivialC1Ev"]
        pub(crate) fn __rust_thunk___ZN10NontrivialC1Ev<'a>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::Nontrivial>,
        );
        #[link_name = "_ZN10NontrivialC1EOS_"]
        pub(crate) fn __rust_thunk___ZN10NontrivialC1EOS_<'a, 'b>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::Nontrivial>,
            __param_0: ctor::RvalueReference<'b, crate::Nontrivial>,
        );
        pub(crate) fn __rust_thunk___ZN44TriviallyCopyableButNontriviallyDestructibleC1ERKS_<
            'a,
            'b,
        >(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<
                crate::TriviallyCopyableButNontriviallyDestructible,
            >,
            __param_0: &'b crate::TriviallyCopyableButNontriviallyDestructible,
        );
        pub(crate) fn __rust_thunk___ZN44TriviallyCopyableButNontriviallyDestructibleD1Ev<'a>(
            __this: crate::rust_std::pin::Pin<
                &'a mut crate::TriviallyCopyableButNontriviallyDestructible,
            >,
        );
        pub(crate) fn __rust_thunk___ZN13NonEmptyUnionC1Ev<'a>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::NonEmptyUnion>,
        );
        pub(crate) fn __rust_thunk___ZN13NonEmptyUnionC1EOS_<'a, 'b>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::NonEmptyUnion>,
            __param_0: ctor::RvalueReference<'b, crate::NonEmptyUnion>,
        );
        pub(crate) fn __rust_thunk___ZN13NonCopyUnion2C1ERKS_<'a, 'b>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::NonCopyUnion2>,
            __param_0: &'b crate::NonCopyUnion2,
        );
        pub(crate) fn __rust_thunk___ZN13NonCopyUnion2C1EOS_<'a, 'b>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::NonCopyUnion2>,
            __param_0: ctor::RvalueReference<'b, crate::NonCopyUnion2>,
        );
        pub(crate) fn __rust_thunk___ZN20UnionWithOpaqueFieldC1Ev<'a>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::UnionWithOpaqueField>,
        );
        pub(crate) fn __rust_thunk___ZN20UnionWithOpaqueFieldC1EOS_<'a, 'b>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::UnionWithOpaqueField>,
            __param_0: ctor::RvalueReference<'b, crate::UnionWithOpaqueField>,
        );
    }
}

const _: () = assert!(rust_std::mem::size_of::<Option<&i32>>() == rust_std::mem::size_of::<&i32>());

const _: () = assert!(rust_std::mem::size_of::<crate::EmptyUnion>() == 1);
const _: () = assert!(rust_std::mem::align_of::<crate::EmptyUnion>() == 1);
const _: () = {
    static_assertions::assert_impl_all!(crate::EmptyUnion: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::EmptyUnion: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_all!(crate::EmptyUnion: Drop);
};

const _: () = assert!(rust_std::mem::size_of::<crate::Nontrivial>() == 4);
const _: () = assert!(rust_std::mem::align_of::<crate::Nontrivial>() == 4);
const _: () = {
    static_assertions::assert_not_impl_all!(crate::Nontrivial: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_all!(crate::Nontrivial: Drop);
};
const _: () = assert!(memoffset_unstable_const::offset_of!(crate::Nontrivial, field) == 0);

const _: () =
    assert!(rust_std::mem::size_of::<crate::TriviallyCopyableButNontriviallyDestructible>() == 1);
const _: () =
    assert!(rust_std::mem::align_of::<crate::TriviallyCopyableButNontriviallyDestructible>() == 1);
const _: () = {
    static_assertions::assert_not_impl_all!(
        crate::TriviallyCopyableButNontriviallyDestructible: Copy
    );
};
const _: () = {
    static_assertions::assert_impl_all!(crate::TriviallyCopyableButNontriviallyDestructible: Drop);
};

const _: () = assert!(rust_std::mem::size_of::<crate::NonEmptyUnion>() == 8);
const _: () = assert!(rust_std::mem::align_of::<crate::NonEmptyUnion>() == 8);
const _: () = {
    static_assertions::assert_impl_all!(crate::NonEmptyUnion: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::NonEmptyUnion: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_all!(crate::NonEmptyUnion: Drop);
};
const _: () = {
    static_assertions::assert_impl_all!(bool: Copy);
};
const _: () = {
    static_assertions::assert_impl_all!(u8: Copy);
};
const _: () = {
    static_assertions::assert_impl_all!(i32: Copy);
};
const _: () = {
    static_assertions::assert_impl_all!(i64: Copy);
};

const _: () = assert!(rust_std::mem::size_of::<crate::NonCopyUnion>() == 4);
const _: () = assert!(rust_std::mem::align_of::<crate::NonCopyUnion>() == 4);
const _: () = {
    static_assertions::assert_not_impl_all!(crate::NonCopyUnion: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_all!(crate::NonCopyUnion: Drop);
};
const _: () = {
    static_assertions::assert_impl_all!(bool: Copy);
};

const _: () = assert!(rust_std::mem::size_of::<crate::NonCopyUnion2>() == 1);
const _: () = assert!(rust_std::mem::align_of::<crate::NonCopyUnion2>() == 1);
const _: () = {
    static_assertions::assert_not_impl_all!(crate::NonCopyUnion2: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_all!(crate::NonCopyUnion2: Drop);
};
const _: () = {
    static_assertions::assert_impl_all!(bool: Copy);
};

const _: () = assert!(rust_std::mem::size_of::<crate::UnionWithOpaqueField>() == 42);
const _: () = assert!(rust_std::mem::align_of::<crate::UnionWithOpaqueField>() == 1);
const _: () = {
    static_assertions::assert_impl_all!(crate::UnionWithOpaqueField: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::UnionWithOpaqueField: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_all!(crate::UnionWithOpaqueField: Drop);
};
