// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_NULLABILITY_LOC_FILTER_H_
#define CRUBIT_NULLABILITY_LOC_FILTER_H_

#include <memory>

#include "clang/include/clang/AST/DeclBase.h"
#include "clang/include/clang/Basic/SourceLocation.h"
#include "clang/include/clang/Basic/SourceManager.h"

namespace clang::tidy::nullability {

// An interface for filtering SourceLocations.
class LocFilter {
 public:
  virtual ~LocFilter() = default;
  virtual bool check(SourceLocation Loc) = 0;
};

enum class LocFilterKind {
  kAllowAll,          // No filtering.
  kMainFileOrHeader,  // Restrict to the main file or its associated header.
  kAllowAllButNotMainFile,    // Restrict to all files but the main file.
  kMainHeaderButNotMainFile,  // Restrict to the header associated with the
                              // main file, but not the main file itself.
};

// Returns a LocFilter that restricts according to the given LocFilterKind.
std::unique_ptr<LocFilter> getLocFilter(const SourceManager &SM,
                                        LocFilterKind Kind);

}  // namespace clang::tidy::nullability

#endif  // CRUBIT_NULLABILITY_LOC_FILTER_H_
