// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Declare the namespace we use for C++ bindings to the Rust `std` crate to
// catch any uses of `std` from inside the `rs` namespace that were meant to
// refer to top-level `std`.
namespace rs::std {}

#include "support/rs_std/iterator_adapter.h"

#include <vector>

#include "gmock/gmock.h"
#include "gtest/gtest.h"

namespace {

using ::testing::ElementsAre;

// Minimal stand-in for an `rs::Option<T&>` specialization, which is what
// `next()` returns for reference items under the `option_references` Crubit
// feature.
template <typename T>
class FakeOption;

template <typename T>
class FakeOption<T&> {
 public:
  FakeOption() = default;
  explicit FakeOption(T& value) : ptr_(&value) {}
  bool has_value() const { return ptr_ != nullptr; }
  T& operator*() const { return *ptr_; }

 private:
  T* ptr_ = nullptr;
};

// Iterators over the `int`s in `[begin, end)`, whose `next()` returns
// `FakeOption<const int&>`, with `Item` being `const int&` or `const int*`
// (depending on the `iterator_item_references` Crubit feature).
struct RefItemIter {
  const int* begin;
  const int* end;
};
struct PtrItemIter {
  const int* begin;
  const int* end;
};

template <typename Iter>
FakeOption<const int&> NextInt(Iter& self) {
  if (self.begin == self.end) {
    return FakeOption<const int&>();
  }
  return FakeOption<const int&>(*self.begin++);
}

}  // namespace

template <>
struct rs::impl<RefItemIter, rs::core::iter::Iterator> {
  static constexpr bool kIsImplemented = true;
  using Item = const int&;
  static FakeOption<const int&> next(RefItemIter& self) {
    return NextInt(self);
  }
};

template <>
struct rs::impl<PtrItemIter, rs::core::iter::Iterator> {
  static constexpr bool kIsImplemented = true;
  using Item = const int*;
  static FakeOption<const int&> next(PtrItemIter& self) {
    return NextInt(self);
  }
};

namespace {

TEST(IteratorAdapterTest, Compiles) {}

TEST(IteratorAdapterTest, OptionalReferenceWithReferenceItems) {
  const std::vector<int> values = {1, 2, 3};
  std::vector<const int*> addresses;
  for (auto it = rs::IteratorAdapter(
           RefItemIter{values.data(), values.data() + values.size()});
       it != rs::IteratorEnd(); ++it) {
    const int& item = *it;
    addresses.push_back(&item);
  }
  EXPECT_THAT(addresses, ElementsAre(&values[0], &values[1], &values[2]));
}

TEST(IteratorAdapterTest, OptionalReferenceWithPointerItems) {
  const std::vector<int> values = {1, 2, 3};
  std::vector<const int*> addresses;
  for (auto it = rs::IteratorAdapter(
           PtrItemIter{values.data(), values.data() + values.size()});
       it != rs::IteratorEnd(); ++it) {
    const int* item = *it;
    addresses.push_back(item);
  }
  EXPECT_THAT(addresses, ElementsAre(&values[0], &values[1], &values[2]));
}

TEST(IteratorAdapterTest, OptionalReferenceThroughMutableReference) {
  const std::vector<int> values = {1, 2, 3};
  RefItemIter iter{values.data(), values.data() + values.size()};
  std::vector<const int*> addresses;
  // Uses `impl<T*, rs::core::iter::Iterator>`, which mirrors
  // `impl<I: Iterator> Iterator for &mut I`.
  for (auto it = rs::IteratorAdapter(&iter); it != rs::IteratorEnd(); ++it) {
    const int& item = *it;
    addresses.push_back(&item);
  }
  EXPECT_THAT(addresses, ElementsAre(&values[0], &values[1], &values[2]));
}

}  // namespace
