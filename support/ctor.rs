// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#![cfg_attr(not(test), no_std)]
#![feature(auto_traits, negative_impls, allow_internal_unstable)]
#![allow(internal_features)] // allow_internal_unstable 🤔

//! Support for in-place construction and mutation of Rust types, especially those that were
//! originally defined in C++.
//!
//! This draws from moveit, pin-init, and the current in-place initialization proposal at
//! https://hackmd.io/@aliceryhl/BJutRcPblx.
//!
//! * Introductory documentation: crubit.rs/types/non_rust_movable/intro_short
//! * Full documentation: crubit.rs/types/non_rust_movable/intro_advanced
//! * Cheat sheet: crubit.rs/types/non_rust_movable/cheat_sheet
//!
//! # Unstable Rust Features
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

pub use ctor_proc_macros::{recursively_pinned, CtorFrom_Default, MoveAndAssignViaCopy};

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

/// A trait for pinned in-place initialization of values.
///
/// A parameter or return value of type `impl Ctor<Output=T, Error=Infallible>` (commonly spelled
/// `Ctor![T]`) is analogous to a parameter or return value of type `T`, except that it can be
/// initialized in place and pinned. This means that the construction logic can assume that its
/// address will never change, and can construct values directly into their final location.
///
/// The `Ctor` implementation doesn't represent the value itself, but deferred initialization for
/// that value. To actually construct it, you must use `emplace!()` or methods like `Box::emplace`
/// from the `Emplace` trait, which are analogous to `pin!()` and `Box::pin` respectively.
///
/// More generally, because passing or returning `Ctor` is analogous to passing or returning by
/// value, there is a corresponding family of analogues for things you might do by value:
///
/// * `ctor!{MyStruct{a: b})` is analogous to `MyStruct {a: b}`, but constructs the field and the
///   struct itself in-place. `b` is a `Ctor`, and the expression evaluates to a `Ctor`.
/// * `CtorNew<T>` is analogous to `From<T>`, except it returns a `Ctor![Self]`.
/// * `emplace!(ctor)` is analogous to `pin!(value)`, but it evaluates the `ctor` to construct
///   the value in-place.
/// * The `Emplace` trait is analogous to methods like `Box::pin`, but accepts a `Ctor![T]`.
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
    /// `dest` is valid for writes and uninitialized.
    ///
    /// This function pins `dest`, so unless `Output: Unpin`, `*dest` must not be moved or otherwise
    /// invalidated.
    unsafe fn ctor(self, dest: *mut Self::Output) -> Result<(), Self::Error>;

    /// Converts a `Ctor` with error type `Infallible` to one with the provided error type `E`.
    ///
    /// This is useful for chaining possibly-fallible operations (such as `ctor_then`) on top of
    /// an existing infallible `Ctor`.
    fn ctor_make_fallible<E>(self) -> CtorMakeFallible<Self, E>
    where
        Self: Ctor<Error = Infallible>,
    {
        CtorMakeFallible { ctor: self, marker: PhantomData }
    }

    /// Maps this `Ctor`'s error type into a new error type using the `Into` trait.
    fn ctor_err_into<E>(self) -> CtorErrInto<Self, E>
    where
        Self::Error: Into<E>,
    {
        CtorErrInto { ctor: self, marker: PhantomData }
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
    fn ctor_then<F>(self, f: F) -> CtorThen<Self, F>
    where
        F: FnOnce(Pin<&mut Self::Output>) -> Result<(), Self::Error>,
    {
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
    fn ctor_map_err<F, E>(self, f: F) -> CtorMapErr<Self, F, E>
    where
        F: FnOnce(Self::Error) -> E,
    {
        CtorMapErr { ctor: self, f }
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
    fn ctor_or_else<F, O>(self, f: F) -> CtorOrElse<Self, F, O>
    where
        F: FnOnce(Self::Error) -> O,
        O: Ctor<Output = Self::Output>,
    {
        CtorOrElse { ctor: self, f }
    }

    /// Returns a `Ctor` which will panic with the given message if the original construction
    /// fails.
    ///
    /// This functions similarly to `Result::expect`.
    fn ctor_expect<'a>(self, msg: &'a str) -> CtorExpect<'a, Self>
    where
        Self::Error: Debug,
    {
        CtorExpect { ctor: self, msg }
    }

    /// Returns a `Ctor` which will panic if the original construction fails.
    ///
    /// This functions similarly to `Result::unwrap`.
    fn ctor_unwrap(self) -> CtorUnwrap<Self>
    where
        Self::Error: Debug,
    {
        CtorUnwrap { ctor: self }
    }

    /// Returns a `Ctor` which will return a default value if the original construction fails.
    ///
    /// This functions similarly to `Result::unwrap_or_default`.
    fn ctor_unwrap_or_default(self) -> CtorUnwrapOrDefault<Self>
    where
        Self::Output: CtorNew<()>,
    {
        CtorUnwrapOrDefault { ctor: self }
    }
}

// =======================
// Ctor trait return types
// =======================

/// Return type of [`Ctor::ctor_make_fallible`].
pub struct CtorMakeFallible<C, E>
where
    C: Ctor<Error = Infallible>,
{
    ctor: C,
    marker: PhantomData<fn() -> E>,
}

impl<C, E> !SelfCtor for CtorMakeFallible<C, E> where C: Ctor<Error = Infallible> {}

unsafe impl<C, E> Ctor for CtorMakeFallible<C, E>
where
    C: Ctor<Error = Infallible>,
{
    type Output = C::Output;
    type Error = E;
    unsafe fn ctor(self, dest: *mut Self::Output) -> Result<(), Self::Error> {
        unsafe { self.ctor.ctor(dest).map_err(|e| match e {}) }
    }
}

/// Return type of [`Ctor::ctor_err_into`].
pub struct CtorErrInto<C, E>
where
    C: Ctor,
    C::Error: Into<E>,
{
    ctor: C,
    marker: PhantomData<fn() -> E>,
}

impl<C, E> !SelfCtor for CtorErrInto<C, E>
where
    C: Ctor,
    C::Error: Into<E>,
{
}

unsafe impl<C, E> Ctor for CtorErrInto<C, E>
where
    C: Ctor,
    C::Error: Into<E>,
{
    type Output = C::Output;
    type Error = E;
    unsafe fn ctor(self, dest: *mut Self::Output) -> Result<(), Self::Error> {
        unsafe { self.ctor.ctor(dest).map_err(Into::into) }
    }
}

/// Return type of [`Ctor::ctor_then`].
pub struct CtorThen<C: Ctor, F: FnOnce(Pin<&mut C::Output>) -> Result<(), C::Error>> {
    ctor: C,
    f: F,
}

impl<C, F> !SelfCtor for CtorThen<C, F>
where
    C: Ctor,
    F: FnOnce(Pin<&mut C::Output>) -> Result<(), C::Error>,
{
}

// SAFETY: unconditionally initializes dest.
unsafe impl<C: Ctor, F: FnOnce(Pin<&mut C::Output>) -> Result<(), C::Error>> Ctor
    for CtorThen<C, F>
{
    type Output = C::Output;
    type Error = C::Error;
    unsafe fn ctor(self, dest: *mut Self::Output) -> Result<(), Self::Error> {
        unsafe {
            self.ctor.ctor(dest)?;
        }
        let dest = unsafe { Pin::new_unchecked(&mut *dest) };
        (self.f)(dest)
    }
}

/// Return type of [`Ctor::ctor_map_err`].
pub struct CtorMapErr<C, F, E>
where
    C: Ctor,
    F: FnOnce(C::Error) -> E,
{
    ctor: C,
    f: F,
}

impl<C, F, E> !SelfCtor for CtorMapErr<C, F, E>
where
    C: Ctor,
    F: FnOnce(C::Error) -> E,
{
}

unsafe impl<C, F, E> Ctor for CtorMapErr<C, F, E>
where
    C: Ctor,
    F: FnOnce(C::Error) -> E,
{
    type Output = C::Output;
    type Error = E;
    unsafe fn ctor(self, dest: *mut Self::Output) -> Result<(), Self::Error> {
        unsafe { self.ctor.ctor(dest).map_err(|e| (self.f)(e)) }
    }
}

/// Return type of [`Ctor::ctor_or_else`].
pub struct CtorOrElse<C, F, O>
where
    C: Ctor,
    F: FnOnce(C::Error) -> O,
    O: Ctor<Output = C::Output>,
{
    ctor: C,
    f: F,
}

impl<C, F, O> !SelfCtor for CtorOrElse<C, F, O>
where
    C: Ctor,
    F: FnOnce(C::Error) -> O,
    O: Ctor<Output = C::Output>,
{
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
        unsafe {
            let Err(e) = self.ctor.ctor(dest) else { return Ok(()) };
            (self.f)(e).ctor(dest)
        }
    }
}

