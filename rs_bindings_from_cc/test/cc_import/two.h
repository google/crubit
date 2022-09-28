// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_CC_IMPORT_TWO_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_CC_IMPORT_TWO_H_

namespace simple_math {
inline int get_two() { return 2; }
}  // namespace simple_math

namespace complex_math {
namespace two_only {
inline int get_square() { return 4; }
}  // namespace two_only
}  // namespace complex_math

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_CC_IMPORT_TWO_H_
