// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use crubit_support::Unsafe;
use googletest::{expect_eq, gtest};
use static_assertions::{assert_impl_all, assert_not_impl_any};
use std::cell::Cell;
use std::fmt;
use std::marker::PhantomPinned;
use std::mem::{align_of, size_of};
use std::rc::Rc;

struct DropCounter<'a>(&'a Cell<usize>);

impl Drop for DropCounter<'_> {
    fn drop(&mut self) {
        self.0.set(self.0.get() + 1);
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct ReprCPair {
    a: u32,
    b: u64,
}

// `Send`, `Sync` and `Unpin` follow `T`.
assert_impl_all!(Unsafe<i32>: Send, Sync, Unpin);
assert_impl_all!(Unsafe<Cell<i32>>: Send);
assert_not_impl_any!(Unsafe<Cell<i32>>: Sync);
assert_not_impl_any!(Unsafe<Rc<i32>>: Send, Sync);
assert_not_impl_any!(Unsafe<PhantomPinned>: Unpin);

// `Copy` and `Clone` only for `Copy` types.
assert_impl_all!(Unsafe<ReprCPair>: Copy, Clone);
assert_not_impl_any!(Unsafe<String>: Copy, Clone);

#[gtest]
fn test_layout_matches_inner() {
    expect_eq!(size_of::<Unsafe<()>>(), size_of::<()>());
    expect_eq!(align_of::<Unsafe<()>>(), align_of::<()>());
    expect_eq!(size_of::<Unsafe<u8>>(), size_of::<u8>());
    expect_eq!(align_of::<Unsafe<u8>>(), align_of::<u8>());
    expect_eq!(size_of::<Unsafe<ReprCPair>>(), size_of::<ReprCPair>());
    expect_eq!(align_of::<Unsafe<ReprCPair>>(), align_of::<ReprCPair>());
    expect_eq!(size_of::<Unsafe<&'static i32>>(), size_of::<&'static i32>());
    expect_eq!(align_of::<Unsafe<&'static i32>>(), align_of::<&'static i32>());
    // The niche of `T` is kept, as for `T` itself.
    expect_eq!(size_of::<Option<Unsafe<&'static i32>>>(), size_of::<&'static i32>());
}

#[gtest]
fn test_accessors_and_into_inner() {
    let mut wrapped = Unsafe::new(10i32);
    // SAFETY: `i32` has no lifetime parameters.
    unsafe {
        expect_eq!(*wrapped.as_ref(), 10);
        *wrapped.as_mut() = 42;
        expect_eq!(*wrapped.as_ref(), 42);
        expect_eq!(wrapped.into_inner(), 42);
    }
}

#[gtest]
fn test_drop_runs_destructor_once() {
    let drops = Cell::new(0usize);

    let wrapped = Unsafe::new(DropCounter(&drops));
    expect_eq!(drops.get(), 0);
    drop(wrapped);
    // Dropping `Unsafe<T>` drops the `T` it holds.
    expect_eq!(drops.get(), 1);

    let wrapped = Unsafe::new(DropCounter(&drops));
    // SAFETY: `drops` is live until the end of the test.
    let inner = unsafe { wrapped.into_inner() };
    // Unwrapping moves the value out without dropping it.
    expect_eq!(drops.get(), 1);
    drop(inner);
    expect_eq!(drops.get(), 2);

    let wrapped = Unsafe::new(DropCounter(&drops));
    std::mem::forget(wrapped);
    // `forget` is the way to not drop it.
    expect_eq!(drops.get(), 2);
}

#[gtest]
fn test_copy_and_clone() {
    let a = Unsafe::new(ReprCPair { a: 1, b: 2 });
    let b = a;
    #[allow(clippy::clone_on_copy)]
    let c = a.clone();
    // SAFETY: `ReprCPair` has no lifetime parameters.
    unsafe {
        expect_eq!(a.into_inner(), ReprCPair { a: 1, b: 2 });
        expect_eq!(b.into_inner(), ReprCPair { a: 1, b: 2 });
        expect_eq!(c.into_inner(), ReprCPair { a: 1, b: 2 });
    }
}

#[gtest]
fn test_debug_does_not_read_inner() {
    struct PanickingDebug;
    impl fmt::Debug for PanickingDebug {
        fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
            panic!("Unsafe::fmt must not delegate to T::fmt");
        }
    }

    let wrapped = Unsafe::new(PanickingDebug);
    expect_eq!(format!("{wrapped:?}"), "Unsafe(..)");
}

#[gtest]
fn test_ffi_safe() {
    // Compile-time check that `Unsafe<T>` passes `improper_ctypes` for FFI-safe `T`.
    #[deny(improper_ctypes)]
    unsafe extern "C" {
        #[allow(dead_code)]
        fn _crubit_unsafe_ffi_check(x: Unsafe<ReprCPair>) -> Unsafe<ReprCPair>;
    }
}
