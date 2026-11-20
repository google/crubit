// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! `std::variant` C++ interoperability.

use crate::std::variant_internal::{
    RawUnionClone, RawUnionDebug, RawUnionEq, RawUnionIndex, RawUnionPartialEq, RawVariantIndex,
    TrivialVariantTypes, VariantAlternative, VariantAtCtor, VariantCtor, VariantTypes,
    RAW_VARIANT_NPOS, VARIANT_NPOS,
};

use core::fmt::{self, Debug, Formatter};
use core::pin::Pin;
use ctor::Ctor;

/// A C++ `std::variant` in Rust, when at least one alternative is not trivially destructible.
///
/// When all alternatives are trivially destructible (`Copy` in Rust), `std::variant` is bound as
/// [`trivial_variant`] instead. Use the [`variant!`] macro to write the type of a `variant` with
/// any number of alternatives: for up to 8 alternatives it expands to a tuple-parameterized
/// `variant<(T0, ..., Tn)>`, and for more than 8 alternatives it automatically falls back to
/// nested [`Coproduct`]($crate::std::coproduct::Coproduct) types.
///
/// # Examples
///
/// ```
/// use cpp_std::{variant, variant_enums::Variant2};
///
/// let mut v: variant![i32, f32] = variant::new(42i32);
/// assert!(v.holds_alternative::<i32, _>());
/// assert_eq!(v.get::<i32, _>(), Some(&42));
///
/// v.set(2.5f32);
/// assert_eq!(v.as_ref(), Variant2::V1(&2.5));
/// ```
#[crubit_annotate::cpp_layout_equivalent(
    cpp_type = "::std::variant<{Ts...}>",
    include_path = "<variant>"
)]
#[repr(C)]
pub struct variant<Ts: VariantTypes> {
    pub(crate) raw: Ts::RawUnion,
    pub(crate) index: RawVariantIndex,
}

impl<Ts: VariantTypes> Drop for variant<Ts> {
    fn drop(&mut self) {
        let prev_index = core::mem::replace(&mut self.index, RAW_VARIANT_NPOS);
        if prev_index != RAW_VARIANT_NPOS {
            // SAFETY: `prev_index != RAW_VARIANT_NPOS`, so alternative `prev_index` is active and
            // initialized. Setting `self.index` to `RAW_VARIANT_NPOS` first leaves `self` valueless
            // if the active alternative's destructor unwinds.
            unsafe { Ts::drop_active(&mut self.raw, prev_index) }
        }
    }
}

impl<Ts: VariantTypes> Clone for variant<Ts>
where
    Ts::RawUnion: RawUnionClone,
{
    fn clone(&self) -> Self {
        if self.index == RAW_VARIANT_NPOS {
            Self { raw: Ts::new_raw_uninit(), index: RAW_VARIANT_NPOS }
        } else {
            Self { raw: unsafe { self.raw.clone_active(self.index) }, index: self.index }
        }
    }
}

impl<Ts: VariantTypes> variant<Ts> {
    /// Returns the zero-based index of the alternative that is currently held by the variant.
    #[inline]
    pub fn index(&self) -> usize {
        if self.index == RAW_VARIANT_NPOS {
            VARIANT_NPOS
        } else {
            self.index as usize
        }
    }

    /// Returns `true` if and only if the variant holds no value (`valueless_by_exception` in C++).
    #[inline]
    #[doc(hidden)]
    pub fn valueless_by_exception(&self) -> bool {
        self.index == RAW_VARIANT_NPOS
    }

    #[inline(always)]
    fn check_valueless(&self) {
        if self.index == RAW_VARIANT_NPOS {
            panic!("std::variant is valueless_by_exception");
        }
    }

