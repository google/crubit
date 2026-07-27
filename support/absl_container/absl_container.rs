// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Helper types for the Rust projections of C++ `absl::flat_hash_map` template instantiations.

use std::fmt::{self, Debug, Display, Formatter};

/// The error returned by `try_insert` when the key already exists.
#[derive(Debug)]
pub struct OccupiedError<'a, K, V> {
    pub element: (&'a K, &'a mut V),
    pub key: K,
    pub value: V,
}

impl<'a, K, V> Display for OccupiedError<'a, K, V>
where
    K: Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let Self { element, key, value } = self;
        write!(f, "insert of ({key:?}, {value:?}) was prevented by element {element:?}")
    }
}

impl<'a, K, V> std::error::Error for OccupiedError<'a, K, V>
where
    K: Debug,
    V: Debug,
{
}

/// The error returned by `try_insert_mov` when the key already exists.
#[derive(Debug)]
pub struct OccupiedMovError<E> {
    pub element: E,
}

impl<E> Display for OccupiedMovError<E>
where
    E: Debug,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let Self { element } = self;
        write!(f, "insert was prevented by element {element:?}")
    }
}

impl<E> std::error::Error for OccupiedMovError<E> where E: Debug {}
