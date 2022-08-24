// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <cstddef>
#include <memory>

#include "rs_bindings_from_cc/support/cxx20_backports.h"
#include "rs_bindings_from_cc/support/offsetof.h"
#include "rs_bindings_from_cc/test/golden/templates.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"
extern "C" void __rust_thunk___ZN14DifferentScopeC1Ev(
    struct DifferentScope* __this) {
  crubit::construct_at(__this);
}
extern "C" void __rust_thunk___ZN14DifferentScopeC1EOS_(
    struct DifferentScope* __this, struct DifferentScope* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}
extern "C" void
__rust_thunk___ZN23test_namespace_bindings10MyTemplateI14DifferentScopeEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(
    class test_namespace_bindings::MyTemplate<DifferentScope>* __this) {
  crubit::construct_at(__this);
}
extern "C" void
__rust_thunk___ZN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(
    class test_namespace_bindings::MyTemplate<
        test_namespace_bindings::TemplateParam>* __this) {
  crubit::construct_at(__this);
}
extern "C" void
__rust_thunk___ZN23test_namespace_bindings10MyTemplateIiEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(
    class test_namespace_bindings::MyTemplate<int>* __this) {
  crubit::construct_at(__this);
}
extern "C" void
__rust_thunk___ZN23test_namespace_bindings10MyTemplateI14DifferentScopeEC1EOS2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(
    class test_namespace_bindings::MyTemplate<DifferentScope>* __this,
    class test_namespace_bindings::MyTemplate<DifferentScope>* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}
extern "C" void
__rust_thunk___ZN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEC1EOS2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(
    class test_namespace_bindings::MyTemplate<
        test_namespace_bindings::TemplateParam>* __this,
    class test_namespace_bindings::MyTemplate<
        test_namespace_bindings::TemplateParam>* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}
extern "C" void
__rust_thunk___ZN23test_namespace_bindings10MyTemplateIiEC1EOS1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(
    class test_namespace_bindings::MyTemplate<int>* __this,
    class test_namespace_bindings::MyTemplate<int>* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}
extern "C" class test_namespace_bindings::MyTemplate<DifferentScope>
__rust_thunk___ZN23test_namespace_bindings10MyTemplateI14DifferentScopeE6CreateES1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(
    struct DifferentScope value) {
  return test_namespace_bindings::MyTemplate<DifferentScope>::Create(value);
} extern "C" class test_namespace_bindings::MyTemplate<
    test_namespace_bindings::TemplateParam>
__rust_thunk___ZN23test_namespace_bindings10MyTemplateINS_13TemplateParamEE6CreateES1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(
    struct test_namespace_bindings::TemplateParam value) {
  return test_namespace_bindings::MyTemplate<
      test_namespace_bindings::TemplateParam>::Create(value);
} extern "C" class test_namespace_bindings::MyTemplate<int>
__rust_thunk___ZN23test_namespace_bindings10MyTemplateIiE6CreateEi__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(
    int value) {
  return test_namespace_bindings::MyTemplate<int>::Create(value);
} extern "C" const struct DifferentScope*
__rust_thunk___ZNK23test_namespace_bindings10MyTemplateI14DifferentScopeE5valueEv__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(
    const class test_namespace_bindings::MyTemplate<DifferentScope>* __this) {
  return &__this->value();
}
extern "C" const struct test_namespace_bindings::TemplateParam*
__rust_thunk___ZNK23test_namespace_bindings10MyTemplateINS_13TemplateParamEE5valueEv__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(
    const class test_namespace_bindings::MyTemplate<
        test_namespace_bindings::TemplateParam>* __this) {
  return &__this->value();
}
extern "C" int const*
__rust_thunk___ZNK23test_namespace_bindings10MyTemplateIiE5valueEv__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(
    const class test_namespace_bindings::MyTemplate<int>* __this) {
  return &__this->value();
}
extern "C" void __rust_thunk___ZN23test_namespace_bindings13TemplateParamC1Ev(
    struct test_namespace_bindings::TemplateParam* __this) {
  crubit::construct_at(__this);
}
extern "C" void
__rust_thunk___ZN23test_namespace_bindings13TemplateParamC1EOS0_(
    struct test_namespace_bindings::TemplateParam* __this,
    struct test_namespace_bindings::TemplateParam* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}
