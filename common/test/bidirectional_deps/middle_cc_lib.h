// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_COMMON_TEST_MIDDLE_CC_LIB_H_
#define CRUBIT_COMMON_TEST_MIDDLE_CC_LIB_H_

#include <utility>

#include "absl/functional/any_invocable.h"
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
// The functions below take `const unsigned char&` on purpose: the result
// borrows from the argument, and the reference parameters are what give it a
// lifetime (two of them make it ambiguous).
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
inline unsigned char UnwrapUnknownLifetime(
    leaf_rs_lib::LeafRsLifetime $(unknown) x) {
  return leaf_rs_lib::unwrap_lifetime(std::move(x));
}
inline leaf_rs_lib::LeafRsLifetimeAlias $a
WrapLifetimeAlias(const unsigned char& $a x) {
  return leaf_rs_lib::wrap_lifetime(&x);
}
inline leaf_rs_lib::StaticLeafRsLifetime MakeStaticLifetimeAlias() {
  return leaf_rs_lib::static_lifetime();
}

// `LeafRsSolo<'solo>`: the name of its lifetime parameter is reserved in C++
// (`$solo`), but a Rust type's own parameters are not C++ annotations, so it
// binds like `LeafRsLifetime`, with no reserved-name warning.
inline leaf_rs_lib::LeafRsSolo $a WrapSolo(const unsigned char& $a x) {
  return leaf_rs_lib::wrap_solo(&x);
}
inline leaf_rs_lib::LeafRsSolo WrapSoloElided(const unsigned char& x) {
  return leaf_rs_lib::wrap_solo(&x);
}
inline unsigned char UnwrapSolo(leaf_rs_lib::LeafRsSolo x) {
  return leaf_rs_lib::unwrap_solo(std::move(x));
}

// `LeafRsBounded<'a, 'b: 'a, 'c>` has three lifetime parameters, two of them
// related by an outlives bound. C++ cannot state the bound; the Rust signatures
// below must still satisfy it.
inline leaf_rs_lib::LeafRsBounded $a $a $c
WrapBounded(const unsigned char& $a x, const unsigned char& $c z) {
  return leaf_rs_lib::wrap_bounded(&x, &z);
}
inline leaf_rs_lib::LeafRsBounded $a $b $c
WrapBoundedDistinct(const unsigned char& $a x, const unsigned char& $b y,
                    const unsigned char& $c z) {
  return leaf_rs_lib::wrap_bounded(&x, &z);
}
inline leaf_rs_lib::LeafRsBounded WrapBoundedElided(const unsigned char& x) {
  return leaf_rs_lib::wrap_bounded(&x, &x);
}
inline leaf_rs_lib::LeafRsBounded MakeBoundedAmbiguous(const unsigned char& x,
                                                       const unsigned char& z) {
  return leaf_rs_lib::wrap_bounded(&x, &z);
}
inline unsigned char UnwrapBounded(leaf_rs_lib::LeafRsBounded x) {
  return leaf_rs_lib::unwrap_bounded(std::move(x));
}

// `LeafRsStaticBound<'a: 'static>` only admits `'static`.
inline leaf_rs_lib::LeafRsStaticBound $static
WrapStaticBound(const unsigned char& $static x) {
  return leaf_rs_lib::wrap_static_bound(&x);
}
inline leaf_rs_lib::LeafRsStaticBound WrapStaticBoundElided(
    const unsigned char& x) {
  return leaf_rs_lib::wrap_static_bound(&x);
}
inline unsigned char UnwrapStaticBound(leaf_rs_lib::LeafRsStaticBound x) {
  return leaf_rs_lib::unwrap_static_bound(std::move(x));
}

// A callback whose parameter type has a lifetime parameter. The argument only
// lives for the call, so the Rust parameter type takes any lifetime
// (`dyn Fn(LeafRsLifetime<'_>)`) rather than being wrapped in `Unsafe`.
inline unsigned char CallWithLifetime(
    absl::AnyInvocable<unsigned char(leaf_rs_lib::LeafRsLifetime) const> f) {
  const unsigned char x = 7;
  return f(leaf_rs_lib::wrap_lifetime(&x));
}

}  // namespace crubit

using LeafRsTypeAlias2 = leaf_rs_lib::LeafRsTypeAlias;
using LeafRsLifetimeAlias2 = leaf_rs_lib::LeafRsLifetimeAlias;
#endif  // CRUBIT_COMMON_TEST_MIDDLE_CC_LIB_H_
