// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//TODO(rosica): We need namespaces in order to be able to test more here.

use cc_std::*;
use googletest::prelude::*;

#[gtest]
fn test_ctime() {
    let zone: Vec<u8> = "zone\0".into();
    // Tests of items from the `<ctime>` header.
    let _t = tm {
        tm_gmtoff: 0.into(),
        tm_hour: 1,
        tm_isdst: 2,
        tm_mday: 3,
        tm_min: 4,
        tm_mon: 5,
        tm_sec: 6,
        tm_wday: 7,
        tm_yday: 8,
        tm_year: 9,
        tm_zone: zone.as_ptr() as *mut ffi_11::c_char,
    };
}

#[gtest]
fn test_limits_inline() {
    // Tests of items from the `<limits>` header.
    // https://en.cppreference.com/w/cpp/types/numeric_limits/float_round_style:
    expect_eq!(i32::from(std::float_round_style::round_toward_zero), 0);
    expect_eq!(i32::from(std::float_round_style::round_to_nearest), 1);
    expect_eq!(i32::from(std::float_round_style::round_toward_infinity), 2);
    expect_eq!(i32::from(std::float_round_style::round_toward_neg_infinity), 3);
    expect_eq!(i32::from(std::float_round_style::round_indeterminate), -1);
}

#[gtest]
fn test_sockaddr_in() {
    let mut s = sockaddr_in::default();
    s.sin_family = 2;
    s.sin_port = 80;
    s.sin_addr.s_addr = 0x0100007f;
    expect_eq!(s.sin_family, 2);
    expect_eq!(s.sin_port, 80);
    expect_eq!(s.sin_addr.s_addr, 0x0100007f);
}

#[gtest]
fn test_sockaddr() {
    let mut s = sockaddr::default();
    s.sa_family = 2;
    expect_eq!(s.sa_family, 2);
}

#[gtest]
fn test_sockaddr_in6() {
    let mut s = sockaddr_in6::default();
    s.sin6_family = 10;
    s.sin6_port = 80;
    expect_eq!(s.sin6_family, 10);
    expect_eq!(s.sin6_port, 80);
}

#[gtest]
fn test_sockaddr_storage() {
    let s = sockaddr_storage::default();
    expect_eq!(::std::mem::size_of_val(&s), 128);
}
