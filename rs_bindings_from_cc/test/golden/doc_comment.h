// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_DOC_COMMENT_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_DOC_COMMENT_H_

#pragma clang lifetime_elision

/// Doc comment
///
///  * with three slashes
struct DocCommentSlashes final {
  /// The default constructor which will get translated into
  /// `impl Default for DocCommentSlashes`.
  DocCommentSlashes();

  /// An implicit conversion constructor which will get translated into `impl
  /// From<int> for DocCommentSlashes`.
  // NOLINTNEXTLINE(google-explicit-constructor)
  DocCommentSlashes(int);

  /// A non-static member function (`const` flavor).
  int get_field_value() const;

  /// A non-static member function (non-`const` flavor).
  void set_field_value(int new_value);

  /// A static method.
  static int static_method();

  /// A field.
  int i;
};

//! Doc comment
//!
//!  * with slashes and bang
struct DocCommentBang final {
  //! A field
  int i;
};

/** Multiline comment

     * with two stars */
struct MultilineCommentTwoStars final {
  /** A field */
  int i;
};

// Line comment
//
//  * with two slashes
struct LineComment final {
  // A field
  int i;
};

/* Multiline comment

    * with one star */
struct MultilineOneStar final {
  /* A field */
  int i;
};

/// A function
inline int foo() { return 42; }

/// A type alias
using MyTypeAlias = DocCommentSlashes;

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_DOC_COMMENT_H_
