// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[derive(Copy, Clone)]
pub enum Color {
    /// A completely transparent color (no payload)
    Transparent,
    /// A grayscale value from 0 to 255
    Grayscale(u8),
    /// Red, Green, and Blue values from 0 to 255
    Rgb(u8, u8, u8),
}
