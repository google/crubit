// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use item_exists::{type_exists, value_exists};
use no_bindings::crubit::no_bindings;

#[test]
fn test_non_extern_c() {
    assert!(!value_exists!(no_bindings::crubit_non_extern_c));
}

#[test]
fn test_nontrivial_type() {
    assert!(!type_exists!(no_bindings::Nontrivial));
}

#[test]
fn test_accepts_nontrivial_ptr() {
    assert!(!value_exists!(no_bindings::crubit_accepts_nontrivial_ptr));
}

#[test]
fn test_accepts_nontrivial_value() {
    assert!(!value_exists!(no_bindings::crubit_accepts_nontrivial_value));
}

#[test]
fn test_returns_nontrivial_ptr() {
    assert!(!value_exists!(no_bindings::crubit_returns_nontrivial_ptr));
}

#[test]
fn test_returns_nontrivial_value() {
    assert!(!value_exists!(no_bindings::crubit_returns_nontrivial_value));
}

// vectorcall attribute is outright ignored on e.g. ARM -- so on that platform,
// this isn't actually a different calling convention, and we'd expect bindings
// to exist after all.
#[cfg(target_arch = "x86_64")]
#[test]
fn test_vectorcall() {
    assert!(!value_exists!(no_bindings::crubit_vectorcall));
}

#[test]
fn test_noreturn() {
    assert!(!value_exists!(no_bindings::crubit_noreturn));
}