    /// Projects a shared reference to the variant into a Rust enum of shared references to its
    /// alternatives ([`Variant1`](crate::std::variant_enums::Variant1)..
    /// [`Variant8`](crate::std::variant_enums::Variant8) for tuple-parameterized variants, or
    /// [`Coproduct`]($crate::std::coproduct::Coproduct) for >8 alternatives).
    ///
    /// # Panics
    ///
    /// Panics if the variant is in the `valueless_by_exception` state.
    #[inline]
    pub fn as_ref(&self) -> Ts::Ref<'_> {
        self.check_valueless();
        unsafe { Ts::as_ref(&self.raw, self.index) }
    }

    /// Projects an exclusive reference to the variant into a Rust enum of mutable references to
    /// its alternatives.
    ///
    /// # Panics
    ///
    /// Panics if the variant is in the `valueless_by_exception` state.
    #[inline]
    pub fn as_mut(&mut self) -> Ts::Mut<'_> {
        self.check_valueless();
        unsafe { Ts::as_mut(&mut self.raw, self.index) }
    }

    /// Projects a pinned mutable reference to the variant into a Rust enum of pinned mutable
    /// references to its alternatives.
    ///
    /// # Panics
    ///
    /// Panics if the variant is in the `valueless_by_exception` state.
    #[inline]
    pub fn as_pin_mut(self: Pin<&mut Self>) -> Ts::PinMut<'_> {
        self.check_valueless();
        unsafe {
            let this = self.get_unchecked_mut();
            Ts::as_pin_mut(&mut this.raw, this.index)
        }
    }

    /// Consumes the variant and returns the active alternative wrapped in the corresponding Rust
    /// projection enum ([`Variant1`](crate::std::variant_enums::Variant1)..
    /// [`Variant8`](crate::std::variant_enums::Variant8) or
    /// [`Coproduct`]($crate::std::coproduct::Coproduct)).
    ///
    /// # Panics
    ///
    /// Panics if the variant is in the `valueless_by_exception` state.
    #[inline]
    pub fn into_enum(mut self) -> Ts::Owned {
        self.check_valueless();
        let raw = unsafe { core::ptr::read(&self.raw) };
        let index = self.index;
        self.index = RAW_VARIANT_NPOS;
        unsafe { Ts::into_owned(raw, index) }
    }

    /// Returns a shared reference to the alternative at zero-based index `I` if it is currently
    /// active, or `None` otherwise (similar to `std::get_if<I>` in C++).
    #[inline]
    pub fn get_at<const I: usize>(&self) -> Option<&<Ts::RawUnion as RawUnionIndex<I>>::Element>
    where
        Ts::RawUnion: RawUnionIndex<I>,
    {
        if self.index != I as RawVariantIndex {
            return None;
        }
        Some(unsafe { <Ts::RawUnion as RawUnionIndex<I>>::get_unchecked(&self.raw) })
    }

    /// Returns a mutable reference to the alternative at zero-based index `I` if it is currently
    /// active, or `None` otherwise (similar to `std::get_if<I>` in C++).
    #[inline]
    pub fn get_at_mut<const I: usize>(
        &mut self,
    ) -> Option<&mut <Ts::RawUnion as RawUnionIndex<I>>::Element>
    where
        Ts::RawUnion: RawUnionIndex<I>,
    {
        if self.index != I as RawVariantIndex {
            return None;
        }
        Some(unsafe { <Ts::RawUnion as RawUnionIndex<I>>::get_unchecked_mut(&mut self.raw) })
    }

    /// Returns a pinned mutable reference to the alternative at zero-based index `I` if it is
    /// currently active, or `None` otherwise.
    #[inline]
    pub fn get_at_pin_mut<const I: usize>(
        self: Pin<&mut Self>,
    ) -> Option<Pin<&mut <Ts::RawUnion as RawUnionIndex<I>>::Element>>
    where
        Ts::RawUnion: RawUnionIndex<I>,
    {
        if self.index != I as RawVariantIndex {
            return None;
        }
        unsafe {
            Some(Pin::new_unchecked(<Ts::RawUnion as RawUnionIndex<I>>::get_unchecked_mut(
                &mut self.get_unchecked_mut().raw,
            )))
        }
    }

    /// Returns `true` if the alternative at zero-based index `I` is currently active.
    #[inline]
    pub fn holds_alternative_at<const I: usize>(&self) -> bool
    where
        Ts::RawUnion: RawUnionIndex<I>,
    {
        self.index == I as RawVariantIndex
    }

    /// Creates a new `variant` holding `val` as the alternative at zero-based index `I`
    /// (similar to `std::variant<Ts...>(std::in_place_index<I>, val)` in C++).
    #[inline]
    pub fn new_at<const I: usize>(val: <Ts::RawUnion as RawUnionIndex<I>>::Element) -> Self
    where
        Ts::RawUnion: RawUnionIndex<I>,
    {
        Self {
            raw: {
                let mut raw = Ts::new_raw_uninit();
                // SAFETY: `raw_ptr_mut` projects `&raw mut raw` to `*mut Element` at offset 0
                // without materializing a reference to uninitialized memory, and `raw` is valid
                // for writes.
                unsafe {
                    core::ptr::write(
                        <Ts::RawUnion as RawUnionIndex<I>>::raw_ptr_mut(&raw mut raw),
                        val,
                    );
                }
                raw
            },
            index: I as RawVariantIndex,
        }
    }

    /// Replaces the contained value with `val` as the alternative at zero-based index `I`.
    #[inline]
    pub fn set_at<const I: usize>(&mut self, val: <Ts::RawUnion as RawUnionIndex<I>>::Element)
    where
        Ts::RawUnion: RawUnionIndex<I>,
    {
        *self = Self::new_at::<I>(val);
    }

    /// Destroys the currently active value and constructs the alternative at zero-based index `I`
    /// in place using the infallible constructor `value`, returning a pinned mutable reference to
    /// the newly constructed value.
    #[inline]
    pub fn emplace_at<const I: usize>(
        self: Pin<&mut Self>,
        value: impl Ctor<
            Output = <Ts::RawUnion as RawUnionIndex<I>>::Element,
            Error = core::convert::Infallible,
        >,
    ) -> Pin<&mut <Ts::RawUnion as RawUnionIndex<I>>::Element>
    where
        Ts::RawUnion: RawUnionIndex<I>,
    {
        match self.try_emplace_at::<I, core::convert::Infallible>(value) {
            Ok(p) => p,
            Err(e) => match e {},
        }
    }

    /// Destroys the currently active value and attempts to construct the alternative at zero-based
    /// index `I` in place using `value`. If `value` fails, the variant is left in the
    /// `valueless_by_exception` state and the error is returned.
    #[inline]
    pub fn try_emplace_at<const I: usize, E>(
        mut self: Pin<&mut Self>,
        value: impl Ctor<Output = <Ts::RawUnion as RawUnionIndex<I>>::Element, Error = E>,
    ) -> Result<Pin<&mut <Ts::RawUnion as RawUnionIndex<I>>::Element>, E>
    where
        Ts::RawUnion: RawUnionIndex<I>,
    {
        // SAFETY: We do not move out of `this`. `this.index` is replaced with `RAW_VARIANT_NPOS`
        // before `drop_active` so unwinding cannot double-drop. `raw_ptr_mut` projects to `*mut
        // Element` without materializing `&mut` to uninitialized memory, and `&mut *ptr` is formed
        // only after `value.ctor(ptr)` succeeds.
        unsafe {
            let this = self.as_mut().get_unchecked_mut();
            let prev_index = core::mem::replace(&mut this.index, RAW_VARIANT_NPOS);
            if prev_index != RAW_VARIANT_NPOS {
                Ts::drop_active(&mut this.raw, prev_index);
            }
            let ptr = <Ts::RawUnion as RawUnionIndex<I>>::raw_ptr_mut(&raw mut this.raw);
            value.ctor(ptr)?;
            this.index = I as RawVariantIndex;
            Ok(Pin::new_unchecked(&mut *ptr))
        }
    }

    /// Returns a [`Ctor`] that constructs a `variant` in place with the alternative at zero-based
    /// index `I` initialized by `c`.
    #[inline]
    pub fn ctor_new_at<const I: usize>(
        c: impl Ctor<
            Output = <Ts::RawUnion as RawUnionIndex<I>>::Element,
            Error = core::convert::Infallible,
        >,
    ) -> VariantAtCtor<
        Ts,
        I,
        impl Ctor<
            Output = <Ts::RawUnion as RawUnionIndex<I>>::Element,
            Error = core::convert::Infallible,
        >,
    >
    where
        Ts::RawUnion: RawUnionIndex<I>,
    {
        VariantAtCtor::<Ts, I, _>(c, core::marker::PhantomData)
    }

    /// Returns a mutable reference to the value of alternative type `T`.
    /// Returns `None` if the variant does not currently hold alternative `T` or is valueless.
    #[inline]
    pub fn get_mut<T, Index>(&mut self) -> Option<&mut T>
    where
        Ts::RawUnion: VariantAlternative<T, Index>,
    {
        if self.index != <Ts::RawUnion as VariantAlternative<T, Index>>::INDEX {
            return None;
        }
        Some(unsafe {
            <Ts::RawUnion as VariantAlternative<T, Index>>::get_unchecked_mut(&mut self.raw)
        })
    }

    /// Returns a pinned mutable reference to the value of alternative type `T`.
    /// Returns `None` if the variant does not currently hold alternative `T` or is valueless.
    #[inline]
    pub fn get_pin_mut<T, Index>(self: Pin<&mut Self>) -> Option<Pin<&mut T>>
    where
        Ts::RawUnion: VariantAlternative<T, Index>,
    {
        if self.index != <Ts::RawUnion as VariantAlternative<T, Index>>::INDEX {
            return None;
        }
        Some(unsafe {
            Pin::new_unchecked(<Ts::RawUnion as VariantAlternative<T, Index>>::get_unchecked_mut(
                &mut self.get_unchecked_mut().raw,
            ))
        })
    }

    /// Returns `true` if the variant holds an alternative of type `T`.
    #[inline]
    pub fn holds_alternative<T, Index>(&self) -> bool
    where
        Ts::RawUnion: VariantAlternative<T, Index>,
    {
        self.index == <Ts::RawUnion as VariantAlternative<T, Index>>::INDEX
    }

    /// Returns a shared reference to the alternative of type `T` if it is currently active, or
    /// `None` otherwise (similar to `std::get_if<T>` in C++).
    #[inline]
    pub fn get<T, Index>(&self) -> Option<&T>
    where
        Ts::RawUnion: VariantAlternative<T, Index>,
    {
        if self.index != <Ts::RawUnion as VariantAlternative<T, Index>>::INDEX {
            return None;
        }
        Some(unsafe { <Ts::RawUnion as VariantAlternative<T, Index>>::get_unchecked(&self.raw) })
    }

    /// Creates a new `variant` holding `val` of alternative type `T`.
    #[inline]
    pub fn new<T, Index>(val: T) -> Self
    where
        Ts::RawUnion: VariantAlternative<T, Index>,
    {
        Self {
            raw: <Ts::RawUnion as VariantAlternative<T, Index>>::from_element(val),
            index: <Ts::RawUnion as VariantAlternative<T, Index>>::INDEX,
        }
    }

    /// Replaces the contained value with `val` of alternative type `T`.
    #[inline]
    pub fn set<T, Index>(&mut self, val: T)
    where
        Ts::RawUnion: VariantAlternative<T, Index>,
        T: Sized,
    {
        *self = Self::new::<T, _>(val);
    }

    /// Destroys the currently active value and constructs the alternative of type `C::Output` in
    /// place using the infallible constructor `value`, returning a pinned mutable reference to the
    /// newly constructed value.
    #[inline]
    pub fn emplace<T, Index>(
        self: Pin<&mut Self>,
        value: impl Ctor<Output = T, Error = core::convert::Infallible>,
    ) -> Pin<&mut T>
    where
        Ts::RawUnion: VariantAlternative<T, Index>,
    {
        match self.try_emplace::<T, Index, core::convert::Infallible>(value) {
            Ok(p) => p,
            Err(e) => match e {},
        }
    }

    /// Destroys the currently active value and attempts to construct the alternative of type
    /// `C::Output` in place using `value`. If `value` fails, the variant is left in the
    /// `valueless_by_exception` state and the error is returned.
    #[inline]
    pub fn try_emplace<T, Index, E>(
        mut self: Pin<&mut Self>,
        value: impl Ctor<Output = T, Error = E>,
    ) -> Result<Pin<&mut T>, E>
    where
        Ts::RawUnion: VariantAlternative<T, Index>,
    {
        // SAFETY: We do not move out of `this`. `this.index` is replaced with `RAW_VARIANT_NPOS`
        // before `drop_active` so unwinding cannot double-drop. `raw_ptr_mut` projects to `*mut T`
        // without materializing `&mut` to uninitialized memory, and `&mut *ptr` is formed only
        // after `value.ctor(ptr)` succeeds.
        unsafe {
            let this = self.as_mut().get_unchecked_mut();
            let prev_index = core::mem::replace(&mut this.index, RAW_VARIANT_NPOS);
            if prev_index != RAW_VARIANT_NPOS {
                Ts::drop_active(&mut this.raw, prev_index);
            }
            let ptr =
                <Ts::RawUnion as VariantAlternative<T, Index>>::raw_ptr_mut(&raw mut this.raw);
            value.ctor(ptr)?;
            this.index = <Ts::RawUnion as VariantAlternative<T, Index>>::INDEX;
            Ok(Pin::new_unchecked(&mut *ptr))
        }
    }

    /// Returns a [`Ctor`] that constructs a `variant` in place with the alternative of type
    /// `C::Output` initialized by `c`.
    #[inline]
    pub fn ctor_new<T, Index>(
        c: impl Ctor<Output = T, Error = core::convert::Infallible>,
    ) -> VariantCtor<Ts, Index, impl Ctor<Output = T, Error = core::convert::Infallible>>
    where
        Ts::RawUnion: VariantAlternative<T, Index>,
    {
        VariantCtor::<Ts, Index, _>(c, core::marker::PhantomData)
    }
}

