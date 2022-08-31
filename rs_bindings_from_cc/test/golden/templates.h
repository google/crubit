// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_TEMPLATES_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_TEMPLATES_H_

#pragma clang lifetime_elision

struct DifferentScope final {};

namespace test_namespace_bindings {

template <typename T>
class MyTemplate final {
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

struct TemplateParam final {};
using TemplateWithStructTemplateParam = MyTemplate<TemplateParam>;
using ParamFromDifferentScope = MyTemplate<DifferentScope>;

template <typename T1, typename T2>
struct TemplateWithTwoParams final {
  T1 value1;
  T2 value2;
};

using AliasToTemplateWithTwoParams = TemplateWithTwoParams<int, float>;

using AliasToTemplateOfATemplate =
    TemplateWithTwoParams<TemplateWithTwoParams<int, int>, int>;

template <typename T>
struct MyStruct {
  T t;
};

template <>
struct MyStruct<char> {};

}  // namespace test_namespace_bindings

template <typename T>
struct MyTopLevelTemplate final {
  T value;
};

using TopLevelTemplateWithNonTopLevelParam =
    MyTopLevelTemplate<test_namespace_bindings::TemplateParam>;

template <>
struct MyTopLevelTemplate<int>;

void processForwardDeclaredSpecialization(MyTopLevelTemplate<int>* i);

namespace template_template_params {

template <typename TPolicyType>
struct Policy {
  static constexpr TPolicyType policy = TPolicyType();
};

template <>
struct Policy<int> {
  static constexpr int policy = 42;
};

template <template <class> class TPolicy>
class MyTemplate {
 public:
  static int GetPolicy() { return TPolicy<int>::policy; }
};

using MyTypeAlias = MyTemplate<Policy>;

}  // namespace template_template_params

namespace forward_declared_template {

// This namespaces is a regression test for b/244227110 that is based on
// `<iosfwd>`:
// - `ForwardDeclaredTemplate` corresponds roughly to the `basic_ios` class
//   template.
// - `TypeAliasToForwardDeclaredTemplate` corresponds toughtly to the
//   `typedef basic_ios<char> ios` type alias.

template <typename T>
class ForwardDeclaredTemplate;

using TypeAliasToForwardDeclaredTemplate = ForwardDeclaredTemplate<int>;

}  // namespace forward_declared_template

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_TEMPLATES_H_
