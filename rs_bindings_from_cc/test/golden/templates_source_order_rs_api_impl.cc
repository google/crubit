// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:templates_source_order_cc

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/golden/templates_source_order.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

static_assert(sizeof(struct TopLevel) == 1);
static_assert(alignof(struct TopLevel) == 1);

extern "C" void __rust_thunk___ZN8TopLevelC1Ev(struct TopLevel* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___ZN8TopLevelC1EOS_(struct TopLevel* __this,
                                                 struct TopLevel* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

extern "C" struct TopLevel* __rust_thunk___ZN8TopLevelaSERKS_(
    struct TopLevel* __this, const struct TopLevel* __param_0) {
  return &__this->operator=(*__param_0);
}

extern "C" struct TopLevel* __rust_thunk___ZN8TopLevelaSEOS_(
    struct TopLevel* __this, struct TopLevel* __param_0) {
  return &__this->operator=(std::move(*__param_0));
}

static_assert(sizeof(struct test_namespace_bindings::Inner) == 1);
static_assert(alignof(struct test_namespace_bindings::Inner) == 1);

extern "C" void __rust_thunk___ZN23test_namespace_bindings5InnerC1Ev(
    struct test_namespace_bindings::Inner* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___ZN23test_namespace_bindings5InnerC1EOS0_(
    struct test_namespace_bindings::Inner* __this,
    struct test_namespace_bindings::Inner* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

extern "C" struct test_namespace_bindings::Inner*
__rust_thunk___ZN23test_namespace_bindings5InneraSERKS0_(
    struct test_namespace_bindings::Inner* __this,
    const struct test_namespace_bindings::Inner* __param_0) {
  return &__this->operator=(*__param_0);
}

extern "C" struct test_namespace_bindings::Inner*
__rust_thunk___ZN23test_namespace_bindings5InneraSEOS0_(
    struct test_namespace_bindings::Inner* __this,
    struct test_namespace_bindings::Inner* __param_0) {
  return &__this->operator=(std::move(*__param_0));
}

static_assert(sizeof(class MyTemplate<TopLevel>) == 1);
static_assert(alignof(class MyTemplate<TopLevel>) == 1);

extern "C" void
__rust_thunk___ZN10MyTemplateI8TopLevelEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<TopLevel>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk___ZN10MyTemplateI8TopLevelEC1EOS1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<TopLevel>* __this, class MyTemplate<TopLevel>* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

extern "C" class MyTemplate<TopLevel>*
__rust_thunk___ZN10MyTemplateI8TopLevelEaSERKS1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<TopLevel>* __this,
    const class MyTemplate<TopLevel>* __param_0) {
  return &__this->operator=(*__param_0);
}

extern "C" class MyTemplate<TopLevel>*
__rust_thunk___ZN10MyTemplateI8TopLevelEaSEOS1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<TopLevel>* __this, class MyTemplate<TopLevel>* __param_0) {
  return &__this->operator=(std::move(*__param_0));
}

extern "C" void
__rust_thunk___ZN10MyTemplateI8TopLevelE8processTES0___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<TopLevel>* __this, struct TopLevel* t) {
  __this->processT(std::move(*t));
}

static_assert(sizeof(class MyTemplate<test_namespace_bindings::Inner>) == 1);
static_assert(alignof(class MyTemplate<test_namespace_bindings::Inner>) == 1);

extern "C" void
__rust_thunk___ZN10MyTemplateIN23test_namespace_bindings5InnerEEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<test_namespace_bindings::Inner>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk___ZN10MyTemplateIN23test_namespace_bindings5InnerEEC1EOS2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<test_namespace_bindings::Inner>* __this,
    class MyTemplate<test_namespace_bindings::Inner>* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

extern "C" class MyTemplate<test_namespace_bindings::Inner>*
__rust_thunk___ZN10MyTemplateIN23test_namespace_bindings5InnerEEaSERKS2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<test_namespace_bindings::Inner>* __this,
    const class MyTemplate<test_namespace_bindings::Inner>* __param_0) {
  return &__this->operator=(*__param_0);
}

extern "C" class MyTemplate<test_namespace_bindings::Inner>*
__rust_thunk___ZN10MyTemplateIN23test_namespace_bindings5InnerEEaSEOS2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<test_namespace_bindings::Inner>* __this,
    class MyTemplate<test_namespace_bindings::Inner>* __param_0) {
  return &__this->operator=(std::move(*__param_0));
}