impl<Ts: VariantTypes> PartialEq for variant<Ts>
where
    Ts::RawUnion: RawUnionPartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        if self.index != other.index {
            return false;
        }
        if self.index == RAW_VARIANT_NPOS {
            return true;
        }
        unsafe { self.raw.eq_active(&other.raw, self.index) }
    }
}

impl<Ts: VariantTypes> Eq for variant<Ts> where Ts::RawUnion: RawUnionEq {}

impl<Ts: VariantTypes> Debug for variant<Ts>
where
    Ts::RawUnion: RawUnionDebug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.index == RAW_VARIANT_NPOS {
            write!(f, "valueless_by_exception")
        } else {
            unsafe { self.raw.fmt_active(self.index, f) }
        }
    }
}

impl<Ts: VariantTypes> Default for variant<Ts>
where
    Ts::RawUnion: RawUnionIndex<0>,
    <Ts::RawUnion as RawUnionIndex<0>>::Element: Default,
{
    fn default() -> Self {
        Self::new_at::<0>(Default::default())
    }
}

/// A C++ `std::variant` in Rust, when all alternatives are trivially destructible (`Copy` in Rust).
///
/// Unlike [`variant`], `trivial_variant` implements `Copy`, `Clone`, and `Unpin`, so it can be
/// passed and mutated by value without `Pin`. Convert between `trivial_variant<Ts>` and
/// `variant<Ts>` using `From`/`Into`.
///
/// # Examples
///
/// ```
/// use cpp_std::{trivial_variant, variant_enums::Variant2};
///
/// let mut v: trivial_variant![i32, bool] = trivial_variant::new(10i32);
/// assert_eq!(v.into_enum(), Variant2::V0(10));
///
/// v.set(true);
/// assert_eq!(v.get::<bool, _>(), Some(&true));
/// ```
#[crubit_annotate::cpp_layout_equivalent(
    cpp_type = "::std::variant<{Ts...}>",
    include_path = "<variant>"
)]
#[repr(C)]
pub struct trivial_variant<Ts: TrivialVariantTypes>
where
    Ts::RawUnion: Copy,
{
    pub(crate) raw: Ts::RawUnion,
    pub(crate) index: RawVariantIndex,
}

