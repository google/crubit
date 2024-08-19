// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#![cfg_attr(test, feature(negative_impls))]

//! # Object-Oriented Programming Support (OOPS).
//!
//! ## Upcasting
//!
//! To cast a reference to its base class type, use `my_reference.upcast()`.
//! For example:
//!
//! ```ignore
//! let x : &mut Derived = ...;
//! let y : Pin<&mut Base> = x.upcast();
//! ```
//!
//! Because base classes are always `!Unpin`, mutable references to base must
//! take the form of `Pin<&mut Base>`. See
//! docs/unpin.md
//!
//! To implement upcasting, implement the `Inherits` trait.
//!
//! ## Downcasting
//!
//! TODO(b/216195042): dynamic downcasting
//! TODO(b/216195042): static downcasting

use std::pin::Pin;

/// Upcast a reference or smart pointer. This operation cannot fail at runtime.
///
/// If `Derived` has a (public, unambiguous) base class `Base`, then:
///
/// ```ignore
/// &Derived : Upcast<&Base>
/// Pin<&mut Derived> : Upcast<Pin<&mut Base>>
/// ```
///
/// In addition, if `Derived : Unpin`, then `&mut Derived : Upcast<Pin<&mut
/// Base>>`.
///
/// (And, while it is not possible in Crubit bindings, if `Base` is also
/// `Unpin`, then  `&mut Derived : Upcast<&mut Base>`.)
///
/// For the purpose of `Upcast`, any type `T` is its own ("improper") base
/// class.
pub trait Upcast<Target> {
    fn upcast(self) -> Target;
}

