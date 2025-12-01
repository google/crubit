// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/inference/collect_evidence.h"

#include <algorithm>
#include <cassert>
#include <cstdint>
#include <memory>
#include <optional>
#include <string>
#include <string_view>
#include <type_traits>
#include <utility>
#include <vector>

#include "absl/base/nullability.h"
#include "absl/container/flat_hash_map.h"
#include "absl/log/check.h"
#include "nullability/ast_helpers.h"
#include "nullability/forwarding_functions.h"
#include "nullability/inference/inferable.h"
#include "nullability/inference/inference.proto.h"
#include "nullability/inference/slot_fingerprint.h"
#include "nullability/inference/usr_cache.h"
#include "nullability/loc_filter.h"
#include "nullability/macro_arg_capture.h"
#include "nullability/pointer_nullability.h"
#include "nullability/pointer_nullability_analysis.h"
#include "nullability/pointer_nullability_lattice.h"
#include "nullability/pragma.h"
#include "nullability/type_nullability.h"
#include "clang/AST/ASTContext.h"
#include "clang/AST/Attr.h"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclBase.h"
#include "clang/AST/DeclCXX.h"
#include "clang/AST/DeclGroup.h"
#include "clang/AST/DeclTemplate.h"
#include "clang/AST/Expr.h"
#include "clang/AST/ExprCXX.h"
#include "clang/AST/OperationKinds.h"
#include "clang/AST/RecursiveASTVisitor.h"
#include "clang/AST/Stmt.h"
#include "clang/AST/Type.h"
#include "clang/AST/TypeBase.h"
#include "clang/AST/TypeLoc.h"
#include "clang/Analysis/CFG.h"
#include "clang/Analysis/FlowSensitive/ASTOps.h"
#include "clang/Analysis/FlowSensitive/AdornedCFG.h"
#include "clang/Analysis/FlowSensitive/Arena.h"
#include "clang/Analysis/FlowSensitive/DataflowAnalysis.h"
#include "clang/Analysis/FlowSensitive/DataflowAnalysisContext.h"
#include "clang/Analysis/FlowSensitive/DataflowEnvironment.h"
#include "clang/Analysis/FlowSensitive/Formula.h"
#include "clang/Analysis/FlowSensitive/FormulaSerialization.h"
#include "clang/Analysis/FlowSensitive/Solver.h"
#include "clang/Analysis/FlowSensitive/StorageLocation.h"
#include "clang/Analysis/FlowSensitive/Value.h"
#include "clang/Analysis/FlowSensitive/WatchedLiteralsSolver.h"
#include "clang/Basic/Builtins.h"
#include "clang/Basic/LLVM.h"
#include "clang/Basic/OperatorKinds.h"
#include "clang/Basic/SourceLocation.h"
#include "clang/Basic/Specifiers.h"
#include "clang/Index/USRGeneration.h"
#include "llvm/ADT/ArrayRef.h"
#include "llvm/ADT/DenseMap.h"
#include "llvm/ADT/DenseSet.h"
#include "llvm/ADT/FunctionExtras.h"
#include "llvm/ADT/STLFunctionalExtras.h"
#include "llvm/ADT/StringRef.h"
#include "llvm/ADT/StringSet.h"
#include "llvm/Support/Errc.h"
#include "llvm/Support/Error.h"
#include "llvm/Support/ErrorHandling.h"
#include "llvm/Support/raw_ostream.h"
#include "google/protobuf/repeated_ptr_field.h"

namespace clang::tidy::nullability {
using ::clang::dataflow::Atom;
using ::clang::dataflow::DataflowAnalysisContext;
using ::clang::dataflow::Environment;
using ::clang::dataflow::Formula;
using ::clang::dataflow::RecordInitListHelper;
using ::clang::dataflow::WatchedLiteralsSolver;

using ConcreteNullabilityCache =
    absl::flat_hash_map<const Decl *,
                        std::optional<const PointerTypeNullability>>;

namespace {
/// Shared base class for visitors that walk the AST for evidence collection
/// purposes, to ensure they see the same nodes.
template <typename Derived>
struct EvidenceLocationsWalker : public RecursiveASTVisitor<Derived> {
  // We do want to see concrete code, including function instantiations.
  bool shouldVisitTemplateInstantiations() const { return true; }

  // In order to collect from more default member initializers, we do want to
  // see defaulted default constructors, which are implicitly-defined
  // functions whether the declaration is implicit or explicit. We also want
  // to see lambda bodies in the form of operator() definitions that are not
  // themselves implicit but show up in an implicit context.
  bool shouldVisitImplicitCode() const { return true; }
};
}  // namespace

// Returns a LocFilter that allows testing if the SourceLocation is in a
// GoogleTest "main" file (by inverting, and allowing locations in all files but
// the main file). Returns nullptr if the `Ctx` does not have any recognized
// GoogleTest macros, which means that no file in the TU is a GoogleTest main
// file.
static std::unique_ptr<LocFilter> getNotTestMainFileLocFilter(ASTContext& Ctx) {
  if (Ctx.Idents.get("ASSERT_NE").hasMacroDefinition() &&
      Ctx.Idents.get("GTEST_TEST").hasMacroDefinition()) {
    return nullability::getLocFilter(
        Ctx.getSourceManager(),
        nullability::LocFilterKind::kAllowAllButNotMainFile);
  }
  return nullptr;
}

VirtualMethodIndex getVirtualMethodIndex(ASTContext &Ctx, USRCache &UC) {
  struct Walker : public EvidenceLocationsWalker<Walker> {
    explicit Walker(USRCache &UC) : USRCache(UC) {}
    VirtualMethodIndex Index;
    USRCache &USRCache;

    // Note: the correctness of this function relies on the pre-order traversal
    // order of `Walker` (which is the default for RecursiveASTVisitor), because
    // `Derived` must be visited *after* any methods it overrides, so that they
    // are in `OverriddenMap`.
    llvm::StringSet<> getOverridden(const CXXMethodDecl *absl_nonnull Derived) {
      llvm::StringSet<> Overridden;
      for (const CXXMethodDecl *Base : Derived->overridden_methods()) {
        if (Base == nullptr) continue;
        std::string_view BaseUSR = getOrGenerateUSR(USRCache, *Base);
        if (BaseUSR.empty()) continue;
        Overridden.insert(BaseUSR);
        auto It = Index.Bases.find(BaseUSR);
        if (It != Index.Bases.end()) {
          auto &BaseOverrides = It->second;
          Overridden.insert(BaseOverrides.begin(), BaseOverrides.end());
        }
      }
      return Overridden;
    }

    bool VisitCXXMethodDecl(const CXXMethodDecl *MD) {
      if (MD && MD->isVirtual()) {
        llvm::StringSet<> Overridden = getOverridden(MD);
        if (Overridden.empty()) return true;

        // Filter for Nullability relevance. Optimization note: we filter
        // *after* calling getOverridden on the assumption that, for irrelevant
        // methods, it is cheaper, on average, to call `getOverridden` than
        // `getInferableSlotIndices`. But, no data informed this choice.
        llvm::SmallVector<int> SlotIndices = getInferableSlotIndices(*MD);
        // No slots -> irrelevant method.
        if (SlotIndices.empty()) return true;

        std::string_view USR = getOrGenerateUSR(USRCache, *MD);
        if (USR.empty()) return true;

        for (auto &O : Overridden) {
          auto &S = Index.Overrides[O.getKey()];
          // MD must have the same inferable slot indices as any methods it
          // overrides, so we can set their SlotIndices from MD's.
          S.SlotIndices = SlotIndices;
          S.OverridingUSRs.insert(USR);
        }
        Index.Bases[USR] = std::move(Overridden);
      }
      return true;
    }
  };

  // Don't use a LocFilter here to restrict to the main file or header, because
  // we want to propagate evidence from virtual methods to/from their overrides,
  // no matter where they are each defined.
  Walker W(UC);
  W.TraverseAST(Ctx);
  return std::move(W.Index);
}

RelatedSymbols saveVirtualMethodsMap(const RelatedVirtualMethodsMap &M) {
  RelatedSymbols Result;
  auto &RelatedMethods = *Result.mutable_related_symbols();
  for (auto &[KeyMethod, MethodSet] : M) {
    RelatedSymbols::SymbolSet &Methods = RelatedMethods[KeyMethod];
    for (auto &Method : MethodSet)
      Methods.add_symbols()->set_usr(Method.getKey());
  }
  return Result;
}

RelatedVirtualMethodsMap loadVirtualMethodsMap(const RelatedSymbols &R) {
  RelatedVirtualMethodsMap Related;
  for (const auto &[KeyMethod, Methods] : R.related_symbols()) {
    llvm::StringSet<> &RelatedMethods = Related[KeyMethod];
    for (auto &Symbol : Methods.symbols()) RelatedMethods.insert(Symbol.usr());
  }
  return Related;
}

VirtualMethodEvidenceFlowDirection getFlowDirection(Evidence::Kind Kind,
                                                    bool ForReturnSlot) {
  switch (Kind) {
    case Evidence::ANNOTATED_NONNULL:
    case Evidence::UNCHECKED_DEREFERENCE:
    case Evidence::NONNULL_ARGUMENT:
    case Evidence::NONNULL_RETURN:
    case Evidence::ASSIGNED_TO_NONNULL:
    case Evidence::ABORT_IF_NULL:
    case Evidence::ARITHMETIC:
    case Evidence::GCC_NONNULL_ATTRIBUTE:
    case Evidence::ASSIGNED_TO_NONNULL_REFERENCE:
    case Evidence::WELL_KNOWN_NONNULL:
    case Evidence::ARRAY_SUBSCRIPT:
    // Evidence pointing toward Unknown is only used to prevent Nonnull
    // inferences; it cannot override Nullable. So propagate it in the same
    // direction we do for Nonnull-pointing evidence.
    case Evidence::ANNOTATED_UNKNOWN:
    case Evidence::UNKNOWN_ARGUMENT:
    case Evidence::UNKNOWN_RETURN:
    case Evidence::ASSIGNED_FROM_NONNULL:
    case Evidence::ASSIGNED_FROM_UNKNOWN:
      return ForReturnSlot
                 ? VirtualMethodEvidenceFlowDirection::kFromBaseToDerived
                 : VirtualMethodEvidenceFlowDirection::kFromDerivedToBase;
    case Evidence::ANNOTATED_NULLABLE:
    case Evidence::NULLABLE_ARGUMENT:
    case Evidence::NULLABLE_RETURN:
    case Evidence::ASSIGNED_TO_MUTABLE_NULLABLE:
    case Evidence::ASSIGNED_FROM_NULLABLE:
    case Evidence::LEFT_NULLABLE_BY_CONSTRUCTOR:
    // Used to prevent a Nullable inference in combination with
    // LEFT_NULLABLE_BY_CONSTRUCTOR evidence, so propagate in the same direction
    // as the evidence with which it is combined.
    case Evidence::LEFT_NOT_NULLABLE_BY_LATE_INITIALIZER:
    case Evidence::NULLPTR_DEFAULT_MEMBER_INITIALIZER:
    case Evidence::WELL_KNOWN_NULLABLE:
      return ForReturnSlot
                 ? VirtualMethodEvidenceFlowDirection::kFromDerivedToBase
                 : VirtualMethodEvidenceFlowDirection::kFromBaseToDerived;
    case Evidence::NULLABLE_REFERENCE_RETURN:
    case Evidence::NONNULL_REFERENCE_RETURN:
    case Evidence::NONNULL_REFERENCE_RETURN_AS_CONST:
    case Evidence::UNKNOWN_REFERENCE_RETURN:
    case Evidence::NULLABLE_REFERENCE_ARGUMENT:
    case Evidence::NONNULL_REFERENCE_ARGUMENT:
    case Evidence::NONNULL_REFERENCE_ARGUMENT_AS_CONST:
    case Evidence::UNKNOWN_REFERENCE_ARGUMENT:
    case Evidence::ASSIGNED_TO_OR_FROM_INVARIANT_NONNULL:
    case Evidence::ASSIGNED_TO_OR_FROM_INVARIANT_NULLABLE:
      return VirtualMethodEvidenceFlowDirection::kBoth;
  }
}

static void appendUSRs(const llvm::StringSet<> &Strings,
                       std::vector<std::string_view> &USRs) {
  for (auto &Entry : Strings) USRs.push_back(Entry.getKey());
}

static std::vector<std::string_view> getAdditionalTargetsForVirtualMethod(
    std::string_view USR, Evidence::Kind Kind, bool ForReturnSlot,
    const VirtualMethodIndex &Index) {
  VirtualMethodEvidenceFlowDirection FlowDirection =
      getFlowDirection(Kind, ForReturnSlot);
  std::vector<std::string_view> Results;
  switch (FlowDirection) {
    case VirtualMethodEvidenceFlowDirection::kFromBaseToDerived:
      if (auto It = Index.Overrides.find(USR); It != Index.Overrides.end())
        appendUSRs(It->second.OverridingUSRs, Results);
      return Results;
    case VirtualMethodEvidenceFlowDirection::kFromDerivedToBase:
      if (auto It = Index.Bases.find(USR); It != Index.Bases.end())
        appendUSRs(It->second, Results);
      return Results;
    case VirtualMethodEvidenceFlowDirection::kBoth:
      // Simply concatenate the two sets -- given the acyclic nature of the AST,
      // they must be exclusive.
      if (auto It = Index.Bases.find(USR); It != Index.Bases.end())
        appendUSRs(It->second, Results);
      if (auto It = Index.Overrides.find(USR); It != Index.Overrides.end())
        appendUSRs(It->second.OverridingUSRs, Results);
      return Results;
  }
}

namespace {
class InferableSlot {
 public:
  InferableSlot(PointerTypeNullability Nullability, Slot Slot,
                std::string InferenceTargetUSR)
      : SymbolicNullability(Nullability),
        TargetSlot(Slot),
        InferenceTargetUSR(std::move(InferenceTargetUSR)) {}

  InferableSlot(PointerTypeNullability Nullability, Slot Slot,
                const Decl &Target, USRCache &USRCache)
      : InferableSlot(Nullability, Slot,
                      std::string(getOrGenerateUSR(USRCache, Target))) {
    CHECK(isInferenceTarget(Target))
        << "InferableSlot created for a Target which is not an inference "
           "target: "
        << (dyn_cast<NamedDecl>(&Target)
                ? dyn_cast<NamedDecl>(&Target)->getQualifiedNameAsString()
                : "not a named decl");
  }

  const PointerTypeNullability &getSymbolicNullability() const {
    return SymbolicNullability;
  }
  Slot getTargetSlot() const { return TargetSlot; }
  std::string_view getInferenceTargetUSR() const { return InferenceTargetUSR; }