extern "C" void
__rust_thunk___ZN10MyTemplateIN23test_namespace_bindings5InnerEE8processTES1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<test_namespace_bindings::Inner>* __this,
    struct test_namespace_bindings::Inner* t) {
  __this->processT(std::move(*t));
}

static_assert(sizeof(class MyTemplate<MyTemplate<TopLevel>>) == 1);
static_assert(alignof(class MyTemplate<MyTemplate<TopLevel>>) == 1);

extern "C" void
__rust_thunk___ZN10MyTemplateIS_I8TopLevelEEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<MyTemplate<TopLevel>>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk___ZN10MyTemplateIS_I8TopLevelEEC1EOS2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<MyTemplate<TopLevel>>* __this,
    class MyTemplate<MyTemplate<TopLevel>>* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

extern "C" class MyTemplate<MyTemplate<TopLevel>>*
__rust_thunk___ZN10MyTemplateIS_I8TopLevelEEaSERKS2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<MyTemplate<TopLevel>>* __this,
    const class MyTemplate<MyTemplate<TopLevel>>* __param_0) {
  return &__this->operator=(*__param_0);
}

extern "C" class MyTemplate<MyTemplate<TopLevel>>*
__rust_thunk___ZN10MyTemplateIS_I8TopLevelEEaSEOS2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<MyTemplate<TopLevel>>* __this,
    class MyTemplate<MyTemplate<TopLevel>>* __param_0) {
  return &__this->operator=(std::move(*__param_0));
}

extern "C" void
__rust_thunk___ZN10MyTemplateIS_I8TopLevelEE8processTES1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<MyTemplate<TopLevel>>* __this,
    class MyTemplate<TopLevel>* t) {
  __this->processT(std::move(*t));
}

static_assert(
    sizeof(class MyTemplate<MyTemplate<test_namespace_bindings::Inner>>) == 1);
static_assert(
    alignof(class MyTemplate<MyTemplate<test_namespace_bindings::Inner>>) == 1);

extern "C" void
__rust_thunk___ZN10MyTemplateIS_IN23test_namespace_bindings5InnerEEEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<MyTemplate<test_namespace_bindings::Inner>>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk___ZN10MyTemplateIS_IN23test_namespace_bindings5InnerEEEC1EOS3___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<MyTemplate<test_namespace_bindings::Inner>>* __this,
    class MyTemplate<MyTemplate<test_namespace_bindings::Inner>>* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

extern "C" class MyTemplate<MyTemplate<test_namespace_bindings::Inner>>*
__rust_thunk___ZN10MyTemplateIS_IN23test_namespace_bindings5InnerEEEaSERKS3___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<MyTemplate<test_namespace_bindings::Inner>>* __this,
    const class MyTemplate<MyTemplate<test_namespace_bindings::Inner>>*
        __param_0) {
  return &__this->operator=(*__param_0);
}

extern "C" class MyTemplate<MyTemplate<test_namespace_bindings::Inner>>*
__rust_thunk___ZN10MyTemplateIS_IN23test_namespace_bindings5InnerEEEaSEOS3___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<MyTemplate<test_namespace_bindings::Inner>>* __this,
    class MyTemplate<MyTemplate<test_namespace_bindings::Inner>>* __param_0) {
  return &__this->operator=(std::move(*__param_0));
}

extern "C" void
__rust_thunk___ZN10MyTemplateIS_IN23test_namespace_bindings5InnerEEE8processTES2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<MyTemplate<test_namespace_bindings::Inner>>* __this,
    class MyTemplate<test_namespace_bindings::Inner>* t) {
  __this->processT(std::move(*t));
}

static_assert(sizeof(class MyTemplate<bool>) == 1);
static_assert(alignof(class MyTemplate<bool>) == 1);

extern "C" void
__rust_thunk___ZN10MyTemplateIbEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<bool>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk___ZN10MyTemplateIbEC1EOS0___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<bool>* __this, class MyTemplate<bool>* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

extern "C" class MyTemplate<bool>*
__rust_thunk___ZN10MyTemplateIbEaSERKS0___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<bool>* __this, const class MyTemplate<bool>* __param_0) {
  return &__this->operator=(*__param_0);
}