/// Return type of [`Ctor::ctor_expect`].
pub struct CtorExpect<'a, C>
where
    C: Ctor,
{
    ctor: C,
    msg: &'a str,
}

impl<'a, C> !SelfCtor for CtorExpect<'a, C> where C: Ctor {}

unsafe impl<'a, C> Ctor for CtorExpect<'a, C>
where
    C: Ctor,
    C::Error: Debug,
{
    type Output = C::Output;
    type Error = Infallible;
    unsafe fn ctor(self, dest: *mut Self::Output) -> Result<(), Self::Error> {
        unsafe {
            self.ctor.ctor(dest).unwrap_or_else(|e| panic!("{}: {:?}", self.msg, e));
            Ok(())
        }
    }
}

/// Return type of [`Ctor::ctor_unwrap`].
pub struct CtorUnwrap<C>
where
    C: Ctor,
{
    ctor: C,
}

impl<C> !SelfCtor for CtorUnwrap<C> where C: Ctor {}

unsafe impl<C> Ctor for CtorUnwrap<C>
where
    C: Ctor,
    C::Error: Debug,
{
    type Output = C::Output;
    type Error = Infallible;
    unsafe fn ctor(self, dest: *mut Self::Output) -> Result<(), Self::Error> {
        unsafe {
            self.ctor.ctor(dest).unwrap_or_else(|e| {
                panic!(
                    "Construction of {type_name} failed: {e:?}",
                    type_name = core::any::type_name::<C::Output>(),
                )
            });
            Ok(())
        }
    }
}

