// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_ASYNC_FN_ASYNC_FN_EXISTING_CPP_TYPES_H_
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_ASYNC_FN_ASYNC_FN_EXISTING_CPP_TYPES_H_

namespace crubit {
namespace test {

// A C++ struct used to test the `cpp_convertible` bridging annotation.
// This type requires custom conversion thunks (`rust_to_cpp_converter` and
// `cpp_to_rust_converter`). Crubit currently rejects async functions returning
// such bridged types.
struct AsyncFnCppConvertible {
 public:
  AsyncFnCppConvertible() : x(0) {}
  explicit AsyncFnCppConvertible(int _x) : x(_x) {}
  int get_x() const { return x; }

 private:
  int x;
};

// A C++ struct used to test the `cpp_layout_equivalent` bridging annotation.
// This type is representation-equivalent to its Rust counterpart and requires
// no conversion thunks, allowing it to be returned from async functions.
struct AsyncFnCppLayoutEquivalent {
 public:
  AsyncFnCppLayoutEquivalent() : x(0) {}
  explicit AsyncFnCppLayoutEquivalent(int _x) : x(_x) {}
  int get_x() const { return x; }

 private:
  int x;
};

}  // namespace test
}  // namespace crubit

#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_ASYNC_FN_ASYNC_FN_EXISTING_CPP_TYPES_H_
