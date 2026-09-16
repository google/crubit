// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#![feature(negative_impls)]

//! # Object-Oriented Programming Support (OOPS).
//!
//! ## Upcasting
//!
//! To cast a reference or pointer to its base class type, use `.upcast()` when the base class is
//! not virtual, use `.virtual_upcast()` on raw pointers when the base class is virtual.
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
///
/// # Safety
///
/// Implementations of `Upcast` must guarantee that they fulfill the safety promises of `upcast`.
pub unsafe trait Upcast<Target> {
    /// Upcasts `self` to the target `Base` type.
    ///
    /// # Safety
    ///
    /// If `self` is a raw pointer, `self` must be a valid, dereferenceable pointer to an
    /// initialized object of the derived class.
    ///
    /// # Safety promises
    ///
    /// If `self` is a valid pointer or reference, the returned target will be a valid, properly
    /// aligned, and dereferenceable pointer or reference to the `Target` base subobject within
    /// `self`, with the same lifetime as of `self`.
    unsafe fn upcast(self) -> Target;
}

/// Upcast `*const` -> `*const`.
///
/// # Safety
///
/// Implementing upcast for converting `Derived` ptr to `Base` ptr is safe, because for a valid
/// Derived pointer, Inherits promises to return a valid base object inside Derived.
unsafe impl<Derived, Base> Upcast<*const Base> for *const Derived
where
    Derived: Inherits<Base>,
{
    unsafe fn upcast(self: *const Derived) -> *const Base {
        // SAFETY: By the safety contract of `Upcast::upcast`, `self` is a valid,
        // dereferenceable pointer to a `Derived` object, which satisfies the safety
        // contract of `Derived::upcast_ptr`.
        unsafe { Derived::upcast_ptr(self) }
    }
}

/// Upcast `*mut` -> `*mut`.
///
/// # Safety
///
/// Implementing upcast for converting `Derived` ptr to `Base` ptr is safe, because for a valid
/// Derived pointer, Inherits promises to return a valid base object inside Derived.
unsafe impl<Derived, Base> Upcast<*mut Base> for *mut Derived
where
    Derived: Inherits<Base>,
{
    unsafe fn upcast(self: *mut Derived) -> *mut Base {
        // SAFETY: By the safety contract of `Upcast::upcast`, `self` is a valid,
        // dereferenceable pointer to a `Derived` object, which satisfies the safety
        // contract of `Derived::upcast_ptr_mut`.
        unsafe { Derived::upcast_ptr_mut(self) }
    }
}