 private:
  const PointerTypeNullability SymbolicNullability;
  const Slot TargetSlot;
  const std::string InferenceTargetUSR;
};
}  // namespace

/// If Stmt is a dereference, returns its target and location.
static std::pair<const Expr *, SourceLocation> describeDereference(
    const Stmt &Stmt) {
  if (auto *Op = dyn_cast<UnaryOperator>(&Stmt);
      Op && Op->getOpcode() == UO_Deref) {
    return {Op->getSubExpr(), Op->getOperatorLoc()};
  }
  if (auto *ME = dyn_cast<MemberExpr>(&Stmt); ME && ME->isArrow()) {
    return {ME->getBase(), ME->getOperatorLoc()};
  }
  // pointers to members; at the time of writing, they aren't a supported
  // pointer type, so this is a no-op.
  if (const auto *BO = dyn_cast<BinaryOperator>(&Stmt);
      BO && (BO->getOpcode() == clang::BinaryOperatorKind::BO_PtrMemD ||
             BO->getOpcode() == clang::BinaryOperatorKind::BO_PtrMemI)) {
    return {BO->getRHS(), BO->getOperatorLoc()};
  }
  if (const auto *OCE = dyn_cast<CXXOperatorCallExpr>(&Stmt);
      OCE && OCE->getOperator() == clang::OO_Star &&
      isSupportedSmartPointerType(OCE->getArg(0)->getType())) {
    return {OCE->getArg(0), OCE->getOperatorLoc()};
  }
  return {nullptr, SourceLocation()};
}

/// Inferable slots are nullability slots not explicitly annotated in source
/// code and which we are currently capable of handling. We represent their
/// nullability symbolically, and then constrain those symbols during inference.
/// Slots with no previous inference are constrained to Unknown, while those
/// with inferred annotations are constrained correspondingly. This function
/// accumulates the constraints on all inferable slots and expresses them as a
/// single formula.
static const Formula &getConstraintsOnInferableSlots(
    const std::vector<InferableSlot> &InferableSlots,
    const PreviousInferences &PreviousInferences, dataflow::Arena &A) {
  const Formula *Constraint = &A.makeLiteral(true);
  for (auto &IS : InferableSlots) {
    SlotFingerprint Fingerprint =
        fingerprint(IS.getInferenceTargetUSR(), IS.getTargetSlot());
    auto Nullability = IS.getSymbolicNullability();
    const Formula &Nullable = PreviousInferences.Nullable->contains(Fingerprint)
                                  ? Nullability.isNullable(A)
                                  : A.makeNot(Nullability.isNullable(A));
    const Formula &Nonnull = PreviousInferences.Nonnull->contains(Fingerprint)
                                 ? Nullability.isNonnull(A)
                                 : A.makeNot(Nullability.isNonnull(A));
    Constraint = &A.makeAnd(*Constraint, A.makeAnd(Nullable, Nonnull));
  }
  return *Constraint;
}

static void overrideNullability(const ValueDecl &D,
                                const PointerNullabilityLattice &Lattice,
                                TypeNullability &N) {
  if (N.empty()) {
    // We expect this not to be the case, but not to a crash-worthy level, so
    // just log if it is.
    llvm::errs() << "Nullability for type " << D.getType().getAsString();
    if (auto *ND = dyn_cast<clang::NamedDecl>(&D)) {
      llvm::errs() << " for Decl named " << ND->getQualifiedNameAsString();
    }
    llvm::errs() << " requested with overrides, but is an empty vector.\n";
  } else {
    Lattice.overrideNullabilityFromDecl(&D, N);
  }
}

static TypeNullability getNullabilityAnnotationsFromDeclAndOverrides(
    const ValueDecl &D, const PointerNullabilityLattice &Lattice) {
  TypeNullability N = getTypeNullability(D, Lattice.defaults());
  overrideNullability(D, Lattice, N);
  return N;
}

static TypeNullability getReturnTypeNullabilityAnnotations(
    const FunctionDecl &D, const TypeNullabilityDefaults &Defaults) {
  // Use the QualType, FileID overload of getTypeNullability for return types,
  // because of complexity around the following cases:
  //
  // The return TypeLoc for `auto`-returning functions contains an undeduced
  // `auto` type, even if the `auto` has been deduced. See
  // https://github.com/llvm/llvm-project/issues/42259 for more.
  //
  // FunctionDecls with top-level TypeLocs that are not simple
  // FunctionTypeLoc, such as those with attributes, would need excavation of
  // the function's FunctionTypeLoc before being able to retrieve the return
  // TypeLoc.
  return getTypeNullability(D.getReturnType(), getGoverningFile(&D), Defaults);
}

static TypeNullability getReturnTypeNullabilityAnnotationsWithOverrides(
    const FunctionDecl &D, const PointerNullabilityLattice &Lattice) {
  TypeNullability N =
      getReturnTypeNullabilityAnnotations(D, Lattice.defaults());

  // The FunctionDecl is the key used for overrides for the return
  // type. To look up overrides for parameters, we would pass a
  // ParmVarDecl to `overrideNullability`.
  overrideNullability(D, Lattice, N);
  return N;
}

// Type properties relevant to evidence generation.
struct EvidenceTypeProperties {
  // Whether the type is an lvalue reference.
  bool IsLValueRef;
  // Whether, after stripping away any potential outer reference type, the
  // remaining type is `const`-qualified.
  bool IsNonReferenceConst;
};

static EvidenceTypeProperties getEvidenceTypeProperties(QualType ParamType) {
  bool IsReference = ParamType->isLValueReferenceType();
  bool IsNonReferenceConst = ParamType.getNonReferenceType().isConstQualified();
  return {IsReference, IsNonReferenceConst};
}

static Evidence::Kind getArgEvidenceKindFromNullability(
    NullabilityKind Nullability, EvidenceTypeProperties TyProps) {
  switch (Nullability) {
    case NullabilityKind::Nullable:
      return TyProps.IsLValueRef ? Evidence::NULLABLE_REFERENCE_ARGUMENT
                                 : Evidence::NULLABLE_ARGUMENT;
    case NullabilityKind::NonNull: {
      return TyProps.IsLValueRef
                 ? (TyProps.IsNonReferenceConst
                        ? Evidence::NONNULL_REFERENCE_ARGUMENT_AS_CONST
                        : Evidence::NONNULL_REFERENCE_ARGUMENT)
                 : Evidence::NONNULL_ARGUMENT;
    }
    default:
      return TyProps.IsLValueRef ? Evidence::UNKNOWN_REFERENCE_ARGUMENT
                                 : Evidence::UNKNOWN_ARGUMENT;
  }
}

static Evidence::Kind getArgEvidenceKindFromNullability(
    NullabilityKind Nullability, QualType ParamType) {
  return getArgEvidenceKindFromNullability(
      Nullability, getEvidenceTypeProperties(ParamType));
}

static std::optional<Evidence::Kind> evidenceKindFromDeclaredNullability(
    const TypeNullability &Nullability) {
  switch (Nullability.front().concrete()) {
    default:
      return std::nullopt;
    case NullabilityKind::NonNull:
      return Evidence::ANNOTATED_NONNULL;
    case NullabilityKind::Nullable:
      return Evidence::ANNOTATED_NULLABLE;
  }
}

static std::optional<Evidence::Kind> evidenceKindFromDeclaredTypeLoc(
    TypeLoc Loc, const TypeNullabilityDefaults &Defaults) {
  if (!hasInferable(Loc.getType())) return std::nullopt;
  auto Nullability = getTypeNullability(Loc, Defaults);
  return evidenceKindFromDeclaredNullability(Nullability);
}

static std::optional<Evidence::Kind> evidenceKindFromDeclaredReturnType(
    const FunctionDecl &D, const TypeNullabilityDefaults &Defaults) {
  if (!hasInferable(D.getReturnType())) return std::nullopt;
  return evidenceKindFromDeclaredNullability(
      getReturnTypeNullabilityAnnotations(D, Defaults));
}

static bool isOrIsConstructedFromNullPointerConstant(const Expr *absl_nonnull E,
                                                     ASTContext &Ctx) {
  if (E->isNullPointerConstant(Ctx, Expr::NPC_ValueDependentIsNotNull) !=
      Expr::NPCK_NotNull) {
    return true;
  }
  if (auto *DefaultInit = dyn_cast<CXXDefaultInitExpr>(E)) {
    E = DefaultInit->getExpr();
  }
  const Expr *SubExpr = &dataflow::ignoreCFGOmittedNodes(*E);
  if (auto *MaterializeTempExpr = dyn_cast<MaterializeTemporaryExpr>(SubExpr)) {
    SubExpr = MaterializeTempExpr->getSubExpr();
  }
  if (auto *BindTemp = dyn_cast<CXXBindTemporaryExpr>(SubExpr)) {
    SubExpr = BindTemp->getSubExpr();
  }
  auto *CE = dyn_cast<CXXConstructExpr>(SubExpr->IgnoreImpCasts());
  if (!CE) return false;
  return CE != nullptr && CE->getNumArgs() == 1 &&
         CE->getArg(0)->isNullPointerConstant(
             Ctx, Expr::NPC_ValueDependentIsNotNull) != Expr::NPCK_NotNull;
}

llvm::unique_function<EvidenceEmitter> evidenceEmitterWithPropagation(
    llvm::unique_function<EvidenceEmitter> Emit, VirtualMethodIndex Index) {
  return [Emit = std::move(Emit),
          Index = std::move(Index)](Evidence E) mutable {
    Emit(E);
    // Virtual methods and their overrides constrain each other's nullabilities,
    // so propagate evidence in the appropriate direction based on the evidence
    // kind and whether the evidence is for the return type or a parameter type.
    std::vector<std::string_view> Targets =
        getAdditionalTargetsForVirtualMethod(
            E.symbol().usr(), E.kind(), E.slot() == SLOT_RETURN_TYPE, Index);
    *E.mutable_propagated_from() = E.symbol();
    for (std::string_view USR : Targets) {
      E.mutable_symbol()->set_usr(USR);
      Emit(E);
    }
  };
}

llvm::unique_function<EvidenceEmitter> evidenceEmitterWithPropagation(
    llvm::unique_function<EvidenceEmitter> Emit, USRCache &USRCache,
    ASTContext &Ctx) {
  return evidenceEmitterWithPropagation(std::move(Emit),
                                        getVirtualMethodIndex(Ctx, USRCache));
}

static Evidence makeEvidence(std::string_view USR, Slot S, Evidence::Kind Kind,
                             std::string_view LocAsString) {
  Evidence E;
  E.set_slot(S);
  E.set_kind(Kind);
  if (!LocAsString.empty()) E.set_location(LocAsString);
  E.mutable_symbol()->set_usr(USR);
  return E;
}

namespace {
// Serialized version of source locations. Currently, we're representing the
// serialized data as a string, but we introduce a fresh type in the spirit of
// "strong types."
struct SerializedSrcLoc {
  std::string Loc;
};
}  // namespace

static SerializedSrcLoc serializeLoc(const SourceManager &SM,
                                     SourceLocation Loc) {
  if (Loc = SM.getFileLoc(Loc); Loc.isValid())
    return SerializedSrcLoc{Loc.printToString(SM)};
  return SerializedSrcLoc{};
}

namespace {
// Collects nullability evidence from data extracted from the AST.
class EvidenceCollector {
 public:
  EvidenceCollector(const std::vector<InferableSlot> &InferableSlots,
                    const Formula &InferableSlotsConstraint,
                    const Environment &Env, const dataflow::Solver &Solver,
                    llvm::function_ref<EvidenceEmitter> Emit)
      : Env(Env),
        InferableSlots(InferableSlots),
        InferableSlotsConstraint(InferableSlotsConstraint),
        Emit(Emit),
        Solver(Solver) {}

  void emit(std::string_view USR, Slot S, Evidence::Kind Kind,
            const SerializedSrcLoc &Loc) {
    if (!USR.empty()) Emit(makeEvidence(USR, S, Kind, Loc.Loc));
  }

  /// Collects evidence for Nonnull-ness of one slot, derived from the necessity
  /// that `MustBeTrue` must be true.
  ///
  /// Used when we have reason to believe that `MustBeTrue` can be made true by
  /// marking a slot Nonnull.
  void mustBeTrueByMarkingNonnull(const Formula &MustBeTrue,
                                  const SerializedSrcLoc &Loc,
                                  Evidence::Kind EvidenceKind) {
    auto &A = Env.arena();
    // If `MustBeTrue` is already proven true or false (or both, which indicates
    // unsatisfiable flow conditions), collect no evidence.
    if (Env.proves(MustBeTrue) || Env.proves(A.makeNot(MustBeTrue))) return;

    for (auto &IS : InferableSlots) {
      auto &SlotNonnull = IS.getSymbolicNullability().isNonnull(A);
      auto &SlotNonnullImpliesFormulaTrue =
          A.makeImplies(SlotNonnull, MustBeTrue);
      // Don't collect evidence if the implication is true by virtue of
      // `SlotNonnull` being false.
      //
      // In practice, `SlotNonnull` can be made false by a flow condition, and
      // marking the slot Nonnull would make that conditioned block dead code.
      // Technically, this does make a dereference, etc. "safe", but we'd prefer
      // to mark a different slot Nonnull that has a more direct relationship
      // with `MustBeTrue`.
      //
      // e.g. We'd prefer to mark `q` Nonnull rather than `p` in the following:
      // ```
      // void target(int* p, int* q) {
      //   if (!p) {
      //     *q;
      //   }
      // }
      // ```
      if (Env.allows(SlotNonnull) &&
          Env.proves(SlotNonnullImpliesFormulaTrue)) {
        emit(IS.getInferenceTargetUSR(), IS.getTargetSlot(), EvidenceKind, Loc);
        return;
      }
    }
  }

  /// Collects evidence for Nullable-ness for potentially multiple slots,
  /// derived from the necessity that `MustBeTrue` must be true.
  ///
  /// Used when we have reason to believe that `MustBeTrue` can be made provably
  /// true by marking a single slot Nullable, and that all such slots should be
  /// marked Nullable.
  void mustBeTrueByMarkingNullable(const Formula &MustBeTrue,
                                   const SerializedSrcLoc &Loc,
                                   Evidence::Kind EvidenceKind) {
    auto &A = Env.arena();
    // If `MustBeTrue` is already proven true or false (or both, which indicates
    // unsatisfiable flow conditions), collect no evidence.
    if (Env.proves(MustBeTrue) || Env.proves(A.makeNot(MustBeTrue))) return;

    for (auto &IS : InferableSlots) {
      auto &SlotNullable = IS.getSymbolicNullability().isNullable(A);
      auto &SlotNullableImpliesFormulaTrue =
          A.makeImplies(SlotNullable, MustBeTrue);
      // Don't collect evidence if the implication is true by virtue of
      // `SlotNullable` being false.
      if (Env.allows(SlotNullable) &&
          Env.proves(SlotNullableImpliesFormulaTrue)) {
        emit(IS.getInferenceTargetUSR(), IS.getTargetSlot(), EvidenceKind, Loc);
        // Continue the loop, emitting evidence for all such slots.
      }
    }
  }

  /// For a variety of assignment-like operations, where the assignee is a
  /// declaration (e.g. field, variable, parameter), collects evidence regarding
  /// the type of the assignment's right-hand side (RHS) and any inferable-slots
  /// which may influence the value's nullability. For example, if the assignee
  /// is Nonnull, collects evidence that the RHS is nonnull as well.
  ///
  /// The parameters define the properties of the assignment:
  ///
  /// * `IsLHSTypeConst` indicates whether the LHS declarations's type is const,
  ///   after stripping away any potential outer reference type.
  ///
  /// * `LHSTopLevel` is the outermost nullability property of the declaration's
  ///   nullability vector.
  ///
  /// * `RHSTypeNullability` holds the (top-level) nullability of the RHS's type
  ///    *if and only if* the LHS declaration is a reference to a pointer. In
  ///    this (unusual) case, the assignment places stricter constraints on the
  ///    RHS's nullability, so we can collect additional evidence.
  ///
  /// * `RHSValueNullability` is the value nullability of the RHS expression.
  ///
  /// * `RHSLoc` is the beginning source location of the RHS expression.
  ///
  /// The (unused) fingerprint argument is only included in the signature
  /// because this method implements the abstract interface required by
  /// `NullabilityBehaviorVisitor`.
  void collectAssignmentToType(
      bool IsLHSTypeConst, const PointerTypeNullability &LHSTopLevel,
      std::optional<SlotFingerprint> /* UNUSED */,
      const std::optional<PointerTypeNullability> &RHSTypeNullability,
      const PointerNullState &RHSValueNullability,
      const SerializedSrcLoc &RHSLoc) {
    dataflow::Arena &A = Env.arena();
    if (RHSTypeNullability) {
      // The RHS type nullability is only provided when the LHS is a reference.
      // If the reference is either to a (mutable or const) Nonnull pointer or
      // to a mutable Nullable pointer, emit evidence that makes the top level
      // type nullability of `RHSExpr` match the top level of
      // `LHSTypeNullability`. This is distinct from the value nullability of
      // `RHSExpr` being constrained, which is handled below.
      if (LHSTopLevel.concrete() == NullabilityKind::NonNull ||
          (LHSTopLevel.isSymbolic() &&
           Env.proves(A.makeImplies(InferableSlotsConstraint,
                                    LHSTopLevel.isNonnull(A))))) {
        mustBeTrueByMarkingNonnull(RHSTypeNullability->isNonnull(A), RHSLoc,
                                   Evidence::ASSIGNED_TO_NONNULL_REFERENCE);
        // It would be duplicative to emit both ASSIGNED_TO_NONNULL_REFERENCE
        // and ASSIGNED_TO_NONNULL for the same assignment.
        return;
      }
      if (!IsLHSTypeConst &&
          (LHSTopLevel.concrete() == NullabilityKind::Nullable ||
           (LHSTopLevel.isSymbolic() &&
            Env.proves(A.makeImplies(InferableSlotsConstraint,
                                     LHSTopLevel.isNullable(A)))))) {
        mustBeTrueByMarkingNullable(RHSTypeNullability->isNullable(A), RHSLoc,
                                    Evidence::ASSIGNED_TO_MUTABLE_NULLABLE);
        // The LHS can't be Nullable and also Nonnull, so we can skip the
        // later checks for it being Nonnull.
        return;
      }
    }

    // If the left hand side is Nonnull, emit evidence that the PointerValue on
    // the right hand side must also be Nonnull, unless
    // `RHSValueNullability.IsNull` is top, in which case we can't infer
    // anything about the RHS.
    if (RHSValueNullability.IsNull != nullptr &&
        (LHSTopLevel.concrete() == NullabilityKind::NonNull ||
         (LHSTopLevel.isSymbolic() &&
          Env.proves(A.makeImplies(InferableSlotsConstraint,
                                   LHSTopLevel.isNonnull(A)))))) {
      const Formula &RHSNotIsNull =
          Env.arena().makeNot(*RHSValueNullability.IsNull);
      mustBeTrueByMarkingNonnull(RHSNotIsNull, RHSLoc,
                                 Evidence::ASSIGNED_TO_NONNULL);
    }
  }

  /// For an unannotated decl's nullability slot identified by `TargetUSR` and
  /// `TargetSlot`, collect evidence to indicate that the nullability slot must
  /// match the type nullability `NullabilityToMatch`.
  void collectMustHaveInvariantTypeNullability(
      std::string_view TargetUSR, Slot TargetSlot,
      const PointerTypeNullability& NullabilityToMatch,
      const SerializedSrcLoc& CollectionLoc) {
    dataflow::Arena& A = Env.arena();
    if (NullabilityToMatch.concrete() == NullabilityKind::NonNull ||
        (NullabilityToMatch.isSymbolic() &&
         Env.proves(A.makeImplies(InferableSlotsConstraint,
                                  NullabilityToMatch.isNonnull(A))))) {
      emit(TargetUSR, TargetSlot,
           Evidence::ASSIGNED_TO_OR_FROM_INVARIANT_NONNULL, CollectionLoc);
    } else if (NullabilityToMatch.concrete() == NullabilityKind::Nullable ||
               (NullabilityToMatch.isSymbolic() &&
                Env.proves(A.makeImplies(InferableSlotsConstraint,
                                         NullabilityToMatch.isNullable(A))))) {
      emit(TargetUSR, TargetSlot,
           Evidence::ASSIGNED_TO_OR_FROM_INVARIANT_NULLABLE, CollectionLoc);
    }
  }

  /// Collect evidence that will lead to `TargetNullability` matching
  /// `NullabilityToMatch`.
  void collectMustHaveInvariantTypeNullability(
      const PointerTypeNullability& TargetNullability,
      const PointerTypeNullability& NullabilityToMatch,
      std::optional<SlotFingerprint> /*UNUSED*/,
      const SerializedSrcLoc& CollectionLoc) {
    dataflow::Arena& A = Env.arena();
    if (NullabilityToMatch.concrete() == NullabilityKind::NonNull ||
        (NullabilityToMatch.isSymbolic() &&
         Env.proves(A.makeImplies(InferableSlotsConstraint,
                                  NullabilityToMatch.isNonnull(A))))) {
      mustBeTrueByMarkingNonnull(
          TargetNullability.isNonnull(A), CollectionLoc,
          Evidence::ASSIGNED_TO_OR_FROM_INVARIANT_NONNULL);
    } else if (NullabilityToMatch.concrete() == NullabilityKind::Nullable ||
               (NullabilityToMatch.isSymbolic() &&
                Env.proves(A.makeImplies(InferableSlotsConstraint,
                                         NullabilityToMatch.isNullable(A))))) {
      mustBeTrueByMarkingNullable(
          TargetNullability.isNullable(A), CollectionLoc,
          Evidence::ASSIGNED_TO_OR_FROM_INVARIANT_NULLABLE);
    }
  }

  /// Collects evidence for parameter nullability based on arguments passed at
  /// call sites. Considers two distinct cases, based on the setting of
  /// `ArgNullState` -- when populated, the argument is a pointer value; when
  /// nullopt, the argument is a nullptr literal.
  void collectArgBinding(std::string_view FunctionUSR, Slot ParamSlot,
                         EvidenceTypeProperties ParamTyProps,
                         std::optional<PointerNullState> ArgNullState,
                         const SerializedSrcLoc &ArgLoc) {
    // Calculate the parameter's nullability, using InferableSlotsConstraint to
    // reflect the current knowledge of the annotations from previous inference
    // rounds, and not all possible annotations for them.
    NullabilityKind ArgNullability =
        ArgNullState
            ? getNullability(*ArgNullState, Env, &InferableSlotsConstraint)
            : getNullabilityForNullptrT(Env, &InferableSlotsConstraint);
    emit(FunctionUSR, ParamSlot,
         getArgEvidenceKindFromNullability(ArgNullability, ParamTyProps),
         ArgLoc);
  }

  /// Collects evidence from an operation that requires two pointer operands to
  /// differ in value. Specifically, considers cases where one operand is known
  /// statically to be null, in which case we have evidence that the other is
  /// Nonnull. The canonical example is `CHECK_NE(p, q)`, but other operations
  /// could conceivably provide the same potential evidence.
  ///
  /// This function is not intended for use when either operand is a nullptr
  /// literal, since that can be handled more efficiently by avoiding the calls
  /// to `proves()`.
  void collectAbortIfEqual(const dataflow::Formula &FirstIsNull,
                           const SerializedSrcLoc &FirstLoc,
                           const dataflow::Formula &SecondIsNull,
                           const SerializedSrcLoc &SecondLoc) {
    auto &A = Env.arena();
    if (Env.proves(FirstIsNull)) {
      mustBeTrueByMarkingNonnull(A.makeNot(SecondIsNull), SecondLoc,
                                 Evidence::ABORT_IF_NULL);
    } else if (Env.proves(SecondIsNull)) {
      mustBeTrueByMarkingNonnull(A.makeNot(FirstIsNull), FirstLoc,
                                 Evidence::ABORT_IF_NULL);
    }
  }

  /// Collects evidence for return-type Nullability.
  void collectReturn(std::string_view FunctionUSR,
                     EvidenceTypeProperties ReturnTyProps,
                     PointerNullState ReturnNullState,
                     const SerializedSrcLoc &ReturnLoc) {
    NullabilityKind ReturnNullability =
        getNullability(ReturnNullState, Env, &InferableSlotsConstraint);

    Evidence::Kind ReturnEvidenceKind;
    if (ReturnTyProps.IsLValueRef) {
      switch (ReturnNullability) {
        case NullabilityKind::Nullable:
          ReturnEvidenceKind = Evidence::NULLABLE_REFERENCE_RETURN;
          break;
        case NullabilityKind::NonNull:
          ReturnEvidenceKind = ReturnTyProps.IsNonReferenceConst
                                   ? Evidence::NONNULL_REFERENCE_RETURN_AS_CONST
                                   : Evidence::NONNULL_REFERENCE_RETURN;
          break;
        default:
          ReturnEvidenceKind = Evidence::UNKNOWN_REFERENCE_RETURN;
      }
    } else {
      switch (ReturnNullability) {
        case NullabilityKind::Nullable:
          ReturnEvidenceKind = Evidence::NULLABLE_RETURN;
          break;
        case NullabilityKind::NonNull:
          ReturnEvidenceKind = Evidence::NONNULL_RETURN;
          break;
        default:
          ReturnEvidenceKind = Evidence::UNKNOWN_RETURN;
      }
    }
    emit(FunctionUSR, SLOT_RETURN_TYPE, ReturnEvidenceKind, ReturnLoc);
  }

