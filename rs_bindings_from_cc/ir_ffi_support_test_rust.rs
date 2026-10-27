// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use ir::ir_ffi_support::{FfiMap, FfiRc, FfiString, FfiVec};

#[unsafe(no_mangle)]
pub extern "C" fn crubit_test_roundtrip_ffi_string(s: &mut FfiString, out: &mut FfiString) {
    let owned_from_cpp = std::mem::take(s);
    let updated = format!("{}_from_rust", owned_from_cpp.as_str());
    *out = FfiString::from_str(&updated);
}

#[unsafe(no_mangle)]
pub extern "C" fn crubit_test_pass_ffi_string_by_value(s: FfiString, out: &mut FfiString) {
    let updated = format!("{}_by_value", s.as_str());
    *out = FfiString::from_str(&updated);
}

#[unsafe(no_mangle)]
pub extern "C" fn crubit_test_roundtrip_ffi_vec(
    v: &mut FfiVec<FfiString>,
    out: &mut FfiVec<FfiString>,
) {
    let mut owned_from_cpp = std::mem::take(v);
    owned_from_cpp.insert(1, FfiString::from_str("inserted_in_rust"));
    *out = owned_from_cpp;
}

#[unsafe(no_mangle)]
pub extern "C" fn crubit_test_clone_and_cow_ffi_rc(
    shared: &FfiRc<FfiString>,
    out_clone: &mut FfiRc<FfiString>,
    out_mutated: &mut FfiRc<FfiString>,
) {
    *out_clone = shared.clone();
    let mut cow = shared.clone();
    *cow.make_mut() = FfiString::from_str("cow_from_rust");
    *out_mutated = cow;
}

#[unsafe(no_mangle)]
pub extern "C" fn crubit_test_ffi_rc_equals(
    lhs: &FfiRc<FfiString>,
    rhs: &FfiRc<FfiString>,
) -> bool {
    lhs == rhs
}

#[unsafe(no_mangle)]
pub extern "C" fn crubit_test_roundtrip_ffi_map(
    map: &mut FfiMap<FfiString, FfiString>,
    out: &mut FfiMap<FfiString, FfiString>,
) {
    let mut owned_from_cpp = std::mem::take(map);
    assert_eq!(owned_from_cpp.get("a").map(FfiString::as_str), Some("1"));
    assert_eq!(owned_from_cpp.get("z").map(FfiString::as_str), Some("26"));
    owned_from_cpp.insert(FfiString::from_str("m"), FfiString::from_str("13"));
    owned_from_cpp.insert(FfiString::from_str("a"), FfiString::from_str("updated_1"));
    *out = owned_from_cpp;
}
