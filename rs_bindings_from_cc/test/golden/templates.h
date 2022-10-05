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

// Explicit class template specialization with definition should not be imported
// unless also instantiated.
template <>
struct MyStruct<bool> {};

// Explicit class template specialization with definition should be imported
// even when not instantiated if there is a type alias for it.
template <>
struct MyStruct<char> {};
using MyCharStruct = MyStruct<char>;

// Forward declared explicit class template specialization should be imported
// so the forward declaration code is generated (`forward_declare!`).
template <>
struct MyStruct<int>;

// Explicit class template instantiation definition is imported similarly to
// how implicit typedeffed instantiations are.
template class MyStruct<float>;

// Explicit class template instantiation declaration is not handled (yet?)
// TODO(b/245467707): Consider handling these as a build speed/ergonomic
// optimization.
extern template class MyStruct<double>;

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

// This namespace is a regression test for b/244227110 that is based on
// `<iosfwd>`:
// - `ForwardDeclaredTemplate` corresponds roughly to the `basic_ios` class
//   template.
// - `TypeAliasToForwardDeclaredTemplate` corresponds toughtly to the
//   `typedef basic_ios<char> ios` type alias.
namespace forward_declared_template {

template <typename T>
class ForwardDeclaredTemplate;

using TypeAliasToForwardDeclaredTemplate = ForwardDeclaredTemplate<int>;

}  // namespace forward_declared_template

namespace private_classes {

class HasPrivateType {
 private:
  struct PrivateType {
    using Foo = test_namespace_bindings::MyTemplate<PrivateType>;
    Foo* get();
  };

 protected:
  HasPrivateType(test_namespace_bindings::MyTemplate<PrivateType> x) {}
};
}  // namespace private_classes

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_TEMPLATES_H_
