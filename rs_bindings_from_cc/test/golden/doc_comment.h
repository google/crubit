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

/// Class template.
template <typename T>
struct MyTemplate final {
  /// A non-static member function.
  const T& get_field_value() const { return value; }

  /// Data member.
  T value;
};

/// Class template specialization.
template <>
struct MyTemplate<float> final {
  /// A non-static member function in a specialization.
  const float& get_field_value() const { return value; }

  /// Data member in a specialization.
  float value;
};

/// Type alias to template instantiation.
using MyInstantiation = MyTemplate<int>;

/// Type alias to instantiation of a template specialization.
using MySpecializedInstantiation = MyTemplate<float>;

/// Class template with nested struct inside.
template <typename T>
struct OuterTemplate final {
  /// Doc comment for the nested struct.
  struct NestedStruct {
    /// Data member in a nested struct.
    T value;
  };
};

/// Type alias to a struct nested in a template instantiation.
using ConcreteNestedStruct = OuterTemplate<int>::NestedStruct;

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_DOC_COMMENT_H_
