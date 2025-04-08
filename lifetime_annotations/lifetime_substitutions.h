// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#ifndef CRUBIT_LIFETIME_ANNOTATIONS_LIFETIME_SUBSTITUTIONS_H_
#define CRUBIT_LIFETIME_ANNOTATIONS_LIFETIME_SUBSTITUTIONS_H_

#include "lifetime_annotations/lifetime.h"
#include "llvm/include/llvm/ADT/DenseMap.h"

namespace clang {
namespace tidy {
namespace lifetimes {

// A set of substitutions of a lifetime variable with another lifetime (variable
// or constant).
class LifetimeSubstitutions {
 public:
  // Constructs an empty set of substitutions.
  LifetimeSubstitutions() = default;

  // Adds a substitution of `variable` by `substitution`.
  // Precondition: `variable.IsVariable()`
  void Add(Lifetime variable, Lifetime substitution);

  // Returns the result of (transitively) applying all applicable substitutions
  // to `l`.
  // For convenience, `l` may be a constant lifetime; in this case, `l` is
  // always returned unchanged.
  Lifetime Substitute(Lifetime l) const;

  // Dumps the substitutions to llvm::errs().
  void Dump() const;

 private:
  llvm::DenseMap<Lifetime, Lifetime> substitutions_;
};

}  // namespace lifetimes
}  // namespace tidy
}  // namespace clang

#endif  // CRUBIT_LIFETIME_ANNOTATIONS_LIFETIME_SUBSTITUTIONS_H_
