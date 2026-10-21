// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use ir::ir_ffi_support::{FfiArc, FfiMap, FfiStr, FfiVec};

#[unsafe(no_mangle)]
pub extern "C" fn crubit_test_roundtrip_ffi_str(s: &mut FfiStr, out: &mut FfiStr) {
    let owned_from_cpp = std::mem::take(s);
    let updated = format!("{}_from_rust", owned_from_cpp.as_str());
    *out = FfiStr::from_str(&updated);
}

#[unsafe(no_mangle)]
pub extern "C" fn crubit_test_roundtrip_ffi_vec(v: &mut FfiVec<FfiStr>, out: &mut FfiVec<FfiStr>) {
    let mut owned_from_cpp = std::mem::take(v);
    owned_from_cpp.insert(1, FfiStr::from_str("inserted_in_rust"));
    *out = owned_from_cpp;
}

#[unsafe(no_mangle)]
pub extern "C" fn crubit_test_clone_and_cow_ffi_arc(
    shared: &FfiArc<FfiStr>,
    out_clone: &mut FfiArc<FfiStr>,
    out_mutated: &mut FfiArc<FfiStr>,
) {
    *out_clone = shared.clone();
    let mut cow = shared.clone();
    *cow.make_mut() = FfiStr::from_str("cow_from_rust");
    *out_mutated = cow;
}

#[unsafe(no_mangle)]
pub extern "C" fn crubit_test_ffi_arc_equals(lhs: &FfiArc<FfiStr>, rhs: &FfiArc<FfiStr>) -> bool {
    lhs == rhs
}

#[unsafe(no_mangle)]
pub extern "C" fn crubit_test_roundtrip_ffi_map(
    map: &mut FfiMap<FfiStr, FfiStr>,
    out: &mut FfiMap<FfiStr, FfiStr>,
) {
    let mut owned_from_cpp = std::mem::take(map);
    assert_eq!(owned_from_cpp.get("a").map(FfiStr::as_str), Some("1"));
    assert_eq!(owned_from_cpp.get("z").map(FfiStr::as_str), Some("26"));
    owned_from_cpp.push(FfiStr::from_str("m"), FfiStr::from_str("13"));
    owned_from_cpp.push(FfiStr::from_str("a"), FfiStr::from_str("updated_1"));
    *out = owned_from_cpp;
}
