// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "support/cc_std_impl/test/variant/cc_helper_functions.h"

namespace test_variant {

TrivialVariant return_trivial_variant_int(int i) { return i; }
TrivialVariant return_trivial_variant_float(float f) { return f; }
TrivialVariant return_trivial_variant_bool(bool b) { return b; }

TrivialVariant pass_trivial_variant_by_value(TrivialVariant v) { return v; }

int take_trivial_variant_by_value(TrivialVariant v) {
  if (std::holds_alternative<int>(v)) return std::get<int>(v);
  if (std::holds_alternative<float>(v))
    return static_cast<int>(std::get<float>(v));
  if (std::holds_alternative<bool>(v)) return std::get<bool>(v) ? 1 : 0;
  return -1;
}

int take_trivial_variant_by_const_ref(const TrivialVariant& v) {
  if (std::holds_alternative<int>(v)) return std::get<int>(v);
  if (std::holds_alternative<float>(v))
    return static_cast<int>(std::get<float>(v));
  if (std::holds_alternative<bool>(v)) return std::get<bool>(v) ? 1 : 0;
  return -1;
}

void modify_trivial_variant(TrivialVariant& v) {
  if (std::holds_alternative<int>(v))
    v = std::get<int>(v) + 1;
  else if (std::holds_alternative<float>(v))
    v = std::get<float>(v) + 1.0f;
  else if (std::holds_alternative<bool>(v))
    v = !std::get<bool>(v);
}

NonTrivialUnpinVariant return_non_trivial_unpin_variant_int(int i) { return i; }
NonTrivialUnpinVariant return_non_trivial_unpin_variant_vec(
    std::vector<int> v) {
  return v;
}
NonTrivialUnpinVariant return_non_trivial_unpin_variant_unique_ptr(
    std::unique_ptr<int> p) {
  return std::move(p);
}

int take_non_trivial_unpin_variant_by_value(NonTrivialUnpinVariant v) {
  return take_non_trivial_unpin_variant_by_const_ref(v);
}

int take_non_trivial_unpin_variant_by_const_ref(
    const NonTrivialUnpinVariant& v) {
  if (std::holds_alternative<int>(v)) return std::get<int>(v);
  if (std::holds_alternative<std::vector<int>>(v)) {
    int sum = 0;
    for (int i : std::get<std::vector<int>>(v)) {
      sum += i;
    }
    return sum;
  }
  if (std::holds_alternative<std::unique_ptr<int>>(v)) {
    return *std::get<std::unique_ptr<int>>(v);
  }
  return -1;
}

NonTrivialNotUnpinVariant return_non_trivial_not_unpin_variant_int(int i) {
  return i;
}
NonTrivialNotUnpinVariant return_non_trivial_not_unpin_variant_string() {
  return "hello";
}

int take_non_trivial_not_unpin_variant_by_const_ref(
    const NonTrivialNotUnpinVariant& v) {
  if (std::holds_alternative<int>(v)) return std::get<int>(v);
  if (std::holds_alternative<std::string>(v))
    return std::get<std::string>(v).size();
  return -1;
}

DestructorVariant return_destructor_variant(int* count) {
  return DestructorCounter(count);
}

int count_destructors(DestructorVariant v) {
  if (std::holds_alternative<DestructorCounter>(v)) return 1;
  return 0;
}

Coproduct17 return_coproduct17_double(double d) {
  return Coproduct17(std::in_place_index<16>, d);
}
double take_coproduct17_by_value(Coproduct17 v) { return std::get<16>(v); }

DuplicateVariant return_duplicate_variant_0(int i) {
  return DuplicateVariant(std::in_place_index<0>, i);
}
DuplicateVariant return_duplicate_variant_1(int i) {
  return DuplicateVariant(std::in_place_index<1>, i);
}

SmallVariant return_small_variant_bool(bool b) {
  return SmallVariant(std::in_place_index<1>, b);
}

bool take_small_variant_by_value(SmallVariant v) {
  return std::holds_alternative<bool>(v) && std::get<bool>(v);
}

}  // namespace test_variant
