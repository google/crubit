// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for C++ target
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc
#![rustfmt::skip]
#![feature(const_ptr_offset_from, custom_inner_attributes, negative_impls, type_alias_impl_trait)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use ::std as rust_std;
use memoffset_unstable_const::offset_of;
use static_assertions::{assert_impl_all, assert_not_impl_all};

pub type __builtin_ms_va_list = *mut u8;

// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

/// Nontrivial due to (declared, but not yet defined) user-specified constructor
/// and destructor.
///
/// This makes it nontrivial for calls (so not trivially relocatable), as well
/// as specifically giving it a nontrivial move constructor and destructor.
#[repr(C)]
pub struct Nontrivial {
    pub field: i32,
}

impl !Unpin for Nontrivial {}

impl ctor::CtorNew<i32> for Nontrivial {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(field: i32) -> Self::CtorType {
        ctor::FnCtor::new(
            move |dest: rust_std::pin::Pin<&mut rust_std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN10NontrivialC1Ei(
                    rust_std::pin::Pin::into_inner_unchecked(dest),
                    field,
                );
            },
        )
    }
}

impl<'b> ctor::CtorNew<ctor::RvalueReference<'b, Nontrivial>> for Nontrivial {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(__param_0: ctor::RvalueReference<'b, Nontrivial>) -> Self::CtorType {
        ctor::FnCtor::new(
            move |dest: rust_std::pin::Pin<&mut rust_std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN10NontrivialC1EOS_(
                    rust_std::pin::Pin::into_inner_unchecked(dest),
                    __param_0,
                );
            },
        )
    }
}

impl Drop for Nontrivial {
    #[inline(always)]
    fn drop<'a>(&'a mut self) {
        unsafe { crate::detail::__rust_thunk___ZN10NontrivialD1Ev(self) }
    }
}

impl Nontrivial {
    #[inline(always)]
    pub fn MemberFunction<'a>(self: rust_std::pin::Pin<&'a mut Self>) {
        unsafe { crate::detail::__rust_thunk___ZN10Nontrivial14MemberFunctionEv(self) }
    }
}

/// Nontrivial due to (inline) user-specified constructor and destructor.
///
/// This makes it nontrivial for calls (so not trivially relocatable), as well
/// as specifically giving it a nontrivial move constructor and destructor.
#[repr(C)]
pub struct NontrivialInline {
    pub field: i32,
}

impl !Unpin for NontrivialInline {}

impl ctor::CtorNew<i32> for NontrivialInline {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(field: i32) -> Self::CtorType {
        ctor::FnCtor::new(
            move |dest: rust_std::pin::Pin<&mut rust_std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN16NontrivialInlineC1Ei(
                    rust_std::pin::Pin::into_inner_unchecked(dest),
                    field,
                );
            },
        )
    }
}

impl<'b> ctor::CtorNew<ctor::RvalueReference<'b, NontrivialInline>> for NontrivialInline {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(__param_0: ctor::RvalueReference<'b, NontrivialInline>) -> Self::CtorType {
        ctor::FnCtor::new(
            move |dest: rust_std::pin::Pin<&mut rust_std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN16NontrivialInlineC1EOS_(
                    rust_std::pin::Pin::into_inner_unchecked(dest),
                    __param_0,
                );
            },
        )
    }
}

impl Drop for NontrivialInline {
    #[inline(always)]
    fn drop<'a>(&'a mut self) {
        unsafe { crate::detail::__rust_thunk___ZN16NontrivialInlineD1Ev(self) }
    }
}

impl NontrivialInline {
    #[inline(always)]
    pub fn MemberFunction<'a>(self: rust_std::pin::Pin<&'a mut Self>) {
        unsafe { crate::detail::__rust_thunk___ZN16NontrivialInline14MemberFunctionEv(self) }
    }
}

/// Nontrivial due to member variables.
///
/// This changes how the destructor / drop impl work -- instead of calling
/// the destructor for NontrivialMembers, it just calls the destructors for
/// each field.
#[repr(C)]
pub struct NontrivialMembers {
    pub nontrivial_member: rust_std::mem::ManuallyDrop<Nontrivial>,
}

impl !Unpin for NontrivialMembers {}

impl<'b> ctor::CtorNew<ctor::RvalueReference<'b, NontrivialMembers>> for NontrivialMembers {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(__param_0: ctor::RvalueReference<'b, NontrivialMembers>) -> Self::CtorType {
        ctor::FnCtor::new(
            move |dest: rust_std::pin::Pin<&mut rust_std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN17NontrivialMembersC1EOS_(
                    rust_std::pin::Pin::into_inner_unchecked(dest),
                    __param_0,
                );
            },
        )
    }
}

impl Drop for NontrivialMembers {
    #[inline(always)]
    fn drop<'a>(&'a mut self) {
        unsafe { crate::detail::__rust_thunk___ZN17NontrivialMembersD1Ev(self) }
    }
}

// rs_bindings_from_cc/test/golden/nontrivial_type.h;l=48
// Error while generating bindings for item 'TakesByValue':
// Non-trivial_abi type 'struct Nontrivial' is not supported by value as parameter #0

// rs_bindings_from_cc/test/golden/nontrivial_type.h;l=49
// Error while generating bindings for item 'TakesByValueInline':
// Non-trivial_abi type 'struct NontrivialInline' is not supported by value as parameter #0

