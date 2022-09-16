// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_FORWARD_DECLARATIONS_NO_DEFINITION_IN_HEADERS_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_FORWARD_DECLARATIONS_NO_DEFINITION_IN_HEADERS_H_

#pragma clang lifetime_elision

// This is a regression test for b/246962427.
//
// This test mimics `absl::SynchLocksHeld` which is forward-declared in
// `absl/base/internal/thread_identity.h` and doesn't have a definition in any
// headers (only in `absl/synchronization/mutex.cc`).
//
// OTOH, note that the no-definition-in-headers wasn't the root cause of
// b/246962427.  Instead, there was a minor problem in the integration between
// A) `namespace` support and B) forward-declarations support.
namespace no_definition_in_headers {

struct FwdDeclared;

struct Defined final {
  FwdDeclared* field;
};

}  // namespace no_definition_in_headers

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_FORWARD_DECLARATIONS_NO_DEFINITION_IN_HEADERS_H_
