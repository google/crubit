// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_LIFETIME_ANNOTATIONS_FUNCTION_LIFETIMES_H_
#define THIRD_PARTY_CRUBIT_LIFETIME_ANNOTATIONS_FUNCTION_LIFETIMES_H_

#include <iosfwd>
#include <string>
#include <variant>

#include "lifetime_annotations/type_lifetimes.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/Decl.h"
#include "third_party/llvm/llvm-project/llvm/include/llvm/ADT/SmallVector.h"

namespace devtools_rust {

// Lifetimes for the signature of a function.
struct FunctionLifetimes {
  // Lifetimes for the parameters.
  // These are the same number and order as FunctionDecl::parameters().
  llvm::SmallVector<TypeLifetimes> param_lifetimes;

  // Lifetimes for the return type.
  TypeLifetimes return_lifetimes;

  // Lifetimes for the `this` parameter for non-static member functions.
  TypeLifetimes this_lifetimes;

  // Returns whether this object contains any lifetimes.
  // Note that this returns false if `param_lifetimes` is non-empty but only
  // contains empty TypeLifetimes.
  bool ContainsLifetimes();

  // Returns if the two FunctionLifetimes have the same structures, without
  // requiring them to have the same exact Lifetimes. They have the same
  // structure if unique vs reoccuring Lifetimes in `this` and `other` are found
  // in the same positions.
  bool IsIsomorphic(const FunctionLifetimes& other) const;

  // Returns true if this FunctionLifetimes object is valid for the given
  // function.
  bool Validate(const clang::FunctionDecl* func) const;

  // Returns a human-readable representation of `func_lifetimes`. Formats
  // lifetimes using `formatter`, or Lifetime::DebugString() if `formatter` is
  // null.
  std::string DebugString(LifetimeFormatter formatter = [](Lifetime l) {
    return l.DebugString();
  }) const;
};

std::ostream& operator<<(std::ostream& os,
                         const FunctionLifetimes& func_lifetimes);

// An error that occurred while analyzing a function.
struct FunctionAnalysisError {
  explicit FunctionAnalysisError(llvm::StringRef message) : message(message) {}

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
    std::variant<FunctionLifetimes, FunctionAnalysisError>;

}  // namespace devtools_rust

#endif  // THIRD_PARTY_CRUBIT_LIFETIME_ANNOTATIONS_FUNCTION_LIFETIMES_H_
