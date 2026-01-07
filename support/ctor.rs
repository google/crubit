// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#![cfg_attr(not(test), no_std)]
#![feature(negative_impls, allow_internal_unstable, super_let)]
#![allow(internal_features)] // allow_internal_unstable 🤔
//! Traits for memory management operations on wrapped C++ objects, inspired by
//! moveit, pin-init, and the current in-place initialization proposal at
//! https://hackmd.io/@aliceryhl/BJutRcPblx.
//!
//! # Comparison with pin-init
//!
//! TODO: fill this out. It's an open question whether all of the differences can be removed.
//!
//! Quick overview:
//!
//! * pin-init uses a type parameter for the output, `ctor.rs` uses an associated type.
//!   * Associated type impls can't overlap, meaning someday we could do trait impls like
//!     `impl<T: Ctor<Output=A>> X for Y {}` plus `impl<T: Ctor<Output=B>> X for Y {}`.
//! * pin-init uses a type parameter for the error, `ctor.rs` uses an associated type.
//!   * This results in fewer type inference issues, and is what the upstream proposal
//!     for pin-init currently does.
//! * (trivial) syntax and naming differences
//! * pin-init allows direct-initialization, but `ctor.rs` only allows pinned initialization.
//!   * The upstream proposal for pin-init may do something similar, only having in-place
//!     initialization: https://hackmd.io/@aliceryhl/BJutRcPblx
//! * pin-init uses `#[pin]` to decide if a field is pinned. This is common among other libraries
//!   as well.
//!
//! # Comparison with moveit
//!
//! `ctor.rs` was initially based on `moveit`, taking the `Ctor` trait from there,
//! but has since diverged. In places this is for reasons of C++ interop, and
//! in other places it is to bring it closer to `pin-init` and the upstream
//! proposal for in-place initialization.
//!
//! ## Non-destructive move
//!
//! Unlike moveit, C++ moves are never Rust moves.
//!
//! This is important because it dramatically simplifies the mental model and
//! code involved. Everything to do with DerefMove can be removed. Furthermore,
//! it makes it *more useful*.
//!
//! In particular, imagine trying to write a *Rust* implementation of the
//! following C++ function:
//!
//! ```c++
//! void Swap(CxxClass& x, CxxClass& y) {
//!   CxxClass tmp = std::move(x);
//!   x = std::move(y);
//!   y = std::move(tmp);
//! }
//! ```
//!
//! With destructive moves we're in a bind: any move from x destroys its source,
//! leaving us with nothing to assign to. C++ moves are non-destructive. Traits
//! modeling C++ moves, therefore, in order to be most compatible, should also
//! be non-destructive.
//!
//! Here is an implementation using the traits in this file, in pure Rust:
//!
//! ```
//! fn swap(mut x: Pin<&mut CxxClass>, mut y: Pin<&mut CxxClass>) {
//!  let mut tmp = emplace!(mov!(x.as_mut()));
//!  x.assign(mov!(y.as_mut()));
//!  y.assign(mov!(tmp));
//! }
//! ```
//!
//! ## Curried `Ctor` traits
//!
//! Rather than `CopyCtor::copy_ctor` and `MoveCtor::move_ctor`, this crate
//! embraces `Ctor` itself as the single unique way to initialize everything.
//! And so copy-constructible types are not things which implement `CopyCtor`,
//! but rather, things which implement `CtorNew<&T>`. Similarly,
//! move-constructible things implement `CtorNew<RvalueReference<'_, T>>`.
//!
//! ## Blanket impls
//!
//! So that Rust types can be used against C++ functions (even templated ones),
//! the copy and move traits have blanket impls for all suitable Unpin types.
//!
//! More significantly, we take the position that all `Unpin` types are their own
//! `Ctor`. This makes `Ctor`-based initialization nearly a perfect superset of
//! normal initialization rules: for a non-self-referential Rust type (an `Unpin`
//! type), it looks the same as normal, but for C++-like `!Unpin` types we use
//! custom `Ctor` values to initialize in-place.
//!
//! Blanket implementing based on `Unpin` means that we're treating `T : Unpin`
//! as effectively meaning "`T` is safe to deal with by value", which is close to,
//! but not quite, what it means. (There are some types which are `!Unpin` but
//! would be safe to treat as normal rust types. Unfortunately, there is no way
//! to detect them.)
//!
//! A more general solution would be to use a custom auto trait, e.g.
//! `pub auto trait SelfInit {}`.
//!
//! ## Overload trait
//!
//! This module also includes a prototype alternative API which more directly
//! models overloaded constructors: `T : CtorNew<(A, B, C)>` is roughly
//! equivalent to the C++ "T has a constructor taking arguments of type A, B,
//! and C".
//!
//! ## StackBox
//!
//! Moveit needs a StackBox type which is comparable to Box, except stored on
//! the stack -- in particular, which implements OuterDrop and the like.
//!
//! Since DerefMove is not used here, we instead are able to export only a
//! Pin<&mut T> to users.
//!
//! One still needs something similar to `Slot` or `StackBox`: a `MaybeUninit`
//! type which will run drop. This isn't exposed to users directly if they are
//! simply storing in a local variable.
//!
//! ## Structs and the `ctor!` macro
//!
//! `ctor` adds a `ctor!` macro to make it easy to initialize a struct
//! that contains non-trivially-relocatable fields.
//!
//! ## Errors
//!
//! To support initialization which can fail, without requiring the use
//! of panic `Ctor` also has an associated `Error` type, representing
//! failure. Most of the macros/methods should, over time, gain `try_`
//! variants which can support non-`Infallible` errors.
//!
//! (This particular change was taken from the upstream proposal for
//! in-place initialization.)
//!
//! # Features
//!
//! This library requires the following unstable features enabled in users:
//!
//! **negative_impls:**
//! This is used to allow trait coherence checking for `!Unpin` types. A
//! "blanket impl for all `Unpin` types is used to make them constructible with
//! `ctor`, and Rust believes this may conflict with types that use
//! `PhantomPinned`. It knows no conflict exists if, instead, types impl
//! `!Unpin`.

extern crate alloc;

use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::sync::Arc;
use core::fmt::Debug;
use core::marker::{PhantomData, Unpin};
use core::mem::{ManuallyDrop, MaybeUninit};
use core::ops::{Deref, DerefMut};
use core::pin::Pin;

pub use ctor_proc_macros::{
    project_pin_type, recursively_pinned, CtorFrom_Default, MoveAndAssignViaCopy,
};

/// The error type for an infallible `Ctor`.
///
/// This error type is special-cased for unfallible operations,
/// such as `emplace()`. If you need to handle an error, use
/// the `try_` variants, instead.
pub type Infallible = core::convert::Infallible;

/// The string constant for #[must_use] to describe why you must use a `Ctor`.
macro_rules! must_use_ctor {
    () => {
        "A Ctor is not invoked unless emplaced, using e.g. `emplace!()`, or `Box::emplace()`."
    };
}

/// The string constant for #[must_use] to describe why you must use a
/// `Ctor`/`Assign` source.
macro_rules! must_use_ctor_assign {
    ($name:literal) => {
        concat!(
            $name,
            " is not invoked unless emplaced, using e.g. `emplace!()`, or `Box::emplace()`, or",
            " unless assigned from, using `.assign()`."
        )
    };
}

// ============================
// Core construction operations
// ============================

/// In-place initialization of a value.
///
/// # Safety
///
/// Implementations must satisfy the postconditions of the `ctor` method.
#[must_use = must_use_ctor!()]
pub unsafe trait Ctor: Sized {
    /// The constructed output type.
    type Output: ?Sized;
    /// The error type if initialization fails. This should typically
    /// be `Infallible`, as there's only limited support for real
    /// errors currently.
    type Error;

