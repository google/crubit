// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use atomic_lib::*;
use googletest::prelude::*;
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicPtr, AtomicU64, Ordering};

#[gtest]
fn test_atomic_u64() -> googletest::Result<()> {
    let mut val = AtomicU64::new(0);

    // SAFETY: `val` is a valid, stack-allocated `AtomicU64`. Passing `&mut val` across FFI satisfies C++ alignment and non-null pointer requirements.
    unsafe {
        atomic_lib::inc_atomic(&mut val);
    }

    verify_that!(val.load(Ordering::Relaxed), eq(1))?;

    // Modify from Rust side
    val.store(5, Ordering::Relaxed);

    // SAFETY: `val` remains valid on the stack. Passing `&mut val` across FFI is safe as C++ simply performs an atomic load.
    let loaded = unsafe { atomic_lib::load_atomic(&mut val) };

    verify_that!(loaded, eq(5))
}

#[gtest]
fn test_atomic_bool() -> googletest::Result<()> {
    let mut val = AtomicBool::new(false);
    // SAFETY: `val` is a valid, stack-allocated `AtomicBool`. The C++ side safely performs an atomic exchange on the underlying pointer.
    let old = unsafe { atomic_lib::exchange_atomic_bool(&mut val, true) };
    verify_that!(old, eq(false))?;
    verify_that!(val.load(Ordering::SeqCst), eq(true))
}

#[gtest]
fn test_atomic_int() -> googletest::Result<()> {
    let mut val = AtomicI32::new(10);
    // SAFETY: `val` is a valid, stack-allocated `AtomicI32`. Passing `&mut val` across FFI is safe for C++ atomic fetch and add operations.
    let old = unsafe { atomic_lib::fetch_add_atomic_int(&mut val, 5) };
    verify_that!(old, eq(10))?;
    verify_that!(val.load(Ordering::SeqCst), eq(15))
}

#[gtest]
fn test_atomic_ptr() -> googletest::Result<()> {
    let mut x: i32 = 42;
    let mut y: i32 = 99;
    let mut val = AtomicPtr::new(&mut x);
    // SAFETY: `val` is a valid `AtomicPtr` pointing to `x`, and `y` is a valid stack variable. Both outlive the FFI call, ensuring C++ operates on valid, aligned pointers without dangling references.
    let old = unsafe { atomic_lib::exchange_atomic_ptr(&mut val, &mut y) };
    verify_that!(old, eq(&mut x as *mut i32))?;
    verify_that!(val.load(Ordering::SeqCst), eq(&mut y as *mut i32))
}

#[gtest]
fn test_edge_cases_compilation() -> googletest::Result<()> {
    // Simply instantiating or referencing the container type proves the generator
    // successfully processed atomic enums, floats, and structs without crashing.
    verify_that!(std::mem::size_of::<atomic_lib::EdgeCasesContainer>() > 0, eq(true))
}
