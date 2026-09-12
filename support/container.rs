// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Helper types for the Rust projections of C++ container types.

use std::fmt::{self, Debug, Display, Formatter};

/// The error returned by `try_insert` when the key already exists.
#[derive(Debug)]
pub struct OccupiedError<E, K, V> {
    pub element: E,
    pub key: K,
    pub value: V,
}
