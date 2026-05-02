// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ANNOTATIONS_OWNED_PTR_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ANNOTATIONS_OWNED_PTR_H_

#include <cstdint>

#include "support/annotations.h"

// An example of a C++ struct that supports ownership via the raw pointer.
//
// The CRUBIT_OWNED_PTR_TYPE annotation is used to specify the Rust type that
// will be used to represent the C++ struct when it is used in a position
// annotated with CRUBIT_OWNED_PTR.
struct CRUBIT_OWNED_POINTEE("OwnedThing") CRUBIT_RUST_NAME("RawThing") Thing {
  explicit Thing(int32_t value) : value(value) {};
  int32_t value;

  void Close() { delete this; }
};

// A struct that specifies a custom drop method name.
struct CRUBIT_OWNED_POINTEE("CustomOwnedThing", "CustomDropImpl")
    CRUBIT_RUST_NAME("CustomRawThing") CustomThing {
  explicit CustomThing(int32_t value) : value(value) {}
  int32_t value;

  void CustomDropImpl() { delete this; }
};

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ANNOTATIONS_OWNED_PTR_H_
