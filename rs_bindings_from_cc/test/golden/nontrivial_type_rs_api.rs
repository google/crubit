// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for C++ target
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc
#![rustfmt::skip]
#![feature(const_ptr_offset_from, custom_inner_attributes, negative_impls, type_alias_impl_trait)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use ::std as rust_std;

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
forward_declare::unsafe_define!(forward_declare::symbol!("Nontrivial"), crate::Nontrivial);

impl !Unpin for Nontrivial {}

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

impl ctor::CtorNew<i32> for Nontrivial {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: i32) -> Self::CtorType {
        let field = args;
        ctor::FnCtor::new(
            move |dest: crate::rust_std::pin::Pin<&mut crate::rust_std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN10NontrivialC1Ei(
                    crate::rust_std::pin::Pin::into_inner_unchecked(dest),
                    field,
                );
            },
        )
    }
}
impl ctor::CtorNew<(i32,)> for Nontrivial {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: (i32,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ctor::CtorNew<i32>>::ctor_new(arg)
    }
}

impl ctor::CtorNew<(i32, i32)> for Nontrivial {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: (i32, i32)) -> Self::CtorType {
        let (field, unused) = args;
        ctor::FnCtor::new(
            move |dest: crate::rust_std::pin::Pin<&mut crate::rust_std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN10NontrivialC1Eii(
                    crate::rust_std::pin::Pin::into_inner_unchecked(dest),
                    field,
                    unused,
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

impl Drop for Nontrivial {
    #[inline(always)]
    fn drop<'a>(&'a mut self) {
        unsafe { crate::detail::__rust_thunk___ZN10NontrivialD1Ev(self) }
    }
}

impl Nontrivial {
    #[inline(always)]
    pub fn MemberFunction<'a>(self: crate::rust_std::pin::Pin<&'a mut Self>) {
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
forward_declare::unsafe_define!(
    forward_declare::symbol!("NontrivialInline"),
    crate::NontrivialInline
);

impl !Unpin for NontrivialInline {}

impl ctor::CtorNew<()> for NontrivialInline {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        ctor::FnCtor::new(
            move |dest: crate::rust_std::pin::Pin<&mut crate::rust_std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN16NontrivialInlineC1Ev(
                    crate::rust_std::pin::Pin::into_inner_unchecked(dest),
                );
            },
        )
    }
}

impl ctor::CtorNew<i32> for NontrivialInline {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: i32) -> Self::CtorType {
        let field = args;
        ctor::FnCtor::new(
            move |dest: crate::rust_std::pin::Pin<&mut crate::rust_std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN16NontrivialInlineC1Ei(
                    crate::rust_std::pin::Pin::into_inner_unchecked(dest),
                    field,
                );
            },
        )
    }
}
impl ctor::CtorNew<(i32,)> for NontrivialInline {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: (i32,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ctor::CtorNew<i32>>::ctor_new(arg)
    }
}

impl ctor::CtorNew<(i32, i32)> for NontrivialInline {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: (i32, i32)) -> Self::CtorType {
        let (field, unused) = args;
        ctor::FnCtor::new(
            move |dest: crate::rust_std::pin::Pin<&mut crate::rust_std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN16NontrivialInlineC1Eii(
                    crate::rust_std::pin::Pin::into_inner_unchecked(dest),
                    field,
                    unused,
                );
            },
        )
    }
}

impl<'b> ctor::CtorNew<ctor::RvalueReference<'b, crate::NontrivialInline>> for NontrivialInline {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ctor::RvalueReference<'b, crate::NontrivialInline>) -> Self::CtorType {
        let __param_0 = args;
        ctor::FnCtor::new(
            move |dest: crate::rust_std::pin::Pin<&mut crate::rust_std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN16NontrivialInlineC1EOS_(
                    crate::rust_std::pin::Pin::into_inner_unchecked(dest),
                    __param_0,
                );
            },
        )
    }
}
impl<'b> ctor::CtorNew<(ctor::RvalueReference<'b, crate::NontrivialInline>,)> for NontrivialInline {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: (ctor::RvalueReference<'b, crate::NontrivialInline>,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ctor::CtorNew<ctor::RvalueReference<'b, crate::NontrivialInline>>>::ctor_new(arg)
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
    pub fn MemberFunction<'a>(self: crate::rust_std::pin::Pin<&'a mut Self>) {
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
    pub nontrivial_member: crate::rust_std::mem::ManuallyDrop<crate::Nontrivial>,
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("NontrivialMembers"),
    crate::NontrivialMembers
);

impl !Unpin for NontrivialMembers {}

impl ctor::CtorNew<()> for NontrivialMembers {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        ctor::FnCtor::new(
            move |dest: crate::rust_std::pin::Pin<&mut crate::rust_std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN17NontrivialMembersC1Ev(
                    crate::rust_std::pin::Pin::into_inner_unchecked(dest),
                );
            },
        )
    }
}