/// Return type of [`Ctor::ctor_unwrap_or_default`].
pub struct CtorUnwrapOrDefault<C>
where
    C: Ctor,
    C::Output: CtorNew<()>,
{
    ctor: C,
}

impl<C> !SelfCtor for CtorUnwrapOrDefault<C>
where
    C: Ctor,
    C::Output: CtorNew<()>,
{
}

unsafe impl<C> Ctor for CtorUnwrapOrDefault<C>
where
    C: Ctor,
    C::Output: CtorNew<()>,
{
    type Output = C::Output;
    type Error = <C::Output as CtorNew<()>>::Error;
    unsafe fn ctor(self, dest: *mut Self::Output) -> Result<(), Self::Error> {
        unsafe {
            if self.ctor.ctor(dest).is_err() {
                <C::Output as CtorNew<()>>::ctor_new(()).ctor(dest)?;
            }
            Ok(())
        }
    }
}

/// The type of a value that will be constructed in-place.
///
/// This expands to `impl Ctor<...>`, and is used as a parameter or return type.
///
/// This exists for two reasons:
///
/// 1. To allow for changing the implementation over time.
/// 2. To make the spelling less overly verbose. Instead of `impl Ctor<Output = T, Error = E>`,
///    you can write `Ctor![T]`.
///
/// In codebases not yet migrated to the 2024 edition, if you need a `use<'a>` bound, you must
/// write the full `impl` syntax: `impl Ctor<Output=T, Error=Infallible> + use<'a, 'b>`.
///
/// (It is surprisingly difficult to write a macro that would allow the `use` parameters!)
#[macro_export]
macro_rules! Ctor {
    ( $T:ty $(,)? ) => {
        impl $crate::Ctor<Output = $T, Error = $crate::macro_internal::Infallible>
    };
    ( $T:ty, Error = $E:ty $(,)? ) => {
        impl $crate::Ctor<Output = $T, Error = $E>
    };
}

pub struct CtorError<T: ?Sized, E> {
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

impl<T: ?Sized, E> !SelfCtor for CtorError<T, E> {}

/// Returns a `Ctor` with error type `E` which always fails with the given error.
pub fn ctor_error<T: ?Sized, E>(e: E) -> CtorError<T, E> {
    CtorError { e, marker: PhantomData }
}

/// Construct and return a Rust-movable value from a `Ctor`.
///
/// This is a safe, "by value" version of `Ctor::ctor`.
// TODO(jeanpierreda): Should this be a method on `Ctor`? Should it return `Result`?
pub fn construct<T: Unpin>(ctor: impl Ctor<Output = T, Error = Infallible>) -> T {
    let mut value = MaybeUninit::uninit();
    // SAFETY: `value` is valid for writes and uninitialized, and `T` is `Unpin`.
    unsafe {
        ctor.ctor(value.as_mut_ptr()).unwrap();
    }

    // SAFETY: `ctor.ctor()` initialized `value`.
    unsafe { value.assume_init() }
}

/// Trait for smart pointer types which support in-place initialization via `Ctor`.
///
/// A typical example would be `Box<T>`, allows emplacing a `Ctor` into
/// a `Pin<Box<T>>` by calling `{Box, Rc, Arc}::emplace`.
pub trait Emplace<T>: Sized {
    /// Materialize an infallible `Ctor`.
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

/// An auto trait for types that are values in their own right, rather than
/// dedicated constructors for some *other* type.
///
/// This trait is implemented for all types by default. Types like `FnCtor` that
/// exist only to construct other types should opt out via `impl !SelfCtor for ...`.
///
/// Do not use this trait as a trait bound. Instead, use `Ctor`. `SelfCtor` is a workaround to
/// implement specialization of `Ctor`, and will go away if we ever get a useful form of
/// specialization.
///
/// # Why is `SelfCtor` not implemented for my type?
///
/// `SelfCtor` is an auto trait, which means it is automatically implemented for a type if
/// it is implemented for all of its fields.
///
/// However, trait objects (`dyn Trait`) do not implement auto traits unless they are
/// explicitly listed in the trait bounds (e.g. `dyn Trait + SelfCtor`). Since `SelfCtor` is
/// a custom auto trait, it cannot be listed in trait bounds in general.
///
/// Consequently, any type containing a trait object (such as `Box<dyn Trait>`, `Rc<dyn Trait>`,
/// or a struct containing these) will also fail to implement `SelfCtor`.
///
/// If you encounter a compile error about `SelfCtor` not being implemented for a type `T`,
/// and you are trying to use a value of type `T` as a `Ctor` (for example, in a `ctor!` macro),
/// you can work around this by wrapping the value in `RustMoveCtor::new(value)`. This will
/// wrap the value in a constructor that constructs the type via a Rust move.
#[diagnostic::on_unimplemented(
    message = "`{Self}` cannot be used as a direct initializer in `ctor!`",
    label = "does not implement `SelfCtor` (often due to containing `dyn Trait`)",
    note = "Types containing trait objects (like `dyn Trait`) do not implement `SelfCtor` by default.",
    note = "To use this value in `ctor!`, wrap it in `RustMoveCtor::new(value)`."
)]
pub auto trait SelfCtor {}

