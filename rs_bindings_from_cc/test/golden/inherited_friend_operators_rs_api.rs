// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:inherited_friend_operators_cc

#![rustfmt::skip]
#![feature(cfi_encoding, custom_inner_attributes, impl_trait_in_assoc_type, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![allow(deprecated)]
#![allow(unknown_lints, suspicious_runtime_symbol_definitions)]
#![deny(warnings)]
/// Hidden friends of base classes are found by argument-dependent lookup, so
/// with the `inherited_friend_operators` feature, the operators defined by
/// `EqualityMixin` get bindings, even though they are declared in another
/// target.
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "9Equatable"]
#[repr(C, align(4))]
///CRUBIT_ANNOTATE: cpp_type=Equatable
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct Equatable {
    pub value: ::ffi_11::c_int,
}
impl !Send for Equatable {}
impl !Sync for Equatable {}
unsafe impl ::cxx::ExternType for Equatable {
    type Id = ::cxx::type_id!("Equatable");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for Equatable {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN9EquatableC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

impl PartialEq for crate::Equatable {
    #[inline(always)]
    fn eq<'lhs, 'rhs>(&'lhs self, rhs: &'rhs Self) -> bool {
        unsafe { crate::detail::__rust_thunk___ZN6mixinseqERK9EquatableS2_(self, rhs) }
    }
}

/// Operators are inherited from indirect bases, too.
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "10Comparable"]
#[repr(C, align(4))]
///CRUBIT_ANNOTATE: cpp_type=Comparable
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct Comparable {
    pub value: ::ffi_11::c_int,
}
impl !Send for Comparable {}
impl !Sync for Comparable {}
unsafe impl ::cxx::ExternType for Comparable {
    type Id = ::cxx::type_id!("Comparable");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for Comparable {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN10ComparableC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

impl PartialEq for crate::Comparable {
    #[inline(always)]
    fn eq<'lhs, 'rhs>(&'lhs self, rhs: &'rhs Self) -> bool {
        unsafe { crate::detail::__rust_thunk___ZN6mixinseqERK10ComparableS2_(self, rhs) }
    }
}

impl PartialOrd for crate::Comparable {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        if self == other {
            return Some(core::cmp::Ordering::Equal);
        }
        if self < other {
            return Some(core::cmp::Ordering::Less);
        }
        if other < self {
            return Some(core::cmp::Ordering::Greater);
        }
        None
    }
    #[inline(always)]
    fn lt<'lhs, 'rhs>(&'lhs self, rhs: &'rhs Self) -> bool {
        unsafe { crate::detail::__rust_thunk___ZN6mixinsltERK10ComparableS2_(self, rhs) }
    }
}

pub mod ns {
    #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
    #[cfi_encoding = "N2ns19NamespacedEquatableE"]
    #[repr(C, align(4))]
    ///CRUBIT_ANNOTATE: cpp_type=ns :: NamespacedEquatable
    ///CRUBIT_ANNOTATE: cpp_move_constructible=
    pub struct NamespacedEquatable {
        pub value: ::ffi_11::c_int,
    }
    impl !Send for NamespacedEquatable {}
    impl !Sync for NamespacedEquatable {}
    unsafe impl ::cxx::ExternType for NamespacedEquatable {
        type Id = ::cxx::type_id!("ns :: NamespacedEquatable");
        type Kind = ::cxx::kind::Trivial;
    }

    impl Default for NamespacedEquatable {
        #[inline(always)]
        fn default() -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk___ZN2ns19NamespacedEquatableC1Ev(
                    &raw mut tmp as *mut _,
                );
                tmp.assume_init()
            }
        }
    }

    impl PartialEq for crate::ns::NamespacedEquatable {
        #[inline(always)]
        fn eq<'lhs, 'rhs>(&'lhs self, rhs: &'rhs Self) -> bool {
            unsafe {
                crate::detail::__rust_thunk___ZN6mixinseqERKN2ns19NamespacedEquatableES3_(self, rhs)
            }
        }
    }
}

