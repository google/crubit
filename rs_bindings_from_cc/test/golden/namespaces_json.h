// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_NAMESPACES_JSON_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_NAMESPACES_JSON_H_

namespace foo {
namespace bar {
struct S {};
}  // namespace bar
}  // namespace foo

namespace foo {
namespace bar {
namespace baz {}
namespace {}
}  // namespace bar
inline namespace baz {}
}  // namespace foo

namespace xyz {
namespace foo {}
}  // namespace xyz

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_NAMESPACES_JSON_H_
