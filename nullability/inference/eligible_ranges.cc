// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/inference/eligible_ranges.h"

#include <cassert>
#include <cstddef>
#include <memory>
#include <optional>
#include <string>
#include <string_view>
#include <utility>
#include <vector>

#include "absl/base/nullability.h"
#include "nullability/annotations.h"
#include "nullability/inference/inferable.h"
#include "nullability/inference/inference.proto.h"
#include "nullability/inference/usr_cache.h"
#include "nullability/loc_filter.h"
#include "nullability/type_nullability.h"
#include "clang-tidy/utils/LexerUtils.h"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclBase.h"
#include "clang/AST/ExprCXX.h"
#include "clang/AST/NestedNameSpecifier.h"
#include "clang/AST/RecursiveASTVisitor.h"
#include "clang/AST/TemplateBase.h"
#include "clang/AST/Type.h"
#include "clang/AST/TypeLoc.h"
#include "clang/Basic/CharInfo.h"
#include "clang/Basic/FileEntry.h"
#include "clang/Basic/LLVM.h"
#include "clang/Basic/LangOptions.h"
#include "clang/Basic/SourceLocation.h"
#include "clang/Basic/SourceManager.h"
#include "clang/Basic/Specifiers.h"
#include "clang/Basic/TokenKinds.h"
#include "clang/Index/USRGeneration.h"
#include "clang/Lex/Lexer.h"
#include "clang/Lex/Token.h"
#include "clang/Tooling/Transformer/SourceCode.h"
#include "llvm/ADT/STLFunctionalExtras.h"
#include "llvm/ADT/StringRef.h"
#include "llvm/ADT/StringSwitch.h"
#include "llvm/Support/Errc.h"
#include "llvm/Support/ErrorHandling.h"
#include "llvm/Support/ErrorOr.h"
#include "llvm/Support/Path.h"
#include "llvm/Support/raw_ostream.h"

