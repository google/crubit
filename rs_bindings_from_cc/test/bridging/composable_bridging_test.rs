// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use composable_bridging_lib::*;
use googletest::prelude::*;

#[gtest]
fn test_string_view() {
    let s: MyStringView = MakeHello();
    // SAFETY: The C++ function returns a string_view to a static string.
    let bytes: &[u8] = unsafe { &*s.view.as_raw_bytes() };

    expect_eq!(bytes, b"Hello");
    let does_say_hello = SaysHello(s);
    expect_eq!(does_say_hello, true);
}

#[gtest]
fn test_vec3() {
    expect_eq!(MakeVec3(1.0, 2.0, 3.0), Vec3 { x: 1.0, y: 2.0, z: 3.0 });
}

#[gtest]
fn test_pair() {
    expect_eq!(MakePair(1, 2.0, true), ((1, 2.0), true));
}

#[gtest]
fn test_optional() {
    expect_eq!(MakeOptionalVec3(1.0, 2.0, 3.0, true), Some(Vec3 { x: 1.0, y: 2.0, z: 3.0 }));
    expect_eq!(MakeOptionalVec3(1.0, 2.0, 3.0, false), None);
}

#[gtest]
fn test_map_multiply() {
    expect_eq!(
        MapMultiply(Some(Vec3 { x: 1.0, y: 2.0, z: 3.0 }), 2.0),
        Some(Vec3 { x: 2.0, y: 4.0, z: 6.0 })
    );
    expect_eq!(MapMultiply(None, 2.0), None);
}

#[gtest]
fn test_stuff() {
    expect_eq!(MakeStuff(), (None, Some((3.14, Vec3 { x: 1.0, y: 2.0, z: 3.0 }))));
}
