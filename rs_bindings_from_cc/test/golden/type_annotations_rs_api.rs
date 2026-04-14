// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:type_annotations_cc

#![rustfmt::skip]
#![feature(custom_inner_attributes)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![allow(deprecated)]
#![deny(warnings)]

/// Generated from: rs_bindings_from_cc/test/golden/type_annotations.h;l=8[348,352]
#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
///CRUBIT_ANNOTATE: cpp_type=Enum
pub struct Enum(::ffi_11::c_uint);
impl Enum {
    pub const kValue: Enum = Enum(::ffi_11::new_c_uint(0));
}
impl From<::ffi_11::c_uint> for Enum {
    fn from(value: ::ffi_11::c_uint) -> Enum {
        Enum(value)
    }
}
impl From<Enum> for ::ffi_11::c_uint {
    fn from(value: Enum) -> ::ffi_11::c_uint {
        value.0
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/type_annotations.h;l=9[376,385]
#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
///CRUBIT_ANNOTATE: cpp_type=EnumClass
pub struct EnumClass(::ffi_11::c_int);
impl EnumClass {
    pub const kValue: EnumClass = EnumClass(::ffi_11::new_c_int(0));
}
impl From<::ffi_11::c_int> for EnumClass {
    fn from(value: ::ffi_11::c_int) -> EnumClass {
        EnumClass(value)
    }
}
impl From<EnumClass> for ::ffi_11::c_int {
    fn from(value: EnumClass) -> ::ffi_11::c_int {
        value.0
    }
}

// This file contains Kythe metadata. eyJ0eXBlIjoia3l0aGUwIiwibWV0YSI6W3sidHlwZSI6ImFuY2hvcl9hbmNob3IiLCJzb3VyY2VfYmVnaW4iOjM0OCwic291cmNlX2VuZCI6MzUyLCJ0YXJnZXRfYmVnaW4iOjU5MiwidGFyZ2V0X2VuZCI6NTk2LCJlZGdlIjoiL2t5dGhlL2VkZ2UvaW1wdXRlcyIsInNvdXJjZV92bmFtZSI6eyJjb3JwdXMiOiJjb3JwdXMiLCJwYXRoIjoidGhpcmRfcGFydHkvY3J1Yml0L3JzX2JpbmRpbmdzX2Zyb21fY2MvdGVzdC9nb2xkZW4vdHlwZV9hbm5vdGF0aW9ucy5oIiwibGFuZ3VhZ2UiOiJjKysifX0seyJ0eXBlIjoiYW5jaG9yX2FuY2hvciIsInNvdXJjZV9iZWdpbiI6Mzc2LCJzb3VyY2VfZW5kIjozODUsInRhcmdldF9iZWdpbiI6MTE2NSwidGFyZ2V0X2VuZCI6MTE3NCwiZWRnZSI6Ii9reXRoZS9lZGdlL2ltcHV0ZXMiLCJzb3VyY2Vfdm5hbWUiOnsiY29ycHVzIjoiY29ycHVzIiwicGF0aCI6InRoaXJkX3BhcnR5L2NydWJpdC9yc19iaW5kaW5nc19mcm9tX2NjL3Rlc3QvZ29sZGVuL3R5cGVfYW5ub3RhdGlvbnMuaCIsImxhbmd1YWdlIjoiYysrIn19XX0=
