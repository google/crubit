// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_TEST_BRIDGING_MAPPED_CPP_TYPE_DEF_H
#define CRUBIT_TEST_BRIDGING_MAPPED_CPP_TYPE_DEF_H

#include <cstdint>

#include "support/unsafe_relocate_tag.h"

namespace mapped_cpp_type {
struct MappedCppType {
  int32_t value;
};

template <typename T>
struct GenericMappedCppType {
  T value;
};

// Not C++ move-constructible, but relocatable via `crubit::UnsafeRelocateTag`.
struct UnmovableMappedCppType {
  int32_t value;
  UnmovableMappedCppType(const UnmovableMappedCppType&) = delete;
  UnmovableMappedCppType(UnmovableMappedCppType&&) = delete;
  UnmovableMappedCppType& operator=(const UnmovableMappedCppType&) = delete;
  UnmovableMappedCppType& operator=(UnmovableMappedCppType&&) = delete;
  explicit UnmovableMappedCppType(int32_t v) : value(v) {}
  UnmovableMappedCppType(crubit::UnsafeRelocateTag,
                         UnmovableMappedCppType&& other)
      : value(other.value) {}
  ~UnmovableMappedCppType() {}
};
}  // namespace mapped_cpp_type

#endif  // CRUBIT_TEST_BRIDGING_MAPPED_CPP_TYPE_DEF_H