  /// Collects evidence from assignments, specifically about the nullability of
  /// an assignee based on the nullability of the RHS value.
  ///
  /// `ValueNullState` holds the null state of  the RHS expression. It should be
  ///  nullopt *if and only if* the RHS expression has `std::nullptr_t` type.
  ///
  /// Example:
  /// ```
  /// void target(int* p, int* q, NullabilityUnknown<int*> r) {
  ///   p = nullptr;
  ///   if (!r) {
  ///     q = r;
  ///   }
  ///   int i = 0;
  ///   int* s = &i;
  /// }
  /// ```
  /// From the above, we collect evidence from each of the assignments of `p`
  /// and `q` that they were ASSIGNED_FROM_NULLABLE and evidence from the
  /// assignment of `s` that it was ASSIGNED_FROM_NONNULL.
  void collectAssignmentFromValue(
      PointerTypeNullability TypeNullability,
      std::optional<PointerNullState> ValueNullState,
      const SerializedSrcLoc &ValueLoc,
      Evidence::Kind EvidenceKindForAssignmentFromNullable) {
    dataflow::Arena &A = Env.arena();
    const Formula &TypeIsNullable = TypeNullability.isNullable(A);

    // If the flow conditions already imply that the type is nullable, or
    // that the type is not nullable, we can skip collecting evidence.
    if (Env.proves(TypeIsNullable) || !Env.allows(TypeIsNullable)) return;

    clang::NullabilityKind ValNullability =
        ValueNullState
            ? getNullability(*ValueNullState, Env, &InferableSlotsConstraint)
            : getNullabilityForNullptrT(Env, &InferableSlotsConstraint);

    for (auto &IS : InferableSlots) {
      auto &Implication = A.makeImplies(
          IS.getSymbolicNullability().isNullable(A), TypeIsNullable);
      // It's not expected that a slot's isNullable formula could be proven
      // false by the environment alone (without the
      // InferableSlotsConstraint), but SAT calls are relatively expensive, so
      // only DCHECK. This has so far only been observed in the case of the SAT
      // solver reaching its iteration limit before or during this check, in
      // which case we won't be able to collect accurate evidence anyway, so we
      // simply return early.
      DCHECK(Env.allows(IS.getSymbolicNullability().isNullable(A)));
      if (Solver.reachedLimit()) return;
      if (Env.proves(Implication)) {
        Evidence::Kind EvidenceKind;
        switch (ValNullability) {
          case NullabilityKind::Nullable:
            EvidenceKind = EvidenceKindForAssignmentFromNullable;
            break;
          case NullabilityKind::NonNull:
            EvidenceKind = Evidence::ASSIGNED_FROM_NONNULL;
            break;
          default:
            EvidenceKind = Evidence::ASSIGNED_FROM_UNKNOWN;
        }
        emit(IS.getInferenceTargetUSR(), IS.getTargetSlot(), EvidenceKind,
             ValueLoc);
        return;
      }
    }
  }

  void collectConstructorExitBlock(std::string_view USR,
                                   PointerNullState NullState,
                                   const SerializedSrcLoc &Loc) {
    if (isNullable(NullState, Env, &InferableSlotsConstraint)) {
      emit(USR, Slot(0), Evidence::LEFT_NULLABLE_BY_CONSTRUCTOR, Loc);
    }
  }

  void collectSupportedLateInitializerExitBlock(std::string_view USR,
                                                PointerNullState NullState,
                                                const SerializedSrcLoc &Loc) {
    if (!isNullable(NullState, Env, &InferableSlotsConstraint)) {
      emit(USR, Slot(0), Evidence::LEFT_NOT_NULLABLE_BY_LATE_INITIALIZER, Loc);
    }
  }

 private:
  const Environment &Env;
  const std::vector<InferableSlot> &InferableSlots;
  const Formula &InferableSlotsConstraint;
  llvm::function_ref<EvidenceEmitter> Emit;
  const dataflow::Solver &Solver;
};

template <typename BehaviorConsumer>
class NullabilityBehaviorVisitor {
 public:
  // `Consumer` is an in/out parameter, because it may accumulate state that the
  // caller later needs to access.
  //
  // Instantiate the class only in this function, to restrict the lifetime of
  // the object, which holds reference parameters.
  static void visit(const std::vector<InferableSlot>& InferableSlots,
                    USRCache& USRCache,
                    const PointerNullabilityLattice& Lattice,
                    const Environment& Env, const SourceManager& SM,
                    const CFGElement& CFGElem,
                    LocFilter* absl_nullable NotTestMainFileLocFilter,
                    BehaviorConsumer& BC) {
    NullabilityBehaviorVisitor<BehaviorConsumer> Visitor(
        InferableSlots, USRCache, Lattice, Env, SM, NotTestMainFileLocFilter,
        BC);
    Visitor.visit(CFGElem);
  }

 private:
  NullabilityBehaviorVisitor(const std::vector<InferableSlot>& InferableSlots,
                             USRCache& USRCache,
                             const PointerNullabilityLattice& Lattice,
                             const Environment& Env, const SourceManager& SM,
                             LocFilter* absl_nullable NotTestMainFileLocFilter,
                             BehaviorConsumer& BC)
      : Consumer(BC),
        Env(Env),
        HasInferableSlots(!InferableSlots.empty()),
        USRCache(USRCache),
        Lattice(Lattice),
        SM(SM),
        NotTestMainFileLocFilter(NotTestMainFileLocFilter) {}

  void visit(const CFGElement &CFGElem) {
    if (auto CFGStmt = CFGElem.getAs<clang::CFGStmt>()) {
      const Stmt *S = CFGStmt->getStmt();
      if (!S) return;
      visitDereference(*S);
      visitCallExpr(*S);
      visitConstructExpr(*S);
      visitReturn(*S);
      visitAssignment(*S);
      visitArithmetic(*S);
      visitAggregateInitialization(*S);
      visitArraySubscript(*S);
    } else if (auto CFGInit = CFGElem.getAs<clang::CFGInitializer>()) {
      visitCFGInitializer(*CFGInit);
    }
  }

  /// Captures the necessity that `NullState` is nonnull.  It may be because the
  /// associated value was dereferenced, passed as a nonnull param, etc, per
  /// `EvidenceKind`.
  void mustBeNonnull(const PointerNullState &NullState,
                     const SerializedSrcLoc &Loc, Evidence::Kind EvidenceKind) {
    auto *IsNull = NullState.IsNull;
    // If `IsNull` is top, we can't infer anything about it.
    if (IsNull == nullptr) return;
    auto &A = Env.arena();
    const Formula &F = A.makeNot(*IsNull);
    Consumer.mustBeTrueByMarkingNonnull(F, Loc, EvidenceKind);
  }

  PointerNullState getPointerNullStateOrDie(
      const dataflow::PointerValue &Value) {
    CHECK(hasPointerNullState(Value))
        << "Value should be the value of an expression. Cannot collect "
           "evidence for nullability if there is no null state.";
    return getPointerNullState(Value);
  }

  void visitDereference(const Stmt &S) {
    auto [Target, Loc] = describeDereference(S);
    if (!Target || !isSupportedPointerType(Target->getType())) return;

    // It is a dereference of a pointer. Now gather evidence from it.
    dataflow::PointerValue *DereferencedValue = getPointerValue(Target, Env);
    if (!DereferencedValue) return;
    mustBeNonnull(getPointerNullStateOrDie(*DereferencedValue),
                  serializeLoc(SM, Loc), Evidence::UNCHECKED_DEREFERENCE);
  }

  // Analyzes an assignment to a pointer-typed declaration for nullability
  // constraints on the RHS (potentially both its type and value). These
  // constraints stem from the declaration's Nullability. `LHSType` should be a
  // pointer type or a reference to a pointer type. `LHSDecl` identifies the
  // ValueDecl of the LHS whose type annotation may be overridden by future
  // rounds of inference. When the ValueDecl doesn't take part in inference,
  // nullptr should be passed.
  void visitAssignmentToType(QualType LHSType,
                             const TypeNullability &LHSTypeNullability,
                             const ValueDecl *absl_nullable LHSDecl,
                             const Expr &RHSExpr,
                             const SerializedSrcLoc &RHSLoc) {
    // TODO: Account for variance and each layer of nullability when we handle
    // more than top-level pointers.
    if (LHSTypeNullability.empty()) return;
    const PointerTypeNullability &LHSTopLevel = LHSTypeNullability[0];
    const dataflow::PointerValue *PointerValue = getPointerValue(&RHSExpr, Env);
    if (!PointerValue) return;
    PointerNullState RHSNullState = getPointerNullStateOrDie(*PointerValue);
    EvidenceTypeProperties LHSTyProps = getEvidenceTypeProperties(LHSType);

    std::optional<SlotFingerprint> LHSFingerprint;
    if (LHSDecl != nullptr) {
      Slot S = SLOT_RETURN_TYPE;
      if (auto *PD = dyn_cast<ParmVarDecl>(LHSDecl)) {
        if (auto *Parent = dyn_cast_or_null<FunctionDecl>(
                PD->getParentFunctionOrMethod())) {
          LHSDecl = Parent;
          S = paramSlot(PD->getFunctionScopeIndex());
        }
      }
      LHSFingerprint = fingerprint(getOrGenerateUSR(USRCache, *LHSDecl), S);
    }

    std::optional<PointerTypeNullability> RHSTopLevel;
    if (LHSTyProps.IsLValueRef) {
      const TypeNullability *RHSTypeNullability =
          Lattice.getTypeNullability(&RHSExpr);
      if (RHSTypeNullability != nullptr) {
        CHECK_GT(RHSTypeNullability->size(), 0);
        RHSTopLevel = (*RHSTypeNullability)[0];
      }
    }

    Consumer.collectAssignmentToType(LHSTyProps.IsNonReferenceConst,
                                     LHSTopLevel, LHSFingerprint, RHSTopLevel,
                                     RHSNullState, RHSLoc);
  }

  // Visits the binding of `Arg` to `Param` when their types are inferable but
  // not directly pointers, e.g. a select template with a pointer-type template
  // argument. These bindings conservatively require invariant nullability for
  // the entirety of the two types.
  void visitArgAndParamWithInferableNonPointerTypes(
      const ParmVarDecl& Param, std::string_view ParamSlotUSR, Slot ParamSlot,
      bool CalleeIsInferenceTarget, const Expr& Arg,
      SerializedSrcLoc CollectionLoc) {
    // TODO: Account for each layer of nullability when we handle more than
    // top-level pointers as template arguments.

    const TypeNullability* ArgTypeNullability =
        Lattice.getTypeNullability(&Arg);
    if (ArgTypeNullability == nullptr) return;
    CHECK(!ArgTypeNullability->empty());
    const PointerTypeNullability& ArgTopLevelNullability =
        (*ArgTypeNullability)[0];

    const TypeNullability ParamTypeNullabilityInSource =
        getTypeNullability(Param, Lattice.defaults());
    CHECK(!ParamTypeNullabilityInSource.empty());

    // Collect information about the nullability requirement for the
    // parameter, if we are trying to infer nullability for the parameter and it
    // is not already annotated in source.
    if (CalleeIsInferenceTarget &&
        !evidenceKindFromDeclaredNullability(ParamTypeNullabilityInSource)) {
      Consumer.collectMustHaveInvariantTypeNullability(
          ParamSlotUSR, ParamSlot, ArgTopLevelNullability, CollectionLoc);
    }
    // Collect information about the nullability requirement for the argument.
    // The argument's nullability must be symbolic and influenced by at least
    // one inferable slot in order for this information to be useful. We don't
    // try to determine which slots influence the argument's nullability, since
    // that's done with the InferableSlotsConstraint, which can change over the
    // course of multiple iterations, but we at least check that we have 1+
    // inferable slots.
    if (HasInferableSlots && ArgTopLevelNullability.isSymbolic()) {
      const TypeNullability ParamTypeNullabilityWithOverrides =
          getNullabilityAnnotationsFromDeclAndOverrides(Param, Lattice);
      CHECK(!ParamTypeNullabilityWithOverrides.empty());

      Consumer.collectMustHaveInvariantTypeNullability(
          ArgTopLevelNullability, ParamTypeNullabilityWithOverrides[0],
          fingerprint(ParamSlotUSR, ParamSlot), CollectionLoc);
    }
  }

  template <typename CallOrConstructExpr>
  void visitArgsAndParams(const FunctionDecl &CalleeDecl,
                          const CallOrConstructExpr &Expr,
                          bool MayBeMissingImplicitConversion) {
    bool CalleeIsInferenceTarget = isInferenceTarget(CalleeDecl);
    bool AnalyzeCallee = CalleeIsInferenceTarget;
    bool AnalyzeCaller = HasInferableSlots;

    // Don't consider nullability behaviors where the caller is in a test main
    // file, but the callee is not, to avoid collecting such evidence (test
    // evidence may not be as accurate as non-test evidence). If the caller and
    // callee are both in test main files, then that will be the only evidence,
    // so we consider that behavior.
    if (AnalyzeCallee && NotTestMainFileLocFilter != nullptr) {
      if (NotTestMainFileLocFilter->check(
              CalleeDecl.getCanonicalDecl()->getBeginLoc()) &&
          !NotTestMainFileLocFilter->check(Expr.getBeginLoc())) {
        AnalyzeCallee = false;
      }
    }

    for (ParamAndArgIterator<CallOrConstructExpr> Iter(CalleeDecl, Expr); Iter;
         ++Iter) {
      if (isa<clang::CXXDefaultArgExpr>(Iter.arg())) {
        // Collection for the callee from default argument values is handled
        // when processing declarations, and there's nothing useful available to
        // collect for the caller. Return, not just continue, because once we've
        // seen one default argument used, all remaining parameters also use
        // default arguments.
        return;
      }

      if (!hasInferable(Iter.param().getType())) continue;
      bool ArgIsNullPtrT = Iter.arg().getType()->isNullPtrType();
      bool ParamHasSupportedPointerType =
          isSupportedPointerType(Iter.param().getType().getNonReferenceType());
      // We check only the canonical type because if the call is in a template,
      // we don't care about checking the argument for substituted template
      // arguments.
      bool CanonicalArgTypeIsInferable =
          hasInferable(Iter.arg().getType().getCanonicalType());
      if (!CanonicalArgTypeIsInferable ||
          (ParamHasSupportedPointerType &&
           !isSupportedPointerType(Iter.arg().getType()))) {
        // These builtins are declared with pointer type parameters even when
        // given a valid argument of type uintptr_t. In this case, there's
        // nothing to infer, but also nothing unexpected to crash over.
        auto BuiltinID = CalleeDecl.getBuiltinID();
        if (BuiltinID == Builtin::BI__builtin_is_aligned ||
            BuiltinID == Builtin::BI__builtin_align_up ||
            BuiltinID == Builtin::BI__builtin_align_down) {
          continue;
        }
        // In the case of forwarding functions, implicit conversions may be
        // split between the call site `Expr` and the forwarding function body
        // (e.g., within `std::make_unique`). We don't piece the two together
        // here. Instead of crashing, we just skip the argument.
        // `nullptr_t` is handled since we have an answer for its nullability.
        if (MayBeMissingImplicitConversion && !ArgIsNullPtrT) continue;
      }
      // Otherwise, the corresponding argument's canonical type should also be
      // inferable.
      CHECK(CanonicalArgTypeIsInferable ||
            (MayBeMissingImplicitConversion && ArgIsNullPtrT))
          << "Unsupported argument " << Iter.argIdx()
          << " type: " << Iter.arg().getType().getAsString();

      SerializedSrcLoc ArgLoc = serializeLoc(SM, Iter.arg().getExprLoc());
      if (!ParamHasSupportedPointerType) {
        // Inferable types which are not directly a pointer are conservatively
        // required to have invariant nullability, so we visit this behavior
        // differently for non-pointers.
        visitArgAndParamWithInferableNonPointerTypes(
            Iter.param(), getOrGenerateUSR(USRCache, CalleeDecl),
            paramSlot(Iter.paramIdx()), CalleeIsInferenceTarget, Iter.arg(),
            ArgLoc);
        return;
      }

      // Outside of the special cases skipped above, if the parameter has a
      // supported pointer type, the corresponding argument should also be a
      // supported pointer or of type `nullptr_t`.
      CHECK(isSupportedPointerType(Iter.arg().getType()) ||
            (MayBeMissingImplicitConversion && ArgIsNullPtrT))
          << "Unsupported argument " << Iter.argIdx()
          << " type: " << Iter.arg().getType().getAsString();

      if (AnalyzeCaller) {
        auto ParamNullability = getNullabilityAnnotationsFromDeclAndOverrides(
            Iter.param(), Lattice);

        // Analyzes for potential constraints that the parameter's nullability
        // places on the argument's nullability.
        visitAssignmentToType(Iter.param().getType(), ParamNullability,
                              &Iter.param(), Iter.arg(), ArgLoc);
      }

      if (AnalyzeCallee &&
          // Don't analyze if the parameter is already annotated in
          // source. Otherwise, analyze, even if the parameter has a
          // previously-inferred nullability, to maintain that inference in this
          // iteration.
          !evidenceKindFromDeclaredNullability(
              getTypeNullability(Iter.param(), Lattice.defaults()))) {
        dataflow::PointerValue *PV = getPointerValue(&Iter.arg(), Env);
        if (PV != nullptr) {
          Consumer.collectArgBinding(
              getOrGenerateUSR(USRCache, CalleeDecl),
              paramSlot(Iter.paramIdx()),
              getEvidenceTypeProperties(Iter.param().getType()),
              getPointerNullState(*PV), ArgLoc);
        } else if (ArgIsNullPtrT) {
          Consumer.collectArgBinding(
              getOrGenerateUSR(USRCache, CalleeDecl),
              paramSlot(Iter.paramIdx()),
              getEvidenceTypeProperties(Iter.param().getType()),
              /*ArgNullState*/ std::nullopt, ArgLoc);
        }
      }
    }
  }

  /// Analyzes call expressions with a FunctionProtoType but no corresponding
  /// FunctionDecl, focusing on relating function arguments to the types of the
  /// corresponding parameter.
  /// TODO: When we analyze more complex slots than just top-level pointers,
  /// analyze the function parameter's nullability as a slot in the
  /// appropriate declaration.
  void visitFunctionProtoTypeCall(const FunctionProtoType &CalleeType,
                                  const CallExpr &Expr) {
    // For each pointer parameter of the function, ...
    for (unsigned I = 0; I < CalleeType.getNumParams(); ++I) {
      const auto ParamType = CalleeType.getParamType(I);
      if (!isSupportedPointerType(ParamType.getNonReferenceType())) continue;
      // the corresponding argument should also be a pointer.
      CHECK(isSupportedPointerType(Expr.getArg(I)->getType()))
          << "Unsupported argument " << I
          << " type: " << Expr.getArg(I)->getType().getAsString();

      // Analyze for potential constraints that the parameter's nullability
      // places on the argument's nullability.
      //
      // TODO: when we infer function pointer/reference parameters'
      // nullabilities, we'll need to check for overrides from previous
      // inference iterations. That will affect both
      // `getNullabilityAnnotationsFromType` (which doesn't account for
      // overrides) and in `visitAssignmentToType`, which will require the
      // relevant `LHSDecl` argument.
      auto ParamNullability = getNullabilityAnnotationsFromType(ParamType);
      visitAssignmentToType(ParamType, ParamNullability,
                            /*LHSDecl*/ nullptr, *Expr.getArg(I),
                            serializeLoc(SM, Expr.getArg(I)->getExprLoc()));
    }
  }

