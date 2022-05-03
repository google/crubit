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
forward_declare::unsafe_define!(forward_declare::symbol!("Nontrivial"), Nontrivial);

impl !Unpin for Nontrivial {}

impl ctor::CtorNew<()> for Nontrivial {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        ctor::FnCtor::new(
            move |dest: rust_std::pin::Pin<&mut rust_std::mem::MaybeUninit<Self>>| unsafe {
                detail::__rust_thunk___ZN10NontrivialC1Ev(
                    rust_std::pin::Pin::into_inner_unchecked(dest),
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
            move |dest: rust_std::pin::Pin<&mut rust_std::mem::MaybeUninit<Self>>| unsafe {
                detail::__rust_thunk___ZN10NontrivialC1Ei(
                    rust_std::pin::Pin::into_inner_unchecked(dest),
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
            move |dest: rust_std::pin::Pin<&mut rust_std::mem::MaybeUninit<Self>>| unsafe {
                detail::__rust_thunk___ZN10NontrivialC1Eii(
                    rust_std::pin::Pin::into_inner_unchecked(dest),
                    field,
                    unused,
                );
            },
        )
    }
}

impl<'b> ctor::CtorNew<ctor::RvalueReference<'b, Nontrivial>> for Nontrivial {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ctor::RvalueReference<'b, Nontrivial>) -> Self::CtorType {
        let __param_0 = args;
        ctor::FnCtor::new(
            move |dest: rust_std::pin::Pin<&mut rust_std::mem::MaybeUninit<Self>>| unsafe {
                detail::__rust_thunk___ZN10NontrivialC1EOS_(
                    rust_std::pin::Pin::into_inner_unchecked(dest),
                    __param_0,
                );
            },
        )
    }
}
impl<'b> ctor::CtorNew<(ctor::RvalueReference<'b, Nontrivial>,)> for Nontrivial {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: (ctor::RvalueReference<'b, Nontrivial>,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ctor::CtorNew<ctor::RvalueReference<'b, Nontrivial>>>::ctor_new(arg)
    }
}

impl Drop for Nontrivial {
    #[inline(always)]
    fn drop<'a>(&'a mut self) {
        unsafe { detail::__rust_thunk___ZN10NontrivialD1Ev(self) }
    }
}

impl Nontrivial {
    #[inline(always)]
    pub fn MemberFunction<'a>(self: rust_std::pin::Pin<&'a mut Self>) {
        unsafe { detail::__rust_thunk___ZN10Nontrivial14MemberFunctionEv(self) }
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
forward_declare::unsafe_define!(forward_declare::symbol!("NontrivialInline"), NontrivialInline);

impl !Unpin for NontrivialInline {}

impl ctor::CtorNew<()> for NontrivialInline {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        ctor::FnCtor::new(
            move |dest: rust_std::pin::Pin<&mut rust_std::mem::MaybeUninit<Self>>| unsafe {
                detail::__rust_thunk___ZN16NontrivialInlineC1Ev(
                    rust_std::pin::Pin::into_inner_unchecked(dest),
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
            move |dest: rust_std::pin::Pin<&mut rust_std::mem::MaybeUninit<Self>>| unsafe {
                detail::__rust_thunk___ZN16NontrivialInlineC1Ei(
                    rust_std::pin::Pin::into_inner_unchecked(dest),
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
            move |dest: rust_std::pin::Pin<&mut rust_std::mem::MaybeUninit<Self>>| unsafe {
                detail::__rust_thunk___ZN16NontrivialInlineC1Eii(
                    rust_std::pin::Pin::into_inner_unchecked(dest),
                    field,
                    unused,
                );
            },
        )
    }
}

impl<'b> ctor::CtorNew<ctor::RvalueReference<'b, NontrivialInline>> for NontrivialInline {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ctor::RvalueReference<'b, NontrivialInline>) -> Self::CtorType {
        let __param_0 = args;
        ctor::FnCtor::new(
            move |dest: rust_std::pin::Pin<&mut rust_std::mem::MaybeUninit<Self>>| unsafe {
                detail::__rust_thunk___ZN16NontrivialInlineC1EOS_(
                    rust_std::pin::Pin::into_inner_unchecked(dest),
                    __param_0,
                );
            },
        )
    }
}
impl<'b> ctor::CtorNew<(ctor::RvalueReference<'b, NontrivialInline>,)> for NontrivialInline {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: (ctor::RvalueReference<'b, NontrivialInline>,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ctor::CtorNew<ctor::RvalueReference<'b, NontrivialInline>>>::ctor_new(arg)
    }
}

impl Drop for NontrivialInline {
    #[inline(always)]
    fn drop<'a>(&'a mut self) {
        unsafe { detail::__rust_thunk___ZN16NontrivialInlineD1Ev(self) }
    }
}

impl NontrivialInline {
    #[inline(always)]
    pub fn MemberFunction<'a>(self: rust_std::pin::Pin<&'a mut Self>) {
        unsafe { detail::__rust_thunk___ZN16NontrivialInline14MemberFunctionEv(self) }
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
forward_declare::unsafe_define!(forward_declare::symbol!("NontrivialMembers"), NontrivialMembers);

impl !Unpin for NontrivialMembers {}

impl ctor::CtorNew<()> for NontrivialMembers {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        ctor::FnCtor::new(
            move |dest: rust_std::pin::Pin<&mut rust_std::mem::MaybeUninit<Self>>| unsafe {
                detail::__rust_thunk___ZN17NontrivialMembersC1Ev(
                    rust_std::pin::Pin::into_inner_unchecked(dest),
                );
            },
        )
    }
}

impl<'b> ctor::CtorNew<ctor::RvalueReference<'b, NontrivialMembers>> for NontrivialMembers {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ctor::RvalueReference<'b, NontrivialMembers>) -> Self::CtorType {
        let __param_0 = args;
        ctor::FnCtor::new(
            move |dest: rust_std::pin::Pin<&mut rust_std::mem::MaybeUninit<Self>>| unsafe {
                detail::__rust_thunk___ZN17NontrivialMembersC1EOS_(
                    rust_std::pin::Pin::into_inner_unchecked(dest),
                    __param_0,
                );
            },
        )
    }
}
impl<'b> ctor::CtorNew<(ctor::RvalueReference<'b, NontrivialMembers>,)> for NontrivialMembers {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: (ctor::RvalueReference<'b, NontrivialMembers>,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ctor::CtorNew<ctor::RvalueReference<'b, NontrivialMembers>>>::ctor_new(arg)
    }
}

