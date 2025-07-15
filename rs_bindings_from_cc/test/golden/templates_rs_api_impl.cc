// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:templates_cc

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/golden/templates.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

static_assert(sizeof(struct DifferentScope) == 1);
static_assert(alignof(struct DifferentScope) == 1);

extern "C" void __rust_thunk___ZN14DifferentScopeC1Ev(
    struct DifferentScope* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(struct test_namespace_bindings::TemplateParam) == 1);
static_assert(alignof(struct test_namespace_bindings::TemplateParam) == 1);

extern "C" void __rust_thunk___ZN23test_namespace_bindings13TemplateParamC1Ev(
    struct test_namespace_bindings::TemplateParam* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(class private_classes::HasPrivateType) == 1);
static_assert(alignof(class private_classes::HasPrivateType) == 1);

static_assert(
    sizeof(class test_namespace_bindings::MyTemplate<DifferentScope>) == 1);
static_assert(
    alignof(class test_namespace_bindings::MyTemplate<DifferentScope>) == 1);

extern "C" void
__rust_thunk___ZN23test_namespace_bindings10MyTemplateI14DifferentScopeEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(
    class test_namespace_bindings::MyTemplate<DifferentScope>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk___ZN23test_namespace_bindings10MyTemplateI14DifferentScopeE6CreateES1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(
    class test_namespace_bindings::MyTemplate<DifferentScope>* __return,
    struct DifferentScope* value) {
  new (__return) auto(
      test_namespace_bindings::MyTemplate<DifferentScope>::Create(
          std::move(*value)));
}

extern "C" const struct DifferentScope*
__rust_thunk___ZNK23test_namespace_bindings10MyTemplateI14DifferentScopeE5valueEv__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(
    const class test_namespace_bindings::MyTemplate<DifferentScope>* __this) {
  return &__this->value();
}

static_assert(sizeof(class test_namespace_bindings::MyTemplate<
                     test_namespace_bindings::TemplateParam>) == 1);
static_assert(alignof(class test_namespace_bindings::MyTemplate<
                      test_namespace_bindings::TemplateParam>) == 1);

extern "C" void
__rust_thunk___ZN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(
    class test_namespace_bindings::MyTemplate<
        test_namespace_bindings::TemplateParam>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk___ZN23test_namespace_bindings10MyTemplateINS_13TemplateParamEE6CreateES1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(
    class test_namespace_bindings::MyTemplate<
        test_namespace_bindings::TemplateParam>* __return,
    struct test_namespace_bindings::TemplateParam* value) {
  new (__return) auto(
      test_namespace_bindings::MyTemplate<
          test_namespace_bindings::TemplateParam>::Create(std::move(*value)));
}

extern "C" const struct test_namespace_bindings::TemplateParam*
__rust_thunk___ZNK23test_namespace_bindings10MyTemplateINS_13TemplateParamEE5valueEv__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(
    const class test_namespace_bindings::MyTemplate<
        test_namespace_bindings::TemplateParam>* __this) {
  return &__this->value();
}

static_assert(CRUBIT_SIZEOF(class test_namespace_bindings::MyTemplate<int>) ==
              4);
static_assert(alignof(class test_namespace_bindings::MyTemplate<int>) == 4);

extern "C" void
__rust_thunk___ZN23test_namespace_bindings10MyTemplateIiEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(
    class test_namespace_bindings::MyTemplate<int>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk___ZN23test_namespace_bindings10MyTemplateIiE6CreateEi__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(
    class test_namespace_bindings::MyTemplate<int>* __return, int value) {
  new (__return) auto(test_namespace_bindings::MyTemplate<int>::Create(value));
}

extern "C" int const*
__rust_thunk___ZNK23test_namespace_bindings10MyTemplateIiE5valueEv__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(
    const class test_namespace_bindings::MyTemplate<int>* __this) {
  return &__this->value();
}

static_assert(
    CRUBIT_SIZEOF(
        struct test_namespace_bindings::TemplateWithTwoParams<
            test_namespace_bindings::TemplateWithTwoParams<int, int>, int>) ==
    12);
static_assert(
    alignof(struct test_namespace_bindings::TemplateWithTwoParams<
            test_namespace_bindings::TemplateWithTwoParams<int, int>, int>) ==
    4);
static_assert(
    CRUBIT_OFFSET_OF(
        value1,
        struct test_namespace_bindings::TemplateWithTwoParams<
            test_namespace_bindings::TemplateWithTwoParams<int, int>, int>) ==
    0);
static_assert(
    CRUBIT_OFFSET_OF(
        value2,
        struct test_namespace_bindings::TemplateWithTwoParams<
            test_namespace_bindings::TemplateWithTwoParams<int, int>, int>) ==
    8);

extern "C" void
__rust_thunk___ZN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(
    struct test_namespace_bindings::TemplateWithTwoParams<
        test_namespace_bindings::TemplateWithTwoParams<int, int>, int>*
        __this) {
  crubit::construct_at(__this);
}

static_assert(
    CRUBIT_SIZEOF(
        struct test_namespace_bindings::TemplateWithTwoParams<int, float>) ==
    8);
static_assert(
    alignof(
        struct test_namespace_bindings::TemplateWithTwoParams<int, float>) ==
    4);
static_assert(
    CRUBIT_OFFSET_OF(
        value1,
        struct test_namespace_bindings::TemplateWithTwoParams<int, float>) ==
    0);
static_assert(
    CRUBIT_OFFSET_OF(
        value2,
        struct test_namespace_bindings::TemplateWithTwoParams<int, float>) ==
    4);

extern "C" void
__rust_thunk___ZN23test_namespace_bindings21TemplateWithTwoParamsIifEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(
    struct test_namespace_bindings::TemplateWithTwoParams<int, float>* __this) {
  crubit::construct_at(__this);
}

static_assert(
    CRUBIT_SIZEOF(
        struct test_namespace_bindings::TemplateWithTwoParams<int, int>) == 8);
static_assert(
    alignof(struct test_namespace_bindings::TemplateWithTwoParams<int, int>) ==
    4);
static_assert(
    CRUBIT_OFFSET_OF(
        value1,
        struct test_namespace_bindings::TemplateWithTwoParams<int, int>) == 0);
static_assert(
    CRUBIT_OFFSET_OF(
        value2,
        struct test_namespace_bindings::TemplateWithTwoParams<int, int>) == 4);

extern "C" void
__rust_thunk___ZN23test_namespace_bindings21TemplateWithTwoParamsIiiEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(
    struct test_namespace_bindings::TemplateWithTwoParams<int, int>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(struct test_namespace_bindings::MyStruct<char>) == 1);
static_assert(alignof(struct test_namespace_bindings::MyStruct<char>) == 1);

extern "C" void
__rust_thunk___ZN23test_namespace_bindings8MyStructIcEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(
    struct test_namespace_bindings::MyStruct<char>* __this) {
  crubit::construct_at(__this);
}

static_assert(
    sizeof(struct MyTopLevelTemplate<test_namespace_bindings::TemplateParam>) ==
    1);
static_assert(
    alignof(
        struct MyTopLevelTemplate<test_namespace_bindings::TemplateParam>) ==
    1);
static_assert(
    CRUBIT_OFFSET_OF(
        value,
        struct MyTopLevelTemplate<test_namespace_bindings::TemplateParam>) ==
    0);

extern "C" void
__rust_thunk___ZN18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(
    struct MyTopLevelTemplate<test_namespace_bindings::TemplateParam>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(class template_template_params::MyTemplate<
                     template_template_params::Policy>) == 1);
static_assert(alignof(class template_template_params::MyTemplate<
                      template_template_params::Policy>) == 1);

extern "C" void
__rust_thunk___ZN24template_template_params10MyTemplateINS_6PolicyEEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(
    class template_template_params::MyTemplate<
        template_template_params::Policy>* __this) {
  crubit::construct_at(__this);
}

extern "C" int
__rust_thunk___ZN24template_template_params10MyTemplateINS_6PolicyEE9GetPolicyEv__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc() {
  return template_template_params::MyTemplate<
      template_template_params::Policy>::GetPolicy();
}

#pragma clang diagnostic pop