#[must_use = must_use_ctor!()]
pub struct FnCtor<Output, F: FnOnce(*mut Output)>(F, PhantomData<fn(Output)>);
impl<Output, F: FnOnce(*mut Output)> FnCtor<Output, F> {
    /// Create a new `Ctor` whose `ctor()` method invokes `f`.
    ///
    /// # Safety
    ///
    /// If `f` does not panic, then it must initialize its argument to a valid value.
    ///
    /// `f` may rely on the same safety preconditions described by `Ctor::ctor`. In particular,
    /// the pointer argument is guaranteed to be valid for writes.
    pub unsafe fn new(f: F) -> Self {
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

/// !SelfCtor to override the blanket Ctor impl.
impl<Output, F> !SelfCtor for FnCtor<Output, F> {}

/// A `Ctor` for copying the pointee.
///
/// When used as a `Ctor` or `Assign` source, this initializes a `P::Target` by copying the
/// underlying data, analogous to `Clone`.
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
        unsafe { Output::ctor_new(&*self.0).ctor(dest) }
    }
}

/// !SelfCtor to override the blanket Ctor impl.
impl<P: ?Sized> !SelfCtor for Copy<P> {}

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
/// When used as a `Ctor` or `Assign` source, this initializes a `T` by a mutating move operation.
///
/// All rvalue references are implicitly pinned, to avoid an explosion in the number of reference
/// types.
///
/// Note: this does not actually move until it is used.
#[must_use = must_use_ctor_assign!("RvalueReference")]
#[repr(transparent)]
pub struct RvalueReference<'a, T: ?Sized>(pub Pin<&'a mut T>);

impl<T: ?Sized> RvalueReference<'_, T> {
    /// Returns a const rvalue reference to the underlying data.
    ///
    /// This is usually not useful, but some C++ APIs do have `const` rvalue references due to
    /// templated code.
    pub fn as_const(&self) -> ConstRvalueReference<'_, T> {
        ConstRvalueReference(&*self.0)
    }

    /// Returns an ordinary pinned mutable reference to the underlying data.
    pub fn as_mut(&mut self) -> Pin<&mut T> {
        self.0.as_mut()
    }

    /// Returns an ordinary reference to the underlying data.
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
        &self.0
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
        unsafe { T::ctor_new(self).ctor(dest) }
    }
}

/// Converts to an `RvalueReference`.
///
/// Do not use this trait directly, instead, cast to an `RvalueReference` using
/// the `mov!()` macro.
#[doc(hidden)]
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

/// !SelfCtor to override the blanket `Ctor` impl.
impl<'a, T: ?Sized> !SelfCtor for RvalueReference<'a, T> {}

/// Represents the type `T` by value, but is passed by rvalue reference.
///
/// This is currently an implementation detail of the `impl Ctor` bindings, but
/// it can also be used in trait implementations if `impl Ctor` is not available,
/// due to "overlapping" impls that Rust doesn't recognize as disjoint due to different
/// associated types. (Rust does not recognize `Ctor![i32]` and `Ctor![i64]` as disjoint,
/// even though no type can implement both.)
#[repr(transparent)]
pub struct ByValue<'a, T: ?Sized>(pub RvalueReference<'a, T>);

impl<T: ?Sized> Deref for ByValue<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// SAFETY: forwards to `RvalueReference`'s `Ctor` impl, meets all requirements.
unsafe impl<'a, T: ?Sized> Ctor for ByValue<'a, T>
where
    T: CtorNew<RvalueReference<'a, T>>,
{
    type Output = T;
    type Error = <T as CtorNew<RvalueReference<'a, T>>>::Error;

    unsafe fn ctor(self, dest: *mut T) -> Result<(), Self::Error> {
        // SAFETY: forwards to `RvalueReference`'s `Ctor` impl, meets all requirements.
        unsafe { T::ctor_new(self.0).ctor(dest) }
    }
}

/// !SelfCtor to override the blanket `Ctor` impl.
impl<'a, T: ?Sized> !SelfCtor for ByValue<'a, T> {}

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
        unsafe { T::ctor_new(self).ctor(dest) }
    }
}

/// !SelfCtor to override the blanket `Ctor` impl.
impl<'a, T: ?Sized> !SelfCtor for ConstRvalueReference<'a, T> {}

