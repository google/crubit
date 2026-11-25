// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_INHERITANCE_UPCAST_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_INHERITANCE_UPCAST_H_
#include <stddef.h>
#include <stdint.h>

class Base0 {};

class Base1 {
  int base1_field_ = 1;  // NOLINT(clang-diagnostic-unused-private-field)
};
class Base2 {
  char base2_field_ = 2;  // NOLINT(clang-diagnostic-unused-private-field)
};
class Base3 {
  char base3_field_ = 3;  // NOLINT(clang-diagnostic-unused-private-field)
};

class Base4 : public Base2, public Base3 {
  char base4_field_ = 4;  // NOLINT(clang-diagnostic-unused-private-field)
};

struct Derived final : Base0, Base1, Base4 {
  char derived_field = 5;

  size_t base0_address() const {
    const Base0* base = this;
    return reinterpret_cast<size_t>(base);
  }

  size_t base1_address() const {
    const Base1* base = this;
    return reinterpret_cast<size_t>(base);
  }

  size_t base2_address() const {
    const Base2* base = this;
    return reinterpret_cast<size_t>(base);
  }

  size_t base3_address() const {
    const Base3* base = this;
    return reinterpret_cast<size_t>(base);
  }

  size_t base4_address() const {
    const Base4* base = this;
    return reinterpret_cast<size_t>(base);
  }

 private:
  char private_field;  // NOLINT(clang-diagnostic-unused-private-field)
};

namespace virtual_inheritance {

class VirtualBase2 : public virtual Base1 {};
class VirtualBase3 : public virtual Base1 {};

class VirtualDerived : public virtual VirtualBase2,
                       public virtual VirtualBase3 {
 public:
  size_t base1_address() const {
    const Base1* base = this;
    return reinterpret_cast<size_t>(base);
  }

  size_t base2_address() const {
    const VirtualBase2* base = this;
    return reinterpret_cast<size_t>(base);
  }

  size_t base3_address() const {
    const VirtualBase3* base = this;
    return reinterpret_cast<size_t>(base);
  }
};

struct VirtualBaseWithData {
  int64_t base_value = 100;
  mutable int counter = 0;

  void increment_counter() const { ++counter; }
  int get_counter() const { return counter; }
};

struct DerivedWithFieldAndVirtualBase : public virtual VirtualBaseWithData {
  DerivedWithFieldAndVirtualBase() = default;
  DerivedWithFieldAndVirtualBase(int8_t derived, int64_t base)
      : derived_field(derived) {
    base_value = base;
  }

  size_t base_address() const {
    const VirtualBaseWithData* base = this;
    return reinterpret_cast<size_t>(base);
  }

  int64_t read_base_value() const { return base_value; }

  int8_t derived_field = 42;
};

struct DiamondLeft : public virtual VirtualBaseWithData {
  int left_field = 10;
};

struct DiamondRight : public virtual VirtualBaseWithData {
  int right_field = 20;
};

struct DiamondFinal : public DiamondLeft, public DiamondRight {
  DiamondFinal() = default;
  DiamondFinal(int left, int right, int8_t final_val, int64_t shared)
      : final_field(final_val) {
    left_field = left;
    right_field = right;
    base_value = shared;
  }

  size_t vbase_address() const {
    const VirtualBaseWithData* base = this;
    return reinterpret_cast<size_t>(base);
  }

  size_t left_address() const {
    const DiamondLeft* base = this;
    return reinterpret_cast<size_t>(base);
  }

  size_t right_address() const {
    const DiamondRight* base = this;
    return reinterpret_cast<size_t>(base);
  }

  int8_t final_field = 30;
};

struct alignas(32) OveralignedVirtualBase {
  int64_t data = 77;
};

struct DerivedFromOveralignedVirtualBase
    : public virtual OveralignedVirtualBase {
  size_t base_address() const {
    const OveralignedVirtualBase* base = this;
    return reinterpret_cast<size_t>(base);
  }

  int8_t derived_field = 5;
};

}  // namespace virtual_inheritance

namespace another_namespace {

// This class has the same name as another `VirtualBase2` above (in another
// namespace) to check that upcast thunks have unique names that take the
// namespace into account.
class VirtualBase2 : public virtual Base1 {
 public:
  VirtualBase2() = default;

  size_t base1_address() const {
    const Base1* base = this;
    return reinterpret_cast<size_t>(base);
  }
};

}  // namespace another_namespace

namespace nonunpin_upcast {
class NonUnpinBase {
 public:
  NonUnpinBase() = default;
  NonUnpinBase(const NonUnpinBase&) = delete;
  NonUnpinBase& operator=(const NonUnpinBase&) = delete;

  void non_const_method() {}
};

class NonUnpinDerived : public NonUnpinBase {};
}  // namespace nonunpin_upcast

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_INHERITANCE_UPCAST_H_