impl Drop for NontrivialMembers {
    #[inline(always)]
    fn drop<'a>(&'a mut self) {
        unsafe { detail::__rust_thunk___ZN17NontrivialMembersD1Ev(self) }
    }
}

/// Nontrivial, but trivially relocatable and final (and therefore Unpin).
#[derive(Clone)]
#[repr(C)]
pub struct NontrivialUnpin {
    pub field: i32,
}
forward_declare::unsafe_define!(forward_declare::symbol!("NontrivialUnpin"), NontrivialUnpin);

// rs_bindings_from_cc/test/golden/nontrivial_type.h;l=53
// Error while generating bindings for item 'NontrivialUnpin::operator=':
// Bindings for this kind of operator are not supported

impl Default for NontrivialUnpin {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = rust_std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            detail::__rust_thunk___ZN15NontrivialUnpinC1Ev(&mut tmp);
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

impl<'b> From<ctor::RvalueReference<'b, Nontrivial>> for NontrivialUnpin {
    #[inline(always)]
    fn from(__param_0: ctor::RvalueReference<'b, Nontrivial>) -> Self {
        let mut tmp = rust_std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            detail::__rust_thunk___ZN15NontrivialUnpinC1EO10Nontrivial(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

impl Drop for NontrivialUnpin {
    #[inline(always)]
    fn drop<'a>(&'a mut self) {
        unsafe { detail::__rust_thunk___ZN15NontrivialUnpinD1Ev(self) }
    }
}

impl NontrivialUnpin {
    #[inline(always)]
    pub fn MemberFunction<'a>(&'a mut self) {
        unsafe { detail::__rust_thunk___ZN15NontrivialUnpin14MemberFunctionEv(self) }
    }
}

// rs_bindings_from_cc/test/golden/nontrivial_type.h;l=65
// Error while generating bindings for item 'TakesByValue':
// Non-trivial_abi type 'struct Nontrivial' is not supported by value as parameter #0

// rs_bindings_from_cc/test/golden/nontrivial_type.h;l=66
// Error while generating bindings for item 'TakesByValueInline':
// Non-trivial_abi type 'struct NontrivialInline' is not supported by value as parameter #0

#[inline(always)]
pub fn TakesByValueUnpin(nontrivial: NontrivialUnpin) {
    unsafe { detail::__rust_thunk___Z17TakesByValueUnpin15NontrivialUnpin(nontrivial) }
}

// rs_bindings_from_cc/test/golden/nontrivial_type.h;l=69
// Error while generating bindings for item 'ReturnsByValue':
// Non-trivial_abi type 'struct Nontrivial' is not supported by value as a return type

#[inline(always)]
pub fn ReturnsByValueUnpin() -> NontrivialUnpin {
    unsafe { detail::__rust_thunk___Z19ReturnsByValueUnpinv() }
}

#[inline(always)]
pub fn TakesByConstReference<'a>(nontrivial: &'a Nontrivial) -> &'a Nontrivial {
    unsafe { detail::__rust_thunk___Z21TakesByConstReferenceRK10Nontrivial(nontrivial) }
}

