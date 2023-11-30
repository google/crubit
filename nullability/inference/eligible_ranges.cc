// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/inference/eligible_ranges.h"

#include <cassert>
#include <optional>
#include <string_view>

#include "nullability/inference/inference.proto.h"
#include "nullability/type_nullability.h"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclBase.h"
#include "clang/AST/Type.h"
#include "clang/AST/TypeLoc.h"
#include "clang/Basic/FileEntry.h"
#include "clang/Basic/LLVM.h"
#include "clang/Basic/SourceLocation.h"
#include "clang/Basic/SourceManager.h"
#include "clang/Index/USRGeneration.h"
#include "clang/Tooling/Transformer/SourceCode.h"

namespace clang::tidy::nullability {

namespace {
bool isEligibleTypeLoc(TypeLoc TyLoc) {
  QualType Ty = TyLoc.getType();
  if (!isSupportedPointerType(Ty)) return false;
  // Excludes function-pointer types, which have an odd syntax, and
  // `const`-qualified pointees, whose range is not correctly identified.
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
    // TODO: Allow local `const` qualifiers, function-pointer types and other
    // corner cases that are currently excluded, because they use declarator
    // suffix syntax. These aren't inherently bad, just tricky to edit
    // correctly. This change will require generalizing this function to take
    // more than just a `TypeLoc`, the declarator may be needed as well.
    QualType PointeeTy = PT->getPointeeType();
    // We do not consider the `const`-ness of `PT` itself, because the edit will
    // do the correct thing implicitly: the `const` will be left out of the
    // associated `TypeLoc`, leaving `const` outside the nullability annotation,
    // which is the preferred spelling.
    if (PointeeTy->isFunctionType() || PointeeTy->isArrayType() ||
        PointeeTy.isLocalConstQualified())
      return false;
  }
  return true;
}

std::optional<TypeLocRanges> getEligibleRanges(const FunctionDecl &Fun) {
  FunctionTypeLoc TyLoc = Fun.getFunctionTypeLoc();
  if (TyLoc.isNull()) return std::nullopt;

  const clang::ASTContext &Context = Fun.getParentASTContext();
  const SourceManager &SrcMgr = Context.getSourceManager();
  TypeLocRanges Result;

  // Guarantees that `Loc` is eligible for editing, including that its begin and
  // end locations are in the same file.
  auto GetLocRange = [&Context](TypeLoc Loc) -> std::optional<CharSourceRange> {
    if (!isEligibleTypeLoc(Loc)) return std::nullopt;
    return clang::tooling::getFileRange(
        CharSourceRange::getTokenRange(Loc.getSourceRange()), Context,
        /*IncludeMacroExpansion=*/true);
  };

  FileID DeclFID = SrcMgr.getFileID(SrcMgr.getExpansionLoc(Fun.getLocation()));

  if (auto CSR = GetLocRange(TyLoc.getReturnLoc())) {
    auto [FID, Begin] = SrcMgr.getDecomposedLoc(CSR->getBegin());
    // If the type comes from a different file, then don't attempt to edit -- it
    // might need manual intervention.
    if (FID == DeclFID) {
      auto *R = Result.add_range();
      R->set_slot(SLOT_RETURN_TYPE);
      R->set_begin(Begin);
      R->set_end(SrcMgr.getFileOffset(CSR->getEnd()));
    }
  }

  for (int I = 0, N = Fun.getNumParams(); I < N; ++I) {
    const ParmVarDecl *P = Fun.getParamDecl(I);
    if (auto CSR = GetLocRange(P->getTypeSourceInfo()->getTypeLoc())) {
      auto [FID, Begin] = SrcMgr.getDecomposedLoc(CSR->getBegin());
      // If the type comes from a different file, then don't attempt to edit --
      // it might need manual intervention.
      if (FID != DeclFID) continue;
      auto *R = Result.add_range();
      R->set_slot(SLOT_PARAM + I);
      R->set_begin(Begin);
      R->set_end(SrcMgr.getFileOffset(CSR->getEnd()));
    }
  }

  if (Result.range().empty()) return std::nullopt;
  // Extract the path in which `Fun` is located.
  const clang::FileEntry *Entry =
      Context.getSourceManager().getFileEntryForID(DeclFID);
  if (Entry == nullptr) return std::nullopt;
  Result.set_path(std::string_view(Entry->getName()));
  return Result;
}

}  // namespace

std::optional<TypeLocRanges> getEligibleRanges(const Decl &D) {
  // Only functions are currently supported.
  if (const auto *Fun = clang::dyn_cast<FunctionDecl>(&D))
    return getEligibleRanges(*Fun);
  return std::nullopt;
}

}  // namespace clang::tidy::nullability