  /// Analyzes call expressions involving function pointers. It notes that the
  /// function pointer was dereferenced and matches up parameter/argument
  /// nullabilities.
  void visitFunctionPointerCallExpr(const Type &CalleeFunctionType,
                                    const CallExpr &Expr) {
    if (!HasInferableSlots) return;
    if (const auto *Callee = Expr.getCallee()) {
      // Function pointers are only ever raw pointers.
      if (const auto *PV = getRawPointerValue(Callee, Env)) {
        mustBeNonnull(getPointerNullStateOrDie(*PV),
                      serializeLoc(SM, Expr.getExprLoc()),
                      Evidence::UNCHECKED_DEREFERENCE);
      }
    }

    auto *CalleeFunctionProtoType =
        CalleeFunctionType.getAs<FunctionProtoType>();
    CHECK(CalleeFunctionProtoType);
    visitFunctionProtoTypeCall(*CalleeFunctionProtoType, Expr);
  }

  /// Analyzes a call to a function without a FunctionDecl. This situation can
  /// arise, for example, when a function is provided as a parameter.
  ///
  /// Example: We can collect evidence for the nullability of `p` and (when we
  /// handle more than top-level pointer slots) `j` in the following, based on
  /// the call to `callee`:
  /// ```
  ///  void target(int* p, void (*callee)(Nonnull<int*> i, int* j)) {
  ///    callee(p, nullptr);
  ///  }
  /// ```
  ///
  /// With `CalleeDecl` in this case not being a FunctionDecl as in most
  /// CallExpr cases, distinct handling is needed.
  void visitCallExprWithoutFunctionCalleeDecl(const Decl &CalleeDecl,
                                              const CallExpr &Expr) {
    if (CalleeDecl.isFunctionPointerType()) {
      if (auto *FuncType = CalleeDecl.getFunctionType()) {
        visitFunctionPointerCallExpr(*FuncType, Expr);
      } else {
        llvm::errs() << "Unsupported case of a function pointer type, for "
                        "which we aren't retrieving a valid FunctionType. \n";
        CalleeDecl.dump();
      }
      return;
    }

    // Ignore calls of pointers to members. The dereferencing of the pointer is
    // handled as a dereference at the BinaryOperator node, which additionally
    // captures pointers to fields.
    // TODO(b/309625642) Consider collecting evidence for the arguments being
    // passed as parameters to the pointed-to member.
    if (const auto *BinaryOpCallee = dyn_cast_or_null<BinaryOperator>(
            Expr.getCallee()->IgnoreParenImpCasts());
        BinaryOpCallee &&
        (BinaryOpCallee->getOpcode() == clang::BinaryOperatorKind::BO_PtrMemD ||
         BinaryOpCallee->getOpcode() ==
             clang::BinaryOperatorKind::BO_PtrMemI)) {
      return;
    }

    // Function references are a rare case, but similar to function pointers, we
    // can collect evidence from arguments assigned to parameter types.
    if (auto *FuncType = CalleeDecl.getFunctionType()) {
      if (auto *FuncProtoType = FuncType->getAs<FunctionProtoType>()) {
        visitFunctionProtoTypeCall(*FuncProtoType, Expr);
        return;
      }
    }

    // A reference to a function pointer is another rare case, but we can
    // collect the same evidence we would for a function pointer.
    if (const auto *CalleeAsValueDecl =
            dyn_cast<clang::ValueDecl>(&CalleeDecl)) {
      if (QualType CalleeType = CalleeAsValueDecl->getType();
          CalleeType.getNonReferenceType()->isFunctionPointerType()) {
        visitFunctionPointerCallExpr(
            *(CalleeType.getNonReferenceType()->getPointeeType()), Expr);
        return;
      }
    }

    // If we run into other cases meeting this criterion, skip them, but log
    // first so we can potentially add support later.
    llvm::errs() << "Unsupported case of a CallExpr without a FunctionDecl. "
                    "Skipping this CallExpr:\n";
    Expr.getBeginLoc().dump(CalleeDecl.getASTContext().getSourceManager());
    Expr.dump();
    llvm::errs() << "Which is a call to:\n";
    CalleeDecl.dump();
  }

  /// Analyzes `CallExpr`, which contains a call to our special macro
  /// single-argument capture function, to extract constraints on the argument.
  ///
  /// e.g. From `CHECK(x)`, we constrain `x` to not be null.
  void visitAbortIfFalseMacroCall(const CallExpr &CallExpr) {
    CHECK_EQ(CallExpr.getNumArgs(), 1);
    const Expr *Arg = CallExpr.getArg(0);
    if (!Arg) return;
    QualType ArgType = Arg->getType();
    if (isSupportedPointerType(ArgType)) {
      const dataflow::PointerValue *PV = getPointerValue(Arg, Env);
      if (!PV) return;
      mustBeNonnull(getPointerNullStateOrDie(*PV),
                    serializeLoc(SM, Arg->getExprLoc()),
                    Evidence::ABORT_IF_NULL);
    } else if (ArgType->isBooleanType()) {
      const dataflow::BoolValue *BV = Env.get<dataflow::BoolValue>(*Arg);
      if (!BV || BV->getKind() == dataflow::BoolValue::Kind::TopBool) return;

      Consumer.mustBeTrueByMarkingNonnull(BV->formula(),
                                          serializeLoc(SM, Arg->getExprLoc()),
                                          Evidence::ABORT_IF_NULL);
    }
  }

  /// Analyzes `CallExpr`, which contains a call to our special macro
  /// two-argument capture function for not-equal checks. Extracts potential
  /// constraints between the macro arguments.
  ///
  /// For example, from `CHECK_NE(x, nullptr)`, we constrain `x` to not be null.
  void visitAbortIfEqualMacroCall(const CallExpr &CallExpr) {
    CHECK_EQ(CallExpr.getNumArgs(), 2);
    const Expr *First = CallExpr.getArg(0);
    const Expr *Second = CallExpr.getArg(1);
    bool FirstSupported = isSupportedPointerType(First->getType());
    bool SecondSupported = isSupportedPointerType(Second->getType());
    if (!FirstSupported && !SecondSupported) return;

    ASTContext &Context = CallExpr.getCalleeDecl()->getASTContext();
    if (First->isNullPointerConstant(Context,
                                     Expr::NPC_ValueDependentIsNotNull)) {
      if (!SecondSupported) return;
      const dataflow::PointerValue *PV = getPointerValue(Second, Env);
      if (!PV) return;
      mustBeNonnull(getPointerNullStateOrDie(*PV),
                    serializeLoc(SM, Second->getExprLoc()),
                    Evidence::ABORT_IF_NULL);
    } else if (Second->isNullPointerConstant(
                   Context, Expr::NPC_ValueDependentIsNotNull)) {
      if (!FirstSupported) return;
      const dataflow::PointerValue *PV = getPointerValue(First, Env);
      if (!PV) return;
      mustBeNonnull(getPointerNullStateOrDie(*PV),
                    serializeLoc(SM, First->getExprLoc()),
                    Evidence::ABORT_IF_NULL);
    } else {
      if (!FirstSupported || !SecondSupported) {
        // If this happens outside of the nullptr literal case, we'd like to
        // know about it.
        llvm::errs() << "Value of a supported pointer type compared to a value "
                        "of a type that is not a supported pointer type.: \n";
        CallExpr.dump();
        CallExpr.getExprLoc().dump(
            CallExpr.getCalleeDecl()->getASTContext().getSourceManager());
        return;
      }

      const dataflow::PointerValue *FirstPV = getPointerValue(First, Env);
      if (!FirstPV) return;
      const dataflow::Formula *absl_nullable FirstIsNull =
          getPointerNullState(*FirstPV).IsNull;
      if (!FirstIsNull) return;

      const dataflow::PointerValue *SecondPV = getPointerValue(Second, Env);
      if (!SecondPV) return;
      const dataflow::Formula *absl_nullable SecondIsNull =
          getPointerNullState(*SecondPV).IsNull;
      if (!SecondIsNull) return;

      Consumer.collectAbortIfEqual(
          *FirstIsNull, serializeLoc(SM, First->getExprLoc()), *SecondIsNull,
          serializeLoc(SM, Second->getExprLoc()));
    }
  }

  void visitCallExpr(const Stmt &S) {
    auto *CallExpr = dyn_cast<clang::CallExpr>(&S);
    if (!CallExpr) return;
    auto *CalleeDecl = CallExpr->getCalleeDecl();
    if (!CalleeDecl) return;
    if (auto *CalleeFunctionDecl = dyn_cast<clang::FunctionDecl>(CalleeDecl)) {
      if (CalleeFunctionDecl->getDeclName().isIdentifier()) {
        llvm::StringRef Name = CalleeFunctionDecl->getName();
        if (Name == ArgCaptureAbortIfFalse) {
          visitAbortIfFalseMacroCall(*CallExpr);
          return;
        }
        if (Name == ArgCaptureAbortIfEqual) {
          visitAbortIfEqualMacroCall(*CallExpr);
          return;
        }
        if (const Expr *Initializer =
                getUnderlyingInitExprInStdMakeUnique(*CalleeFunctionDecl)) {
          if (const auto *CE = dyn_cast<CXXConstructExpr>(Initializer)) {
            visitArgsAndParams(*CE->getConstructor(), *CallExpr,
                               /*MayBeMissingImplicitConversion=*/true);
            return;
          }
          if (const auto *PLI = dyn_cast<CXXParenListInitExpr>(Initializer)) {
            visitMakeUniqueFieldInits(RecordInitListHelper(PLI), *CallExpr);
            return;
          }
          if (const auto *ImpCast = dyn_cast<ImplicitCastExpr>(Initializer);
              ImpCast && ImpCast->getCastKind() == CK_UserDefinedConversion) {
            // Just a user-defined-conversion operator, with no arguments being
            // forwarded. No need to log a warning of the unhandled case. Fall
            // through to the generic handling of calls below.
          } else {
            llvm::errs() << "Nullability: Unexpected initializer expression in "
                            "make_unique: "
                         << Initializer->getStmtClassName() << "\n";
            assert(false);
          }
        }
      }
      visitArgsAndParams(*CalleeFunctionDecl, *CallExpr,
                         /*MayBeMissingImplicitConversion=*/false);
    } else {
      visitCallExprWithoutFunctionCalleeDecl(*CalleeDecl, *CallExpr);
    }
  }

  void visitConstructExpr(const Stmt &S) {
    auto *ConstructExpr = dyn_cast<clang::CXXConstructExpr>(&S);
    if (!ConstructExpr) return;
    auto *ConstructorDecl = dyn_cast_or_null<clang::CXXConstructorDecl>(
        ConstructExpr->getConstructor());
    if (!ConstructorDecl) return;

    visitArgsAndParams(*ConstructorDecl, *ConstructExpr,
                       /*MayBeMissingImplicitConversion=*/false);
  }

  void visitReturn(const Stmt &S) {
    // Is this CFGElement a return statement?
    auto *ReturnStmt = dyn_cast<clang::ReturnStmt>(&S);
    if (!ReturnStmt) return;
    auto *ReturnExpr = ReturnStmt->getRetValue();
    if (!ReturnExpr) return;
    const FunctionDecl *CurrentFunc = Env.getCurrentFunc();
    CHECK(CurrentFunc) << "A return statement outside of a function?";
    if (!hasInferable(CurrentFunc->getReturnType()) ||
        !isSupportedPointerType(
            CurrentFunc->getReturnType().getNonReferenceType()))
      return;

    // Only gather evidence about the current function's return type if
    // the current function is an inference target and the return type
    // does not already include an annotation.
    if (isInferenceTarget(*CurrentFunc) &&
        !evidenceKindFromDeclaredReturnType(*CurrentFunc, Lattice.defaults())) {
      const dataflow::PointerValue *PV = getPointerValue(ReturnExpr, Env);
      if (!PV) return;
      Consumer.collectReturn(
          getOrGenerateUSR(USRCache, *CurrentFunc),
          getEvidenceTypeProperties(CurrentFunc->getReturnType()),
          getPointerNullState(*PV), serializeLoc(SM, ReturnExpr->getExprLoc()));
    }

    // Potentially infer back from the return type to returned expressions.
    TypeNullability ReturnTypeNullability =
        getReturnTypeNullabilityAnnotationsWithOverrides(*CurrentFunc, Lattice);
    // The FunctionDecl is the key used for overrides for the return type.
    visitAssignmentToType(CurrentFunc->getReturnType(), ReturnTypeNullability,
                          CurrentFunc, *ReturnExpr,
                          serializeLoc(SM, ReturnExpr->getExprLoc()));
  }

  /// Analyzes a direct assignment statement, aggregate initialization, etc.
  /// `RHS` is the assigned expression,  `LHSType` and `LHSNullability` are the
  /// assignee type and nullability (respectively), `LHSDecl` identifies the
  /// ValueDecl of the assignee whose type annotation may be overridden by
  /// future rounds of inference. When the ValueDecl doesn't take part in
  /// inference, nullptr should be passed for `LHSDecl`.
  void visitAssignmentLike(
      QualType LHSType, const TypeNullability &LHSNullability,
      const ValueDecl *absl_nullable LHSDecl, const Expr &RHS,
      const SerializedSrcLoc &Loc,
      Evidence::Kind EvidenceKindForAssignmentFromNullable =
          Evidence::ASSIGNED_FROM_NULLABLE) {
    const dataflow::PointerValue *PV = getPointerValue(&RHS, Env);
    if (!PV && !RHS.getType()->isNullPtrType()) return;
    visitAssignmentToType(LHSType, LHSNullability, LHSDecl, RHS, Loc);

    // Analyze potential constraints on the LHS.
    if (LHSNullability.empty()) return;
    std::optional<PointerNullState> NullState =
        PV ? std::make_optional(getPointerNullState(*PV)) : std::nullopt;
    Consumer.collectAssignmentFromValue(LHSNullability[0], NullState, Loc,
                                        EvidenceKindForAssignmentFromNullable);
  }

  /// Analyzes an assignment of `RHS` to `LHSDecl`, through a direct assignment
  /// statement, aggregate initialization, etc.
  void visitAssignmentLike(
      const ValueDecl &LHSDecl, const Expr &RHS, const SerializedSrcLoc &Loc,
      Evidence::Kind EvidenceKindForAssignmentFromNullable =
          Evidence::ASSIGNED_FROM_NULLABLE) {
    visitAssignmentLike(
        LHSDecl.getType(),
        getNullabilityAnnotationsFromDeclAndOverrides(LHSDecl, Lattice),
        &LHSDecl, RHS, Loc, EvidenceKindForAssignmentFromNullable);
  }

  /// Analyzes direct assignment statements, e.g. `p = nullptr`, whether
  /// initializing a new declaration or re-assigning to an existing declaration.
  void visitAssignment(const Stmt &S) {
    if (!HasInferableSlots) return;

    // Initialization of new decl.
    if (auto *DeclStmt = dyn_cast<clang::DeclStmt>(&S)) {
      for (auto *Decl : DeclStmt->decls()) {
        if (auto *VarDecl = dyn_cast<clang::VarDecl>(Decl);
            VarDecl && VarDecl->hasInit()) {
          bool DeclTypeSupported =
              isSupportedPointerType(VarDecl->getType().getNonReferenceType());
          bool InitTypeSupported = isSupportedPointerType(
              VarDecl->getInit()->getType().getNonReferenceType());
          if (!DeclTypeSupported) return;
          if (!InitTypeSupported) {
            llvm::errs() << "Unsupported init type for pointer decl: "
                         << VarDecl->getInit()->getType() << "\n";
            return;
          }
          visitAssignmentLike(
              *VarDecl, *VarDecl->getInit(),
              serializeLoc(SM, VarDecl->getInit()->getExprLoc()));
        }
      }
      return;
    }

    // Assignment to existing decl.
    const Expr *LHS = nullptr;
    const Expr *RHS = nullptr;
    std::optional<SourceLocation> Loc = std::nullopt;
    // Raw pointers.
    if (auto *BinaryOp = dyn_cast<clang::BinaryOperator>(&S);
        BinaryOp &&
        BinaryOp->getOpcode() == clang::BinaryOperatorKind::BO_Assign) {
      LHS = BinaryOp->getLHS();
      RHS = BinaryOp->getRHS();
      Loc = BinaryOp->getOperatorLoc();
    } else if (
        // Smart pointers.
        auto *CXXOpCall = dyn_cast<clang::CXXOperatorCallExpr>(&S);
        CXXOpCall && CXXOpCall->getOperator() == clang::OO_Equal) {
      LHS = CXXOpCall->getArg(0);
      RHS = CXXOpCall->getArg(1);
      Loc = CXXOpCall->getOperatorLoc();
    } else {
      return;
    }
    const QualType LHSType = LHS->getType();
    // Don't need to check the NonReferenceType here, because using references
    // in this assignment pattern still results in non-reference types for both
    // sides.
    if (!isSupportedPointerType(LHSType)) return;
    if (!isSupportedPointerType(RHS->getType())) return;

    const TypeNullability *TypeNullability = Lattice.getTypeNullability(LHS);
    // TODO(b/293609145) Without nullability tracked through the conditional
    // operator, we have no LHS type nullability for assignments where the LHS
    // is a conditional expression.
    if (TypeNullability == nullptr &&
        isa<ConditionalOperator>(dataflow::ignoreCFGOmittedNodes(*LHS)))
      return;
    CHECK(TypeNullability);
    // No value in passing the LHSDecl for overriding, because any relevant decl
    // will already be an element of InferableSlots, whose nullability is never
    // directly overridden. Instead, InferableSlots have symbolic nullability,
    // which is constrained (logically) by previous inferences.
    visitAssignmentLike(LHSType, *TypeNullability, /*LHSDecl*/ nullptr, *RHS,
                        serializeLoc(SM, *Loc));
  }

  void visitArithmeticArg(const Expr *Arg, const SerializedSrcLoc &Loc) {
    // No support needed for smart pointers, which do not support arithmetic
    // operations.
    if (!Arg || !isSupportedRawPointerType(Arg->getType())) return;
    if (auto *PV = getPointerValue(Arg, Env))
      mustBeNonnull(getPointerNullStateOrDie(*PV), Loc, Evidence::ARITHMETIC);
  }

  void visitArithmetic(const Stmt &S) {
    // A nullptr can be added to 0 and nullptr can be subtracted from nullptr
    // without hitting UB. But for now, we skip handling these special cases and
    // assume all pointers involved in these operations must be nonnull.
    switch (S.getStmtClass()) {
      default:
        return;
      case Stmt::CompoundAssignOperatorClass: {
        auto *Op = cast<clang::CompoundAssignOperator>(&S);
        switch (Op->getOpcode()) {
          default:
            return;
          case BO_AddAssign:
          case BO_SubAssign:
            visitArithmeticArg(Op->getLHS(),
                               serializeLoc(SM, Op->getExprLoc()));
        }
        break;
      }
      case Stmt::BinaryOperatorClass: {
        auto *Op = cast<clang::BinaryOperator>(&S);
        switch (Op->getOpcode()) {
          default:
            return;
          case BO_Add:
          case BO_Sub:
            visitArithmeticArg(Op->getLHS(),
                               serializeLoc(SM, Op->getExprLoc()));
            visitArithmeticArg(Op->getRHS(),
                               serializeLoc(SM, Op->getExprLoc()));
        }
        break;
      }
      case Stmt::UnaryOperatorClass: {
        auto *Op = cast<clang::UnaryOperator>(&S);
        switch (Op->getOpcode()) {
          default:
            return;
          case UO_PostInc:
          case UO_PreInc:
          case UO_PostDec:
          case UO_PreDec:
            visitArithmeticArg(Op->getSubExpr(),
                               serializeLoc(SM, Op->getExprLoc()));
        }
        break;
      }
    }
  }

