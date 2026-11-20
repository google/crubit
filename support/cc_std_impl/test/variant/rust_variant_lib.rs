// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use cpp_std::{string, trivial_variant, variant};

#[unsafe(no_mangle)]
pub extern "C" fn get_trivial_variant() -> trivial_variant![i32, f32, bool] {
    trivial_variant::new_at::<1>(3.5)
}

#[unsafe(no_mangle)]
pub extern "C" fn check_trivial_variant(v: trivial_variant![i32, f32, bool]) -> bool {
    v.holds_alternative::<f32, _>() && *v.get::<f32, _>().unwrap() == 3.5
}

#[unsafe(no_mangle)]
pub extern "C" fn get_small_variant() -> trivial_variant![u8, bool] {
    trivial_variant::new_at::<1>(true)
}

#[unsafe(no_mangle)]
pub extern "C" fn check_small_variant(v: trivial_variant![u8, bool]) -> bool {
    v.holds_alternative::<bool, _>() && *v.get::<bool, _>().unwrap()
}

// We pass by reference for non-trivial to avoid move constructibility issues for now.
#[unsafe(no_mangle)]
pub extern "C" fn check_non_trivial_variant(v: &variant![i32, string]) -> bool {
    v.holds_alternative::<string, _>() && v.get::<string, _>().unwrap().as_slice() == b"hello"
}

#[unsafe(no_mangle)]
pub extern "C" fn get_seventeen_variant() -> trivial_variant![
    u8,
    u16,
    u32,
    u64,
    usize,
    i8,
    i16,
    i32,
    i64,
    isize,
    f32,
    f64,
    bool,
    char,
    *const u8,
    *mut u8,
    i32
] {
    trivial_variant::new_at::<16>(42)
}

#[unsafe(no_mangle)]
pub extern "C" fn check_seventeen_variant(
    v: trivial_variant![
        u8,
        u16,
        u32,
        u64,
        usize,
        i8,
        i16,
        i32,
        i64,
        isize,
        f32,
        f64,
        bool,
        char,
        *const u8,
        *mut u8,
        i32
    ],
) -> bool {
    v.holds_alternative_at::<16>() && *v.get_at::<16>().unwrap() == 42
}
