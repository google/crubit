// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_NULLABILITY_RESTRICT_TO_MAIN_WALKER_H_
#define THIRD_PARTY_CRUBIT_NULLABILITY_RESTRICT_TO_MAIN_WALKER_H_

#include <string>

#include "absl/container/flat_hash_map.h"
#include "clang/AST/DeclBase.h"
#include "clang/AST/RecursiveASTVisitor.h"
#include "clang/Basic/FileEntry.h"
#include "clang/Basic/SourceLocation.h"
#include "clang/Basic/SourceManager.h"
#include "llvm/ADT/StringRef.h"
#include "llvm/Support/Path.h"

namespace clang::tidy::nullability {

// A base class for AST visitors that may want to only see nodes in the main
// file or its associated header.
template <typename Derived>
struct RestrictToMainFileOrHeaderWalker : public RecursiveASTVisitor<Derived> {
  llvm::StringRef MainFileName;
  const SourceManager &SM;
  bool RestrictToMainFileOrHeader;
  absl::flat_hash_map<std::string, bool> InMainFileOrHeaderCache;

 public:
  RestrictToMainFileOrHeaderWalker(const SourceManager &SM,
                                   bool RestrictToMainFileOrHeader)
      : SM(SM), RestrictToMainFileOrHeader(RestrictToMainFileOrHeader) {
    MainFileName = "";
    if (OptionalFileEntryRef MainFile =
            SM.getFileEntryRefForID(SM.getMainFileID())) {
      MainFileName = MainFile->getName();
      MainFileName = MainFileName.starts_with("./") ? MainFileName.substr(2)
                                                    : MainFileName;
    }
  }

  // Returns whether `loc` is in the main file or its associated header (i.e.
  // a header that has the same file path except for the extension).
  bool inMainFileOrHeader(SourceLocation Loc) {
    if (SM.isInMainFile(Loc)) {
      return true;
    }

    auto FileName = SM.getFilename(Loc);
    auto It = InMainFileOrHeaderCache.find(FileName);
    if (It != InMainFileOrHeaderCache.end()) {
      return It->second;
    }

    auto FileNameWoDotSlash =
        FileName.starts_with("./") ? FileName.substr(2) : FileName;
    bool Ret = !MainFileName.empty() && !FileNameWoDotSlash.empty() &&
               llvm::sys::path::parent_path(FileNameWoDotSlash) ==
                   llvm::sys::path::parent_path(MainFileName) &&
               llvm::sys::path::stem(FileNameWoDotSlash) ==
                   llvm::sys::path::stem(MainFileName);
    InMainFileOrHeaderCache[FileName] = Ret;
    return Ret;
  }
};

}  // namespace clang::tidy::nullability

#endif  // THIRD_PARTY_CRUBIT_NULLABILITY_RESTRICT_TO_MAIN_WALKER_H_
