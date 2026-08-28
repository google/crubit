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
__rust_thunk__602f8b9b__ZN23test_namespace_bindings10MyTemplateI14DifferentScopeEC1Ev(
    class test_namespace_bindings::MyTemplate<DifferentScope>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(class test_namespace_bindings::MyTemplate<
                     test_namespace_bindings::TemplateParam>) == 1);
static_assert(alignof(class test_namespace_bindings::MyTemplate<
                      test_namespace_bindings::TemplateParam>) == 1);

extern "C" void
__rust_thunk__602f8b9b__ZN23test_namespace_bindings10MyTemplateINS_13TemplateParamEEC1Ev(
    class test_namespace_bindings::MyTemplate<
        test_namespace_bindings::TemplateParam>* __this) {
  crubit::construct_at(__this);
}

static_assert(CRUBIT_SIZEOF(class test_namespace_bindings::MyTemplate<int>) ==
              4);
static_assert(alignof(class test_namespace_bindings::MyTemplate<int>) == 4);

extern "C" void
__rust_thunk__602f8b9b__ZN23test_namespace_bindings10MyTemplateIiEC1Ev(
    class test_namespace_bindings::MyTemplate<int>* __this) {
  crubit::construct_at(__this);
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
__rust_thunk__994b0149__ZN23test_namespace_bindings21TemplateWithTwoParamsINS0_IiiEEiEC1Ev(
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
__rust_thunk__994b0149__ZN23test_namespace_bindings21TemplateWithTwoParamsIifEC1Ev(
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
__rust_thunk__994b0149__ZN23test_namespace_bindings21TemplateWithTwoParamsIiiEC1Ev(
    struct test_namespace_bindings::TemplateWithTwoParams<int, int>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(struct test_namespace_bindings::MyStruct<char>) == 1);
static_assert(alignof(struct test_namespace_bindings::MyStruct<char>) == 1);

extern "C" void
__rust_thunk__9d72ecd0__ZN23test_namespace_bindings8MyStructIcEC1Ev(
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
__rust_thunk__b72784dd__ZN18MyTopLevelTemplateIN23test_namespace_bindings13TemplateParamEEC1Ev(
    struct MyTopLevelTemplate<test_namespace_bindings::TemplateParam>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(class template_template_params::MyTemplate<
                     template_template_params::Policy>) == 1);
static_assert(alignof(class template_template_params::MyTemplate<
                      template_template_params::Policy>) == 1);

extern "C" void
__rust_thunk__57a95918__ZN24template_template_params10MyTemplateINS_6PolicyEEC1Ev(
    class template_template_params::MyTemplate<
        template_template_params::Policy>* __this) {
  crubit::construct_at(__this);
}

#pragma clang diagnostic pop