/// Creates an `RvalueReference` from `p`, indicating that it is to be moved.
///
/// This is analogous to C++ `std::move`, except that this can directly create an
/// `RvalueReference<T>` out of e.g. a `Pin<Box<T>>`. The resulting `RvalueReference` has the
/// lifetime of a temporary, after which the parameter is destroyed.
///
/// The resulting `RvalueReference` can be used as a `CtorNew` or `Assign`
/// source, or as a `Ctor` directly.
///
/// Note: this does not actually move the parameter until it is used.
#[macro_export]
macro_rules! mov {
    ($p:expr) => {
        $crate::DerefRvalueReference::deref_rvalue_reference(&mut ($p))
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

/// All Rust types are their own constructor (except those that implement `Ctor`
/// for another type, see `SelfCtor` above.)
///
/// This allows code to safely accept direct initialization of Rust-movable
/// values, while also accepting customized in-place pinned initialization for
/// non-Rust-movable values: use `Ctor`-based initialization for both.
//
// SAFETY: unconditionally initializes dest. Note that while it is pinned after construction,
// it is not pinned beforehand, as we accept it by value. (If it was meant to be pinned, the UB
// already happened as part of passing self by value, and nothing we do here adds additional UB.)
unsafe impl<T: SelfCtor> Ctor for T {
    type Output = T;
    type Error = Infallible;
    unsafe fn ctor(self, dest: *mut Self) -> Result<(), Infallible> {
        // SAFETY: dest is valid for writes and uninitialized.
        unsafe {
            dest.write(self);
        }
        Ok(())
    }
}

/// Constructs via a Rust move.
#[must_use = must_use_ctor!()]
pub struct RustMoveCtor<T, E = Infallible>(T, PhantomData<fn() -> E>);
impl<T, E> !SelfCtor for RustMoveCtor<T, E> {}

impl<T, E> RustMoveCtor<T, E> {
    pub fn new(x: T) -> Self {
        RustMoveCtor(x, PhantomData)
    }
}

// SAFETY: unconditionally initializes dest.
unsafe impl<T, E> Ctor for RustMoveCtor<T, E> {
    type Output = T;
    type Error = E;
    unsafe fn ctor(self, dest: *mut T) -> Result<(), E> {
        // SAFETY: dest is valid for writes and uninitialized.
        unsafe {
            dest.write(self.0);
        }
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
        UnreachableCtor(PhantomData)
    }
}
impl<T: ?Sized, E> !SelfCtor for UnreachableCtor<T, E> {}

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

/// All Rust types that implement `Default` are C++-default-constructible.
impl<T: Default> CtorNew<()> for T {
    type CtorType = RustMoveCtor<Self>;
    type Error = Infallible;
    fn ctor_new(_: ()) -> Self::CtorType {
        RustMoveCtor::new(Default::default())
    }
}

/// All Rust types are C++-copyable if they are Rust-cloneable.
impl<T: Clone> CtorNew<&T> for T {
    type CtorType = RustMoveCtor<Self>;
    type Error = Infallible;
    fn ctor_new(src: &Self) -> Self::CtorType {
        RustMoveCtor::new(src.clone())
    }
}

// ========
// emplace!
// ========

/// In-place construct a `Ctor`.
///
/// This is analoogus to the `pin!()` macro, except that it runs user-defined construction logic
/// via the `Ctor` trait.
///
/// Usage:
///
/// ```
/// /// If some_ctor is a `Ctor<Output=T, Error=Infallible>`, then:
/// let x: Pin<&mut T> = emplace!(some_ctor);
/// ```
///
/// `emplace!` only works with infallible `Ctor`s. See `try_emplace` for
/// initializing `Ctor` objects that can return an error.
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

/// In-place construct a fallible `Ctor`.
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
    pub use crate::Infallible;
    pub use core::mem::MaybeUninit;
    pub use core::pin::Pin;

    /// A helpfully named type alias that tells users to not directly initialize fields of this type
    pub type MustUseCtorToInitialize = [u8; 0];

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
        unsafe {
            Ctor::ctor(ctor, field).unwrap();
        }
        UnsafeDropGuard(field)
    }

    pub fn require_recursively_pinned<_T: RecursivelyPinned>() {}

    /// Function which requires `unsafe` and is called by `raw_ctor!()`.
    ///
    /// This gives a pleasing error message, as they have the same name.
    pub unsafe fn raw_ctor() {}

    /// Create an `FnOnce`. This works around Rust trying to infer that a closure is
    /// `Fn`.
    pub fn make_fn_once<Output, F: FnOnce(*mut Output)>(f: F) -> impl FnOnce(*mut Output) {
        f
    }
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
    type CtorInitializedFields: ?Sized;

    /// The type returned by `project_pin` containing pinned references to the
    /// fields of `Self`.
    type ProjectedPin<'a>
    where
        Self: 'a;

    /// The type returned by `project_ref` containing pinned references to the
    /// fields of `Self`.
    type ProjectedRef<'a>
    where
        Self: 'a;
}

/// The drop trait for `#[recursively_pinned(PinnedDrop)]` types.
///
/// It is not safe to implement `Drop` for a recursively-pinned type, as this
/// would be unsound: the `&mut self` in `drop` would allow the pin guarantee to
/// be violated.
///
/// Instead, to implement `Drop`, users of `#[recursively_pinned]` must pass `PinnedDrop` to
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

