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

// Send + Sync: non-aggregate with no bases (0-byte `__non_field_data` only
// prevents direct initialization and holds no C++ data).
class NonAggregatePlain final {
 public:
  NonAggregatePlain() = default;
  int x_ = 0;
};

// Send + Sync: bitfields and unnamed zero-width bitfields hold only integers /
// alignment directives.
struct WithZeroWidthBitfield final {
  int a : 4;
  int : 0;
  int b : 4;
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

// An empty tag type which is explicitly neither Send nor Sync.
struct CRUBIT_TRAIT_DERIVE("!Send", "!Sync") ThreadAffineTag {};

// Neither: empty base class subobjects are not emitted as Rust fields.
struct DerivedFromThreadAffineTag final : ThreadAffineTag {
  int x;
};

// Neither: private empty base class subobjects are not emitted as Rust fields.
struct EmptyBase {};
class DerivedFromPrivateEmpty final : private EmptyBase {
 public:
  int x_;
};

// Neither: zero-sized `[[no_unique_address]]` members are not emitted as Rust
// fields.
#ifdef _MSC_VER
#define CRUBIT_TEST_NO_UNIQUE_ADDRESS [[msvc::no_unique_address]]
#else
#define CRUBIT_TEST_NO_UNIQUE_ADDRESS [[no_unique_address]]
#endif
struct NoUniqueAddressThreadAffineTag final {
  CRUBIT_TEST_NO_UNIQUE_ADDRESS ThreadAffineTag tag;
  int x;
};
#undef CRUBIT_TEST_NO_UNIQUE_ADDRESS

// Neither: zero-length array members are zero-sized and not emitted as Rust
// fields.
#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wzero-length-array"
struct WithZeroLengthPointerArray final {
  double x;
  int* items[0];
};
#pragma clang diagnostic pop

// Neither: contains a member which is neither.
struct MemberThreadAffineTag final {
  ThreadAffineTag tag;
  int x;
};

// Neither: annotation on a class template applies to its instantiations.
template <typename T>
struct CRUBIT_TRAIT_DERIVE("!Send", "!Sync") ThreadAffineTemplate final {
  T x;
};
using ThreadAffineTemplateInt = ThreadAffineTemplate<int>;

}  // namespace crubit::test

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_SEND_SYNC_INFERENCE_SEND_SYNC_INFERENCE_H_
