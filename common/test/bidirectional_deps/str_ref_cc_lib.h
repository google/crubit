// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_COMMON_TEST_STR_REF_CC_LIB_H_
#define CRUBIT_COMMON_TEST_STR_REF_CC_LIB_H_

#include "absl/functional/any_invocable.h"
#include "common/test/bidirectional_deps/leaf_rs_lib.h"
#include "support/rs_std/str_ref.h"

// `rs::StrRef` is `&'a str` in Rust: its lifetime must flow into the results
// of the functions below the same way a reference's would.
namespace crubit {

// The result borrows from `s`: `HoldStr<'s>(s: &'s str) -> LeafRsStr<'s>`.
inline leaf_rs_lib::LeafRsStr HoldStr(rs::StrRef s) {
  return leaf_rs_lib::wrap_str(s);
}

// Two inputs with lifetimes: the result's lifetime is ambiguous, so it is
// wrapped in `Unsafe`.
inline leaf_rs_lib::LeafRsStr HoldStrAmbiguous(rs::StrRef s,
                                               const unsigned char& x) {
  return leaf_rs_lib::wrap_str(s);
}

inline unsigned char FirstByte(leaf_rs_lib::LeafRsStr h) {
  return leaf_rs_lib::first_byte(h);
}

// `Echo<'s>(s: &'s str) -> &'s str`.
inline rs::StrRef Echo(rs::StrRef s) { return s; }

// Two `rs::StrRef` inputs: ambiguous.
inline rs::StrRef Pick(rs::StrRef a, rs::StrRef b) { return a; }

// A callback taking `rs::StrRef`: the argument only lives for the call, so the
// Rust parameter type is `dyn Fn(&str)` (any lifetime), not `Unsafe<..>`.
inline unsigned char CallWithStr(
    absl::AnyInvocable<unsigned char(rs::StrRef) const> f) {
  return f(rs::StrRef("hello"));
}

}  // namespace crubit

#endif  // CRUBIT_COMMON_TEST_STR_REF_CC_LIB_H_
