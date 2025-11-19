// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use ffi_11::*;
use static_assertions::assert_impl_all;

const _: () = {
    assert_impl_all!(c_char: From<i8>, From<u8>);

    assert_impl_all!(c_short: From<i8>, From<u8>, From<i16>, From<c_schar>);
    assert_impl_all!(c_ushort: From<u8>, From<u16>, From<c_uchar>);

    assert_impl_all!(c_int: From<i8>, From<u8>, From<i16>, From<c_schar>, From<c_short>);
    assert_impl_all!(c_uint: From<u8>, From<u16>, From<c_uchar>, From<c_ushort>);

    assert_impl_all!(c_long: From<i8>, From<u8>, From<i16>, From<i32>, From<c_schar>, From<c_short>, From<c_int>);
    assert_impl_all!(c_ulong: From<u8>, From<u16>, From<u32>, From<c_uchar>, From<c_ushort>, From<c_uint>);

    assert_impl_all!(c_longlong: From<i8>, From<u8>, From<i16>, From<i32>, From<i64>, From<c_schar>, From<c_short>, From<c_int>, From<c_long>);
    assert_impl_all!(c_ulonglong: From<u8>, From<u16>, From<u32>, From<u64>, From<c_uchar>, From<c_ushort>, From<c_uint>, From<c_ulong>);
};

const _: () = {
    assert_impl_all!(u8: From<c_uchar>, From<c_char>);
    assert_impl_all!(i8: From<c_schar>, From<c_char>);

    // ffi_11 types are always convertible to any correctly signed type of equal or greater size.
    // The simplest cross-platform test is: they're always convertible to the equivalent core::ffi
    // type!
    assert_impl_all!(core::ffi::c_short: From<c_short>);
    assert_impl_all!(core::ffi::c_ushort: From<c_ushort>);
    assert_impl_all!(core::ffi::c_int: From<c_int>);
    assert_impl_all!(core::ffi::c_uint: From<c_uint>);
    assert_impl_all!(core::ffi::c_long: From<c_long>);
    assert_impl_all!(core::ffi::c_ulong: From<c_ulong>);

    assert_impl_all!(core::ffi::c_longlong: From<c_longlong>);
    assert_impl_all!(core::ffi::c_ulonglong: From<c_ulonglong>);

    assert_impl_all!(c_longlong: From<i8>, From<u8>, From<i16>, From<i32>, From<i64>, From<c_schar>, From<c_short>, From<c_int>, From<c_long>);
    assert_impl_all!(c_ulonglong: From<u8>, From<u16>, From<u32>, From<u64>, From<c_uchar>, From<c_ushort>, From<c_uint>, From<c_ulong>);
};
