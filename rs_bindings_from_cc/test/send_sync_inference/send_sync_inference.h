// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_SEND_SYNC_INFERENCE_SEND_SYNC_INFERENCE_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_SEND_SYNC_INFERENCE_SEND_SYNC_INFERENCE_H_

#include "support/annotations.h"

// These types are compiled with the `infer_send_sync` Crubit feature, so
// `Send` and `Sync` are inferred by rustc from the fields.

namespace crubit::test {

// Send + Sync: only plain data.
struct Plain final {
  int x;
  float y;
};

// Send + Sync: an empty class holds no data.
struct Empty final {};

// Send + Sync: contains a Send + Sync record.
struct ContainsPlain final {
  Plain plain;
};

// Neither: raw pointers are neither Send nor Sync.
struct WithPointer final {
  int* p;
};

// Neither: contains a record which is neither.
struct ContainsWithPointer final {
  WithPointer with_pointer;
};

// Neither: private fields are opaque bytes.
class WithPrivateField final {
 public:
  int public_field_;

 private:
  [[maybe_unused]] int private_field_;
};

// Neither: fields of unsupported types are opaque bytes.
struct WithUnsupportedField final {
  volatile int unsupported;
};

struct Base {
  int base_field;
};

// Neither: base class subobjects are opaque bytes.
struct Derived final : Base {
  int derived_field;
};

// Sync, but explicitly not Send.
struct CRUBIT_TRAIT_DERIVE("!Send") NotSend final {
  int x;
};

// Send, but explicitly not Sync.
struct CRUBIT_TRAIT_DERIVE("!Sync") NotSync final {
  int x;
};

// Neither, explicitly.
struct CRUBIT_TRAIT_DERIVE("!Send", "!Sync") NotSendNotSync final {
  int x;
};

// Send + Sync, explicitly, despite the pointer.
struct CRUBIT_UNSAFE_IMPL("Send", "Sync") ExplicitSendSync final {
  int* p;
};

}  // namespace crubit::test

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_SEND_SYNC_INFERENCE_SEND_SYNC_INFERENCE_H_
