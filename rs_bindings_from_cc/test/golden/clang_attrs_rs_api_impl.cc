// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <cstddef>
#include <memory>

#include "rs_bindings_from_cc/support/cxx20_backports.h"
#include "rs_bindings_from_cc/test/golden/clang_attrs.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"
extern "C" void __rust_thunk___ZN18HasCustomAlignmentC1Ev(
    class HasCustomAlignment* __this) {
  crubit::construct_at(std::forward<decltype(__this)>(__this));
}
extern "C" void __rust_thunk___ZN18HasCustomAlignmentC1ERKS_(
    class HasCustomAlignment* __this,
    const class HasCustomAlignment& __param_0) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN18HasCustomAlignmentD1Ev(
    class HasCustomAlignment* __this) {
  std::destroy_at(std::forward<decltype(__this)>(__this));
}
extern "C" class HasCustomAlignment&
__rust_thunk___ZN18HasCustomAlignmentaSERKS_(
    class HasCustomAlignment* __this,
    const class HasCustomAlignment& __param_0) {
  return __this->operator=(std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN27HasFieldWithCustomAlignmentC1Ev(
    class HasFieldWithCustomAlignment* __this) {
  crubit::construct_at(std::forward<decltype(__this)>(__this));
}
extern "C" void __rust_thunk___ZN27HasFieldWithCustomAlignmentC1ERKS_(
    class HasFieldWithCustomAlignment* __this,
    const class HasFieldWithCustomAlignment& __param_0) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN27HasFieldWithCustomAlignmentD1Ev(
    class HasFieldWithCustomAlignment* __this) {
  std::destroy_at(std::forward<decltype(__this)>(__this));
}
extern "C" class HasFieldWithCustomAlignment&
__rust_thunk___ZN27HasFieldWithCustomAlignmentaSERKS_(
    class HasFieldWithCustomAlignment* __this,
    const class HasFieldWithCustomAlignment& __param_0) {
  return __this->operator=(std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN35InheritsFromBaseWithCustomAlignmentC1Ev(
    class InheritsFromBaseWithCustomAlignment* __this) {
  crubit::construct_at(std::forward<decltype(__this)>(__this));
}
extern "C" void __rust_thunk___ZN35InheritsFromBaseWithCustomAlignmentC1ERKS_(
    class InheritsFromBaseWithCustomAlignment* __this,
    const class InheritsFromBaseWithCustomAlignment& __param_0) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN35InheritsFromBaseWithCustomAlignmentD1Ev(
    class InheritsFromBaseWithCustomAlignment* __this) {
  std::destroy_at(std::forward<decltype(__this)>(__this));
}
extern "C" class InheritsFromBaseWithCustomAlignment&
__rust_thunk___ZN35InheritsFromBaseWithCustomAlignmentaSERKS_(
    class InheritsFromBaseWithCustomAlignment* __this,
    const class InheritsFromBaseWithCustomAlignment& __param_0) {
  return __this->operator=(std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN29HasCustomAlignmentWithGnuAttrC1Ev(
    class HasCustomAlignmentWithGnuAttr* __this) {
  crubit::construct_at(std::forward<decltype(__this)>(__this));
}
extern "C" void __rust_thunk___ZN29HasCustomAlignmentWithGnuAttrC1ERKS_(
    class HasCustomAlignmentWithGnuAttr* __this,
    const class HasCustomAlignmentWithGnuAttr& __param_0) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN29HasCustomAlignmentWithGnuAttrD1Ev(
    class HasCustomAlignmentWithGnuAttr* __this) {
  std::destroy_at(std::forward<decltype(__this)>(__this));
}
extern "C" class HasCustomAlignmentWithGnuAttr&
__rust_thunk___ZN29HasCustomAlignmentWithGnuAttraSERKS_(
    class HasCustomAlignmentWithGnuAttr* __this,
    const class HasCustomAlignmentWithGnuAttr& __param_0) {
  return __this->operator=(std::forward<decltype(__param_0)>(__param_0));
}

static_assert(sizeof(class HasCustomAlignment) == 64);
static_assert(alignof(class HasCustomAlignment) == 64);

static_assert(sizeof(class HasFieldWithCustomAlignment) == 64);
static_assert(alignof(class HasFieldWithCustomAlignment) == 64);
static_assert(offsetof(class HasFieldWithCustomAlignment, field) * 8 == 0);

static_assert(sizeof(class InheritsFromBaseWithCustomAlignment) == 64);
static_assert(alignof(class InheritsFromBaseWithCustomAlignment) == 64);

static_assert(sizeof(class HasCustomAlignmentWithGnuAttr) == 64);
static_assert(alignof(class HasCustomAlignmentWithGnuAttr) == 64);

#pragma clang diagnostic pop
