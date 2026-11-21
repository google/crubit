// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_SUPPORT_CC_STD_IMPL_TEST_VARIANT_CC_HELPER_FUNCTIONS_H_
#define CRUBIT_SUPPORT_CC_STD_IMPL_TEST_VARIANT_CC_HELPER_FUNCTIONS_H_

#include <cstdint>
#include <memory>
#include <optional>
#include <string>
#include <variant>
#include <vector>

namespace test_variant {

using TrivialVariant = std::variant<int, float, bool>;
using NonTrivialUnpinVariant =
    std::variant<int, std::vector<int>, std::unique_ptr<int>>;
using NonTrivialNotUnpinVariant = std::variant<int, std::string>;

TrivialVariant return_trivial_variant_int(int i);
TrivialVariant return_trivial_variant_float(float f);
TrivialVariant return_trivial_variant_bool(bool b);
TrivialVariant pass_trivial_variant_by_value(TrivialVariant v);
int take_trivial_variant_by_value(TrivialVariant v);
int take_trivial_variant_by_const_ref(const TrivialVariant& v);
void modify_trivial_variant(TrivialVariant& v);

NonTrivialUnpinVariant return_non_trivial_unpin_variant_int(int i);
NonTrivialUnpinVariant return_non_trivial_unpin_variant_vec(std::vector<int> v);
NonTrivialUnpinVariant return_non_trivial_unpin_variant_unique_ptr(
    std::unique_ptr<int> p);
int take_non_trivial_unpin_variant_by_value(NonTrivialUnpinVariant v);
int take_non_trivial_unpin_variant_by_const_ref(
    const NonTrivialUnpinVariant& v);

NonTrivialNotUnpinVariant return_non_trivial_not_unpin_variant_int(int i);
NonTrivialNotUnpinVariant return_non_trivial_not_unpin_variant_string();
int take_non_trivial_not_unpin_variant_by_const_ref(
    const NonTrivialNotUnpinVariant& v);

struct DestructorCounter {
  int* count;
  DestructorCounter(int* count) : count(count) {}
  DestructorCounter(const DestructorCounter& other) : count(other.count) {}
  DestructorCounter(DestructorCounter&& other) : count(other.count) {
    other.count = nullptr;
  }
  ~DestructorCounter() {
    if (count) {
      ++(*count);
    }
  }
};

using DestructorVariant = std::variant<int, DestructorCounter>;

DestructorVariant return_destructor_variant(int* count);
int count_destructors(DestructorVariant v);

using Coproduct17 = std::variant<int, int, int, int, int, int, int, int, int,
                                 int, int, int, int, int, int, int, double>;

Coproduct17 return_coproduct17_double(double d);
double take_coproduct17_by_value(Coproduct17 v);

using DuplicateVariant = std::variant<int, int>;
DuplicateVariant return_duplicate_variant_0(int i);
DuplicateVariant return_duplicate_variant_1(int i);

using SmallVariant = std::variant<uint8_t, bool>;
SmallVariant return_small_variant_bool(bool b);
bool take_small_variant_by_value(SmallVariant v);

}  // namespace test_variant

#endif  // CRUBIT_SUPPORT_CC_STD_IMPL_TEST_VARIANT_CC_HELPER_FUNCTIONS_H_
