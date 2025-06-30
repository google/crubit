// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_LIFETIME_ANNOTATIONS_FUNCTION_LIFETIMES_H_
#define CRUBIT_LIFETIME_ANNOTATIONS_FUNCTION_LIFETIMES_H_

#include <cassert>
#include <cstddef>
#include <functional>
#include <iosfwd>
#include <optional>
#include <string>
#include <utility>
#include <variant>

#include "lifetime_annotations/lifetime.h"
#include "lifetime_annotations/lifetime_substitutions.h"
#include "lifetime_annotations/type_lifetimes.h"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclCXX.h"
#include "clang/AST/Type.h"
#include "clang/AST/TypeLoc.h"
#include "llvm/ADT/DenseMap.h"
#include "llvm/ADT/DenseSet.h"
#include "llvm/ADT/SmallVector.h"
#include "llvm/ADT/StringRef.h"
#include "llvm/Support/Error.h"
#include "llvm/Support/raw_ostream.h"

namespace clang {
namespace tidy {
namespace lifetimes {

// Interface used to create lifetimes in FunctionLifetimes::CreateForDecl.
// CreateReturnLifetimes will be called with the ValueLifetimes that were
// created through calls to CreateParamLifetimes.
class FunctionLifetimeFactory {
 public:
  virtual ~FunctionLifetimeFactory() = default;

  virtual llvm::Expected<ValueLifetimes> CreateThisLifetimes(
      clang::QualType type, const clang::Expr* lifetime_name) const = 0;

  // Note: The `type_loc` parameter passed into `CreateParamLifetimes` and
  // `CreateReturnLifetimes` may be null if no type location is available.

  virtual llvm::Expected<ValueLifetimes> CreateParamLifetimes(
      clang::QualType type, clang::TypeLoc type_loc) const = 0;
  virtual llvm::Expected<ValueLifetimes> CreateReturnLifetimes(
      clang::QualType type, clang::TypeLoc type_loc,
      const llvm::SmallVector<ValueLifetimes>& param_lifetimes,
      const std::optional<ValueLifetimes>& this_lifetimes) const = 0;
};

// Implementation of FunctionLifetimeFactory that defers to a LifetimeFactory.
class FunctionLifetimeFactorySingleCallback : public FunctionLifetimeFactory {
 public:
  explicit FunctionLifetimeFactorySingleCallback(LifetimeFactory factory)
      : factory_(std::move(factory)) {}
  llvm::Expected<ValueLifetimes> CreateThisLifetimes(
      clang::QualType type,
      const clang::Expr* /*lifetime_name*/) const override {
    // TODO(mboehme): There's currently no way for us to pass `lifetime_name` on
    // into `ValueLifetimes::Create()`. We may need to add another overload of
    // `ValueLifetimes::Create()` if we ever need this.
    return ValueLifetimes::Create(type, TypeLoc(), factory_);
  }
  llvm::Expected<ValueLifetimes> CreateParamLifetimes(
      clang::QualType type, clang::TypeLoc type_loc) const override {
    return ValueLifetimes::Create(type, type_loc, factory_);
  }
  llvm::Expected<ValueLifetimes> CreateReturnLifetimes(
      clang::QualType type, clang::TypeLoc type_loc,
      const llvm::SmallVector<ValueLifetimes>& /*param_lifetimes*/,
      const std::optional<ValueLifetimes>& /*this_lifetimes*/) const override {
    return ValueLifetimes::Create(type, type_loc, factory_);
  }

 private:
  LifetimeFactory factory_;
};

// Lifetimes for the signature of a function.
class FunctionLifetimes {
 public:
  // Returns lifetimes for the `i`-th parameter.
  // These are the same number and order as FunctionDecl::parameters().
  const ValueLifetimes& GetParamLifetimes(size_t i) const {
    return param_lifetimes_[i];
  }

  // Returns the number of function parameters (excluding the implicit `this).
  size_t GetNumParams() const { return param_lifetimes_.size(); }

  // Lifetimes for the return type.
  const ValueLifetimes& GetReturnLifetimes() const { return return_lifetimes_; }

  // Lifetimes for the `this` parameter for non-static member functions.
  const ValueLifetimes& GetThisLifetimes() const {
    assert(this_lifetimes_.has_value());
    return *this_lifetimes_;
  }

  // Returns whether this FunctionLifetimes represents a non-static method.
  bool IsNonStaticMethod() const { return this_lifetimes_.has_value(); }

  // Creates lifetimes for a function with a given decl.
  // Only fails if lifetime_factory fails.
  // Lifetimes will be created first for the object parameter, if any, then for
  // parameters in increasing order, and finally for the return type.
  static llvm::Expected<FunctionLifetimes> CreateForDecl(
      const clang::FunctionDecl* function,
      const FunctionLifetimeFactory& lifetime_factory);

