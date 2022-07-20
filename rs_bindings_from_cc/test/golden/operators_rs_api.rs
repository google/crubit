// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for C++ target
// //rs_bindings_from_cc/test/golden:operators_cc
#![rustfmt::skip]
#![feature(const_ptr_offset_from, custom_inner_attributes, negative_impls, type_alias_impl_trait)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![deny(warnings)]

// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[derive(Clone, Copy)]
#[repr(C, align(4))]
pub struct AddableConstMember {
    __non_field_data: [::std::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) field_: [::std::mem::MaybeUninit<u8>; 4],
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("AddableConstMember"),
    crate::AddableConstMember
);

impl Default for AddableConstMember {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN18AddableConstMemberC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b> From<::ctor::RvalueReference<'b, crate::AddableConstMember>> for AddableConstMember {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, crate::AddableConstMember>) -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN18AddableConstMemberC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

// rs_bindings_from_cc/test/golden/operators.h;l=12
// Error while generating bindings for item 'AddableConstMember::operator=':
// operator= for Unpin types is not yet supported.

// rs_bindings_from_cc/test/golden/operators.h;l=12
// Error while generating bindings for item 'AddableConstMember::operator=':
// operator= for Unpin types is not yet supported.

impl<'a, 'b> ::std::ops::Add<&'b crate::AddableConstMember> for &'a crate::AddableConstMember {
    type Output = crate::AddableConstMember;
    #[inline(always)]
    fn add(self, rhs: &'b crate::AddableConstMember) -> Self::Output {
        unsafe { crate::detail::__rust_thunk___ZNK18AddableConstMemberplERKS_(self, rhs) }
    }
}

#[derive(Clone, Copy)]
#[repr(C, align(4))]
pub struct AddableNonConstMember {
    __non_field_data: [::std::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) field_: [::std::mem::MaybeUninit<u8>; 4],
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("AddableNonConstMember"),
    crate::AddableNonConstMember
);

impl Default for AddableNonConstMember {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN21AddableNonConstMemberC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b> From<::ctor::RvalueReference<'b, crate::AddableNonConstMember>> for AddableNonConstMember {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, crate::AddableNonConstMember>) -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN21AddableNonConstMemberC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

// rs_bindings_from_cc/test/golden/operators.h;l=20
// Error while generating bindings for item 'AddableNonConstMember::operator=':
// operator= for Unpin types is not yet supported.

// rs_bindings_from_cc/test/golden/operators.h;l=20
// Error while generating bindings for item 'AddableNonConstMember::operator=':
// operator= for Unpin types is not yet supported.

impl<'a, 'b> ::std::ops::Add<&'b crate::AddableNonConstMember>
    for &'a mut crate::AddableNonConstMember
{
    type Output = crate::AddableNonConstMember;
    #[inline(always)]
    fn add(self, rhs: &'b crate::AddableNonConstMember) -> Self::Output {
        unsafe { crate::detail::__rust_thunk___ZN21AddableNonConstMemberplERKS_(self, rhs) }
    }
}

#[derive(Clone, Copy)]
#[repr(C, align(4))]
pub struct AddableFriend {
    __non_field_data: [::std::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) field_: [::std::mem::MaybeUninit<u8>; 4],
}
forward_declare::unsafe_define!(forward_declare::symbol!("AddableFriend"), crate::AddableFriend);

impl Default for AddableFriend {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN13AddableFriendC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b> From<::ctor::RvalueReference<'b, crate::AddableFriend>> for AddableFriend {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, crate::AddableFriend>) -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN13AddableFriendC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

// rs_bindings_from_cc/test/golden/operators.h;l=28
// Error while generating bindings for item 'AddableFriend::operator=':
// operator= for Unpin types is not yet supported.

// rs_bindings_from_cc/test/golden/operators.h;l=28
// Error while generating bindings for item 'AddableFriend::operator=':
// operator= for Unpin types is not yet supported.

#[derive(Clone, Copy)]
#[repr(C)]
pub struct AddableFree {
    __non_field_data: [::std::mem::MaybeUninit<u8>; 1],
}
forward_declare::unsafe_define!(forward_declare::symbol!("AddableFree"), crate::AddableFree);

impl Default for AddableFree {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN11AddableFreeC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b> From<::ctor::RvalueReference<'b, crate::AddableFree>> for AddableFree {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, crate::AddableFree>) -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN11AddableFreeC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

// rs_bindings_from_cc/test/golden/operators.h;l=37
// Error while generating bindings for item 'AddableFree::operator=':
// operator= for Unpin types is not yet supported.

// rs_bindings_from_cc/test/golden/operators.h;l=37
// Error while generating bindings for item 'AddableFree::operator=':
// operator= for Unpin types is not yet supported.

impl<'a, 'b> ::std::ops::Add<&'b crate::AddableFree> for &'a crate::AddableFree {
    type Output = crate::AddableFree;
    #[inline(always)]
    fn add(self, rhs: &'b crate::AddableFree) -> Self::Output {
        unsafe { crate::detail::__rust_thunk___ZplRK11AddableFreeS1_(self, rhs) }
    }
}

impl<'a, 'b> ::std::ops::Add<&'b mut crate::AddableFree> for &'a mut crate::AddableFree {
    type Output = crate::AddableFree;
    #[inline(always)]
    fn add(self, rhs: &'b mut crate::AddableFree) -> Self::Output {
        unsafe { crate::detail::__rust_thunk___ZplR11AddableFreeS0_(self, rhs) }
    }
}

impl ::std::ops::Add<crate::AddableFree> for AddableFree {
    type Output = crate::AddableFree;
    #[inline(always)]
    fn add(self, rhs: crate::AddableFree) -> Self::Output {
        unsafe { crate::detail::__rust_thunk___Zpl11AddableFreeS_(self, rhs) }
    }
}

// rs_bindings_from_cc/test/golden/operators.h;l=41
// Error while generating bindings for item 'operator+':
// Not yet supported for rvalue references (b/219826128)

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Overloaded {
    __non_field_data: [::std::mem::MaybeUninit<u8>; 1],
}
forward_declare::unsafe_define!(forward_declare::symbol!("Overloaded"), crate::Overloaded);

impl Default for Overloaded {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN10OverloadedC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b> From<::ctor::RvalueReference<'b, crate::Overloaded>> for Overloaded {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, crate::Overloaded>) -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN10OverloadedC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

// rs_bindings_from_cc/test/golden/operators.h;l=43
// Error while generating bindings for item 'Overloaded::operator=':
// operator= for Unpin types is not yet supported.

// rs_bindings_from_cc/test/golden/operators.h;l=43
// Error while generating bindings for item 'Overloaded::operator=':
// operator= for Unpin types is not yet supported.

impl<'a> ::std::ops::Add<i16> for &'a crate::Overloaded {
    type Output = i32;
    #[inline(always)]
    fn add(self, rhs: i16) -> Self::Output {
        unsafe { crate::detail::__rust_thunk___ZplRK10Overloadeds(self, rhs) }
    }
}

impl<'a> ::std::ops::Add<i32> for &'a crate::Overloaded {
    type Output = i32;
    #[inline(always)]
    fn add(self, rhs: i32) -> Self::Output {
        unsafe { crate::detail::__rust_thunk___ZplRK10Overloadedi(self, rhs) }
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct IncompatibleLHS {
    __non_field_data: [::std::mem::MaybeUninit<u8>; 1],
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("IncompatibleLHS"),
    crate::IncompatibleLHS
);

impl Default for IncompatibleLHS {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN15IncompatibleLHSC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b> From<::ctor::RvalueReference<'b, crate::IncompatibleLHS>> for IncompatibleLHS {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, crate::IncompatibleLHS>) -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN15IncompatibleLHSC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

// rs_bindings_from_cc/test/golden/operators.h;l=47
// Error while generating bindings for item 'IncompatibleLHS::operator=':
// operator= for Unpin types is not yet supported.

// rs_bindings_from_cc/test/golden/operators.h;l=47
// Error while generating bindings for item 'IncompatibleLHS::operator=':
// operator= for Unpin types is not yet supported.

// rs_bindings_from_cc/test/golden/operators.h;l=48
// Error while generating bindings for item 'operator+':
// Expected first parameter to be a record or reference

// rs_bindings_from_cc/test/golden/operators.h;l=49
// Error while generating bindings for item 'operator+':
// Expected first parameter referent to be a record

#[derive(Clone, Copy)]
#[repr(C, align(4))]
pub struct AddableReturnsVoid {
    __non_field_data: [::std::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) field_: [::std::mem::MaybeUninit<u8>; 4],
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("AddableReturnsVoid"),
    crate::AddableReturnsVoid
);

impl Default for AddableReturnsVoid {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN18AddableReturnsVoidC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b> From<::ctor::RvalueReference<'b, crate::AddableReturnsVoid>> for AddableReturnsVoid {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, crate::AddableReturnsVoid>) -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN18AddableReturnsVoidC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

// rs_bindings_from_cc/test/golden/operators.h;l=51
// Error while generating bindings for item 'AddableReturnsVoid::operator=':
// operator= for Unpin types is not yet supported.

// rs_bindings_from_cc/test/golden/operators.h;l=51
// Error while generating bindings for item 'AddableReturnsVoid::operator=':
// operator= for Unpin types is not yet supported.

impl<'a, 'b> ::std::ops::Add<&'b crate::AddableReturnsVoid> for &'a crate::AddableReturnsVoid {
    type Output = ();
    #[inline(always)]
    fn add(self, rhs: &'b crate::AddableReturnsVoid) -> Self::Output {
        unsafe { crate::detail::__rust_thunk___ZNK18AddableReturnsVoidplERKS_(self, rhs) }
    }
}

#[::ctor::recursively_pinned(PinnedDrop)]
#[repr(C, align(4))]
pub struct AddableConstMemberNonunpin {
    __non_field_data: [::std::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) field_: [::std::mem::MaybeUninit<u8>; 4],
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("AddableConstMemberNonunpin"),
    crate::AddableConstMemberNonunpin
);

impl ::ctor::CtorNew<()> for AddableConstMemberNonunpin {
    type CtorType = impl ::ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        ::ctor::FnCtor::new(
            move |dest: ::std::pin::Pin<&mut ::std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN26AddableConstMemberNonunpinC1Ev(
                    ::std::pin::Pin::into_inner_unchecked(dest),
                );
            },
        )
    }
}

impl<'b> ::ctor::CtorNew<&'b crate::AddableConstMemberNonunpin> for AddableConstMemberNonunpin {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: &'b crate::AddableConstMemberNonunpin) -> Self::CtorType {
        let __param_0 = args;
        ::ctor::FnCtor::new(
            move |dest: ::std::pin::Pin<&mut ::std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN26AddableConstMemberNonunpinC1ERKS_(
                    ::std::pin::Pin::into_inner_unchecked(dest),
                    __param_0,
                );
            },
        )
    }
}
impl<'b> ::ctor::CtorNew<(&'b crate::AddableConstMemberNonunpin,)> for AddableConstMemberNonunpin {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: (&'b crate::AddableConstMemberNonunpin,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'b crate::AddableConstMemberNonunpin>>::ctor_new(arg)
    }
}

impl<'b> ::ctor::Assign<&'b crate::AddableConstMemberNonunpin> for AddableConstMemberNonunpin {
    #[inline(always)]
    fn assign<'a>(
        self: ::std::pin::Pin<&'a mut Self>,
        __param_0: &'b crate::AddableConstMemberNonunpin,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN26AddableConstMemberNonunpinaSERKS_(self, __param_0);
        }
    }
}

impl<'a, 'b> ::std::ops::Add<&'b crate::AddableConstMemberNonunpin>
    for &'a crate::AddableConstMemberNonunpin
{
    type Output = impl ::ctor::Ctor<Output = crate::AddableConstMemberNonunpin>
        + ::ctor::Captures<'a>
        + ::ctor::Captures<'b>;
    #[inline(always)]
    fn add(self, rhs: &'b crate::AddableConstMemberNonunpin) -> Self::Output {
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::std::pin::Pin<
                    &mut ::std::mem::MaybeUninit<crate::AddableConstMemberNonunpin>,
                >| {
                    crate::detail::__rust_thunk___ZNK26AddableConstMemberNonunpinplERKS_(
                        ::std::pin::Pin::into_inner_unchecked(dest),
                        self,
                        rhs,
                    );
                },
            )
        }
    }
}