// rs_bindings_from_cc/test/golden/nontrivial_type.h;l=51
// Error while generating bindings for item 'ReturnsByValue':
// Non-trivial_abi type 'struct Nontrivial' is not supported by value as a return type

#[inline(always)]
pub fn TakesByConstReference<'a>(nontrivial: &'a Nontrivial) -> &'a Nontrivial {
    unsafe { crate::detail::__rust_thunk___Z21TakesByConstReferenceRK10Nontrivial(nontrivial) }
}

#[inline(always)]
pub fn TakesByReference<'a>(
    nontrivial: rust_std::pin::Pin<&'a mut Nontrivial>,
) -> rust_std::pin::Pin<&'a mut Nontrivial> {
    unsafe { crate::detail::__rust_thunk___Z16TakesByReferenceR10Nontrivial(nontrivial) }
}

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_NONTRIVIAL_TYPE_H_

mod detail {
    #[allow(unused_imports)]
    use super::*;
    extern "C" {
        #[link_name = "_ZN10NontrivialC1Ei"]
        pub(crate) fn __rust_thunk___ZN10NontrivialC1Ei<'a>(
            __this: &'a mut rust_std::mem::MaybeUninit<Nontrivial>,
            field: i32,
        );
        #[link_name = "_ZN10NontrivialC1EOS_"]
        pub(crate) fn __rust_thunk___ZN10NontrivialC1EOS_<'a, 'b>(
            __this: &'a mut rust_std::mem::MaybeUninit<Nontrivial>,
            __param_0: ctor::RvalueReference<'b, Nontrivial>,
        );
        #[link_name = "_ZN10NontrivialD1Ev"]
        pub(crate) fn __rust_thunk___ZN10NontrivialD1Ev<'a>(__this: *mut Nontrivial);
        #[link_name = "_ZN10Nontrivial14MemberFunctionEv"]
        pub(crate) fn __rust_thunk___ZN10Nontrivial14MemberFunctionEv<'a>(
            __this: rust_std::pin::Pin<&'a mut Nontrivial>,
        );
        pub(crate) fn __rust_thunk___ZN16NontrivialInlineC1Ei<'a>(
            __this: &'a mut rust_std::mem::MaybeUninit<NontrivialInline>,
            field: i32,
        );
        pub(crate) fn __rust_thunk___ZN16NontrivialInlineC1EOS_<'a, 'b>(
            __this: &'a mut rust_std::mem::MaybeUninit<NontrivialInline>,
            __param_0: ctor::RvalueReference<'b, NontrivialInline>,
        );
        pub(crate) fn __rust_thunk___ZN16NontrivialInlineD1Ev<'a>(__this: *mut NontrivialInline);
        pub(crate) fn __rust_thunk___ZN16NontrivialInline14MemberFunctionEv<'a>(
            __this: rust_std::pin::Pin<&'a mut NontrivialInline>,
        );
        pub(crate) fn __rust_thunk___ZN17NontrivialMembersC1EOS_<'a, 'b>(
            __this: &'a mut rust_std::mem::MaybeUninit<NontrivialMembers>,
            __param_0: ctor::RvalueReference<'b, NontrivialMembers>,
        );
        pub(crate) fn __rust_thunk___ZN17NontrivialMembersD1Ev<'a>(__this: *mut NontrivialMembers);
        #[link_name = "_Z21TakesByConstReferenceRK10Nontrivial"]
        pub(crate) fn __rust_thunk___Z21TakesByConstReferenceRK10Nontrivial<'a>(
            nontrivial: &'a Nontrivial,
        ) -> &'a Nontrivial;
        #[link_name = "_Z16TakesByReferenceR10Nontrivial"]
        pub(crate) fn __rust_thunk___Z16TakesByReferenceR10Nontrivial<'a>(
            nontrivial: rust_std::pin::Pin<&'a mut Nontrivial>,
        ) -> rust_std::pin::Pin<&'a mut Nontrivial>;
    }
}

const _: () = assert!(rust_std::mem::size_of::<Option<&i32>>() == rust_std::mem::size_of::<&i32>());

const _: () = assert!(rust_std::mem::size_of::<Nontrivial>() == 4usize);
const _: () = assert!(rust_std::mem::align_of::<Nontrivial>() == 4usize);
const _: () = {
    assert_not_impl_all!(Nontrivial: Copy);
};
const _: () = {
    assert_impl_all!(Nontrivial: Drop);
};
const _: () = assert!(offset_of!(Nontrivial, field) * 8 == 0usize);
const _: () = {
    assert_impl_all!(i32: Copy);
};

const _: () = assert!(rust_std::mem::size_of::<NontrivialInline>() == 4usize);
const _: () = assert!(rust_std::mem::align_of::<NontrivialInline>() == 4usize);
const _: () = {
    assert_not_impl_all!(NontrivialInline: Copy);
};
const _: () = {
    assert_impl_all!(NontrivialInline: Drop);
};
const _: () = assert!(offset_of!(NontrivialInline, field) * 8 == 0usize);
const _: () = {
    assert_impl_all!(i32: Copy);
};

const _: () = assert!(rust_std::mem::size_of::<NontrivialMembers>() == 4usize);
const _: () = assert!(rust_std::mem::align_of::<NontrivialMembers>() == 4usize);
const _: () = {
    assert_not_impl_all!(NontrivialMembers: Copy);
};
const _: () = {
    assert_impl_all!(NontrivialMembers: Drop);
};
const _: () = assert!(offset_of!(NontrivialMembers, nontrivial_member) * 8 == 0usize);
