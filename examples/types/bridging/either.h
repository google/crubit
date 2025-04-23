// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_EXAMPLES_TYPES_BRIDGING_EITHER_H_
#define THIRD_PARTY_CRUBIT_EXAMPLES_TYPES_BRIDGING_EITHER_H_

#include "examples/types/bridging/either_internal.h"  // IWYU pragma: keep

namespace either {

template <typename L, typename R>
struct Either {
  bool is_left;
  union {
    L left;
    R right;
  };
};

}  // namespace either

#endif  // THIRD_PARTY_CRUBIT_EXAMPLES_TYPES_BRIDGING_EITHER_H_