// namespace ns

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "5Outer"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=Outer
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct Outer {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for Outer {}
impl !Sync for Outer {}
unsafe impl ::cxx::ExternType for Outer {
    type Id = ::cxx::type_id!("Outer");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for Outer {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN5OuterC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

pub mod outer {
    #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
    #[cfi_encoding = "N5Outer15NestedEquatableE"]
    #[repr(C, align(4))]
    ///CRUBIT_ANNOTATE: cpp_type=Outer :: NestedEquatable
    ///CRUBIT_ANNOTATE: cpp_move_constructible=
    pub struct NestedEquatable {
        pub value: ::ffi_11::c_int,
    }
    impl !Send for NestedEquatable {}
    impl !Sync for NestedEquatable {}
    unsafe impl ::cxx::ExternType for NestedEquatable {
        type Id = ::cxx::type_id!("Outer :: NestedEquatable");
        type Kind = ::cxx::kind::Trivial;
    }

    impl Default for NestedEquatable {
        #[inline(always)]
        fn default() -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk___ZN5Outer15NestedEquatableC1Ev(&raw mut tmp as *mut _);
                tmp.assume_init()
            }
        }
    }

    impl PartialEq for crate::outer::NestedEquatable {
        #[inline(always)]
        fn eq<'lhs, 'rhs>(&'lhs self, rhs: &'rhs Self) -> bool {
            unsafe {
                crate::detail::__rust_thunk___ZN6mixinseqERKN5Outer15NestedEquatableES3_(self, rhs)
            }
        }
    }
}

/// Types that aren't `Unpin` can inherit operators as well.
#[::ctor::recursively_pinned(PinnedDrop)]
#[cfi_encoding = "19NonTrivialEquatable"]
#[repr(C, align(4))]
///CRUBIT_ANNOTATE: cpp_type=NonTrivialEquatable
pub struct NonTrivialEquatable {
    pub value: ::ffi_11::c_int,
}
impl !Send for NonTrivialEquatable {}
impl !Sync for NonTrivialEquatable {}
unsafe impl ::cxx::ExternType for NonTrivialEquatable {
    type Id = ::cxx::type_id!("NonTrivialEquatable");
    type Kind = ::cxx::kind::Opaque;
}

impl ::ctor::CtorNew<()> for NonTrivialEquatable {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                let () = args;
                crate::detail::__rust_thunk___ZN19NonTrivialEquatableC1Ev(
                    __crubit_dest as *mut ::core::ffi::c_void,
                );
            })
        }
    }
}

impl<'__param_0> ::ctor::CtorNew<&'__param_0 Self> for NonTrivialEquatable {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__param_0>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: &'__param_0 Self) -> Self::CtorType {
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                let mut __param_0 = args;
                crate::detail::__rust_thunk___ZN19NonTrivialEquatableC1ERKS_(
                    __crubit_dest as *mut ::core::ffi::c_void,
                    __param_0,
                );
            })
        }
    }
}
impl<'__param_0> ::ctor::CtorNew<(&'__param_0 Self,)> for NonTrivialEquatable {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__param_0>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (&'__param_0 Self,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'__param_0 Self>>::ctor_new(arg)
    }
}

impl<'__param_0> ::ctor::Assign<&'__param_0 Self> for NonTrivialEquatable {
    #[inline(always)]
    fn assign<'__this>(self: ::core::pin::Pin<&'__this mut Self>, __param_0: &'__param_0 Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN19NonTrivialEquatableaSERKS_(self, __param_0);
        }
    }
}

impl ::ctor::PinnedDrop for NonTrivialEquatable {
    #[inline(always)]
    unsafe fn pinned_drop<'__this>(self: ::core::pin::Pin<&'__this mut Self>) {
        unsafe { crate::detail::__rust_thunk___ZN19NonTrivialEquatableD1Ev(self) }
    }
}

// NOLINT(modernize-use-equals-default)

impl PartialEq for crate::NonTrivialEquatable {
    #[inline(always)]
    fn eq<'lhs, 'rhs>(&'lhs self, rhs: &'rhs Self) -> bool {
        unsafe { crate::detail::__rust_thunk___ZN6mixinseqERK19NonTrivialEquatableS2_(self, rhs) }
    }
}

