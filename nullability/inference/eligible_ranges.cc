// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/inference/eligible_ranges.h"

#include <algorithm>
#include <cassert>
#include <iterator>
#include <memory>
#include <optional>
#include <string>
#include <utility>
#include <vector>

#include "absl/base/nullability.h"
#include "absl/log/check.h"
#include "nullability/inference/inferable.h"
#include "nullability/inference/inference.proto.h"
#include "nullability/loc_filter.h"
#include "nullability/type_nullability.h"
#include "third_party/llvm/llvm-project/clang-tools-extra/clang-tidy/utils/LexerUtils.h"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclBase.h"
#include "clang/AST/ExprCXX.h"
#include "clang/AST/NestedNameSpecifier.h"
#include "clang/AST/RecursiveASTVisitor.h"
#include "clang/AST/TemplateBase.h"
#include "clang/AST/Type.h"
#include "clang/AST/TypeLoc.h"
#include "clang/AST/TypeLocVisitor.h"
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
#include "llvm/ADT/StringRef.h"
#include "llvm/Support/ErrorHandling.h"
#include "llvm/Support/Path.h"

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

static void initSlotRange(SlotRange &R, SlotNum SlotInType, unsigned Begin,
                          unsigned End,
                          std::optional<NullabilityKind> Nullability) {
  R.set_slot_in_type(SlotInType);
  R.set_begin(Begin);
  R.set_end(End);
  if (Nullability) {
    R.set_existing_annotation(toProtoNullability(*Nullability));
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
  // and one for each identifier. Same for absl::Nonnull< and absl::Nullable<.
  Token PrevTok = utils::lexer::getPreviousToken(Begin, SM, LangOpts);
  if (!PrevTok.is(tok::less)) return {};
  if (PrevTok =
          utils::lexer::getPreviousToken(PrevTok.getLocation(), SM, LangOpts);
      !PrevTok.is(tok::raw_identifier))
    return {};
  if (const StringRef ID = PrevTok.getRawIdentifier();
      ID != "NullabilityUnknown" && ID != "Nullable" && ID != "Nonnull")
    return {};
  if (PrevTok =
          utils::lexer::getPreviousToken(PrevTok.getLocation(), SM, LangOpts);
      PrevTok.isNot(tok::coloncolon))
    return {};
  if (PrevTok =
          utils::lexer::getPreviousToken(PrevTok.getLocation(), SM, LangOpts);
      !PrevTok.is(tok::raw_identifier))
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
  if (!NextTok.is(tok::greater) && !NextTok.is(tok::greatergreater)) {
    std::optional<Token> MaybeNextTok =
        utils::lexer::findNextTokenSkippingComments(End, SM, LangOpts);
    if (!MaybeNextTok || (!MaybeNextTok->is(tok::greater) &&
                          !MaybeNextTok->is(tok::greatergreater)))
      return {};
    NextTok = *MaybeNextTok;
  }

  auto [NextTokFID, NextTokOffset] = SM.getDecomposedLoc(NextTok.getEndLoc());
  if (NextTokFID != DeclFID) return {};
  if (NextTok.is(tok::greatergreater)) {
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
  if (!NextTok->is(tok::raw_identifier)) return std::nullopt;
  if (const StringRef ID = NextTok->getRawIdentifier();
      ID != "_Null_unspecified" && ID != "_Nonnull" && ID != "_Nullable")
    return std::nullopt;

  auto [FID, Offset] = SM.getDecomposedLoc(NextTok->getEndLoc());
  if (FID != DeclFID) return std::nullopt;

  return Offset;
}

/// If the range specified by `Begin` and `End` is immediately wrapped in an
/// absl nullability annotation or immediately followed by a clang nullability
/// attribute, set the pre- and post-range lengths for that
/// annotation/attribute.
static void addAnnotationPreAndPostRangeLength(
    SourceLocation Begin, SourceLocation End, unsigned BeginOffset,
    unsigned EndOffset, const FileID &DeclFID, const SourceManager &SM,
    const LangOptions &LangOpts, SlotRange &Range) {
  auto [AnnotationStartOffset, AnnotationEndOffset] =
      getStartAndEndOffsetsOfImmediateAbslAnnotation(Begin, End, SM, LangOpts,
                                                     DeclFID);
  if (AnnotationStartOffset && AnnotationEndOffset) {
    Range.set_existing_annotation_pre_range_length(BeginOffset -
                                                   *AnnotationStartOffset);
    Range.set_existing_annotation_post_range_length(*AnnotationEndOffset -
                                                    EndOffset);
  } else if (std::optional<unsigned> AttributeEndOffset =
                 getEndOffsetOfImmediateClangAttribute(End, SM, LangOpts,
                                                       DeclFID)) {
    Range.set_existing_annotation_pre_range_length(0);
    Range.set_existing_annotation_post_range_length(*AttributeEndOffset -
                                                    EndOffset);
  }
}

/// Declarations involving combinations of pointers, arrays, and functions can
/// require re-arrangement of the type to add or remove nullability annotations.
///
/// The known relevant types are (potentially nested) raw pointers to arrays or
/// functions and (potentially nested) arrays of raw pointers.
///
/// e.g. a length-3 array of length-2 arrays of pointers to length-1 arrays of
/// int* with the name `p` would start as `int* (*p[3][2])[1]` and if we need to
/// mark the type of the pointers to the length-1 arrays Nullable, we would need
/// to re-write this is as `Nullable<int* (*)[1]> p[3][2]`.
///
/// Similarly, a function pointer with the name `f` with a single int*
/// parameter named `a` might start as `void(*f)(int* a)`. If we need to mark
/// the function pointer as Nullable, we would need to re-write this as
/// `Nullable<void(*)(int* a)> f`.
///
/// If Decl and its TypeLoc TL are such a declaration, returns a vector of
/// optional ComplexDeclaratorRanges, indexed by nullability slot in TL, to be
/// added to the corresponding SlotRange.
///
/// e.g. For `f` above, returns [{"f", [{7, 8}]}, std::nullopt]. For `p` above,
/// returns [{"p", [{8, 15}]}, std::nullopt]. And for `void (*(*(f))[])(int)`,
/// returns [{"(f)", [{10, 13}]}, {"(*)[]", [{8, 10}, {13, 16}]}].
///
/// The ranges are produced under the assumption that all slots will receive an
/// annotation. The types would need to be modified differently if e.g. some
/// inner slots are annotated but outer slots are not.
///
/// Template parameters are not considered, nor are smart pointers, so types
/// containing these may not have the correct number of results returned and the
/// results should not be used.
static std::vector<std::optional<ComplexDeclaratorRanges>>
getComplexDeclaratorRanges(const DeclaratorDecl &Decl, TypeLoc TL) {
  class Walker : public TypeLocVisitor<Walker> {
   public:
    std::vector<std::optional<ComplexDeclaratorRanges>> Results;

    Walker(const DeclaratorDecl &Decl)
        : SM(Decl.getASTContext().getSourceManager()), LO(Decl.getLangOpts()) {
      // Include any name for Decl in the range for the first slot.
      StartForSlot = Decl.getLocation();
      if (Decl.getDeclName().isEmpty()) {
        EndForSlot = StartForSlot;
      } else {
        std::optional<Token> NextTok =
            utils::lexer::findNextTokenSkippingComments(StartForSlot, SM, LO);
        if (NextTok) {
          EndForSlot = NextTok->getLocation();
        } else {
          EndForSlot = StartForSlot;
        }
      }
    }

    void Visit(TypeLoc TL) {
      // If the type ends before the start of the name, then Decl is not a
      // complex declarator.
      if (TL.getEndLoc() <= StartForSlot) {
        unsigned PointersInTL = countPointersInType(TL.getType());
        Results.reserve(PointersInTL);
        for (int I = 0; I < PointersInTL; ++I) Results.push_back(std::nullopt);
      } else {
        TypeLocVisitor::Visit(TL);
      }
    }

    void VisitTypeLoc(TypeLoc TL) {
      if (TL.getNextTypeLoc()) TypeLocVisitor::Visit(TL.getNextTypeLoc());
    }

    void VisitArrayTypeLoc(ArrayTypeLoc ArrayTL) {
      // Move the end marker to the right to include the array brackets,
      // including any size expression.
      EndForSlot = ArrayTL.getRBracketLoc().getLocWithOffset(1);
      Visit(ArrayTL.getElementLoc());
    }

    void VisitParenTypeLoc(ParenTypeLoc ParenTL) {
      StartForSlot = ParenTL.getLParenLoc();
      EndForSlot = ParenTL.getRParenLoc().getLocWithOffset(1);
      Visit(ParenTL.getInnerLoc());
    }

    void VisitPointerTypeLoc(PointerTypeLoc PointerTL) {
      if (StartForSlot == EndForSlot) {
        Results.push_back(std::nullopt);
      } else {
        // Save the result for the current slot.
        auto &Result = Results.emplace_back(ComplexDeclaratorRanges());
        if (StartForPreviousSlot && EndForPreviousSlot) {
          Result->set_following_annotation(
              (Lexer::getSourceText(clang::CharSourceRange::getCharRange(
                                        StartForSlot, *StartForPreviousSlot),
                                    SM, LO) +
               Lexer::getSourceText(clang::CharSourceRange::getCharRange(
                                        *EndForPreviousSlot, EndForSlot),
                                    SM, LO))
                  .str());

          if (StartForPreviousSlot != StartForSlot) {
            auto *Removal = Result->add_removal();
            Removal->set_begin(SM.getFileOffset(StartForSlot));
            Removal->set_end(SM.getFileOffset(*StartForPreviousSlot));
          }
          if (EndForPreviousSlot != EndForSlot) {
            auto *Removal = Result->add_removal();
            Removal->set_begin(SM.getFileOffset(*EndForPreviousSlot));
            Removal->set_end(SM.getFileOffset(EndForSlot));
          }
        } else {
          Result->set_following_annotation(Lexer::getSourceText(
              clang::CharSourceRange::getCharRange(StartForSlot, EndForSlot),
              SM, LO));
          auto &Removal = *Result->add_removal();
          Removal.set_begin(SM.getFileOffset(StartForSlot));
          Removal.set_end(SM.getFileOffset(EndForSlot));
        }
      }

      // Prepare for the next slot.
      StartForPreviousSlot = StartForSlot;
      EndForPreviousSlot = EndForSlot;

      // Move the begin marker to include the star to prepare for the next slot.
      StartForSlot = PointerTL.getStarLoc();

      Visit(PointerTL.getPointeeLoc());
    }

    void VisitFunctionProtoTypeLoc(FunctionProtoTypeLoc FuncTL) {
      Visit(FuncTL.getReturnLoc());
      for (const auto &ParamDecl : FuncTL.getParams()) {
        if (auto *TSI = ParamDecl->getTypeSourceInfo()) {
          Walker Recurse(*ParamDecl);
          Recurse.Visit(TSI->getTypeLoc());
          for (const auto &Result : Recurse.Results) {
            Results.push_back(std::move(Result));
          }
        }
      }
    }

   private:
    const SourceManager &SM;
    const LangOptions &LO;
    SourceLocation StartForSlot;
    SourceLocation EndForSlot;
    std::optional<SourceLocation> StartForPreviousSlot;
    std::optional<SourceLocation> EndForPreviousSlot;
  };

  Walker W(Decl);
  W.Visit(TL);
  return std::move(W.Results);
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
static void addRangesQualifierAware(absl::Nullable<const DeclaratorDecl *> Decl,
                                    TypeLoc WholeLoc, SlotNum StartingSlot,
                                    const ASTContext &Context,
                                    const FileID &DeclFID,
                                    const TypeNullabilityDefaults &Defaults,
                                    EligibleRanges &Ranges) {
  std::vector<TypeNullabilityLoc> NullabilityLocs =
      getTypeNullabilityLocs(WholeLoc, Defaults);
  std::vector<std::optional<ComplexDeclaratorRanges>>
      AllComplexDeclaratorRanges;
  if (Decl) {
    AllComplexDeclaratorRanges = getComplexDeclaratorRanges(*Decl, WholeLoc);
  }
  const auto &SM = Context.getSourceManager();
  const auto &LangOpts = Context.getLangOpts();
  for (auto &[SlotInLoc, T, MaybeLoc, Nullability] : NullabilityLocs) {
    if (!MaybeLoc || !isSupportedPointerType(MaybeLoc->getType())) continue;
    auto R = tooling::getFileRange(
        CharSourceRange::getTokenRange(MaybeLoc->getSourceRange()), Context,
        /*IncludeMacroExpansion=*/true);
    if (!R) continue;

    // The start of the new range.
    SourceLocation Begin = R->getBegin();

    // We don't annotate bare template type arguments or bare `auto`.
    // For example, we would annotate only the types of B, D, and F in
    // ```cc
    //   template <typename T>
    //   void f(T A, T* B, auto C, auto* D) {
    //     auto E = A;
    //     auto* F = B;
    //   }
    // ```
    // The only known case of a bare `auto` range being included in
    // NullabilityLocs is in a function template instantiation with a template
    // parameter introduced by using `auto` as a function parameter type. Other
    // cases are not collected by the NullabilityWalker, and so don't need to be
    // skipped here.
    if (MaybeLoc->getAs<SubstTemplateTypeParmTypeLoc>()) {
      continue;
    }

    // Update `Begin` as we search backwards and find qualifier tokens.
    auto PrevTok = utils::lexer::getPreviousToken(Begin, SM, LangOpts);
    while (PrevTok.getKind() != tok::unknown) {
      if (!PrevTok.is(tok::raw_identifier)) break;
      StringRef RawID = PrevTok.getRawIdentifier();
      if (RawID != "const" && RawID != "volatile" && RawID != "restrict") break;
      Begin = PrevTok.getLocation();
      PrevTok = utils::lexer::getPreviousToken(Begin, SM, LangOpts);
    }

    auto [FID, BeginOffset] = SM.getDecomposedLoc(Begin);
    // If the type comes from a different file, then don't attempt to edit -- it
    // might need manual intervention.
    if (FID != DeclFID) continue;

    unsigned EndOffset = SM.getFileOffset(R->getEnd());

    // TODO(b/323509132) When we can infer more than just top-level pointers,
    // synchronize these slot numbers with inference's slot numbers. For now,
    // assign no slot to anything but a first slot in an inferable type.
    std::optional<Slot> SlotInContext =
        SlotInLoc == 0 && hasInferable(WholeLoc.getType())
            ? std::optional(Slot(StartingSlot + SlotInLoc))
            : std::nullopt;

    EligibleRange &Range = Ranges.emplace_back(SlotInContext);
    initSlotRange(Range.Range, SlotInLoc, BeginOffset, EndOffset, Nullability);
    if (Nullability)
      addAnnotationPreAndPostRangeLength(Begin, R->getEnd(), BeginOffset,
                                         EndOffset, DeclFID, SM, LangOpts,
                                         Range.Range);

    // If we don't have a std::nullopt or ComplexDeclaratorRange for every slot,
    // don't add any ComplexDeclaratorRanges. The Decl is a complex declarator
    // but contains at least one unsupported slot syntax, such as slots in
    // template parameters or smart pointers.
    if (Decl && AllComplexDeclaratorRanges.size() == NullabilityLocs.size()) {
      CHECK(AllComplexDeclaratorRanges.size() > SlotInLoc);
      std::optional<ComplexDeclaratorRanges> &CDR =
          AllComplexDeclaratorRanges[SlotInLoc];
      // If all removal ranges are after the end of the range to enclose in the
      // annotation, then we don't need to add any ComplexDeclaratorRanges and
      // can leave the text where it is.
      if (CDR && std::any_of(CDR->removal().begin(), CDR->removal().end(),
                             [EndOffset](const RemovalRange &Removal) {
                               return Removal.begin() < EndOffset;
                             })) {
        *Range.Range.mutable_complex_declarator_ranges() = std::move(*CDR);
      }
    }

    auto PTL = MaybeLoc->getAsAdjusted<PointerTypeLoc>();
    if (PTL) {
      while (auto PointeeTL =
                 PTL.getPointeeLoc().getAsAdjusted<PointerTypeLoc>()) {
        PTL = PointeeTL;
      }
      if (PTL.getPointeeLoc().getAs<AutoTypeLoc>()) {
        Range.Range.set_contains_auto_star(true);
      }
    }
  }
}

static std::optional<std::string> getPath(FileID FID,
                                          const SourceManager &SrcMgr) {
  const clang::OptionalFileEntryRef Entry = SrcMgr.getFileEntryRefForID(FID);
  if (!Entry) return std::nullopt;
  return std::string(
      llvm::sys::path::remove_leading_dotslash(Entry->getName()));
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
  Walker(const TypeNullabilityDefaults &Defaults,
         std::unique_ptr<LocFilter> LocFilter)
      : Defaults(Defaults), LocFilter(std::move(LocFilter)) {}

  // Must outlive the walker.
  const TypeNullabilityDefaults &Defaults;
  EligibleRanges Out;
  std::unique_ptr<LocFilter> LocFilter;

  // We can't walk the nullabilities in templates themselves, but walking the
  // instantiations will let us at least see the templates that get used.
  bool shouldVisitTemplateInstantiations() const { return true; }

  template <typename DeclT>
  void insertPointerRanges(absl::Nonnull<const DeclT *> Decl) {
    if (!LocFilter->check(Decl->getBeginLoc())) return;
    EligibleRanges Ranges = getEligibleRanges(*Decl, Defaults);
    Out.reserve(Out.size() + Ranges.size());
    std::move(Ranges.begin(), Ranges.end(), std::back_inserter(Out));
  }

  bool VisitFunctionDecl(absl::Nonnull<const FunctionDecl *> FD) {
    insertPointerRanges(FD);
    return true;
  }

  bool VisitFieldDecl(absl::Nonnull<const FieldDecl *> FD) {
    insertPointerRanges(FD);
    return true;
  }

  bool VisitVarDecl(absl::Nonnull<const VarDecl *> VD) {
    // We'll see these as part of function decls.
    if (isa<ParmVarDecl>(VD)) return true;

    insertPointerRanges(VD);
    return true;
  }

  bool VisitLambdaExpr(absl::Nonnull<const LambdaExpr *> LE) {
    if (LE->hasExplicitParameters() || LE->hasExplicitResultType()) {
      insertPointerRanges(LE->getCallOperator());
    }

    return true;
  }
};
}  // namespace

EligibleRanges getEligibleRanges(ASTContext &Ctx,
                                 const TypeNullabilityDefaults &Defaults,
                                 bool RestrictToMainFileOrHeader) {
  Walker W(Defaults,
           getLocFilter(Ctx.getSourceManager(), RestrictToMainFileOrHeader));
  W.TraverseAST(Ctx);
  return std::move(W.Out);
}

}  // namespace clang::tidy::nullability