impl<'b> ctor::CtorNew<ctor::RvalueReference<'b, crate::NontrivialMembers>> for NontrivialMembers {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ctor::RvalueReference<'b, crate::NontrivialMembers>) -> Self::CtorType {
        let __param_0 = args;
        ctor::FnCtor::new(
            move |dest: crate::rust_std::pin::Pin<&mut crate::rust_std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN17NontrivialMembersC1EOS_(
                    crate::rust_std::pin::Pin::into_inner_unchecked(dest),
                    __param_0,
                );
            },
        )
    }
}
impl<'b> ctor::CtorNew<(ctor::RvalueReference<'b, crate::NontrivialMembers>,)>
    for NontrivialMembers
{
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: (ctor::RvalueReference<'b, crate::NontrivialMembers>,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ctor::CtorNew<ctor::RvalueReference<'b, crate::NontrivialMembers>>>::ctor_new(arg)
    }
}

impl Drop for NontrivialMembers {
    #[inline(always)]
    fn drop<'a>(&'a mut self) {
        unsafe { crate::detail::__rust_thunk___ZN17NontrivialMembersD1Ev(self) }
    }
}

/// Nontrivial, but trivially relocatable and final (and therefore Unpin).
#[derive(Clone)]
#[repr(C)]
pub struct NontrivialUnpin {
    pub field: i32,
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("NontrivialUnpin"),
    crate::NontrivialUnpin
);

// rs_bindings_from_cc/test/golden/nontrivial_type.h;l=53
// Error while generating bindings for item 'NontrivialUnpin::operator=':
// Bindings for this kind of operator are not supported

impl Default for NontrivialUnpin {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = crate::rust_std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN15NontrivialUnpinC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

// rs_bindings_from_cc/test/golden/nontrivial_type.h;l=55
// Error while generating bindings for item 'NontrivialUnpin::NontrivialUnpin':
// Not yet supported type of constructor parameter

// rs_bindings_from_cc/test/golden/nontrivial_type.h;l=56
// Error while generating bindings for item 'NontrivialUnpin::NontrivialUnpin':
// More than 1 constructor parameter is not supported yet

impl<'b> From<ctor::RvalueReference<'b, crate::Nontrivial>> for NontrivialUnpin {
    #[inline(always)]
    fn from(__param_0: ctor::RvalueReference<'b, crate::Nontrivial>) -> Self {
        let mut tmp = crate::rust_std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN15NontrivialUnpinC1EO10Nontrivial(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

impl Drop for NontrivialUnpin {
    #[inline(always)]
    fn drop<'a>(&'a mut self) {
        unsafe { crate::detail::__rust_thunk___ZN15NontrivialUnpinD1Ev(self) }
    }
}

impl NontrivialUnpin {
    #[inline(always)]
    pub fn MemberFunction<'a>(&'a mut self) {
        unsafe { crate::detail::__rust_thunk___ZN15NontrivialUnpin14MemberFunctionEv(self) }
    }
}

// rs_bindings_from_cc/test/golden/nontrivial_type.h;l=65
// Error while generating bindings for item 'TakesByValue':
// Non-trivial_abi type 'struct Nontrivial' is not supported by value as parameter #0

// rs_bindings_from_cc/test/golden/nontrivial_type.h;l=66
// Error while generating bindings for item 'TakesByValueInline':
// Non-trivial_abi type 'struct NontrivialInline' is not supported by value as parameter #0

#[inline(always)]
pub fn TakesByValueUnpin(nontrivial: crate::NontrivialUnpin) {
    unsafe { crate::detail::__rust_thunk___Z17TakesByValueUnpin15NontrivialUnpin(nontrivial) }
}

// rs_bindings_from_cc/test/golden/nontrivial_type.h;l=69
// Error while generating bindings for item 'ReturnsByValue':
// Non-trivial_abi type 'struct Nontrivial' is not supported by value as a return type

#[inline(always)]
pub fn ReturnsByValueUnpin() -> crate::NontrivialUnpin {
    unsafe { crate::detail::__rust_thunk___Z19ReturnsByValueUnpinv() }
}

#[inline(always)]
pub fn TakesByConstReference<'a>(nontrivial: &'a crate::Nontrivial) -> &'a crate::Nontrivial {
    unsafe { crate::detail::__rust_thunk___Z21TakesByConstReferenceRK10Nontrivial(nontrivial) }
}