    /// Constructs a value in place.
    ///
    /// Before this call, `dest` is uninitialized. After this call,
    /// if `ctor` does not panic, and returns `Ok`, then `dest` is
    /// initialized to the constructed value.
    ///
    /// # Safety
    ///
    /// `dest` is valid for writes, pinned, and uninitialized.
    unsafe fn ctor(self, dest: *mut Self::Output) -> Result<(), Self::Error>;

    /// Converts a `Ctor` with error type `Infallible` to one with the provided error type `E`.
    ///
    /// This is useful for chaining possibly-fallible operations (such as `ctor_then`) on top of
    /// an existing infallible `Ctor`.
    fn ctor_make_fallible<E>(self) -> impl Ctor<Output = Self::Output, Error = E>
    where
        Self: Ctor<Error = Infallible>,
    {
        self.ctor_map_err(|e: Infallible| match e {})
    }

    /// Maps this `Ctor`'s error type into a new error type using the `Into` trait.
    fn ctor_err_into<E>(self) -> impl Ctor<Output = Self::Output, Error = E>
    where
        Self::Error: Into<E>,
    {
        self.ctor_map_err(|e| e.into())
    }

    /// Returns a `Ctor`, which will invoke `f` after construction, if successful.
    ///
    /// This functions similarly to `Result::and_then`.
    ///
    /// For example, these two snippets are equivalent:
    ///
    /// ```
    /// let mut x = emplace!(y);
    /// x.mutating_method();
    /// ```
    ///
    /// ```
    /// let new_ctor = y.ctor_then(|mut initialized| {
    ///     initialized.mutating_method();
    ///     Ok(())
    /// });
    /// let x = emplace!(new_ctor);
    /// ```
    fn ctor_then<F>(self, f: F) -> impl Ctor<Output = Self::Output, Error = Self::Error>
    where
        F: FnOnce(Pin<&mut Self::Output>) -> Result<(), Self::Error>,
    {
        struct CtorThen<C: Ctor, F: FnOnce(Pin<&mut C::Output>) -> Result<(), C::Error>> {
            ctor: C,
            f: F,
        }

        // SAFETY: unconditionally initializes dest.
        unsafe impl<C: Ctor, F: FnOnce(Pin<&mut C::Output>) -> Result<(), C::Error>> Ctor
            for CtorThen<C, F>
        {
            type Output = C::Output;
            type Error = C::Error;
            unsafe fn ctor(self, dest: *mut Self::Output) -> Result<(), Self::Error> {
                self.ctor.ctor(dest)?;
                let dest = Pin::new_unchecked(&mut *dest);
                (self.f)(dest)
            }
        }

        impl<C, F> !Unpin for CtorThen<C, F>
        where
            C: Ctor,
            F: FnOnce(Pin<&mut C::Output>) -> Result<(), C::Error>,
        {
        }

        CtorThen { ctor: self, f }
    }

    /// Returns a `Ctor` which will invoke `f` if construction fails,
    /// transforming the error type.
    ///
    /// This functions similarly to `Result::map_err`.
    ///
    /// For example, these two snippets are equivalent:
    ///
    /// ```
    /// let x: Result<Pin<&mut T>, E> = try_emplace!(y);
    /// let x = x.map_err(|e| e.into_new_error())
    /// ```
    ///
    /// ```
    /// let new_ctor = y.ctor_map_err(|e| e.into_new_error());
    /// let x = try_emplace!(new_ctor);
    /// ```
    fn ctor_map_err<F, E>(self, f: F) -> impl Ctor<Output = Self::Output, Error = E>
    where
        F: FnOnce(Self::Error) -> E,
    {
        self.ctor_or_else(|e| ctor_error(f(e)))
    }

    /// Returns a `Ctor` which, if the original construction fails,
    /// invoke `f` in order to attempt to produce the value.
    ///
    /// This functions similarly to `Result::or_else`.
    ///
    /// For example:
    ///
    /// ```
    /// let new_ctor = Y::first_attempt().ctor_or_else(|_| Y::fallback_attempt());
    /// let x = try_emplace!(new_ctor);
    /// ```
    fn ctor_or_else<F, O>(self, f: F) -> impl Ctor<Output = Self::Output, Error = O::Error>
    where
        F: FnOnce(Self::Error) -> O,
        O: Ctor<Output = Self::Output>,
    {
        struct CtorOrElse<C, F, O>
        where
            C: Ctor,
            F: FnOnce(C::Error) -> O,
            O: Ctor<Output = C::Output>,
        {
            ctor: C,
            f: F,
        }

        // SAFETY: initializes dest or returns an error.
        unsafe impl<C, F, O> Ctor for CtorOrElse<C, F, O>
        where
            C: Ctor,
            F: FnOnce(C::Error) -> O,
            O: Ctor<Output = C::Output>,
        {
            type Output = C::Output;
            type Error = O::Error;
            unsafe fn ctor(self, dest: *mut Self::Output) -> Result<(), Self::Error> {
                let Err(e) = self.ctor.ctor(dest) else { return Ok(()) };
                (self.f)(e).ctor(dest)
            }
        }

        impl<C, F, O> !Unpin for CtorOrElse<C, F, O>
        where
            C: Ctor,
            F: FnOnce(C::Error) -> O,
            O: Ctor<Output = C::Output>,
        {
        }

        CtorOrElse { ctor: self, f }
    }

    /// Returns a `Ctor` which will panic with the given message if the original construction
    /// fails.
    ///
    /// This functions similarly to `Result::expect`.
    fn ctor_expect<'a>(
        self,
        msg: &'a str,
    ) -> impl Ctor<Output = Self::Output, Error = Infallible> + use<'a, Self>
    where
        Self::Error: Debug,
    {
        // Unused: convert `!` into a proper `Ctor`-implementing type.
        #[allow(unused)]
        self.ctor_or_else(move |e| ctor_error(panic!("{msg}: {e:?}")))
    }

    /// Returns a `Ctor` which will panic if the original construction fails.
    ///
    /// This functions similarly to `Result::unwrap`.
    fn ctor_unwrap(self) -> impl Ctor<Output = Self::Output, Error = Infallible>
    where
        Self::Error: Debug,
    {
        self.ctor_or_else(|e| {
            // Unused: convert `!` into a proper `Ctor`-implementing type.
            #[allow(unused)]
            ctor_error(panic!(
                "Construction of {type_name} failed: {e:?}",
                type_name = core::any::type_name::<Self::Output>(),
            ))
        })
    }

    /// Returns a `Ctor` which will return a default value if the original construction fails.
    ///
    /// This functions similarly to `Result::unwrap_or_default`.
    fn ctor_unwrap_or_default(
        self,
    ) -> impl Ctor<Output = Self::Output, Error = <Self::Output as CtorNew<()>>::Error>
    where
        Self::Output: CtorNew<()>,
    {
        self.ctor_or_else(|_| Self::Output::ctor_new(()))
    }
}

/// Returns a `Ctor` with error type `E` which always fails with the given error.
pub fn ctor_error<T: ?Sized, E>(e: E) -> impl Ctor<Output = T, Error = E> {
    struct CtorError<T: ?Sized, E> {
        e: E,
        marker: PhantomData<fn() -> T>,
    }

    // SAFETY: unconditionally returns an error.
    unsafe impl<T: ?Sized, E> Ctor for CtorError<T, E> {
        type Output = T;
        type Error = E;
        unsafe fn ctor(self, _: *mut Self::Output) -> Result<(), Self::Error> {
            Err(self.e)
        }
    }

    impl<T: ?Sized, E> !Unpin for CtorError<T, E> {}

    CtorError { e, marker: PhantomData }
}

