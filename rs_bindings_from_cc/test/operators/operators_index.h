// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_OPERATORS_OPERATORS_INDEX_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_OPERATORS_OPERATORS_INDEX_H_

namespace crubit::test {

struct ItemUnpin final {
  int value = 0;
};

struct ItemNonUnpin final {
  int value = 0;
  // NOLINTNEXTLINE(modernize-use-equals-default)
  ~ItemNonUnpin() {};
};

class ContainerUnpinItemUnpin final {
 public:
  ContainerUnpinItemUnpin() = default;
  const ItemUnpin& operator[](unsigned int index) const {
    return items_[index];
  }
  ItemUnpin& operator[](unsigned int index) { return items_[index]; }

 private:
  ItemUnpin items_[10];
};

class ContainerUnpinItemNonUnpin final {
 public:
  ContainerUnpinItemNonUnpin() : items_(items_storage_) {}
  explicit ContainerUnpinItemNonUnpin(ItemNonUnpin* items) : items_(items) {}

  const ItemNonUnpin& operator[](unsigned int index) const {
    return items_[index];
  }
  ItemNonUnpin& operator[](unsigned int index) { return items_[index]; }

 public:
  ItemNonUnpin items_storage_[10];
  ItemNonUnpin* items_;
};

class ContainerNonUnpinItemUnpin final {
 public:
  ContainerNonUnpinItemUnpin() = default;
  // NOLINTNEXTLINE(modernize-use-equals-default)
  ~ContainerNonUnpinItemUnpin() {};

  const ItemUnpin& operator[](unsigned int index) const {
    return items_[index];
  }
  ItemUnpin& operator[](unsigned int index) { return items_[index]; }

 private:
  ItemUnpin items_[10];
};

class ContainerNonUnpinItemNonUnpin final {
 public:
  ContainerNonUnpinItemNonUnpin() = default;
  // NOLINTNEXTLINE(modernize-use-equals-default)
  ~ContainerNonUnpinItemNonUnpin() {};

  const ItemNonUnpin& operator[](unsigned int index) const {
    return items_[index];
  }
  ItemNonUnpin& operator[](unsigned int index) { return items_[index]; }

 private:
  ItemNonUnpin items_[10];
};

struct ContainerValue final {
  int value = 42;
  // TODO: b/242938276 - support returning values from const operator[].
  int operator[](unsigned int index) const { return value; }
};

// R-value qualified overloads are not supported.
struct ContainerRvalue final {
  int value = 42;
  int& operator[](unsigned int index) && { return value; }
};

// The following two cases where we have
// - non-const references returned from const indexing, or
// - const references returned from non-const indexing
// are invalid overload signatures and should not generate bindings.
struct ContainerMutRefFromConst final {
  int value = 42;
  int& operator[](unsigned int index) const { return const_cast<int&>(value); }
};

struct ContainerConstRefFromMut final {
  int value = 42;
  const int& operator[](unsigned int index) { return value; }
};

}  // namespace crubit::test

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_OPERATORS_OPERATORS_INDEX_H_
