// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#![feature(negative_impls)]
//! Traits for memory management operations on wrapped C++ objects, based on
//! moveit.
//!
//! # Comparison with moveit
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
//!  emplace!{ let mut tmp = mov(x.as_mut()); }
//!  x.assign(mov(y.as_mut()));
//!  y.assign(mov(tmp));
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
//! More significantly, we take the position that all Unpin types are their own
//! `Ctor`. This makes `Ctor`-based initialization nearly a perfect superset of
//! normal initialization rules: for a non-self-referential Rust type (an Unpin
//! type), it looks the same as normal, but for C++-like !Unpin types we use
//! custom `Ctor` values to initialize in-place.
//!
//! Blanket implementing based on `Unpin` means that we're treating `T : Unpin`
//! as effectively meaning "T is safe to deal with by value", which is close to,
//! but not quite, what it means. (There are some types which are !Unpin but
//! would be safe to treat as normal rust types. Unfortunately, there is no way
//! to detect them.)
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

use std::marker::PhantomData;
use std::mem::{ManuallyDrop, MaybeUninit};
use std::ops::{Deref, DerefMut};
use std::pin::Pin;

pub use ctor_proc_macros::*;

/// The string constant for #[must_use] to describe why you must use a `Ctor`.
macro_rules! must_use_ctor {
    () => {
        "A Ctor is not invoked unless emplaced, using e.g. `emplace!{}`, or `Box::emplace()`."
    };
}

/// The string constant for #[must_use] to describe why you must use a
/// `Ctor`/`Assign` source.
macro_rules! must_use_ctor_assign {
    ($name:literal) => {
        concat!(
            $name,
            " is not invoked unless emplaced, using e.g. `emplace!{}`, or `Box::emplace()`, or",
            " unless assigned from, using `.assign()`."
        )
    };
}

// ======================
// ~Unchanged from moveit
// ======================

#[must_use = must_use_ctor!()]
pub trait Ctor {
    type Output;
    unsafe fn ctor(self, dest: Pin<&mut MaybeUninit<Self::Output>>);

    /// Returns a chained Ctor, which will invoke `f` after construction.
    ///
    /// for example, these two snippets are equivalent:
    ///
    /// ```
    /// emplace! { let x = y; }
    /// x.mutating_method();
    /// ```
    ///
    /// ```
    /// emplace! { let x = y.ctor_then(|mut inited| inited.mutating_method()); }
    /// ```
    fn ctor_then<F: FnOnce(Pin<&mut Self::Output>)>(self, f: F) -> CtorThen<Self, F>
    where
        Self: Sized,
    {
        CtorThen { ctor: self, f }
    }
}

pub trait Emplace<T>: Sized {
    fn emplace<C: Ctor<Output = T>>(c: C) -> Pin<Self>;
}

impl<T> Emplace<T> for Box<T> {
    fn emplace<C: Ctor<Output = T>>(ctor: C) -> Pin<Box<T>> {
        let mut uninit = Box::new(MaybeUninit::<T>::uninit());
        unsafe {
            let pinned = Pin::new_unchecked(&mut *uninit);
            ctor.ctor(pinned);
            Pin::new_unchecked(Box::from_raw(Box::into_raw(uninit).cast::<T>()))
        }
    }
}

#[must_use = must_use_ctor!()]
pub struct FnCtor<Output, F: FnOnce(Pin<&mut MaybeUninit<Output>>)>(pub F, PhantomData<fn(Output)>);
impl<Output, F: FnOnce(Pin<&mut MaybeUninit<Output>>)> FnCtor<Output, F> {
    pub fn new(f: F) -> Self {
        Self(f, PhantomData)
    }
}

impl<Output, F: FnOnce(Pin<&mut MaybeUninit<Output>>)> Ctor for FnCtor<Output, F> {
    type Output = Output;

    unsafe fn ctor(self, dest: Pin<&mut MaybeUninit<Output>>) {
        self.0(dest);
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
pub struct Copy<P: Deref>(P);

impl<Output: for<'a> CtorNew<&'a Output>, P: Deref<Target = Output>> Ctor for Copy<P> {
    type Output = Output;

    unsafe fn ctor(self, dest: Pin<&mut MaybeUninit<Output>>) {
        Output::ctor_new(&*self.0).ctor(dest);
    }
}

/// !Unpin to override the blanket Ctor impl.
impl<P> !Unpin for Copy<P> {}

/// Returns a `Copy` which can be used as a `CtorNew` or `Assign` source, or as
/// a `Ctor` directly.
///
/// Note: this does not actually copy the parameter until it is used.
pub fn copy<T: for<'a> CtorNew<&'a T>, P: Deref<Target = T>>(src: P) -> Copy<P> {
    Copy(src)
}

// ================================
// DerefMut based move construction
// ================================

#[repr(transparent)]
pub struct RvalueReference<'a, T>(Pin<&'a mut T>);

impl<T> RvalueReference<'_, T> {
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
        // whose lifetime is bound by  self, not 'a.)
        &*self.0
    }
}

