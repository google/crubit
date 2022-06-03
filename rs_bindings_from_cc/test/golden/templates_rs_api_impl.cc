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
extern "C" void
__rust_thunk___ZN10MyTemplateIiEC1Ev___third_party_crubit_rs_bindings_from_cc_test_golden_templates_cc(
    class MyTemplate<int>* __this) {
  crubit::construct_at(std::forward<decltype(__this)>(__this));
}
extern "C" void
__rust_thunk___ZN10MyTemplateIiEC1ERKS0____third_party_crubit_rs_bindings_from_cc_test_golden_templates_cc(
    class MyTemplate<int>* __this, const class MyTemplate<int>& __param_0) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void
__rust_thunk___ZN10MyTemplateIiEC1EOS0____third_party_crubit_rs_bindings_from_cc_test_golden_templates_cc(
    class MyTemplate<int>* __this, class MyTemplate<int>&& __param_0) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void
__rust_thunk___ZN10MyTemplateIiED1Ev___third_party_crubit_rs_bindings_from_cc_test_golden_templates_cc(
    class MyTemplate<int>* __this) {
  std::destroy_at(std::forward<decltype(__this)>(__this));
}
extern "C" class MyTemplate<int>&
__rust_thunk___ZN10MyTemplateIiEaSERKS0____third_party_crubit_rs_bindings_from_cc_test_golden_templates_cc(
    class MyTemplate<int>* __this, const class MyTemplate<int>& __param_0) {
  return __this->operator=(std::forward<decltype(__param_0)>(__param_0));
} extern "C" class MyTemplate<int>&
__rust_thunk___ZN10MyTemplateIiEaSEOS0____third_party_crubit_rs_bindings_from_cc_test_golden_templates_cc(
    class MyTemplate<int>* __this, class MyTemplate<int>&& __param_0) {
  return __this->operator=(std::forward<decltype(__param_0)>(__param_0));
} extern "C" class MyTemplate<int>
__rust_thunk___ZN10MyTemplateIiE6CreateEi___third_party_crubit_rs_bindings_from_cc_test_golden_templates_cc(
    int value) {
  return MyTemplate<int>::Create(std::forward<decltype(value)>(value));
} extern "C" int const&
__rust_thunk___ZNK10MyTemplateIiE5valueEv___third_party_crubit_rs_bindings_from_cc_test_golden_templates_cc(
    const class MyTemplate<int>* __this) {
  return __this->value();
}
extern "C" void
__rust_thunk___ZN21TemplateWithTwoParamsIifEC1Ev___third_party_crubit_rs_bindings_from_cc_test_golden_templates_cc(
    class TemplateWithTwoParams<int, float>* __this) {
  crubit::construct_at(std::forward<decltype(__this)>(__this));
}
extern "C" void
__rust_thunk___ZN21TemplateWithTwoParamsIifEC1ERKS0____third_party_crubit_rs_bindings_from_cc_test_golden_templates_cc(
    class TemplateWithTwoParams<int, float>* __this,
    const class TemplateWithTwoParams<int, float>& __param_0) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void
__rust_thunk___ZN21TemplateWithTwoParamsIifEC1EOS0____third_party_crubit_rs_bindings_from_cc_test_golden_templates_cc(
    class TemplateWithTwoParams<int, float>* __this,
    class TemplateWithTwoParams<int, float>&& __param_0) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void
__rust_thunk___ZN21TemplateWithTwoParamsIifED1Ev___third_party_crubit_rs_bindings_from_cc_test_golden_templates_cc(
    class TemplateWithTwoParams<int, float>* __this) {
  std::destroy_at(std::forward<decltype(__this)>(__this));
}
extern "C" class TemplateWithTwoParams<int, float>&
__rust_thunk___ZN21TemplateWithTwoParamsIifEaSERKS0____third_party_crubit_rs_bindings_from_cc_test_golden_templates_cc(
    class TemplateWithTwoParams<int, float>* __this,
    const class TemplateWithTwoParams<int, float>& __param_0) {
  return __this->operator=(std::forward<decltype(__param_0)>(__param_0));
} extern "C" class TemplateWithTwoParams<int, float>&
__rust_thunk___ZN21TemplateWithTwoParamsIifEaSEOS0____third_party_crubit_rs_bindings_from_cc_test_golden_templates_cc(
    class TemplateWithTwoParams<int, float>* __this,
    class TemplateWithTwoParams<int, float>&& __param_0) {
  return __this->operator=(std::forward<decltype(__param_0)>(__param_0));
}

static_assert(sizeof(class MyTemplate<int>) == 4);
static_assert(alignof(class MyTemplate<int>) == 4);

static_assert(sizeof(class TemplateWithTwoParams<int, float>) == 8);
static_assert(alignof(class TemplateWithTwoParams<int, float>) == 4);
static_assert(CRUBIT_OFFSET_OF(value1,
                               class TemplateWithTwoParams<int, float>) == 0);
static_assert(CRUBIT_OFFSET_OF(value2,
                               class TemplateWithTwoParams<int, float>) == 4);

#pragma clang diagnostic pop