/// Upcast `&` -> `&`.
///
/// # Safety
///
/// Implementing upcast for converting `Derived` ptr to `Base` ptr is safe, because for a valid
/// Derived reference, Inherits promises to return a valid base object inside Derived.
unsafe impl<'a, Derived, Base> Upcast<&'a Base> for &'a Derived
where
    Derived: Inherits<Base>,
{
    unsafe fn upcast(self: &'a Derived) -> &'a Base {
        // SAFETY: By the safety contract of `Upcast::upcast`, `self` is a valid reference
        // to a `Derived` object, which satisfies the safety contract of `Derived::upcast_ptr`.
        unsafe { &*Derived::upcast_ptr(self as *const Derived) }
    }
}

/// Upcast a raw pointer. `Derived : Inherits<Base>` means that `Derived` can
/// be upcast to `Base`.
///
/// # Safety
///
/// Implementations must uphold the safety contract and safety promises requirement mentioned
/// on associated methods of this trait.
pub unsafe trait Inherits<Base> {
    /// Upcast a `const` pointer.
    ///
    /// # Safety
    ///
    /// `derived` must be a valid, dereferenceable pointer to a `Self`.
    ///
    /// # Safety promises
    ///
    /// - If `derived` is a valid, dereferenceable pointer to a `Self`, this returns a valid,
    ///   properly aligned, and dereferenceable pointer to the `Base` subobject within `derived`.
    unsafe fn upcast_ptr(derived: *const Self) -> *const Base;

    /// Upcast a `mut` pointer.
    ///
    /// # Safety
    ///
    /// `derived` must be a valid, dereferenceable pointer to a `Self`.
    ///
    /// # Safety promises
    ///
    /// - If `derived` is a valid, dereferenceable pointer to a `Self`, this returns a valid,
    ///   properly aligned, and dereferenceable pointer to the `Base` subobject within `derived`.
    unsafe fn upcast_ptr_mut(derived: *mut Self) -> *mut Base {
        // SAFETY: This function has the same safety contract as `Self::upcast_ptr`.
        unsafe { Self::upcast_ptr(derived) as *mut _ }
    }
}

/// All classes are their own improper base.
unsafe impl<T> Inherits<T> for T {
    unsafe fn upcast_ptr(derived: *const Self) -> *const Self {
        derived
    }
}

/// Trait for upcasting a Derived class pointer to its virtual Base class.
///
/// # Safety
///
/// Implementations of `VirtualUpcast` must guarantee that they fulfill the safety promises of
/// `virtual_upcast`.
pub unsafe trait VirtualUpcast<Target> {
    /// Upcasts `self` to the target virtual base pointer type.
    ///
    /// # Safety
    ///
    /// The caller must ensure that if `self` is a ptr, it is either null or valid, dereferenceable
    /// pointer to an initialized object of the derived class.
    ///
    /// # Safety promises
    ///
    /// If `self` is a valid pointer or reference, the returned target will be a valid, properly
    /// aligned, and dereferenceable pointer or reference to the `Target` base subobject within
    /// `self`.
    unsafe fn virtual_upcast(self) -> Target;
}

/// Upcast `*const` -> `*const` for virtual base class.
unsafe impl<Derived, Base> VirtualUpcast<*const Base> for *const Derived
where
    Derived: InheritsVirtual<Base>,
{
    unsafe fn virtual_upcast(self: *const Derived) -> *const Base {
        // SAFETY: By the safety contract of `VirtualUpcast::virtual_upcast`, `self` is
        // either null or a valid, dereferenceable pointer to a `Derived` object, which
        // satisfies the safety contract of `Derived::upcast_ptr_to_virtual_base`.
        unsafe { Derived::upcast_ptr_to_virtual_base(self) }
    }
}

/// Upcast `*mut` -> `*mut` for virtual base class.
unsafe impl<Derived, Base> VirtualUpcast<*mut Base> for *mut Derived
where
    Derived: InheritsVirtual<Base>,
{
    unsafe fn virtual_upcast(self: *mut Derived) -> *mut Base {
        // SAFETY: By the safety contract of `VirtualUpcast::virtual_upcast`, `self` is
        // either null or a valid, dereferenceable pointer to a `Derived` object, which
        // satisfies the safety contract of `Derived::upcast_ptr_mut_to_virtual_base`.
        unsafe { Derived::upcast_ptr_mut_to_virtual_base(self) }
    }
}

/// Unsafely upcast a raw pointer to its virtual base class. `Derived : InheritsVirtual<Base>`
/// means that `Derived` can be upcast to `Base`.
///
/// To upcast in safe code, use the `VirtualUpcast` trait. `InheritsVirtual` is used for
/// unsafe pointer upcasts, and to implement upcasting.
///
/// # Safety
///
/// Implementations must uphold the safety contract and safety promises of the
/// unsafe functions in this trait.
pub unsafe trait InheritsVirtual<Base> {
    /// Upcast a `const` pointer.
    ///
    /// # Safety
    ///
    /// `derived` must be a valid, dereferenceable pointer to a `Self`, or null.
    ///
    /// # Safety promises
    ///
    /// - If `derived` is null, this returns a null pointer.
    /// - If `derived` is a valid, dereferenceable pointer to a `Self`, this returns a valid,
    ///   properly aligned, and dereferenceable pointer to the `Base` virtual base subobject within
    ///   `derived`, with the same lifetime as `derived`.
    unsafe fn upcast_ptr_to_virtual_base(derived: *const Self) -> *const Base;

    /// Upcast a `mut` pointer.
    ///
    /// # Safety
    ///
    /// `derived` must be a valid, dereferenceable pointer to a `Self`, or null.
    ///
    /// # Safety promises
    ///
    /// - If `derived` is null, this returns a null pointer.
    /// - If `derived` is a valid, dereferenceable pointer to a `Self`, this returns a valid,
    ///   properly aligned, and dereferenceable pointer to the `Base` virtual base subobject within
    ///   `derived`, with the same lifetime as `derived`.
    unsafe fn upcast_ptr_mut_to_virtual_base(derived: *mut Self) -> *mut Base {
        // SAFETY: This function has the same safety contract as `Self::upcast_ptr_to_virtual_base`.
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

        unsafe impl Inherits<Base> for Derived {
            unsafe fn upcast_ptr(derived: *const Self) -> *const Base {
                // SAFETY: `derived` is a valid pointer to a `Derived` value.
                unsafe { &(*derived).base }
            }
        }
        let mut derived = Derived::default();
        // SAFETY: `derived` is a valid pointer to a `Derived` value.
        assert_eq!(ptr_location(&derived.base), unsafe {
            ptr_location::<&Base>((&derived).upcast())
        });

        let _: *const Base = unsafe { Derived::upcast_ptr(&derived) };
        let _: *mut Base = unsafe { Derived::upcast_ptr_mut(&mut derived) };
        // SAFETY: `derived` is a valid pointer to a `Derived` value.
        unsafe {
            let _: *const Base = (&derived as *const Derived).upcast();
            let _: *mut Base = (&mut derived as *mut Derived).upcast();
        }
        // let _: &mut Base = (&mut derived).upcast(); // does not compile
        // let _: Pin<&mut Base> = (&mut derived).upcast(); // does not compile
        // let _: Pin<&mut Base> = Pin::new(&mut derived).upcast(); // does not compile

        // This write must not be UB:
        // SAFETY: `derived` is a valid pointer to a `Derived` value. `base` is a
        // valid pointer to a `Base` value.
        unsafe {
            let base: *mut Base = (&mut derived as *mut Derived).upcast();
            (*base).0 = 42;
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
                // SAFETY: `derived` is a valid pointer to a `Derived` value.
                unsafe { &(*derived).base }
            }
        }
        let mut derived = Derived::default();
        // SAFETY: `derived` is a valid pointer to a `Derived` value.
        assert_eq!(ptr_location(&derived.base), unsafe {
            ptr_location::<&Base>((&derived).upcast())
        });

        let _: *const Base = unsafe { Derived::upcast_ptr(&derived) };
        let _: *mut Base = unsafe { Derived::upcast_ptr_mut(&mut derived) };
        // let _: &mut Base = (&mut derived).upcast(); // does not compile
        // let _: Pin<&mut Base> = (&mut derived).upcast(); // does not compile
        // let _: Pin<&mut Base> = Pin::new(&mut derived).upcast(); // does not compile

        // This write must not be UB:
        {
            let base: *mut Base = unsafe { Derived::upcast_ptr_mut(&mut derived) };
            // SAFETY: `base` is a valid pointer to a `Base` value.
            unsafe { (*base).0 = 42 };
        }
        assert_eq!(derived.base.0, 42);
    }
}
