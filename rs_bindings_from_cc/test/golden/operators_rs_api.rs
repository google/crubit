// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for C++ target
// //rs_bindings_from_cc/test/golden:operators_cc
#![rustfmt::skip]
#![feature(const_ptr_offset_from, custom_inner_attributes)]
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

// rs_bindings_from_cc/test/golden/operators.h;l=10
// Error while generating bindings for item 'AddableConstMember::operator=':
// operator= for Unpin types is not yet supported.

// rs_bindings_from_cc/test/golden/operators.h;l=10
// Error while generating bindings for item 'AddableConstMember::operator=':
// operator= for Unpin types is not yet supported.

impl<'a, 'b> ::std::ops::Add<&'b crate::AddableConstMember> for &'a crate::AddableConstMember {
    type Output = crate::AddableConstMember;
    #[inline(always)]
    fn add(self, rhs: &'b crate::AddableConstMember) -> crate::AddableConstMember {
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

// rs_bindings_from_cc/test/golden/operators.h;l=18
// Error while generating bindings for item 'AddableNonConstMember::operator=':
// operator= for Unpin types is not yet supported.

// rs_bindings_from_cc/test/golden/operators.h;l=18
// Error while generating bindings for item 'AddableNonConstMember::operator=':
// operator= for Unpin types is not yet supported.

impl<'a, 'b> ::std::ops::Add<&'b crate::AddableNonConstMember>
    for &'a mut crate::AddableNonConstMember
{
    type Output = crate::AddableNonConstMember;
    #[inline(always)]
    fn add(self, rhs: &'b crate::AddableNonConstMember) -> crate::AddableNonConstMember {
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

// rs_bindings_from_cc/test/golden/operators.h;l=26
// Error while generating bindings for item 'AddableFriend::operator=':
// operator= for Unpin types is not yet supported.

// rs_bindings_from_cc/test/golden/operators.h;l=26
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

// rs_bindings_from_cc/test/golden/operators.h;l=35
// Error while generating bindings for item 'AddableFree::operator=':
// operator= for Unpin types is not yet supported.

// rs_bindings_from_cc/test/golden/operators.h;l=35
// Error while generating bindings for item 'AddableFree::operator=':
// operator= for Unpin types is not yet supported.

// rs_bindings_from_cc/test/golden/operators.h;l=37
// Error while generating bindings for item 'operator+':
// operator+ must be a member function (b/219826128).

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
