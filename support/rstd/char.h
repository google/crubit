// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_SUPPORT_RSTD_CHAR_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_SUPPORT_RSTD_CHAR_H_

#include <cstdint>

namespace rstd {

// `rstd::Char` is a C++ representation of the `char` type from Rust.
//
// See "layout tests" comments in `char_test.cc` for explanation why `char32_t`
// is not used.
//
// TODO(b/265338802): Reject `char` values with invalid bit patterns (possibly
// retaining `constexpr` aspect of some conversions).
using Char = std::uint32_t;

}  // namespace rstd

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_SUPPORT_RSTD_CHAR_H_
