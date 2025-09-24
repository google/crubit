// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_SUPPORT_CC_STD_IMPL_TEST_VECTOR_CC_HELPER_FUNCTIONS_H_
#define THIRD_PARTY_CRUBIT_SUPPORT_CC_STD_IMPL_TEST_VECTOR_CC_HELPER_FUNCTIONS_H_

#include <stddef.h>

#include <cstdint>
#include <vector>

namespace crubit_test {

inline int32_t vector_int32_sum(void* v) {
  std::vector<int32_t>* vec = reinterpret_cast<std::vector<int32_t>*>(v);
  int32_t sum = 0;
  for (int32_t value : *vec) {
    sum += value;
  }
  return sum;
}

inline size_t vector_int32_capacity(void* v) {
  std::vector<int32_t>* vec = reinterpret_cast<std::vector<int32_t>*>(v);
  return vec->capacity();
}

inline void vector_int32_push_back(void* v, int32_t value) {
  std::vector<int32_t>* vec = reinterpret_cast<std::vector<int32_t>*>(v);
  vec->push_back(value);
}

inline void vector_int32_construct(void* v) { new (v) std::vector<int32_t>(); }

inline void vector_int32_call_destructor(void* v) {
  reinterpret_cast<std::vector<int32_t>*>(v)->~vector<int32_t>();
}

inline void vector_int32_delete(void* v) {
  delete reinterpret_cast<std::vector<int32_t>*>(v);
}

inline void vector_int32_clear(void* v) {
  std::vector<int32_t>* vec = reinterpret_cast<std::vector<int32_t>*>(v);
  vec->clear();
}

}  // namespace crubit_test

#endif  // THIRD_PARTY_CRUBIT_SUPPORT_CC_STD_IMPL_TEST_VECTOR_CC_HELPER_FUNCTIONS_H_
