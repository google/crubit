// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Internal helper traits and types for `std::variant`.

#[doc(hidden)]
pub mod variant_internal {
    use crate::std::coproduct::{CNil, Coproduct};
    use crate::std::variant_enums::{
        Variant1, Variant2, Variant3, Variant4, Variant5, Variant6, Variant7, Variant8,
    };
    use crate::std::{trivial_variant, variant};

    use core::fmt::{self, Debug, Formatter};
    use core::mem::ManuallyDrop;
    use core::pin::Pin;

    pub const VARIANT_NPOS: usize = usize::MAX;
    pub type RawVariantIndex =
        crate::crubit_cc_std_internal::conversion_function_helpers::VariantIndex;
    pub(crate) const RAW_VARIANT_NPOS: RawVariantIndex = RawVariantIndex::MAX;

    pub trait RawUnionOps {
        fn new_raw_uninit() -> Self;
        unsafe fn drop_active(&mut self, index: RawVariantIndex);
    }

    pub trait RawUnionClone: RawUnionOps {
        unsafe fn clone_active(&self, index: RawVariantIndex) -> Self;
    }
    pub trait RawUnionPartialEq {
        unsafe fn eq_active(&self, other: &Self, index: RawVariantIndex) -> bool;
    }
    pub trait RawUnionEq: RawUnionPartialEq {}
    pub trait RawUnionDebug {
        unsafe fn fmt_active(&self, index: RawVariantIndex, f: &mut Formatter<'_>) -> fmt::Result;
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub union RawVariantNil {
        pub(crate) __uninit: [u8; 0],
    }
    #[repr(C)]
    pub union RawVariantUnion<H, Tail> {
        pub(crate) __uninit: u8,
        pub(crate) __head: ManuallyDrop<H>,
        pub(crate) __tail: ManuallyDrop<Tail>,
    }
    impl<H: Copy, Tail: Copy> Copy for RawVariantUnion<H, Tail> {}
    impl<H: Copy, Tail: Copy> Clone for RawVariantUnion<H, Tail> {
        fn clone(&self) -> Self {
            *self
        }
    }

    impl RawUnionOps for RawVariantNil {
        fn new_raw_uninit() -> Self {
            Self { __uninit: [] }
        }
        unsafe fn drop_active(&mut self, _index: RawVariantIndex) {}
    }
    impl<H, Tail: RawUnionOps> RawUnionOps for RawVariantUnion<H, Tail> {
        fn new_raw_uninit() -> Self {
            Self { __uninit: 0 }
        }
        unsafe fn drop_active(&mut self, index: RawVariantIndex) {
            unsafe {
                if index == 0 {
                    ManuallyDrop::drop(&mut self.__head);
                } else {
                    self.__tail.drop_active(index - 1);
                }
            }
        }
    }

    impl RawUnionClone for RawVariantNil {
        unsafe fn clone_active(&self, _index: RawVariantIndex) -> Self {
            Self { __uninit: [] }
        }
    }
    impl<H: Clone, Tail: RawUnionClone> RawUnionClone for RawVariantUnion<H, Tail> {
        unsafe fn clone_active(&self, index: RawVariantIndex) -> Self {
            unsafe {
                if index == 0 {
                    Self { __head: ManuallyDrop::new((*self.__head).clone()) }
                } else {
                    Self { __tail: ManuallyDrop::new(self.__tail.clone_active(index - 1)) }
                }
            }
        }
    }

    impl RawUnionPartialEq for RawVariantNil {
        unsafe fn eq_active(&self, _other: &Self, _index: RawVariantIndex) -> bool {
            true
        }
    }
    impl<H: PartialEq, Tail: RawUnionPartialEq> RawUnionPartialEq for RawVariantUnion<H, Tail> {
        unsafe fn eq_active(&self, other: &Self, index: RawVariantIndex) -> bool {
            unsafe {
                if index == 0 {
                    *self.__head == *other.__head
                } else {
                    self.__tail.eq_active(&other.__tail, index - 1)
                }
            }
        }
    }
    impl RawUnionEq for RawVariantNil {}
    impl<H: Eq, Tail: RawUnionEq> RawUnionEq for RawVariantUnion<H, Tail> {}

    impl RawUnionDebug for RawVariantNil {
        unsafe fn fmt_active(&self, _index: RawVariantIndex, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "CNil")
        }
    }
    impl<H: Debug, Tail: RawUnionDebug> RawUnionDebug for RawVariantUnion<H, Tail> {
        unsafe fn fmt_active(&self, index: RawVariantIndex, f: &mut Formatter<'_>) -> fmt::Result {
            unsafe {
                if index == 0 {
                    Debug::fmt(&*self.__head, f)
                } else {
                    self.__tail.fmt_active(index - 1, f)
                }
            }
        }
    }