impl<Ts: TrivialVariantTypes> Copy for trivial_variant<Ts> {}
impl<Ts: TrivialVariantTypes> Clone for trivial_variant<Ts> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<Ts: TrivialVariantTypes> trivial_variant<Ts> {
    /// Returns the zero-based index of the alternative that is currently held by the variant.
    #[inline]
    pub fn index(&self) -> usize {
        if self.index == RAW_VARIANT_NPOS {
            VARIANT_NPOS
        } else {
            self.index as usize
        }
    }

    /// Returns `true` if and only if the variant holds no value (`valueless_by_exception` in C++).
    #[inline]
    #[doc(hidden)]
    pub fn valueless_by_exception(&self) -> bool {
        self.index == RAW_VARIANT_NPOS
    }

    #[inline(always)]
    fn check_valueless(&self) {
        if self.index == RAW_VARIANT_NPOS {
            panic!("std::variant is valueless_by_exception");
        }
    }

    /// Projects a shared reference to the variant into a Rust enum of shared references to its
    /// alternatives ([`Variant1`](crate::std::variant_enums::Variant1)..
    /// [`Variant8`](crate::std::variant_enums::Variant8) or
    /// [`Coproduct`]($crate::std::coproduct::Coproduct)).
    ///
    /// # Panics
    ///
    /// Panics if the variant is in the `valueless_by_exception` state.
    #[inline]
    pub fn as_ref(&self) -> Ts::Ref<'_> {
        self.check_valueless();
        unsafe { Ts::as_ref(&self.raw, self.index) }
    }

    /// Projects an exclusive reference to the variant into a Rust enum of mutable references to
    /// its alternatives.
    ///
    /// # Panics
    ///
    /// Panics if the variant is in the `valueless_by_exception` state.
    #[inline]
    pub fn as_mut(&mut self) -> Ts::Mut<'_> {
        self.check_valueless();
        unsafe { Ts::as_mut(&mut self.raw, self.index) }
    }

    /// Consumes the variant and returns the active alternative wrapped in the corresponding Rust
    /// projection enum ([`Variant1`](crate::std::variant_enums::Variant1)..
    /// [`Variant8`](crate::std::variant_enums::Variant8) or
    /// [`Coproduct`]($crate::std::coproduct::Coproduct)).
    ///
    /// # Panics
    ///
    /// Panics if the variant is in the `valueless_by_exception` state.
    #[inline]
    pub fn into_enum(self) -> Ts::Owned {
        self.check_valueless();
        unsafe { Ts::into_owned(self.raw, self.index) }
    }

    /// Returns a shared reference to the alternative at zero-based index `I` if it is currently
    /// active, or `None` otherwise (similar to `std::get_if<I>` in C++).
    #[inline]
    pub fn get_at<const I: usize>(&self) -> Option<&<Ts::RawUnion as RawUnionIndex<I>>::Element>
    where
        Ts::RawUnion: RawUnionIndex<I>,
    {
        if self.index != I as RawVariantIndex {
            return None;
        }
        Some(unsafe { <Ts::RawUnion as RawUnionIndex<I>>::get_unchecked(&self.raw) })
    }

    /// Returns a mutable reference to the alternative at zero-based index `I` if it is currently
    /// active, or `None` otherwise (similar to `std::get_if<I>` in C++).
    #[inline]
    pub fn get_at_mut<const I: usize>(
        &mut self,
    ) -> Option<&mut <Ts::RawUnion as RawUnionIndex<I>>::Element>
    where
        Ts::RawUnion: RawUnionIndex<I>,
    {
        if self.index != I as RawVariantIndex {
            return None;
        }
        Some(unsafe { <Ts::RawUnion as RawUnionIndex<I>>::get_unchecked_mut(&mut self.raw) })
    }

    /// Returns `true` if the alternative at zero-based index `I` is currently active.
    #[inline]
    pub fn holds_alternative_at<const I: usize>(&self) -> bool
    where
        Ts::RawUnion: RawUnionIndex<I>,
    {
        self.index == I as RawVariantIndex
    }

    /// Creates a new `trivial_variant` holding `val` as the alternative at zero-based index `I`.
    #[inline]
    pub fn new_at<const I: usize>(val: <Ts::RawUnion as RawUnionIndex<I>>::Element) -> Self
    where
        Ts::RawUnion: RawUnionIndex<I>,
    {
        Self {
            raw: {
                let mut raw = Ts::new_raw_uninit();
                // SAFETY: `raw_ptr_mut` projects `&raw mut raw` to `*mut Element` at offset 0
                // without materializing a reference to uninitialized memory, and `raw` is valid
                // for writes.
                unsafe {
                    core::ptr::write(
                        <Ts::RawUnion as RawUnionIndex<I>>::raw_ptr_mut(&raw mut raw),
                        val,
                    );
                }
                raw
            },
            index: I as RawVariantIndex,
        }
    }

    /// Replaces the contained value with `val` as the alternative at zero-based index `I`.
    #[inline]
    pub fn set_at<const I: usize>(&mut self, val: <Ts::RawUnion as RawUnionIndex<I>>::Element)
    where
        Ts::RawUnion: RawUnionIndex<I>,
    {
        *self = Self::new_at::<I>(val);
    }

    /// Returns a mutable reference to the value of alternative type `T`.
    /// Returns `None` if the variant does not currently hold alternative `T` or is valueless.
    #[inline]
    pub fn get_mut<T, Index>(&mut self) -> Option<&mut T>
    where
        Ts::RawUnion: VariantAlternative<T, Index>,
    {
        if self.index != <Ts::RawUnion as VariantAlternative<T, Index>>::INDEX {
            return None;
        }
        Some(unsafe {
            <Ts::RawUnion as VariantAlternative<T, Index>>::get_unchecked_mut(&mut self.raw)
        })
    }

    /// Returns `true` if the variant holds an alternative of type `T`.
    #[inline]
    pub fn holds_alternative<T, Index>(&self) -> bool
    where
        Ts::RawUnion: VariantAlternative<T, Index>,
    {
        self.index == <Ts::RawUnion as VariantAlternative<T, Index>>::INDEX
    }

    /// Returns a shared reference to the alternative of type `T` if it is currently active, or
    /// `None` otherwise (similar to `std::get_if<T>` in C++).
    #[inline]
    pub fn get<T, Index>(&self) -> Option<&T>
    where
        Ts::RawUnion: VariantAlternative<T, Index>,
    {
        if self.index != <Ts::RawUnion as VariantAlternative<T, Index>>::INDEX {
            return None;
        }
        Some(unsafe { <Ts::RawUnion as VariantAlternative<T, Index>>::get_unchecked(&self.raw) })
    }

    /// Creates a new `trivial_variant` holding `val` of alternative type `T`.
    #[inline]
    pub fn new<T, Index>(val: T) -> Self
    where
        Ts::RawUnion: VariantAlternative<T, Index>,
    {
        Self {
            raw: <Ts::RawUnion as VariantAlternative<T, Index>>::from_element(val),
            index: <Ts::RawUnion as VariantAlternative<T, Index>>::INDEX,
        }
    }

    /// Replaces the contained value with `val` of alternative type `T`.
    #[inline]
    pub fn set<T, Index>(&mut self, val: T)
    where
        Ts::RawUnion: VariantAlternative<T, Index>,
        T: Sized,
    {
        *self = Self::new::<T, _>(val);
    }
}