#[inline(always)]
pub fn TakesByReference<'a>(
    nontrivial: rust_std::pin::Pin<&'a mut Nontrivial>,
) -> rust_std::pin::Pin<&'a mut Nontrivial> {
    unsafe { detail::__rust_thunk___Z16TakesByReferenceR10Nontrivial(nontrivial) }
}

#[inline(always)]
pub fn TakesByConstReferenceUnpin<'a>(nontrivial: &'a NontrivialUnpin) -> &'a NontrivialUnpin {
    unsafe { detail::__rust_thunk___Z26TakesByConstReferenceUnpinRK15NontrivialUnpin(nontrivial) }
}

#[inline(always)]
pub fn TakesByReferenceUnpin<'a>(nontrivial: &'a mut NontrivialUnpin) -> &'a mut NontrivialUnpin {
    unsafe { detail::__rust_thunk___Z21TakesByReferenceUnpinR15NontrivialUnpin(nontrivial) }
}

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_NONTRIVIAL_TYPE_H_

mod detail {
    #[allow(unused_imports)]
    use super::*;
    extern "C" {
        #[link_name = "_ZN10NontrivialC1Ev"]
        pub(crate) fn __rust_thunk___ZN10NontrivialC1Ev<'a>(
            __this: &'a mut rust_std::mem::MaybeUninit<Nontrivial>,
        );
        #[link_name = "_ZN10NontrivialC1Ei"]
        pub(crate) fn __rust_thunk___ZN10NontrivialC1Ei<'a>(
            __this: &'a mut rust_std::mem::MaybeUninit<Nontrivial>,
            field: i32,
        );
        #[link_name = "_ZN10NontrivialC1Eii"]
        pub(crate) fn __rust_thunk___ZN10NontrivialC1Eii<'a>(
            __this: &'a mut rust_std::mem::MaybeUninit<Nontrivial>,
            field: i32,
            unused: i32,
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
        pub(crate) fn __rust_thunk___ZN16NontrivialInlineC1Ev<'a>(
            __this: &'a mut rust_std::mem::MaybeUninit<NontrivialInline>,
        );
        pub(crate) fn __rust_thunk___ZN16NontrivialInlineC1Ei<'a>(
            __this: &'a mut rust_std::mem::MaybeUninit<NontrivialInline>,
            field: i32,
        );
        pub(crate) fn __rust_thunk___ZN16NontrivialInlineC1Eii<'a>(
            __this: &'a mut rust_std::mem::MaybeUninit<NontrivialInline>,
            field: i32,
            unused: i32,
        );
        pub(crate) fn __rust_thunk___ZN16NontrivialInlineC1EOS_<'a, 'b>(
            __this: &'a mut rust_std::mem::MaybeUninit<NontrivialInline>,
            __param_0: ctor::RvalueReference<'b, NontrivialInline>,
        );
        pub(crate) fn __rust_thunk___ZN16NontrivialInlineD1Ev<'a>(__this: *mut NontrivialInline);
        pub(crate) fn __rust_thunk___ZN16NontrivialInline14MemberFunctionEv<'a>(
            __this: rust_std::pin::Pin<&'a mut NontrivialInline>,
        );
        pub(crate) fn __rust_thunk___ZN17NontrivialMembersC1Ev<'a>(
            __this: &'a mut rust_std::mem::MaybeUninit<NontrivialMembers>,
        );
        pub(crate) fn __rust_thunk___ZN17NontrivialMembersC1EOS_<'a, 'b>(
            __this: &'a mut rust_std::mem::MaybeUninit<NontrivialMembers>,
            __param_0: ctor::RvalueReference<'b, NontrivialMembers>,
        );
        pub(crate) fn __rust_thunk___ZN17NontrivialMembersD1Ev<'a>(__this: *mut NontrivialMembers);
        #[link_name = "_ZN15NontrivialUnpinC1Ev"]
        pub(crate) fn __rust_thunk___ZN15NontrivialUnpinC1Ev<'a>(
            __this: &'a mut rust_std::mem::MaybeUninit<NontrivialUnpin>,
        );
        #[link_name = "_ZN15NontrivialUnpinC1EO10Nontrivial"]
        pub(crate) fn __rust_thunk___ZN15NontrivialUnpinC1EO10Nontrivial<'a, 'b>(
            __this: &'a mut rust_std::mem::MaybeUninit<NontrivialUnpin>,
            __param_0: ctor::RvalueReference<'b, Nontrivial>,
        );
        #[link_name = "_ZN15NontrivialUnpinD1Ev"]
        pub(crate) fn __rust_thunk___ZN15NontrivialUnpinD1Ev<'a>(__this: *mut NontrivialUnpin);
        #[link_name = "_ZN15NontrivialUnpin14MemberFunctionEv"]
        pub(crate) fn __rust_thunk___ZN15NontrivialUnpin14MemberFunctionEv<'a>(
            __this: &'a mut NontrivialUnpin,
        );
        #[link_name = "_Z17TakesByValueUnpin15NontrivialUnpin"]
        pub(crate) fn __rust_thunk___Z17TakesByValueUnpin15NontrivialUnpin(
            nontrivial: NontrivialUnpin,
        );
        #[link_name = "_Z19ReturnsByValueUnpinv"]
        pub(crate) fn __rust_thunk___Z19ReturnsByValueUnpinv() -> NontrivialUnpin;
        #[link_name = "_Z21TakesByConstReferenceRK10Nontrivial"]
        pub(crate) fn __rust_thunk___Z21TakesByConstReferenceRK10Nontrivial<'a>(
            nontrivial: &'a Nontrivial,
        ) -> &'a Nontrivial;
        #[link_name = "_Z16TakesByReferenceR10Nontrivial"]
        pub(crate) fn __rust_thunk___Z16TakesByReferenceR10Nontrivial<'a>(
            nontrivial: rust_std::pin::Pin<&'a mut Nontrivial>,
        ) -> rust_std::pin::Pin<&'a mut Nontrivial>;
        #[link_name = "_Z26TakesByConstReferenceUnpinRK15NontrivialUnpin"]
        pub(crate) fn __rust_thunk___Z26TakesByConstReferenceUnpinRK15NontrivialUnpin<'a>(
            nontrivial: &'a NontrivialUnpin,
        ) -> &'a NontrivialUnpin;
        #[link_name = "_Z21TakesByReferenceUnpinR15NontrivialUnpin"]
        pub(crate) fn __rust_thunk___Z21TakesByReferenceUnpinR15NontrivialUnpin<'a>(
            nontrivial: &'a mut NontrivialUnpin,
        ) -> &'a mut NontrivialUnpin;
    }
}

