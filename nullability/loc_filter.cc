// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/loc_filter.h"

#include <memory>
#include <string>

#include "absl/log/check.h"
#include "clang/Basic/FileEntry.h"
#include "clang/Basic/SourceLocation.h"
#include "clang/Basic/SourceManager.h"
#include "llvm/ADT/DenseMap.h"
#include "llvm/ADT/StringRef.h"
#include "llvm/Support/ErrorHandling.h"
#include "llvm/Support/Path.h"

namespace clang::tidy::nullability {
namespace {

// A filter for SourceLocations that restricts to those in the main file or its
// associated header.
class InMainFileOrHeader : public LocFilter {
 private:
  OptionalFileEntryRef MainFile;
  std::string MainFileStem;
  const SourceManager &SM;
  llvm::DenseMap<FileID, bool> IsMainFileOrHeaderCache;

 public:
  InMainFileOrHeader(const SourceManager &SM) : SM(SM) {
    FileID MainFileID = SM.getMainFileID();
    MainFile = SM.getFileEntryRefForID(MainFileID);
    CHECK(MainFile) << "Unable to compute main file for filtering.";
    llvm::StringRef MainFileName = MainFile->getName();
    CHECK(!MainFileName.empty());
    MainFileStem = llvm::sys::path::stem(MainFileName);
    IsMainFileOrHeaderCache.insert({MainFileID, true});
  }

  bool isMainFileOrHeader(FileID FileID) {
    OptionalFileEntryRef FileEntry = SM.getFileEntryRefForID(FileID);

    if (!FileEntry) return false;

    if (FileEntry->getDir() != MainFile->getDir()) return false;

    return llvm::sys::path::stem(FileEntry->getName()) == MainFileStem;
  }

  // Returns whether `loc` is in the main file or its associated header (i.e.
  // a header that has the same file path except for the extension).
  bool check(SourceLocation Loc) override {
    if (Loc.isInvalid()) return false;

    if (SM.isInMainFile(Loc)) {
      return true;
    }

    auto FileID = SM.getFileID(Loc);
    if (FileID.isInvalid()) return false;

    // Insert true (arbitrarily chosen over false) to avoid pre-computing the
    // result for cache hits. This does not overwrite the existing value for
    // cache hits.
    auto [It, Inserted] = IsMainFileOrHeaderCache.try_emplace(FileID, true);
    if (Inserted) {
      // Compare the directory and the stem, but not the file extension, to
      // allow matches for the main implementation file and the associated
      // header.
      It->second = isMainFileOrHeader(FileID);
    }

    return It->second;
  }
};

// A filter that allows all locations.
class NoOpFilter : public LocFilter {
  bool check(SourceLocation) override { return true; }
};

}  // namespace

std::unique_ptr<LocFilter> getLocFilter(const SourceManager &SM,
                                        LocFilterKind FilterKind) {
  switch (FilterKind) {
    case LocFilterKind::kAllowAll:
      return std::make_unique<NoOpFilter>();
    case LocFilterKind::kMainFileOrHeader:
      return std::make_unique<InMainFileOrHeader>(SM);
  }
  llvm_unreachable("Unknown LocFilterKind");
  return std::make_unique<NoOpFilter>();
}

}  // namespace clang::tidy::nullability