impl ::ctor::PinnedDrop for AddableConstMemberNonunpin {
    #[inline(always)]
    unsafe fn pinned_drop<'a>(self: ::std::pin::Pin<&'a mut Self>) {
        crate::detail::__rust_thunk___ZN26AddableConstMemberNonunpinD1Ev(self)
    }
}

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_OPERATORS_H_

mod detail {
    #[allow(unused_imports)]
    use super::*;
    extern "C" {
        pub(crate) fn __rust_thunk___ZN18AddableConstMemberC1Ev<'a>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::AddableConstMember>,
        );
        pub(crate) fn __rust_thunk___ZN18AddableConstMemberC1EOS_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::AddableConstMember>,
            __param_0: ::ctor::RvalueReference<'b, crate::AddableConstMember>,
        );
        #[link_name = "_ZNK18AddableConstMemberplERKS_"]
        pub(crate) fn __rust_thunk___ZNK18AddableConstMemberplERKS_<'a, 'b>(
            __this: &'a crate::AddableConstMember,
            rhs: &'b crate::AddableConstMember,
        ) -> crate::AddableConstMember;
        pub(crate) fn __rust_thunk___ZN21AddableNonConstMemberC1Ev<'a>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::AddableNonConstMember>,
        );
        pub(crate) fn __rust_thunk___ZN21AddableNonConstMemberC1EOS_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::AddableNonConstMember>,
            __param_0: ::ctor::RvalueReference<'b, crate::AddableNonConstMember>,
        );
        #[link_name = "_ZN21AddableNonConstMemberplERKS_"]
        pub(crate) fn __rust_thunk___ZN21AddableNonConstMemberplERKS_<'a, 'b>(
            __this: &'a mut crate::AddableNonConstMember,
            rhs: &'b crate::AddableNonConstMember,
        ) -> crate::AddableNonConstMember;
        pub(crate) fn __rust_thunk___ZN13AddableFriendC1Ev<'a>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::AddableFriend>,
        );
        pub(crate) fn __rust_thunk___ZN13AddableFriendC1EOS_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::AddableFriend>,
            __param_0: ::ctor::RvalueReference<'b, crate::AddableFriend>,
        );
        pub(crate) fn __rust_thunk___ZN11AddableFreeC1Ev<'a>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::AddableFree>,
        );
        pub(crate) fn __rust_thunk___ZN11AddableFreeC1EOS_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::AddableFree>,
            __param_0: ::ctor::RvalueReference<'b, crate::AddableFree>,
        );
        #[link_name = "_ZplRK11AddableFreeS1_"]
        pub(crate) fn __rust_thunk___ZplRK11AddableFreeS1_<'a, 'b>(
            lhs: &'a crate::AddableFree,
            rhs: &'b crate::AddableFree,
        ) -> crate::AddableFree;
        #[link_name = "_ZplR11AddableFreeS0_"]
        pub(crate) fn __rust_thunk___ZplR11AddableFreeS0_<'a, 'b>(
            lhs: &'a mut crate::AddableFree,
            rhs: &'b mut crate::AddableFree,
        ) -> crate::AddableFree;
        #[link_name = "_Zpl11AddableFreeS_"]
        pub(crate) fn __rust_thunk___Zpl11AddableFreeS_(
            lhs: crate::AddableFree,
            rhs: crate::AddableFree,
        ) -> crate::AddableFree;
        pub(crate) fn __rust_thunk___ZN10OverloadedC1Ev<'a>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::Overloaded>,
        );
        pub(crate) fn __rust_thunk___ZN10OverloadedC1EOS_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::Overloaded>,
            __param_0: ::ctor::RvalueReference<'b, crate::Overloaded>,
        );
        #[link_name = "_ZplRK10Overloadeds"]
        pub(crate) fn __rust_thunk___ZplRK10Overloadeds<'a>(
            lhs: &'a crate::Overloaded,
            rhs: i16,
        ) -> i32;
        #[link_name = "_ZplRK10Overloadedi"]
        pub(crate) fn __rust_thunk___ZplRK10Overloadedi<'a>(
            lhs: &'a crate::Overloaded,
            rhs: i32,
        ) -> i32;
        pub(crate) fn __rust_thunk___ZN15IncompatibleLHSC1Ev<'a>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::IncompatibleLHS>,
        );
        pub(crate) fn __rust_thunk___ZN15IncompatibleLHSC1EOS_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::IncompatibleLHS>,
            __param_0: ::ctor::RvalueReference<'b, crate::IncompatibleLHS>,
        );
        pub(crate) fn __rust_thunk___ZN18AddableReturnsVoidC1Ev<'a>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::AddableReturnsVoid>,
        );
        pub(crate) fn __rust_thunk___ZN18AddableReturnsVoidC1EOS_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::AddableReturnsVoid>,
            __param_0: ::ctor::RvalueReference<'b, crate::AddableReturnsVoid>,
        );
        #[link_name = "_ZNK18AddableReturnsVoidplERKS_"]
        pub(crate) fn __rust_thunk___ZNK18AddableReturnsVoidplERKS_<'a, 'b>(
            __this: &'a crate::AddableReturnsVoid,
            rhs: &'b crate::AddableReturnsVoid,
        );
        pub(crate) fn __rust_thunk___ZN26AddableConstMemberNonunpinC1Ev<'a>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::AddableConstMemberNonunpin>,
        );
        pub(crate) fn __rust_thunk___ZN26AddableConstMemberNonunpinC1ERKS_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::AddableConstMemberNonunpin>,
            __param_0: &'b crate::AddableConstMemberNonunpin,
        );
        pub(crate) fn __rust_thunk___ZN26AddableConstMemberNonunpinaSERKS_<'a, 'b>(
            __this: ::std::pin::Pin<&'a mut crate::AddableConstMemberNonunpin>,
            __param_0: &'b crate::AddableConstMemberNonunpin,
        ) -> ::std::pin::Pin<&'a mut crate::AddableConstMemberNonunpin>;
        pub(crate) fn __rust_thunk___ZNK26AddableConstMemberNonunpinplERKS_<'a, 'b>(
            __return: &mut ::std::mem::MaybeUninit<crate::AddableConstMemberNonunpin>,
            __this: &'a crate::AddableConstMemberNonunpin,
            rhs: &'b crate::AddableConstMemberNonunpin,
        );
        pub(crate) fn __rust_thunk___ZN26AddableConstMemberNonunpinD1Ev<'a>(
            __this: ::std::pin::Pin<&'a mut crate::AddableConstMemberNonunpin>,
        );
    }
}