const _: () = assert!(rust_std::mem::size_of::<Option<&i32>>() == rust_std::mem::size_of::<&i32>());

const _: () = assert!(rust_std::mem::size_of::<Nontrivial>() == 4usize);
const _: () = assert!(rust_std::mem::align_of::<Nontrivial>() == 4usize);
const _: () = {
    static_assertions::assert_not_impl_all!(Nontrivial: Copy);
};
const _: () = {
    static_assertions::assert_impl_all!(Nontrivial: Drop);
};
const _: () = assert!(offset_of!(Nontrivial, field) * 8 == 0usize);
const _: () = {
    static_assertions::assert_impl_all!(i32: Copy);
};

const _: () = assert!(rust_std::mem::size_of::<NontrivialInline>() == 4usize);
const _: () = assert!(rust_std::mem::align_of::<NontrivialInline>() == 4usize);
const _: () = {
    static_assertions::assert_not_impl_all!(NontrivialInline: Copy);
};
const _: () = {
    static_assertions::assert_impl_all!(NontrivialInline: Drop);
};
const _: () = assert!(offset_of!(NontrivialInline, field) * 8 == 0usize);
const _: () = {
    static_assertions::assert_impl_all!(i32: Copy);
};

const _: () = assert!(rust_std::mem::size_of::<NontrivialMembers>() == 4usize);
const _: () = assert!(rust_std::mem::align_of::<NontrivialMembers>() == 4usize);
const _: () = {
    static_assertions::assert_not_impl_all!(NontrivialMembers: Copy);
};
const _: () = {
    static_assertions::assert_impl_all!(NontrivialMembers: Drop);
};
const _: () = assert!(offset_of!(NontrivialMembers, nontrivial_member) * 8 == 0usize);

const _: () = assert!(rust_std::mem::size_of::<NontrivialUnpin>() == 4usize);
const _: () = assert!(rust_std::mem::align_of::<NontrivialUnpin>() == 4usize);
const _: () = {
    static_assertions::assert_impl_all!(NontrivialUnpin: Clone);
};
const _: () = {
    static_assertions::assert_not_impl_all!(NontrivialUnpin: Copy);
};
const _: () = {
    static_assertions::assert_impl_all!(NontrivialUnpin: Drop);
};
const _: () = assert!(offset_of!(NontrivialUnpin, field) * 8 == 0usize);
const _: () = {
    static_assertions::assert_impl_all!(i32: Copy);
};
