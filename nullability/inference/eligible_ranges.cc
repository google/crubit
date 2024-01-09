// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/inference/eligible_ranges.h"

#include <cassert>
#include <cstdint>
#include <optional>
#include <string_view>

#include "nullability/inference/inference.proto.h"
#include "nullability/type_nullability.h"
#include "third_party/llvm/llvm-project/clang-tools-extra/clang-tidy/utils/LexerUtils.h"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclBase.h"
#include "clang/AST/Type.h"
#include "clang/AST/TypeLoc.h"
#include "clang/Basic/FileEntry.h"
#include "clang/Basic/LLVM.h"
#include "clang/Basic/SourceLocation.h"
#include "clang/Basic/SourceManager.h"
#include "clang/Basic/TokenKinds.h"
#include "clang/Index/USRGeneration.h"
#include "clang/Lex/Lexer.h"
#include "clang/Lex/Token.h"
#include "clang/Tooling/Transformer/SourceCode.h"
#include "llvm/Support/ErrorHandling.h"
#include "llvm/Support/Path.h"

namespace clang::tidy::nullability {

// TODO: incorporate the predicate used by inference to identify relevant
// top-level slots.
static bool isEligibleTypeLoc(TypeLoc TyLoc) {
  QualType Ty = TyLoc.getType();
  if (!isSupportedPointerType(Ty)) return false;
  // Excludes function-pointer types and other corner cases that use declarator
  // suffix syntax.
  // TODO(sammccall): In general, a declaration looks like BEGIN * MIDDLE NAME
  // END e.g. `int * const (*a) []`.  We need to rewrite as `Nullable<BEGIN *
  // MIDDLE END> NAME, e.g. `Nullable<int * const (*) []> a`. But, if we're not
  // careful, we'll instead rewrite as `Nullable<BEGIN * MIDDLE NAME END>`
  // e.g. `Nullable<int * const (*a) []`.
  //
  // The two are equivalent if either NAME or END is empty. The former is just
  // "is the param unnamed" and the latter is "is PointerTypeLoc.getEndLoc() >
  // Decl.getLocation()?".
  if (const auto *PT = Ty.getNonReferenceType()->getAs<PointerType>()) {
    // TODO: Allow function-pointer types and other corner cases that are
    // currently excluded, because they use declarator suffix syntax. These
    // aren't inherently bad, just tricky to edit correctly. This change will
    // require generalizing this function to take more than just a `TypeLoc`,
    // the declarator may be needed as well.
    QualType PointeeTy = PT->getPointeeType();
    if (PointeeTy->isFunctionType() || PointeeTy->isArrayType()) return false;
  }
  return true;
}

// Extracts the source range of `Loc`, accounting for (nested) qualifiers.
// Guarantees that `Loc` is eligible for editing, including that its begin and
// end locations are in the same file.
//
// We do not consider the `const`-ness of `Loc` itself, because the edit will do
// the correct thing implicitly: the `const` will be left out of `Loc`'s range,
// leaving `const` outside the nullability annotation, which is the preferred
// spelling.
static std::optional<CharSourceRange> getRangeQualifierAware(
    TypeLoc Loc, const ASTContext &Context) {
  if (!isEligibleTypeLoc(Loc)) return std::nullopt;

  auto R = tooling::getFileRange(
      CharSourceRange::getTokenRange(Loc.getSourceRange()), Context,
      /*IncludeMacroExpansion=*/true);
  if (!R) return std::nullopt;

  const auto &SM = Context.getSourceManager();
  const auto &LangOpts = Context.getLangOpts();

  // The start of the new range.
  SourceLocation Begin = R->getBegin();

  // Update `Begin` as we search backwards and find qualifier tokens.
  auto PrevTok = utils::lexer::getPreviousToken(Begin, SM, LangOpts);
  while (PrevTok.getKind() != tok::unknown) {
    if (!PrevTok.is(tok::raw_identifier)) break;
    StringRef RawID = PrevTok.getRawIdentifier();
    if (RawID != "const" && RawID != "volatile" && RawID != "restrict") break;
    Begin = PrevTok.getLocation();
    PrevTok = utils::lexer::getPreviousToken(Begin, SM, LangOpts);
  }

  return CharSourceRange::getCharRange(Begin, R->getEnd());
}

static void initSlotRange(SlotRange &R, uint32_t Slot, unsigned Begin,
                          unsigned End) {
  R.set_slot(Slot);
  R.set_begin(Begin);
  R.set_end(End);
}

static std::optional<TypeLocRanges> getEligibleRanges(const FunctionDecl &Fun) {
  FunctionTypeLoc TyLoc = Fun.getFunctionTypeLoc();
  if (TyLoc.isNull()) return std::nullopt;

  const clang::ASTContext &Context = Fun.getParentASTContext();
  const SourceManager &SrcMgr = Context.getSourceManager();
  TypeLocRanges Result;

  FileID DeclFID = SrcMgr.getFileID(SrcMgr.getExpansionLoc(Fun.getLocation()));

  if (auto CSR = getRangeQualifierAware(TyLoc.getReturnLoc(), Context)) {
    auto [FID, Begin] = SrcMgr.getDecomposedLoc(CSR->getBegin());
    // If the type comes from a different file, then don't attempt to edit -- it
    // might need manual intervention.
    if (FID == DeclFID)
      initSlotRange(*Result.add_range(), SLOT_RETURN_TYPE, Begin,
                    SrcMgr.getFileOffset(CSR->getEnd()));
  }

  for (int I = 0, N = Fun.getNumParams(); I < N; ++I) {
    const ParmVarDecl *P = Fun.getParamDecl(I);
    if (auto CSR = getRangeQualifierAware(P->getTypeSourceInfo()->getTypeLoc(),
                                          Context)) {
      auto [FID, Begin] = SrcMgr.getDecomposedLoc(CSR->getBegin());
      // If the type comes from a different file, then don't attempt to edit --
      // it might need manual intervention.
      if (FID == DeclFID)
        initSlotRange(*Result.add_range(), SLOT_PARAM + I, Begin,
                      SrcMgr.getFileOffset(CSR->getEnd()));
    }
  }

  if (Result.range().empty()) return std::nullopt;
  // Extract the path in which `Fun` is located.
  const clang::OptionalFileEntryRef Entry =
      Context.getSourceManager().getFileEntryRefForID(DeclFID);
  if (!Entry) return std::nullopt;
  Result.set_path(std::string_view(
      llvm::sys::path::remove_leading_dotslash(Entry->getName())));
  return Result;
}

std::optional<TypeLocRanges> getEligibleRanges(const Decl &D) {
  // Only functions are currently supported.
  if (const auto *Fun = clang::dyn_cast<FunctionDecl>(&D))
    return getEligibleRanges(*Fun);
  return std::nullopt;
}

}  // namespace clang::tidy::nullability