const _: () = assert!(::std::mem::size_of::<Option<&i32>>() == ::std::mem::size_of::<&i32>());

const _: () = assert!(::std::mem::size_of::<crate::AddableConstMember>() == 4);
const _: () = assert!(::std::mem::align_of::<crate::AddableConstMember>() == 4);
const _: () = {
    static_assertions::assert_impl_all!(crate::AddableConstMember: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::AddableConstMember: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::AddableConstMember: Drop);
};
const _: () = assert!(memoffset_unstable_const::offset_of!(crate::AddableConstMember, field_) == 0);

const _: () = assert!(::std::mem::size_of::<crate::AddableNonConstMember>() == 4);
const _: () = assert!(::std::mem::align_of::<crate::AddableNonConstMember>() == 4);
const _: () = {
    static_assertions::assert_impl_all!(crate::AddableNonConstMember: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::AddableNonConstMember: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::AddableNonConstMember: Drop);
};
const _: () =
    assert!(memoffset_unstable_const::offset_of!(crate::AddableNonConstMember, field_) == 0);

const _: () = assert!(::std::mem::size_of::<crate::AddableFriend>() == 4);
const _: () = assert!(::std::mem::align_of::<crate::AddableFriend>() == 4);
const _: () = {
    static_assertions::assert_impl_all!(crate::AddableFriend: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::AddableFriend: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::AddableFriend: Drop);
};
const _: () = assert!(memoffset_unstable_const::offset_of!(crate::AddableFriend, field_) == 0);