impl<Ts: TrivialVariantTypes> PartialEq for trivial_variant<Ts>
where
    Ts::RawUnion: RawUnionPartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        if self.index != other.index {
            return false;
        }
        if self.index == RAW_VARIANT_NPOS {
            return true;
        }
        unsafe { self.raw.eq_active(&other.raw, self.index) }
    }
}

impl<Ts: TrivialVariantTypes> Eq for trivial_variant<Ts> where Ts::RawUnion: RawUnionEq {}

impl<Ts: TrivialVariantTypes> Debug for trivial_variant<Ts>
where
    Ts::RawUnion: RawUnionDebug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.index == RAW_VARIANT_NPOS {
            write!(f, "valueless_by_exception")
        } else {
            unsafe { self.raw.fmt_active(self.index, f) }
        }
    }
}

impl<Ts: TrivialVariantTypes> Default for trivial_variant<Ts>
where
    Ts::RawUnion: Copy + RawUnionIndex<0>,
    <Ts::RawUnion as RawUnionIndex<0>>::Element: Default,
{
    fn default() -> Self {
        Self::new_at::<0>(Default::default())
    }
}

/// Expands to [`variant`]`<(T0, ..., Tn)>` for up to 8 alternatives, or falls back to nested
/// [`Coproduct`](crate::std::coproduct::Coproduct) types for more than 8 alternatives.
#[macro_export]
macro_rules! variant {
    ($($tt:tt)*) => { $crate::__crubit_variant_macro!($($tt)*) }
}

