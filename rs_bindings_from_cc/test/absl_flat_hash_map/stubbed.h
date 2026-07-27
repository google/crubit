// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ABSL_FLAT_HASH_MAP_STUBBED_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ABSL_FLAT_HASH_MAP_STUBBED_H_

#include <cstddef>
#include <cstdint>
#include <optional>
#include <utility>

// NOTE: This file declares `absl::flat_hash_map` and cannot be compiled
// together with the real implementation. That means this header cannot be
// tested in C++ with GoogleTest.

namespace absl {

template <typename K, typename V>
class flat_hash_map;

}  // namespace absl

namespace crubit::test {

// Disambiguation tags for `Stubs`. They can't be nested inside the class
// template because Crubit doesn't bind those yet.
struct SizeT final {};
struct CapacityT final {};
struct EmptyT final {};
struct TryEmplaceT final {};

// Provides values for the stub `flat_hash_map` implementation to return to
// exercise different FFI codepaths.
template <typename K, typename V>
class Stubs final {
 public:
  friend class absl::flat_hash_map<K, V>;

  Stubs() = default;
  // Static methods on class template instantiations aren't bound yet, so these
  // need to be constructors.
  Stubs(SizeT t, size_t size) : size_(size) {}
  Stubs(CapacityT t, size_t capacity) : capacity_(capacity) {}
  Stubs(EmptyT t, bool empty) : empty_(empty) {}
  Stubs(TryEmplaceT t, K&& k, V&& v, bool inserted)
      : pair_(k, v), try_emplace_inserted_(inserted) {}

 private:
  // The key-value pair that is referred to in returned values wherever
  // necessary. Must be provided via constructor if `K` or `V` is not default
  // constructible.
  std::pair<const K, V> pair_;

  std::optional<size_t> size_;
  std::optional<size_t> capacity_;
  std::optional<bool> empty_;
  std::optional<bool> try_emplace_inserted_;
};

}  // namespace crubit::test

namespace absl {

// A stub implementation of absl::flat_hash_map to test code generation without
// the absl dependency.
template <typename K, typename V>
class flat_hash_map final {
 public:
  using Stubs = crubit::test::Stubs<K, V>;
  using value_type = std::pair<const K, V>;
  using iterator = value_type*;

  explicit flat_hash_map(Stubs&& stubs) : stubs_(stubs) {}

  // Stubs for `flat_hash_map` methods that the generated bindings call.
  size_t size() const { return stubs_.size_.value_or(0); }
  size_t capacity() const { return stubs_.capacity_.value_or(0); }
  bool empty() const { return stubs_.empty_.value_or(false); }
  template <typename T, typename U>
  std::pair<iterator, bool> try_emplace(T t, U u) {
    return std::make_pair(&stubs_.pair_,
                          stubs_.try_emplace_inserted_.value_or(false));
  }

 private:
  Stubs stubs_;
};

}  // namespace absl

namespace crubit::test {

using IntsMap = absl::flat_hash_map<int32_t, uint64_t>;
using IntsStubs = IntsMap::Stubs;

// An integer wrapper that is `!Unpin` in Rust.
class Nonunpin {
 public:
  explicit Nonunpin(int32_t value) : value_(value) {}
  Nonunpin(const Nonunpin&) = default;
  Nonunpin& operator=(const Nonunpin&) = default;
  Nonunpin(Nonunpin&&) = default;
  Nonunpin& operator=(Nonunpin&&) = default;
  // Deliberately nontrivial destructor; leaving this implicit or explicitly
  // defaulting it would both result in a trivial destructor.
  // NOLINTNEXTLINE(modernize-use-equals-default)
  ~Nonunpin() {}

  int32_t value() const { return value_; }

 private:
  int32_t value_;
};

using NonunpinKeyMap = absl::flat_hash_map<Nonunpin, uint64_t>;
using NonunpinKeyStubs = NonunpinKeyMap::Stubs;

using NonunpinValueMap = absl::flat_hash_map<int32_t, Nonunpin>;
using NonunpinValueStubs = NonunpinValueMap::Stubs;

using NonunpinBothMap = absl::flat_hash_map<Nonunpin, Nonunpin>;
using NonunpinBothStubs = NonunpinBothMap::Stubs;

}  //  namespace crubit::test

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ABSL_FLAT_HASH_MAP_STUBBED_H_