const _: () = assert!(::std::mem::size_of::<crate::AddableFree>() == 1);
const _: () = assert!(::std::mem::align_of::<crate::AddableFree>() == 1);
const _: () = {
    static_assertions::assert_impl_all!(crate::AddableFree: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::AddableFree: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::AddableFree: Drop);
};

const _: () = assert!(::std::mem::size_of::<crate::Overloaded>() == 1);
const _: () = assert!(::std::mem::align_of::<crate::Overloaded>() == 1);
const _: () = {
    static_assertions::assert_impl_all!(crate::Overloaded: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::Overloaded: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::Overloaded: Drop);
};

const _: () = assert!(::std::mem::size_of::<crate::IncompatibleLHS>() == 1);
const _: () = assert!(::std::mem::align_of::<crate::IncompatibleLHS>() == 1);
const _: () = {
    static_assertions::assert_impl_all!(crate::IncompatibleLHS: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::IncompatibleLHS: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::IncompatibleLHS: Drop);
};

const _: () = assert!(::std::mem::size_of::<crate::AddableReturnsVoid>() == 4);
const _: () = assert!(::std::mem::align_of::<crate::AddableReturnsVoid>() == 4);
const _: () = {
    static_assertions::assert_impl_all!(crate::AddableReturnsVoid: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::AddableReturnsVoid: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::AddableReturnsVoid: Drop);
};
const _: () = assert!(memoffset_unstable_const::offset_of!(crate::AddableReturnsVoid, field_) == 0);

const _: () = assert!(::std::mem::size_of::<crate::AddableConstMemberNonunpin>() == 4);
const _: () = assert!(::std::mem::align_of::<crate::AddableConstMemberNonunpin>() == 4);
const _: () = {
    static_assertions::assert_not_impl_any!(crate::AddableConstMemberNonunpin: Copy);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::AddableConstMemberNonunpin: Drop);
};
const _: () =
    assert!(memoffset_unstable_const::offset_of!(crate::AddableConstMemberNonunpin, field_) == 0);