/// Trait for smart pointer types which support initialization via `Ctor`.
///
/// A typical example would be `Box<T>`, allows emplacing a `Ctor` into
/// a `Pin<Box<T>>` by calling `{Box, Rc, Arc}::emplace`.
pub trait Emplace<T>: Sized {
    /// Materialize an unfailable `Ctor`.
    fn emplace<C: Ctor<Output = T, Error = Infallible>>(c: C) -> Pin<Self> {
        Self::try_emplace(c).unwrap()
    }

    /// Materialize a `Ctor`, returning an error if initialization fails.
    fn try_emplace<C: Ctor<Output = T>>(c: C) -> Result<Pin<Self>, C::Error>;
}

impl<T> Emplace<T> for Box<T> {
    fn try_emplace<C: Ctor<Output = T>>(ctor: C) -> Result<Pin<Box<T>>, C::Error> {
        let mut uninit = Box::new(MaybeUninit::<T>::uninit());
        unsafe {
            ctor.ctor(uninit.as_mut_ptr())?;
            Ok(Pin::new_unchecked(uninit.assume_init()))
        }
    }
}

impl<T> Emplace<T> for Rc<T> {
    fn try_emplace<C: Ctor<Output = T>>(ctor: C) -> Result<Pin<Rc<T>>, C::Error> {
        let uninit = Rc::new(MaybeUninit::<T>::uninit());
        unsafe {
            // TODO: https://github.com/rust-lang/rust/issues/145036 - use cast_init when stable.
            ctor.ctor(Rc::as_ptr(&uninit).cast_mut().cast::<T>())?;
            Ok(Pin::new_unchecked(uninit.assume_init()))
        }
    }
}

impl<T> Emplace<T> for Arc<T> {
    fn try_emplace<C: Ctor<Output = T>>(ctor: C) -> Result<Pin<Arc<T>>, C::Error> {
        let uninit = Arc::new(MaybeUninit::<T>::uninit());
        unsafe {
            // TODO: https://github.com/rust-lang/rust/issues/145036 - use cast_init when stable.
            ctor.ctor(Arc::as_ptr(&uninit).cast_mut().cast::<T>())?;
            Ok(Pin::new_unchecked(uninit.assume_init()))
        }
    }
}

#[must_use = must_use_ctor!()]
pub struct FnCtor<Output, F: FnOnce(*mut Output)>(pub F, PhantomData<fn(Output)>);
impl<Output, F: FnOnce(*mut Output)> FnCtor<Output, F> {
    pub fn new(f: F) -> Self {
        Self(f, PhantomData)
    }
}

// SAFETY: unconditionally initializes dest.
unsafe impl<Output, F: FnOnce(*mut Output)> Ctor for FnCtor<Output, F> {
    type Output = Output;
    type Error = Infallible;

    unsafe fn ctor(self, dest: *mut Output) -> Result<(), Infallible> {
        self.0(dest);
        Ok(())
    }
}

/// !Unpin to override the blanket Ctor impl.
impl<Output, F> !Unpin for FnCtor<Output, F> {}

/// Copy type.
///
/// This creates a new `P::Target` by copying -- either copy-construction
/// (construction from `&*P`), or copy-assignment (assignment from &*P`). It can
/// also be used directly as a `Ctor`.
///
/// Note: this does not actually copy `P` until it is used.
#[must_use = must_use_ctor_assign!("Copy")]
pub struct Copy<P: ?Sized + Deref>(P);

// SAFETY: unconditionally initializes dest.
unsafe impl<Output: ?Sized, Error, P: Deref<Target = Output>> Ctor for Copy<P>
where
    Output: for<'a> CtorNew<&'a Output, Error = Error>,
{
    type Output = Output;
    type Error = Error;

    unsafe fn ctor(self, dest: *mut Output) -> Result<(), Self::Error> {
        Output::ctor_new(&*self.0).ctor(dest)
    }
}

/// !Unpin to override the blanket Ctor impl.
impl<P: ?Sized> !Unpin for Copy<P> {}

/// Returns a `Copy` which can be used as a `CtorNew` or `Assign` source, or as
/// a `Ctor` directly.
///
/// Note: this does not actually copy the parameter until it is used.
pub fn copy<T: ?Sized + for<'a> CtorNew<&'a T>, P: Deref<Target = T>>(src: P) -> Copy<P> {
    Copy(src)
}

// ================================
// DerefMut based move construction
// ================================

/// Rvalue Reference (move-reference) type.
///
/// This creates a new `T` by moving -- either move-construction (construction
/// from `RvalueReference(&*P)`), or move-assignment (assignment from
/// `RvalueReference(&*P)`).
///
/// Note: this does not actually move until it is used.
#[must_use = must_use_ctor_assign!("RvalueReference")]
#[repr(transparent)]
pub struct RvalueReference<'a, T: ?Sized>(pub Pin<&'a mut T>);

impl<T: ?Sized> RvalueReference<'_, T> {
    pub fn as_const(&self) -> ConstRvalueReference<'_, T> {
        ConstRvalueReference(&*self.0)
    }

    pub fn as_mut(&mut self) -> Pin<&mut T> {
        self.0.as_mut()
    }

    pub fn get_ref(&self) -> &T {
        // It would be nice to return &'a T, but that would not be sound, and as a
        // result Pin makes it impossible (in safe code). Consider:
        //
        //   let my_pin: Pin<&mut T> = ...;
        //   let my_borrow = RvalueReference(my_pin.as_mut()).get_ref();
        //
        // The lifetime of my_borrow CANNOT be 'a, but instead MUST be scoped to the
        // reborrowed lifetime of the pin, or else it would violate the aliasing
        // rules by coexisting with my_pin. Thus, get_ref must return &T, not
        // &'a T. (For the same reason, as_const returns a ConstRvalueReference
        // whose lifetime is bound by self, not 'a.)
        &*self.0
    }
}

impl<T: ?Sized> Deref for RvalueReference<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.get_ref()
    }
}

// SAFETY: unconditionally initializes dest.
unsafe impl<'a, T: ?Sized> Ctor for RvalueReference<'a, T>
where
    T: CtorNew<Self>,
{
    type Output = T;
    type Error = <T as CtorNew<Self>>::Error;

    unsafe fn ctor(self, dest: *mut T) -> Result<(), Self::Error> {
        T::ctor_new(self).ctor(dest)
    }
}

/// Converts to an RvalueReference.
///
/// Do not use this trait directly, instead, cast to an RvalueReference using
/// the `mov!()` macro.
pub trait DerefRvalueReference: Deref
where
    Self::Target: Sized,
{
    fn deref_rvalue_reference(&mut self) -> RvalueReference<'_, Self::Target>;
}

impl<T> DerefRvalueReference for Pin<T>
where
    T: DerefMut,
    Self::Target: Sized,
{
    fn deref_rvalue_reference(&mut self) -> RvalueReference<'_, Self::Target> {
        RvalueReference(self.as_mut())
    }
}

impl<T> DerefRvalueReference for &mut T
where
    T: Unpin,
{
    fn deref_rvalue_reference(&mut self) -> RvalueReference<'_, Self::Target> {
        RvalueReference(Pin::new(self))
    }
}

/// !Unpin to override the blanket `Ctor` impl.
impl<'a, T: ?Sized> !Unpin for RvalueReference<'a, T> {}

/// Const rvalue reference (move-reference) type. Usually not very helpful.
///
/// This implicitly converts to `T` by const-moving -- either
/// const-move-construction (construction from `ConstRvalueReference(&x)`), or
/// const-move-assignment (assignment from `ConstRvalueReference(&x)`).
#[must_use = must_use_ctor_assign!("ConstRvalueReference")]
#[repr(transparent)]
pub struct ConstRvalueReference<'a, T: ?Sized>(pub &'a T);