  void visitCFGInitializer(const CFGInitializer &CFGInit) {
    const CXXCtorInitializer *Initializer = CFGInit.getInitializer();
    if (!Initializer) {
      // We expect this not to be the case, but not to a production-crash-worthy
      // level, so assert instead of CHECK.
      llvm::errs() << "CFGInitializer with null CXXCtorInitializer.\n";
      CFGInit.dump();
      assert(Initializer);
    }

    // Base and delegating initializers are collected from when we see the
    // underlying CXXConstructExpr, so we don't need to handle those, only the
    // member initializers.
    const FieldDecl *Field = Initializer->getAnyMember();
    if (Field == nullptr || !HasInferableSlots ||
        !isSupportedPointerType(Field->getType()))
      return;

    bool IsDefaultInitializer = Initializer->isInClassMemberInitializer();
    if (isSupportedSmartPointerType(Field->getType()) &&
        !IsDefaultInitializer && !Initializer->isWritten()) {
      // We skip unwritten non-default member initializers for smart pointer
      // fields because we check the end block of the constructor for the
      // fields' nullability later. This allows us to avoid inferring Nullable
      // for smart pointers without default initializers that are only ever (and
      // always) assigned to a Nonnull value in constructor bodies.
      return;
    }

    const Expr *InitExpr = Initializer->getInit();
    bool NullptrDefaultInit =
        IsDefaultInitializer && isOrIsConstructedFromNullPointerConstant(
                                    InitExpr, Field->getASTContext());

    visitAssignmentLike(
        *Field, *InitExpr, serializeLoc(SM, InitExpr->getExprLoc()),
        NullptrDefaultInit ? Evidence::NULLPTR_DEFAULT_MEMBER_INITIALIZER
                           : Evidence::ASSIGNED_FROM_NULLABLE);
  }

  void visitFieldInits(const RecordInitListHelper &Helper) {
    // Any initialization of base classes/fields will be collected from the
    // InitListExpr for the base initialization, so, here, we only need to
    // analyze the field inits.
    for (auto [Field, InitExpr] : Helper.field_inits()) {
      if (!isSupportedPointerType(Field->getType())) continue;

      visitAssignmentLike(*Field, *InitExpr,
                          serializeLoc(SM, InitExpr->getExprLoc()));
    }
  }

  void visitMakeUniqueFieldInits(const RecordInitListHelper &Helper,
                                 const CallExpr &MakeUniqueCall) {
    // Skip through the base inits to get to the field inits. Any initialization
    // of base classes/fields will be collected from the InitListExpr for the
    // base initialization.
    int I = Helper.base_inits().size();
    // Use the arguments from the `MakeUniqueCall` instead of the `field_inits`
    // from `Helper`.
    int NumArgs = MakeUniqueCall.getNumArgs();
    for (auto [Field, InitExpr] : Helper.field_inits()) {
      if (!isSupportedPointerType(Field->getType())) {
        ++I;
        continue;
      }
      const Expr *Arg = (I < NumArgs) ? MakeUniqueCall.getArg(I) : InitExpr;
      // We might be missing implicit conversions (in the make_unique body
      // instead of the call site). So we check the type of the argument is
      // as expected as well.
      if (!isSupportedPointerType(Arg->getType()) &&
          !Arg->getType()->isNullPtrType()) {
        ++I;
        continue;
      }
      visitAssignmentLike(*Field, *Arg, serializeLoc(SM, Arg->getExprLoc()));
      ++I;
    }
  }

  void visitAggregateInitialization(const Stmt &S) {
    if (auto *InitList = dyn_cast<clang::InitListExpr>(&S);
        InitList && InitList->getType()->isRecordType() &&
        !(InitList->isSemanticForm() && InitList->isTransparent())) {
      visitFieldInits(RecordInitListHelper(InitList));
      return;
    }
    if (auto *ParenListInit = dyn_cast<clang::CXXParenListInitExpr>(&S);
        ParenListInit && ParenListInit->getType()->isRecordType()) {
      visitFieldInits(RecordInitListHelper(ParenListInit));
    }
  }

  void visitArraySubscript(const Stmt &S) {
    // For raw pointers, we see an ArraySubscriptExpr.
    if (auto *Op = dyn_cast<clang::ArraySubscriptExpr>(&S)) {
      const Expr *Base = Op->getBase();
      if (!Base || !isSupportedRawPointerType(Base->getType())) return;
      if (auto *PV = getPointerValue(Base, Env))
        mustBeNonnull(getPointerNullStateOrDie(*PV),
                      serializeLoc(SM, Op->getRBracketLoc()),
                      Evidence::ARRAY_SUBSCRIPT);
      return;
    }
    // For smart pointers to arrays, we see a CXXOperatorCallExpr.
    // Other smart pointers do not have a subscript operator.
    if (auto *Call = dyn_cast<clang::CXXOperatorCallExpr>(&S);
        Call && Call->getOperator() == clang::OO_Subscript) {
      const Expr *Base = Call->getArg(0);
      if (!Base || !isSupportedSmartPointerType(Base->getType())) return;
      if (auto *PV = getPointerValue(Base, Env))
        mustBeNonnull(getPointerNullStateOrDie(*PV),
                      serializeLoc(SM, Call->getOperatorLoc()),
                      Evidence::ARRAY_SUBSCRIPT);
    }
  }

 private:
  BehaviorConsumer &Consumer;
  const Environment &Env;
  // Whether the definition being analyzed has any inferable slots. Lack of
  // inferable slots simplifies the analysis.
  const bool HasInferableSlots;
  USRCache &USRCache;
  const PointerNullabilityLattice &Lattice;
  const SourceManager &SM;
  // Null if the TU is not a GoogleTest TU. Otherwise, if it is a test TU,
  // then this LocFilter will allow all locations, except the test main file.
  LocFilter* absl_nullable NotTestMainFileLocFilter;
};
}  // namespace

// TODO: b/440317964 -- move this function to upstream, where Atoms are defined.
static unsigned getAtomNumber(Atom A) {
  return static_cast<std::underlying_type_t<Atom>>(A);
}

static FormulaProto saveFormula(const dataflow::Formula &F) {
  FormulaProto Proto;
  llvm::raw_string_ostream OS(*Proto.mutable_serialized());
  serializeFormula(F, OS);
  return Proto;
}

static LogicalContext saveLogicalContext(llvm::DenseSet<Atom> UsedTokens,
                                         const DataflowAnalysisContext &AC) {
  clang::dataflow::SimpleLogicalContext Ctx =
      AC.exportLogicalContext(std::move(UsedTokens));

  LogicalContext CtxProto;

  if (Ctx.Invariant != nullptr)
    *CtxProto.mutable_invariant() = saveFormula(*Ctx.Invariant);

  auto &AtomDefs = *CtxProto.mutable_atom_defs();
  for (auto [A, F] : Ctx.TokenDefs) {
    CHECK(F != nullptr);
    AtomDefs[getAtomNumber(A)] = saveFormula(*F);
  }

  auto &AtomDeps = *CtxProto.mutable_atom_deps();
  for (auto &[A, Deps] : Ctx.TokenDeps) {
    LogicalContext::AtomSet &DepSet = AtomDeps[getAtomNumber(A)];
    for (Atom Dep : Deps) {
      DepSet.add_atoms(getAtomNumber(Dep));
    }
  }

  return CtxProto;
}

// TODO: b/440317964 -- lift these next two functions out into a common library
// and share with eligible_ranges.
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

static NullabilityKind fromProtoNullability(Nullability N) {
  switch (N) {
    case Nullability::NONNULL:
      return NullabilityKind::NonNull;
    case Nullability::NULLABLE:
      return NullabilityKind::Nullable;
    case Nullability::UNKNOWN:
      return NullabilityKind::Unspecified;
  }
  llvm_unreachable("Unsupported nullability value");
}

static PointerTypeNullabilityProto savePointerTypeNullability(
    const PointerTypeNullability &PTN) {
  PointerTypeNullabilityProto Proto;

  if (PTN.isSymbolic()) {
    auto &S = *Proto.mutable_symbolic();
    S.set_nonnull_atom(getAtomNumber(PTN.nonnullAtom()));
    S.set_nullable_atom(getAtomNumber(PTN.nullableAtom()));
    return Proto;
  }

  Proto.set_concrete(toProtoNullability(PTN.concrete()));

  return Proto;
}

static PointerNullStateProto savePointerNullState(
    const PointerNullState &NullState) {
  PointerNullStateProto Proto;
  if (NullState.FromNullable)
    *Proto.mutable_from_nullable() = saveFormula(*NullState.FromNullable);
  if (NullState.IsNull)
    *Proto.mutable_is_null() = saveFormula(*NullState.IsNull);
  return Proto;
}

static InferableSlotProto saveInferableSlot(const InferableSlot &IS) {
  InferableSlotProto Proto;
  const PointerTypeNullability &PTN = IS.getSymbolicNullability();
  Proto.set_nonnull_atom(getAtomNumber(PTN.nonnullAtom()));
  Proto.set_nullable_atom(getAtomNumber(PTN.nullableAtom()));
  Proto.set_slot(IS.getTargetSlot());
  Proto.mutable_symbol()->set_usr(IS.getInferenceTargetUSR());
  return Proto;
}

// When collecting evidence from summaries, there are two means by which we can
// take previous inferences into account:
//
// 1. InferableSlots -- these are represented symbolically in the summary, and
// we directly tie the symbols to any information learned about them in previous
// inference rounds.
//
// 2. Explicitly identified declarations -- these are declarations for which we
// infer Nullability, but are not referred to by name in the context they are
// used and therefore do not have a corresponding InferableSlot. These only
// occur in the case of ValueAssignedToType and are identified explicitly with a
// fingerprint of the corresponding symbol.
//
// For the latter case, we maintain a map from fingerprints to Nullability
// information, if available.
using FingerprintNullabilityMap =
    absl::flat_hash_map<SlotFingerprint, std::optional<PointerTypeNullability>>;

// A nullability overlay function specifically for the declarations tracked in
// the FingerprintNullabilityMap (see corresponding comments).
using NullabilityOverlay = llvm::function_ref<
    const PointerTypeNullability *absl_nullable(SlotFingerprint)>;

// Creates a NullabilityOverlay that captures previous inferences. Uses
// `NullabilityMap` to back the overlay. The map is updated on demand as calls
// are made to the overlay. `NullabilityMap` must outlive the returned lambda.
static auto getUsrBasedNullabilityOverlay(
    FingerprintNullabilityMap &NullabilityMap,
    const PreviousInferences &PreviousInferences) {
  return [&](SlotFingerprint Fingerprint)
             -> const PointerTypeNullability *absl_nullable {
    auto [It, Inserted] = NullabilityMap.try_emplace(Fingerprint);
    if (Inserted) {
      if (PreviousInferences.Nullable->contains(Fingerprint)) {
        It->second.emplace(NullabilityKind::Nullable);
      } else if (PreviousInferences.Nonnull->contains(Fingerprint)) {
        It->second.emplace(NullabilityKind::NonNull);
      } else {
        It->second = std::nullopt;
      }
    }
    if (!It->second) return nullptr;
    return &*It->second;
  };
}

// Gets the Atom associated with `AtomNumber`, creating a new one if needed.
static Atom getAtom(unsigned AtomNumber, dataflow::Arena &Arena,
                    llvm::DenseMap<unsigned, Atom> &AtomMap) {
  auto [It, Inserted] = AtomMap.try_emplace(AtomNumber, Arena.makeAtom());
  return It->second;
}

static llvm::Expected<PointerTypeNullability> loadPointerTypeNullability(
    const PointerTypeNullabilityProto &Proto, dataflow::Arena &Arena,
    llvm::DenseMap<unsigned, Atom> &AtomMap) {
  if (Proto.has_symbolic()) {
    return PointerTypeNullability::createSymbolic(
        getAtom(Proto.symbolic().nonnull_atom(), Arena, AtomMap),
        getAtom(Proto.symbolic().nullable_atom(), Arena, AtomMap));
  }
  return PointerTypeNullability(fromProtoNullability(Proto.concrete()));
}

static llvm::Expected<PointerNullState> loadPointerNullState(
    const PointerNullStateProto &Proto, dataflow::Arena &Arena,
    llvm::DenseMap<unsigned, Atom> &AtomMap) {
  const Formula *FromNullable = nullptr;
  if (Proto.has_from_nullable()) {
    llvm::Expected<const Formula *> Loaded =
        parseFormula(Proto.from_nullable().serialized(), Arena, AtomMap);
    if (!Loaded) return Loaded.takeError();
    FromNullable = *Loaded;
  }
  const Formula *IsNull = nullptr;
  if (Proto.has_is_null()) {
    llvm::Expected<const Formula *> Loaded =
        parseFormula(Proto.is_null().serialized(), Arena, AtomMap);
    if (!Loaded) return Loaded.takeError();
    IsNull = *Loaded;
  }
  return PointerNullState{FromNullable, IsNull};
}

static std::vector<InferableSlot> loadInferableSlots(
    const proto2::RepeatedPtrField<InferableSlotProto> &Protos,
    dataflow::Arena &Arena, llvm::DenseMap<unsigned, Atom> &AtomMap) {
  std::vector<InferableSlot> InferableSlots;
  for (const auto &ISProto : Protos) {
    InferableSlots.emplace_back(
        PointerTypeNullability::createSymbolic(
            getAtom(ISProto.nonnull_atom(), Arena, AtomMap),
            getAtom(ISProto.nullable_atom(), Arena, AtomMap)),
        static_cast<Slot>(ISProto.slot()), ISProto.symbol().usr());
  }
  return InferableSlots;
}

static llvm::Expected<dataflow::SimpleLogicalContext> loadLogicalContext(
    const LogicalContext &LC, dataflow::Arena &Arena,
    llvm::DenseMap<unsigned, Atom> &AtomMap) {
  dataflow::SimpleLogicalContext Ctx;

  if (LC.has_invariant()) {
    llvm::Expected<const Formula *> Invariant =
        parseFormula(LC.invariant().serialized(), Arena, AtomMap);
    if (!Invariant) return Invariant.takeError();
    Ctx.Invariant = *Invariant;
  } else {
    Ctx.Invariant = nullptr;
  }

  for (const auto &[AtomNumber, FProto] : LC.atom_defs()) {
    llvm::Expected<const Formula *> Formula =
        parseFormula(FProto.serialized(), Arena, AtomMap);
    if (!Formula) return Formula.takeError();
    Ctx.TokenDefs[getAtom(AtomNumber, Arena, AtomMap)] = *Formula;
  }

  for (const auto &[AtomNumber, DepSet] : LC.atom_deps()) {
    llvm::DenseSet<Atom> Deps;
    for (uint32_t DepAtomNumber : DepSet.atoms())
      Deps.insert(getAtom(DepAtomNumber, Arena, AtomMap));
    Ctx.TokenDeps[getAtom(AtomNumber, Arena, AtomMap)] = std::move(Deps);
  }

  return Ctx;
}

namespace {
// Collects nullability summaries and outputs them as protobufs.
class SummaryCollector {
 public:
  static llvm::SmallVector<NullabilityBehaviorSummary> summarizeElement(
      llvm::ArrayRef<InferableSlot> InferableSlots,
      llvm::DenseSet<Atom>& UsedTokens, USRCache& USRCache,
      const PointerNullabilityLattice& Lattice, const Environment& Env,
      const SourceManager& SM, const CFGElement& CFGElem,
      LocFilter* absl_nullable NotTestMainFileLocFilter) {
    SummaryCollector SCollector(Env, UsedTokens);
    NullabilityBehaviorVisitor<SummaryCollector>::visit(
        InferableSlots, USRCache, Lattice, Env, SM, CFGElem,
        NotTestMainFileLocFilter, SCollector);
    // Consume the summaries generated in this visit.
    return std::move(SCollector.Summaries);
  }

  void mustBeTrueByMarkingNonnull(const Formula &MustBeTrue,
                                  const SerializedSrcLoc &Loc,
                                  Evidence::Kind EvidenceKind) {
    addRequiresAnnotationSummary(Nullability::NONNULL, MustBeTrue, Loc,
                                 EvidenceKind);
  }

  void mustBeTrueByMarkingNullable(const Formula &MustBeTrue,
                                   const SerializedSrcLoc &Loc,
                                   Evidence::Kind EvidenceKind) {
    addRequiresAnnotationSummary(Nullability::NULLABLE, MustBeTrue, Loc,
                                 EvidenceKind);
  }

  // `IsLHSTypeConst` indicates whether the LHS type is const, after stripping
  // away any potential outer reference type.
  //
  // `LHSFingerprint` provides the fingerprint of the LHS decl, when necessary
  // for tracking potential overrides from previous rounds of inference. This
  // specifically applies to decls that can be overridden but are not inferable
  // slots. Currently, that covers return types and callee parameters (which are
  // not named at call sites).
  void collectAssignmentToType(
      bool IsLHSTypeConst, const PointerTypeNullability &LHSTypeNullability,
      std::optional<SlotFingerprint> LHSFingerprint,
      const std::optional<PointerTypeNullability> &RHSTypeNullability,
      const PointerNullState &RHSValueNullability,
      const SerializedSrcLoc &RHSLoc) {
    ValueAssignedToTypeSummary &S = *addSummary().mutable_value_assigned();
    S.set_lhs_is_non_reference_const(IsLHSTypeConst);
    *S.mutable_lhs_type_nullability() =
        savePointerTypeNullability(LHSTypeNullability);
    if (LHSFingerprint) S.set_lhs_decl_fingerprint(*LHSFingerprint);
    if (RHSTypeNullability) {
      *S.mutable_rhs_type_nullability() =
          savePointerTypeNullability(*RHSTypeNullability);
    }
    *S.mutable_rhs_value_nullability() =
        savePointerNullState(RHSValueNullability);
    S.set_rhs_loc(RHSLoc.Loc);
  }

  void collectMustHaveInvariantTypeNullability(
      std::string_view TargetUSR, Slot TargetSlot,
      const PointerTypeNullability& NullabilityToMatch,
      const SerializedSrcLoc& CollectionLoc) {
    TypeAssignedToInvariantTypeSummary& S =
        *addSummary().mutable_assigned_to_invariant_type();
    *S.mutable_nullability_to_match() =
        savePointerTypeNullability(NullabilityToMatch);
    S.mutable_target_symbol()->set_usr(TargetUSR);
    S.set_target_slot(TargetSlot);
    S.set_location(CollectionLoc.Loc);
  }

  void collectMustHaveInvariantTypeNullability(
      const PointerTypeNullability& TargetNullability,
      const PointerTypeNullability& NullabilityToMatch,
      SlotFingerprint FingerprintOfNullabilityToMatch,
      const SerializedSrcLoc& CollectionLoc) {
    TypeAssignedToInvariantTypeSummary& S =
        *addSummary().mutable_assigned_to_invariant_type();
    *S.mutable_nullability_to_match() =
        savePointerTypeNullability(NullabilityToMatch);
    S.set_nullability_to_match_fingerprint(FingerprintOfNullabilityToMatch);
    *S.mutable_target_nullability() =
        savePointerTypeNullability(TargetNullability);
    S.set_location(CollectionLoc.Loc);
  }