/// Expands to [`trivial_variant`]`<(T0, ..., Tn)>` for up to 8 alternatives, or falls back to
/// nested [`Coproduct`](crate::std::coproduct::Coproduct) types for more than 8 alternatives.
#[macro_export]
macro_rules! trivial_variant {
    ($($tt:tt)*) => { $crate::__crubit_trivial_variant_macro!($($tt)*) }
}

impl<Ts: VariantTypes> ctor::SelfCtor for variant<Ts> {}

impl<Ts: crate::std::variant_internal::TrivialVariantTypes> From<trivial_variant<Ts>>
    for variant<Ts>
{
    #[inline]
    fn from(v: trivial_variant<Ts>) -> Self {
        Self { raw: v.raw, index: v.index }
    }
}

impl<Ts: crate::std::variant_internal::TrivialVariantTypes> From<variant<Ts>>
    for trivial_variant<Ts>
{
    #[inline]
    fn from(mut v: variant<Ts>) -> Self {
        let raw = unsafe { core::ptr::read(&v.raw) };
        let index = v.index;
        v.index = crate::std::variant_internal::RAW_VARIANT_NPOS;
        Self { raw, index }
    }
}

impl<Ts, const I: usize, C> !ctor::SelfCtor for VariantAtCtor<Ts, I, C> {}
impl<Ts, I, C> !ctor::SelfCtor for VariantCtor<Ts, I, C> {}

