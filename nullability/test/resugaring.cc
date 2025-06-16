// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests for recovery of nullability type information that is lost in the AST.

#include "nullability_test.h"

template <typename T>
T Instance = {};
template <typename T>
struct StaticWrapper {
  constexpr static T Value = {};
};
template <typename T>
struct Wrapper {
  T Value = {};
  T get() { return Value; }
};

TEST void testDeclRefExpr() {
  nonnull(Instance<int *_Nonnull>);
  nonnull(StaticWrapper<int *_Nonnull>::Value);
}

TEST void testMemberExpr(Wrapper<int *_Nonnull> &W) {
  nonnull(W.Value);
  nonnull(W.get());

  struct Derived : Wrapper<char>, Wrapper<int *> {
  } D;
  unknown(D.Wrapper<int *_Nonnull>::Value);  // TODO(b/332558689): nonnull
}

TEST void testCXXTemporaryExpr() {
  type<Wrapper<int *_Null_unspecified>>(  // TODO(b/332562229): nonnull
      Wrapper<int *_Nonnull>{});
}

struct TemplateWrapper {
  template <typename T>
  T get();
};
TEST void testMemberTemplate(TemplateWrapper &s) {
  unknown(s.get<int *>());
  nonnull(s.get<int *_Nonnull>());
  nullable(s.get<int *_Nullable>());
}

namespace template_already_has_nullability {

template <typename T>
struct StaticWrapperAnnotated {
  constexpr static _Nonnull T Value = {};
};

struct Annotated {
  template <typename T>
  _Nonnull T get_nonnull();
  template <typename T>
  _Nullable T get_nullable();
};

TEST void testDeclRefExprAlreadyAnnotated() {
  nonnull(StaticWrapperAnnotated<int *>::Value);
  nonnull(StaticWrapperAnnotated<int *_Nonnull>::Value);
}

TEST void testMemberExprAlreadyAnnotated(Annotated &s) {
  nonnull(s.get_nonnull<int *>());
  nonnull(s.get_nonnull<int *_Nonnull>());
}

TEST void testMemberExprAlreadyAnnotatedConflict(Annotated &s) {
  // We may want to warn about such conflicts, but for now we pick the
  // annotation from the type (the outermost annotation).
  nullable(s.get_nullable<int *_Nonnull>());
}

TEST void testMemberExprAlreadyAnnotatedMultipleStars(Annotated &s) {
  nonnull(s.get_nonnull<int *_Nullable *_Nullable *_Nonnull>());
}

}  // namespace template_already_has_nullability

namespace variable_template {

template <class T>
T VarTempl = {};
TEST void testVariableTemplate() {
  type<int *_Nullable>(VarTempl<int *_Nullable>);
}

}  // namespace variable_template

namespace variable_template_explicit_specialization {

template <class T>
bool VarTempl = true;
template <>
int *VarTempl<int *> = nullptr;
TEST void testVariableTemplateExplicitSpecialization() {
  // The type of the specialized variable is unrelated to the template argument
  // type, so the type of the expression has unknown nullability.
  type<int *_Null_unspecified>(VarTempl<int *_Nullable>);
}

}  // namespace variable_template_explicit_specialization
