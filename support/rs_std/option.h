// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// IWYU pragma: private, include "support/rs_std/option.h"

#ifndef THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_OPTIONAL_H_
#define THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_OPTIONAL_H_

namespace rs {

template <typename T>
struct Option final {
  static_assert(false,
                "This type should only be used via a generated specialization");
};

}  // namespace rs

namespace rs_std {
template <typename T>
using Option [[deprecated("Use rs::Option instead")]] = rs::Option<T>;
}  // namespace rs_std

#endif  // THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_OPTIONAL_H_
