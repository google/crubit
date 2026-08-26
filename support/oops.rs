// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#![feature(negative_impls)]

//! # Object-Oriented Programming Support (OOPS).
//!
//! ## Upcasting
//!
//! To cast a reference to its base class type, use `my_reference.upcast()` when base class is not
//! virtual else use `my_reference.virtual_upcast()`.
//!
//! For example:
//!
//! ```ignore
//! let x : *mut Derived = ...;
//! let y : *mut Base = x.upcast();
//! let z : *mut VirtualBase = x.virtual_upcast();
//! ```
//!
//! To implement upcasting, implement the `Inherits` trait for non-virtual bases and
//! `InheritsVirtual` for virtual bases.
//!
//! ## Downcasting
//!
//! TODO(b/216195042): dynamic downcasting
//! TODO(b/216195042): static downcasting

/// Trait for upcasting a Derived class to its non-virtual Base class.
pub trait Upcast<Target> {
    fn upcast(self) -> Target;
}

/// Upcast `*const` -> `*const`.
impl<Derived, Base> Upcast<*const Base> for *const Derived
where
    Derived: Inherits<Base>,
{
    fn upcast(self: *const Derived) -> *const Base {
        Derived::upcast_ptr(self)
    }
}

/// Upcast `*mut` -> `*mut`.
impl<Derived, Base> Upcast<*mut Base> for *mut Derived
where
    Derived: Inherits<Base>,
{
    fn upcast(self: *mut Derived) -> *mut Base {
        Derived::upcast_ptr_mut(self)
    }
}

/// Upcast `&` -> `&`.
impl<'a, Derived, Base> Upcast<&'a Base> for &'a Derived
where
    Derived: Inherits<Base>,
{
    fn upcast(self: &'a Derived) -> &'a Base {
        // SAFETY: `self` is a valid reference for `'a`, so `self as *const Derived` is non-null and
        // aligned. `upcast_ptr` returns a non-null, properly aligned pointer to the `Base`
        // subobject within `self`.
        unsafe { &*Derived::upcast_ptr(self as *const Derived) }
    }
}

/// Upcast a raw pointer. `Derived : Inherits<Base>` means that `Derived` can
/// be upcast to `Base`.
pub trait Inherits<Base> {
    /// Upcast a `const` pointer.
    fn upcast_ptr(derived: *const Self) -> *const Base;

    /// Upcast a `mut` pointer.
    fn upcast_ptr_mut(derived: *mut Self) -> *mut Base {
        Self::upcast_ptr(derived) as *mut _
    }
}

/// All classes are their own improper base.
impl<T> Inherits<T> for T {
    fn upcast_ptr(derived: *const Self) -> *const Self {
        derived
    }
}

/// Trait for upcasting a Derived class to its virtual Base class.
pub trait VirtualUpcast<Target> {
    fn virtual_upcast(self) -> Target;
}

/// Upcast `*const` -> `*const` for virtual base class.
impl<Derived, Base> VirtualUpcast<*const Base> for *const Derived
where
    Derived: InheritsVirtual<Base>,
{
    fn virtual_upcast(self: *const Derived) -> *const Base {
        // SAFETY: `self` is a valid reference for `'a`, so `self as *const Derived` is non-null and
        // aligned. `upcast_ptr_to_virtual_base` returns a non-null, properly aligned pointer to the
        // `Base` subobject within `self`.
        unsafe { Derived::upcast_ptr_to_virtual_base(self) }
    }
}

/// Upcast `*mut` -> `*mut` for virtual base class.
impl<Derived, Base> VirtualUpcast<*mut Base> for *mut Derived
where
    Derived: InheritsVirtual<Base>,
{
    fn virtual_upcast(self: *mut Derived) -> *mut Base {
        // SAFETY: `self` is a valid reference for `'a`, so `self as *const Derived` is non-null and
        // aligned. `upcast_ptr_mut_to_virtual_base` returns a non-null, properly aligned pointer to
        // the `Base` subobject within `self`.
        unsafe { Derived::upcast_ptr_mut_to_virtual_base(self) }
    }
}

/// Unsafely upcast a raw pointer to its virtual base class. `Derived : Inherits<Base>`
/// means that `Derived` can be upcast to `Base`.
///
/// To upcast in safe code, use the `Upcast` trait. `InheritsVirtual` is used for
/// unsafe pointer upcasts, and to implement upcasting.
///
/// (Note that unlike `Upcast`, `InheritsVirtual` is not implemented on the pointers
/// themselves -- this is solely for trait coherence reasons, as owning `T` does
/// not currently grant ownership over `*const T` or `*mut T`.)
///
/// ## Safety
///
/// Implementations must uphold the safety contract of the unsafe functions in
/// this trait.
pub unsafe trait InheritsVirtual<Base> {
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
    unsafe fn upcast_ptr_to_virtual_base(derived: *const Self) -> *const Base;

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
    unsafe fn upcast_ptr_mut_to_virtual_base(derived: *mut Self) -> *mut Base {
        // SAFETY: This function has the same safety contract as `Self::upcast_ptr`.
        unsafe { Self::upcast_ptr_to_virtual_base(derived) as *mut _ }
    }
}

/// All classes are their own improper base.
unsafe impl<T> InheritsVirtual<T> for T {
    unsafe fn upcast_ptr_to_virtual_base(derived: *const Self) -> *const Self {
        derived
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use googletest::gtest;

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

        impl Inherits<Base> for Derived {
            fn upcast_ptr(derived: *const Self) -> *const Base {
                // SAFETY: `derived` is a valid pointer to a `Derived` value.
                unsafe { &(*derived).base }
            }
        }
        let mut derived = Derived::default();
        assert_eq!(ptr_location(&derived.base), ptr_location::<&Base>((&derived).upcast()));

        let _: *const Base = Derived::upcast_ptr(&derived);
        let _: *mut Base = Derived::upcast_ptr_mut(&mut derived);
        let _: *const Base = (&derived as *const Derived).upcast();
        let _: *mut Base = (&mut derived as *mut Derived).upcast();
        // let _: &mut Base = (&mut derived).upcast(); // does not compile
        // let _: Pin<&mut Base> = (&mut derived).upcast(); // does not compile
        // let _: Pin<&mut Base> = Pin::new(&mut derived).upcast(); // does not compile

        // This write must not be UB:
        {
            let base: *mut Base = (&mut derived as *mut Derived).upcast();
            // SAFETY: `base` is a valid pointer to a `Base` value.
            unsafe { (*base).0 = 42 };
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

        impl Inherits<Base> for Derived {
            fn upcast_ptr(derived: *const Self) -> *const Base {
                // SAFETY: `derived` is a valid pointer to a `Derived` value.
                unsafe { &(*derived).base }
            }
        }
        let mut derived = Derived::default();
        assert_eq!(ptr_location(&derived.base), ptr_location::<&Base>((&derived).upcast()));

        let _: *const Base = Derived::upcast_ptr(&derived);
        let _: *mut Base = Derived::upcast_ptr_mut(&mut derived);
        // let _: &mut Base = (&mut derived).upcast(); // does not compile
        // let _: Pin<&mut Base> = (&mut derived).upcast(); // does not compile
        // let _: Pin<&mut Base> = Pin::new(&mut derived).upcast(); // does not compile

        // This write must not be UB:
        {
            let base: *mut Base = Derived::upcast_ptr_mut(&mut derived);
            // SAFETY: `base` is a valid pointer to a `Base` value.
            unsafe { (*base).0 = 42 };
        }
        assert_eq!(derived.base.0, 42);
    }
}