/// Move type.
///
/// This creates a new `P::Target` by moving -- either move-construction
/// (construction from `RvalueReference(&*P)`), or move-assignment (assignment
/// from `RvalueReference(&*P)`).
///
/// Note: this does not actually move `P` until it is used.
#[must_use = must_use_ctor_assign!("Move")]
pub struct Move<P: DerefMut>(Pin<P>);

impl<Output: for<'a> CtorNew<RvalueReference<'a, Output>>, P: DerefMut<Target = Output>> Ctor
    for Move<P>
{
    type Output = Output;

    unsafe fn ctor(mut self, dest: Pin<&mut MaybeUninit<Output>>) {
        Output::ctor_new(RvalueReference(self.0.as_mut())).ctor(dest);
    }
}

/// !Unpin to override the blanket Ctor impl.
impl<P> !Unpin for Move<P> {}

#[repr(transparent)]
pub struct ConstRvalueReference<'a, T>(&'a T);

impl<'a, T> ConstRvalueReference<'a, T> {
    pub fn get_ref(&mut self) -> &'a T {
        self.0
    }
}

/// Const-move type. Usually not very helpful.
///
/// This implicitly converts to `P::Target` by const-moving -- either
/// const-move-construction (construction from `ConstRvalueReference(&*P)`), or
/// const-move-assignment (assignment from `ConstRvalueReference(&*P)`).
#[must_use = must_use_ctor_assign!("ConstMove")]
pub struct ConstMove<P: Deref>(P);

impl<Output: for<'a> CtorNew<ConstRvalueReference<'a, Output>>, P: Deref<Target = Output>> Ctor
    for ConstMove<P>
{
    type Output = Output;

    unsafe fn ctor(self, dest: Pin<&mut MaybeUninit<Output>>) {
        Output::ctor_new(ConstRvalueReference(&*self.0)).ctor(dest);
    }
}

/// !Unpin to override the blanket Ctor impl.
impl<P> !Unpin for ConstMove<P> {}

/// Creates a "to-be-moved" pointer for `src`.
///
/// In other words, this is analogous to C++ `std::move`, except that the
/// pointer can be owned.
///
/// The resulting `Move` can be used as a `CtorNew` or `Assign` source, or as a
/// `Ctor` directly.
///
/// Note: this does not actually move the parameter until it is used.
pub fn mov<P: DerefMut>(src: Pin<P>) -> Move<P> {
    Move(src)
}