extern "C" class MyTemplate<bool>*
__rust_thunk___ZN10MyTemplateIbEaSEOS0___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<bool>* __this, class MyTemplate<bool>* __param_0) {
  return &__this->operator=(std::move(*__param_0));
}

extern "C" void
__rust_thunk___ZN10MyTemplateIbE8processTEb__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<bool>* __this, bool t) {
  __this->processT(t);
}

static_assert(sizeof(class MyTemplate<char>) == 1);
static_assert(alignof(class MyTemplate<char>) == 1);

extern "C" void
__rust_thunk___ZN10MyTemplateIcEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<char>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk___ZN10MyTemplateIcEC1EOS0___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<char>* __this, class MyTemplate<char>* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

extern "C" class MyTemplate<char>*
__rust_thunk___ZN10MyTemplateIcEaSERKS0___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<char>* __this, const class MyTemplate<char>* __param_0) {
  return &__this->operator=(*__param_0);
}

extern "C" class MyTemplate<char>*
__rust_thunk___ZN10MyTemplateIcEaSEOS0___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<char>* __this, class MyTemplate<char>* __param_0) {
  return &__this->operator=(std::move(*__param_0));
}

extern "C" void
__rust_thunk___ZN10MyTemplateIcE8processTEc__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<char>* __this, char t) {
  __this->processT(t);
}

static_assert(CRUBIT_SIZEOF(class MyTemplate<double>) == 8);
static_assert(alignof(class MyTemplate<double>) == 8);

extern "C" void
__rust_thunk___ZN10MyTemplateIdEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<double>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk___ZN10MyTemplateIdEC1EOS0___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<double>* __this, class MyTemplate<double>* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

extern "C" class MyTemplate<double>*
__rust_thunk___ZN10MyTemplateIdEaSERKS0___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<double>* __this,
    const class MyTemplate<double>* __param_0) {
  return &__this->operator=(*__param_0);
}

extern "C" class MyTemplate<double>*
__rust_thunk___ZN10MyTemplateIdEaSEOS0___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<double>* __this, class MyTemplate<double>* __param_0) {
  return &__this->operator=(std::move(*__param_0));
}

extern "C" void
__rust_thunk___ZN10MyTemplateIdE8processTEd__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<double>* __this, double t) {
  __this->processT(t);
}

static_assert(CRUBIT_SIZEOF(class MyTemplate<float>) == 4);
static_assert(alignof(class MyTemplate<float>) == 4);

extern "C" void
__rust_thunk___ZN10MyTemplateIfEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<float>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk___ZN10MyTemplateIfEC1EOS0___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<float>* __this, class MyTemplate<float>* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

extern "C" class MyTemplate<float>*
__rust_thunk___ZN10MyTemplateIfEaSERKS0___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<float>* __this, const class MyTemplate<float>* __param_0) {
  return &__this->operator=(*__param_0);
}

extern "C" class MyTemplate<float>*
__rust_thunk___ZN10MyTemplateIfEaSEOS0___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<float>* __this, class MyTemplate<float>* __param_0) {
  return &__this->operator=(std::move(*__param_0));
}

extern "C" void
__rust_thunk___ZN10MyTemplateIfE8processTEf__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<float>* __this, float t) {
  __this->processT(t);
}

static_assert(CRUBIT_SIZEOF(class MyTemplate<int>) == 4);
static_assert(alignof(class MyTemplate<int>) == 4);

extern "C" void
__rust_thunk___ZN10MyTemplateIiEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<int>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk___ZN10MyTemplateIiEC1EOS0___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<int>* __this, class MyTemplate<int>* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

extern "C" class MyTemplate<int>*
__rust_thunk___ZN10MyTemplateIiEaSERKS0___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<int>* __this, const class MyTemplate<int>* __param_0) {
  return &__this->operator=(*__param_0);
}

extern "C" class MyTemplate<int>*
__rust_thunk___ZN10MyTemplateIiEaSEOS0___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<int>* __this, class MyTemplate<int>* __param_0) {
  return &__this->operator=(std::move(*__param_0));
}

extern "C" void
__rust_thunk___ZN10MyTemplateIiE8processTEi__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<int>* __this, int t) {
  __this->processT(t);
}

#pragma clang diagnostic pop
