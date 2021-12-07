// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <cstddef>
#include <memory>
#include "rs_bindings_from_cc/test/golden/doc_comment.h"

extern "C" void __rust_thunk___ZN17DocCommentSlashesD1Ev(
    DocCommentSlashes* __this) {
  return std ::destroy_at(__this);
}
extern "C" void __rust_thunk___ZN14DocCommentBangD1Ev(DocCommentBang* __this) {
  return std ::destroy_at(__this);
}
extern "C" void __rust_thunk___ZN24MultilineCommentTwoStarsD1Ev(
    MultilineCommentTwoStars* __this) {
  return std ::destroy_at(__this);
}
extern "C" void __rust_thunk___ZN11LineCommentD1Ev(LineComment* __this) {
  return std ::destroy_at(__this);
}
extern "C" void __rust_thunk___ZN16MultilineOneStarD1Ev(
    MultilineOneStar* __this) {
  return std ::destroy_at(__this);
}
extern "C" int __rust_thunk___Z3foov() { return foo(); }

static_assert(sizeof(DocCommentSlashes) == 4);
static_assert(alignof(DocCommentSlashes) == 4);
static_assert(offsetof(DocCommentSlashes, i) * 8 == 0);

static_assert(sizeof(DocCommentBang) == 4);
static_assert(alignof(DocCommentBang) == 4);
static_assert(offsetof(DocCommentBang, i) * 8 == 0);

static_assert(sizeof(MultilineCommentTwoStars) == 4);
static_assert(alignof(MultilineCommentTwoStars) == 4);
static_assert(offsetof(MultilineCommentTwoStars, i) * 8 == 0);

static_assert(sizeof(LineComment) == 4);
static_assert(alignof(LineComment) == 4);
static_assert(offsetof(LineComment, i) * 8 == 0);

static_assert(sizeof(MultilineOneStar) == 4);
static_assert(alignof(MultilineOneStar) == 4);
static_assert(offsetof(MultilineOneStar, i) * 8 == 0);
