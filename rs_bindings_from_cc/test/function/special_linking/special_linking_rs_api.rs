// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/function/special_linking:special_linking

#![rustfmt::skip]
#![feature(custom_inner_attributes)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![allow(deprecated)]
#![deny(warnings)]

// error: function `weak_import_func` could not be bound
//   Function is weakly imported

/// This test function is only available in Android API 9999+.
/// If we compile targeting Android API 23, this is a weak import
/// (so the caveats of `weak_import_func` apply).
///
/// This problem was discovered when trying to enable build tests targeting
/// Android in cl/937668344.
///
/// Original scenario: Functions like `sighold`, `sigignore`, `sigpause`,
/// `sigrelse`, and `sigset` in Bionic's `<signal.h>` are introduced in API 26.
/// When compiling targeting API 23 with
/// `__ANDROID_UNAVAILABLE_SYMBOLS_ARE_WEAK__` defined (standard for NDK), Clang
/// treats them as weakly imported (unavailable, but compile-able as weak).
/// See Bionic's signal.h:
/// https://github.com/aosp-mirror/platform_bionic/blob/android-14.0.0_r1/libc/include/signal.h
#[inline(always)]
pub fn min_android_version_9999() {
    unsafe { crate::detail::__rust_thunk___Z24min_android_version_9999v() }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        #[link_name = "_Z24min_android_version_9999v"]
        pub(crate) unsafe fn __rust_thunk___Z24min_android_version_9999v();
    }
}