/// The operators of `EqualityMixin<Equatable>` compare `Equatable`s, not
/// `NotEquatable`s, so they don't get bindings here.
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "12NotEquatable"]
#[repr(C, align(4))]
///CRUBIT_ANNOTATE: cpp_type=NotEquatable
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct NotEquatable {
    pub value: ::ffi_11::c_int,
}
impl !Send for NotEquatable {}
impl !Sync for NotEquatable {}
unsafe impl ::cxx::ExternType for NotEquatable {
    type Id = ::cxx::type_id!("NotEquatable");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for NotEquatable {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN12NotEquatableC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

/// Inherited operators whose first parameter is not the derived class don't get
/// bindings.
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "12IntEquatable"]
#[repr(C, align(4))]
///CRUBIT_ANNOTATE: cpp_type=IntEquatable
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct IntEquatable {
    pub value: ::ffi_11::c_int,
}
impl !Send for IntEquatable {}
impl !Sync for IntEquatable {}
unsafe impl ::cxx::ExternType for IntEquatable {
    type Id = ::cxx::type_id!("IntEquatable");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for IntEquatable {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN12IntEquatableC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN9EquatableC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN6mixinseqERK9EquatableS2_<'lhs, 'rhs>(
            lhs: &'lhs crate::Equatable,
            rhs: &'rhs crate::Equatable,
        ) -> bool;
        pub(crate) unsafe fn __rust_thunk___ZN10ComparableC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN6mixinseqERK10ComparableS2_<'lhs, 'rhs>(
            lhs: &'lhs crate::Comparable,
            rhs: &'rhs crate::Comparable,
        ) -> bool;
        pub(crate) unsafe fn __rust_thunk___ZN6mixinsltERK10ComparableS2_<'lhs, 'rhs>(
            lhs: &'lhs crate::Comparable,
            rhs: &'rhs crate::Comparable,
        ) -> bool;
        pub(crate) unsafe fn __rust_thunk___ZN2ns19NamespacedEquatableC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6mixinseqERKN2ns19NamespacedEquatableES3_<
            'lhs,
            'rhs,
        >(
            lhs: &'lhs crate::ns::NamespacedEquatable,
            rhs: &'rhs crate::ns::NamespacedEquatable,
        ) -> bool;
        pub(crate) unsafe fn __rust_thunk___ZN5OuterC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN5Outer15NestedEquatableC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6mixinseqERKN5Outer15NestedEquatableES3_<'lhs, 'rhs>(
            lhs: &'lhs crate::outer::NestedEquatable,
            rhs: &'rhs crate::outer::NestedEquatable,
        ) -> bool;
        pub(crate) unsafe fn __rust_thunk___ZN19NonTrivialEquatableC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN19NonTrivialEquatableC1ERKS_<'__param_0>(
            __this: *mut ::core::ffi::c_void,
            __param_0: &'__param_0 crate::NonTrivialEquatable,
        );
        pub(crate) unsafe fn __rust_thunk___ZN19NonTrivialEquatableaSERKS_<'__param_0, '__this>(
            __this: ::core::pin::Pin<&'__this mut crate::NonTrivialEquatable>,
            __param_0: &'__param_0 crate::NonTrivialEquatable,
        ) -> ::core::pin::Pin<&'__this mut crate::NonTrivialEquatable>;
        pub(crate) unsafe fn __rust_thunk___ZN19NonTrivialEquatableD1Ev<'__this>(
            __this: ::core::pin::Pin<&'__this mut crate::NonTrivialEquatable>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6mixinseqERK19NonTrivialEquatableS2_<'lhs, 'rhs>(
            lhs: &'lhs crate::NonTrivialEquatable,
            rhs: &'rhs crate::NonTrivialEquatable,
        ) -> bool;
        pub(crate) unsafe fn __rust_thunk___ZN12NotEquatableC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN12IntEquatableC1Ev(__this: *mut ::core::ffi::c_void);
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::Equatable>() == 4);
    assert!(::core::mem::align_of::<crate::Equatable>() == 4);
    static_assertions::assert_impl_all!(crate::Equatable: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::Equatable: Drop);
    assert!(::core::mem::offset_of!(crate::Equatable, value) == 0);
    assert!(::core::mem::size_of::<crate::Comparable>() == 4);
    assert!(::core::mem::align_of::<crate::Comparable>() == 4);
    static_assertions::assert_impl_all!(crate::Comparable: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::Comparable: Drop);
    assert!(::core::mem::offset_of!(crate::Comparable, value) == 0);
    assert!(::core::mem::size_of::<crate::ns::NamespacedEquatable>() == 4);
    assert!(::core::mem::align_of::<crate::ns::NamespacedEquatable>() == 4);
    static_assertions::assert_impl_all!(crate::ns::NamespacedEquatable: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::ns::NamespacedEquatable: Drop);
    assert!(::core::mem::offset_of!(crate::ns::NamespacedEquatable, value) == 0);
    assert!(::core::mem::size_of::<crate::outer::NestedEquatable>() == 4);
    assert!(::core::mem::align_of::<crate::outer::NestedEquatable>() == 4);
    static_assertions::assert_impl_all!(crate::outer::NestedEquatable: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::outer::NestedEquatable: Drop);
    assert!(::core::mem::offset_of!(crate::outer::NestedEquatable, value) == 0);
    assert!(::core::mem::size_of::<crate::Outer>() == 1);
    assert!(::core::mem::align_of::<crate::Outer>() == 1);
    static_assertions::assert_impl_all!(crate::Outer: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::Outer: Drop);

    assert!(::core::mem::size_of::<crate::NonTrivialEquatable>() == 4);
    assert!(::core::mem::align_of::<crate::NonTrivialEquatable>() == 4);
    static_assertions::assert_impl_all!(crate::NonTrivialEquatable: Drop);
    static_assertions::assert_not_impl_any!(crate::NonTrivialEquatable: Copy);
    assert!(::core::mem::offset_of!(crate::NonTrivialEquatable, value) == 0);
    static_assertions::assert_impl_all!(::ffi_11::c_int: Copy);
    assert!(::core::mem::size_of::<crate::NotEquatable>() == 4);
    assert!(::core::mem::align_of::<crate::NotEquatable>() == 4);
    static_assertions::assert_impl_all!(crate::NotEquatable: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::NotEquatable: Drop);
    assert!(::core::mem::offset_of!(crate::NotEquatable, value) == 0);
    assert!(::core::mem::size_of::<crate::IntEquatable>() == 4);
    assert!(::core::mem::align_of::<crate::IntEquatable>() == 4);
    static_assertions::assert_impl_all!(crate::IntEquatable: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::IntEquatable: Drop);
    assert!(::core::mem::offset_of!(crate::IntEquatable, value) == 0);
};
