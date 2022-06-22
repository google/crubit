// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <cstddef>
#include <memory>

#include "rs_bindings_from_cc/support/cxx20_backports.h"
#include "rs_bindings_from_cc/support/offsetof.h"
#include "rs_bindings_from_cc/test/golden/templates_source_order.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"
extern "C" void
__rust_thunk___ZN10MyTemplateI8TopLevelEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<TopLevel>* __this) {
  crubit::construct_at(std::forward<decltype(__this)>(__this));
}
extern "C" void
__rust_thunk___ZN10MyTemplateIN23test_namespace_bindings5InnerEEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<test_namespace_bindings::Inner>* __this) {
  crubit::construct_at(std::forward<decltype(__this)>(__this));
}
extern "C" void
__rust_thunk___ZN10MyTemplateIS_I8TopLevelEEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<MyTemplate<TopLevel>>* __this) {
  crubit::construct_at(std::forward<decltype(__this)>(__this));
}
extern "C" void
__rust_thunk___ZN10MyTemplateIS_IN23test_namespace_bindings5InnerEEEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<MyTemplate<test_namespace_bindings::Inner>>* __this) {
  crubit::construct_at(std::forward<decltype(__this)>(__this));
}
extern "C" void
__rust_thunk___ZN10MyTemplateIbEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<bool>* __this) {
  crubit::construct_at(std::forward<decltype(__this)>(__this));
}
extern "C" void
__rust_thunk___ZN10MyTemplateIcEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<char>* __this) {
  crubit::construct_at(std::forward<decltype(__this)>(__this));
}
extern "C" void
__rust_thunk___ZN10MyTemplateIdEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<double>* __this) {
  crubit::construct_at(std::forward<decltype(__this)>(__this));
}
extern "C" void
__rust_thunk___ZN10MyTemplateIfEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<float>* __this) {
  crubit::construct_at(std::forward<decltype(__this)>(__this));
}
extern "C" void
__rust_thunk___ZN10MyTemplateIiEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<int>* __this) {
  crubit::construct_at(std::forward<decltype(__this)>(__this));
}
extern "C" void
__rust_thunk___ZN10MyTemplateI8TopLevelEC1ERKS1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<TopLevel>* __this,
    const class MyTemplate<TopLevel>& __param_0) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void
__rust_thunk___ZN10MyTemplateIN23test_namespace_bindings5InnerEEC1ERKS2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<test_namespace_bindings::Inner>* __this,
    const class MyTemplate<test_namespace_bindings::Inner>& __param_0) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void
__rust_thunk___ZN10MyTemplateIS_I8TopLevelEEC1ERKS2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<MyTemplate<TopLevel>>* __this,
    const class MyTemplate<MyTemplate<TopLevel>>& __param_0) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void
__rust_thunk___ZN10MyTemplateIS_IN23test_namespace_bindings5InnerEEEC1ERKS3___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<MyTemplate<test_namespace_bindings::Inner>>* __this,
    const class MyTemplate<MyTemplate<test_namespace_bindings::Inner>>&
        __param_0) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void
__rust_thunk___ZN10MyTemplateIbEC1ERKS0___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<bool>* __this, const class MyTemplate<bool>& __param_0) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void
__rust_thunk___ZN10MyTemplateIcEC1ERKS0___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<char>* __this, const class MyTemplate<char>& __param_0) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void
__rust_thunk___ZN10MyTemplateIdEC1ERKS0___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<double>* __this,
    const class MyTemplate<double>& __param_0) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void