  void collectAssignmentFromValue(
      PointerTypeNullability TypeNullability,
      std::optional<PointerNullState> ValueNullState,
      const SerializedSrcLoc &ValueLoc,
      Evidence::Kind EvidenceKindForAssignmentFromNullable) {
    AssignmentFromValueSummary &S =
        *addSummary().mutable_assignment_from_value();
    *S.mutable_lhs_type_nullability() =
        savePointerTypeNullability(TypeNullability);
    if (ValueNullState)
      *S.mutable_rhs_null_state() = savePointerNullState(*ValueNullState);
    S.set_rhs_loc(ValueLoc.Loc);
    S.set_evidence_kind(EvidenceKindForAssignmentFromNullable);
  }

  void collectArgBinding(std::string_view FunctionUSR, Slot ParamSlot,
                         EvidenceTypeProperties ParamTyProps,
                         std::optional<PointerNullState> ArgNullState,
                         const SerializedSrcLoc &ArgLoc) {
    BindingSummary &S = *addSummary().mutable_argument_binding();
    S.mutable_function_symbol()->set_usr(FunctionUSR);
    S.set_slot(ParamSlot);
    S.set_type_is_lvalue_ref(ParamTyProps.IsLValueRef);
    S.set_type_is_const(ParamTyProps.IsNonReferenceConst);
    if (ArgNullState)
      *S.mutable_null_state() = savePointerNullState(*ArgNullState);
    S.set_location(ArgLoc.Loc);
  }

  void collectAbortIfEqual(const dataflow::Formula &FirstIsNull,
                           const SerializedSrcLoc &FirstLoc,
                           const dataflow::Formula &SecondIsNull,
                           const SerializedSrcLoc &SecondLoc) {
    AbortIfEqualSummary &S = *addSummary().mutable_abort_if_equal();
    *S.mutable_first_is_null() = saveFormula(FirstIsNull);
    S.set_first_loc(FirstLoc.Loc);
    *S.mutable_second_is_null() = saveFormula(SecondIsNull);
    S.set_second_loc(SecondLoc.Loc);
  }

  void collectReturn(std::string_view FunctionUSR,
                     EvidenceTypeProperties ReturnTyProps,
                     PointerNullState ReturnNullState,
                     const SerializedSrcLoc &ReturnLoc) {
    BindingSummary &S = *addSummary().mutable_returned();
    S.mutable_function_symbol()->set_usr(FunctionUSR);
    S.set_slot(SLOT_RETURN_TYPE);
    S.set_type_is_lvalue_ref(ReturnTyProps.IsLValueRef);
    S.set_type_is_const(ReturnTyProps.IsNonReferenceConst);
    *S.mutable_null_state() = savePointerNullState(ReturnNullState);
    S.set_location(ReturnLoc.Loc);
  }

 private:
  // Constructed object retains all reference parameters. They must outlive
  // it. `UsedTokens` is considered an in/out parameter -- it may be non-empty
  // on entry to the function.
  SummaryCollector(const Environment &Env, llvm::DenseSet<Atom> &UsedTokens)
      : UsedTokens(UsedTokens), Env(Env) {}

  NullabilityBehaviorSummary &addSummary() {
    Atom A = Env.getFlowConditionToken();
    UsedTokens.insert(A);
    NullabilityBehaviorSummary &S = Summaries.emplace_back();
    S.set_block_atom(getAtomNumber(A));
    return S;
  }

  void addRequiresAnnotationSummary(Nullability N, const Formula &MustBeTrue,
                                    const SerializedSrcLoc &Loc,
                                    Evidence::Kind EvidenceKind) {
    RequiresAnnotationSummary &RAS =
        *addSummary().mutable_requires_annotation();
    RAS.set_required_annotation(N);
    *RAS.mutable_formula() = saveFormula(MustBeTrue);
    RAS.set_location(Loc.Loc);
    RAS.set_evidence_kind(EvidenceKind);
  }

  llvm::SmallVector<NullabilityBehaviorSummary> Summaries;
  llvm::DenseSet<Atom> &UsedTokens;
  const Environment &Env;
};

class SummaryEvidenceCollector {
 public:
  static llvm::Error collectEvidenceFromBehaviorSummary(
      const std::vector<InferableSlot> &InferableSlots,
      const Formula &InferableSlotsConstraint, const Environment &Env,
      const dataflow::Solver &Solver,
      const NullabilityBehaviorSummary &BehaviorSummary,
      const NullabilityOverlay &NullabilityOverlay,
      llvm::function_ref<EvidenceEmitter> Emit,
      llvm::DenseMap<unsigned, dataflow::Atom> &AtomMap) {
    SummaryEvidenceCollector Collector(InferableSlots, InferableSlotsConstraint,
                                       Emit, Env, Solver, AtomMap,
                                       NullabilityOverlay);
    return Collector.collectEvidenceFromBehaviorSummary(BehaviorSummary);
  }

 private:
  SummaryEvidenceCollector(const std::vector<InferableSlot> &InferableSlots,
                           const Formula &InferableSlotsConstraint,
                           llvm::function_ref<EvidenceEmitter> Emit,
                           const Environment &Env,
                           const dataflow::Solver &Solver,
                           llvm::DenseMap<unsigned, dataflow::Atom> &AtomMap,
                           const NullabilityOverlay &NullabilityOverlay)
      : Collector(InferableSlots, InferableSlotsConstraint, Env, Solver, Emit),
        Arena(Env.arena()),
        AtomMap(AtomMap),
        NullabilityOverlay(NullabilityOverlay) {}

  llvm::Error collectEvidenceFromBehaviorSummary(
      const NullabilityBehaviorSummary &BehaviorSummary) {
    switch (BehaviorSummary.behavior_case()) {
      case NullabilityBehaviorSummary::kRequiresAnnotation:
        return collectRequiresAnnotation(BehaviorSummary.requires_annotation());
      case NullabilityBehaviorSummary::kValueAssigned:
        return collectValueAssignedToType(BehaviorSummary.value_assigned());
      case NullabilityBehaviorSummary::kAssignmentFromValue:
        return collectAssignmentFromValue(
            BehaviorSummary.assignment_from_value());
      case NullabilityBehaviorSummary::kArgumentBinding:
        return collectArgumentBinding(BehaviorSummary.argument_binding());
      case NullabilityBehaviorSummary::kAbortIfEqual:
        return collectAbortIfEqual(BehaviorSummary.abort_if_equal());
      case NullabilityBehaviorSummary::kReturned:
        return collectReturn(BehaviorSummary.returned());
      case NullabilityBehaviorSummary::kOnExit:
        return collectExitBlock(BehaviorSummary.on_exit());
      case NullabilityBehaviorSummary::kAssignedToInvariantType:
        return collectTypeAssignedToInvariantType(
            BehaviorSummary.assigned_to_invariant_type());

      case NullabilityBehaviorSummary::BEHAVIOR_NOT_SET:
        return llvm::createStringError(llvm::errc::invalid_argument,
                                       "No NullabilityBehaviorSummary set");
    }
    return llvm::createStringError(
        llvm::errc::invalid_argument,
        "Unexpected NullabilityBehaviorSummary case");
  }

  llvm::Error collectRequiresAnnotation(
      const RequiresAnnotationSummary &Summary) {
    llvm::Expected<const Formula *> MustBeTrue =
        parseFormula(Summary.formula().serialized(), Arena, AtomMap);
    if (!MustBeTrue) return MustBeTrue.takeError();

    SerializedSrcLoc Loc = {Summary.location()};
    Evidence::Kind EvidenceKind = Summary.evidence_kind();

    switch (Summary.required_annotation()) {
      case Nullability::NONNULL:
        Collector.mustBeTrueByMarkingNonnull(**MustBeTrue, Loc, EvidenceKind);
        break;
      case Nullability::NULLABLE:
        Collector.mustBeTrueByMarkingNullable(**MustBeTrue, Loc, EvidenceKind);
        break;
      default:
        return llvm::createStringError(llvm::errc::invalid_argument,
                                       "Unsupported RequiredAnnotation in "
                                       "collectRequiresAnnotation");
    }
    return llvm::Error::success();
  }

  llvm::Error collectValueAssignedToType(
      const ValueAssignedToTypeSummary &Summary) {
    auto LHSTypeNullability = [&]() -> llvm::Expected<PointerTypeNullability> {
      if (Summary.has_lhs_decl_fingerprint()) {
        if (const PointerTypeNullability *absl_nullable O =
                NullabilityOverlay(Summary.lhs_decl_fingerprint()))
          return *O;
      }
      return loadPointerTypeNullability(Summary.lhs_type_nullability(), Arena,
                                        AtomMap);
    }();
    if (!LHSTypeNullability) return LHSTypeNullability.takeError();

    std::optional<PointerTypeNullability> RHSTypeNullability;
    if (Summary.has_rhs_type_nullability()) {
      llvm::Expected<PointerTypeNullability> Loaded =
          loadPointerTypeNullability(Summary.rhs_type_nullability(), Arena,
                                     AtomMap);
      if (!Loaded) return Loaded.takeError();
      RHSTypeNullability = *std::move(Loaded);
    }

    llvm::Expected<PointerNullState> RHSValueNullability =
        loadPointerNullState(Summary.rhs_value_nullability(), Arena, AtomMap);
    if (!RHSValueNullability) return RHSValueNullability.takeError();

    // Callee doesn't use the `LHSFingerprint` argument -- it is only included
    // because the signature is required by `SummaryCollector`.
    Collector.collectAssignmentToType(
        Summary.lhs_is_non_reference_const(), *LHSTypeNullability,
        /*LHSFingerprint*/ std::nullopt, RHSTypeNullability,
        *RHSValueNullability, {Summary.rhs_loc()});
    return llvm::Error::success();
  }

  llvm::Error collectTypeAssignedToInvariantType(
      const TypeAssignedToInvariantTypeSummary& Summary) {
    if (Summary.has_target_nullability() ==
            (Summary.has_target_symbol() && Summary.has_target_slot()) ||
        (Summary.has_target_symbol() != Summary.has_target_slot())) {
      return llvm::createStringError(
          "Exactly one of `target_nullability` or (`target_symbol` and "
          "`target_slot`) should be set, depending on whether the side of the "
          "assignment we are inferring for was a known decl or a potentially "
          "arbitrary expression.");
    }

    auto NullabilityToMatch = [&]() -> llvm::Expected<PointerTypeNullability> {
      if (Summary.has_nullability_to_match_fingerprint()) {
        if (const PointerTypeNullability* absl_nullable O =
                NullabilityOverlay(Summary.nullability_to_match_fingerprint()))
          return *O;
      }
      return loadPointerTypeNullability(Summary.nullability_to_match(), Arena,
                                        AtomMap);
    }();
    if (!NullabilityToMatch) return NullabilityToMatch.takeError();

    if (Summary.has_target_nullability()) {
      auto TargetNullability = loadPointerTypeNullability(
          Summary.target_nullability(), Arena, AtomMap);

      if (!TargetNullability) return TargetNullability.takeError();
      Collector.collectMustHaveInvariantTypeNullability(
          *TargetNullability, *NullabilityToMatch, std::nullopt,
          {Summary.location()});
    } else {
      Collector.collectMustHaveInvariantTypeNullability(
          Summary.target_symbol().usr(),
          static_cast<Slot>(Summary.target_slot()), *NullabilityToMatch,
          {Summary.location()});
    }
    return llvm::Error::success();
  }

  llvm::Error collectAssignmentFromValue(
      const AssignmentFromValueSummary &Summary) {
    llvm::Expected<PointerTypeNullability> LHSTypeNullability =
        loadPointerTypeNullability(Summary.lhs_type_nullability(), Arena,
                                   AtomMap);
    if (!LHSTypeNullability) return LHSTypeNullability.takeError();

    std::optional<PointerNullState> RHSNullState;
    if (Summary.has_rhs_null_state()) {
      llvm::Expected<PointerNullState> Loaded =
          loadPointerNullState(Summary.rhs_null_state(), Arena, AtomMap);
      if (!Loaded) return Loaded.takeError();
      RHSNullState = *std::move(Loaded);
    }

    Collector.collectAssignmentFromValue(*LHSTypeNullability, RHSNullState,
                                         {Summary.rhs_loc()},
                                         Summary.evidence_kind());
    return llvm::Error::success();
  }

  llvm::Error collectArgumentBinding(const BindingSummary &Summary) {
    std::optional<PointerNullState> ArgNullState;
    if (Summary.has_null_state()) {
      llvm::Expected<PointerNullState> Loaded =
          loadPointerNullState(Summary.null_state(), Arena, AtomMap);
      if (!Loaded) return Loaded.takeError();
      ArgNullState = *std::move(Loaded);
    }

    Collector.collectArgBinding(
        Summary.function_symbol().usr(), static_cast<Slot>(Summary.slot()),
        {Summary.type_is_lvalue_ref(), Summary.type_is_const()}, ArgNullState,
        {Summary.location()});
    return llvm::Error::success();
  }

  llvm::Error collectAbortIfEqual(const AbortIfEqualSummary &Summary) {
    llvm::Expected<const Formula *> ParsedFormula =
        parseFormula(Summary.first_is_null().serialized(), Arena, AtomMap);
    if (!ParsedFormula) return ParsedFormula.takeError();
    // If parsing succeeds, we expect the pointer is nonnull. But, we have no
    // guarantee so we check to be safe.
    if (*ParsedFormula == nullptr)
      return llvm::createStringError(
          llvm::errc::invalid_argument,
          "Error in formula parsing: null formula returned");
    const Formula &FirstIsNull = **ParsedFormula;

    ParsedFormula =
        parseFormula(Summary.second_is_null().serialized(), Arena, AtomMap);
    if (!ParsedFormula) return ParsedFormula.takeError();
    // If parsing succeeds, we expect the pointer is nonnull. But, we have no
    // guarantee so we check to be safe.
    if (*ParsedFormula == nullptr)
      return llvm::createStringError(
          llvm::errc::invalid_argument,
          "Error in formula parsing: null formula returned");
    const Formula &SecondIsNull = **ParsedFormula;

    Collector.collectAbortIfEqual(FirstIsNull, {Summary.first_loc()},
                                  SecondIsNull, {Summary.second_loc()});
    return llvm::Error::success();
  }

  llvm::Error collectReturn(const BindingSummary &Summary) {
    llvm::Expected<PointerNullState> ReturnNullState =
        loadPointerNullState(Summary.null_state(), Arena, AtomMap);
    if (!ReturnNullState) return ReturnNullState.takeError();

    Collector.collectReturn(
        Summary.function_symbol().usr(),
        {Summary.type_is_lvalue_ref(), Summary.type_is_const()},
        *ReturnNullState, {Summary.location()});
    return llvm::Error::success();
  }

  llvm::Error collectExitBlock(const ExitBlockSummary &Summary) {
    for (const auto &InitOnExit : Summary.ctor_inits_on_exit()) {
      llvm::Expected<PointerNullState> NullState =
          loadPointerNullState(InitOnExit.null_state(), Arena, AtomMap);
      if (!NullState) return NullState.takeError();

      Collector.collectConstructorExitBlock(
          InitOnExit.field().usr(), *NullState, {InitOnExit.location()});
    }
    for (const auto &InitOnExit : Summary.late_inits_on_exit()) {
      llvm::Expected<PointerNullState> NullState =
          loadPointerNullState(InitOnExit.null_state(), Arena, AtomMap);
      if (!NullState) return NullState.takeError();

      Collector.collectSupportedLateInitializerExitBlock(
          InitOnExit.field().usr(), *NullState, {InitOnExit.location()});
    }
    return llvm::Error::success();
  }