impl<'a, T: ?Sized> ConstRvalueReference<'a, T> {
    pub fn get_ref(&mut self) -> &'a T {
        self.0
    }
}

impl<T: ?Sized> Deref for ConstRvalueReference<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}

// SAFETY: unconditionally initializes dest.
unsafe impl<'a, T: ?Sized> Ctor for ConstRvalueReference<'a, T>
where
    T: CtorNew<Self>,
{
    type Output = T;
    type Error = <T as CtorNew<Self>>::Error;

    unsafe fn ctor(self, dest: *mut T) -> Result<(), Self::Error> {
        T::ctor_new(self).ctor(dest)
    }
}

/// !Unpin to override the blanket `Ctor` impl.
impl<'a, T: ?Sized> !Unpin for ConstRvalueReference<'a, T> {}

/// Creates a "to-be-moved" pointer for `src`.
///
/// In other words, this is analogous to C++ `std::move`, except that this can
/// directly create an `RvalueReference<T>` out of e.g. a `Pin<Box<T>>`. The
/// resulting `RvalueReference` has the lifetime of a temporary, after which the
/// parameter is destroyed.
///
/// The resulting `RvalueReference` can be used as a `CtorNew` or `Assign`
/// source, or as a `Ctor` directly.
///
/// Note: this does not actually move the parameter until it is used.
#[macro_export]
macro_rules! mov {
    ($p:expr) => {
        $crate::DerefRvalueReference::deref_rvalue_reference(&mut { $p })
    };
}

#[macro_export]
macro_rules! const_mov {
    ($p:expr) => {
        $crate::ConstRvalueReference(&*{ $p })
    };
}

// =============
// Blanket impls
// =============
//
// For interop with C++ templates that may need to take Rust types, we will want
// blanket impls for Rust types where it is safe to do so.

/// All Rust types are their own constructor, if it is known to be safe (i.e.
/// Unpin).
///
/// (Note that this is an approximation: some types are safe to deal with by
/// value, as long as they have never been pinned. In `ctor`, we assume
/// that *anything* which is `!Unpin` is only safe to initialize via a manually
/// provided `Ctor`.)
///
/// (Note: to avoid overlapping impls, every type that implements `Ctor` by hand
/// must be `!Unpin`.)
///
/// This allows code to safely accept direct initialization of `Unpin` Rust
/// values, while also accepting `Ctor`-based initialization for `!Unpin`
/// values: use `Ctor`-based initialization for both, and as a special case,
/// `Ctor`-based initialization looks identical to direct initialization when
/// the value is `Unpin`. For example, the `ctor!` macro looks like direct
/// initialization for everything `Unpin`.
///
/// A contrasting design might be to have a separate initialization syntax for
/// direct vs `Ctor`-based initialization. However, that would still likely need
/// to be restricted to `Unpin` types for safety.
// SAFETY: unconditionally initializes dest.
unsafe impl<T: Unpin> Ctor for T {
    type Output = T;
    type Error = Infallible;
    unsafe fn ctor(self, dest: *mut Self) -> Result<(), Infallible> {
        dest.write(self);
        Ok(())
    }
}

/// Constructs via a Rust move.
#[must_use = must_use_ctor!()]
pub struct RustMoveCtor<T, E = Infallible>(T, PhantomData<fn() -> E>);
impl<T, E> !Unpin for RustMoveCtor<T, E> {}

impl<T, E> RustMoveCtor<T, E> {
    pub fn new(x: T) -> Self {
        RustMoveCtor(x, PhantomData::default())
    }
}

// SAFETY: unconditionally initializes dest.
unsafe impl<T, E> Ctor for RustMoveCtor<T, E> {
    type Output = T;
    type Error = E;
    unsafe fn ctor(self, dest: *mut T) -> Result<(), E> {
        dest.write(self.0);
        Ok(())
    }
}

/// A `Ctor` which represents unreachable control flow.
///
/// This can be used instead of `!`, when an `impl Ctor` is needed:
///
/// ```
/// pub fn foo() -> impl Ctor<Output=Bar, Error=Infallible> {
///     todo!("TODO: implement foo");
///     #[allow(unreachable_code)]
///     UnreachableCtor::new()
/// }
/// ```
///
/// Unfortunately, it is not enough to have just a `todo!()`, because
/// `!` doesn't implement `Ctor<Output=Bar>`. (Or, if it did, it would
/// not implement `Ctor<Output=Baz>`.) `UnreachableCtor` implements
/// `Ctor<Output=T>` for any `T`, and can be used to satisfy the type
/// obligations.
#[must_use = must_use_ctor!()]
#[derive(Copy, Clone)]
pub struct UnreachableCtor<T: ?Sized, E = Infallible>(PhantomData<(fn() -> T, fn() -> E)>);
impl<T: ?Sized, E> UnreachableCtor<T, E> {
    pub fn new() -> Self {
        UnreachableCtor(PhantomData::default())
    }
}
impl<T: ?Sized, E> !Unpin for UnreachableCtor<T, E> {}

// TODO(jeanpierreda): Might be more interesting to make this return `Result<!, ()>`,
// but that requires unstable features, and also means it can't be used in `emplace!`.
// SAFETY: always panics, so trivially satisfies postconditions.
unsafe impl<T: ?Sized, E> Ctor for UnreachableCtor<T, E> {
    type Output = T;
    type Error = E;
    unsafe fn ctor(self, _: *mut T) -> Result<(), E> {
        unreachable!();
    }
}

/// All Rust types are C++-default-constructible if safe (i.e. Unpin + Default).
impl<T: Unpin + Default> CtorNew<()> for T {
    type CtorType = RustMoveCtor<Self>;
    type Error = Infallible;
    fn ctor_new(_: ()) -> Self::CtorType {
        RustMoveCtor::new(Default::default())
    }
}

/// All Unpin Rust types are C++-copyable if they are Rust-cloneable.
///
/// (Unpin is required for safety; otherwise, this would violate the Pin
/// guarantee.)
impl<T: Unpin + Clone> CtorNew<&T> for T {
    type CtorType = RustMoveCtor<Self>;
    type Error = Infallible;
    fn ctor_new(src: &Self) -> Self::CtorType {
        RustMoveCtor::new(src.clone())
    }
}

// ========
// emplace!
// ========
//
// The emplace!{} macro is now a little simpler, as it doesn't require StackBox
// in the public interface: it can use &mut. It also uses `super_let` to avoid
// the awkward sytnax otherwise required.

/// Emplace a constructor into a local or temporary.
///
/// This can be used similarly to the `pin!()` macro: `let x = emplace!(some_ctor)`, where
/// `some_ctor` evaluates to a `Ctor<Output=T>`. `x` will be a `Pin<&mut T>`.
///
/// `emplace!` only works with non-failable `Ctor`s. See `try_emplace` for
/// failable `Ctor`s.
#[macro_export]
#[allow_internal_unstable(super_let)]
// `super` gets removed by rustfmt, apparently.
#[rustfmt::skip]
macro_rules! emplace {
    ($expr:expr) => {
        {
            super let mut slot = $crate::Slot::unsafe_new();
            slot.unsafe_construct($expr).unsafe_as_pin_unchecked()
        }
    };
}

