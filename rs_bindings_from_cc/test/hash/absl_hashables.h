// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_HASH_ABSL_HASHABLES_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_HASH_ABSL_HASHABLES_H_

#include <cstdint>
#include <utility>

#include "support/annotations.h"

// `AbslHashValue` is a hidden friend. It ignores `ignored_by_hash`, which lets
// the tests check that the C++ hash is used (and not e.g. `#[derive(Hash)]`).
struct CRUBIT_MUST_BIND HashedByFriend {
  int32_t value;
  int32_t ignored_by_hash;

  template <typename H>
  friend H AbslHashValue(H h, const HashedByFriend& x) {
    return H::combine(std::move(h), x.value);
  }
};

namespace ns {

// `AbslHashValue` is a free function in the same namespace, and takes the
// hashed value by value.
struct CRUBIT_MUST_BIND HashedByFreeFunction {
  int32_t value;
  int32_t ignored_by_hash;
};

template <typename H>
H AbslHashValue(H h, HashedByFreeFunction x) {
  return H::combine(std::move(h), x.value);
}

}  // namespace ns

struct CRUBIT_MUST_BIND NotHashable {
  int32_t value;
};

template <typename T>
struct Wrapper {
  T value;

  template <typename H>
  friend H AbslHashValue(H h, const Wrapper& x) {
    return H::combine(std::move(h), x.value);
  }
};

// Calling `AbslHashValue` for this class template instantiation doesn't
// compile.
using WrappedNotHashable = Wrapper<NotHashable>;

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_HASH_ABSL_HASHABLES_H_