/// Evaluates to a `Ctor` which initializes a struct in-place, field-by-field.
///
/// The `ctor!` macro evaluates to a `Ctor` for a Rust struct. Each field has its value provided
/// using a `Ctor`, so that each is initialized in-place.
///
/// Example use:
///
/// ```
/// fn new() -> impl Ctor<Output=MyStruct> {
///   ctor!(MyStruct {field_1: MyType::ctor_new(()), field_2: 42})
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
    // Struct {} ctor.
    ( $t:ident $(:: $ts:ident)* $(:: < $($gp:tt),+ >)? {$($body:tt)*} ) => {
        {
            use $t $(:: $ts)* as Type;

            $crate::unsafe_ctor_impl!(
                #[fields =
                    <Type $(< $( $gp ),+ >)? as $crate::RecursivelyPinned>::CtorInitializedFields]
                #[unsafe_assert = $crate::macro_internal::require_recursively_pinned::<Type $(< $( $gp ),+ >)?>();]
                Type $(:: < $($gp),+ >)? {$($body)*}
            )
        }
    };

    // Unit struct ctor.
    ($t:ident $(:: $ts:ident)* $(:: < $($gp:tt),+ >)? ) => {$crate::ctor!($t $(:: $ts)* $(:: < $($gp),+ >)? {  })};

    // Conventional tuple struct syntax (with parens, no integer names) supported for < 8 fields.
    // Otherwise, use MyTupleStruct{0: ..., 1: ...} syntax, which works for any number of fields.
    // Generated as so:
    /* python3 -c 'for i in range(8):
        ctor_ins = ", ".join(f"$ctor_{j}:expr" for j in range(i))
        ctor_outs = ", ".join(f"{j}: $ctor_{j}" for j in range(i))
        print(f"    ($t:ident $(:: $ts:ident)* $(:: < $($gp:tt),+ >)? ({ctor_ins})) => {{$crate::ctor!($t $(:: $ts)* $(:: < $($gp),+ >)? {{ {ctor_outs} }})}};")' */
    ($t:ident $(:: $ts:ident)* $(:: < $($gp:tt),+ >)? ()) => {$crate::ctor!($t $(:: $ts)* $(:: < $($gp),+ >)? {  })};
    ($t:ident $(:: $ts:ident)* $(:: < $($gp:tt),+ >)? ($ctor_0:expr)) => {$crate::ctor!($t $(:: $ts)* $(:: < $($gp),+ >)? { 0: $ctor_0 })};
    ($t:ident $(:: $ts:ident)* $(:: < $($gp:tt),+ >)? ($ctor_0:expr, $ctor_1:expr)) => {$crate::ctor!($t $(:: $ts)* $(:: < $($gp),+ >)? { 0: $ctor_0, 1: $ctor_1 })};
    ($t:ident $(:: $ts:ident)* $(:: < $($gp:tt),+ >)? ($ctor_0:expr, $ctor_1:expr, $ctor_2:expr)) => {$crate::ctor!($t $(:: $ts)* $(:: < $($gp),+ >)? { 0: $ctor_0, 1: $ctor_1, 2: $ctor_2 })};
    ($t:ident $(:: $ts:ident)* $(:: < $($gp:tt),+ >)? ($ctor_0:expr, $ctor_1:expr, $ctor_2:expr, $ctor_3:expr)) => {$crate::ctor!($t $(:: $ts)* $(:: < $($gp),+ >)? { 0: $ctor_0, 1: $ctor_1, 2: $ctor_2, 3: $ctor_3 })};
    ($t:ident $(:: $ts:ident)* $(:: < $($gp:tt),+ >)? ($ctor_0:expr, $ctor_1:expr, $ctor_2:expr, $ctor_3:expr, $ctor_4:expr)) => {$crate::ctor!($t $(:: $ts)* $(:: < $($gp),+ >)? { 0: $ctor_0, 1: $ctor_1, 2: $ctor_2, 3: $ctor_3, 4: $ctor_4 })};
    ($t:ident $(:: $ts:ident)* $(:: < $($gp:tt),+ >)? ($ctor_0:expr, $ctor_1:expr, $ctor_2:expr, $ctor_3:expr, $ctor_4:expr, $ctor_5:expr)) => {$crate::ctor!($t $(:: $ts)* $(:: < $($gp),+ >)? { 0: $ctor_0, 1: $ctor_1, 2: $ctor_2, 3: $ctor_3, 4: $ctor_4, 5: $ctor_5 })};
    ($t:ident $(:: $ts:ident)* $(:: < $($gp:tt),+ >)? ($ctor_0:expr, $ctor_1:expr, $ctor_2:expr, $ctor_3:expr, $ctor_4:expr, $ctor_5:expr, $ctor_6:expr)) => {$crate::ctor!($t $(:: $ts)* $(:: < $($gp),+ >)? { 0: $ctor_0, 1: $ctor_1, 2: $ctor_2, 3: $ctor_3, 4: $ctor_4, 5: $ctor_5, 6: $ctor_6 })};
}