unsafe impl<Ts, const I: usize, C> ctor::Ctor for VariantAtCtor<Ts, I, C>
where
    Ts: VariantTypes,
    Ts::RawUnion: RawUnionIndex<I>,
    C: ctor::Ctor<Output = <Ts::RawUnion as RawUnionIndex<I>>::Element>,
{
    type Output = variant<Ts>;
    type Error = C::Error;

    #[inline]
    unsafe fn ctor(self, dest: *mut Self::Output) -> Result<(), Self::Error> {
        // SAFETY: Caller guarantees `dest` is valid for writes. `index` is initialized to
        // `RAW_VARIANT_NPOS` first so `dest` holds a valid valueless `variant` if `self.0.ctor`
        // fails or unwinds. `raw_ptr_mut` projects `&raw mut (*dest).raw` to `*mut Element`
        // without materializing a reference to uninitialized memory.
        unsafe {
            (&raw mut (*dest).index).write(RAW_VARIANT_NPOS);
            let val_ptr = <Ts::RawUnion as RawUnionIndex<I>>::raw_ptr_mut(&raw mut (*dest).raw);
            self.0.ctor(val_ptr)?;
            (&raw mut (*dest).index).write(I as RawVariantIndex);
            Ok(())
        }
    }
}

unsafe impl<Ts, I, C, T> ctor::Ctor for VariantCtor<Ts, I, C>
where
    Ts: VariantTypes,
    Ts::RawUnion: VariantAlternative<T, I>,
    C: ctor::Ctor<Output = T>,
{
    type Output = variant<Ts>;
    type Error = C::Error;

    #[inline]
    unsafe fn ctor(self, dest: *mut Self::Output) -> Result<(), Self::Error> {
        // SAFETY: Caller guarantees `dest` is valid for writes. `index` is initialized to
        // `RAW_VARIANT_NPOS` first so `dest` holds a valid valueless `variant` if `self.0.ctor`
        // fails or unwinds. `raw_ptr_mut` projects `&raw mut (*dest).raw` to `*mut T` without
        // materializing a reference to uninitialized memory.
        unsafe {
            (&raw mut (*dest).index).write(RAW_VARIANT_NPOS);
            let val_ptr =
                <Ts::RawUnion as VariantAlternative<T, I>>::raw_ptr_mut(&raw mut (*dest).raw);
            self.0.ctor(val_ptr)?;
            (&raw mut (*dest).index).write(<Ts::RawUnion as VariantAlternative<T, I>>::INDEX);
            Ok(())
        }
    }
}
