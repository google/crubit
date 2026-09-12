// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ABSL_FLAT_HASH_MAP_STUBBED_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ABSL_FLAT_HASH_MAP_STUBBED_H_

#include <cstddef>
#include <cstdint>
#include <functional>
#include <unordered_map>
#include <utility>

// NOTE: This file declares `absl::flat_hash_map` and cannot be compiled
// together with the real implementation. That means this header cannot be
// tested in C++ with GoogleTest.

namespace crubit::test {

// Tells the fake `flat_hash_map` implementation which hasher to use for each
// key type without having to inject specializations of `std::hash`.
template <typename T>
struct hasher_for {
  using type = std::hash<T>;
};

template <typename T>
using hasher_for_t = typename hasher_for<T>::type;

}  // namespace crubit::test

namespace absl {

// A fake implementation of `absl::flat_hash_map` to test code generation
// without the absl dependency, backed by `std::unordered_map`.
template <typename K, typename V>
class flat_hash_map final {
 public:
  using iterator = std::unordered_map<K, V>::iterator;

  flat_hash_map() = default;

  size_t size() const { return map_.size(); }
  size_t capacity() const {
    // Pretend this is a flat map instead of a flexible bucketed map and just
    // say we're always half full. This means the capacity is zero when empty.
    return map_.size() * 2;
  }
  template <typename... Args>
  std::pair<iterator, bool> try_emplace(K&& key, Args&&... args) {
    return map_.try_emplace(std::move(key), std::forward<Args...>(args...));
  }

 private:
  std::unordered_map<K, V, crubit::test::hasher_for_t<K>> map_;
};

}  // namespace absl

namespace crubit::test {

using IntsMap = absl::flat_hash_map<int32_t, uint64_t>;

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

  bool operator==(const Nonunpin&) const = default;

 private:
  int32_t value_;
};

struct NonunpinHash {
  size_t operator()(const Nonunpin& x) const noexcept {
    return std::hash<int32_t>{}(x.value());
  }
};

template <>
struct hasher_for<Nonunpin> {
  using type = NonunpinHash;
};

using NonunpinKeyMap = absl::flat_hash_map<Nonunpin, uint64_t>;
using NonunpinValueMap = absl::flat_hash_map<int32_t, Nonunpin>;
using NonunpinBothMap = absl::flat_hash_map<Nonunpin, Nonunpin>;

class MoveOnly final {
 public:
  explicit MoveOnly(int32_t value) : value_(value) {}
  MoveOnly(const MoveOnly&) = delete;
  MoveOnly& operator=(const MoveOnly&) = delete;
  MoveOnly(MoveOnly&& other) : value_(other.value_) { other.value_ = 0; }
  MoveOnly& operator=(MoveOnly&& other) {
    value_ = other.value_;
    other.value_ = 0;
    return *this;
  }

  int32_t value() const { return value_; }

  bool operator==(const MoveOnly&) const = default;

 private:
  int32_t value_;
};

struct MoveOnlyHash {
  size_t operator()(const MoveOnly& x) const noexcept {
    return std::hash<int32_t>{}(x.value());
  }
};

template <>
struct hasher_for<MoveOnly> {
  using type = MoveOnlyHash;
};

using MoveOnlyMap = absl::flat_hash_map<MoveOnly, MoveOnly>;

}  //  namespace crubit::test

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ABSL_FLAT_HASH_MAP_STUBBED_H_
