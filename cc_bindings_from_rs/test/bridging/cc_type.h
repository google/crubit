// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_BRIDGING_CC_TYPE_H_
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_BRIDGING_CC_TYPE_H_

namespace crubit {
namespace test {

struct TheCppType {
 public:
  TheCppType(int _x) : y(23432421), x(_x) {}
  int get_x() const { return x; }

 private:
  // Added this field so the type is guaranteed to have a different
  // memory layout from the 'TheRustType'.
  int y;
  int x;
};

}  // namespace test
}  // namespace crubit

extern "C" void crubit_test_new_cpp_type(int x, void* cc_type_out) {
  new (cc_type_out) crubit::test::TheCppType(x);
}

extern "C" int crubit_test_cpp_type_get_x(const void* cc_type_out) {
  return reinterpret_cast<const crubit::test::TheCppType*>(cc_type_out)
      ->get_x();
}

#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_BRIDGING_CC_TYPE_H_
