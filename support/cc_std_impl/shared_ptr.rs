// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

extern crate alloc;

use crate::crubit_cc_std_internal::std_allocator::{
    self, shared_weak_count, DynControlBlock, FunctionToCall,
};
use alloc::boxed::Box;
use alloc::sync::Arc;
use core::ffi::c_void;
use core::mem::{ManuallyDrop, MaybeUninit};
use core::ops::Deref;
use core::pin::Pin;

/// A smart pointer that shares ownership of another object of type `T` via a pointer,
/// ABI-compatible with `std::shared_ptr<T>`.
///
/// # Thread Safety
///
/// Like `Arc<T>`, `shared_ptr<T>` implements [`Send`] and [`Sync`] only if `T` implements both
/// [`Send`] and [`Sync`], and only offers access to the underlying `T` via shared references.
///
/// # Mutation
///
/// `shared_ptr<T>` only hands out shared references (`&T`), which are only valid to mutate when the
/// mutated data is wrapped in [`UnsafeCell`](std::cell::UnsafeCell). This has two consequences for
/// users of `shared_ptr<T>` in Rust:
///
/// 1. Thread-safe C++ types `T` that manage their own internal synchronization should be marked
///    with `CRUBIT_THREAD_SAFE` so their methods can be called from Rust within `shared_ptr<T>`.
///    Internally, this annotation wraps all fields in `UnsafeCell`, making them not only safe to
///    mutate, but also safe to hold a shared reference to (`&T`) while being mutated by someone
///    else.
///
/// 2. Non-thread-safe C++ types `T` can still be mutated safely from C++ as long as synchronization
///    is handled externally, but due to Rust's aliasing rules, it is undefined behavior for a Rust
///    user to hold a `&T` during the mutation (modulo anything within `UnsafeCell`, of course).
///    If the Rust user expects externally synchronized mutation from C++, they must also
///    participate in the synchronization to ensure that they never hold an active `&T` during the
///    C++ mutation window, otherwise they are subject to undefined behavior.
///
/// See crubit.rs/cpp/cookbook#thread_safety for more details.
#[crubit_annotate::cpp_layout_equivalent(
    cpp_type = "::std::shared_ptr<{T}>",
    include_path = "<memory>"
)]
#[allow(non_snake_case)]
#[repr(C)]
pub struct shared_ptr<T: Sized> {
    // Safety: `ptr` and `cntrl` come from a valid `std::shared_ptr<T>`
    ptr: *const T,
    // Safety: `cntrl` is a nullable pointer to a valid  `std::__shared_weak_count`
    cntrl: *mut shared_weak_count,
}

// SAFETY: shared_ptr is Send and Sync if T is Send and Sync. Anything less is too restrictive
// since a &shared_ptr<T> can be cloned by another thread, effectively moving the `T` to the
// other thread.
unsafe impl<T: Sized + Send + Sync> Send for shared_ptr<T> {}
unsafe impl<T: Sized + Send + Sync> Sync for shared_ptr<T> {}

impl<T: Sized> shared_ptr<T> {
    /// Creates a new `shared_ptr` managing `value`.
    ///
    /// This function performs a single heap allocation (similar to `std::make_shared`),
    /// co-allocating the control block and the `value`.
    pub fn new(value: T) -> Self {
        let mut inner = Box::leak(Box::new(SharedInner {
            cntrl: MaybeUninit::uninit(),
            value: ManuallyDrop::new(value),
        }));

        // SAFETY: `this.cntrl.as_mut_ptr()` is a valid location to write a `DynControlBlock` to,
        // and `SharedInner::<T>::deleter` satisfies the C++ deleter callback preconditions.
        let cntrl = unsafe {
            DynControlBlock::Emplace(inner.cntrl.as_mut_ptr(), Some(SharedInner::<T>::deleter))
        };

        shared_ptr { ptr: &*inner.value, cntrl }
    }

