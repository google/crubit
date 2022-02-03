// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <cstddef>
#include <memory>

#include "rs_bindings_from_cc/support/cxx20_backports.h"
#include "rs_bindings_from_cc/test/golden/doc_comment.h"

extern "C" void __rust_thunk___ZN17DocCommentSlashesC1ERKS_(
    class DocCommentSlashes* __this, const class DocCommentSlashes& __param_0) {
  rs_api_impl_support ::construct_at(__this, __param_0);
}
extern "C" void __rust_thunk___ZN17DocCommentSlashesD1Ev(
    class DocCommentSlashes* __this) {
  std ::destroy_at(__this);
}
extern "C" class DocCommentSlashes& __rust_thunk___ZN17DocCommentSlashesaSERKS_(
    class DocCommentSlashes* __this, const class DocCommentSlashes& __param_0) {
  return __this->operator=(__param_0);
}
extern "C" void __rust_thunk___ZN14DocCommentBangC1Ev(
    class DocCommentBang* __this) {
  rs_api_impl_support ::construct_at(__this);
}
extern "C" void __rust_thunk___ZN14DocCommentBangC1ERKS_(
    class DocCommentBang* __this, const class DocCommentBang& __param_0) {
  rs_api_impl_support ::construct_at(__this, __param_0);
}
extern "C" void __rust_thunk___ZN14DocCommentBangD1Ev(
    class DocCommentBang* __this) {
  std ::destroy_at(__this);
}
extern "C" class DocCommentBang& __rust_thunk___ZN14DocCommentBangaSERKS_(
    class DocCommentBang* __this, const class DocCommentBang& __param_0) {
  return __this->operator=(__param_0);
}
extern "C" void __rust_thunk___ZN24MultilineCommentTwoStarsC1Ev(
    class MultilineCommentTwoStars* __this) {
  rs_api_impl_support ::construct_at(__this);
}
extern "C" void __rust_thunk___ZN24MultilineCommentTwoStarsC1ERKS_(
    class MultilineCommentTwoStars* __this,
    const class MultilineCommentTwoStars& __param_0) {
  rs_api_impl_support ::construct_at(__this, __param_0);
}
extern "C" void __rust_thunk___ZN24MultilineCommentTwoStarsD1Ev(
    class MultilineCommentTwoStars* __this) {
  std ::destroy_at(__this);
}
extern "C" class MultilineCommentTwoStars&
__rust_thunk___ZN24MultilineCommentTwoStarsaSERKS_(
    class MultilineCommentTwoStars* __this,
    const class MultilineCommentTwoStars& __param_0) {
  return __this->operator=(__param_0);
}
extern "C" void __rust_thunk___ZN11LineCommentC1Ev(class LineComment* __this) {
  rs_api_impl_support ::construct_at(__this);
}
extern "C" void __rust_thunk___ZN11LineCommentC1ERKS_(
    class LineComment* __this, const class LineComment& __param_0) {
  rs_api_impl_support ::construct_at(__this, __param_0);
}
extern "C" void __rust_thunk___ZN11LineCommentD1Ev(class LineComment* __this) {
  std ::destroy_at(__this);
}
extern "C" class LineComment& __rust_thunk___ZN11LineCommentaSERKS_(
    class LineComment* __this, const class LineComment& __param_0) {
  return __this->operator=(__param_0);
}
extern "C" void __rust_thunk___ZN16MultilineOneStarC1Ev(
    class MultilineOneStar* __this) {
  rs_api_impl_support ::construct_at(__this);
}
extern "C" void __rust_thunk___ZN16MultilineOneStarC1ERKS_(
    class MultilineOneStar* __this, const class MultilineOneStar& __param_0) {
  rs_api_impl_support ::construct_at(__this, __param_0);
}
extern "C" void __rust_thunk___ZN16MultilineOneStarD1Ev(
    class MultilineOneStar* __this) {
  std ::destroy_at(__this);
}
extern "C" class MultilineOneStar& __rust_thunk___ZN16MultilineOneStaraSERKS_(
    class MultilineOneStar* __this, const class MultilineOneStar& __param_0) {
  return __this->operator=(__param_0);
}
extern "C" int __rust_thunk___Z3foov() { return foo(); }

static_assert(sizeof(class DocCommentSlashes) == 4);
static_assert(alignof(class DocCommentSlashes) == 4);
static_assert(offsetof(class DocCommentSlashes, i) * 8 == 0);

static_assert(sizeof(class DocCommentBang) == 4);
static_assert(alignof(class DocCommentBang) == 4);
static_assert(offsetof(class DocCommentBang, i) * 8 == 0);

static_assert(sizeof(class MultilineCommentTwoStars) == 4);
static_assert(alignof(class MultilineCommentTwoStars) == 4);
static_assert(offsetof(class MultilineCommentTwoStars, i) * 8 == 0);

static_assert(sizeof(class LineComment) == 4);
static_assert(alignof(class LineComment) == 4);
static_assert(offsetof(class LineComment, i) * 8 == 0);

static_assert(sizeof(class MultilineOneStar) == 4);
static_assert(alignof(class MultilineOneStar) == 4);
static_assert(offsetof(class MultilineOneStar, i) * 8 == 0);
