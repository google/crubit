// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_NULLABILITY_LOC_FILTER_H_
#define THIRD_PARTY_CRUBIT_NULLABILITY_LOC_FILTER_H_

#include <memory>

#include "clang/AST/DeclBase.h"
#include "clang/Basic/SourceLocation.h"
#include "clang/Basic/SourceManager.h"

namespace clang::tidy::nullability {

// An interface for filtering SourceLocations.
class LocFilter {
 public:
  virtual ~LocFilter() = default;
  virtual bool check(SourceLocation Loc) = 0;
};

// Returns a LocFilter that does or does not restrict to the main file or its
// associated header, per `RestrictToMainFileOrHeader`.
std::unique_ptr<LocFilter> getLocFilter(const SourceManager &SM,
                                        bool RestrictToMainFileOrHeader);

}  // namespace clang::tidy::nullability

#endif  // THIRD_PARTY_CRUBIT_NULLABILITY_LOC_FILTER_H_
