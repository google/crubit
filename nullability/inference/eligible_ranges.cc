// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/inference/eligible_ranges.h"

#include <cassert>
#include <optional>
#include <string_view>
#include <utility>
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

static Nullability toProtoNullability(NullabilityKind Kind) {
  switch (Kind) {
    case NullabilityKind::NonNull:
      return Nullability::NONNULL;
    case NullabilityKind::Nullable:
    case NullabilityKind::NullableResult:
      return Nullability::NULLABLE;
    case NullabilityKind::Unspecified:
      return Nullability::UNKNOWN;
  }
  llvm_unreachable("Unhandled NullabilityKind");
}

static void initSlotRange(SlotRange &R, std::optional<SlotNum> Slot,
                          unsigned Begin, unsigned End,
                          std::optional<NullabilityKind> Nullability,
                          std::optional<int> AnnotationPreRangeLength,
                          std::optional<int> AnnotationPostRangeLength) {
  if (Slot) R.set_slot(*Slot);
  R.set_begin(Begin);
  R.set_end(End);
  if (Nullability) {
    R.set_existing_annotation(toProtoNullability(*Nullability));

    if (AnnotationPreRangeLength)
      R.set_existing_annotation_pre_range_length(*AnnotationPreRangeLength);
    if (AnnotationPostRangeLength)
      R.set_existing_annotation_post_range_length(*AnnotationPostRangeLength);
  }
}

/// If the tokens immediately before `Begin` are an absl::NullabilityUnknown<
/// annotation start, returns the start location of the absl token. Else,
/// returns std::nullopt.
static std::pair<std::optional<unsigned>, std::optional<unsigned>>
getStartAndEndOffsetsOfImmediateAbslAnnotation(SourceLocation Begin,
                                               SourceLocation End,
                                               const SourceManager &SM,
                                               const LangOptions &LangOpts,
                                               const FileID &DeclFID) {
  // absl::NullabilityUnknown< is 4 tokens, one for the `<`, one for the `::`,
  // and one for each identifier.
  Token PrevTok = utils::lexer::getPreviousToken(Begin, SM, LangOpts);
  if (!PrevTok.is(tok::TokenKind::less)) return {};
  if (PrevTok =
          utils::lexer::getPreviousToken(PrevTok.getLocation(), SM, LangOpts);
      !PrevTok.is(tok::TokenKind::raw_identifier))
    return {};
  if (PrevTok.getRawIdentifier() != "NullabilityUnknown") return {};
  if (PrevTok =
          utils::lexer::getPreviousToken(PrevTok.getLocation(), SM, LangOpts);
      PrevTok.isNot(tok::TokenKind::coloncolon))
    return {};
  if (PrevTok =
          utils::lexer::getPreviousToken(PrevTok.getLocation(), SM, LangOpts);
      !PrevTok.is(tok::TokenKind::raw_identifier))
    return {};
  if (PrevTok.getRawIdentifier() != "absl") return {};

  auto [PrevTokFID, PrevTokOffset] = SM.getDecomposedLoc(PrevTok.getLocation());
  if (PrevTokFID != DeclFID) return {};

  Token NextTok;
  // If the token immediately at `End` is a `>`, use the end location of that
  // token. Otherwise, look for the next non-comment token, which should be a
  // `>`.
  if (bool Failed = Lexer::getRawToken(End, NextTok, SM, LangOpts,
                                       /*IgnoreWhiteSpace=*/true))
    return {};
  if (!NextTok.is(tok::TokenKind::greater) &&
      !NextTok.is(tok::TokenKind::greatergreater)) {
    std::optional<Token> MaybeNextTok =
        utils::lexer::findNextTokenSkippingComments(End, SM, LangOpts);
    if (!MaybeNextTok || (!MaybeNextTok->is(tok::TokenKind::greater) &&
                          !MaybeNextTok->is(tok::TokenKind::greatergreater)))
      return {};
    NextTok = *MaybeNextTok;
  }

  auto [NextTokFID, NextTokOffset] = SM.getDecomposedLoc(NextTok.getEndLoc());
  if (NextTokFID != DeclFID) return {};
  if (NextTok.is(tok::TokenKind::greatergreater)) {
    // We need to step back one character.
    --NextTokOffset;
  }

  return {PrevTokOffset, NextTokOffset};
}