    /// Constructs a `shared_ptr<U>` by projecting the underlying `T` via `f`.
    ///
    /// This is functionally equivalent to C++'s `std::shared_ptr` aliasing constructor: the
    /// returned `shared_ptr<U>` points to the target `&U` returned by `f`, while sharing ownership
    /// of the original control block. This ensures that `T` (and any allocations it owns) remains
    /// alive in memory for as long as any reference to the projected `shared_ptr<U>` exists.
    ///
    /// Because the `T` bound is lost in the return type, this function requires `T: 'static` to
    /// ensure that holding onto the returned `shared_ptr<U>` does not extend the lifetime of the
    /// underlying `T` longer than it is valid to do so.
    pub fn project<U>(this: Self, f: impl for<'a> FnOnce(&'a T) -> &'a U) -> shared_ptr<U>
    where
        T: 'static,
    {
        // SAFETY: `T: 'static` guarantees that `T` has no lifetimes for `f` to project away.
        unsafe { shared_ptr::project_unchecked(this, f) }
    }

    /// Constructs a `shared_ptr<U>` by projecting the underlying `T` via `f`.
    ///
    /// This function is identical to `shared_ptr::project`, except it is unsafe because it does not
    /// require `T: 'static`.
    ///
    /// # Safety
    ///
    /// The returned `shared_ptr<U>` must ensure that the underlying `T` is still valid by the time
    /// the `shared_ptr<U>` is dropped. Specifically, this function is safe to call if `U` contains
    /// all the lifetimes that appear in `T`, or if `T` is `'static` (has no lifetimes).
    ///
    /// Here's an example of how improper usage can lead to undefined behavior:
    ///
    /// ```rust
    /// struct MutateOnDrop<'a>(&'a mut i32);
    /// impl<'a> Drop for MutateOnDrop<'a> {
    ///     fn drop(&mut self) {
    ///         *self.0 += 1;
    ///     }
    /// }
    /// fn create_bad_ptr() -> shared_ptr<i32> {
    ///     let mut stack_var = 0;
    ///     let sp: shared_ptr<MutateOnDrop<'_>> = shared_ptr::new(MutateOnDrop(&mut stack_var));
    ///     // 💥 Erases the lifetime of `'_` by projecting to `i32`!
    ///     let projected: shared_ptr<i32> = unsafe { shared_ptr::project_unchecked(sp, |x| &*x.0) }
    ///     // `stack_var` is destroyed when this stack frame pops.
    ///     projected
    /// }
    /// fn main() {
    ///     let sp = create_bad_ptr();
    ///     // When `sp` is dropped, `MutateOnDrop::drop` runs and writes to `stack_var`
    ///     // on a popped stack frame -> 💥 STACK USE-AFTER-RETURN / MEMORY CORRUPTION!
    ///     drop(sp);
    /// }
    /// ```
    pub unsafe fn project_unchecked<U>(
        this: Self,
        f: impl for<'a> FnOnce(&'a T) -> &'a U,
    ) -> shared_ptr<U> {
        let (ptr, cntrl) = shared_ptr::into_raw_parts(this);

        // SAFETY: `ptr` is either null or points to a live `T` owned by `cntrl`.
        // If non-null, `f` is called with `&T`, returning a reference `&U` that borrows
        // from `T`. Because `T: 'static`, `U` will remain at this memory address for as
        // long as `cntrl` keeps `T` alive.
        let ptr = unsafe { ptr.as_ref() }.map_or(core::ptr::null(), |r| f(r) as *const U);
        shared_ptr { ptr, cntrl }
    }

    /// Creates a new `shared_ptr` taking ownership of an existing `Box<T>`.
    ///
    /// Note that this still requires an additional allocation for the control block, but reuses
    /// the existing `Box` allocation so the value does not have to be moved.
    pub fn from_box(b: Box<T>) -> Self {
        // SAFETY: The projection `&Box<T>` -> `&T` doesn't erase any lifetimes, meaning that
        // whenever the returned `shared_ptr<T>` is dropped, the underlying `Box<T>` that it points
        // into will still be valid as well.
        unsafe { shared_ptr::project_unchecked(shared_ptr::new(b), |b| &**b) }
    }

    /// Creates a new `shared_ptr` taking ownership of an existing `Pin<Box<T>>`.
    ///
    /// Note that this still requires an additional allocation for the control block, but reuses
    /// the existing `Box` allocation so the value does not have to be moved.
    pub fn from_pinned_box(b: Pin<Box<T>>) -> Self {
        // SAFETY: The projection `&Pin<Box<T>>` -> `&T` doesn't erase any lifetimes, meaning that
        // whenever the returned `shared_ptr<T>` is dropped, the underlying `Pin<Box<T>>` that it
        // points into will still be valid as well.
        unsafe { shared_ptr::project_unchecked(shared_ptr::new(b), |b| &**b) }
    }

    /// Creates a new `shared_ptr` taking ownership of an existing `Arc<T>`.
    ///
    /// Note that this still requires an additional allocation for the control block, but reuses
    /// the existing `Arc` allocation so the value does not have to be moved.
    ///
    /// Also note that the returned `shared_ptr<T>` and all clones of it will only count towards
    /// one `Arc::strong_count`.
    pub fn from_arc(arc: Arc<T>) -> Self {
        // SAFETY: The projection `&Arc<T>` -> `&T` doesn't erase any lifetimes, meaning that
        // whenever the returned `shared_ptr<T>` is dropped, the underlying `Arc<T>` that it points
        // into will still be valid as well.
        unsafe { shared_ptr::project_unchecked(shared_ptr::new(arc), |a| &**a) }
    }

    /// Creates a new `shared_ptr` from a raw pointer and a custom deleter.
    ///
    /// When the number of strong owners reaches zero, the deleter will be called with the raw
    /// pointer.
    ///
    /// # Safety
    ///
    /// ## Preconditions
    /// - `ptr` must be a valid, aligned pointer to an initialized `T`, or null.
    /// - It must be safe to call `deleter` on `ptr` when the last strong owner is dropped.
    ///
    /// ## Postconditions
    /// - The caller can be assured that the deleter will be called exactly once when the last
    ///   strong owner is dropped.
    pub unsafe fn from_ptr_and_deleter(
        ptr: *mut T,
        deleter: impl FnOnce(*mut T) + Send + 'static,
    ) -> Self {
        struct DeleteOnDrop<T, F: FnOnce(*mut T) + Send + 'static> {
            ptr: *mut T,
            // Safety: `deleter` is always initialized, and is only wrapped in `MaybeUninit` so
            // it can be taken and consumed during `Drop::drop`.
            deleter: MaybeUninit<F>,
        }

        impl<T, F: FnOnce(*mut T) + Send + 'static> Drop for DeleteOnDrop<T, F> {
            fn drop(&mut self) {
                // SAFETY: `deleter` is always initialized, and is only wrapped in `MaybeUninit` so
                // it can be taken and consumed during `Drop::drop` (right now). Invoking it also
                // runs its drop implementation, so nothing is leaked.
                let deleter = unsafe { self.deleter.assume_init_read() };
                deleter(self.ptr);
            }
        }

        let delete_on_drop =
            shared_ptr::new(DeleteOnDrop { ptr, deleter: MaybeUninit::new(deleter) });

        // SAFETY: We only project away the deleter FnOnce object, which we guaranteed was 'static.
        unsafe { shared_ptr::project_unchecked(delete_on_drop, |d| &*d.ptr) }
    }

    /// Creates a `shared_ptr` from a raw pointer and a control block pointer.
    ///
    /// # Safety
    ///
    /// ## Preconditions
    /// - `ptr` must be a valid, aligned pointer to an initialized `T`, or null.
    /// - `cntrl` must be a valid, nullable pointer to a `std::__shared_weak_count` control block
    ///   managing `ptr` (or null if `ptr` is null).
    /// - `(ptr, cntrl)` must constitute a valid `std::shared_ptr<T>` state.
    /// - The strong reference count of `cntrl` must already account for the returned `shared_ptr`;
    ///   this function transfers ownership of an existing strong reference without incrementing it.
    ///
    /// ## Postconditions
    /// - Returns a `shared_ptr<T>` owning the transferred strong reference, which will decrement
    ///   the reference count when dropped.
    pub unsafe fn from_raw_parts(ptr: *const T, cntrl: *mut shared_weak_count) -> Self {
        shared_ptr { ptr, cntrl }
    }

    /// Returns `true` if `self` is a null pointer.
    pub fn is_null(&self) -> bool {
        self.ptr.is_null()
    }

    /// Returns a raw pointer to the contents.
    pub fn as_ptr(&self) -> *const T {
        self.ptr
    }

    /// Releases the ownership of the object and control block pointed to by `self` without
    /// decrementing the reference count, replacing this `shared_ptr` with a null pointer.
    ///
    /// Where possible, prefer `into_raw_parts` in order to avoid null `shared_ptr`s.
    pub fn release(&mut self) -> (*const T, *mut shared_weak_count) {
        (
            core::mem::replace(&mut self.ptr, core::ptr::null()),
            core::mem::replace(&mut self.cntrl, core::ptr::null_mut()),
        )
    }

    /// Consumes the `shared_ptr` without decrementing the reference count, returning the
    /// owned raw pointer and control block pointer.
    pub fn into_raw_parts(mut self) -> (*const T, *mut shared_weak_count) {
        self.release()
    }

    /// Returns a reference to the underlying object, if it exists.
    ///
    /// # Safety
    ///
    /// If the inner object not thread safe and is mutated by C++ in a nonlocally-synchronized way,
    /// it's unsafe to hold references to `&T` across the mutations (unless the mutated data is
    /// wrapped in `UnsafeCell`).
    pub fn try_as_ref(&self) -> Option<&T> {
        // SAFETY: By `shared_ptr` invariants, `self.ptr` is either null or points to an
        // initialized `T` whose lifetime is managed by `self`.
        unsafe { self.ptr.as_ref() }
    }

    /// Returns the number of `shared_ptr` instances managing the current object.
    #[must_use]
    pub fn use_count(&self) -> usize {
        // SAFETY: `self.cntrl` is a nullable pointer to a valid `std::__shared_weak_count`.
        unsafe { std_allocator::shared_ptr_use_count(self.cntrl) }
    }

    /// Returns `true` if the two `shared_ptr`s manage the same control block.
    ///
    /// Note that because of projection, it's possible for two `shared_ptr`s to point to different
    /// parts of the same underlying object, thus having different `T` types, but still share the
    /// same control block.
    #[must_use]
    pub fn owner_equal<U: Sized>(&self, other: &shared_ptr<U>) -> bool {
        core::ptr::addr_eq(self.cntrl, other.cntrl)
    }
}

