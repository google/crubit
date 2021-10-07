// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_DOC_COMMENT_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_DOC_COMMENT_H_

/// Doc comment
///
///  * with three slashes
struct DocCommentSlashes {
  /// A field
  int i;
};

//! Doc comment
//!
//!  * with slashes and bang
struct DocCommentBang {
  //! A field
  int i;
};

/** Multiline comment

     * with two stars */
struct MultilineCommentTwoStars {
  /** A field */
  int i;
};

// Line comment
//
//  * with two slashes
struct LineComment {
  // A field
  int i;
};

/* Multiline comment

    * with one star */
struct MultilineOneStar {
  /* A field */
  int i;
};

/// A function
inline int foo() { return 42; }

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_DOC_COMMENT_H_