/// Unsafe in-place construction of a struct, field by field.
///
/// The `raw_ctor!` macro evaluates to a `Ctor` for a Rust struct, with
/// user-specified values for the fields, each also initialized in-place.
///
/// This is identical in use to `ctor!`, but it does not enforce that the
/// struct is `RecursivelyPinned`.
///
/// The caller must ensure that the struct meets the safety criteria of
/// `RecursivelyPinned`.
///
/// If the struct would have used a different `CtorInitializedFields` type,
/// this can be specified using `#[fields = ...]`, as so:
///
/// ```
/// raw_ctor!(#[fields = Mystruct2] MyStruct { field: 42 })
/// ```
///
/// NOTE: `raw_ctor` only supports `struct {}` syntax.
#[macro_export]
macro_rules! raw_ctor {
    ( $t:ident $(:: $ts:ident)* $(:: < $($gp:tt),+ >)? {$($body:tt)*} ) => {
        {
            use $t $(:: $ts)* as Type;

            $crate::unsafe_ctor_impl!(
                #[fields = Type $(< $( $gp ),+ >)?]
                #[unsafe_assert = $crate::macro_internal::raw_ctor();]
                Type $(:: < $($gp),+ >)? {$($body)*}
            )
        }
    };
    ( #[fields = $($fields_ty:tt)*] $t:ident $(:: $ts:ident)* $(:: < $($gp:tt),+ >)? {$($body:tt)*} ) => {
        {
            use $t $(:: $ts)* as Type;

            $crate::unsafe_ctor_impl!(
                #[fields = $($fields_ty)*]
                #[unsafe_assert = $crate::macro_internal::raw_ctor();]
                Type $(:: < $($gp),+ >)? {$($body)*}
            )
        }
    };
}

/// Implementation of the ctor! macro.
///
/// # Safety
///
/// * fields_ty must have every field that `$Type` does.
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
#[doc(hidden)]
#[macro_export]
macro_rules! unsafe_ctor_impl {
    (   #[fields = $($fields_ty:tt)*]
        #[unsafe_assert = $($unsafe_assert:tt)*]
        $Type:ident $(:: < $($gp:tt),+ >)? {$( $name:tt: $sub_ctor:expr ),* $(,)?}
    ) => {{
        // We need to capture all the sub_ctor values in advance (in case they are Copy and thus
        // borrowed by reference).
        //
        // Note that we can't just do `move ||` without pre-binding them, in case
        // they want to borrow something. Consider: `ctor!(Foo(f(&x))`. A move closure
        // will capture `x` by value, but in fact we only want to capture `&x` by value.
        //
        // Ordinarily, we'd do something like `$(let $name = $sub_ctor;)*`, but `$name`
        // can be an integer for tuple-structs. So, instead, we build everything up into a tuple
        // that we can progressively unpack.
        let capture = $crate::internal_hlist!($($sub_ctor,)*);
        let _ = &capture;  // silence unused_variables warning if Type is fieldless.
        let init = $crate::macro_internal::make_fn_once(move |x: *mut $Type $(< $( $gp ),+ >)?| {
            struct DropGuard;
            let drop_guard = DropGuard;
            let _ = &x; // silence unused_variables warning if Type is fieldless.

            // Enforce that the ctor!()/etc. expression resembles a valid direct initialization
            // expression, by using the names in a conventional literal.
            // For structs, this fails to compile unless the names are fully exhaustive.
            // For unions, it fails to compile unless precisely one name is used.
            // In both cases, this ensures that we only compile when expressions corresponding
            // to normal init are used, with unsurprising semantics.
            let _ = |x: $Type $(< $( $gp ),+ >)?| {
                let _ = &x; // silence unused_variables warning if Type is fieldless.
                // If this fails to compile, not every field was specified in the ctor! invocation.
                // The `panic!(...)` allows us to avoid moving out of x, while still pretending to
                // fill in each field.
                #[allow(unreachable_code, unused_unsafe)] $crate::macro_internal::Identity::<
                    $($fields_ty)*> {
                        // SAFETY: this code is not executed.
                        // The unsafe {} block is in case this is a *union* literal, rather than
                        // a struct literal.
                        $($name: panic!("{}", unsafe {&x.$name} as *const _ as usize),)*
                    };
            };

            $($unsafe_assert)*

            $(
                let (sub_ctor, capture) = capture;
                let field_drop = unsafe {
                    // SAFETY: the place is in bounds, just uninitialized. See e.g. second
                    // example: https://doc.rust-lang.org/nightly/std/ptr/macro.addr_of_mut.html
                    $crate::macro_internal::init_field(
                        &raw mut (*x).$name,
                        sub_ctor)
                };
                let drop_guard = (drop_guard, field_drop);
            )*
            #[allow(clippy::forget_non_drop)] // if fieldless
            ::core::mem::forget(drop_guard);
        });
        // SAFETY: In the non-panicking case, we initialize each field using `Ctor::ctor`, which
        // guarantees initialization.
        //
        // We guarantee that no fields are missed by pretending to construct using ordinary Rust
        // struct syntax in a never-executed code path. If the user failed to
        // mention initialization of a field, then this will fail with a compilation error.
        // The type used is specified by $fields_ty, which must be correct for this to be
        // safe to call. (In practice, it's either unsafely passed by the user, or
        // automatically generated to match by `#[recursively_pinned]`).
        unsafe {$crate::FnCtor::new(init)}
    }};
}