impl<T: Sized> Deref for shared_ptr<T> {
    type Target = T;

    #[track_caller]
    fn deref(&self) -> &Self::Target {
        assert!(!self.ptr.is_null(), "dereferencing a null shared_ptr");
        // SAFETY: By `shared_ptr` invariants, `self.ptr` is either null or points to an
        // initialized `T` whose lifetime is managed by `self`. We asserted that it is not null.
        unsafe { &*self.ptr }
    }
}

impl<T: Sized> Clone for shared_ptr<T> {
    fn clone(&self) -> Self {
        // SAFETY: `self.cntrl` is a nullable pointer to a valid `std::__shared_weak_count`.
        // Incrementing the strong reference count via `shared_ptr_ref` keeps the object alive
        // for the new clone.
        unsafe {
            std_allocator::shared_ptr_ref(self.cntrl);
        }
        shared_ptr { ptr: self.ptr, cntrl: self.cntrl }
    }
}

impl<T: Sized> Drop for shared_ptr<T> {
    fn drop(&mut self) {
        // SAFETY: `self.cntrl` is a nullable pointer to a valid `std::__shared_weak_count`.
        // Decrementing the strong reference count via `shared_ptr_unref` releases this instance's
        // ownership and triggers destruction if the strong count reaches zero.
        unsafe {
            std_allocator::shared_ptr_unref(self.cntrl);
        }
    }
}

