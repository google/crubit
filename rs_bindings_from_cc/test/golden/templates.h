// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_TEMPLATES_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_TEMPLATES_H_

#pragma clang lifetime_elision

struct DifferentScope {};

namespace test_namespace_bindings {

template <typename T>
class MyTemplate {
 public:
  static MyTemplate Create(T value) {
    MyTemplate result;
    result.value_ = value;
    return result;
  }

  const T& value() const { return value_; }

 private:
  T value_;
};

using MyTypeAlias = MyTemplate<int>;
using OtherTypeAliasInSameTarget = MyTemplate<int>;

struct TemplateParam {};
using TemplateWithStructTemplateParam = MyTemplate<TemplateParam>;
using ParamFromDifferentScope = MyTemplate<DifferentScope>;

template <typename T1, typename T2>
struct TemplateWithTwoParams {
  T1 value1;
  T2 value2;
};

using AliasToTemplateWithTwoParams = TemplateWithTwoParams<int, float>;

using AliasToTemplateOfATemplate =
    TemplateWithTwoParams<TemplateWithTwoParams<int, int>, int>;

}  // namespace test_namespace_bindings

template <typename T>
struct MyTopLevelTemplate {
  T value;
};

using TopLevelTemplateWithNonTopLevelParam =
    MyTopLevelTemplate<test_namespace_bindings::TemplateParam>;

template <>
struct MyTopLevelTemplate<int>;

void processForwardDeclaredSpecialization(MyTopLevelTemplate<int>* i);

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_TEMPLATES_H_
