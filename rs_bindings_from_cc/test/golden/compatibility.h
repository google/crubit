// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_COMPATIBILITY_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_COMPATIBILITY_H_

#pragma clang lifetime_elision

// This type renames the special member functions so that they can be
// overridden in Rust instead -- this is proof that you can write bindings
// that are forward-compatible, as described in
// additional_rust_srcs_for_crubit_bindings_aspect_hint.bzl
class CompatibleType {
 public:
  [[clang::annotate("crubit_rust_name", "renamed_default_constructor")]]
  CompatibleType();

  [[clang::annotate("crubit_rust_name", "renamed_copy_constructor")]]
  CompatibleType(const CompatibleType&);
  CompatibleType& operator=(const CompatibleType&);

  [[clang::annotate("crubit_rust_name", "renamed_constructor")]]
  explicit CompatibleType(int);
};

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_COMPATIBILITY_H_
