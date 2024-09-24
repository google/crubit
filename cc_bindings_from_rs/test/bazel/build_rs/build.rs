// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! This file is based on
//! https://doc.rust-lang.org/cargo/reference/build-script-examples.html#code-generation
//!
//! See the top-level comment in BUILD for a high-level description and
//! motivation of the test.

use std::env;
use std::fs;
use std::path::Path;

/// Generate "${OUT_DIR}/include_me.rs" with a public `add_two_integers`
/// function.
fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("include_me.rs");
    fs::write(
        dest_path,
        r#"
            pub fn add_two_integers(x: i32, y: i32) -> i32 {
                x + y
            }
        "#,
    )
    .unwrap();
    println!("cargo:rerun-if-changed=build.rs");

    println!("cargo:rustc-cfg=feature=\"cfg_set_by_build_rs\"");
}