#[inline(always)]
pub fn TakesByReference<'a>(
    nontrivial: crate::rust_std::pin::Pin<&'a mut crate::Nontrivial>,
) -> crate::rust_std::pin::Pin<&'a mut crate::Nontrivial> {
    unsafe { crate::detail::__rust_thunk___Z16TakesByReferenceR10Nontrivial(nontrivial) }
}

#[inline(always)]
pub fn TakesByConstReferenceUnpin<'a>(
    nontrivial: &'a crate::NontrivialUnpin,
) -> &'a crate::NontrivialUnpin {
    unsafe {
        crate::detail::__rust_thunk___Z26TakesByConstReferenceUnpinRK15NontrivialUnpin(nontrivial)
    }
}

#[inline(always)]
pub fn TakesByReferenceUnpin<'a>(
    nontrivial: &'a mut crate::NontrivialUnpin,
) -> &'a mut crate::NontrivialUnpin {
    unsafe { crate::detail::__rust_thunk___Z21TakesByReferenceUnpinR15NontrivialUnpin(nontrivial) }
}

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_NONTRIVIAL_TYPE_H_

mod detail {
    #[allow(unused_imports)]
    use super::*;
    extern "C" {
        #[link_name = "_ZN10NontrivialC1Ev"]
        pub(crate) fn __rust_thunk___ZN10NontrivialC1Ev<'a>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::Nontrivial>,
        );
        #[link_name = "_ZN10NontrivialC1Ei"]
        pub(crate) fn __rust_thunk___ZN10NontrivialC1Ei<'a>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::Nontrivial>,
            field: i32,
        );
        #[link_name = "_ZN10NontrivialC1Eii"]
        pub(crate) fn __rust_thunk___ZN10NontrivialC1Eii<'a>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::Nontrivial>,
            field: i32,
            unused: i32,
        );
        #[link_name = "_ZN10NontrivialC1EOS_"]
        pub(crate) fn __rust_thunk___ZN10NontrivialC1EOS_<'a, 'b>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::Nontrivial>,
            __param_0: ctor::RvalueReference<'b, crate::Nontrivial>,
        );
        #[link_name = "_ZN10NontrivialD1Ev"]
        pub(crate) fn __rust_thunk___ZN10NontrivialD1Ev<'a>(__this: *mut crate::Nontrivial);
        #[link_name = "_ZN10Nontrivial14MemberFunctionEv"]
        pub(crate) fn __rust_thunk___ZN10Nontrivial14MemberFunctionEv<'a>(
            __this: crate::rust_std::pin::Pin<&'a mut crate::Nontrivial>,
        );
        pub(crate) fn __rust_thunk___ZN16NontrivialInlineC1Ev<'a>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::NontrivialInline>,
        );
        pub(crate) fn __rust_thunk___ZN16NontrivialInlineC1Ei<'a>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::NontrivialInline>,
            field: i32,
        );
        pub(crate) fn __rust_thunk___ZN16NontrivialInlineC1Eii<'a>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::NontrivialInline>,
            field: i32,
            unused: i32,
        );
        pub(crate) fn __rust_thunk___ZN16NontrivialInlineC1EOS_<'a, 'b>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::NontrivialInline>,
            __param_0: ctor::RvalueReference<'b, crate::NontrivialInline>,
        );
        pub(crate) fn __rust_thunk___ZN16NontrivialInlineD1Ev<'a>(
            __this: *mut crate::NontrivialInline,
        );
        pub(crate) fn __rust_thunk___ZN16NontrivialInline14MemberFunctionEv<'a>(
            __this: crate::rust_std::pin::Pin<&'a mut crate::NontrivialInline>,
        );
        pub(crate) fn __rust_thunk___ZN17NontrivialMembersC1Ev<'a>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::NontrivialMembers>,
        );
        pub(crate) fn __rust_thunk___ZN17NontrivialMembersC1EOS_<'a, 'b>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::NontrivialMembers>,
            __param_0: ctor::RvalueReference<'b, crate::NontrivialMembers>,
        );
        pub(crate) fn __rust_thunk___ZN17NontrivialMembersD1Ev<'a>(
            __this: *mut crate::NontrivialMembers,
        );
        #[link_name = "_ZN15NontrivialUnpinC1Ev"]
        pub(crate) fn __rust_thunk___ZN15NontrivialUnpinC1Ev<'a>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::NontrivialUnpin>,
        );
        #[link_name = "_ZN15NontrivialUnpinC1EO10Nontrivial"]
        pub(crate) fn __rust_thunk___ZN15NontrivialUnpinC1EO10Nontrivial<'a, 'b>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::NontrivialUnpin>,
            __param_0: ctor::RvalueReference<'b, crate::Nontrivial>,
        );
        #[link_name = "_ZN15NontrivialUnpinD1Ev"]
        pub(crate) fn __rust_thunk___ZN15NontrivialUnpinD1Ev<'a>(
            __this: *mut crate::NontrivialUnpin,
        );
        #[link_name = "_ZN15NontrivialUnpin14MemberFunctionEv"]
        pub(crate) fn __rust_thunk___ZN15NontrivialUnpin14MemberFunctionEv<'a>(
            __this: &'a mut crate::NontrivialUnpin,
        );
        #[link_name = "_Z17TakesByValueUnpin15NontrivialUnpin"]
        pub(crate) fn __rust_thunk___Z17TakesByValueUnpin15NontrivialUnpin(
            nontrivial: crate::NontrivialUnpin,
        );
        #[link_name = "_Z19ReturnsByValueUnpinv"]
        pub(crate) fn __rust_thunk___Z19ReturnsByValueUnpinv() -> crate::NontrivialUnpin;
        #[link_name = "_Z21TakesByConstReferenceRK10Nontrivial"]
        pub(crate) fn __rust_thunk___Z21TakesByConstReferenceRK10Nontrivial<'a>(
            nontrivial: &'a crate::Nontrivial,
        ) -> &'a crate::Nontrivial;
        #[link_name = "_Z16TakesByReferenceR10Nontrivial"]
        pub(crate) fn __rust_thunk___Z16TakesByReferenceR10Nontrivial<'a>(
            nontrivial: crate::rust_std::pin::Pin<&'a mut crate::Nontrivial>,
        ) -> crate::rust_std::pin::Pin<&'a mut crate::Nontrivial>;
        #[link_name = "_Z26TakesByConstReferenceUnpinRK15NontrivialUnpin"]
        pub(crate) fn __rust_thunk___Z26TakesByConstReferenceUnpinRK15NontrivialUnpin<'a>(
            nontrivial: &'a crate::NontrivialUnpin,
        ) -> &'a crate::NontrivialUnpin;
        #[link_name = "_Z21TakesByReferenceUnpinR15NontrivialUnpin"]
        pub(crate) fn __rust_thunk___Z21TakesByReferenceUnpinR15NontrivialUnpin<'a>(
            nontrivial: &'a mut crate::NontrivialUnpin,
        ) -> &'a mut crate::NontrivialUnpin;
    }
}

