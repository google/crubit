// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Repro for `Option<&T>` and `Option<*const T>` colliding in C++.
//!
//! `Option<&Value>` is a template argument, so the reference is formatted as a
//! pointer (`Value const* $static crubit_nonnull`). `Option<*const Value>` is
//! formatted as `Value const*`. Lifetime and nullability annotations do not
//! change the C++ type, so both specializations are
//! `rs_std::Option<Value const*>`, which is a redefinition.
//!
//! The two Rust types also have different layouts (`Option<&T>` uses the null
//! niche, `Option<*const T>` needs a separate tag), so they cannot simply be
//! deduplicated into one specialization.

#[derive(Clone, Copy, Default)]
pub struct Value {
    pub x: i32,
}

/// Produces `Option<&Value>`.
pub fn by_ref(v: &Value) -> Option<&Value> {
    Some(v)
}

/// Produces `Option<*const Value>`.
pub fn by_ptr(v: &Value) -> Option<*const Value> {
    Some(v as *const Value)
}
