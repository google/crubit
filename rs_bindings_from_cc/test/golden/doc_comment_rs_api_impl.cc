// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:doc_comment_cc

#include <cstddef>
#include <memory>

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/golden/doc_comment.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"
extern "C" void __rust_thunk___ZN17DocCommentSlashesC1EOS_(
    struct DocCommentSlashes* __this, struct DocCommentSlashes* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}
extern "C" void __rust_thunk___ZN14DocCommentBangC1Ev(
    struct DocCommentBang* __this) {
  crubit::construct_at(__this);
}
extern "C" void __rust_thunk___ZN14DocCommentBangC1EOS_(
    struct DocCommentBang* __this, struct DocCommentBang* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}
extern "C" void __rust_thunk___ZN24MultilineCommentTwoStarsC1Ev(
    struct MultilineCommentTwoStars* __this) {
  crubit::construct_at(__this);
}
extern "C" void __rust_thunk___ZN24MultilineCommentTwoStarsC1EOS_(
    struct MultilineCommentTwoStars* __this,
    struct MultilineCommentTwoStars* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}
extern "C" void __rust_thunk___ZN11LineCommentC1Ev(struct LineComment* __this) {
  crubit::construct_at(__this);
}
extern "C" void __rust_thunk___ZN11LineCommentC1EOS_(
    struct LineComment* __this, struct LineComment* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}
extern "C" void __rust_thunk___ZN16MultilineOneStarC1Ev(
    struct MultilineOneStar* __this) {
  crubit::construct_at(__this);
}
extern "C" void __rust_thunk___ZN16MultilineOneStarC1EOS_(
    struct MultilineOneStar* __this, struct MultilineOneStar* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}
extern "C" int __rust_thunk___Z3foov() { return foo(); }
extern "C" void
__rust_thunk___ZN10MyTemplateIiEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3adoc_5fcomment_5fcc(
    struct MyTemplate<int>* __this) {
  crubit::construct_at(__this);
}
extern "C" void
__rust_thunk___ZN10MyTemplateIiEC1EOS0___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3adoc_5fcomment_5fcc(
    struct MyTemplate<int>* __this, struct MyTemplate<int>* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}
extern "C" int const*
__rust_thunk___ZNK10MyTemplateIiE15get_field_valueEv__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3adoc_5fcomment_5fcc(
    const struct MyTemplate<int>* __this) {
  return &__this->get_field_value();
}
extern "C" void
__rust_thunk___ZN10MyTemplateIfEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3adoc_5fcomment_5fcc(
    struct MyTemplate<float>* __this) {
  crubit::construct_at(__this);
}
extern "C" void
__rust_thunk___ZN10MyTemplateIfEC1EOS0___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3adoc_5fcomment_5fcc(
    struct MyTemplate<float>* __this, struct MyTemplate<float>* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}
extern "C" float const*
__rust_thunk___ZNK10MyTemplateIfE15get_field_valueEv__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3adoc_5fcomment_5fcc(
    const struct MyTemplate<float>* __this) {
  return &__this->get_field_value();
}

static_assert(sizeof(struct DocCommentSlashes) == 4);
static_assert(alignof(struct DocCommentSlashes) == 4);
static_assert(CRUBIT_OFFSET_OF(i, struct DocCommentSlashes) == 0);

static_assert(sizeof(struct DocCommentBang) == 4);
static_assert(alignof(struct DocCommentBang) == 4);
static_assert(CRUBIT_OFFSET_OF(i, struct DocCommentBang) == 0);

static_assert(sizeof(struct MultilineCommentTwoStars) == 4);
static_assert(alignof(struct MultilineCommentTwoStars) == 4);
static_assert(CRUBIT_OFFSET_OF(i, struct MultilineCommentTwoStars) == 0);

static_assert(sizeof(struct LineComment) == 4);
static_assert(alignof(struct LineComment) == 4);
static_assert(CRUBIT_OFFSET_OF(i, struct LineComment) == 0);

static_assert(sizeof(struct MultilineOneStar) == 4);
static_assert(alignof(struct MultilineOneStar) == 4);
static_assert(CRUBIT_OFFSET_OF(i, struct MultilineOneStar) == 0);

static_assert(sizeof(struct MyTemplate<int>) == 4);
static_assert(alignof(struct MyTemplate<int>) == 4);
static_assert(CRUBIT_OFFSET_OF(value, struct MyTemplate<int>) == 0);

static_assert(sizeof(struct MyTemplate<float>) == 4);
static_assert(alignof(struct MyTemplate<float>) == 4);
static_assert(CRUBIT_OFFSET_OF(value, struct MyTemplate<float>) == 0);

#pragma clang diagnostic pop
