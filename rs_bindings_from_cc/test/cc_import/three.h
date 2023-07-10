// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_CC_IMPORT_THREE_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_CC_IMPORT_THREE_H_

namespace simple_math {
inline int get_three() { return 3; }
}  // namespace simple_math

namespace complex_math {
namespace three_only {
inline int get_square() { return 9; }
}  // namespace three_only
}  // namespace complex_math
#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_CC_IMPORT_THREE_H_
