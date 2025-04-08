// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/inference/replace_macros.h"

#include <cassert>
#include <string_view>

#include "clang/include/clang/Basic/IdentifierTable.h"
#include "clang/include/clang/Basic/SourceLocation.h"
#include "clang/include/clang/Basic/SourceManager.h"
#include "clang/include/clang/Lex/MacroInfo.h"
#include "clang/include/clang/Lex/PPCallbacks.h"
#include "llvm/include/llvm/ADT/StringRef.h"
#include "llvm/include/llvm/Support/Path.h"

namespace clang::tidy::nullability {
namespace {

// Prefix for the names of our copies of replaced macros.
//
// Used to identify whether a defined macro is itself a copy and to
// set each copy macro's definition equal to the pre-replacement definition of
// the macro it copies.
inline constexpr std::string_view CopyPrefix = "__clang_tidy_nullability_";
}  // namespace

void ReplaceMacrosCallbacks::FileChanged(SourceLocation Loc,
                                         FileChangeReason Reason,
                                         SrcMgr::CharacteristicKind FileType,
                                         FileID PrevFID) {
  if (State == State::FinishedReplacementFile) return;
  if (State == State::InReplacementFile && Reason == PPCallbacks::ExitFile) {
    State = State::FinishedReplacementFile;
    return;
  }

  if (Reason != PPCallbacks::EnterFile) return;

  auto FileName = PP.getSourceManager().getFilename(Loc);
  if (llvm::sys::path::remove_leading_dotslash(FileName) ==
      ReplacementMacrosHeaderFileName) {
    State = State::InReplacementFile;
  }
}

void ReplaceMacrosCallbacks::MacroDefined(const clang::Token &MacroNameTok,
                                          const clang::MacroDirective *MD) {
  auto *IIForCurrentMacro = MacroNameTok.getIdentifierInfo();
  assert(IIForCurrentMacro);

  if (State == State::InReplacementFile) {
    // The only macros in this file are the replacements, the copies of original
    // definitions, and the header guard.
    if (!IIForCurrentMacro->getName().starts_with(CopyPrefix)) {
      // Replacements are seen before the original definitions, so cache the
      // replacements for lookup later when we see the originals.

      // Unnecessary for the header guard, but not a problem, so don't bother
      // filtering.
      Replacements.insert({IIForCurrentMacro, MD});
    }
    return;
  }

  if (auto It = Replacements.find(IIForCurrentMacro);
      It != Replacements.end()) {
    const clang::MacroDirective *ReplacementDef = It->second;

    IdentifierInfo *CopyII =
        PP.getIdentifierInfo((CopyPrefix + IIForCurrentMacro->getName()).str());

    // Replace the (empty) definition of the copy with the pre-replacement
    // definition for this macro.
    PP.appendDefMacroDirective(
        CopyII, const_cast<clang::MacroInfo *>(MD->getMacroInfo()),
        MD->getLocation());

    // Replace the definition of this to-be-replaced macro with the
    // definition from the replacement file.
    PP.appendDefMacroDirective(
        IIForCurrentMacro,
        const_cast<clang::MacroInfo *>(ReplacementDef->getMacroInfo()),
        ReplacementDef->getLocation());
  }
}

}  // namespace clang::tidy::nullability