    pub trait VariantTypes {
        type RawUnion: RawUnionOps;
        type Owned;
        type Ref<'a>
        where
            Self: 'a;
        type Mut<'a>
        where
            Self: 'a;
        type PinMut<'a>
        where
            Self: 'a;
        const LEN: usize;

        fn new_raw_uninit() -> Self::RawUnion;
        unsafe fn drop_active(raw: &mut Self::RawUnion, index: RawVariantIndex);
        unsafe fn as_ref<'a>(raw: &'a Self::RawUnion, index: RawVariantIndex) -> Self::Ref<'a>;
        unsafe fn as_mut<'a>(raw: &'a mut Self::RawUnion, index: RawVariantIndex) -> Self::Mut<'a>;
        unsafe fn as_pin_mut<'a>(
            raw: &'a mut Self::RawUnion,
            index: RawVariantIndex,
        ) -> Self::PinMut<'a>;
        unsafe fn into_owned(raw: Self::RawUnion, index: RawVariantIndex) -> Self::Owned;
        fn from_owned(owned: Self::Owned) -> (Self::RawUnion, RawVariantIndex);
    }

    pub trait TrivialVariantTypes: VariantTypes<RawUnion: Copy> {}
    impl<Ts: VariantTypes<RawUnion: Copy>> TrivialVariantTypes for Ts {}

    pub struct VariantAtCtor<Ts, const I: usize, C>(pub C, pub core::marker::PhantomData<Ts>);
    pub struct VariantCtor<Ts, I, C>(pub C, pub core::marker::PhantomData<(Ts, I)>);

    pub trait RawUnionIndex<const I: usize>: Sized {
        type Element;
        unsafe fn raw_ptr(this: *const Self) -> *const Self::Element;
        unsafe fn raw_ptr_mut(this: *mut Self) -> *mut Self::Element;
        unsafe fn get_unchecked(this: &Self) -> &Self::Element;
        unsafe fn get_unchecked_mut(this: &mut Self) -> &mut Self::Element;
    }

    pub struct Here;
    pub struct There<T>(core::marker::PhantomData<T>);

    macro_rules! impl_raw_union_index {
    ($($n:literal),*) => {
        $(
            impl<H, Tail> RawUnionIndex<$n> for RawVariantUnion<H, Tail>
            where
                Tail: RawUnionIndex<{ $n - 1 }>,
            {
                type Element = <Tail as RawUnionIndex<{ $n - 1 }>>::Element;
                #[inline]
                unsafe fn raw_ptr(this: *const Self) -> *const Self::Element {
                    // SAFETY: `RawVariantUnion` is `#[repr(C)]` and `ManuallyDrop<Tail>` is
                    // `#[repr(transparent)]`, so `Tail` is at byte offset 0 of `Self`.
                    unsafe { <Tail as RawUnionIndex<{ $n - 1 }>>::raw_ptr(this.cast::<Tail>()) }
                }
                #[inline]
                unsafe fn raw_ptr_mut(this: *mut Self) -> *mut Self::Element {
                    // SAFETY: `RawVariantUnion` is `#[repr(C)]` and `ManuallyDrop<Tail>` is
                    // `#[repr(transparent)]`, so `Tail` is at byte offset 0 of `Self`.
                    unsafe { <Tail as RawUnionIndex<{ $n - 1 }>>::raw_ptr_mut(this.cast::<Tail>()) }
                }
                #[inline]
                unsafe fn get_unchecked(this: &Self) -> &Self::Element {
                    // SAFETY: Caller guarantees alternative `$n` is initialized and active.
                    unsafe { &*<Self as RawUnionIndex<$n>>::raw_ptr(this) }
                }
                #[inline]
                unsafe fn get_unchecked_mut(this: &mut Self) -> &mut Self::Element {
                    // SAFETY: Caller guarantees alternative `$n` is initialized and active.
                    unsafe { &mut *<Self as RawUnionIndex<$n>>::raw_ptr_mut(this) }
                }
            }
        )*
    };}

    impl<H, Tail> RawUnionIndex<0> for RawVariantUnion<H, Tail> {
        type Element = H;
        #[inline]
        unsafe fn raw_ptr(this: *const Self) -> *const Self::Element {
            this.cast::<H>()
        }
        #[inline]
        unsafe fn raw_ptr_mut(this: *mut Self) -> *mut Self::Element {
            this.cast::<H>()
        }
        #[inline]
        unsafe fn get_unchecked(this: &Self) -> &Self::Element {
            // SAFETY: Caller guarantees alternative 0 (`__head`) is initialized and active, and
            // `RawVariantUnion` is `#[repr(C)]` with `ManuallyDrop<H>` at offset 0.
            unsafe { &*<Self as RawUnionIndex<0>>::raw_ptr(this) }
        }
        #[inline]
        unsafe fn get_unchecked_mut(this: &mut Self) -> &mut Self::Element {
            // SAFETY: Caller guarantees alternative 0 (`__head`) is initialized and active, and
            // `RawVariantUnion` is `#[repr(C)]` with `ManuallyDrop<H>` at offset 0.
            unsafe { &mut *<Self as RawUnionIndex<0>>::raw_ptr_mut(this) }
        }
    }

    impl_raw_union_index!(
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
        26, 27, 28, 29, 30, 31
    );

    pub trait VariantAlternative<T, I>: Sized {
        const INDEX: RawVariantIndex;
        unsafe fn raw_ptr(this: *const Self) -> *const T;
        unsafe fn raw_ptr_mut(this: *mut Self) -> *mut T;
        unsafe fn get_unchecked(this: &Self) -> &T;
        unsafe fn get_unchecked_mut(this: &mut Self) -> &mut T;
        fn from_element(val: T) -> Self;
    }

    impl<H, Tail> VariantAlternative<H, Here> for RawVariantUnion<H, Tail> {
        const INDEX: RawVariantIndex = 0;
        #[inline]
        unsafe fn raw_ptr(this: *const Self) -> *const H {
            this.cast::<H>()
        }
        #[inline]
        unsafe fn raw_ptr_mut(this: *mut Self) -> *mut H {
            this.cast::<H>()
        }
        #[inline]
        unsafe fn get_unchecked(this: &Self) -> &H {
            // SAFETY: Caller guarantees alternative `H` (`__head`) is initialized and active, and
            // `RawVariantUnion` is `#[repr(C)]` with `ManuallyDrop<H>` at offset 0.
            unsafe { &*<Self as VariantAlternative<H, Here>>::raw_ptr(this) }
        }
        #[inline]
        unsafe fn get_unchecked_mut(this: &mut Self) -> &mut H {
            // SAFETY: Caller guarantees alternative `H` (`__head`) is initialized and active, and
            // `RawVariantUnion` is `#[repr(C)]` with `ManuallyDrop<H>` at offset 0.
            unsafe { &mut *<Self as VariantAlternative<H, Here>>::raw_ptr_mut(this) }
        }
        #[inline]
        fn from_element(val: H) -> Self {
            Self { __head: ManuallyDrop::new(val) }
        }
    }

    impl<T, Idx, H, Tail> VariantAlternative<T, There<Idx>> for RawVariantUnion<H, Tail>
    where
        Tail: VariantAlternative<T, Idx>,
    {
        const INDEX: RawVariantIndex = 1 + Tail::INDEX;
        #[inline]
        unsafe fn raw_ptr(this: *const Self) -> *const T {
            // SAFETY: `RawVariantUnion` is `#[repr(C)]` and `ManuallyDrop<Tail>` is
            // `#[repr(transparent)]`, so `Tail` is at byte offset 0 of `Self`.
            unsafe { <Tail as VariantAlternative<T, Idx>>::raw_ptr(this.cast::<Tail>()) }
        }
        #[inline]
        unsafe fn raw_ptr_mut(this: *mut Self) -> *mut T {
            // SAFETY: `RawVariantUnion` is `#[repr(C)]` and `ManuallyDrop<Tail>` is
            // `#[repr(transparent)]`, so `Tail` is at byte offset 0 of `Self`.
            unsafe { <Tail as VariantAlternative<T, Idx>>::raw_ptr_mut(this.cast::<Tail>()) }
        }
        #[inline]
        unsafe fn get_unchecked(this: &Self) -> &T {
            // SAFETY: Caller guarantees the target alternative in `Tail` is initialized and active.
            unsafe { &*<Self as VariantAlternative<T, There<Idx>>>::raw_ptr(this) }
        }
        #[inline]
        unsafe fn get_unchecked_mut(this: &mut Self) -> &mut T {
            // SAFETY: Caller guarantees the target alternative in `Tail` is initialized and active.
            unsafe { &mut *<Self as VariantAlternative<T, There<Idx>>>::raw_ptr_mut(this) }
        }
        #[inline]
        fn from_element(val: T) -> Self {
            Self { __tail: ManuallyDrop::new(Tail::from_element(val)) }
        }
    }

    macro_rules! impl_tuple_variant {
    ($tuple:ident, $($T:ident, $i:literal, $V:ident),*) => {
        impl<$($T),*> VariantTypes for ($($T,)*) {
            type RawUnion = impl_tuple_variant!(@raw $($T),*);
            type Owned = $tuple<$($T,)*>;
            type Ref<'a> = $tuple<$(&'a $T,)*> where $($T: 'a),*;
            type Mut<'a> = $tuple<$(&'a mut $T,)*> where $($T: 'a),*;
            type PinMut<'a> = $tuple<$(Pin<&'a mut $T>,)*> where $($T: 'a),*;
            const LEN: usize = [$($i),*].len();

            #[inline]
            fn new_raw_uninit() -> Self::RawUnion {
                <Self::RawUnion as RawUnionOps>::new_raw_uninit()
            }
            #[inline]
            unsafe fn drop_active(raw: &mut Self::RawUnion, index: RawVariantIndex) {
                unsafe { raw.drop_active(index) }
            }
            // SAFETY for the pointer casts below: `Self::RawUnion` is a nested `#[repr(C)]` union
            // (`RawVariantUnion<T0, RawVariantUnion<T1, ...>>`), so every alternative `$T` lives
            // at byte offset 0 within `Self::RawUnion`. Casting the union pointer directly avoids
            // recursive `RawUnionIndex` trait solving and MIR inlining across every match arm.
            #[inline]
            unsafe fn as_ref<'a>(raw: &'a Self::RawUnion, index: RawVariantIndex) -> Self::Ref<'a> {
                match index {
                    $($i => $tuple::$V(unsafe { &*(raw as *const Self::RawUnion as *const $T) }),)*
                    _ => unsafe { core::hint::unreachable_unchecked() }
                }
            }
            #[inline]
            unsafe fn as_mut<'a>(
                raw: &'a mut Self::RawUnion,
                index: RawVariantIndex,
            ) -> Self::Mut<'a> {
                match index {
                    $($i => $tuple::$V(unsafe { &mut *(raw as *mut Self::RawUnion as *mut $T) }),)*
                    _ => unsafe { core::hint::unreachable_unchecked() }
                }
            }
            #[inline]
            unsafe fn as_pin_mut<'a>(
                raw: &'a mut Self::RawUnion,
                index: RawVariantIndex,
            ) -> Self::PinMut<'a> {
                match index {
                    $($i => $tuple::$V(unsafe { Pin::new_unchecked(&mut *(raw as *mut Self::RawUnion as *mut $T)) }),)*
                    _ => unsafe { core::hint::unreachable_unchecked() }
                }
            }
            #[inline]
            unsafe fn into_owned(raw: Self::RawUnion, index: RawVariantIndex) -> Self::Owned {
                match index {
                    $($i => $tuple::$V(unsafe { core::ptr::read(&raw as *const Self::RawUnion as *const $T) }),)*
                    _ => unsafe { core::hint::unreachable_unchecked() }
                }
            }
            #[inline]
            fn from_owned(owned: Self::Owned) -> (Self::RawUnion, RawVariantIndex) {
                match owned {
                    $($tuple::$V(val) => {
                        let mut raw = Self::new_raw_uninit();
                        unsafe { core::ptr::write(&mut raw as *mut Self::RawUnion as *mut $T, val); }
                        (raw, $i)
                    })*
                }
            }
        }

        impl<$($T),*> From<$tuple<$($T),*>> for crate::std::variant<($($T,)*)>
        where ($($T,)*): VariantTypes<Owned = $tuple<$($T),*>> {
            fn from(val: $tuple<$($T),*>) -> Self {
                let (raw, index) = <($($T,)*) as VariantTypes>::from_owned(val);
                Self { raw, index }
            }
        }
        impl<$($T),*> From<$tuple<$($T),*>> for crate::std::trivial_variant<($($T,)*)>
        where ($($T,)*): TrivialVariantTypes<Owned = $tuple<$($T),*>> {
            fn from(val: $tuple<$($T),*>) -> Self {
                let (raw, index) = <($($T,)*) as VariantTypes>::from_owned(val);
                Self { raw, index }
            }
        }
    };
    (@raw $T0:ident) => { crate::std::variant_internal::RawVariantUnion<$T0, crate::std::variant_internal::RawVariantNil> };
    (@raw $T0:ident, $($T:ident),+) => { crate::std::variant_internal::RawVariantUnion<$T0, impl_tuple_variant!(@raw $($T),+)> };}

    impl_tuple_variant!(Variant1, T0, 0, V0);
    impl_tuple_variant!(Variant2, T0, 0, V0, T1, 1, V1);
    impl_tuple_variant!(Variant3, T0, 0, V0, T1, 1, V1, T2, 2, V2);
    impl_tuple_variant!(Variant4, T0, 0, V0, T1, 1, V1, T2, 2, V2, T3, 3, V3);
    impl_tuple_variant!(Variant5, T0, 0, V0, T1, 1, V1, T2, 2, V2, T3, 3, V3, T4, 4, V4);
    impl_tuple_variant!(Variant6, T0, 0, V0, T1, 1, V1, T2, 2, V2, T3, 3, V3, T4, 4, V4, T5, 5, V5);
    impl_tuple_variant!(
        Variant7, T0, 0, V0, T1, 1, V1, T2, 2, V2, T3, 3, V3, T4, 4, V4, T5, 5, V5, T6, 6, V6
    );
    impl_tuple_variant!(
        Variant8, T0, 0, V0, T1, 1, V1, T2, 2, V2, T3, 3, V3, T4, 4, V4, T5, 5, V5, T6, 6, V6, T7,
        7, V7
    );

    impl VariantTypes for CNil {
        type RawUnion = RawVariantNil;
        type Owned = core::convert::Infallible;
        type Ref<'a> = core::convert::Infallible;
        type Mut<'a> = core::convert::Infallible;
        type PinMut<'a> = core::convert::Infallible;
        const LEN: usize = 0;
        #[inline]
        fn new_raw_uninit() -> Self::RawUnion {
            <Self::RawUnion as RawUnionOps>::new_raw_uninit()
        }
        #[inline]
        unsafe fn drop_active(_raw: &mut Self::RawUnion, _index: RawVariantIndex) {}
        #[inline]
        unsafe fn as_ref<'a>(_raw: &'a Self::RawUnion, _index: RawVariantIndex) -> Self::Ref<'a> {
            unsafe { core::hint::unreachable_unchecked() }
        }
        #[inline]
        unsafe fn as_mut<'a>(
            _raw: &'a mut Self::RawUnion,
            _index: RawVariantIndex,
        ) -> Self::Mut<'a> {
            unsafe { core::hint::unreachable_unchecked() }
        }
        #[inline]
        unsafe fn as_pin_mut<'a>(
            _raw: &'a mut Self::RawUnion,
            _index: RawVariantIndex,
        ) -> Self::PinMut<'a> {
            unsafe { core::hint::unreachable_unchecked() }
        }
        #[inline]
        unsafe fn into_owned(_raw: Self::RawUnion, _index: RawVariantIndex) -> Self::Owned {
            unsafe { core::hint::unreachable_unchecked() }
        }
        #[inline]
        fn from_owned(owned: Self::Owned) -> (Self::RawUnion, RawVariantIndex) {
            match owned {}
        }
    }

    impl<H, T: VariantTypes> VariantTypes for Coproduct<H, T> {
        type RawUnion = RawVariantUnion<H, T::RawUnion>;
        type Owned = Coproduct<H, T::Owned>;
        type Ref<'a>
            = Coproduct<&'a H, T::Ref<'a>>
        where
            H: 'a,
            T: 'a;
        type Mut<'a>
            = Coproduct<&'a mut H, T::Mut<'a>>
        where
            H: 'a,
            T: 'a;
        type PinMut<'a>
            = Coproduct<core::pin::Pin<&'a mut H>, T::PinMut<'a>>
        where
            H: 'a,
            T: 'a;
        const LEN: usize = 1 + T::LEN;

        #[inline]
        fn new_raw_uninit() -> Self::RawUnion {
            <Self::RawUnion as RawUnionOps>::new_raw_uninit()
        }
        #[inline]
        unsafe fn drop_active(raw: &mut Self::RawUnion, index: RawVariantIndex) {
            unsafe {
                if index < 1 {
                    core::ptr::drop_in_place(&mut raw.__head as *mut ManuallyDrop<H> as *mut H)
                } else {
                    T::drop_active(&mut raw.__tail, index - 1)
                }
            }
        }
        #[inline]
        unsafe fn as_ref<'a>(raw: &'a Self::RawUnion, index: RawVariantIndex) -> Self::Ref<'a> {
            unsafe {
                if index < 1 {
                    Coproduct::Inl(&*(&raw.__head as *const ManuallyDrop<H> as *const H))
                } else {
                    Coproduct::Inr(T::as_ref(&raw.__tail, index - 1))
                }
            }
        }
        #[inline]
        unsafe fn as_mut<'a>(raw: &'a mut Self::RawUnion, index: RawVariantIndex) -> Self::Mut<'a> {
            unsafe {
                if index < 1 {
                    Coproduct::Inl(&mut *(&mut raw.__head as *mut ManuallyDrop<H> as *mut H))
                } else {
                    Coproduct::Inr(T::as_mut(&mut raw.__tail, index - 1))
                }
            }
        }
        #[inline]
        unsafe fn as_pin_mut<'a>(
            raw: &'a mut Self::RawUnion,
            index: RawVariantIndex,
        ) -> Self::PinMut<'a> {
            unsafe {
                if index < 1 {
                    Coproduct::Inl(core::pin::Pin::new_unchecked(
                        &mut *(&mut raw.__head as *mut ManuallyDrop<H> as *mut H),
                    ))
                } else {
                    Coproduct::Inr(T::as_pin_mut(&mut raw.__tail, index - 1))
                }
            }
        }
        #[inline]
        unsafe fn into_owned(raw: Self::RawUnion, index: RawVariantIndex) -> Self::Owned {
            unsafe {
                if index < 1 {
                    Coproduct::Inl(core::ptr::read((&raw const raw.__head).cast::<H>()))
                } else {
                    Coproduct::Inr(T::into_owned(
                        core::ptr::read((&raw const raw.__tail).cast::<T::RawUnion>()),
                        index - 1,
                    ))
                }
            }
        }
        #[inline]
        fn from_owned(owned: Self::Owned) -> (Self::RawUnion, RawVariantIndex) {
            match owned {
                Coproduct::Inl(h) => (RawVariantUnion { __head: ManuallyDrop::new(h) }, 0),
                Coproduct::Inr(t) => {
                    let (r, i) = T::from_owned(t);
                    (RawVariantUnion { __tail: ManuallyDrop::new(r) }, i + 1)
                }
            }
        }
    }

    #[macro_export]
    #[doc(hidden)]
    macro_rules! __crubit_variant_macro {
    ($T0:ty $(,)?) => { $crate::std::variant<($T0,)> };
    ($T0:ty, $T1:ty $(,)?) => { $crate::std::variant<($T0, $T1,)> };
    ($T0:ty, $T1:ty, $T2:ty $(,)?) => { $crate::std::variant<($T0, $T1, $T2,)> };
    ($T0:ty, $T1:ty, $T2:ty, $T3:ty $(,)?) => { $crate::std::variant<($T0, $T1, $T2, $T3,)> };
    ($T0:ty, $T1:ty, $T2:ty, $T3:ty, $T4:ty $(,)?) => { $crate::std::variant<($T0, $T1, $T2, $T3, $T4,)> };
    ($T0:ty, $T1:ty, $T2:ty, $T3:ty, $T4:ty, $T5:ty $(,)?) => { $crate::std::variant<($T0, $T1, $T2, $T3, $T4, $T5,)> };
    ($T0:ty, $T1:ty, $T2:ty, $T3:ty, $T4:ty, $T5:ty, $T6:ty $(,)?) => { $crate::std::variant<($T0, $T1, $T2, $T3, $T4, $T5, $T6,)> };
    ($T0:ty, $T1:ty, $T2:ty, $T3:ty, $T4:ty, $T5:ty, $T6:ty, $T7:ty $(,)?) => { $crate::std::variant<($T0, $T1, $T2, $T3, $T4, $T5, $T6, $T7,)> };
    ($T0:ty, $($T:ty),+ $(,)?) => { $crate::std::variant<$crate::std::coproduct::Coproduct<$T0, $crate::__crubit_variant_coproduct_macro!($($T),+)>> };
}

    #[macro_export]
    #[doc(hidden)]
    macro_rules! __crubit_variant_coproduct_macro {
    ($T0:ty $(,)?) => { $crate::std::coproduct::Coproduct<$T0, $crate::std::coproduct::CNil> };
    ($T0:ty, $($T:ty),+ $(,)?) => { $crate::std::coproduct::Coproduct<$T0, $crate::__crubit_variant_coproduct_macro!($($T),+)> };
}

    #[macro_export]
    #[doc(hidden)]
    macro_rules! __crubit_trivial_variant_macro {
    ($T0:ty $(,)?) => { $crate::std::trivial_variant<($T0,)> };
    ($T0:ty, $T1:ty $(,)?) => { $crate::std::trivial_variant<($T0, $T1,)> };
    ($T0:ty, $T1:ty, $T2:ty $(,)?) => { $crate::std::trivial_variant<($T0, $T1, $T2,)> };
    ($T0:ty, $T1:ty, $T2:ty, $T3:ty $(,)?) => { $crate::std::trivial_variant<($T0, $T1, $T2, $T3,)> };
    ($T0:ty, $T1:ty, $T2:ty, $T3:ty, $T4:ty $(,)?) => { $crate::std::trivial_variant<($T0, $T1, $T2, $T3, $T4,)> };
    ($T0:ty, $T1:ty, $T2:ty, $T3:ty, $T4:ty, $T5:ty $(,)?) => { $crate::std::trivial_variant<($T0, $T1, $T2, $T3, $T4, $T5,)> };
    ($T0:ty, $T1:ty, $T2:ty, $T3:ty, $T4:ty, $T5:ty, $T6:ty $(,)?) => { $crate::std::trivial_variant<($T0, $T1, $T2, $T3, $T4, $T5, $T6,)> };
    ($T0:ty, $T1:ty, $T2:ty, $T3:ty, $T4:ty, $T5:ty, $T6:ty, $T7:ty $(,)?) => { $crate::std::trivial_variant<($T0, $T1, $T2, $T3, $T4, $T5, $T6, $T7,)> };
    ($T0:ty, $($T:ty),+ $(,)?) => { $crate::std::trivial_variant<$crate::std::coproduct::Coproduct<$T0, $crate::__crubit_variant_coproduct_macro!($($T),+)>> };
}
}
