// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/pragma.h"

#include <optional>

#include "clang/include/clang/Basic/Diagnostic.h"
#include "clang/include/clang/Basic/SourceLocation.h"
#include "clang/include/clang/Basic/Specifiers.h"
#include "clang/include/clang/Basic/TokenKinds.h"
#include "clang/include/clang/Lex/Pragma.h"
#include "clang/include/clang/Lex/Preprocessor.h"
#include "llvm/include/llvm/ADT/StringSwitch.h"

namespace clang {
namespace tidy::nullability {
namespace {

class DefaultNullabilityPragmaHandler : public PragmaHandler {
  NullabilityPragmas &Out;
  DiagnosticsEngine &Diags;
  unsigned DiagInvalidPragma;
  unsigned DiagRepeatedPragma;

  void HandlePragma(Preprocessor &PP, PragmaIntroducer Introducer,
                    Token &FirstToken) override {
    FileID File =
        PP.getSourceManager().getDecomposedExpansionLoc(Introducer.Loc).first;
    if (!File.isValid()) return;

    Token NKTok;
    PP.Lex(NKTok);
    auto NK = parseNullabilityKind(NKTok);
    if (!NK) {
      Diags.Report(Introducer.Loc, DiagInvalidPragma);
      return;
    }

    if (!Out.try_emplace(File, *NK).second)
      Diags.Report(Introducer.Loc, DiagRepeatedPragma);
  }

  std::optional<NullabilityKind> parseNullabilityKind(Token &Tok) {
    if (!Tok.is(tok::identifier)) return std::nullopt;
    return llvm::StringSwitch<std::optional<NullabilityKind>>(
               Tok.getIdentifierInfo()->getName())
        .Case("nullable", NullabilityKind::Nullable)
        .Case("nonnull", NullabilityKind::NonNull)
        .Default(std::nullopt);
  }

 public:
  DefaultNullabilityPragmaHandler(NullabilityPragmas &Out,
                                  DiagnosticsEngine &Diags)
      : PragmaHandler("file_default"),
        Out(Out),
        Diags(Diags),
        DiagInvalidPragma(Diags.getCustomDiagID(
            DiagnosticsEngine::Level::Warning,
            "ignoring invalid #pragma nullability file_default directive")),
        DiagRepeatedPragma(Diags.getCustomDiagID(
            DiagnosticsEngine::Level::Warning,
            "ignoring repeated #pragma nullability file_default directive")) {}
};

}  // namespace

void registerPragmaHandler(Preprocessor &PP, NullabilityPragmas &Out) {
  PP.AddPragmaHandler("nullability", new DefaultNullabilityPragmaHandler(
                                         Out, PP.getDiagnostics()));
}

}  // namespace tidy::nullability
}  // namespace clang