/// Attempt to emplace a constructor into a local or temporary.
///
/// The resulting value will be a `Result<Pin<&mut C::Output>, C::Error>` where `C` is the type of
/// the provided `Ctor`.
///
/// This is similar to `emplace!`, but can be used with fallible `Ctor`s (ones with an error type
/// that is not `Infallible`).
#[macro_export]
#[allow_internal_unstable(super_let)]
// `super` gets removed by rustfmt, apparently.
#[rustfmt::skip]
macro_rules! try_emplace {
    ($expr:expr) => {
        {
            super let mut slot = $crate::Slot::unsafe_new();
            slot.try_unsafe_construct($expr)
        }
    };
}

// ====
// Slot
// ====
//
// Alternate design: we could expose without the is_initialized flag, which
// would require that all users initialize the type before drop. We may still
// need such a type for interop, since that can be repr(transparent). But it is
// *exceptionally* dangerous, as it requires that all code paths will initialize
// before drop. For example, it means that Ctor is not allowed to panic.
//
// Hypothesis: at least for local variables and reasoning, rustc will be able to
// elide the bool and be equally performant, while also being substantially
// safer.

/// A pinned optional type, which can store in-place constructed objects.
///
/// To create a slot safely, it must be constructed in place, using (for
/// example) the `emplace!` macro. It then can operate as a structurally pinned
/// variant of `Option`, allowing for pinned access to the interior.
///
/// Examples:
///
/// ```
/// // Slots can be used to implement a "slotted return value".
/// fn slotted_return(slot: Pin<&mut Slot<u32>>) -> Pin<&mut u32> {
///     slot.replace(42)
/// }
///
/// let slot = emplace! (Slot::uninit());
/// let rv = slotted_return(slot);
/// assert_eq!(*rv, 42);
/// ```
///
/// ```
/// // Slots can also be used for plain output parameters.
/// fn slotted_out_param(slot: Pin<&mut Slot<u32>>) {
///     slot.replace(42);
/// }
///
/// let mut slot = emplace!(Slot::uninit());
/// slotted_out_param(slot.as_mut());
/// assert_eq!(*slot.as_opt().unwrap(), 42);
/// ```
pub struct Slot<T> {
    is_initialized: bool,
    maybe_uninit: MaybeUninit<T>,
}

impl<T> Drop for Slot<T> {
    fn drop(&mut self) {
        unsafe { Pin::new_unchecked(self) }.clear()
    }
}

impl<T> !Unpin for Slot<T> {}

impl<T> Slot<T> {
    pub fn uninit() -> impl Ctor<Output = Self, Error = Infallible> {
        RustMoveCtor::new(Self::unsafe_new())
    }

    pub fn new<C: Ctor<Output = T>>(value: C) -> impl Ctor<Output = Self, Error = C::Error> {
        RustMoveCtor::new(Self::unsafe_new()).ctor_then(|slot| {
            slot.try_replace(value)?;
            Ok(())
        })
    }

    pub fn clear(self: Pin<&mut Self>) {
        if self.is_initialized {
            let Self { is_initialized, maybe_uninit } = unsafe { Pin::into_inner_unchecked(self) };
            unsafe {
                core::ptr::drop_in_place(maybe_uninit.as_mut_ptr());
            }
            *is_initialized = false;
        }
    }

    /// Replace the value in the slot, and return a pinned reference to the
    /// result.
    ///
    /// Requires that the `Ctor` is infallible.
    pub fn replace<C: Ctor<Output = T, Error = Infallible>>(
        self: Pin<&mut Self>,
        value: C,
    ) -> Pin<&mut T> {
        self.try_replace(value).unwrap()
    }

    /// Replace the value in the slot, and return a pinned reference to the
    /// result. Returns an error if the `Ctor` returns an error.
    pub fn try_replace<C: Ctor<Output = T>>(
        mut self: Pin<&mut Self>,
        value: C,
    ) -> Result<Pin<&mut T>, C::Error> {
        self.as_mut().clear();
        {
            let Self { is_initialized, maybe_uninit } =
                unsafe { Pin::into_inner_unchecked(self.as_mut()) };
            // SAFETY: the memory is struturally pinned and valid for writes,
            // and it's uninitialized because of the .clear() call above.
            unsafe {
                value.ctor(maybe_uninit.as_mut_ptr())?;
            }
            *is_initialized = true;
        }
        Ok(self.as_opt_mut().unwrap())
    }

    pub fn as_opt_mut(self: Pin<&mut Self>) -> Option<Pin<&mut T>> {
        if self.is_initialized {
            Some(unsafe { Pin::into_inner_unchecked(self) }.unsafe_as_pin_unchecked())
        } else {
            None
        }
    }

    pub fn as_opt(&self) -> Option<&T> {
        if self.is_initialized {
            Some(unsafe { self.maybe_uninit.assume_init_ref() })
        } else {
            None
        }
    }
}

// Hidden: these are only for use by safety macros.
#[doc(hidden)]
impl<T> Slot<T> {
    pub fn unsafe_new() -> Self {
        Slot { is_initialized: false, maybe_uninit: MaybeUninit::uninit() }
    }

    // The following two functions are not marked `unsafe` so that they can be
    // called from a single expression in a macro, without an `unsafe{}` block
    // which would also cover macro parameters.
    //
    // One alternative would be an immediately-invoked lambda, but this forces type
    // inference, which leads to confusing error messages when it isn't
    // inferrable. And, since this is a lambda, the type is not known concretely
    // to begin with!
    //
    // We could instead use a generic function with `impl Ctor` and the like, but
    // this function would be unsafe... leading back to the very original
    // problem!
    //
    // So here we bite the bullet: these functions should ideally be marked
    // "unsafe", but, seeing as they are only invoked from a macro, and that
    // macro needs to invoke expressions in calls to these functions, they are
    // only *named* "unsafe_foo" instead.

    /// Safety: must not have already been constructed, as that would violate
    /// the pin guarantee.
    pub fn unsafe_construct(
        &mut self,
        ctor: impl Ctor<Output = T, Error = Infallible>,
    ) -> &mut Self {
        unsafe { ctor.ctor(self.maybe_uninit.as_mut_ptr()).unwrap() };
        self.is_initialized = true;
        self
    }

    /// Safety: must not have already been constructed, as that would violate
    /// the pin guarantee.
    pub fn try_unsafe_construct<C: Ctor<Output = T>>(
        &mut self,
        ctor: C,
    ) -> Result<Pin<&mut T>, C::Error> {
        unsafe { ctor.ctor(self.maybe_uninit.as_mut_ptr())? };
        self.is_initialized = true;
        Ok(self.unsafe_as_pin_unchecked())
    }

    /// Safety: pin guarantee, assumes init.
    pub fn unsafe_as_pin_unchecked(&mut self) -> Pin<&mut T> {
        unsafe { Pin::new_unchecked(self.maybe_uninit.assume_init_mut()) }
    }
}

#[doc(hidden)]
pub mod macro_internal {
    use super::*;
    pub use core::mem::MaybeUninit;
    pub use core::pin::Pin;

    /// Workaround for more_qualified_paths.
    /// Instead of `<Foo as Bar>::Assoc { ... }`, which requires that feature,
    /// we can use `Identity<<Foo as Bar>::Assoc> { ... }`.
    ///
    /// See https://github.com/rust-lang/rust/issues/86935#issuecomment-1146670057
    ///
    /// TODO(jeanpierreda): Delete this when the feature is stabilized.
    pub type Identity<T> = T;

    /// Trait which causes compilation error if a `#[recursively_pinned]` struct
    /// impls `Drop`.
    ///
    /// Idea from https://docs.rs/pin-project/latest/pin_project/attr.pin_project.html#safety
    pub trait DoNotImplDrop {}
    #[allow(drop_bounds)]
    impl<T: Drop> DoNotImplDrop for T {}