namespace clang::tidy::nullability {
namespace {
using SlotNum = unsigned;
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

static SourceLocation getLocationAfterNewlinePrefixesIfIdentifier(Token Tok) {
  if (!Tok.is(tok::raw_identifier)) return Tok.getLocation();
  llvm::StringRef Identifier = Tok.getRawIdentifier();
  size_t OriginalSize = Identifier.size();
  int OffsetForEscapedNewlines =
      OriginalSize - skipEscapedNewLinePrefixes(Identifier).size();
  return Tok.getLocation().getLocWithOffset(OffsetForEscapedNewlines);
}

static bool isQualifierPositionAnnotation(StringRef Identifier) {
  return llvm::StringSwitch<bool>(skipEscapedNewLinePrefixes(Identifier))
      .Case(ClangNullable, true)
      .Case(ClangNonnull, true)
      .Case(ClangUnknown, true)
      .Case(AbslMacroNullable, true)
      .Case(AbslMacroNonnull, true)
      .Case(AbslMacroUnknown, true)
      .Default(false);
}

/// If the token immediately at or after `EndOfPtr` is a complete nullability
/// annotation, returns the end offset of the annotation. Else, returns
/// std::nullopt.
static std::optional<unsigned> getEndOffsetOfEastQualifierAnnotation(
    SourceLocation EndOfStar, const SourceManager &SM,
    const LangOptions &LangOpts, const FileID &DeclFID) {
  std::optional<Token> PossibleAttribute;
  Token AtEndOfStar;
  // The annotation may appear at `EndOfStar`, so check the token there first.
  // Skip whitespace, and if it is a comment, check the next token skipping
  // comments.
  if (bool Failed = Lexer::getRawToken(EndOfStar, AtEndOfStar, SM, LangOpts,
                                       /*IgnoreWhiteSpace=*/true);
      !Failed && !AtEndOfStar.is(tok::comment)) {
    PossibleAttribute = AtEndOfStar;
  } else {
    PossibleAttribute =
        utils::lexer::findNextTokenSkippingComments(EndOfStar, SM, LangOpts);
  }
  if (!PossibleAttribute) return std::nullopt;
  if (!PossibleAttribute->is(tok::raw_identifier)) return std::nullopt;

  const StringRef ID = PossibleAttribute->getRawIdentifier();
  if (!isQualifierPositionAnnotation(ID)) return std::nullopt;

  auto [FID, Offset] = SM.getDecomposedLoc(PossibleAttribute->getEndLoc());
  if (FID != DeclFID) return std::nullopt;

  return Offset;
}

/// If the token before `BeginningOfPtr` is a complete nullability
/// annotation, returns the begin offset of the annotation. Else, returns
/// std::nullopt.
static std::optional<unsigned> getBeginOffsetOfWestQualifierAnnotation(
    SourceLocation BeginningOfPtr, const SourceManager &SM,
    const LangOptions &LangOpts, const FileID &DeclFID) {
  Token PrevTok = utils::lexer::getPreviousToken(BeginningOfPtr, SM, LangOpts);
  if (!PrevTok.is(tok::raw_identifier)) return std::nullopt;

  if (!isQualifierPositionAnnotation(PrevTok.getRawIdentifier()))
    return std::nullopt;

  auto [FID, Offset] = SM.getDecomposedLoc(PrevTok.getLocation());
  if (FID != DeclFID) return std::nullopt;

  return Offset;
}

static bool isCVR(llvm::StringRef ID) {
  return llvm::StringSwitch<bool>(ID)
      .Case("const", true)
      .Case("volatile", true)
      .Case("restrict", true)
      .Default(false);
}

static SourceLocation includePrecedingCVRQualifiers(
    SourceLocation Begin, const SourceManager &SM, const LangOptions &LangOpts,
    bool AfterNewlinePrefixesIfIdentifier = true) {
  std::optional<Token> FinalQualifierSeen;
  // Update `Begin` as we search and find qualifier tokens.
  Token Tok = utils::lexer::getPreviousToken(Begin, SM, LangOpts);
  while (!Tok.is(tok::unknown)) {
    if (!Tok.is(tok::raw_identifier)) break;
    if (!isCVR(skipEscapedNewLinePrefixes(Tok.getRawIdentifier()))) break;
    FinalQualifierSeen = Tok;
    Begin = Tok.getLocation();
    Tok = utils::lexer::getPreviousToken(Begin, SM, LangOpts);
  }

  if (!FinalQualifierSeen.has_value() || !AfterNewlinePrefixesIfIdentifier)
    return Begin;
  assert(FinalQualifierSeen->is(tok::raw_identifier));
  return getLocationAfterNewlinePrefixesIfIdentifier(*FinalQualifierSeen);
}

static SourceLocation includeFollowingCVRQualifiers(
    SourceLocation End, const SourceManager &SM, const LangOptions &LangOpts) {
  Token Tok;
  Lexer::getRawToken(End, Tok, SM, LangOpts,
                     /*IgnoreWhiteSpace=*/true);
  if (Tok.is(tok::comment)) {
    Tok = utils::lexer::findNextTokenSkippingComments(End, SM, LangOpts)
              .value_or(Token());
  }

  std::optional<SourceLocation> LastQualifierEnd;
  // Update `End` as we search and find qualifier tokens.
  while (!Tok.is(tok::unknown) && !Tok.is(tok::eof)) {
    if (!Tok.is(tok::raw_identifier)) break;
    if (!isCVR(skipEscapedNewLinePrefixes(Tok.getRawIdentifier()))) break;
    LastQualifierEnd = Tok.getEndLoc();
    End = Tok.getLocation();
    Tok = utils::lexer::findNextTokenSkippingComments(End, SM, LangOpts)
              .value_or(Token());
  }
  return LastQualifierEnd.has_value() ? *LastQualifierEnd : End;
}

/// If the range specified by `Begin` and `End` is immediately wrapped in an
/// absl nullability annotation and is not a complex declarator, or if
/// `EndOfStarOffset` is immediately followed by a clang nullability attribute,
/// set the pre- and post-range lengths for that annotation/attribute.
static void addExistingAnnotationRemovalRanges(
    SourceLocation Begin, SourceLocation End, SourceLocation EndOfStar,
    unsigned BeginOffset, unsigned EndOffset, unsigned EndOfStarOffset,
    bool IsComplexDeclarator, const FileID &DeclFID, const SourceManager &SM,
    const LangOptions &LangOpts, SlotRange &Range) {
  if (std::optional<unsigned> AttributeEndOffset =
          getEndOffsetOfEastQualifierAnnotation(EndOfStar, SM, LangOpts,
                                                DeclFID)) {
    RemovalRange *Removal = Range.add_existing_annotation_removal_range();
    Removal->set_begin(EndOfStarOffset);
    Removal->set_end(*AttributeEndOffset);
    return;
  }
  if (std::optional<unsigned> AttributeBeginOffset =
          getBeginOffsetOfWestQualifierAnnotation(Begin, SM, LangOpts,
                                                  DeclFID)) {
    RemovalRange *Removal = Range.add_existing_annotation_removal_range();
    Removal->set_begin(*AttributeBeginOffset);
    Removal->set_end(BeginOffset);
    return;
  }
}

static StringRef skipOneEscapedNewlinePrefix(StringRef Str) {
  const char *Ptr = Str.data();
  size_t OriginalSize = Str.size();

  if (*Ptr == '\\') {
    Ptr++;
  } else {
    return Str;
  }

  // Whitespace is allowed after the `\`, but before the newline.
  while (Ptr < Str.data() + OriginalSize && isWhitespace(*Ptr)) {
    if (*Ptr == '\n' || *Ptr == '\r') {
      Ptr++;
      // `\n\r` and `\r\n` can be escaped by a single `\`, but not `\n\n` or
      // `\r\r`.
      if ((*Ptr == '\n' || *Ptr == '\r') && *Ptr != *(Ptr - 1)) {
        Ptr++;
      }
      return StringRef(Ptr, OriginalSize - (Ptr - Str.data()));
    }
    Ptr++;
  }

  return Str;
}

StringRef skipEscapedNewLinePrefixes(StringRef Str) {
  while (true) {
    StringRef New = skipOneEscapedNewlinePrefix(Str);
    if (New == Str) break;
    Str = New;
  }
  return Str;
}

static bool isComplexDeclarator(const Type *T) {
  if (T == nullptr) return false;
  while (T->isArrayType()) {
    T = T->getArrayElementTypeNoTypeQual();
    if (T->isPointerType()) return true;
  }
  if (T->isPointerType()) {
    if (T->getPointeeType()->isArrayType() ||
        T->getPointeeType()->isFunctionType())
      return true;
    return isComplexDeclarator(T->getPointeeType().getTypePtr());
  }
  return false;
}

static llvm::ErrorOr<std::optional<SourceLocation>> maybeSetEndOfStarOffset(
    TypeLoc TL, CharSourceRange &R, bool IsComplexDeclarator,
    const SourceManager &SM) {
  // For raw pointers, we want to add any post-star annotations immediately
  // after the `*` instead of at the end of the range. These locations are
  // different in the case of complex declarators, such as pointers to
  // functions or arrays and arrays of pointers.
  //
  // We don't need to compute this for smart pointers, because the post-star
  // annotation should always be added at the end of the range. There is no
  // analogous set of complex declarator cases where the smart pointer type is
  // actually in the middle of the range.
  auto PTL = TL.getUnqualifiedLoc().getAsAdjusted<PointerTypeLoc>();
  if (!PTL) return std::nullopt;

  SourceLocation StarLoc = SM.getSpellingLoc(PTL.getStarLoc());
  if (StarLoc.isInvalid()) {
    // Allow the end offset to be used if we can't find a star.
    return std::nullopt;
  }

  // If the star is in an entirely different file from the type's range, we do
  // not support annotation addition/removal for this range. This type is
  // likely partially constructed via macro and that macro is likely to be
  // used for multiple types, in a manner complex enough that it should
  // probably be examined by a human anyway.
  // One example that has been seen is the use of late binding patterns to
  // declare either functions or function pointers, and the star location for
  // the function pointers is in a macro used for every single function bound
  // that way.
  if (SM.getFileID(StarLoc) != SM.getFileID(R.getBegin())) {
    return llvm::errc::not_supported;
  }

  // If the star is not inside the range `R`, e.g. it's in a macro that may or
  // may not expand to the entire type range, then we will not set the offset
  // after the star.
  //
  // This will result in the end offset being used to insert any annotation.
  //
  // This works well for simple pointers.
  //
  // For complex declarators, in cases where the macro expands to the entire
  // type, we should already be looking at the macro definition range, so the
  // star should be inside the range. However, if the macro supplies only a
  // part of the range, we can end up with a star inside a macro that is
  // outside the range but still in the same file. This is unusual enough to
  // log, but can probably be supported by returning the star location.
  if (!(StarLoc >= R.getBegin() && StarLoc < R.getEnd())) {
    if (!IsComplexDeclarator) {
      return std::nullopt;
    }
    llvm::errs() << "Complex declarator with star outside range but in same "
                    "file. This is rare and it may be worth double-checking "
                    "the results for this range. Star location is "
                 << StarLoc.printToString(SM) << "\n";
  }

  return StarLoc.getLocWithOffset(1);
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
static void addRangesQualifierAware(const DeclaratorDecl *absl_nullable Decl,
                                    TypeLoc WholeLoc, SlotNum StartingSlot,
                                    const ASTContext &Context,
                                    const FileID &DeclFID,
                                    const TypeNullabilityDefaults &Defaults,
                                    EligibleRanges &Ranges) {
  std::vector<TypeNullabilityLoc> NullabilityLocs =
      getTypeNullabilityLocs(WholeLoc, Defaults);
  const auto &SM = Context.getSourceManager();
  const auto &LangOpts = Context.getLangOpts();
  for (auto &[SlotInLoc, T, MaybeLoc, Nullability] : NullabilityLocs) {
    if (!MaybeLoc || !isSupportedPointerType(MaybeLoc->getType())) continue;

    // We don't annotate bare template type arguments or bare `auto`, qualified
    // or not, or references to such types. For example, we would annotate only
    // the types of B, D, and G in
    // ```cc
    //   template <typename T>
    //   void f(T A, T* B, auto C, auto* D, const T& E) {
    //     auto F = A;
    //     auto* G = B;
    //     const auto& H = C;
    //   }
    // ```
    // The only known case of a bare `auto` range being included in
    // NullabilityLocs is in a function template instantiation with a template
    // parameter introduced by using `auto` as a function parameter type. Other
    // cases are not collected by the NullabilityWalker, and so don't need to be
    // skipped here.
    if (MaybeLoc->getUnqualifiedLoc().getAs<SubstTemplateTypeParmTypeLoc>()) {
      continue;
    }

    SourceRange SR = MaybeLoc->getSourceRange();
    std::optional<CharSourceRange> R;
    bool IsComplexDeclarator = isComplexDeclarator(MaybeLoc->getTypePtr());
    // For complex declarators, prefer a range in a macro definition, since we
    // can't annotate the type outside the macro if it is defined within one.
    // For simple pointers, we can and would prefer to annotate outside the
    // macro if the entire macro definition is the type, but this won't compile
    // for complex declarators.
    if (!IsComplexDeclarator) {
      R = tooling::getFileRange(CharSourceRange::getTokenRange(SR), Context,
                                /*IncludeMacroExpansion=*/true);
    }
    // If our first attempt at getting a file range failed or the type is a
    // complex declarator, and the range is spelled entirely within a macro, try
    // using the spelling locations. This gives us a chance of being able to
    // edit the macro definition
    if (!R && SR.getBegin().isMacroID() && SR.getEnd().isMacroID()) {
      SourceLocation SpellingBegin = SM.getSpellingLoc(SR.getBegin());
      SourceLocation SpellingEnd = SM.getSpellingLoc(SR.getEnd());
      if (SpellingBegin.isInvalid() || SpellingEnd.isInvalid()) continue;
      R = tooling::getFileRange(CharSourceRange::getTokenRange(
                                    SourceRange(SpellingBegin, SpellingEnd)),
                                Context,
                                /*IncludeMacroExpansion=*/true);
    }
    if (!R && IsComplexDeclarator) {
      R = tooling::getFileRange(CharSourceRange::getTokenRange(SR), Context,
                                /*IncludeMacroExpansion=*/true);
    }
    if (!R) continue;
    assert(R->getBegin().isValid() && R->getEnd().isValid());
    if (R->getBegin().isInvalid() || R->getEnd().isInvalid()) continue;

    auto [FID, BeginOffset] = SM.getDecomposedLoc(R->getBegin());
    // If the type comes from a different file, then don't attempt to edit -- it
    // might need manual intervention.
    if (FID != DeclFID) continue;

    unsigned EndOffset = SM.getFileOffset(R->getEnd());

    auto EndOfStarResult =
        maybeSetEndOfStarOffset(*MaybeLoc, *R, IsComplexDeclarator, SM);
    if (EndOfStarResult.getError()) {
      // An error indicates that the star location and range characteristics
      // invalidate our ability to annotate this range correctly, so we don't
      // add the range.
      continue;
    }

    // TODO(b/323509132) When we can infer more than just top-level pointers,
    // synchronize these slot numbers with inference's slot numbers. For now,
    // assign no slot to anything but a first slot in an inferable type.
    std::optional<Slot> SlotInContext =
        SlotInLoc == 0 && hasInferable(WholeLoc.getType())
            ? std::optional(Slot(StartingSlot + SlotInLoc))
            : std::nullopt;

    EligibleRange &Range = Ranges.emplace_back(SlotInContext);

    std::optional<SourceLocation> EndOfStarLoc = *EndOfStarResult;
    std::optional<unsigned> EndOfStarOffset;
    if (EndOfStarLoc) {
      EndOfStarOffset = SM.getFileOffset(*EndOfStarLoc);
      Range.Range.set_qualifier_annotation_insertion_offset(*EndOfStarOffset);
    }

    if (Nullability) {
      Range.Range.set_existing_annotation(toProtoNullability(*Nullability));
      bool UseEndOfStarLoc =
          EndOfStarLoc && EndOfStarLoc->isValid() && EndOfStarOffset;
      SourceLocation EndOfStarOrEnd =
          UseEndOfStarLoc ? *EndOfStarLoc : R->getEnd();
      unsigned EndOfStarOrEndOffset =
          UseEndOfStarLoc ? *EndOfStarOffset : EndOffset;
      addExistingAnnotationRemovalRanges(
          R->getBegin(), R->getEnd(), EndOfStarOrEnd, BeginOffset, EndOffset,
          EndOfStarOrEndOffset, IsComplexDeclarator, DeclFID, SM, LangOpts,
          Range.Range);

      if (Range.Range.existing_annotation_removal_range_size() == 0) {
        // Include any preceding/following CVR qualifiers that could be nearer
        // than the annotation and try again. Our preferred style is to not
        // include these closer to the pointer type than the nullability
        // annotation, but they can be placed closer. We don't check all
        // potential combinations of CVR placements, but having more than one of
        // these on a single type is very rare and would trigger
        // -Wduplicate-decl-specifier.
        SourceLocation BeginWithCVRs = includePrecedingCVRQualifiers(
            R->getBegin(), SM, LangOpts,
            // Intentionally get the location prior to any escaped newlines that
            // are considered part of the CVR qualifier token, since this is a
            // starting point for moving further backwards.
            /*AfterNewlinePrefixesIfIdentifier=*/false);
        unsigned BeginWithCVRsOffset = SM.getFileOffset(BeginWithCVRs);
        SourceLocation EndWithCVRs =
            includeFollowingCVRQualifiers(R->getEnd(), SM, LangOpts);
        unsigned EndWithCVRsOffset = SM.getFileOffset(EndWithCVRs);
        SourceLocation EndOfStarWithCVRs =
            includeFollowingCVRQualifiers(EndOfStarOrEnd, SM, LangOpts);
        unsigned EndOfStarWithCVRsOffset = SM.getFileOffset(EndOfStarWithCVRs);
        addExistingAnnotationRemovalRanges(
            BeginWithCVRs, EndWithCVRs, EndOfStarWithCVRs, BeginWithCVRsOffset,
            EndWithCVRsOffset, EndOfStarWithCVRsOffset, IsComplexDeclarator,
            DeclFID, SM, LangOpts, Range.Range);
      }
    }

    if (!Range.Range.has_qualifier_annotation_insertion_offset()) {
      if (MaybeLoc->getUnqualifiedLoc().getAsAdjusted<PointerTypeLoc>()) {
        Range.Range.set_qualifier_annotation_insertion_offset(EndOffset);
      } else {
        Range.Range.set_qualifier_annotation_insertion_offset(BeginOffset);
      }
    }
  }
}

static std::optional<std::string> getPath(FileID FID,
                                          const SourceManager &SrcMgr) {
  const clang::OptionalFileEntryRef Entry = SrcMgr.getFileEntryRefForID(FID);
  if (!Entry) return std::nullopt;
  llvm::SmallString<128> Path = Entry->getName();
  llvm::sys::path::remove_dots(Path, true);
  return std::string(Path);
}

static std::optional<Nullability> getPragmaNullability(
    FileID FID, const TypeNullabilityDefaults &Defaults) {
  // Don't use Defaults.get(File) in order to avoid treating a lack of pragma as
  // a pragma setting of Defaults.DefaultNullability.
  if (!Defaults.FileNullability) return std::nullopt;
  if (auto It = Defaults.FileNullability->find(FID);
      It != Defaults.FileNullability->end()) {
    return toProtoNullability(It->second);
  }
  return std::nullopt;
}

static EligibleRanges getEligibleRanges(
    const FunctionDecl &Fun, const TypeNullabilityDefaults &Defaults) {
  // NullabilityWalker doesn't work on dependent types.
  if (Fun.getReturnType()->isDependentType()) return {};
  for (const auto &Param : Fun.parameters()) {
    if (Param->getType()->isDependentType()) return {};
  }
  FunctionTypeLoc TyLoc = Fun.getFunctionTypeLoc();
  if (TyLoc.isNull()) return {};
  const clang::ASTContext &Context = Fun.getParentASTContext();
  const SourceManager &SrcMgr = Context.getSourceManager();
  FileID DeclFID = SrcMgr.getFileID(SrcMgr.getExpansionLoc(Fun.getLocation()));
  if (!DeclFID.isValid()) return {};

  std::optional<std::string> Path = getPath(DeclFID, SrcMgr);
  if (!Path) return {};

  EligibleRanges Result;
  addRangesQualifierAware(nullptr, TyLoc.getReturnLoc(), SLOT_RETURN_TYPE,
                          Context, DeclFID, Defaults, Result);

  for (int I = 0, N = Fun.getNumParams(); I < N; ++I) {
    const ParmVarDecl *P = Fun.getParamDecl(I);
    addRangesQualifierAware(P, P->getTypeSourceInfo()->getTypeLoc(),
                            SLOT_PARAM + I, Context, DeclFID, Defaults, Result);
  }

  if (Result.empty()) return {};

  std::optional<Nullability> PragmaNullability =
      getPragmaNullability(DeclFID, Defaults);
  for (EligibleRange &Range : Result) {
    Range.Range.set_path(*Path);
    if (PragmaNullability)
      Range.Range.set_pragma_nullability(*PragmaNullability);
  }

  return Result;
}

static EligibleRanges getEligibleRanges(
    const DeclaratorDecl &D, const TypeNullabilityDefaults &Defaults) {
  // NullabilityWalker doesn't work on dependent types.
  if (D.getType()->isDependentType()) return {};
  TypeLoc TyLoc = D.getTypeSourceInfo()->getTypeLoc();
  if (TyLoc.isNull()) return {};
  const clang::ASTContext &Context = D.getASTContext();
  const SourceManager &SrcMgr = Context.getSourceManager();
  FileID DeclFID = SrcMgr.getFileID(SrcMgr.getExpansionLoc(D.getLocation()));
  if (!DeclFID.isValid()) return {};

  std::optional<std::string> Path = getPath(DeclFID, SrcMgr);
  if (!Path) return {};

  EligibleRanges Result;
  addRangesQualifierAware(&D, TyLoc, Slot(0), Context, DeclFID, Defaults,
                          Result);
  if (Result.empty()) return {};

  std::optional<Nullability> PragmaNullability =
      getPragmaNullability(DeclFID, Defaults);
  for (EligibleRange &Range : Result) {
    Range.Range.set_path(*Path);
    if (PragmaNullability)
      Range.Range.set_pragma_nullability(*PragmaNullability);
  }
  return Result;
}

EligibleRanges getEligibleRanges(const Decl &D,
                                 const TypeNullabilityDefaults &Defaults) {
  // We'll never be able to edit a written type for an implicit declaration.
  if (D.isImplicit()) return {};
  if (const auto *Fun = clang::dyn_cast<FunctionDecl>(&D))
    return getEligibleRanges(*Fun, Defaults);
  if (const auto *Field = clang::dyn_cast<FieldDecl>(&D))
    return getEligibleRanges(*Field, Defaults);
  if (const auto *Var = clang::dyn_cast<VarDecl>(&D))
    return getEligibleRanges(*Var, Defaults);
  return {};
}

EligibleRanges getInferenceRanges(const Decl &D,
                                  const TypeNullabilityDefaults &Defaults) {
  if (!isInferenceTarget(D)) return {};
  return getEligibleRanges(D, Defaults);
}

namespace {
struct Walker : public RecursiveASTVisitor<Walker> {
  Walker(llvm::function_ref<void(const EligibleRange &)> Func,
         const TypeNullabilityDefaults &Defaults,
         std::unique_ptr<LocFilter> LocFilter, USRCache *absl_nullable USRs)
      : Func(Func),
        Defaults(Defaults),
        LocFilter(std::move(LocFilter)),
        USRs(USRs) {}

  llvm::function_ref<void(const EligibleRange &)> Func;
  // Must outlive the walker.
  const TypeNullabilityDefaults &Defaults;
  std::unique_ptr<LocFilter> LocFilter;
  // Must outlive the walker.
  USRCache *absl_nullable USRs;

  // We can't walk the nullabilities in templates themselves, but walking the
  // instantiations will let us at least see the templates that get used.
  bool shouldVisitTemplateInstantiations() const { return true; }

  template <typename DeclT>
  void insertPointerRanges(const DeclT *absl_nonnull Decl) {
    // We can't walk the nullabilities in dependent contexts.
    if (Decl->getDeclContext()->isDependentContext()) return;
    if (!LocFilter->check(Decl->getBeginLoc())) return;
    EligibleRanges Ranges = getEligibleRanges(*Decl, Defaults);
    if (USRs) {
      for (EligibleRange &Range : Ranges) {
        std::string_view USR = getOrGenerateUSR(*USRs, *Decl);
        if (!USR.empty()) Range.USR = std::string(USR);
      }
    }

    for (const EligibleRange &Range : Ranges) {
      Func(Range);
    }
  }

  bool VisitFunctionDecl(const FunctionDecl *absl_nonnull FD) {
    // We can't walk the nullabilities of dependent contexts.
    if (FD->isDependentContext()) return true;
    insertPointerRanges(FD);
    return true;
  }

  bool VisitFieldDecl(const FieldDecl *absl_nonnull FD) {
    insertPointerRanges(FD);
    return true;
  }

  bool VisitVarDecl(const VarDecl *absl_nonnull VD) {
    // We'll see these as part of function decls.
    if (isa<ParmVarDecl>(VD)) return true;
    // We can't walk the nullabilities in templates.
    if (VD->isTemplated()) return true;
    insertPointerRanges(VD);
    return true;
  }

  bool VisitLambdaExpr(const LambdaExpr *absl_nonnull LE) {
    if (LE->hasExplicitParameters() || LE->hasExplicitResultType()) {
      insertPointerRanges(LE->getCallOperator());
    }

    return true;
  }
};
}  // namespace

void forAllEligibleRanges(llvm::function_ref<void(const EligibleRange &)> Func,
                          ASTContext &Ctx,
                          const TypeNullabilityDefaults &Defaults,
                          USRCache *absl_nullable USRs,
                          bool RestrictToMainFileOrHeader) {
  Walker W(
      Func, Defaults,
      getLocFilter(Ctx.getSourceManager(),
                   RestrictToMainFileOrHeader ? LocFilterKind::kMainFileOrHeader
                                              : LocFilterKind::kAllowAll),
      USRs);
  W.TraverseAST(Ctx);
}

}  // namespace clang::tidy::nullability
