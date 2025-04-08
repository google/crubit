// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_NULLABILITY_AST_HELPERS_H_
#define CRUBIT_NULLABILITY_AST_HELPERS_H_

// Helpers that simplify accessing the Clang AST.

#include <type_traits>

#include "clang/include/clang/AST/Decl.h"
#include "clang/include/clang/AST/DeclCXX.h"
#include "clang/include/clang/AST/Expr.h"
#include "clang/include/clang/AST/ExprCXX.h"

namespace clang::tidy::nullability {

/// Jointly iterates over the parameters and arguments of a CallExpr or
/// CXXConstructExpr.
///
/// This class helps with two issues in matching up parameters and arguments:
/// *  `CXXOperatorCallExpr`s for member operators have a first argument that
///    gives the implicit `this` pointer. There is no corresponding
///    `ParmVarDecl` for this argument.
/// *  Calls to variadic functions have arguments with no corresponding
///    `ParmVarDecl`.
///
/// Typical usage:
///
/// ```
/// CallExpr *CE = ...;
/// const FunctionDecl *Callee = CE->getDirectCallee();
/// if (Callee == nullptr) return;
/// for (ParamAndArgIterator<CallExpr> Iter(*FunctionDecl, CE); Iter; ++Iter) {
///   // Do something with `Iter.param()` and `Iter.arg()`
/// }
/// ```
template <
    typename CallOrConstructExpr,
    std::enable_if_t<std::is_same_v<CallOrConstructExpr, CallExpr> ||
                         std::is_same_v<CallOrConstructExpr, CXXConstructExpr>,
                     int> = 0>
class ParamAndArgIterator {
 public:
  /// Initializes the iterator. `Callee` must be the `FunctionDecl` called by
  /// `E`.
  ParamAndArgIterator(const FunctionDecl &Callee, const CallOrConstructExpr &E)
      : Callee(Callee), E(E) {
    // Member operator calls hold the function object as the first argument,
    // offsetting the indices of parameters and corresponding arguments by 1.
    // For example: Given struct S { bool operator+(int*); }
    // The CXXMethodDecl has one parameter, but a call S{}+p is a
    // CXXOperatorCallExpr with two arguments: an S and an int*.
    if (isa<clang::CXXOperatorCallExpr>(E) && isa<clang::CXXMethodDecl>(Callee))
      ArgI = 1;
  }

  /// Returns whether this iterator contains a valid parameter / argument pair.
  /// If this returns false, iteration has finished and no other methods may be
  /// called.
  operator bool() const {
    return ParamI < Callee.param_size() && ArgI < E.getNumArgs();
  }

  /// Moves to the next parameter / argument pair.
  ParamAndArgIterator &operator++() {
    ++ParamI;
    ++ArgI;
    return *this;
  }

  const Expr &arg() const { return *E.getArg(ArgI); }

  unsigned argIdx() const { return ArgI; }

  const ParmVarDecl &param() const { return *Callee.getParamDecl(ParamI); }

  unsigned paramIdx() const { return ParamI; }

 private:
  const FunctionDecl &Callee;
  const CallOrConstructExpr &E;
  unsigned ParamI = 0;
  unsigned ArgI = 0;
};

}  // namespace clang::tidy::nullability

#endif  // CRUBIT_NULLABILITY_AST_HELPERS_H_
