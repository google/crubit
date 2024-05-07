// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/inference/eligible_ranges.h"

#include <cassert>
#include <optional>
#include <string_view>
#include <vector>

#include "nullability/inference/inferable.h"
#include "nullability/inference/inference.proto.h"
#include "nullability/type_nullability.h"
#include "third_party/llvm/llvm-project/clang-tools-extra/clang-tidy/utils/LexerUtils.h"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclBase.h"
#include "clang/AST/NestedNameSpecifier.h"
#include "clang/AST/TemplateBase.h"
#include "clang/AST/Type.h"
#include "clang/AST/TypeLoc.h"
#include "clang/Basic/FileEntry.h"
#include "clang/Basic/LLVM.h"
#include "clang/Basic/SourceLocation.h"
#include "clang/Basic/SourceManager.h"
#include "clang/Basic/Specifiers.h"
#include "clang/Basic/TokenKinds.h"
#include "clang/Index/USRGeneration.h"
#include "clang/Lex/Lexer.h"
#include "clang/Lex/Token.h"
#include "clang/Tooling/Transformer/SourceCode.h"
#include "llvm/Support/ErrorHandling.h"
#include "llvm/Support/Path.h"

namespace clang::tidy::nullability {
namespace {
using SlotNum = unsigned;
}

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

static void initSlotRange(SlotRange &R, std::optional<SlotNum> Slot,
                          unsigned Begin, unsigned End,
                          std::optional<NullabilityKind> Nullability) {
  if (Slot) R.set_slot(*Slot);
  R.set_begin(Begin);
  R.set_end(End);
  if (Nullability) {
    switch (*Nullability) {
      case NullabilityKind::NonNull:
        R.set_existing_annotation(Nullability::NONNULL);
        break;
      case NullabilityKind::Nullable:
      case NullabilityKind::NullableResult:
        R.set_existing_annotation(Nullability::NULLABLE);
        break;
      case NullabilityKind::Unspecified:
        R.set_existing_annotation(Nullability::UNKNOWN);
        break;
    }
  }
}

// Extracts the source ranges and associated slot values of each eligible type
// within `Loc`, accounting for (nested) qualifiers. Guarantees that each source
// range is eligible for editing, including that its begin and end locations are
// in the same file.
//
// For each eligible TypeLoc, we do not consider the `const`-ness of the TypeLoc
// itself, because the edit will do the correct thing implicitly: the `const`
// will be left out of the TypeLoc's range, leaving `const` outside the
// nullability annotation, which is the preferred spelling.
static void addRangesQualifierAware(TypeLoc WholeLoc, SlotNum StartingSlot,
                                    const ASTContext &Context,
                                    const FileID &DeclFID,
                                    TypeLocRanges &Result) {
  std::vector<TypeNullabilityLoc> NullabilityLocs =
      getTypeNullabilityLocs(WholeLoc);
  const auto &SM = Context.getSourceManager();
  for (auto &[SlotInLoc, T, MaybeLoc, Nullability] : NullabilityLocs) {
    if (!MaybeLoc || !isEligibleTypeLoc(*MaybeLoc)) continue;
    auto R = tooling::getFileRange(
        CharSourceRange::getTokenRange(MaybeLoc->getSourceRange()), Context,
        /*IncludeMacroExpansion=*/true);
    if (!R) continue;

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

    // TODO(b/323509132) When we can infer more than just top-level pointers,
    // synchronize these slot numbers with inference's slot numbers. For now,
    // assign no slot to anything but a first slot in an inferable type.
    std::optional<SlotNum> SlotInContext =
        SlotInLoc == 0 && hasInferable(WholeLoc.getType())
            ? std::optional(StartingSlot + SlotInLoc)
            : std::nullopt;

    auto [FID, BeginOffset] = SM.getDecomposedLoc(Begin);
    // If the type comes from a different file, then don't attempt to edit -- it
    // might need manual intervention.
    if (FID == DeclFID)
      initSlotRange(*Result.add_range(), SlotInContext, BeginOffset,
                    SM.getFileOffset(R->getEnd()), Nullability);
  }
}

static bool trySetPath(FileID FID, const SourceManager &SrcMgr,
                       TypeLocRanges &Ranges) {
  const clang::OptionalFileEntryRef Entry = SrcMgr.getFileEntryRefForID(FID);
  if (!Entry) return false;
  Ranges.set_path(std::string_view(
      llvm::sys::path::remove_leading_dotslash(Entry->getName())));
  return true;
}

static std::optional<TypeLocRanges> getEligibleRanges(const FunctionDecl &Fun) {
  FunctionTypeLoc TyLoc = Fun.getFunctionTypeLoc();
  if (TyLoc.isNull()) return std::nullopt;

  const clang::ASTContext &Context = Fun.getParentASTContext();
  const SourceManager &SrcMgr = Context.getSourceManager();
  TypeLocRanges Result;

  FileID DeclFID = SrcMgr.getFileID(SrcMgr.getExpansionLoc(Fun.getLocation()));
  if (!trySetPath(DeclFID, SrcMgr, Result)) return std::nullopt;

  addRangesQualifierAware(TyLoc.getReturnLoc(), SLOT_RETURN_TYPE, Context,
                          DeclFID, Result);

  for (int I = 0, N = Fun.getNumParams(); I < N; ++I) {
    const ParmVarDecl *P = Fun.getParamDecl(I);
    addRangesQualifierAware(P->getTypeSourceInfo()->getTypeLoc(),
                            SLOT_PARAM + I, Context, DeclFID, Result);
  }

  if (Result.range().empty()) return std::nullopt;

  return Result;
}

static std::optional<TypeLocRanges> getEligibleRanges(const DeclaratorDecl &D) {
  TypeLoc TyLoc = D.getTypeSourceInfo()->getTypeLoc();
  if (TyLoc.isNull()) return std::nullopt;

  const clang::ASTContext &Context = D.getASTContext();
  const SourceManager &SrcMgr = Context.getSourceManager();
  TypeLocRanges Result;

  FileID DeclFID = SrcMgr.getFileID(SrcMgr.getExpansionLoc(D.getLocation()));
  if (!trySetPath(DeclFID, SrcMgr, Result)) return std::nullopt;

  addRangesQualifierAware(TyLoc, Slot(0), Context, DeclFID, Result);
  if (Result.range().empty()) return std::nullopt;

  return Result;
}

std::optional<TypeLocRanges> getEligibleRanges(const Decl &D) {
  if (!isInferenceTarget(D)) return std::nullopt;
  if (const auto *Fun = clang::dyn_cast<FunctionDecl>(&D))
    return getEligibleRanges(*Fun);
  if (const auto *Field = clang::dyn_cast<FieldDecl>(&D))
    return getEligibleRanges(*Field);
  if (const auto *Var = clang::dyn_cast<VarDecl>(&D))
    return getEligibleRanges(*Var);
  return std::nullopt;
}

}  // namespace clang::tidy::nullability
