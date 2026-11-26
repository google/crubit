// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ABSL_FLAT_HASH_MAP_FLAT_HASH_MAP_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ABSL_FLAT_HASH_MAP_FLAT_HASH_MAP_H_

#include <cstddef>
#include <utility>

namespace absl {

// An empty implementation of absl::flat_hash_map to test code generation
// without the absl dependency.
template <typename K, typename V, int ThirdParameterMustBeAccepted = 42>
class flat_hash_map final {
 public:
  void FunctionRemovedByCustomization() const;

  template <typename F>
  void HarmlessTemplateFunction(F f) const;

  size_t size() const { return 0; }
  size_t capacity() const { return 0; }
  void clear() {}
  void reserve(size_t count) {}
  template <typename LookupKey = K>
  bool contains(const LookupKey& k) const {
    return false;
  }
  template <typename LookupKey = K>
  std::pair<const K, V>* find(const LookupKey& k) {
    return nullptr;
  }
  template <typename LookupKey = K>
  const std::pair<const K, V>* find(const LookupKey& k) const {
    return nullptr;
  }
  template <typename Unused = void>
  std::pair<const K, V>* end() {
    return nullptr;
  }
  template <typename Unused = void>
  const std::pair<const K, V>* end() const {
    return nullptr;
  }
  template <typename LookupKey = K>
  size_t erase(const LookupKey& k) {
    return 0;
  }
  template <typename... Args>
  std::pair<std::pair<const K, V>*, bool> try_emplace(K key, Args&&... args) {
    return std::make_pair(nullptr, false);
  }
};

}  // namespace absl

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ABSL_FLAT_HASH_MAP_FLAT_HASH_MAP_H_