/// Creates a linked list. `internal_hlist!(a, b, c)` -> `(a, (b, (c, ()))`
#[doc(hidden)]
#[macro_export]
macro_rules! internal_hlist {
    ($(,)?) => {
        ()
    };
    ($a:expr, $($rest:tt)*) => {
        ($a, $crate::internal_hlist!($($rest)*))
    };
}

// ==========
// Assignment
// ==========

/// Destroy-then-reconstruct. Sidesteps `operator=`, instead reconstructing
/// in-place.
///
/// For Rust-movable types, this is equivalent to `*p.as_mut().into_inner() = construct(ctor)`.
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
/// directly call `reconstruct(x, foo)` -- provided you are OK with the
/// differing constructor/destructor ordering, and satisfy safety criteria.
///
/// # Safety
///
/// The behavior is only undefined if `p` refers to C++ object storage, and is a
/// base class subobject or `[[no_unique_address]]` member.
/// See: http://eel.is/c++draft/basic.life#8.
///
/// If `p` refers to an object stored anywhere else, such as a (non-`no_unique_address`)
/// field of a struct, or a local variable, then this is safe. In particular, if the caller is
/// pure Rust code, and uses ordinary Rust storage, then this is safe.
///
/// (Note that since this calls `drop()` on the pinned pointer, it satisfies the pin
/// guarantee, and is allowed to then re-init it with something else. In effect, this
/// is just the in-place Ctor version of the existing method `Pin<T>::set(T)`.)
pub unsafe fn reconstruct<T>(p: Pin<&mut T>, ctor: impl Ctor<Output = T, Error = Infallible>) {
    let raw_ptr = unsafe { Pin::into_inner_unchecked(p) } as *mut _;
    unsafe {
        core::ptr::drop_in_place(raw_ptr);
    }
    abort_on_unwind(move || {
        unsafe { ctor.ctor(raw_ptr) }.unwrap();
    });
}

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
        *self = construct(T::ctor_new(src));
    }
}

impl<'a, T> Assign<RvalueReference<'a, T>> for T
where
    T: Unpin + CtorNew<RvalueReference<'a, T>, Error = Infallible>,
{
    fn assign(mut self: Pin<&mut Self>, src: RvalueReference<'a, Self>) {
        *self = construct(T::ctor_new(src));
    }
}

impl<'a, T> Assign<ConstRvalueReference<'a, T>> for T
where
    T: Unpin + CtorNew<ConstRvalueReference<'a, T>, Error = Infallible>,
{
    fn assign(mut self: Pin<&mut Self>, src: ConstRvalueReference<'a, Self>) {
        *self = construct(T::ctor_new(src));
    }
}

/// Overloaded assignment operator, but for Unpin types
///  TODO(b/219963671): use specialization instead of a distinct trait
pub trait UnpinAssign<From> {
    fn unpin_assign(&mut self, src: From);
}

/// A conversion trait that is considered unsafe.
///
/// This is used for `Unpin` types when the conversion involves unsafe operations
/// or types.
pub trait UnsafeFrom<From> {
    /// Performs the conversion.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the conversion is safe.
    unsafe fn unsafe_from(src: From) -> Self;
}

// =======================
// Constructor overloading
// =======================

/// Overloadable constructor trait.
///
/// This is analogous to `From<A>`, but it returns a `Ctor` instead of `Self` by value.
pub trait CtorNew<ConstructorArgs> {
    type CtorType: Ctor<Output = Self, Error = Self::Error>;
    type Error;

    /// Returns the `Ctor` for a new instance.
    fn ctor_new(args: ConstructorArgs) -> Self::CtorType;
}

/// Overloaded constructor trait for constructors that are considered unsafe.
///
/// This is used when the constructor accepts arguments that are unsafe to use,
/// or when the constructor itself is marked unsafe in C++.
///
/// The safety conditions are documented on the `UnsafeCtorNew` implementation.
pub trait UnsafeCtorNew<ConstructorArgs> {
    type CtorType: Ctor<Output = Self, Error = Self::Error>;
    type Error;

    /// Creates a new constructor.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the arguments are safe to use for construction, as
    /// documented by the implementation.
    unsafe fn ctor_new(args: ConstructorArgs) -> Self::CtorType;
}

// ====
// Misc
// ====

/// A constructor for ManuallyDrop<T>, given a constructor for T.
///
/// ManuallyDrop is special as the only non-Copy type allowed in a union, so we
/// specifically support its use, even though it is not guaranteed to be
/// structurally pinned.
#[must_use = must_use_ctor!()]
pub struct ManuallyDropCtor<T: Ctor>(T);

impl<T: Ctor> ManuallyDropCtor<T> {
    /// # Safety
    ///
    /// This structurally pins the contents of ManuallyDrop.
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
        unsafe { self.0.ctor(dest as *mut _) }
    }
}

impl<T: Ctor> !SelfCtor for ManuallyDropCtor<T> {}
