// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//TODO(rosica): We need namespaces in order to be able to test more here.

use cc_std::*;

#[test]
fn test_ctime() {
    let zone: Vec<u8> = "zone\0".into();
    // Tests of items from the `<ctime>` header.
    let _t = tm {
        tm_gmtoff: 0,
        tm_hour: 1,
        tm_isdst: 2,
        tm_mday: 3,
        tm_min: 4,
        tm_mon: 5,
        tm_sec: 6,
        tm_wday: 7,
        tm_yday: 8,
        tm_year: 9,
        tm_zone: zone.as_ptr() as *mut core::ffi::c_char,
    };
}

#[test]
fn test_limits_inline() {
    // Tests of items from the `<limits>` header.
    // https://en.cppreference.com/w/cpp/types/numeric_limits/float_round_style:
    assert_eq!(0, std::float_round_style::round_toward_zero.into());
    assert_eq!(1, std::float_round_style::round_to_nearest.into());
    assert_eq!(2, std::float_round_style::round_toward_infinity.into());
    assert_eq!(3, std::float_round_style::round_toward_neg_infinity.into());
    assert_eq!(-1, std::float_round_style::round_indeterminate.into());
}