extern "C" void
__rust_thunk___ZN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(
    struct test_namespace_bindings::TemplateWithTwoParams<
        test_namespace_bindings::TemplateWithTwoParams<int, int>, int>*
        __this) {
  crubit::construct_at(__this);
}
extern "C" void
__rust_thunk___ZN23test_namespace_bindings21TemplateWithTwoParamsIifEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(
    struct test_namespace_bindings::TemplateWithTwoParams<int, float>* __this) {
  crubit::construct_at(__this);
}
extern "C" void
__rust_thunk___ZN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEC1EOS2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(
    struct test_namespace_bindings::TemplateWithTwoParams<
        test_namespace_bindings::TemplateWithTwoParams<int, int>, int>* __this,
    struct test_namespace_bindings::TemplateWithTwoParams<
        test_namespace_bindings::TemplateWithTwoParams<int, int>, int>*
        __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}
extern "C" void
__rust_thunk___ZN23test_namespace_bindings21TemplateWithTwoParamsIifEC1EOS1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(
    struct test_namespace_bindings::TemplateWithTwoParams<int, float>* __this,
    struct test_namespace_bindings::TemplateWithTwoParams<int, float>*
        __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}
extern "C" void
__rust_thunk___ZN23test_namespace_bindings8MyStructIcEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(
    struct test_namespace_bindings::MyStruct<char>* __this) {
  crubit::construct_at(__this);
}
extern "C" void
__rust_thunk___ZN23test_namespace_bindings8MyStructIcEC1ERKS1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(
    struct test_namespace_bindings::MyStruct<char>* __this,
    const struct test_namespace_bindings::MyStruct<char>* __param_0) {
  crubit::construct_at(__this, *__param_0);
}
extern "C" void
__rust_thunk___ZN23test_namespace_bindings8MyStructIcEC1EOS1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(
    struct test_namespace_bindings::MyStruct<char>* __this,
    struct test_namespace_bindings::MyStruct<char>* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}
extern "C" struct test_namespace_bindings::MyStruct<char>*
__rust_thunk___ZN23test_namespace_bindings8MyStructIcEaSERKS1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(
    struct test_namespace_bindings::MyStruct<char>* __this,
    const struct test_namespace_bindings::MyStruct<char>* __param_0) {
  return &__this->operator=(*__param_0);
} extern "C" struct test_namespace_bindings::MyStruct<char>*
__rust_thunk___ZN23test_namespace_bindings8MyStructIcEaSEOS1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(
    struct test_namespace_bindings::MyStruct<char>* __this,
    struct test_namespace_bindings::MyStruct<char>* __param_0) {
  return &__this->operator=(std::move(*__param_0));
} extern "C" void
__rust_thunk___ZN18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(
    struct MyTopLevelTemplate<test_namespace_bindings::TemplateParam>* __this) {
  crubit::construct_at(__this);
}
extern "C" void
__rust_thunk___ZN18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEEC1EOS2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fcc(
    struct MyTopLevelTemplate<test_namespace_bindings::TemplateParam>* __this,
    struct MyTopLevelTemplate<test_namespace_bindings::TemplateParam>*
        __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

static_assert(sizeof(struct DifferentScope) == 1);
static_assert(alignof(struct DifferentScope) == 1);

static_assert(
    sizeof(class test_namespace_bindings::MyTemplate<DifferentScope>) == 1);
static_assert(
    alignof(class test_namespace_bindings::MyTemplate<DifferentScope>) == 1);

static_assert(sizeof(class test_namespace_bindings::MyTemplate<
                     test_namespace_bindings::TemplateParam>) == 1);
static_assert(alignof(class test_namespace_bindings::MyTemplate<
                      test_namespace_bindings::TemplateParam>) == 1);

static_assert(sizeof(class test_namespace_bindings::MyTemplate<int>) == 4);
static_assert(alignof(class test_namespace_bindings::MyTemplate<int>) == 4);

static_assert(sizeof(struct test_namespace_bindings::TemplateParam) == 1);
static_assert(alignof(struct test_namespace_bindings::TemplateParam) == 1);

static_assert(
    sizeof(struct test_namespace_bindings::TemplateWithTwoParams<
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

static_assert(
    sizeof(struct test_namespace_bindings::TemplateWithTwoParams<int, float>) ==
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

static_assert(sizeof(struct test_namespace_bindings::MyStruct<char>) == 1);
static_assert(alignof(struct test_namespace_bindings::MyStruct<char>) == 1);

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

#pragma clang diagnostic pop
