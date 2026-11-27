// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_DESTRUCTORS_FIELD_DESTRUCTION_ORDER_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_DESTRUCTORS_FIELD_DESTRUCTION_ORDER_H_

#include <memory>
#include <utility>

#include "absl/base/nullability.h"
#include "support/annotations.h"

// When destroyed, appends `int_field` to `*record` as a decimal digit, so that
// a test can check the order in which objects were destroyed. Each test passes
// in its own `record`.
//
// `std::shared_ptr` isn't trivial-ABI on every platform, and where it isn't,
// Clang ignores `[[clang::trivial_abi]]` on this class. Moving it with `memcpy`
// is still sound, so `CRUBIT_UNSAFE_BITWISE_RELOCATABLE` keeps it Rust-movable
// there too.
class CRUBIT_UNSAFE_BITWISE_RELOCATABLE [[clang::trivial_abi]]
DestructionOrderRecorder final {
 public:
  DestructionOrderRecorder(int int_field,
                           absl_nonnull std::shared_ptr<int> record)
      : int_field_(int_field), record_(std::move(record)) {}
  DestructionOrderRecorder(const DestructionOrderRecorder& other) = delete;
  DestructionOrderRecorder& operator=(const DestructionOrderRecorder& other) =
      delete;
  DestructionOrderRecorder(DestructionOrderRecorder&& other)
      : int_field_(other.int_field_), record_(std::move(other.record_)) {}
  DestructionOrderRecorder& operator=(DestructionOrderRecorder&& other) {
    RecordDestruction();
    int_field_ = other.int_field_;
    record_ = std::move(other.record_);
    return *this;
  }
  ~DestructionOrderRecorder() { RecordDestruction(); }

 private:
  void RecordDestruction() {
    if (record_ != nullptr) {
      *record_ = *record_ * 10 + int_field_;
    }
  }

  int int_field_;
  // Null if this object has been moved from.
  std::shared_ptr<int> record_;
};

class [[clang::trivial_abi]] FieldDestructionOrderTester final {
 public:
  FieldDestructionOrderTester(DestructionOrderRecorder field1,
                              DestructionOrderRecorder field2,
                              DestructionOrderRecorder field3)
      : field1_(std::move(field1)),
        field2_(std::move(field2)),
        field3_(std::move(field3)) {}

  static void DestructFromCpp(int field1, int field2, int field3,
                              absl_nonnull std::shared_ptr<int> record) {
    auto tester =
        FieldDestructionOrderTester(DestructionOrderRecorder(field1, record),
                                    DestructionOrderRecorder(field2, record),
                                    DestructionOrderRecorder(field3, record));
  }

 private:
  DestructionOrderRecorder field1_;
  DestructionOrderRecorder field2_;
  DestructionOrderRecorder field3_;
};

// Shares ownership of `token`, so that a test can tell from `token`'s use count
// whether this object has been destroyed.
class [[clang::trivial_abi]] PrivateOperatorDelete final {
 public:
  explicit PrivateOperatorDelete(absl_nonnull std::shared_ptr<int> token)
      : token_(std::move(token)) {}

 private:
  void operator delete(void*);
  std::shared_ptr<int> token_;
};

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_DESTRUCTORS_FIELD_DESTRUCTION_ORDER_H_