/// A control block for `shared_ptr` that embeds a `DynControlBlock` and a `T` value.
///
/// # Layout
///
/// `SharedInner<T>` is `#[repr(C)]` with `MaybeUninit<DynControlBlock>` as its
/// first field. This means that pointers to this type can be freely upcasted to
/// `*mut DynControlBlock`, and (when safe to do so) downcasted back to
/// `*mut SharedInner<T>`.
#[repr(C)]
struct SharedInner<T> {
    /// The C++ control block.
    ///
    /// It is safety critical that this remains the first field, and that this type is repr(C).
    cntrl: MaybeUninit<DynControlBlock>,

    /// The value.
    ///
    /// The value is dropped in place when the strong refcount hits 0, which may happen before this
    /// object is deallocated (due to lingering weak pointers).
    value: ManuallyDrop<T>,
}

impl<T> SharedInner<T> {
    /// Deletes the `value` or destructs and deallocates the entire `SharedInner<T>`.
    ///
    /// This function is only intended to be used by `DynControlBlock`s embedded in
    /// `SharedInner<T>`.
    ///
    /// # Safety
    ///
    /// ## Preconditions
    /// - `cntrl` must point to a live `DynControlBlock` embedded as the first field of an
    ///   active `SharedInner<T>` allocated via `SharedInner::new`.
    /// - `function_to_call` must correspond to the exact lifecycle transition:
    ///   - `FunctionToCall::kDestroyValue`: called exactly once when the strong reference count
    ///     reaches 0, while `value` is still initialized and live.
    ///   - `FunctionToCall::kDeleteControlBlock`: called exactly once when all strong and weak
    ///     reference counts reach 0, after `kDestroyValue` has already run.
    ///
    /// ## Postconditions
    /// - If `kDestroyValue`: `value` is dropped in place; the control block and
    ///   containing allocation remain live.
    /// - If `kDeleteControlBlock`: `cntrl` is dropped in place and the entire
    ///   `SharedInner<T>` allocation is freed via `Box::from_raw`.
    unsafe extern "C" fn deleter(function_to_call: FunctionToCall, cntrl: *mut DynControlBlock) {
        // `SharedInner<T>` is `#[repr(C)]` with `MaybeUninit<DynControlBlock>` as
        // its first field at byte offset 0. Since `cntrl` was allocated as part of a
        // `SharedInner<T>`, upcasting `cntrl as *mut Self` safely recovers the
        // pointer to the containing `SharedInner<T>`.
        let this = cntrl as *mut Self;

        match function_to_call {
            FunctionToCall::kDestroyValue => {
                // SAFETY: Preconditions guarantee this is called exactly once when the strong
                // reference count is zero, and that `value` contains a live, undropped `T`.
                unsafe { core::ptr::drop_in_place(&raw mut (*this).value as *mut T) };
            }
            FunctionToCall::kDeleteControlBlock => {
                // SAFETY: Preconditions guarantee this is called exactly once when the weak
                // reference count hits zero, and that `this` points to a valid
                // `SharedInner<T>` that was previously leaked from a `Box`.
                let mut this = unsafe { Box::from_raw(this) };

                // SAFETY: Preconditions guarantee `cntrl` was initialized with placement-new in
                // `DynControlBlock::Emplace` and is valid to drop in place.
                unsafe { this.cntrl.assume_init_drop() };

                // `this` is dropped here, deallocating the `SharedInner<T>`.
            }
        }
    }
}