/// If the token immediately after `End` is a clang _Null_unspecified attribute,
/// returns the end location of the attribute. Else, returns std::nullopt.
static std::optional<unsigned> getEndOffsetOfImmediateClangAttribute(
    SourceLocation End, const SourceManager &SM, const LangOptions &LangOpts,
    const FileID &DeclFID) {
  // We can simply use `findNextTokenSkippingComments` because the attribute
  // must come at least one space or comment after the type, so it will come
  // after `End`, not at `End`.
  std::optional<Token> NextTok =
      utils::lexer::findNextTokenSkippingComments(End, SM, LangOpts);
  if (!NextTok) return std::nullopt;
  if (!NextTok->is(tok::TokenKind::raw_identifier) ||
      NextTok->getRawIdentifier() != "_Null_unspecified")
    return std::nullopt;

  auto [FID, Offset] = SM.getDecomposedLoc(NextTok->getEndLoc());
  if (FID != DeclFID) return std::nullopt;

  return Offset;
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
                                    const TypeNullabilityDefaults &Defaults,
                                    TypeLocRanges &Result) {
  std::vector<TypeNullabilityLoc> NullabilityLocs =
      getTypeNullabilityLocs(WholeLoc, Defaults);
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
    if (FID != DeclFID) continue;

    unsigned int EndOffset = SM.getFileOffset(R->getEnd());

    // If the type is immediately wrapped in an absl nullability annotation or
    // immediately followed by a clang nullability attribute, collect the
    // pre- and post-range lengths for that annotation/attribute.
    std::optional<int> AnnotationPreRangeLength;
    std::optional<int> AnnotationPostRangeLength;
    if (Nullability && *Nullability == NullabilityKind::Unspecified) {
      auto [AnnotationStartOffset, AnnotationEndOffset] =
          getStartAndEndOffsetsOfImmediateAbslAnnotation(Begin, R->getEnd(), SM,
                                                         LangOpts, DeclFID);
      if (AnnotationStartOffset && AnnotationEndOffset) {
        AnnotationPreRangeLength = BeginOffset - *AnnotationStartOffset;
        AnnotationPostRangeLength = *AnnotationEndOffset - EndOffset;
      } else if (std::optional<unsigned> AttributeEndOffset =
                     getEndOffsetOfImmediateClangAttribute(R->getEnd(), SM,
                                                           LangOpts, DeclFID)) {
        AnnotationPreRangeLength = 0;
        AnnotationPostRangeLength = *AttributeEndOffset - EndOffset;
      }
    }

    initSlotRange(*Result.add_range(), SlotInContext, BeginOffset, EndOffset,
                  Nullability, AnnotationPreRangeLength,
                  AnnotationPostRangeLength);
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

static void setPragmaNullability(FileID FID,
                                 const TypeNullabilityDefaults &Defaults,
                                 TypeLocRanges &Ranges) {
  // Don't use Defaults.get(File) in order to avoid treating a lack of pragma as
  // a pragma setting of Defaults.DefaultNullability.
  if (!Defaults.FileNullability) return;
  if (auto It = Defaults.FileNullability->find(FID);
      It != Defaults.FileNullability->end()) {
    Ranges.set_pragma_nullability(toProtoNullability(It->second));
  }
}

static std::optional<TypeLocRanges> getEligibleRanges(
    const FunctionDecl &Fun, const TypeNullabilityDefaults &Defaults) {
  FunctionTypeLoc TyLoc = Fun.getFunctionTypeLoc();
  if (TyLoc.isNull()) return std::nullopt;
  const clang::ASTContext &Context = Fun.getParentASTContext();
  const SourceManager &SrcMgr = Context.getSourceManager();
  FileID DeclFID = SrcMgr.getFileID(SrcMgr.getExpansionLoc(Fun.getLocation()));
  if (!DeclFID.isValid()) return std::nullopt;

  TypeLocRanges Result;
  if (!trySetPath(DeclFID, SrcMgr, Result)) return std::nullopt;
  setPragmaNullability(DeclFID, Defaults, Result);

  addRangesQualifierAware(TyLoc.getReturnLoc(), SLOT_RETURN_TYPE, Context,
                          DeclFID, Defaults, Result);

  for (int I = 0, N = Fun.getNumParams(); I < N; ++I) {
    const ParmVarDecl *P = Fun.getParamDecl(I);
    addRangesQualifierAware(P->getTypeSourceInfo()->getTypeLoc(),
                            SLOT_PARAM + I, Context, DeclFID, Defaults, Result);
  }

  if (Result.range().empty()) return std::nullopt;

  return Result;
}

static std::optional<TypeLocRanges> getEligibleRanges(
    const DeclaratorDecl &D, const TypeNullabilityDefaults &Defaults) {
  TypeLoc TyLoc = D.getTypeSourceInfo()->getTypeLoc();
  if (TyLoc.isNull()) return std::nullopt;
  const clang::ASTContext &Context = D.getASTContext();
  const SourceManager &SrcMgr = Context.getSourceManager();
  FileID DeclFID = SrcMgr.getFileID(SrcMgr.getExpansionLoc(D.getLocation()));
  if (!DeclFID.isValid()) return std::nullopt;

  TypeLocRanges Result;
  if (!trySetPath(DeclFID, SrcMgr, Result)) return std::nullopt;
  setPragmaNullability(DeclFID, Defaults, Result);

  addRangesQualifierAware(TyLoc, Slot(0), Context, DeclFID, Defaults, Result);
  if (Result.range().empty()) return std::nullopt;

  return Result;
}

std::optional<TypeLocRanges> getEligibleRanges(
    const Decl &D, const TypeNullabilityDefaults &Defaults) {
  if (const auto *Fun = clang::dyn_cast<FunctionDecl>(&D))
    return getEligibleRanges(*Fun, Defaults);
  if (const auto *Field = clang::dyn_cast<FieldDecl>(&D))
    return getEligibleRanges(*Field, Defaults);
  if (const auto *Var = clang::dyn_cast<VarDecl>(&D))
    return getEligibleRanges(*Var, Defaults);
  return std::nullopt;
}

std::optional<TypeLocRanges> getInferenceRanges(
    const Decl &D, const TypeNullabilityDefaults &Defaults) {
  if (!isInferenceTarget(D)) return std::nullopt;
  return getEligibleRanges(D, Defaults);
}

}  // namespace clang::tidy::nullability