  static llvm::Expected<FunctionLifetimes> CreateForFunctionType(
      const clang::FunctionProtoType* function, clang::TypeLoc func_type_loc,
      const FunctionLifetimeFactory& lifetime_factory);
  static llvm::Expected<FunctionLifetimes> CreateForFunctionType(
      const clang::FunctionProtoType* function,
      const FunctionLifetimeFactory& lifetime_factory);

  // TODO(veluca): add support for pointer-to-member-fn.

  // Creates a copy of this FunctionLifetimes with the same structure, but
  // fresh, unrelated lifetimes independently of whether the lifetimes where
  // identical in this FunctionLifetimes.
  // TODO(veluca): remove this method once FunctionLifetimes keeps track of its
  // own type, and replace it with an appropriate call to Create().
  llvm::Expected<FunctionLifetimes> CreateCopy(
      const LifetimeFactory& factory) const;

  // Returns FunctionLifetimes for a method that this method overrides.
  // Precondition: `IsNonStaticMethod()` is true,
  // `method`'s signature is compatible with this FunctionLifetimes except for
  // the `this` parameter.
  FunctionLifetimes ForOverriddenMethod(const clang::CXXMethodDecl* method);

  // Checks if this FunctionLifetimes represents valid lifetimes for the given
  // Decl.
  bool IsValidForDecl(const clang::FunctionDecl* function);

  // Returns a human-readable representation of `func_lifetimes`. Formats
  // lifetimes using `formatter`, or Lifetime::DebugString() if `formatter` is
  // null.
  std::string DebugString(LifetimeFormatter formatter = [](Lifetime l) {
    return l.DebugString();
  }) const;

  // Returns true if `predicate` returns true for any lifetime that appears in
  // the `FunctionLifetimes`.
  bool HasAny(const std::function<bool(Lifetime)>& predicate) const;

  // Returns the set of all lifetimes that are either lifetime parameters of
  // this function, or (if this FunctionLifetimes is a declaration of a method)
  // of the enclosing class.
  llvm::DenseSet<Lifetime> AllFreeLifetimes() const;

  // Applies `subst` to all lifetimes in this FunctionLifetimes.
  // Any lifetime parameter declarations will moved to the innermost location
  // that is valid for the new lifetimes. Note that this operation is
  // well-defined and declarations of lifetime parameters can only "move up";
  // in particular, it results lifetime parameters being as tightly bound as
  // possible, which is what we want inference to infer.
  void SubstituteLifetimes(const LifetimeSubstitutions& subst);

  // Traverses all the lifetimes in the function signature, recursively. The
  // visit is done in post-order on the lifetime tree of this type.
  void Traverse(std::function<void(Lifetime&, Variance)> visitor);
  void Traverse(std::function<void(const Lifetime&, Variance)> visitor) const;

 private:
  llvm::SmallVector<ValueLifetimes> param_lifetimes_;
  ValueLifetimes return_lifetimes_;
  std::optional<ValueLifetimes> this_lifetimes_;

  friend class ObjectRepository;

  static llvm::Expected<FunctionLifetimes> Create(
      const clang::FunctionProtoType* type, clang::TypeLoc type_loc,
      clang::QualType this_type,
      const FunctionLifetimeFactory& lifetime_factory);
};

std::ostream& operator<<(std::ostream& os,
                         const FunctionLifetimes& func_lifetimes);

// An error that occurred while analyzing a function.
struct FunctionAnalysisError {
  explicit FunctionAnalysisError(llvm::StringRef message = "")
      : message(message) {}

  explicit FunctionAnalysisError(const llvm::Error& err) {
    ::llvm::raw_string_ostream stream(message);
    stream << err;
  }

  // Human-readable description of the error.
  std::string message;
};

// Lifetimes for a function, or an error if we couldn't analyze the function.
// We can't use llvm::Expected<FunctionLifetimes> for this because:
// - llvm::Expected doesn't allow us to check for an error state without moving
//   the error out of the llvm::Expected
// - llvm::Expected asserts in the destructor if we didn't check for an error
using FunctionLifetimesOrError =
    std::variant<FunctionAnalysisError, FunctionLifetimes>;

using FunctionLifetimesMap =
    llvm::DenseMap<const FunctionDecl*, FunctionLifetimesOrError>;

}  // namespace lifetimes
}  // namespace tidy
}  // namespace clang

#endif  // CRUBIT_LIFETIME_ANNOTATIONS_FUNCTION_LIFETIMES_H_
