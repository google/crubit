// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_COMMON_TEST_MIDDLE_CC_LIB_H_
#define CRUBIT_COMMON_TEST_MIDDLE_CC_LIB_H_

#include <utility>

#include "common/test/bidirectional_deps/leaf_rs_lib.h"
#include "support/lifetime_annotations.h"

#pragma clang lifetime_elision

namespace crubit {

inline leaf_rs_lib::LeafRsType Wrap(unsigned char x) {
  return leaf_rs_lib::wrap(x);
}
inline unsigned char Unwrap(leaf_rs_lib::LeafRsType x) {
  return leaf_rs_lib::unwrap(std::move(x));
}

inline leaf_rs_lib::LeafRsEnum WrapEnum(unsigned char x) {
  return leaf_rs_lib::wrap_enum(x);
}
inline unsigned char UnwrapEnum(leaf_rs_lib::LeafRsEnum x) {
  return leaf_rs_lib::unwrap_enum(std::move(x));
}

inline leaf_rs_lib::LeafRsLifetime $static MakeStaticLifetime() {
  return leaf_rs_lib::static_lifetime();
}
inline leaf_rs_lib::LeafRsLifetime MakeUnannotatedLifetime() {
  return leaf_rs_lib::static_lifetime();
}
inline leaf_rs_lib::LeafRsLifetime MakeAmbiguousLifetime(
    const unsigned char& x, const unsigned char& y) {
  return leaf_rs_lib::wrap_lifetime(&x);
}
inline leaf_rs_lib::LeafRsLifetime $a WrapLifetime(const unsigned char& $a x) {
  return leaf_rs_lib::wrap_lifetime(&x);
}
inline leaf_rs_lib::LeafRsLifetime WrapLifetimeElided(const unsigned char& x) {
  return leaf_rs_lib::wrap_lifetime(&x);
}
inline unsigned char UnwrapLifetime(leaf_rs_lib::LeafRsLifetime x) {
  return leaf_rs_lib::unwrap_lifetime(std::move(x));
}
inline leaf_rs_lib::LeafRsLifetimeAlias $a
WrapLifetimeAlias(const unsigned char& $a x) {
  return leaf_rs_lib::wrap_lifetime(&x);
}
inline leaf_rs_lib::StaticLeafRsLifetime MakeStaticLifetimeAlias() {
  return leaf_rs_lib::static_lifetime();
}

}  // namespace crubit

using LeafRsTypeAlias2 = leaf_rs_lib::LeafRsTypeAlias;
using LeafRsLifetimeAlias2 = leaf_rs_lib::LeafRsLifetimeAlias;
#endif  // CRUBIT_COMMON_TEST_MIDDLE_CC_LIB_H_