const _: () = assert!(rust_std::mem::size_of::<Option<&i32>>() == rust_std::mem::size_of::<&i32>());

const _: () = assert!(rust_std::mem::size_of::<crate::Nontrivial>() == 4usize);
const _: () = assert!(rust_std::mem::align_of::<crate::Nontrivial>() == 4usize);
const _: () = {
    static_assertions::assert_not_impl_all!(crate::Nontrivial: Copy);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::Nontrivial: Drop);
};
const _: () = assert!(memoffset_unstable_const::offset_of!(crate::Nontrivial, field) * 8 == 0usize);
const _: () = {
    static_assertions::assert_impl_all!(i32: Copy);
};

const _: () = assert!(rust_std::mem::size_of::<crate::NontrivialInline>() == 4usize);
const _: () = assert!(rust_std::mem::align_of::<crate::NontrivialInline>() == 4usize);
const _: () = {
    static_assertions::assert_not_impl_all!(crate::NontrivialInline: Copy);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::NontrivialInline: Drop);
};
const _: () =
    assert!(memoffset_unstable_const::offset_of!(crate::NontrivialInline, field) * 8 == 0usize);
const _: () = {
    static_assertions::assert_impl_all!(i32: Copy);
};

const _: () = assert!(rust_std::mem::size_of::<crate::NontrivialMembers>() == 4usize);
const _: () = assert!(rust_std::mem::align_of::<crate::NontrivialMembers>() == 4usize);
const _: () = {
    static_assertions::assert_not_impl_all!(crate::NontrivialMembers: Copy);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::NontrivialMembers: Drop);
};
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::NontrivialMembers, nontrivial_member) * 8 == 0usize
);

const _: () = assert!(rust_std::mem::size_of::<crate::NontrivialUnpin>() == 4usize);
const _: () = assert!(rust_std::mem::align_of::<crate::NontrivialUnpin>() == 4usize);
const _: () = {
    static_assertions::assert_impl_all!(crate::NontrivialUnpin: Clone);
};
const _: () = {
    static_assertions::assert_not_impl_all!(crate::NontrivialUnpin: Copy);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::NontrivialUnpin: Drop);
};
const _: () =
    assert!(memoffset_unstable_const::offset_of!(crate::NontrivialUnpin, field) * 8 == 0usize);
const _: () = {
    static_assertions::assert_impl_all!(i32: Copy);
};