/// Upcast `&` -> `&`.
impl<'a, Derived, Base> Upcast<&'a Base> for &'a Derived
where
    Derived: Inherits<Base>,
{
    fn upcast(self: &'a Derived) -> &'a Base {
        unsafe { &*Derived::upcast_ptr(self as *const Derived) }
    }
}

/// Upcast `Pin<&mut>` -> `Pin<&mut>.
impl<'a, Derived, Base> Upcast<Pin<&'a mut Base>> for Pin<&'a mut Derived>
where
    Derived: Inherits<Base>,
{
    fn upcast(self: Pin<&'a mut Derived>) -> Pin<&'a mut Base> {
        unsafe {
            let inner = Pin::into_inner_unchecked(self) as *mut Derived;
            Pin::new_unchecked(&mut *Derived::upcast_ptr_mut(inner))
        }
    }
}

/// Upcast `&mut` -> `Pin<&mut>.
///
/// Since all C++ base classes are `!Unpin`, this is the normal shape of a
/// mutable reference upcast for an `Unpin` derived class.
impl<'a, Derived, Base> Upcast<Pin<&'a mut Base>> for &'a mut Derived
where
    Pin<&'a mut Derived>: Upcast<Pin<&'a mut Base>>,
    Derived: Unpin,
{
    fn upcast(self: &'a mut Derived) -> Pin<&'a mut Base> {
        Pin::new(self).upcast()
    }
}

/// Upcast `&mut` -> `&mut`.
///
/// This impl is never applicable to C++ types (a C++ base class is `!Unpin`),
/// but could work for inheritance implemented in pure Rust.
impl<'a, Derived, Base> Upcast<&'a mut Base> for &'a mut Derived
where
    Pin<&'a mut Derived>: Upcast<Pin<&'a mut Base>>,
    Derived: Unpin,
    Base: Unpin,
{
    fn upcast(self: &'a mut Derived) -> &'a mut Base {
        Pin::into_inner(Pin::new(self).upcast())
    }
}

/// Upcast a raw pointer from derived class to base class.
/// This operation can fail at runtime.
///
/// ## Safety
///
/// `self` must be `*const T` or `*mut T`
pub unsafe trait UnsafeUpcast<Target> {
    /// # Safety
    ///
    /// `self` must be a valid pointer, e.g. a pointer to a live derived object.
    unsafe fn unsafe_upcast(self) -> Target;
}

/// Upcast `*const` -> `*const`.
unsafe impl<Derived, Base> UnsafeUpcast<*const Base> for *const Derived
where
    Derived: Inherits<Base>,
{
    unsafe fn unsafe_upcast(self: *const Derived) -> *const Base {
        Derived::upcast_ptr(self)
    }
}

/// Upcast `*mut` -> `*mut`.
unsafe impl<Derived, Base> UnsafeUpcast<*mut Base> for *mut Derived
where
    Derived: Inherits<Base>,
{
    unsafe fn unsafe_upcast(self: *mut Derived) -> *mut Base {
        Derived::upcast_ptr_mut(self)
    }
}

/// Unsafely upcast a raw pointer. `Derived : Inherits<Base>` means that
/// `Derived` can be upcast to `Base`.
///
/// To upcast in safe code, use the `Upcast` trait. `Inherits` is used for
/// unsafe pointer upcasts, and to implement upcasting.
///
/// (Note that unlike `Upcast`, `Inherits` is not implemented on the pointers
/// themselves -- this is solely for trait coherence reasons, as owning `T` does
/// not currently grant ownership over `*const T` or `*mut T`.)
///
/// ## Safety
///
/// Implementations must uphold the safety contract of the unsafe functions in
/// this trait.
///
/// TODO(jeanpierreda): Should this be split into two traits?
/// We could have `Inherits` (with safe functions) and `InheritsVirtual` (with
/// unsafe functions). For now, these are all merged into one trait, as it is
/// not an immediately obvious benefit to make raw pointer upcasts a safe
/// operation.
pub unsafe trait Inherits<Base> {
    /// Upcast a `const` pointer.
    ///
    /// ## Safety
    ///
    /// Casting follows the same safety and dereferencability rules as C++:
    ///
    /// If `derived` is a dereferencable pointer, then the upcasted pointer is a
    /// dereferencable pointer with the same lifetime.
    ///
    /// If `derived` is null, this returns null.
    ///
    /// If `derived` is non-dereferencable, and `Base` is a non-virtual base
    /// class, then the return value is non-dereferencable.
    ///
    /// Otherwise, if `derived` is non-dereferencable and `Base` is a virtual
    /// base class, the behavior is undefined.
    unsafe fn upcast_ptr(derived: *const Self) -> *const Base;

    /// Upcast a `mut` pointer.
    ///
    /// ## Safety
    ///
    /// Casting follows the same safety and dereferencability rules as C++:
    ///
    /// If `derived` is a dereferencable pointer, then the upcasted pointer is a
    /// dereferencable pointer with the same lifetime.
    ///
    /// If `derived` is null, this returns null.
    ///
    /// If `derived` is non-dereferencable, and `Base` is a non-virtual base
    /// class, then the return value is non-dereferencable.
    ///
    /// Otherwise, if `derived` is non-dereferencable and `Base` is a virtual
    /// base class, the behavior is undefined.
    unsafe fn upcast_ptr_mut(derived: *mut Self) -> *mut Base {
        Self::upcast_ptr(derived) as *mut _
    }
}

/// All classes are their own improper base.
unsafe impl<T> Inherits<T> for T {
    unsafe fn upcast_ptr(derived: *const Self) -> *const Self {
        derived
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use googletest::prelude::*;

    fn ptr_location<T: std::ops::Deref>(x: T) -> usize {
        &*x as *const _ as *const u8 as usize
    }

    #[gtest]
    fn test_unpin_upcast() {
        #[derive(Default)]
        struct Base(i32);

        #[derive(Default)]
        struct Derived {
            _other_field: u32,
            base: Base,
        }

        unsafe impl Inherits<Base> for Derived {
            unsafe fn upcast_ptr(derived: *const Self) -> *const Base {
                &(*derived).base
            }
        }
        let mut derived = Derived::default();
        assert_eq!(ptr_location(&derived.base), ptr_location::<&Base>((&derived).upcast()));

        let _: *const Base = unsafe { Derived::upcast_ptr(&derived) };
        let _: *mut Base = unsafe { Derived::upcast_ptr_mut(&mut derived) };
        let _: *const Base = unsafe { (&derived as *const Derived).unsafe_upcast() };
        let _: *mut Base = unsafe { (&mut derived as *mut Derived).unsafe_upcast() };
        let _: &mut Base = (&mut derived).upcast();
        let _: Pin<&mut Base> = (&mut derived).upcast();
        let _: Pin<&mut Base> = Pin::new(&mut derived).upcast();

        // This write must not be UB:
        {
            let base: &mut Base = (&mut derived).upcast();
            base.0 = 42;
        }
        assert_eq!(derived.base.0, 42);
    }

    #[gtest]
    fn test_nonunpin_upcast() {
        #[derive(Default)]
        struct Base(i32);
        impl !Unpin for Base {}

        #[derive(Default)]
        struct Derived {
            _other_field: u32,
            base: Base,
        }
        impl Unpin for Derived {}

        unsafe impl Inherits<Base> for Derived {
            unsafe fn upcast_ptr(derived: *const Self) -> *const Base {
                &(*derived).base
            }
        }
        let mut derived = Derived::default();
        assert_eq!(ptr_location(&derived.base), ptr_location::<&Base>((&derived).upcast()));

        let _: *const Base = unsafe { Derived::upcast_ptr(&derived) };
        let _: *mut Base = unsafe { Derived::upcast_ptr_mut(&mut derived) };
        // let _: &mut Base = (&mut derived).upcast(); // does not compile
        let _: Pin<&mut Base> = (&mut derived).upcast();
        let _: Pin<&mut Base> = Pin::new(&mut derived).upcast();

        // This write must not be UB:
        unsafe {
            let base: *mut Base = Derived::upcast_ptr_mut(&mut derived);

            (&mut *base).0 = 42;
        }
        assert_eq!(derived.base.0, 42);
    }
}