    /// Drops a pointer when dropped.
    ///
    /// Safety: this will drop the pointer, so this is only safe if no other
    /// code will access the pointer afterwards. This includes drop: the
    /// pointer must itself be in a ManuallyDrop.
    pub struct UnsafeDropGuard<T>(*mut T);
    impl<T> Drop for UnsafeDropGuard<T> {
        fn drop(&mut self) {
            unsafe { core::ptr::drop_in_place(self.0) };
        }
    }

    /// Initializes a field, and returns a drop guard which will drop that
    /// field.
    ///
    /// Intended use: when initializing a struct field-by-field, each
    /// initialized field should be guarded in case of panic. Once the
    /// struct is completely initialized, the drop guards can be forgotten
    /// with `std::mem::forget()`. See the `ctor!` macro, where this is used.
    ///
    /// Safety: the field must satisfy the Pin guarantee.
    pub unsafe fn init_field<T>(
        field: *mut T,
        ctor: impl Ctor<Output = T, Error = Infallible>,
    ) -> impl Drop {
        // safety: the field is not yet initialized, the caller guarantees it's
        // pinned.
        Ctor::ctor(ctor, field).unwrap();
        UnsafeDropGuard(field)
    }

    pub fn require_recursively_pinned<_T: RecursivelyPinned>() {}
}

// =====================
// #[recursively_pinned]
// =====================

/// The `RecursivelyPinned` trait asserts that when the struct is pinned, every
/// (non-Unpin) field is also pinned.
///
/// This trait is automatically implemented for any `#[recursively_pinned]`
/// struct.
///
/// ## Safety
///
/// * All fields of the struct are pinned in all code paths that access them.
/// * The `CtorInitializedFields` type has the same fields as `Self`, except
///   for any fields that do not need initialization. (For instance, ZST fields
///   or fields of type `MaybeUninit`.)
pub unsafe trait RecursivelyPinned {
    /// An associated type with the same fields as `Self`, minus any ZST
    /// fields hich are not initialized by the `ctor!()` macro.
    ///
    /// The intended use of this is to help disable direct initialization of
    /// the type and *require* the use of `ctor!()`. This forces the struct
    /// to only ever exist in a pinned state.
    ///
    /// ```
    /// // (Alternatively, `#[non_exhaustive]` may be used instead of the private field.)
    /// pub struct CtorOnly {
    ///   pub field: i32,
    ///   _must_construct_using_ctor: [(); 0],
    /// }
    ///
    /// // The same struct, but without the private field.
    /// // (Alternatively, without `#[non_exhaustive]`.)
    /// pub struct CtorOnlyFields {
    ///   pub field: i32,
    /// }
    ///
    /// unsafe impl RecursivelyPinned for CtorOnly {
    ///   type CtorInitializedFields = CtorOnlyFields;
    /// }
    /// ```
    ///
    /// By using `CtorInitializedFields` paired with a private field (or
    /// `#[non_exhaustive]`), the following code is now invalid:
    ///
    /// ```ignore
    /// # // TODO(jeanpierreda): make this tested, somehow.
    /// // Fails to compile: did not specify _must_construct_using_ctor, and cannot,
    /// // because it is private
    /// let x = CtorOnly {field: 3};
    /// ```
    ///
    /// While construction using `ctor!()` works fine:
    ///
    /// ```ignore
    /// let x = emplace!(ctor!(CtorOnly {field: 3}));
    /// ```
    ///
    /// The size and layout of `CtorInitializedFields` is ignored; it only
    /// affects which field names are required for complete `ctor!()`
    /// initialization. Any fields left out of the `CtorInitializedFields` type
    /// will not be initialized, so they must permit uninitialized memory.
    /// (For example, ZST or MaybeUninit.)
    type CtorInitializedFields;
}

/// The drop trait for `#[recursively_pinned(PinnedDrop)]` types.
///
/// It is never valid to implement `Drop` for a recursively-pinned type, as this
/// would be unsound: the `&mut self` in `drop` would allow the pin guarantee to
/// be violated.
///
/// Instead, if such a struct is to implement drop, it must pass `PinnedDrop` to
/// `recursively_pinned`, and implement the `PinnedDrop` trait.
///
/// See also the [analogous `pin_project` feature](https://docs.rs/pin-project/latest/pin_project/attr.pinned_drop.html)
pub trait PinnedDrop {
    /// Run the drop logic for self.
    ///
    /// ## Safety
    ///
    /// If called from anywhere other than the automatically-generated
    /// `Drop::drop`, the behavior is undefined.
    ///
    /// To manually drop the value, use `ManuallyDrop` or use
    /// `std::ptr::drop_in_place` (etc.) instead.
    unsafe fn pinned_drop(self: Pin<&mut Self>);
}

// =====
// ctor!
// =====

