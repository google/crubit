// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! This crate is used as a test input for `cc_bindings_from_rs` and the
//! generated C++ bindings are then tested via `index_test.cc`.

use std::ops::{Index, IndexMut};

pub struct IntPair {
    pub x: i32,
    pub y: i32,
}

impl IntPair {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

// Index by primitive
impl Index<usize> for IntPair {
    type Output = i32;

    fn index(&self, index: usize) -> &Self::Output {
        if index == 0 {
            &self.x
        } else {
            &self.y
        }
    }
}

impl IndexMut<usize> for IntPair {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index == 0 {
            &mut self.x
        } else {
            &mut self.y
        }
    }
}

impl Index<u64> for IntPair {
    type Output = i32;

    fn index(&self, index: u64) -> &Self::Output {
        if index == 0 {
            &self.x
        } else {
            &self.y
        }
    }
}

impl IndexMut<u64> for IntPair {
    fn index_mut(&mut self, index: u64) -> &mut Self::Output {
        if index == 0 {
            &mut self.x
        } else {
            &mut self.y
        }
    }
}

#[derive(Clone, Copy)]
pub struct CustomIndex(pub usize);
impl CustomIndex {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
}

// Index by custom bridgeable type
impl Index<CustomIndex> for IntPair {
    type Output = i32;

    fn index(&self, index: CustomIndex) -> &Self::Output {
        if index.0 == 0 {
            &self.x
        } else {
            &self.y
        }
    }
}

impl IndexMut<CustomIndex> for IntPair {
    fn index_mut(&mut self, index: CustomIndex) -> &mut Self::Output {
        if index.0 == 0 {
            &mut self.x
        } else {
            &mut self.y
        }
    }
}

pub struct Map {
    row_size: usize,
    data: Vec<String>,
}

impl Map {
    pub fn new(row_size: usize, col_size: usize) -> Self {
        Self {
            row_size,
            data: (0..row_size)
                .flat_map(|row| (0..col_size).map(move |col| format!("tile({}, {})", row, col)))
                .collect(),
        }
    }
}

impl Index<(usize, usize)> for Map {
    type Output = str;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.0 * self.row_size + index.1]
    }
}

impl IndexMut<(usize, usize)> for Map {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.data[index.0 * self.row_size + index.1]
    }
}

pub struct Id(pub i32);
impl Id {
    pub fn new(id: i32) -> Self {
        Self(id)
    }
}

impl Index<&Id> for Map {
    type Output = str;

    fn index(&self, _index: &Id) -> &Self::Output {
        &self.data[0]
    }
}