__rust_thunk___ZN10MyTemplateIfEC1ERKS0___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<float>* __this, const class MyTemplate<float>& __param_0) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void
__rust_thunk___ZN10MyTemplateIiEC1ERKS0___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<int>* __this, const class MyTemplate<int>& __param_0) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void
__rust_thunk___ZN10MyTemplateI8TopLevelED1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<TopLevel>* __this) {
  std::destroy_at(std::forward<decltype(__this)>(__this));
}
extern "C" void
__rust_thunk___ZN10MyTemplateIN23test_namespace_bindings5InnerEED1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<test_namespace_bindings::Inner>* __this) {
  std::destroy_at(std::forward<decltype(__this)>(__this));
}
extern "C" void
__rust_thunk___ZN10MyTemplateIS_I8TopLevelEED1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<MyTemplate<TopLevel>>* __this) {
  std::destroy_at(std::forward<decltype(__this)>(__this));
}
extern "C" void
__rust_thunk___ZN10MyTemplateIS_IN23test_namespace_bindings5InnerEEED1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<MyTemplate<test_namespace_bindings::Inner>>* __this) {
  std::destroy_at(std::forward<decltype(__this)>(__this));
}
extern "C" void
__rust_thunk___ZN10MyTemplateIbED1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<bool>* __this) {
  std::destroy_at(std::forward<decltype(__this)>(__this));
}
extern "C" void
__rust_thunk___ZN10MyTemplateIcED1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<char>* __this) {
  std::destroy_at(std::forward<decltype(__this)>(__this));
}
extern "C" void
__rust_thunk___ZN10MyTemplateIdED1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<double>* __this) {
  std::destroy_at(std::forward<decltype(__this)>(__this));
}
extern "C" void
__rust_thunk___ZN10MyTemplateIfED1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<float>* __this) {
  std::destroy_at(std::forward<decltype(__this)>(__this));
}
extern "C" void
__rust_thunk___ZN10MyTemplateIiED1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<int>* __this) {
  std::destroy_at(std::forward<decltype(__this)>(__this));
}
extern "C" class MyTemplate<TopLevel>&
__rust_thunk___ZN10MyTemplateI8TopLevelEaSERKS1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<TopLevel>* __this,
    const class MyTemplate<TopLevel>& __param_0) {
  return __this->operator=(std::forward<decltype(__param_0)>(__param_0));
} extern "C" class MyTemplate<test_namespace_bindings::Inner>&
__rust_thunk___ZN10MyTemplateIN23test_namespace_bindings5InnerEEaSERKS2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<test_namespace_bindings::Inner>* __this,
    const class MyTemplate<test_namespace_bindings::Inner>& __param_0) {
  return __this->operator=(std::forward<decltype(__param_0)>(__param_0));
} extern "C" class MyTemplate<MyTemplate<TopLevel>>&
__rust_thunk___ZN10MyTemplateIS_I8TopLevelEEaSERKS2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<MyTemplate<TopLevel>>* __this,
    const class MyTemplate<MyTemplate<TopLevel>>& __param_0) {
  return __this->operator=(std::forward<decltype(__param_0)>(__param_0));
} extern "C" class MyTemplate<MyTemplate<test_namespace_bindings::Inner>>&
__rust_thunk___ZN10MyTemplateIS_IN23test_namespace_bindings5InnerEEEaSERKS3___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<MyTemplate<test_namespace_bindings::Inner>>* __this,
    const class MyTemplate<MyTemplate<test_namespace_bindings::Inner>>&
        __param_0) {
  return __this->operator=(std::forward<decltype(__param_0)>(__param_0));
} extern "C" class MyTemplate<bool>&
__rust_thunk___ZN10MyTemplateIbEaSERKS0___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<bool>* __this, const class MyTemplate<bool>& __param_0) {
  return __this->operator=(std::forward<decltype(__param_0)>(__param_0));
} extern "C" class MyTemplate<char>&
__rust_thunk___ZN10MyTemplateIcEaSERKS0___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<char>* __this, const class MyTemplate<char>& __param_0) {
  return __this->operator=(std::forward<decltype(__param_0)>(__param_0));
} extern "C" class MyTemplate<double>&
__rust_thunk___ZN10MyTemplateIdEaSERKS0___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<double>* __this,
    const class MyTemplate<double>& __param_0) {
  return __this->operator=(std::forward<decltype(__param_0)>(__param_0));
} extern "C" class MyTemplate<float>&
__rust_thunk___ZN10MyTemplateIfEaSERKS0___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<float>* __this, const class MyTemplate<float>& __param_0) {
  return __this->operator=(std::forward<decltype(__param_0)>(__param_0));
} extern "C" class MyTemplate<int>&
__rust_thunk___ZN10MyTemplateIiEaSERKS0___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<int>* __this, const class MyTemplate<int>& __param_0) {
  return __this->operator=(std::forward<decltype(__param_0)>(__param_0));
} extern "C" void
__rust_thunk___ZN10MyTemplateI8TopLevelE8processTES0___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<TopLevel>* __this, class TopLevel t) {
  __this->processT(std::forward<decltype(t)>(t));
}
extern "C" void
__rust_thunk___ZN10MyTemplateIN23test_namespace_bindings5InnerEE8processTES1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<test_namespace_bindings::Inner>* __this,
    class test_namespace_bindings::Inner t) {
  __this->processT(std::forward<decltype(t)>(t));
}
extern "C" void
__rust_thunk___ZN10MyTemplateIS_I8TopLevelEE8processTES1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<MyTemplate<TopLevel>>* __this,
    class MyTemplate<TopLevel> t) {
  __this->processT(std::forward<decltype(t)>(t));
}
extern "C" void
__rust_thunk___ZN10MyTemplateIS_IN23test_namespace_bindings5InnerEEE8processTES2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<MyTemplate<test_namespace_bindings::Inner>>* __this,
    class MyTemplate<test_namespace_bindings::Inner> t) {
  __this->processT(std::forward<decltype(t)>(t));
}
extern "C" void
__rust_thunk___ZN10MyTemplateIbE8processTEb__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<bool>* __this, bool t) {
  __this->processT(std::forward<decltype(t)>(t));
}
extern "C" void
__rust_thunk___ZN10MyTemplateIcE8processTEc__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<char>* __this, char t) {
  __this->processT(std::forward<decltype(t)>(t));
}
extern "C" void
__rust_thunk___ZN10MyTemplateIdE8processTEd__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<double>* __this, double t) {
  __this->processT(std::forward<decltype(t)>(t));
}
extern "C" void
__rust_thunk___ZN10MyTemplateIfE8processTEf__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<float>* __this, float t) {
  __this->processT(std::forward<decltype(t)>(t));
}
extern "C" void
__rust_thunk___ZN10MyTemplateIiE8processTEi__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
    class MyTemplate<int>* __this, int t) {
  __this->processT(std::forward<decltype(t)>(t));
}
extern "C" void __rust_thunk___ZN8TopLevelC1Ev(class TopLevel* __this) {
  crubit::construct_at(std::forward<decltype(__this)>(__this));
}
extern "C" void __rust_thunk___ZN8TopLevelC1ERKS_(
    class TopLevel* __this, const class TopLevel& __param_0) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN8TopLevelD1Ev(class TopLevel* __this) {
  std::destroy_at(std::forward<decltype(__this)>(__this));
}
extern "C" class TopLevel& __rust_thunk___ZN8TopLevelaSERKS_(
    class TopLevel* __this, const class TopLevel& __param_0) {
  return __this->operator=(std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN23test_namespace_bindings5InnerC1Ev(
    class test_namespace_bindings::Inner* __this) {
  crubit::construct_at(std::forward<decltype(__this)>(__this));
}
extern "C" void __rust_thunk___ZN23test_namespace_bindings5InnerC1ERKS0_(
    class test_namespace_bindings::Inner* __this,
    const class test_namespace_bindings::Inner& __param_0) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN23test_namespace_bindings5InnerD1Ev(
    class test_namespace_bindings::Inner* __this) {
  std::destroy_at(std::forward<decltype(__this)>(__this));
}
extern "C" class test_namespace_bindings::Inner&
__rust_thunk___ZN23test_namespace_bindings5InneraSERKS0_(
    class test_namespace_bindings::Inner* __this,
    const class test_namespace_bindings::Inner& __param_0) {
  return __this->operator=(std::forward<decltype(__param_0)>(__param_0));
}

static_assert(sizeof(class MyTemplate<TopLevel>) == 1);
static_assert(alignof(class MyTemplate<TopLevel>) == 1);

static_assert(sizeof(class MyTemplate<test_namespace_bindings::Inner>) == 1);
static_assert(alignof(class MyTemplate<test_namespace_bindings::Inner>) == 1);

static_assert(sizeof(class MyTemplate<MyTemplate<TopLevel>>) == 1);
static_assert(alignof(class MyTemplate<MyTemplate<TopLevel>>) == 1);

static_assert(
    sizeof(class MyTemplate<MyTemplate<test_namespace_bindings::Inner>>) == 1);
static_assert(
    alignof(class MyTemplate<MyTemplate<test_namespace_bindings::Inner>>) == 1);

static_assert(sizeof(class MyTemplate<bool>) == 1);
static_assert(alignof(class MyTemplate<bool>) == 1);

static_assert(sizeof(class MyTemplate<char>) == 1);
static_assert(alignof(class MyTemplate<char>) == 1);

static_assert(sizeof(class MyTemplate<double>) == 8);
static_assert(alignof(class MyTemplate<double>) == 8);

static_assert(sizeof(class MyTemplate<float>) == 4);
static_assert(alignof(class MyTemplate<float>) == 4);

static_assert(sizeof(class MyTemplate<int>) == 4);
static_assert(alignof(class MyTemplate<int>) == 4);

static_assert(sizeof(class TopLevel) == 1);
static_assert(alignof(class TopLevel) == 1);

static_assert(sizeof(class test_namespace_bindings::Inner) == 1);
static_assert(alignof(class test_namespace_bindings::Inner) == 1);

#pragma clang diagnostic pop