/// The `ctor!` macro evaluates to a `Ctor` for a Rust struct, with
/// user-specified fields.
///
/// Example use:
///
/// ```
/// fn new() -> impl Ctor<Output=MyStruct> {
///   ctor!(MyStruct {field_1: default_ctor(), field_2: 42})
/// }
///
/// // Actually invoke the Ctor to create a new MyStruct:
/// let mut my_struct = emplace!(MyStruct::new());
///
/// The type must implement `RecursivelyPinned`, so that `ctor!()` can
/// safely construct the struct pinned in place.
/// ```
#[macro_export]
macro_rules! ctor {
    // Note: we're using `ident (::ident)*` for the type names because neither `ident` nor `path`
    // really work perfectly -- `ident` is great, except that `foo::bar` isn't an ident. `path` is
    // great, except that e.g. parentheses can't follow a path. (This is not fixable: FnOnce(T) is
    // also a path, so parens can't follow paths due to the ambiguous parse). Thus... we use ident,
    // and manually reconstruct the path.
    //
    // TODO(jeanpierreda): support <X as Y> and foo<Z> in paths.
    //
    // tt is used for the field names, because this allows for both integer fields for tuple
    // structs, and named fields for non-tuple structs.
    ( $t:ident $(:: $ts:ident)* $(< $($gp:tt),+ >)? {$( $name:tt: $sub_ctor:expr ),* $(,)?} ) => {
        {
            use $t $(:: $ts)* as Type;
            $crate::FnCtor::new(|x: *mut Type $(< $( $gp ),+ >)?| {
                struct DropGuard;
                let drop_guard = DropGuard;
                let _ = &x; // silence unused_variables warning if Type is fieldless.

                // Enforce that the ctor!() expression resembles a valid direct initialization
                // expression, by using the names in a conventional literal.
                // For structs, this fails to compile unless the names are fully exhaustive.
                // For unions, it fails to compile unless precisely one name is used.
                // In both cases, this ensures that we only compile when expressions corresponding
                // to normal init are used, with unsurprising semantics.
                let _ = |x: Type $(< $( $gp ),+ >)?| {
                    let _ = &x; // silence unused_variables warning if Type is fieldless.
                    // If this fails to compile, not every field was specified in the ctor! invocation.
                    // The `panic!(...)` allows us to avoid moving out of x, while still pretending to
                    // fill in each field.
                    #[allow(unreachable_code, unused_unsafe)] $crate::macro_internal::Identity::<
                        <Type $(< $( $gp ),+ >)? as $crate::RecursivelyPinned>::CtorInitializedFields> {
                            // SAFETY: this code is not executed.
                            // The unsafe {} block is in case this is a *union* literal, rather than
                            // a struct literal.
                            $($name: panic!("{}", unsafe {&x.$name} as *const _ as usize),)*
                        };
                };

                // Enforce that the type is RecursivelyPinned.
                $crate::macro_internal::require_recursively_pinned::<Type $(< $( $gp ),+ >)?>();

                $(
                    let sub_ctor = $sub_ctor; // outside of unsafe block.
                    let field_drop = unsafe {
                        // SAFETY: the place is in bounds, just uninitialized. See e.g. second
                        // example: https://doc.rust-lang.org/nightly/std/ptr/macro.addr_of_mut.html
                        $crate::macro_internal::init_field(
                            &raw mut (*x).$name,
                            sub_ctor)
                    };
                    let drop_guard = (drop_guard, field_drop);
                )*
                ::core::mem::forget(drop_guard);
            })
        }
    };

    // Unit struct ctor.
    ($t:ident $(:: $ts:ident)*) => {$crate::ctor!($t $(:: $ts)* {  })};

    // Conventional tuple struct syntax (with parens, no integer names) supported for < 8 fields.
    // Otherwise, use MyTupleStruct{0: ..., 1: ...} syntax, which works for any number of fields.
    // Generated as so:
    /* python3 -c 'for i in range(8):
        ctor_ins = ", ".join(f"$ctor_{j}:expr" for j in range(i))
        ctor_outs = ", ".join(f"{j}: $ctor_{j}" for j in range(i))
        print(f"    ($t:ident $(:: $ts:ident)* ({ctor_ins})) => {{$crate::ctor!($t $(:: $ts)* {{ {ctor_outs} }})}};")' */
    ($t:ident $(:: $ts:ident)* ()) => {$crate::ctor!($t $(:: $ts)* {  })};
    ($t:ident $(:: $ts:ident)* ($ctor_0:expr)) => {$crate::ctor!($t $(:: $ts)* { 0: $ctor_0 })};
    ($t:ident $(:: $ts:ident)* ($ctor_0:expr, $ctor_1:expr)) => {$crate::ctor!($t $(:: $ts)* { 0: $ctor_0, 1: $ctor_1 })};
    ($t:ident $(:: $ts:ident)* ($ctor_0:expr, $ctor_1:expr, $ctor_2:expr)) => {$crate::ctor!($t $(:: $ts)* { 0: $ctor_0, 1: $ctor_1, 2: $ctor_2 })};
    ($t:ident $(:: $ts:ident)* ($ctor_0:expr, $ctor_1:expr, $ctor_2:expr, $ctor_3:expr)) => {$crate::ctor!($t $(:: $ts)* { 0: $ctor_0, 1: $ctor_1, 2: $ctor_2, 3: $ctor_3 })};
    ($t:ident $(:: $ts:ident)* ($ctor_0:expr, $ctor_1:expr, $ctor_2:expr, $ctor_3:expr, $ctor_4:expr)) => {$crate::ctor!($t $(:: $ts)* { 0: $ctor_0, 1: $ctor_1, 2: $ctor_2, 3: $ctor_3, 4: $ctor_4 })};
    ($t:ident $(:: $ts:ident)* ($ctor_0:expr, $ctor_1:expr, $ctor_2:expr, $ctor_3:expr, $ctor_4:expr, $ctor_5:expr)) => {$crate::ctor!($t $(:: $ts)* { 0: $ctor_0, 1: $ctor_1, 2: $ctor_2, 3: $ctor_3, 4: $ctor_4, 5: $ctor_5 })};
    ($t:ident $(:: $ts:ident)* ($ctor_0:expr, $ctor_1:expr, $ctor_2:expr, $ctor_3:expr, $ctor_4:expr, $ctor_5:expr, $ctor_6:expr)) => {$crate::ctor!($t $(:: $ts)* { 0: $ctor_0, 1: $ctor_1, 2: $ctor_2, 3: $ctor_3, 4: $ctor_4, 5: $ctor_5, 6: $ctor_6 })};

}

// ==========
// CtorNew traits
// ==========
//
// Finally, we introduce some new traits for assignment.

/// Destroy-then-reconstruct. Sidesteps `operator=`, instead reconstructing
/// in-place.
///
/// If the object cannot be destroyed/reconstructed in place (e.g. it is a base
/// class subobject), the behavior is undefined.
///
/// If `ctor` unwinds, the process will crash.
///
/// This is a bit more fragile than, and a lot less common than, `operator=`,
/// but allows for taking advantage of copy/move elision more aggressively,
/// rather than requiring materialization into a temporary before triggering
/// assignment.
///
/// That means that e.g. instead of calling `x.assign(&*emplace!(foo))`, you can
/// directly call `x.reconstruct_unchecked(foo)` -- provided you are OK with the
/// differing constructor/destructor ordering, and satisfy safety criteria.
///
/// # Safety
///
/// Dividing sources of UB by language ruleset:
///
/// **C++**: The behavior is undefined if `self` is a base class subobject
/// or `[[no_unique_address]]` member.
///
/// See: http://eel.is/c++draft/basic.life#8
///
/// And see https://chat.google.com/room/AAAAl3r59xQ/auNOifgQa1c for discussion.
///
/// **Rust**: This is safe. Note that since this calls `drop()` on
/// the pinned pointer, it satisfies the pin guarantee, and is allowed to then
/// re-init it with something else. In effect, this is just the in-place Ctor
/// version of the existing method `Pin<T>::set(T)`.
pub trait ReconstructUnchecked: Sized {
    /// # Safety
    /// See trait documentation.
    unsafe fn reconstruct_unchecked(
        self: Pin<&mut Self>,
        ctor: impl Ctor<Output = Self, Error = Infallible>,
    ) {
        let self_ptr = Pin::into_inner_unchecked(self) as *mut _;
        core::ptr::drop_in_place(self_ptr);
        abort_on_unwind(move || {
            ctor.ctor(self_ptr).unwrap();
        });
    }
}

impl<T> ReconstructUnchecked for T {}

/// Destroy-then-reconstruct. Sidesteps `operator=`, instead reconstructing
/// in-place.
///
/// If `ctor` unwinds, the process will crash.
///
/// This is a bit more fragile than, and a lot less common than, `operator=`,
/// but allows for taking advantage of copy/move elision more aggressively,
/// rather than requiring materialization into a temporary before triggering
/// assignment.
///
/// That means that e.g. instead of calling `x.assign(&*emplace!(foo))`, you can
/// directly call `x.reconstruct(foo)` -- provided you are OK with the differing
/// constructor/destructor ordering.
///
/// # Implementation safety
///
/// Implementors must ensure that it is safe to destroy and reconstruct the
/// object. Most notably, if `Self` is a C++ class, it must not be a base class.
/// See http://eel.is/c++draft/basic.life#8
///
/// # Safety
///
/// Note that this is not safe to call on `[[no_unique_address]]` member
/// variables, but the interface is marked safe, because those can only be
/// produced via unsafe means.
pub unsafe trait Reconstruct: ReconstructUnchecked {
    fn reconstruct(self: Pin<&mut Self>, ctor: impl Ctor<Output = Self, Error = Infallible>) {
        unsafe { self.reconstruct_unchecked(ctor) };
    }
}

/// Safety: anything implementing `Unpin` is Rust-assignable, and
/// Rust-assignment is inherently destroy+reconstruct.
unsafe impl<T: Unpin> Reconstruct for T {}