  EvidenceCollector Collector;
  dataflow::Arena &Arena;
  llvm::DenseMap<unsigned, dataflow::Atom> &AtomMap;
  const NullabilityOverlay &NullabilityOverlay;
};
}  // namespace

// Convenience override for functions that follow. Handles translation from AST
// data into the lower-level evidence format used by `Emit`.
static void wrappedEmit(llvm::function_ref<EvidenceEmitter> Emit,
                        USRCache &USRCache, const Decl &Target, Slot S,
                        Evidence::Kind Kind, SourceLocation Loc) {
  CHECK(isInferenceTarget(Target))
      << "Evidence emitted for a Target which is not an inference target: "
      << (dyn_cast<NamedDecl>(&Target)
              ? dyn_cast<NamedDecl>(&Target)->getQualifiedNameAsString()
              : "not a named decl");
  auto &SM = Target.getDeclContext()->getParentASTContext().getSourceManager();

  std::string_view USR = getOrGenerateUSR(USRCache, Target);
  if (!USR.empty()) Emit(makeEvidence(USR, S, Kind, serializeLoc(SM, Loc).Loc));
}

/// Returns a function that the analysis can use to override Decl nullability
/// values from the source code being analyzed with previously inferred
/// nullabilities.
///
/// In practice, this should only override the default nullability for Decls
/// that do not spell out a nullability in source code, because we only pass in
/// inferences from the previous round which are non-trivial and annotations
/// "inferred" by reading an annotation from source code in the previous round
/// were marked trivial.
static auto getConcreteNullabilityOverrideFromPreviousInferences(
    ConcreteNullabilityCache &Cache, USRCache &USRCache,
    const PreviousInferences &PreviousInferences) {
  return [&](const Decl &D) -> std::optional<const PointerTypeNullability *> {
    auto [It, Inserted] = Cache.try_emplace(&D);
    if (Inserted) {
      std::optional<const Decl *> FingerprintedDecl;
      Slot Slot;
      if (auto *FD = dyn_cast<FunctionDecl>(&D)) {
        FingerprintedDecl = FD;
        Slot = SLOT_RETURN_TYPE;
      } else if (auto *PD = dyn_cast<ParmVarDecl>(&D)) {
        if (auto *Parent = dyn_cast_or_null<FunctionDecl>(
                PD->getParentFunctionOrMethod())) {
          FingerprintedDecl = Parent;
          Slot = paramSlot(PD->getFunctionScopeIndex());
        }
      }
      if (!FingerprintedDecl) return std::nullopt;
      auto Fingerprint =
          fingerprint(getOrGenerateUSR(USRCache, **FingerprintedDecl), Slot);
      if (PreviousInferences.Nullable->contains(Fingerprint)) {
        It->second.emplace(NullabilityKind::Nullable);
      } else if (PreviousInferences.Nonnull->contains(Fingerprint)) {
        It->second.emplace(NullabilityKind::NonNull);
      } else {
        It->second = std::nullopt;
      }
    }
    if (!It->second) return std::nullopt;
    return &*It->second;
  };
}

template <typename ContainerT>
static bool hasAnyInferenceTargets(const ContainerT &Decls) {
  return std::any_of(Decls.begin(), Decls.end(),
                     [](const Decl *D) { return D && isInferenceTarget(*D); });
}

static bool hasAnyInferenceTargets(dataflow::ReferencedDecls &RD) {
  return hasAnyInferenceTargets(RD.Fields) ||
         hasAnyInferenceTargets(RD.Globals) ||
         hasAnyInferenceTargets(RD.Functions) ||
         hasAnyInferenceTargets(RD.Locals) ||
         hasAnyInferenceTargets(RD.LambdaCapturedParams);
}

std::unique_ptr<dataflow::Solver> makeDefaultSolverForInference() {
  constexpr std::int64_t MaxSATIterations = 2'000'000;
  return std::make_unique<dataflow::WatchedLiteralsSolver>(MaxSATIterations);
}

using EBInitHandler =
    llvm::function_ref<void(std::string_view USR, PointerNullState NullState,
                            const SerializedSrcLoc &Loc)>;

// If D is a constructor definition, summarizes cases of potential
// LEFT_NULLABLE_BY_CONSTRUCTOR evidence for smart pointer fields implicitly
// default-initialized and left nullable in the exit block of the constructor
// body.
static void processConstructorExitBlock(const clang::Decl &MaybeConstructor,
                                        const Environment &ExitEnv,
                                        USRCache &USRCache,
                                        EBInitHandler InitHandler) {
  auto *Ctor = dyn_cast<CXXConstructorDecl>(&MaybeConstructor);
  if (!Ctor) return;
  for (auto *Initializer : Ctor->inits()) {
    if (Initializer->isWritten() || Initializer->isInClassMemberInitializer()) {
      // We collect evidence from explicitly-written member initializers and
      // default member initializers elsewhere, when analyzing the
      // constructor's CFGInitializers.
      continue;
    }
    const FieldDecl *Field = Initializer->getAnyMember();
    if (Field == nullptr || !isSupportedSmartPointerType(Field->getType()) ||
        !isInferenceTarget(*Field))
      continue;
    // `Field` is a smart pointer field that was not explicitly
    // initialized in the constructor member initializer list and does not
    // have a default member initializer, so it was default constructed
    // (and null) at the beginning of the constructor body.

    // If it is still nullable in the constructor's exit block environment, we
    // will collect evidence that it was assigned from a nullable value.
    const dataflow::PointerValue *PV = getPointerValueFromSmartPointer(
        cast<dataflow::RecordStorageLocation>(
            ExitEnv.getThisPointeeStorageLocation()->getChild(*Field)),
        ExitEnv);
    if (PV == nullptr) continue;
    // We have seen constructors that copy/move into *this that leave smart
    // pointer fields without null state, because the field of the
    // copied/moved-from value is not modeled or referenced and so has no null
    // state. We don't get useful evidence from these cases anyway, because
    // whether the field has been initialized with a non-null value is
    // determined by some other form of construction for the same type, and
    // we'll collect the relevant evidence there. If we did try to model the
    // fields that are not directly referenced in the function body, we would
    // only get null state based on the annotated type, which is always Unknown
    // if we're trying to infer for it. In these cases, skip this field.
    //
    // It's possible there are other cases that result in a lack of null state
    // that should be fixed, but we don't have the tools to detect the
    // difference between the case detailed above and other cases, so we give up
    // on finding other cases by way of loudly detecting a lack of null state
    // here.
    if (!hasPointerNullState(*PV)) continue;

    PointerNullState NullState = getPointerNullState(*PV);
    auto &SM =
        Field->getDeclContext()->getParentASTContext().getSourceManager();
    std::string_view USR = getOrGenerateUSR(USRCache, *Field);
    SerializedSrcLoc Loc = serializeLoc(
        SM, Ctor->isImplicit() ? Field->getBeginLoc() : Ctor->getBeginLoc());
    InitHandler(USR, NullState, Loc);
  }
}

// Supported late initializers are no-argument SetUp methods of classes that
// inherit from ::testing::Test. From the exit block of such a method, we
// collect LEFT_NOT_NULLABLE_BY_LATE_INITIALIZER evidence for smart pointer
// fields that are not nullable. This allows ignoring the
// LEFT_NULLABLE_BY_CONSTRUCTOR evidence for such a field.
static void processSupportedLateInitializerExitBlock(
    const clang::Decl &MaybeLateInitializationMethod,
    const Environment &ExitEnv, USRCache &USRCache, EBInitHandler InitHandler) {
  auto *Method = dyn_cast<CXXMethodDecl>(&MaybeLateInitializationMethod);
  if (!Method || !Method->isVirtual() || Method->getNumParams() != 0) return;
  if (IdentifierInfo *Identifier = Method->getIdentifier();
      !Identifier || Identifier->getName() != "SetUp") {
    return;
  }
  const CXXRecordDecl *absl_nullable Parent = Method->getParent();
  if (!Parent) return;
  CXXBasePaths BasePaths;
  if (!Parent->lookupInBases(
          [](const clang::CXXBaseSpecifier *BaseSpec,
             clang::CXXBasePath &Path) {
            return BaseSpec->getType().getCanonicalType().getAsString() ==
                   "class testing::Test";
          },
          BasePaths)) {
    return;
  }

  const dataflow::RecordStorageLocation *ThisPointeeLoc =
      ExitEnv.getThisPointeeStorageLocation();
  CHECK(ThisPointeeLoc) << "Storage location for *this should be available "
                           "while analyzing a method.";
  for (const auto [ChildDecl, ChildLoc] : ThisPointeeLoc->children()) {
    if (!isSupportedSmartPointerType(ChildDecl->getType()) ||
        !isInferenceTarget(*ChildDecl))
      continue;
    const dataflow::PointerValue *PV = getPointerValueFromSmartPointer(
        cast<dataflow::RecordStorageLocation>(ChildLoc), ExitEnv);
    if (PV != nullptr && hasPointerNullState(*PV)) {
      PointerNullState NullState = getPointerNullState(*PV);
      std::string_view USR = getOrGenerateUSR(USRCache, *ChildDecl);
      SerializedSrcLoc Loc =
          serializeLoc(Method->getParentASTContext().getSourceManager(),
                       Method->getBeginLoc());
      InitHandler(USR, NullState, Loc);
    }
  }
}

static void collectEvidenceFromConstructorExitBlock(
    const clang::Decl &MaybeConstructor, const Environment &ExitEnv,
    USRCache &USRCache, EvidenceCollector &Collector) {
  processConstructorExitBlock(
      MaybeConstructor, ExitEnv, USRCache,
      [&Collector](std::string_view USR, PointerNullState NullState,
                   const SerializedSrcLoc &Loc) {
        Collector.collectConstructorExitBlock(USR, NullState, Loc);
      });
}

static void collectEvidenceFromSupportedLateInitializerExitBlock(
    const clang::Decl &MaybeLateInitializationMethod,
    const Environment &ExitEnv, USRCache &USRCache,
    EvidenceCollector &Collector) {
  processSupportedLateInitializerExitBlock(
      MaybeLateInitializationMethod, ExitEnv, USRCache,
      [&Collector](std::string_view USR, PointerNullState NullState,
                   const SerializedSrcLoc &Loc) {
        Collector.collectSupportedLateInitializerExitBlock(USR, NullState, Loc);
      });
}

// `EBSummary` is an out-parameter for accumulating results.
static void summarizeConstructorExitBlock(const clang::Decl &MaybeConstructor,
                                          const Environment &ExitEnv,
                                          USRCache &USRCache,
                                          ExitBlockSummary &EBSummary) {
  processConstructorExitBlock(
      MaybeConstructor, ExitEnv, USRCache,
      [&EBSummary](std::string_view USR, PointerNullState NullState,
                   const SerializedSrcLoc &Loc) {
        InitOnExitSummary &InitSummary = *EBSummary.add_ctor_inits_on_exit();
        InitSummary.mutable_field()->set_usr(USR);
        *InitSummary.mutable_null_state() = savePointerNullState(NullState);
        InitSummary.set_location(Loc.Loc);
      });
}

// `EBSummary` is an out-parameter for accumulating results.
static void summarizeSupportedLateInitializerExitBlock(
    const clang::Decl &MaybeLateInitializationMethod,
    const Environment &ExitEnv, USRCache &USRCache,
    ExitBlockSummary &EBSummary) {
  processSupportedLateInitializerExitBlock(
      MaybeLateInitializationMethod, ExitEnv, USRCache,
      [&EBSummary](std::string_view USR, PointerNullState NullState,
                   const SerializedSrcLoc &Loc) {
        InitOnExitSummary &LateInitSummary =
            *EBSummary.add_late_inits_on_exit();
        LateInitSummary.mutable_field()->set_usr(USR);
        *LateInitSummary.mutable_null_state() = savePointerNullState(NullState);
        LateInitSummary.set_location(Loc.Loc);
      });
}

// Checks the "last layer" forwarding functions called from the given statement.
// This allows us to collect references made within forwarding functions, as if
// they were made directly by the statement. (skipping through the forwarding).
static llvm::DenseSet<const FunctionDecl *>
collectLastLayerForwardingFunctionsCalled(Stmt &S) {
  llvm::DenseSet<const FunctionDecl *> Results;

  class ForwardingFunctionsCallVisitor : public dataflow::AnalysisASTVisitor {
   public:
    ForwardingFunctionsCallVisitor(
        llvm::DenseSet<const FunctionDecl *> &Results)
        : Results(Results) {}

    bool VisitCallExpr(CallExpr *E) override {
      const FunctionDecl *Callee = E->getDirectCallee();
      if (Callee != nullptr) {
        if (const FunctionDecl *FD = getLastForwardingFunctionLayer(*Callee))
          Results.insert(FD);
      }
      return true;
    }

    llvm::DenseSet<const FunctionDecl *> &Results;
  };

  ForwardingFunctionsCallVisitor Visitor(Results);
  Visitor.TraverseStmt(&S);
  return Results;
}

static void collectReferencesFromForwardingFunctions(
    Stmt &S, dataflow::ReferencedDecls &ReferencedDecls) {
  llvm::DenseSet<const FunctionDecl *> ForwardingFunctions =
      collectLastLayerForwardingFunctionsCalled(S);
  for (const auto *ForwardingFunction : ForwardingFunctions) {
    dataflow::ReferencedDecls More =
        dataflow::getReferencedDecls(*ForwardingFunction);
    ReferencedDecls.Fields.insert(More.Fields.begin(), More.Fields.end());
    ReferencedDecls.Globals.insert(More.Globals.begin(), More.Globals.end());
    ReferencedDecls.Locals.insert(More.Locals.begin(), More.Locals.end());
    ReferencedDecls.Functions.insert(More.Functions.begin(),
                                     More.Functions.end());
    ReferencedDecls.LambdaCapturedParams.insert(
        More.LambdaCapturedParams.begin(), More.LambdaCapturedParams.end());
  }
}

static bool containsInitListExpr(const Expr &E) {
  // Short-circuit for the obvious case.
  if (isa<InitListExpr>(&E)) return true;

  class ContainsInitListExprVisitor
      : public RecursiveASTVisitor<ContainsInitListExprVisitor> {
   public:
    bool TraverseInitListExpr(const InitListExpr *E) {
      Found = true;
      return false;
    }

    bool Found = false;
  };

  ContainsInitListExprVisitor Visitor;
  // Unfortunately, RecursiveASTVisitor requires a non-const input.
  Visitor.TraverseStmt(&const_cast<Expr &>(E));
  return Visitor.Found;
}

static llvm::Expected<Stmt *absl_nonnull> getTarget(
    const Decl &Definition, std::optional<DeclStmt> &DeclStmtForVarDecl) {
  if (const auto *TargetAsFunc = dyn_cast<FunctionDecl>(&Definition)) {
    if (!TargetAsFunc->doesThisDeclarationHaveABody()) {
      return llvm::createStringError(llvm::errc::invalid_argument,
                                     "Function definitions must have a body.");
    }
    Stmt *TargetStmt = TargetAsFunc->getBody();
    CHECK(TargetStmt) << "TargetStmt should have been assigned a non-null "
                         "value, because function must have body.";
    return TargetStmt;
  }

  if (const auto *Var = dyn_cast<VarDecl>(&Definition)) {
    if (!Var->hasInit()) {
      return llvm::createStringError(
          llvm::errc::invalid_argument,
          "Variable definitions must have an initializer.");
    }
    if (auto *VTSD = dyn_cast<VarTemplateSpecializationDecl>(Var);
        VTSD && Var->getInit() && containsInitListExpr(*Var->getInit())) {
      return llvm::createStringError(llvm::errc::not_supported,
                                     "Variable template specializations with "
                                     "InitListExprs in their initializers "
                                     "are currently unsupported.");
    }
    // Synthesize a temporary DeclStmt for the assignment of the variable to
    // its initializing expression. This is an unusual pattern that does not
    // perfectly reflect the CFG or AST for declaration or assignment of a
    // global variable, and it is possible that this may cause unexpected
    // behavior in clang tools/utilities.
    Stmt *TargetStmt =
        &DeclStmtForVarDecl.emplace(DeclGroupRef(const_cast<VarDecl *>(Var)),
                                    Var->getBeginLoc(), Var->getEndLoc());
    return TargetStmt;
  }

  std::string Msg =
      "Unable to find a valid target definition from Definition:\n";
  llvm::raw_string_ostream Stream(Msg);
  Definition.dump(Stream);
  return llvm::createStringError(llvm::inconvertibleErrorCode(), Msg);
}

static std::vector<InferableSlot> gatherInferableSlots(
    TypeNullabilityDefaults Defaults,
    const FunctionDecl* absl_nullable TargetAsFunc, const Stmt& TargetStmt,
    const dataflow::ReferencedDecls& ReferencedDecls,
    PointerNullabilityAnalysis& Analysis, dataflow::Arena& Arena,
    USRCache& USRCache, LocFilter* absl_nullable NotTestMainFileLocFilter) {
  std::vector<InferableSlot> InferableSlots;

  // Don't consider slots where the target (e.g., caller) is defined in a test
  // main file, but the referenced decl (e.g., callee) is not.
  LocFilter* absl_nullable ActiveFilter = nullptr;
  if (NotTestMainFileLocFilter != nullptr) {
    bool IsTargetInTestMainFile =
        !NotTestMainFileLocFilter->check(TargetStmt.getBeginLoc());
    // If the target is defined in a test main file, then have a LocFilter
    // to check the referenced decl. Otherwise, we can skip the check.
    if (IsTargetInTestMainFile) ActiveFilter = NotTestMainFileLocFilter;
  }
  auto ShouldConsiderReference = [&](const DeclaratorDecl& D) {
    if (ActiveFilter == nullptr) return true;
    return !ActiveFilter->check(D.getCanonicalDecl()->getBeginLoc());
  };

  if (TargetAsFunc && isInferenceTarget(*TargetAsFunc)) {
    auto Parameters = TargetAsFunc->parameters();
    for (auto I = 0; I < Parameters.size(); ++I) {
      const ParmVarDecl *Param = Parameters[I];
      const TypeSourceInfo *TSI = Param->getTypeSourceInfo();
      if (hasInferable(Param->getType()) &&
          (!TSI ||
           !evidenceKindFromDeclaredTypeLoc(TSI->getTypeLoc(), Defaults))) {
        InferableSlots.emplace_back(
            Analysis.assignNullabilityVariable(Param, Arena), paramSlot(I),
            *TargetAsFunc, USRCache);
      }
    }
  }

  for (const FieldDecl *Field : ReferencedDecls.Fields) {
    const TypeSourceInfo *TSI = Field->getTypeSourceInfo();
    if (isInferenceTarget(*Field) &&
        (!TSI ||
         !evidenceKindFromDeclaredTypeLoc(TSI->getTypeLoc(), Defaults)) &&
        ShouldConsiderReference(*Field)) {
      InferableSlots.emplace_back(
          Analysis.assignNullabilityVariable(Field, Arena), Slot(0), *Field,
          USRCache);
    }
  }
  for (const VarDecl *Global : ReferencedDecls.Globals) {
    const TypeSourceInfo *TSI = Global->getTypeSourceInfo();
    if (isInferenceTarget(*Global) &&
        (!TSI ||
         !evidenceKindFromDeclaredTypeLoc(TSI->getTypeLoc(), Defaults)) &&
        ShouldConsiderReference(*Global)) {
      InferableSlots.emplace_back(
          Analysis.assignNullabilityVariable(Global, Arena), Slot(0), *Global,
          USRCache);
    }
  }
  for (const VarDecl *Local : ReferencedDecls.Locals) {
    const TypeSourceInfo *TSI = Local->getTypeSourceInfo();
    if (isInferenceTarget(*Local) &&
        (!TSI ||
         !evidenceKindFromDeclaredTypeLoc(TSI->getTypeLoc(), Defaults))) {
      InferableSlots.emplace_back(
          Analysis.assignNullabilityVariable(Local, Arena), Slot(0), *Local,
          USRCache);
    }
  }

  // Add functions in the reverse of the order in which they are referenced.
  // This is a heuristic based on the use of fallback functions to provide
  // nonnull values when one or more initial functions may provide a nullable
  // value. The goal is to reduce conflicting evidence for the return types of
  // those initial functions. For example, we'd prefer to collect evidence for a
  // dereference of the value of `getNonnullFallback()` from the following
  // pattern, rather than the value of `getNullable()`, which is likely to get
  // contradictory evidence of a nullable return value from its body:
  // ```
  // int* P = getNullable();
  // if (P == nullptr) {
  //   P = getNonnullFallback();
  // }
  // *P;
  // ```
  for (auto Iter = ReferencedDecls.Functions.rbegin();
       Iter != ReferencedDecls.Functions.rend(); ++Iter) {
    const FunctionDecl* Function = *Iter;
    if (isInferenceTarget(*Function) &&
        hasInferable(Function->getReturnType()) &&
        !evidenceKindFromDeclaredReturnType(*Function, Defaults) &&
        ShouldConsiderReference(*Function)) {
      InferableSlots.emplace_back(
          Analysis.assignNullabilityVariable(Function, Arena), SLOT_RETURN_TYPE,
          *Function, USRCache);
    }
  }
  for (const ParmVarDecl *Param : ReferencedDecls.LambdaCapturedParams) {
    CHECK_EQ(Param->getFunctionScopeDepth(), 0)
        << "Not expecting lambda capture of anything with depth > 0.";
    if (const auto *ContainingFunction =
            dyn_cast<FunctionDecl>(Param->getParentFunctionOrMethod());
        ContainingFunction && isInferenceTarget(*ContainingFunction) &&
        hasInferable(Param->getType()) &&
        (!Param->getTypeSourceInfo() ||
         !evidenceKindFromDeclaredTypeLoc(
             Param->getTypeSourceInfo()->getTypeLoc(), Defaults))) {
      unsigned Index = Param->getFunctionScopeIndex();
      InferableSlots.emplace_back(
          Analysis.assignNullabilityVariable(Param, Arena), paramSlot(Index),
          *ContainingFunction, USRCache);
    }
  }
  return InferableSlots;
}

llvm::Error collectEvidenceFromDefinition(
    const Decl &Definition, llvm::function_ref<EvidenceEmitter> Emit,
    USRCache &USRCache, const NullabilityPragmas &Pragmas,
    const PreviousInferences &PreviousInferences,
    const SolverFactory &MakeSolver) {
  std::optional<DeclStmt> DeclStmtForVarDecl;
  auto T = getTarget(Definition, DeclStmtForVarDecl);
  if (!T) return T.takeError();
  Stmt &TargetStmt = **T;

  const auto *absl_nullable TargetFunc = dyn_cast<FunctionDecl>(&Definition);
  dataflow::ReferencedDecls ReferencedDecls =
      TargetFunc != nullptr ? dataflow::getReferencedDecls(*TargetFunc)
                            : dataflow::getReferencedDecls(TargetStmt);
  collectReferencesFromForwardingFunctions(TargetStmt, ReferencedDecls);

  // TODO: b/416755108 -- We should be able to check functions as
  // well (and therefore drop the `!TargetFunc` filter), but we're missing some
  // Referenced constructors, so `hasAnyInferenceTargets` will fail for certain
  // functions.
  if (!TargetFunc && !isInferenceTarget(Definition) &&
      !hasAnyInferenceTargets(ReferencedDecls))
    return llvm::Error::success();

  ASTContext &Ctx = Definition.getASTContext();
  llvm::Expected<dataflow::AdornedCFG> ACFG =
      dataflow::AdornedCFG::build(Definition, TargetStmt, Ctx);
  if (!ACFG) return ACFG.takeError();

  std::unique_ptr<dataflow::Solver> Solver = MakeSolver();
  DataflowAnalysisContext AnalysisContext(*Solver);
  Environment Env = TargetFunc ? Environment(AnalysisContext, *TargetFunc)
                               : Environment(AnalysisContext, TargetStmt);
  PointerNullabilityAnalysis Analysis(Ctx, Env, Pragmas);

  std::unique_ptr<LocFilter> NotTestMainFileLocFilter =
      getNotTestMainFileLocFilter(Ctx);

  std::vector<InferableSlot> InferableSlots = gatherInferableSlots(
      TypeNullabilityDefaults(Ctx, Pragmas), TargetFunc, TargetStmt,
      ReferencedDecls, Analysis, AnalysisContext.arena(), USRCache,
      NotTestMainFileLocFilter.get());

  // Here, we overlay new knowledge from past iterations over the symbolic
  // entities for the `InferableSlots` (whose symbols are invariant across
  // inference iterations).
  const auto &InferableSlotsConstraint = getConstraintsOnInferableSlots(
      InferableSlots, PreviousInferences, AnalysisContext.arena());

  ConcreteNullabilityCache ConcreteNullabilityCache;
  Analysis.assignNullabilityOverride(
      getConcreteNullabilityOverrideFromPreviousInferences(
          ConcreteNullabilityCache, USRCache, PreviousInferences));

  std::vector<
      std::optional<dataflow::DataflowAnalysisState<PointerNullabilityLattice>>>
      Results;
  dataflow::CFGEltCallbacks<PointerNullabilityAnalysis> PostAnalysisCallbacks;
  PostAnalysisCallbacks.Before =
      [&](const CFGElement& Element,
          const dataflow::DataflowAnalysisState<PointerNullabilityLattice>&
              State) {
        if (Solver->reachedLimit()) return;

        EvidenceCollector Collector(InferableSlots, InferableSlotsConstraint,
                                    State.Env, *Solver, Emit);
        NullabilityBehaviorVisitor<EvidenceCollector>::visit(
            InferableSlots, USRCache, State.Lattice, State.Env,
            Ctx.getSourceManager(), Element, NotTestMainFileLocFilter.get(),
            Collector);
      };
  if (llvm::Error Error = dataflow::runDataflowAnalysis(*ACFG, Analysis, Env,
                                                        PostAnalysisCallbacks)
                              .moveInto(Results))
    return Error;

  if (Results.empty()) return llvm::Error::success();

  if (std::optional<dataflow::DataflowAnalysisState<PointerNullabilityLattice>>
          &ExitBlockResult = Results[ACFG->getCFG().getExit().getBlockID()]) {
    EvidenceCollector Collector(InferableSlots, InferableSlotsConstraint,
                                ExitBlockResult->Env, *Solver, Emit);
    collectEvidenceFromConstructorExitBlock(Definition, ExitBlockResult->Env,
                                            USRCache, Collector);
    collectEvidenceFromSupportedLateInitializerExitBlock(
        Definition, ExitBlockResult->Env, USRCache, Collector);
  }

  if (Solver->reachedLimit()) {
    return llvm::createStringError(llvm::errc::interrupted,
                                   "SAT solver reached iteration limit");
  }

  return llvm::Error::success();
}

llvm::Expected<CFGSummary> summarizeDefinition(
    const Decl &Definition, USRCache &USRCache,
    const NullabilityPragmas &Pragmas, const SolverFactory &MakeSolver) {
  std::optional<DeclStmt> DeclStmtForVarDecl;
  llvm::Expected<Stmt *absl_nonnull> T =
      getTarget(Definition, DeclStmtForVarDecl);
  if (!T) return T.takeError();
  Stmt &TargetStmt = **T;

  const auto *absl_nullable TargetFunc = dyn_cast<FunctionDecl>(&Definition);
  dataflow::ReferencedDecls ReferencedDecls =
      TargetFunc != nullptr ? dataflow::getReferencedDecls(*TargetFunc)
                            : dataflow::getReferencedDecls(TargetStmt);
  collectReferencesFromForwardingFunctions(TargetStmt, ReferencedDecls);

  CFGSummary Summary;
  // TODO: b/416755108 -- We should be able to check functions as
  // well (and therefore drop the `!TargetFunc` filter), but we're missing some
  // Referenced constructors, so `hasAnyInferenceTargets` will fail for certain
  // functions.
  if (!TargetFunc && !isInferenceTarget(Definition) &&
      !hasAnyInferenceTargets(ReferencedDecls))
    return Summary;

  ASTContext &Ctx = Definition.getASTContext();
  llvm::Expected<dataflow::AdornedCFG> ACFG =
      dataflow::AdornedCFG::build(Definition, TargetStmt, Ctx);
  if (!ACFG) return ACFG.takeError();

  std::unique_ptr<dataflow::Solver> Solver = MakeSolver();
  DataflowAnalysisContext AnalysisContext(*Solver);
  Environment Env = TargetFunc ? Environment(AnalysisContext, *TargetFunc)
                               : Environment(AnalysisContext, TargetStmt);
  PointerNullabilityAnalysis Analysis(Ctx, Env, Pragmas);

  std::unique_ptr<LocFilter> NotTestMainFileLocFilter =
      getNotTestMainFileLocFilter(Ctx);

  std::vector<InferableSlot> InferableSlots = gatherInferableSlots(
      TypeNullabilityDefaults(Ctx, Pragmas), TargetFunc, TargetStmt,
      ReferencedDecls, Analysis, AnalysisContext.arena(), USRCache,
      NotTestMainFileLocFilter.get());

  for (const auto &IS : InferableSlots) {
    *Summary.add_inferable_slots() = saveInferableSlot(IS);
  }

  llvm::DenseSet<Atom> UsedTokens;
  std::vector<
      std::optional<dataflow::DataflowAnalysisState<PointerNullabilityLattice>>>
      Results;
  dataflow::CFGEltCallbacks<PointerNullabilityAnalysis> PostAnalysisCallbacks;
  PostAnalysisCallbacks.Before =
      [&](const CFGElement& Element,
          const dataflow::DataflowAnalysisState<PointerNullabilityLattice>&
              State) {
        if (Solver->reachedLimit()) return;
        for (NullabilityBehaviorSummary& S : SummaryCollector::summarizeElement(
                 InferableSlots, UsedTokens, USRCache, State.Lattice, State.Env,
                 Ctx.getSourceManager(), Element,
                 NotTestMainFileLocFilter.get())) {
          *Summary.add_behavior_summaries() = std::move(S);
        }
      };
  if (llvm::Error Error = dataflow::runDataflowAnalysis(*ACFG, Analysis, Env,
                                                        PostAnalysisCallbacks)
                              .moveInto(Results))
    return Error;

  // When `Results` are empty, the post-analysis callbacks should not have been
  // called, so `Summary` should be empty. But, to be sure, explicitly return a
  // fresh empty summary.
  if (Results.empty()) return CFGSummary();

  if (std::optional<dataflow::DataflowAnalysisState<PointerNullabilityLattice>>
          &ExitBlockResult = Results[ACFG->getCFG().getExit().getBlockID()]) {
    ExitBlockSummary EBSummary;
    summarizeConstructorExitBlock(Definition, ExitBlockResult->Env, USRCache,
                                  EBSummary);
    summarizeSupportedLateInitializerExitBlock(Definition, ExitBlockResult->Env,
                                               USRCache, EBSummary);
    if (EBSummary.ctor_inits_on_exit_size() > 0 ||
        EBSummary.late_inits_on_exit_size() > 0) {
      Atom A = ExitBlockResult->Env.getFlowConditionToken();
      UsedTokens.insert(A);

      NullabilityBehaviorSummary &NBS = *Summary.add_behavior_summaries();
      NBS.set_block_atom(getAtomNumber(A));
      *NBS.mutable_on_exit() = std::move(EBSummary);
    }
  }

  *Summary.mutable_logical_context() =
      saveLogicalContext(std::move(UsedTokens), AnalysisContext);

  if (Solver->reachedLimit()) {
    return llvm::createStringError(llvm::errc::interrupted,
                                   "SAT solver reached iteration limit");
  }

  return Summary;
}

llvm::Error collectEvidenceFromSummary(
    const CFGSummary &Summary, llvm::function_ref<EvidenceEmitter> Emit,
    const PreviousInferences &PreviousInferences,
    const SolverFactory &MakeSolver) {
  std::unique_ptr<dataflow::Solver> Solver = MakeSolver();
  DataflowAnalysisContext AnalysisContext(*Solver);
  llvm::DenseMap<unsigned, dataflow::Atom> AtomMap;
  llvm::Expected<dataflow::SimpleLogicalContext> Ctx = loadLogicalContext(
      Summary.logical_context(), AnalysisContext.arena(), AtomMap);
  if (!Ctx) return Ctx.takeError();
  AnalysisContext.initLogicalContext(*std::move(Ctx));
  dataflow::Arena &Arena = AnalysisContext.arena();
  std::vector<InferableSlot> InferableSlots =
      loadInferableSlots(Summary.inferable_slots(), Arena, AtomMap);

  const auto &InferableSlotsConstraint =
      getConstraintsOnInferableSlots(InferableSlots, PreviousInferences, Arena);

  FingerprintNullabilityMap NullabilityMap;
  auto NullabilityOverlay =
      getUsrBasedNullabilityOverlay(NullabilityMap, PreviousInferences);

  for (const auto &BehaviorSummary : Summary.behavior_summaries()) {
    Environment Env(AnalysisContext,
                    getAtom(BehaviorSummary.block_atom(), Arena, AtomMap));
    if (llvm::Error Err =
            SummaryEvidenceCollector::collectEvidenceFromBehaviorSummary(
                InferableSlots, InferableSlotsConstraint, Env, *Solver,
                BehaviorSummary, NullabilityOverlay, Emit, AtomMap))
      return Err;
    if (Solver->reachedLimit()) {
      return llvm::createStringError(llvm::errc::interrupted,
                                     "SAT solver reached iteration limit");
    }
  }

  return llvm::Error::success();
}

static void collectEvidenceFromDefaultArgument(
    const clang::FunctionDecl &Fn, const clang::ParmVarDecl &ParamDecl,
    Slot ParamSlot, llvm::function_ref<EvidenceEmitter> Emit,
    USRCache &USRCache) {
  // We don't handle all cases of default arguments, because the expressions
  // used for the argument are not available in any CFG, because the AST nodes
  // are once-per-decl children of the ParmVarDecl, not once-per-call children
  // of the CallExpr. Including them in the callsite CFG would be a
  // significant undertaking, so for now, only handle nullptr literals (and 0)
  // and expressions whose types already include an annotation, which we can
  // handle just from declarations instead of call sites and should handle the
  // majority of cases.
  if (!isSupportedPointerType(ParamDecl.getType().getNonReferenceType()))
    return;
  if (!ParamDecl.hasDefaultArg()) return;
  if (ParamDecl.hasUnparsedDefaultArg() ||
      ParamDecl.hasUninstantiatedDefaultArg()) {
    wrappedEmit(Emit, USRCache, Fn, ParamSlot, Evidence::UNKNOWN_ARGUMENT,
                ParamDecl.getEndLoc());
    return;
  }
  const Expr *DefaultArg = ParamDecl.getDefaultArg();
  CHECK(DefaultArg);

  if (isOrIsConstructedFromNullPointerConstant(DefaultArg,
                                               Fn.getASTContext())) {
    wrappedEmit(Emit, USRCache, Fn, ParamSlot, Evidence::NULLABLE_ARGUMENT,
                DefaultArg->getExprLoc());
  } else {
    auto Nullability = getNullabilityAnnotationsFromType(DefaultArg->getType());
    if (auto K = getArgEvidenceKindFromNullability(
            Nullability.front().concrete(), ParamDecl.getType())) {
      wrappedEmit(Emit, USRCache, Fn, ParamSlot, K, DefaultArg->getExprLoc());
    } else {
      wrappedEmit(Emit, USRCache, Fn, ParamSlot, Evidence::UNKNOWN_ARGUMENT,
                  DefaultArg->getExprLoc());
    }
  }
}

static void collectNonnullAttributeEvidence(
    const clang::FunctionDecl &Fn, unsigned ParamIndex, SourceLocation Loc,
    llvm::function_ref<EvidenceEmitter> Emit, USRCache &USRCache) {
  const ParmVarDecl *ParamDecl = Fn.getParamDecl(ParamIndex);
  // The attribute does not apply to references-to-pointers or nested pointers
  // or smart pointers.
  if (isSupportedRawPointerType(ParamDecl->getType())) {
    wrappedEmit(Emit, USRCache, Fn, paramSlot(ParamIndex),
                Evidence::GCC_NONNULL_ATTRIBUTE, Loc);
  }
}

static void emitWellKnownNullability(const clang::FunctionDecl &Fn,
                                     llvm::function_ref<EvidenceEmitter> Emit,
                                     USRCache &USRCache) {
  if (Fn.isMain() && Fn.getNumParams() > 1) {
    if (const auto *ArgvParam = Fn.getParamDecl(1)) {
      wrappedEmit(Emit, USRCache, Fn, paramSlot(1),
                  Evidence::WELL_KNOWN_NONNULL, ArgvParam->getBeginLoc());
      // When we infer for nested pointers, we can add here that the inner
      // pointer, if the type is declared as char**, is Nullable. We need to
      // check the type though, as in many cases it is defined as a pointer to
      // an array of char instead.
    }
  }
}

void collectEvidenceFromTargetDeclaration(
    const clang::Decl &D, llvm::function_ref<EvidenceEmitter> Emit,
    USRCache &USRCache, const NullabilityPragmas &Pragmas) {
  TypeNullabilityDefaults Defaults(D.getASTContext(), Pragmas);
  if (const auto *Fn = dyn_cast<clang::FunctionDecl>(&D)) {
    if (auto K = evidenceKindFromDeclaredReturnType(*Fn, Defaults))
      wrappedEmit(Emit, USRCache, *Fn, SLOT_RETURN_TYPE, *K,
                  Fn->getReturnTypeSourceRange().getBegin());
    emitWellKnownNullability(*Fn, Emit, USRCache);

    if (const auto *RNNA = Fn->getAttr<ReturnsNonNullAttr>()) {
      // The attribute does not apply to references-to-pointers or nested
      // pointers or smart pointers.
      if (isSupportedRawPointerType(Fn->getReturnType())) {
        wrappedEmit(Emit, USRCache, *Fn, SLOT_RETURN_TYPE,
                    Evidence::GCC_NONNULL_ATTRIBUTE, RNNA->getLocation());
      }
    }

    for (unsigned I = 0; I < Fn->param_size(); ++I) {
      const ParmVarDecl *ParamDecl = Fn->getParamDecl(I);
      const TypeSourceInfo *TSI = ParamDecl->getTypeSourceInfo();
      if (!TSI) continue;
      if (auto K =
              evidenceKindFromDeclaredTypeLoc(TSI->getTypeLoc(), Defaults)) {
        wrappedEmit(Emit, USRCache, *Fn, paramSlot(I), *K,
                    ParamDecl->getTypeSpecStartLoc());
      }

      collectEvidenceFromDefaultArgument(*Fn, *ParamDecl, paramSlot(I), Emit,
                                         USRCache);

      if (const auto *NNA = ParamDecl->getAttr<NonNullAttr>())
        collectNonnullAttributeEvidence(*Fn, I, NNA->getLocation(), Emit,
                                        USRCache);
    }

    if (const auto *NNA = Fn->getAttr<NonNullAttr>()) {
      // The attribute may have arguments indicating one or more parameters
      // that are nonnull. If no arguments are present, all top-level,
      // non-reference, raw pointer parameter types are nonnull. Return types
      // are not affected.
      if (NNA->args_size() > 0) {
        for (const clang::ParamIdx &P : NNA->args()) {
          // getASTIndex starts with 0 and does not count any implicit `this`
          // parameter, matching FunctionDecl::getParamDecl indexing.
          unsigned I = P.getASTIndex();
          collectNonnullAttributeEvidence(*Fn, I, NNA->getLocation(), Emit,
                                          USRCache);
        }
      } else {
        for (unsigned I = 0; I < Fn->param_size(); ++I) {
          collectNonnullAttributeEvidence(*Fn, I, NNA->getLocation(), Emit,
                                          USRCache);
        }
      }
    }
  } else if (const auto *Field = dyn_cast<clang::FieldDecl>(&D)) {
    const TypeSourceInfo *TSI = Field->getTypeSourceInfo();
    if (!TSI) return;
    if (auto K = evidenceKindFromDeclaredTypeLoc(TSI->getTypeLoc(), Defaults)) {
      wrappedEmit(Emit, USRCache, *Field, Slot(0), *K,
                  Field->getTypeSpecStartLoc());
    }
  } else if (const auto *Var = dyn_cast<clang::VarDecl>(&D)) {
    const TypeSourceInfo *TSI = Var->getTypeSourceInfo();
    if (!TSI) return;
    if (auto K = evidenceKindFromDeclaredTypeLoc(TSI->getTypeLoc(), Defaults)) {
      wrappedEmit(Emit, USRCache, *Var, Slot(0), *K,
                  Var->getTypeSpecStartLoc());
    }
  }
}

void EvidenceSites::forDefinitionsAndForDeclarations(
    ForEach ForDefinitions, ForEach ForDeclarations, ASTContext &Ctx,
    bool RestrictToMainFileOrHeader) {
  struct Walker : public EvidenceLocationsWalker<Walker> {
    Walker(ForEach ForDefinitions, ForEach ForDeclarations,
           std::unique_ptr<LocFilter> LocFilter)
        : ForDefinitions(ForDefinitions),
          ForDeclarations(ForDeclarations),
          LocFilter(std::move(LocFilter)) {}

    ForEach ForDefinitions;
    ForEach ForDeclarations;
    std::unique_ptr<LocFilter> LocFilter;

    bool VisitFunctionDecl(const FunctionDecl *absl_nonnull FD) {
      if (!LocFilter->check(FD->getBeginLoc())) {
        return true;
      }
      if (isInferenceTarget(*FD)) ForDeclarations(*FD);

      // Visiting template instantiations is fine, these are valid functions!
      // But we'll be limited in what we can infer.
      bool IsUsefulDefinition =
          FD->doesThisDeclarationHaveABody() &&
          // We will not get anywhere with dependent code.
          !FD->isDependentContext() &&
          // Defaulted (aka implicitly-defined) default constructors give us a
          // chance to analyze default member initializers more thoroughly, but
          // otherwise implicit functions are not generally useful.
          (!FD->isImplicit() ||
           (isa<CXXConstructorDecl>(FD) &&
            cast<CXXConstructorDecl>(FD)->isDefaultConstructor()));
      if (IsUsefulDefinition) ForDefinitions(*FD);

      return true;
    }

    bool VisitFieldDecl(const FieldDecl *absl_nonnull FD) {
      if (!LocFilter->check(FD->getBeginLoc())) {
        return true;
      }
      if (isInferenceTarget(*FD)) ForDeclarations(*FD);
      return true;
    }

    bool VisitVarDecl(const VarDecl *absl_nonnull VD) {
      if (!LocFilter->check(VD->getBeginLoc())) {
        return true;
      }
      if (isInferenceTarget(*VD)) {
        ForDeclarations(*VD);
      }
      // Variable initializers outside of function bodies may contain evidence
      // we won't otherwise see, even if the variable is not an inference
      // target.
      if (VD->hasInit() && !VD->isTemplated() &&
          (!VD->getDeclContext()->isFunctionOrMethod() || VD->isInitCapture()))
        ForDefinitions(*VD);
      return true;
    }
  };

  Walker W(
      ForDefinitions, ForDeclarations,
      getLocFilter(Ctx.getSourceManager(),
                   RestrictToMainFileOrHeader ? LocFilterKind::kMainFileOrHeader
                                              : LocFilterKind::kAllowAll));
  W.TraverseAST(Ctx);
}

EvidenceSites EvidenceSites::discover(ASTContext &Ctx,
                                      bool RestrictToMainFileOrHeader) {
  EvidenceSites Out;
  forDefinitionsAndForDeclarations(
      [&Out](const Decl &D) { Out.Definitions.insert(&D); },
      [&Out](const Decl &D) { Out.Declarations.insert(&D); }, Ctx,
      RestrictToMainFileOrHeader);
  return Out;
}

}  // namespace clang::tidy::nullability
