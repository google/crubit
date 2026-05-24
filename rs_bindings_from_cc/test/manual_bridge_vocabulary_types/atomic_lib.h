// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_MANUAL_BRIDGE_VOCABULARY_TYPES_ATOMIC_LIB_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_MANUAL_BRIDGE_VOCABULARY_TYPES_ATOMIC_LIB_H_

#include <atomic>

namespace crust {

inline void inc_atomic(
    std::atomic<unsigned long long>* val) {  // NOLINT(google-runtime-int)
  val->fetch_add(1, std::memory_order_relaxed);
}

inline unsigned long long load_atomic(       // NOLINT(google-runtime-int)
    std::atomic<unsigned long long>* val) {  // NOLINT(google-runtime-int)
  return val->load(std::memory_order_relaxed);
}

inline void store_atomic(
    std::atomic<unsigned long long>* val,  // NOLINT(google-runtime-int)
    unsigned long long new_val) {          // NOLINT(google-runtime-int)
  val->store(new_val, std::memory_order_relaxed);
}

inline bool exchange_atomic_bool(std::atomic<bool>* val, bool new_val) {
  return val->exchange(new_val, std::memory_order_seq_cst);
}

inline int fetch_add_atomic_int(std::atomic<int>* val, int arg) {
  return val->fetch_add(arg, std::memory_order_seq_cst);
}

inline int* exchange_atomic_ptr(std::atomic<int*>* val, int* new_ptr) {
  return val->exchange(new_ptr, std::memory_order_seq_cst);
}

enum class SomeEnum {
  kZero = 0,
  kOne = 1,
};

struct MyStruct {
  int x;
  int y;
};

struct EdgeCasesContainer {
  std::atomic<SomeEnum> atomic_enum;
  std::atomic<float> atomic_float;
  std::atomic<MyStruct> atomic_struct;
};

inline const int* exchange_atomic_const_ptr(std::atomic<const int*>* val,
                                            const int* new_ptr) {
  return val->exchange(new_ptr, std::memory_order_seq_cst);
}

}  // namespace crust

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_MANUAL_BRIDGE_VOCABULARY_TYPES_ATOMIC_LIB_H_