pub fn const_mov<P: Deref>(src: P) -> ConstMove<P> {
    ConstMove(src)
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
impl<T: Unpin> Ctor for T {
    type Output = T;
    unsafe fn ctor(self, mut dest: Pin<&mut MaybeUninit<Self>>) {
        dest.as_mut_ptr().write(self);
    }
}

/// Constructs via a Rust move.
#[must_use = must_use_ctor!()]
pub struct RustMoveCtor<T>(T);
impl<T> !Unpin for RustMoveCtor<T> {}
impl<T> Ctor for RustMoveCtor<T> {
    type Output = T;
    unsafe fn ctor(self, dest: Pin<&mut MaybeUninit<T>>) {
        Pin::into_inner_unchecked(dest).as_mut_ptr().write(self.0);
    }
}

/// All Rust types are C++-default-constructible if safe (i.e. Unpin + Default).
impl<T: Unpin + Default + Sized> CtorNew<()> for T {
    type CtorType = RustMoveCtor<Self>;
    fn ctor_new(_: ()) -> Self::CtorType {
        RustMoveCtor(Default::default())
    }
}

/// All Unpin Rust types are C++-copyable if they are Rust-cloneable.
///
/// (Unpin is required for safety; otherwise, this would violate the Pin
/// guarantee.)
impl<T: Unpin + Clone> CtorNew<&T> for T {
    type CtorType = RustMoveCtor<Self>;
    fn ctor_new(src: &Self) -> Self::CtorType {
        RustMoveCtor(src.clone())
    }
}

// ===========
// ctor_then()
// ===========

/// A `Ctor` which constructs using `self.ctor`, and then invokes `f` on the
/// resulting object.
///
/// This struct is created by the `ctor_then` method on `Ctor`. See its
/// documentation for more.
#[must_use = must_use_ctor!()]
pub struct CtorThen<C: Ctor, F: FnOnce(Pin<&mut C::Output>)> {
    ctor: C,
    f: F,
}

impl<C: Ctor, F: FnOnce(Pin<&mut C::Output>)> Ctor for CtorThen<C, F> {
    type Output = C::Output;
    unsafe fn ctor(self, mut dest: Pin<&mut MaybeUninit<Self::Output>>) {
        self.ctor.ctor(dest.as_mut());
        let dest = Pin::new_unchecked(Pin::into_inner_unchecked(dest).assume_init_mut());
        (self.f)(dest)
    }
}

impl<C: Ctor, F: FnOnce(Pin<&mut C::Output>)> !Unpin for CtorThen<C, F> {}

// ========
// emplace!
// ========
//
// The emplace!{} macro is now a little simpler, as it doesn't require StackBox
// in the public interface: it can use &mut.

/// Emplace a constructor into a local or temporary.
/// Syntax: `emplace! { let mut varname = expr() }`, where `expr()` evaluates to
/// a `Ctor<Output=T>`. `varname` will be a `Pin<&mut T>`.
///
/// If the emplaced value will just be used for the duration of one statement,
/// the `emplace!(ctor)` syntax can be used instead, to emplace the ctor into a
/// temporary. e.g. `foo(emplace!(ctor))` to pass a pinned reference to the
/// emplaced value to a function.
#[macro_export]
macro_rules! emplace {
    ($expr:expr) => {
        $crate::Slot::unsafe_new().unsafe_construct($expr).unsafe_as_pin_unchecked()
    };
    (@emplace_one let [$($mut_:tt)?] $var:ident [$($type_:tt)*]= $expr:expr;) => {
        let mut $var = $crate::Slot::unsafe_new();
        $var.unsafe_construct($expr);
        let $($mut_)* $var $($type_)* = $var.unsafe_as_pin_unchecked();
    };
    // Base case for repeated lets: empty emplace.
    () => {};
    // Recursive case: let [mut] x [: T] = ...;
    // There are four different combinations of mutability and explicit type parameter, we have to
    // match all of them.
    (let mut $var:ident : $t:ty = $expr:expr; $($remaining_lets:tt)*) => {
        $crate::emplace! {@emplace_one let [mut] $var [:$t] = $expr;}
        $crate::emplace! {$($remaining_lets)*};
    };
    (let mut $var:ident = $expr:expr; $($remaining_lets:tt)*) => {
        $crate::emplace! {@emplace_one let [mut] $var []= $expr;}
        $crate::emplace! {$($remaining_lets)*};
    };
    (let $var:ident : $t:ty  = $expr:expr; $($remaining_lets:tt)*) => {
        $crate::emplace! {@emplace_one let [] $var [:$t] = $expr;}
        $crate::emplace! {$($remaining_lets)*};
    };
    (let $var:ident = $expr:expr; $($remaining_lets:tt)*) => {
        $crate::emplace! {@emplace_one let [] $var [] = $expr;}
        $crate::emplace! {$($remaining_lets)*};
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
/// emplace! {let slot = Slot::uninit(); }
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
/// emplace! {let mut slot = Slot::uninit(); }
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
    pub fn uninit() -> impl Ctor<Output = Self> {
        RustMoveCtor(Self::unsafe_new())
    }

    pub fn new(value: impl Ctor<Output = T>) -> impl Ctor<Output = Self> {
        RustMoveCtor(Self::unsafe_new()).ctor_then(|slot| {
            slot.replace(value);
        })
    }

    pub fn clear(self: Pin<&mut Self>) {
        if self.is_initialized {
            let Self { is_initialized, maybe_uninit } = unsafe { Pin::into_inner_unchecked(self) };
            unsafe {
                std::ptr::drop_in_place(maybe_uninit.as_mut_ptr());
            }
            *is_initialized = false;
        }
    }

    pub fn replace(mut self: Pin<&mut Self>, value: impl Ctor<Output = T>) -> Pin<&mut T> {
        self.as_mut().clear();
        {
            let Self { is_initialized, maybe_uninit } =
                unsafe { Pin::into_inner_unchecked(self.as_mut()) };
            unsafe {
                value.ctor(Pin::new_unchecked(maybe_uninit));
            }
            *is_initialized = true;
        }
        self.as_opt_mut().unwrap()
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
    pub fn unsafe_construct(&mut self, ctor: impl Ctor<Output = T>) -> &mut Self {
        unsafe { ctor.ctor(Pin::new_unchecked(&mut self.maybe_uninit)) };
        self.is_initialized = true;
        self
    }

    /// Safety: pin guarantee, assumes init.
    pub fn unsafe_as_pin_unchecked(&mut self) -> Pin<&mut T> {
        unsafe { Pin::new_unchecked(self.maybe_uninit.assume_init_mut()) }
    }
}

#[doc(hidden)]
pub mod macro_internal {
    use super::*;
    pub use std::mem::MaybeUninit;
    pub use std::pin::Pin;

    /// Drops a pointer when dropped.
    ///
    /// Safety: this will drop the pointer, so this is only safe if no other
    /// code will access the pointer afterwards. This includes drop: the
    /// pointer must itself be in a ManuallyDrop.
    pub struct UnsafeDropGuard<T>(*mut T);
    impl<T> Drop for UnsafeDropGuard<T> {
        fn drop(&mut self) {
            unsafe { std::ptr::drop_in_place(self.0) };
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
    pub unsafe fn init_field<T>(field: *mut T, ctor: impl Ctor<Output = T>) -> impl Drop {
        // safety: MaybeUninit<T> is the same layout as T, the caller guarantees it's
        // pinned.
        let maybe_uninit = field as *mut MaybeUninit<T>;
        let pinned = Pin::new_unchecked(&mut *maybe_uninit);
        Ctor::ctor(ctor, pinned);
        UnsafeDropGuard(field)
    }

    pub fn require_recursively_pinned<_T: RecursivelyPinned>() {}
}

// =====
// ctor!
// =====

/// The RecursivelyPinned trait asserts that when the struct is pinned, every
/// field is also pinned.
///
/// Safety: Only use if you never directly access fields of a pinned object. For
/// example, with the pin-project crate, all fields should be marked `#[pin]`.
///
/// This trait is automatically implemented for any `#[recursively_pinned]`
/// struct.
pub unsafe trait RecursivelyPinned {}

/// The `ctor!` macro evaluates to a `Ctor` for a Rust struct, with
/// user-specified fields.
///
/// (This was inspired by, but takes no code from, https://crates.io/crates/project-uninit.)
///
/// Example use:
///
/// ```
/// fn new() -> impl Ctor<Output=MyStruct> {
///   ctor!(MyStruct {field_1: default_ctor(), field_2: 42})
/// }
///
/// // Actually invoke the Ctor to create a new MyStruct:
/// emplace! { let mut my_struct = MyStruct::new(); }
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
    ( $t:ident $(:: $ts:ident)* {$( $name:tt: $sub_ctor:expr ),* $(,)?} ) => {
        {
            use $t $(:: $ts)* as Type;
            $crate::FnCtor::new(|x: $crate::macro_internal::Pin<&mut $crate::macro_internal::MaybeUninit<Type>>| {
                use ::std::ptr::addr_of_mut;
                let drop_guard = ();
                let x_mut = unsafe{$crate::macro_internal::Pin::into_inner_unchecked(x)}.as_mut_ptr();

                // Enforce that the ctor!() expression resembles a valid direct initialization
                // expression, by using the names in a conventional literal.
                // For structs, this fails to compile unless the names are fully exhaustive.
                // For unions, it fails to compile unless precisely one name is used.
                // In both cases, this ensures that we only compile when expressions corresponding
                // to normal init are used, with unsurprising semantics.
                let _ = |x: Type| {
                    // If this fails to compile, not every field was specified in the ctor! invocation.
                    // The `panic!(...)` allows us to avoid moving out of x, while still pretending to
                    // fill in each field.
                    #[allow(unreachable_code, unused_unsafe)] Type {
                        // unsafe {} block is in case this is a *union* literal, rather than
                        // a struct literal.
                        $($name: panic!("{}", unsafe {&x.$name} as *const _ as usize),)*
                    };
                };

                // Enforce that the type is RecursivelyUnpinned.
                $crate::macro_internal::require_recursively_pinned::<Type>();

                $(
                    let sub_ctor = $sub_ctor;
                    let field_drop = unsafe {
                        $crate::macro_internal::init_field(
                            // safety: this is almost verbatim the same as the second example at
                            // https://doc.rust-lang.org/nightly/std/ptr/macro.addr_of_mut.html
                            addr_of_mut!((*x_mut).$name),
                            sub_ctor)
                    };
                    let drop_guard = (drop_guard, field_drop);
                )*
                ::std::mem::forget(drop_guard);
            })
        }
    };

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
    unsafe fn reconstruct_unchecked(self: Pin<&mut Self>, ctor: impl Ctor<Output = Self>) {
        let self_ptr = Pin::into_inner_unchecked(self) as *mut _;
        std::ptr::drop_in_place(self_ptr);
        abort_on_unwind(move || {
            let maybe_uninit_self = &mut *(self_ptr as *mut MaybeUninit<Self>);
            ctor.ctor(Pin::new_unchecked(maybe_uninit_self));
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
    fn reconstruct(self: Pin<&mut Self>, ctor: impl Ctor<Output = Self>) {
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

    // It turns out this is a bad idea, because FFI unwinding, if/when added to
    // Rust, will sidestep catch_unwind, but NOT sidestep drop. So a C++
    // exception thrown in a Ctor might not get converted to a panic, but
    // instead to raw foreign unwinding, and drop impls in parent scopes
    // would be run. Since abort_on_unwind is intended to prevent *any* code from
    // executing and seeing temporarily-invalid state, this is incorrect.
    //
    // See e.g.: https://rust-lang.github.io/rfcs/2945-c-unwind-abi.html
    //
    // Instead, the only correct way to protect code from unwinding, even from
    // foreign unwinding, is via drop, as so:

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
    std::mem::forget(bomb);
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

/// Assignment from a Move desugars to an assignment from an RvalueReference.
impl<T: for<'a> Assign<RvalueReference<'a, T>>, P: DerefMut<Target = T>> Assign<Move<P>> for T {
    fn assign(self: Pin<&mut Self>, mut src: Move<P>) {
        self.assign(RvalueReference(src.0.as_mut()));
    }
}

/// Assignment from a ConstMove desugars to an assignment from a
/// ConstRvalueReference.
impl<T: for<'a> Assign<ConstRvalueReference<'a, T>>, P: Deref<Target = T>> Assign<ConstMove<P>>
    for T
{
    fn assign(self: Pin<&mut Self>, src: ConstMove<P>) {
        self.assign(ConstRvalueReference(&*src.0));
    }
}

// TODO(jeanpierreda): Make these less repetitive.

impl<'a, T: Unpin + CtorNew<&'a T>> Assign<&'a T> for T {
    fn assign(mut self: Pin<&mut Self>, src: &'a Self) {
        if std::mem::needs_drop::<Self>() {
            let mut constructed = MaybeUninit::uninit();
            unsafe {
                T::ctor_new(src).ctor(Pin::new(&mut constructed));
                *self = constructed.assume_init();
            }
        } else {
            self.reconstruct(T::ctor_new(src));
        }
    }
}

impl<'a, T: Unpin + CtorNew<RvalueReference<'a, T>>> Assign<RvalueReference<'a, T>> for T {
    fn assign(mut self: Pin<&mut Self>, src: RvalueReference<'a, Self>) {
        if std::mem::needs_drop::<Self>() {
            let mut constructed = MaybeUninit::uninit();
            unsafe {
                T::ctor_new(src).ctor(Pin::new(&mut constructed));
                *self = constructed.assume_init();
            }
        } else {
            self.reconstruct(T::ctor_new(src));
        }
    }
}

impl<'a, T: Unpin + CtorNew<ConstRvalueReference<'a, T>>> Assign<ConstRvalueReference<'a, T>>
    for T
{
    fn assign(mut self: Pin<&mut Self>, src: ConstRvalueReference<'a, Self>) {
        if std::mem::needs_drop::<Self>() {
            let mut constructed = MaybeUninit::uninit();
            unsafe {
                T::ctor_new(src).ctor(Pin::new(&mut constructed));
                *self = constructed.assume_init();
            }
        } else {
            self.reconstruct(T::ctor_new(src));
        }
    }
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
    type CtorType: Ctor<Output = Self>;

    fn ctor_new(args: ConstructorArgs) -> Self::CtorType;
}

/// Degenerate singleton tuple constructor.
// Implementation note: This would be much  better the other way around, where
// you define it for singleton tuples and it gets derived for non-tuples. That
// way, both the code implementing *and* the code using the trait can be generic
// over tuple member counts. The way it is defined right now, any impl must
// specifically special-case 1-tuples and define them as non-tuple args instead.
//
// (That would be relevant for e.g. code generation. Of course, if code
// generation is sufficiently textual, and chooses not to include the trialing
// comma, then it can still define the 1-arg case well enough.)
//
// Unfortunately, I don't know how to flip this the way I want after all:
// https://play.rust-lang.org/?gist=87e3bbdbe8ea6ac0e2d16f4d3a9a37b2
impl<FromType, T: CtorNew<FromType>> CtorNew<(FromType,)> for T {
    type CtorType = <Self as CtorNew<FromType>>::CtorType;

    fn ctor_new(args: (FromType,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as CtorNew<FromType>>::ctor_new(arg)
    }
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
impl Ctor for PhantomPinnedCtor {
    type Output = std::marker::PhantomPinned;
    unsafe fn ctor(self, dest: Pin<&mut MaybeUninit<Self::Output>>) {
        RustMoveCtor(std::marker::PhantomPinned).ctor(dest)
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

impl<T: Ctor> Ctor for ManuallyDropCtor<T> {
    type Output = ManuallyDrop<T::Output>;
    unsafe fn ctor(self, dest: Pin<&mut MaybeUninit<Self::Output>>) {
        // Safety: ManuallyDrop<T> and T have the same layout.
        let dest = Pin::into_inner_unchecked(dest);
        let dest = &mut *(dest as *mut _ as *mut MaybeUninit<T::Output>);
        let dest = Pin::new_unchecked(dest);
        self.0.ctor(dest);
    }
}

impl<T: Ctor> !Unpin for ManuallyDropCtor<T> {}

#[cfg(test)]
mod test {
    use super::*;
    use std::cell::RefCell;
    use std::sync::Mutex;

    #[test]
    fn test_default_rust_type() {
        emplace! {let x = u32::ctor_new(());}
        assert_eq!(*x, 0);
    }

    #[test]
    fn test_copy_rust_type() {
        let x: u32 = 42;
        let mut y = Box::emplace(copy(&x));
        assert_eq!(x, 42);
        assert_eq!(*y, 42);

        // Can also mutate:
        let x = 50;
        y.as_mut().assign(&x);
        assert_eq!(x, 50);
        assert_eq!(*y, 50);

        // Can also mutate:
        let x = 100;
        y.as_mut().assign(copy(&x));
        assert_eq!(x, 100);
        assert_eq!(*y, 100);
    }

    #[test]
    fn test_copy_smart_ptr() {
        let x = Box::new(42_u32);
        emplace! {
            let y = copy(x);
        }
        // x is no longer in scope, as it was moved into the Copy.
        assert_eq!(*y, 42);
    }

    /// Tests that the assigned variables have the correct type.
    #[test]
    fn test_emplace_type() {
        let x: u32 = 42;
        emplace! {
            let foo = copy(&x);
        }
        let _foo: Pin<&mut u32> = foo; // type checks OK
    }

    #[test]
    fn test_emplace() {
        let x: u32 = 42;
        emplace! {
            let foo = copy(&x);
        }
        assert_eq!(*foo, 42);
    }

    #[test]
    fn test_emplace_mut() {
        let x: u32 = 42;
        emplace! {
            let mut foo = copy(&x);
        }
        assert_eq!(*foo, 42);
        *foo = 0;
        assert_eq!(*foo, 0);
    }

    #[test]
    fn test_emplace_multi() {
        let x: u32 = 42;
        emplace! {
            let foo = copy(&x);
            let bar = copy(&*foo);
        }
        assert_eq!(*foo, 42);
        assert_eq!(*bar, 42);
    }

    #[test]
    fn test_emplace_type_syntax() {
        let x: u32 = 42;
        emplace! {
            let mut foo: Pin<&mut u32> = copy(&x);
            let bar: Pin<&mut u32> = copy(&x);
        }
        assert_eq!(*foo, 42);
        *foo = 0;
        assert_eq!(*foo, 0);
        assert_eq!(*bar, 42);
    }

    #[test]
    fn test_ctor_macro() {
        struct MyStruct {
            x: u32,
            y: u32,
        }
        unsafe impl RecursivelyPinned for MyStruct {}
        emplace! { let my_struct = ctor!(MyStruct {
            x: 4,
            y: copy(&2)
        });}
        assert_eq!(my_struct.x, 4);
        assert_eq!(my_struct.y, 2);

        // test that trailing commas compile:
        #[rustfmt::skip]
        let _ = ctor!(MyStruct {
            x: 0,
            y: 0,
        });
    }

    #[test]
    fn test_ctor_macro_named_tuple_struct() {
        struct MyStruct(u32, u32);
        unsafe impl RecursivelyPinned for MyStruct {}
        emplace! { let my_struct = ctor!(MyStruct {
            0: 4,
            1: copy(&2)
        });}
        assert_eq!(my_struct.0, 4);
        assert_eq!(my_struct.1, 2);
    }

    #[test]
    fn test_ctor_macro_tuple_struct() {
        struct MyStruct(u32, u32);
        unsafe impl RecursivelyPinned for MyStruct {}
        emplace! { let my_struct = ctor!(MyStruct (4, copy(&2)));}
        assert_eq!(my_struct.0, 4);
        assert_eq!(my_struct.1, 2);
    }

    #[test]
    fn test_ctor_macro_manuallydrop_struct() {
        struct MyStruct {
            x: ManuallyDrop<Vec<u32>>,
            y: u64,
        }
        unsafe impl RecursivelyPinned for MyStruct {}
        emplace! {let my_struct = ctor!(MyStruct {x: unsafe {ManuallyDropCtor::new(vec![42])}, y: 0 }); }
        assert_eq!(&*my_struct.x, &vec![42]);
        assert_eq!(my_struct.y, 0);
    }

    #[test]
    fn test_ctor_macro_union() {
        union MyUnion {
            x: ManuallyDrop<Vec<u32>>,
            y: u64,
        }
        unsafe impl RecursivelyPinned for MyUnion {}
        emplace! {let mut my_union = ctor!(MyUnion {x: unsafe { ManuallyDropCtor::new(vec![42])} }); }
        assert_eq!(unsafe { &*my_union.x }, &vec![42]);

        // And now write to the union.

        // First, should drop myunion.x.
        unsafe { ManuallyDrop::drop(&mut Pin::into_inner_unchecked(my_union.as_mut()).x) }
        // Second, we can overwrite.
        my_union.as_mut().reconstruct(ctor!(MyUnion { y: 24 }));
        assert_eq!(unsafe { my_union.y }, 24);
    }

    #[test]
    fn test_ctor_macro_nested_struct() {
        mod nested {
            pub struct MyStruct {
                pub x: u32,
                pub y: u32,
            }
            unsafe impl crate::RecursivelyPinned for MyStruct {}
        }
        emplace! { let my_struct = ctor!(nested::MyStruct {
            x: 4,
            y: copy(&2)
        });}
        assert_eq!(my_struct.x, 4);
        assert_eq!(my_struct.y, 2);
    }

    #[test]
    fn test_ctor_macro_nested_tuple_struct() {
        mod nested {
            pub struct MyStruct(pub u32, pub u32);
            unsafe impl crate::RecursivelyPinned for MyStruct {}
        }
        emplace! { let my_struct = ctor!(nested::MyStruct (4, copy(&2)));}
        assert_eq!(my_struct.0, 4);
        assert_eq!(my_struct.1, 2);
    }

    /// Test that the ctor macro safety check doesn't rely on the struct not
    /// implementing Drop.
    #[test]
    fn test_ctor_macro_drop_struct() {
        struct MyStruct {
            x: String,
        }
        unsafe impl RecursivelyPinned for MyStruct {}
        impl Drop for MyStruct {
            fn drop(&mut self) {}
        }

        emplace! { let _my_struct = ctor!(MyStruct {
            x: "".to_string()
        });}
    }

    /// Struct which sets a bool to true when dropped.
    ///
    /// We use Mutex so as to allow for mutating state in a way Rust believes is
    /// unwind-safe.
    struct DropNotify<'a>(&'a Mutex<bool>);

    impl Drop for DropNotify<'_> {
        fn drop(&mut self) {
            *self.0.lock().unwrap() = true;
        }
    }

    /// Ctor which constructs a DropNotify but panics before completing
    /// construction.
    ///
    /// This can be used to test both that a drop occurs (for e.g. previously
    /// constructed DropNotify objects), as well as that one does not:
    /// because it in fact does fully initialize the thing it is meant to
    /// construct, it is not UB to then call drop on it -- even though it would
    /// be UB for almost any other Ctor.
    #[must_use = must_use_ctor!()]
    struct PanicCtor<'a>(DropNotify<'a>);

    impl<'a> Ctor for PanicCtor<'a> {
        type Output = DropNotify<'a>;
        unsafe fn ctor(self, dest: Pin<&mut MaybeUninit<Self::Output>>) {
            self.0.ctor(dest);
            panic!();
        }
    }

    impl !Unpin for PanicCtor<'_> {}

    /// Tests that drop() is called when the Ctor doesn't panic.
    #[test]
    fn test_emplace_drop() {
        let is_dropped = Mutex::new(false);
        {
            emplace! { let _my_drop_notify = DropNotify(&is_dropped); }
        }
        assert!(*is_dropped.lock().unwrap());
    }

    /// Tests that when a panic occurs during emplace!{}, the uninitialized
    /// value is not dropped.
    #[test]
    fn test_emplace_no_drop_on_panic() {
        let is_dropped = Mutex::new(false);
        let panic_result = std::panic::catch_unwind(|| {
            emplace! { let _my_drop_notify = PanicCtor(DropNotify(&is_dropped)); }
        });
        assert!(panic_result.is_err());
        assert!(!*is_dropped.lock().unwrap());
    }

    /// Tests that when a panic occurs during initialization of a struct with
    /// ctor!, the initialized fields are dropped, and the uninitialized
    /// fields are not.
    #[test]
    fn test_ctor_macro_drop() {
        struct MyStruct<'a> {
            x: DropNotify<'a>,
            y: DropNotify<'a>,
        }
        unsafe impl RecursivelyPinned for MyStruct<'_> {}

        let x_dropped = Mutex::new(false);
        let y_dropped = Mutex::new(false);
        let panic_result = std::panic::catch_unwind(|| {
            emplace! { let _my_struct = ctor!(MyStruct {
                x: DropNotify(&x_dropped),
                y: PanicCtor(DropNotify(&y_dropped))
            });}
        });
        assert!(panic_result.is_err());
        assert!(*x_dropped.lock().unwrap());
        assert!(!*y_dropped.lock().unwrap());
    }

    /// logs calls to the constructors, drop.
    struct DropCtorLogger<'a> {
        log: &'a RefCell<Vec<&'static str>>,
    }

    impl Drop for DropCtorLogger<'_> {
        fn drop(&mut self) {
            self.log.borrow_mut().push("drop");
        }
    }

    struct LoggingCtor<'a> {
        log: &'a RefCell<Vec<&'static str>>,
        ctor_message: &'static str,
    }

    impl<'a> Ctor for LoggingCtor<'a> {
        type Output = DropCtorLogger<'a>;
        unsafe fn ctor(self, mut dest: Pin<&mut MaybeUninit<Self::Output>>) {
            self.log.borrow_mut().push(self.ctor_message);
            dest.as_mut_ptr().write(DropCtorLogger { log: self.log });
        }
    }
    impl !Unpin for LoggingCtor<'_> {}

    impl<'a> CtorNew<&DropCtorLogger<'a>> for DropCtorLogger<'a> {
        type CtorType = LoggingCtor<'a>;
        fn ctor_new(src: &DropCtorLogger<'a>) -> Self::CtorType {
            LoggingCtor { log: &src.log, ctor_message: "copy ctor" }
        }
    }

    impl<'a> CtorNew<RvalueReference<'_, DropCtorLogger<'a>>> for DropCtorLogger<'a> {
        type CtorType = LoggingCtor<'a>;
        fn ctor_new(src: RvalueReference<'_, DropCtorLogger<'a>>) -> Self::CtorType {
            LoggingCtor { log: &src.0.log, ctor_message: "move ctor" }
        }
    }

    /// Tests the ctor/drop order for copy-constructible Unpin types: ctor comes
    /// before drop.
    #[test]
    fn test_copy_ctor_drop_order() {
        let log = RefCell::new(vec![]);
        let log = &log;

        emplace! {
            let notify_tester = DropCtorLogger {log};
            let new_value = DropCtorLogger {log};
        }
        notify_tester.assign(&*new_value);
        assert_eq!(*log.borrow(), vec!["copy ctor", "drop"]);
    }

    /// Tests the ctor/drop order for move-constructible Unpin types: ctor comes
    /// before drop.
    #[test]
    fn test_move_ctor_drop_order() {
        let log = RefCell::new(vec![]);
        let log = &log;

        emplace! {
            let notify_tester = DropCtorLogger {log};
            let new_value = DropCtorLogger {log};
        }
        notify_tester.assign(mov(new_value));
        assert_eq!(*log.borrow(), vec!["move ctor", "drop"]);
    }

    #[test]
    fn test_ctor_then() {
        emplace! {
            let x = 40.ctor_then(|mut y| { *y += 2 });
        }
        assert_eq!(*x, 42);
    }

    /// Test that a slot can be created in a temporary.
    #[test]
    fn test_slot_temporary() {
        assert_eq!(*emplace!(Slot::new(42)).as_opt().unwrap(), 42);
    }

    /// Test that a slot can be created in a local.
    #[test]
    fn test_slot_local() {
        emplace! {let slot = Slot::new(42); }
        assert_eq!(*slot.as_opt().unwrap(), 42);
    }

    /// Shows the use of Slot to implement a "slotted return value", similar to
    /// moveit.
    #[test]
    fn test_slotted_return_value() {
        // TODO(jeanpierreda): delete this, use doctests when doctests work.
        fn foo(slot: Pin<&mut Slot<u32>>) -> Pin<&mut u32> {
            slot.replace(42)
        }

        emplace! {let slot = Slot::uninit(); }
        let rv = foo(slot);
        assert_eq!(*rv, 42);
    }

    /// Shows the use of Slot to implement output parameters.
    #[test]
    fn test_slotted_output_parameter() {
        // TODO(jeanpierreda): delete this, use doctests when doctests work.
        fn foo(slot: Pin<&mut Slot<u32>>) {
            slot.replace(42);
        }

        emplace! {let mut slot = Slot::uninit(); }
        foo(slot.as_mut());
        assert_eq!(*slot.as_opt().unwrap(), 42);
    }
}
