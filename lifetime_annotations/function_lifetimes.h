// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_LIFETIME_ANNOTATIONS_FUNCTION_LIFETIMES_H_
#define CRUBIT_LIFETIME_ANNOTATIONS_FUNCTION_LIFETIMES_H_

#include <iosfwd>
#include <string>
#include <variant>

#include "lifetime_annotations/type_lifetimes.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/Decl.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/Type.h"
#include "third_party/llvm/llvm-project/llvm/include/llvm/ADT/SmallVector.h"
#include "third_party/llvm/llvm-project/llvm/include/llvm/ADT/StringRef.h"
#include "third_party/llvm/llvm-project/llvm/include/llvm/Support/Error.h"

namespace devtools_rust {

// Interface used to create lifetimes in FunctionLifetimes::CreateForDecl.
// CreateReturnLifetime will be called with the ValueLifetimes that were created
// through calls to CreateParamLifetimes.
class FunctionLifetimeFactory {
 public:
  virtual ~FunctionLifetimeFactory() {}
  virtual llvm::Expected<Lifetime> CreateParamLifetime(
      clang::QualType type, llvm::StringRef ref) const = 0;
  virtual llvm::Expected<Lifetime> CreateReturnLifetime(
      clang::QualType type, llvm::StringRef ref,
      const llvm::SmallVector<ValueLifetimes>& param_lifetimes,
      const std::optional<ValueLifetimes>& this_lifetimes) const = 0;
};

// Implementation of FunctionLifetimeFactory that just uses a single callback.
class FunctionLifetimeFactorySingleCallback : public FunctionLifetimeFactory {
 public:
  FunctionLifetimeFactorySingleCallback(LifetimeFactory factory)
      : factory_(std::move(factory)) {}
  llvm::Expected<Lifetime> CreateParamLifetime(
      clang::QualType type, llvm::StringRef ref) const override {
    return factory_(type, ref);
  }
  llvm::Expected<Lifetime> CreateReturnLifetime(
      clang::QualType type, llvm::StringRef ref,
      const llvm::SmallVector<ValueLifetimes>& /*param_lifetimes*/,
      const std::optional<ValueLifetimes>& /*this_lifetimes*/) const override {
    return factory_(type, ref);
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

  // Lifetimes for the return type.
  const ValueLifetimes& GetReturnLifetimes() const { return return_lifetimes_; }

  // Lifetimes for the `this` parameter for non-static member functions.
  const ValueLifetimes& GetThisLifetimes() const {
    assert(this_lifetimes_.has_value());
    return *this_lifetimes_;
  }

  // Creates lifetimes for a function with a given decl.
  // Only fails if lifetime_factory fails.
  // Lifetimes will be created first for the object parameter, if any, then for
  // parameters in increasing order, and finally for the return type.
  static llvm::Expected<FunctionLifetimes> CreateForDecl(
      const clang::FunctionDecl* function,
      const FunctionLifetimeFactory& lifetime_factory);

  // Checks if this FunctionLifetimes represents valid lifetimes for the given
  // Decl.
  bool IsValidForDecl(const clang::FunctionDecl* function);

  // Returns if the two FunctionLifetimes have the same structures, without
  // requiring them to have the same exact Lifetimes. They have the same
  // structure if unique vs reoccuring Lifetimes in `this` and `other` are found
  // in the same positions.
  bool IsIsomorphic(const FunctionLifetimes& other) const;

  // Returns a human-readable representation of `func_lifetimes`. Formats
  // lifetimes using `formatter`, or Lifetime::DebugString() if `formatter` is
  // null.
  std::string DebugString(LifetimeFormatter formatter = [](Lifetime l) {
    return l.DebugString();
  }) const;

  // Traverses all the lifetimes in the function signature, recursively. The
  // visit is done in post-order on the lifetime tree of this type.
  void Traverse(std::function<void(Lifetime&, Variance)> visitor);
  void Traverse(std::function<void(const Lifetime&, Variance)> visitor) const;

 private:
  llvm::SmallVector<ValueLifetimes> param_lifetimes_;
  ValueLifetimes return_lifetimes_;
  std::optional<ValueLifetimes> this_lifetimes_;

  static llvm::Expected<FunctionLifetimes> Create(
      const clang::FunctionProtoType* type, const clang::QualType this_type,
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

}  // namespace devtools_rust

#endif  // CRUBIT_LIFETIME_ANNOTATIONS_FUNCTION_LIFETIMES_H_