/// Run f, aborting if it unwinds.
///
/// Because this aborts on unwind, f is not required to be unwind-safe.
#[inline]
fn abort_on_unwind<T, F: FnOnce() -> T>(f: F) -> T {
    // Here is another way we COULD implement abort_on_panic:

    // let f = std::panic::AssertUnwindSafe(f);
    // let result = std::panic::catch_unwind(move || f.0());
    // if result.is_err() {
    //     std::process::abort();
    // }

    // This would work, even for `extern "C-unwind"`, but that wasn't
    // always the case. See https://doc.rust-lang.org/std/panic/fn.catch_unwind.html#notes
    // and compare with https://rust-lang.github.io/rfcs/2945-c-unwind-abi.html

    /// A safety guard which panics if dropped, converting unwinds into aborts.
    ///
    /// In general, you can't, as the *author* of drop(), assume that it will be
    /// called: callers can just call std::mem::forget(), among other
    /// things. However, as the *user*, you can know that drop is called for
    /// unmoved values at the end of a drop scope. Drop is a defined behavior.
    /// So while there is ordinarily a prohibition on relying on drop for
    /// safety, that only occurs for API owners that are allowing values to
    /// be used in arbitrary ways (including forget). Here, we are API
    /// *users* as well, and specifically using the Bomb in a way that
    /// guarantees its drop will be called in a particular circumstance.
    ///
    /// See https://doc.rust-lang.org/reference/destructors.html, specifically this: "When control
    /// flow leaves a drop scope all variables associated to that scope are
    /// dropped in reverse order of declaration (for variables) or creation
    /// (for temporaries)."
    struct Bomb;
    impl Drop for Bomb {
        fn drop(&mut self) {
            panic!("Unwinding occurred when no safe recovery is possible.");
        }
    }

    let bomb = Bomb;
    let rv = f();
    core::mem::forget(bomb);
    rv
}

/// Overloaded assignment operator.
///
/// Conventionally, C++ copy-assignment is Assign<&T>, and C++ move-assignment
/// is Assign<RvalueReference<'_, T>>.
pub trait Assign<From> {
    fn assign(self: Pin<&mut Self>, src: From);
}

/// Assignment from a Copy desugars to an assignment from a &T.
impl<T: for<'a> Assign<&'a T>, P: Deref<Target = T>> Assign<Copy<P>> for T {
    fn assign(self: Pin<&mut Self>, src: Copy<P>) {
        self.assign(&*src.0);
    }
}

// TODO(jeanpierreda): Make these less repetitive.

impl<'a, T> Assign<&'a T> for T
where
    T: Unpin + CtorNew<&'a T, Error = Infallible>,
{
    fn assign(mut self: Pin<&mut Self>, src: &'a Self) {
        if core::mem::needs_drop::<Self>() {
            let mut constructed = MaybeUninit::uninit();
            unsafe {
                T::ctor_new(src).ctor(constructed.as_mut_ptr()).unwrap();
                *self = constructed.assume_init();
            }
        } else {
            self.reconstruct(T::ctor_new(src));
        }
    }
}

impl<'a, T> Assign<RvalueReference<'a, T>> for T
where
    T: Unpin + CtorNew<RvalueReference<'a, T>, Error = Infallible>,
{
    fn assign(mut self: Pin<&mut Self>, src: RvalueReference<'a, Self>) {
        if core::mem::needs_drop::<Self>() {
            let mut constructed = MaybeUninit::uninit();
            unsafe {
                T::ctor_new(src).ctor(constructed.as_mut_ptr()).unwrap();
                *self = constructed.assume_init();
            }
        } else {
            self.reconstruct(T::ctor_new(src));
        }
    }
}

impl<'a, T> Assign<ConstRvalueReference<'a, T>> for T
where
    T: Unpin + CtorNew<ConstRvalueReference<'a, T>, Error = Infallible>,
{
    fn assign(mut self: Pin<&mut Self>, src: ConstRvalueReference<'a, Self>) {
        if core::mem::needs_drop::<Self>() {
            let mut constructed = MaybeUninit::uninit();
            unsafe {
                T::ctor_new(src).ctor(constructed.as_mut_ptr()).unwrap();
                *self = constructed.assume_init();
            }
        } else {
            self.reconstruct(T::ctor_new(src));
        }
    }
}

/// Overloaded assignment operator, but for Unpin types
///  TODO(b/219963671): use specialization instead of a distinct trait
pub trait UnpinAssign<From> {
    fn unpin_assign(&mut self, src: From);
}

// =======================
// Constructor overloading
// =======================
//
// Constructors are unique among C++ functions in that overloading is common,
// *not avoidable*, and involves a nameless function best captured by a trait.
// In other words, it is the ideal choice for having a more generic trait as
// with From/Into.
//
// For exploration purposes, so that we can play with both approaches at the
// same time, this is built on *top* of the traits above.

/// Overloaded constructor trait.
///
/// `T : CtorNew<(A, B, C)>` is roughly equivalent to the C++ "T has a
/// constructor taking arguments of type A, B, and C". As an obvious special
/// case, for singleton tuples, you may use `CtorNew<A>`.
pub trait CtorNew<ConstructorArgs> {
    type CtorType: Ctor<Output = Self, Error = Self::Error>;
    type Error;

    fn ctor_new(args: ConstructorArgs) -> Self::CtorType;
}

// ====
// Misc
// ====

/// A constructor for `PhantomPinned`.
///
/// ## Why doesn't `PhantomPinned` implement `Ctor<Output=Self>` ?
///
/// A self-impl, where `PhantomPinned : Ctor<Output=PhantomPinned>` would also
/// have been safe, even though `PhantomPinned` is `!Unpin`, because it's just a
/// marker. However, the trait coherence rules are not happy about this:
///
/// ```
/// // crate_1
/// #![feature(negative_impls)]
/// pub struct Foo;
/// impl !Unpin for Foo {}
/// ```
///
/// ```
/// // crate_2
/// trait MyTrait {}
/// impl<T: Unpin> MyTrait for T{}
/// impl MyTrait for crate_1::Foo {}
/// ```
///
/// (In our case, `Foo` is `std::marker::PhantomPinned`, and `MyTrait` is
/// `Ctor`)
///
/// Within `crate_1`, Rust understands that `Foo` is `!Unpin`. So `impl MyTrait
/// for Foo {}` is valid in `crate_1`. However, outside of `crate_1`, Rust
/// decides it doesn't know whether `Foo` implements `Unpin` or not, and so
/// incorrectly believes that the impls may conflict.
///
/// So while it is perfectly feasible to implement self-construction for any
/// type locally, we cannot give foreign impls.
pub struct PhantomPinnedCtor;
// SAFETY: unconditionally initializes dest.
unsafe impl Ctor for PhantomPinnedCtor {
    type Output = core::marker::PhantomPinned;
    type Error = Infallible;
    unsafe fn ctor(self, dest: *mut Self::Output) -> Result<(), Infallible> {
        RustMoveCtor::new(core::marker::PhantomPinned).ctor(dest)
    }
}
impl !Unpin for PhantomPinnedCtor {}

/// A constructor for ManuallyDrop<T>, given a constructor for T.
///
/// ManuallyDrop is special as the only non-Copy type allowed in a union, so we
/// specifically support its use, even though it is not guaranteed to be
/// structurally pinned.
#[must_use = must_use_ctor!()]
pub struct ManuallyDropCtor<T: Ctor>(T);

impl<T: Ctor> ManuallyDropCtor<T> {
    /// Safety: this structurally pins the contents of ManuallyDrop.
    /// Therefore, it is not safe to use with anything that assumes that
    /// ManuallyDrop is not structurally pinned.
    pub unsafe fn new(x: T) -> Self {
        ManuallyDropCtor(x)
    }
}

// SAFETY: unconditionally initializes dest.
unsafe impl<T: Ctor> Ctor for ManuallyDropCtor<T> {
    type Output = ManuallyDrop<T::Output>;
    type Error = T::Error;
    unsafe fn ctor(self, dest: *mut Self::Output) -> Result<(), Self::Error> {
        // Safety: ManuallyDrop<T> and T have the same layout.
        // All other preconditions are satisfied by the caller.
        self.0.ctor(dest as *mut _)
    }
}

impl<T: Ctor> !Unpin for ManuallyDropCtor<T> {}
