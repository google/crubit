// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// This file provides a mechanism for replacing macro definitions prior to
// running inference tooling to allow detection of certain macro calls and the
// values of their arguments.

#ifndef CRUBIT_NULLABILITY_INFERENCE_REPLACE_MACROS_H_
#define CRUBIT_NULLABILITY_INFERENCE_REPLACE_MACROS_H_

#include <memory>

#include "clang/include/clang/AST/ASTConsumer.h"
#include "clang/include/clang/Basic/IdentifierTable.h"
#include "clang/include/clang/Basic/LLVM.h"
#include "clang/include/clang/Basic/SourceLocation.h"
#include "clang/include/clang/Basic/SourceManager.h"
#include "clang/include/clang/Frontend/CompilerInstance.h"
#include "clang/include/clang/Frontend/FrontendAction.h"
#include "clang/include/clang/Lex/PPCallbacks.h"
#include "clang/include/clang/Lex/Preprocessor.h"
#include "llvm/include/llvm/ADT/DenseMap.h"
#include "llvm/include/llvm/ADT/StringRef.h"

namespace clang::tidy::nullability {
inline constexpr llvm::StringRef ReplacementMacrosHeaderFileName =
    "clang_tidy_nullability_replacement_macros.h";

class ReplaceMacrosCallbacks : public clang::PPCallbacks {
 public:
  explicit ReplaceMacrosCallbacks(clang::Preprocessor &PP) : PP(PP) {}

 private:
  clang::Preprocessor &PP;
  llvm::DenseMap<clang::IdentifierInfo *, const clang::MacroDirective *>
      Replacements;

  enum class State {
    HaveNotSeenReplacementFile,
    InReplacementFile,
    FinishedReplacementFile,
  };
  State State = State::HaveNotSeenReplacementFile;

  void MacroDefined(const clang::Token &MacroNameTok,
                    const clang::MacroDirective *MD) override;

  void FileChanged(SourceLocation Loc, FileChangeReason Reason,
                   SrcMgr::CharacteristicKind FileType,
                   FileID PrevFID) override;
};

class ReplaceMacrosAction : public clang::ASTFrontendAction {
 public:
  explicit ReplaceMacrosAction() = default;

  std::unique_ptr<clang::ASTConsumer> CreateASTConsumer(
      clang::CompilerInstance &, clang::StringRef) override {
    return std::make_unique<clang::ASTConsumer>();
  }

 protected:
  bool BeginSourceFileAction(clang::CompilerInstance &CI) override {
    if (!ASTFrontendAction::BeginSourceFileAction(CI)) return false;

    CI.getPreprocessor().addPPCallbacks(
        std::make_unique<ReplaceMacrosCallbacks>(CI.getPreprocessor()));
    return true;
  }

 private:
};
}  // namespace clang::tidy::nullability

#endif  // CRUBIT_NULLABILITY_INFERENCE_REPLACE_MACROS_H_
