// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_HEADER_IN_MULTIPLE_TARGETS_SHARED_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_HEADER_IN_MULTIPLE_TARGETS_SHARED_H_

#include <functional>

namespace shared_ns {

// Ordinary `//` comments are used as doc comments, even though precompiled
// header modules don't contain them.
class Info {
 public:
  // Returns the value.
  int value() const { return value_; }

 private:
  int value_ = 42;
};

// A free comment, which isn't attached to any declaration.

// Clang doesn't merge the declarations below (which have no linkage, or
// internal linkage) when it sees them both in a precompiled header module, and
// in a textual inclusion of the same header.

// An alias of a class template specialization whose arguments are declared in
// this header.
using Callback = std::function<int(const Info&)>;

using IntAlias = int;
typedef Info InfoTypedef;

constexpr int kSharedConstant = 3;

enum { kAnonymousEnumerator = 5 };

// Not bound in `:lib`: like overloads, the two copies of this function make
// taking its address ambiguous in the generated C++ code. This checks that the
// generated C++ code still compiles.
static inline int StaticSharedFunction() { return 8; }

// Clang does merge the declarations below (which have external linkage).

enum class Color { kRed, kGreen };

inline int SharedFunction() { return 7; }

}  // namespace shared_ns

namespace other_ns {

using ::shared_ns::Info;

}  // namespace other_ns

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_HEADER_IN_MULTIPLE_TARGETS_SHARED_H_
