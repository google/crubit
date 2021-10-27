// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <cstddef>
#include <memory>
#include "rs_bindings_from_cc/test/golden/doc_comment.h"

extern "C" int __rust_thunk__foo() { return foo(); }

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
