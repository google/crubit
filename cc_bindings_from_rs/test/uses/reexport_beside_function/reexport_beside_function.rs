// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
extern crate middle_crate;

// `Braced` comes from a crate whose bindings this crate cannot refer to, and this crate has the
// shortest path to it, so this crate's bindings define the struct. A definition keeps its name
// beside the function of the same name (tested as a golden only, since the generated layout
// assertions do not yet use an elaborated `struct` specifier when a function shares the name).
pub use middle_crate::inner::Braced;

#[allow(non_snake_case)]
pub fn Braced() -> i32 {
    1
}

// `middle_crate` defines `Local` in its own bindings, so here it is an alias, and an alias makes
// way for the function of the same name.
pub use middle_crate::Local;

#[allow(non_snake_case)]
pub fn Local() -> i32 {
    2
}
