// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/type_transferer.h"

#include <cassert>
#include <deque>
#include <functional>
#include <optional>

#include "absl/base/nullability.h"
#include "absl/log/check.h"
#include "nullability/pointer_nullability.h"
#include "nullability/pointer_nullability_lattice.h"
#include "nullability/type_nullability.h"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclBase.h"
#include "clang/AST/DeclCXX.h"
#include "clang/AST/DeclTemplate.h"
#include "clang/AST/Expr.h"
#include "clang/AST/ExprCXX.h"
#include "clang/AST/NestedNameSpecifierBase.h"
#include "clang/AST/OperationKinds.h"
#include "clang/AST/TemplateBase.h"
#include "clang/AST/TypeBase.h"
#include "clang/ASTMatchers/ASTMatchFinder.h"
#include "clang/ASTMatchers/ASTMatchers.h"
#include "clang/Analysis/FlowSensitive/CFGMatchSwitch.h"
#include "clang/Analysis/FlowSensitive/MatchSwitch.h"
#include "clang/Basic/Builtins.h"
#include "clang/Basic/LLVM.h"
#include "clang/Basic/Specifiers.h"
#include "llvm/ADT/SmallVector.h"
#include "llvm/Support/Casting.h"
#include "llvm/Support/Debug.h"
#include "llvm/Support/raw_ostream.h"

namespace clang::tidy::nullability {
using ast_matchers::anyOf;
using ast_matchers::MatchFinder;
using dataflow::TransferState;

#define DEBUG_TYPE "type_transferer.cc"

static TypeNullability prepend(NullabilityKind Head,
                               const TypeNullability& Tail) {
  TypeNullability Result = {Head};
  Result.insert(Result.end(), Tail.begin(), Tail.end());
  return Result;
}

static void computeNullability(const Expr* absl_nonnull E,
                               TransferState<PointerNullabilityLattice>& State,
                               std::function<TypeNullability()> Compute) {
  (void)State.Lattice.insertExprNullabilityIfAbsent(E, [&] {
    auto Nullability = Compute();
    if (unsigned ExpectedSize = countPointersInType(E);
        ExpectedSize != Nullability.size()) {
      // A nullability vector must have one entry per pointer in the type.
      // If this is violated, we probably failed to handle some AST node.
      LLVM_DEBUG({
        llvm::dbgs()
            << "=== Nullability vector has wrong number of entries: ===\n";
        llvm::dbgs() << "Expression: \n";
        dump(E, llvm::dbgs());
        llvm::dbgs() << "\nNullability (" << Nullability.size()
                     << " pointers): " << nullabilityToString(Nullability)
                     << "\n";
        llvm::dbgs() << "\nType (" << ExpectedSize << " pointers): \n";
        dump(exprType(E), llvm::dbgs());
        llvm::dbgs() << "=================================\n";
      });

      // We can't meaningfully interpret the vector, so discard it.
      // TODO: fix all broken cases and upgrade to CHECK or DCHECK or so.
      Nullability.assign(ExpectedSize, NullabilityKind::Unspecified);
    }
    return Nullability;
  });
}

// Returns the computed nullability for a subexpr of the current expression.
// This is always available as we compute bottom-up.
static const TypeNullability& getNullabilityForChild(
    const Expr* absl_nonnull E,
    TransferState<PointerNullabilityLattice>& State) {
  return State.Lattice.insertExprNullabilityIfAbsent(E, [&] {
    // Since we process child nodes before parents, we should already have
    // computed the child nullability. However, this is not true in all test
    // cases. So, we return unspecified nullability annotations.
    // TODO: fix this issue, and CHECK() instead.
    LLVM_DEBUG({
      llvm::dbgs() << "=== Missing child nullability: ===\n";
      dump(E, llvm::dbgs());
      llvm::dbgs() << "==================================\n";
    });

    return unspecifiedNullability(E);
  });
}

static const Decl* absl_nullable getAssociatedTemplateDecl(
    const SubstTemplateTypeParmType* ST) {
  const Decl* AssociatedDecl = ST->getAssociatedDecl();
  if (!AssociatedDecl) return nullptr;
  if (isa<RedeclarableTemplateDecl>(AssociatedDecl)) return AssociatedDecl;
  if (auto* VTSD = dyn_cast<VarTemplateSpecializationDecl>(AssociatedDecl))
    return VTSD->getSpecializedTemplate();
  if (auto* CTSD = dyn_cast<ClassTemplateSpecializationDecl>(AssociatedDecl))
    return CTSD->getSpecializedTemplate();
  if (auto* FD = dyn_cast<FunctionDecl>(AssociatedDecl);
      FD && FD->isTemplateInstantiation())
    return FD->getPrimaryTemplate();
  return nullptr;
}

namespace {
// The Resugarer describes the nullability of template arguments within types we
// query using getTypeNullability().
//
// When the template arguments are bound within the queried type, e.g.
//   getTypeNullability( vector<int* _Nonnull>::value_type )
// then getTypeNullability() will record the sugar and resolve the
// SubstTemplateTypeParmType within `value_type` itself.
//
// However when the template arguments are bound elsewhere in the code, e.g.
//   vector<int* _Nonnull> a;
//   getTypeNullability( a.front() )
// then we must provide the nullability vector, via the callback passed
// to getTypeNullability().
//
// This class implements that callback interface, based on the common patterns
// where template arguments can be determined from surrounding code.
struct Resugarer {
  using SubstTy = SubstTemplateTypeParmType;
  const TypeNullabilityDefaults& Defaults;

  Resugarer(const TypeNullabilityDefaults& Defaults) : Defaults(Defaults) {}

  // The entity referenced is nested within a class template, e.g. `a.front()`
  // where a is a vector<int* _Nonnull>.
  // We have a nullability vector [Nonnull] for the specialization vector<int*>.
  struct FromEnclosingClassNullability {
    ClassTemplateSpecializationDecl* Specialization;
    const ArrayRef<PointerTypeNullability> SpecializationNullability;

    std::optional<TypeNullability> operator()(const SubstTy* ST) const {
      if (Specialization != ST->getAssociatedDecl()) return std::nullopt;
      // TODO: The code below does not deal correctly with partial
      // specializations. We should eventually handle these, but for now, just
      // bail out.
      if (isa<ClassTemplatePartialSpecializationDecl>(
              ST->getReplacedParameter()->getDeclContext()))
        return std::nullopt;

      unsigned ArgIndex = ST->getIndex();
      auto TemplateArgs = Specialization->getTemplateArgs().asArray();

      // TODO: If the type was substituted from a pack template argument,
      // we must find the slice that pertains to this particular type.
      // For now, just give up on resugaring this type.
      if (ST->getPackIndex()) return std::nullopt;

      unsigned PointerCount =
          countPointersInType(Specialization->getDeclContext());
      for (auto TA : TemplateArgs.take_front(ArgIndex)) {
        PointerCount += countPointersInType(TA);
      }
      unsigned SliceSize = countPointersInType(TemplateArgs[ArgIndex]);
      return SpecializationNullability.slice(PointerCount, SliceSize).vec();
    }
  };
  llvm::SmallVector<FromEnclosingClassNullability> Enclosing;

  // The entity is referenced using template arguments, e.g.
  // `make_unique<int* _Nonnull>`. We have the template arguments.
  struct FromTemplateArgs {
    TemplateDecl* Template;
    ArrayRef<TemplateArgumentLoc> Args;

    std::optional<TypeNullability> operator()(
        const SubstTy* ST, const TypeNullabilityDefaults& Defaults) const {
      if (Template != getAssociatedTemplateDecl(ST)) return std::nullopt;
      // Some or all of the template arguments may be deduced, and we won't
      // see those on the `DeclRefExpr`. If the template argument was deduced,
      // we don't have any sugar for it.
      // TODO(b/268348533): Can we somehow obtain it from e.g. the function
      // param it was deduced from?
      // TODO(b/268345783): This check, as well as the index into
      // `template_arguments` below, may be incorrect in the presence of
      // parameters packs.  In function templates, parameter packs may appear
      // anywhere in the parameter list. The index may therefore refer to one
      // of the pack arguments, but we might incorrectly interpret it as
      // referring to an argument that follows the pack.
      if (ST->getIndex() >= Args.size()) return std::nullopt;

      TypeSourceInfo* TSI = Args[ST->getIndex()].getTypeSourceInfo();
      if (TSI == nullptr) return std::nullopt;
      return getTypeNullability(TSI->getTypeLoc(), Defaults);
    }
  };
  llvm::SmallVector<FromTemplateArgs> Template;

  // Add a FromTemplateArgs context reflecting that the specialization
  // `ResolvedTo` was chosen using the provided template arguments.
  void addTemplateArgs(const ValueDecl* ResolvedTo,
                       ArrayRef<TemplateArgumentLoc> UsingArgs) {
    if (const auto* VD = llvm::dyn_cast<VarDecl>(ResolvedTo)) {
      Template.push_back(
          {cast<VarTemplateSpecializationDecl>(VD)->getSpecializedTemplate(),
           UsingArgs});
    } else if (auto* FD = llvm::dyn_cast<FunctionDecl>(ResolvedTo)) {
      // TODO(b/268345783): We don't currently handle template arguments for
      // function templates with template parameter packs correctly when looking
      // up arguments later. For function templates, other template parameters
      // can follow a template parameter pack and we may report incorrect
      // information for those, so we go out of our way here to avoid that and
      // skip resugaring any arguments for function template specializations
      // with template parameter packs followed by other template parameters.
      auto TemplateArgs = FD->getTemplateSpecializationArgs()->asArray();
      bool SeenPack = false;
      for (const auto& TA : TemplateArgs) {
        if (SeenPack) return;
        if (TA.getKind() == TemplateArgument::Pack) SeenPack = true;
      }
      Template.push_back(
          {FD->getTemplateSpecializationInfo()->getTemplate(), UsingArgs});
    }
  }

  // Implement the getTypeNullability() callback interface by searching
  // all our contexts for a match.
  std::optional<TypeNullability> operator()(const SubstTy* ST) const {
    for (const auto& R : Enclosing)
      if (auto Ret = R(ST)) return Ret;
    for (const auto& R : Template)
      if (auto Ret = R(ST, Defaults)) return Ret;
    return std::nullopt;
  }
};
}  // namespace

static void transferDeclRefExpr(
    const DeclRefExpr* absl_nonnull DRE, const MatchFinder::MatchResult& MR,
    TransferState<PointerNullabilityLattice>& State) {
  computeNullability(DRE, State, [&] {
    Resugarer Resugar(State.Lattice.defaults());

    if (DRE->hasExplicitTemplateArgs())
      Resugar.addTemplateArgs(DRE->getDecl(), DRE->template_arguments());
    std::deque<TypeNullability> ScopeNullabilityStorage;
    for (auto NNS = DRE->getQualifierLoc(); NNS;) {
      if (auto* CTSD = llvm::dyn_cast_or_null<ClassTemplateSpecializationDecl>(
              NNS.getNestedNameSpecifier().getAsRecordDecl())) {
        ScopeNullabilityStorage.push_back(
            getTypeNullability(NNS.getAsTypeLoc(), State.Lattice.defaults()));
        Resugar.Enclosing.push_back({CTSD, ScopeNullabilityStorage.back()});
      }
      if (NNS.getNestedNameSpecifier().getKind() ==
          clang::NestedNameSpecifier::Kind::Namespace)
        NNS = NNS.getAsNamespaceAndPrefix().Prefix;
      else if (NNS.getNestedNameSpecifier().getKind() ==
               clang::NestedNameSpecifier::Kind::Type)
        NNS = NNS.getAsTypeLoc().getPrefix();
      else
        NNS = clang::NestedNameSpecifierLoc();
    }

    return State.Lattice.getTypeNullabilityWithOverrides(*DRE->getDecl(),
                                                         Resugar);
  });
}

static void transferMemberExpr(
    const MemberExpr* absl_nonnull ME, const MatchFinder::MatchResult& MR,
    TransferState<PointerNullabilityLattice>& State) {
  computeNullability(ME, State, [&]() {
    auto* Member = ME->getMemberDecl();
    auto BaseType = ME->getBase()->getType();
    auto BaseNullability =
        ArrayRef(getNullabilityForChild(ME->getBase(), State));
    if (ME->isArrow() && BaseType->isPointerType()) {
      BaseType = BaseType->getPointeeType();
      BaseNullability = BaseNullability.drop_front();
    }

    Resugarer Resugar(State.Lattice.defaults());
    if (const auto* RT = BaseType->getAs<RecordType>()) {
      if (auto* CTSpec =
              dyn_cast<ClassTemplateSpecializationDecl>(RT->getDecl())) {
        Resugar.Enclosing.push_back({CTSpec, BaseNullability});
      }
    }
    if (ME->hasExplicitTemplateArgs())
      Resugar.addTemplateArgs(ME->getMemberDecl(), ME->template_arguments());

    return State.Lattice.getTypeNullabilityWithOverrides(*Member, Resugar);
  });
}

static void transferCastExpr(const CastExpr* absl_nonnull CE,
                             const MatchFinder::MatchResult& MR,
                             TransferState<PointerNullabilityLattice>& State) {
  computeNullability(CE, State, [&]() -> TypeNullability {
    // Most casts that can convert ~unrelated types drop nullability in general.
    // As a special case, preserve nullability of outer raw pointer types.
    // For example, int* p; (void*)p; is a BitCast, but preserves nullability.
    // TODO: b/396242014 - Consider applying the target type's nullability
    // annotations for explicit casts rather than preserving the argument's
    // nullability.
    auto PreserveOuterRawPointers = [&](TypeNullability V) {
      auto ArgNullability = getNullabilityForChild(CE->getSubExpr(), State);
      const PointerType* ArgType = dyn_cast<PointerType>(
          CE->getSubExpr()->getType().getCanonicalType().getTypePtr());
      const PointerType* CastType =
          dyn_cast<PointerType>(CE->getType().getCanonicalType().getTypePtr());
      for (int I = 0; ArgType && CastType; ++I) {
        V[I] = ArgNullability[I];
        ArgType = dyn_cast<PointerType>(ArgType->getPointeeType().getTypePtr());
        CastType =
            dyn_cast<PointerType>(CastType->getPointeeType().getTypePtr());
      }
      return V;
    };

    // We can't assume that all nullability information is identical between
    // base and derived types, such as when a derived type has fewer entries
    // because it always supplies the same template argument(s) to the base
    // type.
    //
    // We preserve the nullability of outer raw pointers, but for implicit
    // casts, for the record type with the inheritance relationship, we attempt
    // some resugaring of the result type nullability from the argument's
    // template arguments.
    auto PreserveAndResugarFromBaseOrDerived = [&](TypeNullability V) {
      V = PreserveOuterRawPointers(V);
      if (auto* ECE = dyn_cast<ExplicitCastExpr>(CE)) {
        // TODO: b/396242014 - Apply the nullability annotations from the target
        // type rather than preserving the argument's nullability. Do this both
        // for outer pointers and for template arguments.
        return V;
      }
      if (auto* ICE = dyn_cast<ImplicitCastExpr>(CE);
          ICE && ICE->isPartOfExplicitCast()) {
        // Let the nullability be picked up from the explicit cast; no need to
        // do work here.
        return V;
      }
      if (CE->path_empty()) {
        llvm::errs() << "Empty path for cast between base and derived types.\n";
        assert(false);
        return V;
      }

      int NumOuterRawPointers = 0;
      const Type* UnderPointers =
          CE->getSubExpr()->getType().getCanonicalType().getTypePtr();
      while (auto* PT = dyn_cast<PointerType>(UnderPointers)) {
        UnderPointers = PT->getPointeeType().getTypePtr();
        ++NumOuterRawPointers;
      }
      if (NumOuterRawPointers < V.size()) {
        // For the elements of V after the nullability for any outer raw
        // pointers, resugar the result type from the argument's template
        // arguments.
        const TypeNullability& ArgNullability =
            getNullabilityForChild(CE->getSubExpr(), State);
        TypeNullability UnderPointersNullability;
        for (int J = NumOuterRawPointers; J < ArgNullability.size(); ++J) {
          UnderPointersNullability.push_back(ArgNullability[J]);
        }
        Resugarer Resugar(State.Lattice.defaults());
        // Resugar from class template arguments, if any.
        if (const auto* RT = UnderPointers->getAs<RecordType>()) {
          if (auto* CTSpec =
                  dyn_cast<ClassTemplateSpecializationDecl>(RT->getDecl())) {
            Resugar.Enclosing.push_back({CTSpec, UnderPointersNullability});
          }
        }
        auto CastNullability = getTypeNullability(
            (*(CE->path_end() - 1))->getTypeSourceInfo()->getTypeLoc(),
            State.Lattice.defaults(), Resugar);
        if (CastNullability.size() + NumOuterRawPointers != V.size()) {
          llvm::errs()
              << "CastNullability.size() + NumOuterRawPointers != V.size(): "
              << (CastNullability.size() + NumOuterRawPointers) << " vs "
              << V.size() << "\n";
          CE->dump();
          assert(false);
        }
        for (int I = 0;
             I + NumOuterRawPointers < V.size() && I < CastNullability.size();
             ++I) {
          V[I + NumOuterRawPointers] = CastNullability[I];
        }
      }
      return V;
    };

    switch (CE->getCastKind()) {
      // Casts between unrelated types: we can't say anything about nullability.
      case CK_LValueBitCast:
      case CK_BitCast:
      case CK_LValueToRValueBitCast:
        return PreserveOuterRawPointers(unspecifiedNullability(CE));

      // Casts between equivalent types.
      case CK_LValueToRValue:
      case CK_NoOp:
      case CK_AtomicToNonAtomic:
      case CK_NonAtomicToAtomic:
      case CK_AddressSpaceConversion:
        return getNullabilityForChild(CE->getSubExpr(), State);

      // Controlled conversions between types
      case CK_BaseToDerived:
      case CK_DerivedToBase:
      case CK_UncheckedDerivedToBase:
        return PreserveAndResugarFromBaseOrDerived(unspecifiedNullability(CE));
      case CK_UserDefinedConversion:
        return unspecifiedNullability(CE);
      case CK_ConstructorConversion:
        if (auto* CCE = llvm::dyn_cast<CXXConstructExpr>(CE->getSubExpr())) {
          // This node is syntactic only.
          return getNullabilityForChild(CE->getSubExpr(), State);
        }
        return unspecifiedNullability(CE);

      case CK_Dynamic: {
        auto Result = unspecifiedNullability(CE);
        // A dynamic_cast to pointer is null if the runtime check fails.
        if (isa<PointerType>(CE->getType().getCanonicalType()))
          Result.front() = NullabilityKind::Nullable;
        return Result;
      }

      // Primitive values have no nullability.
      case CK_ToVoid:
      case CK_MemberPointerToBoolean:
      case CK_PointerToBoolean:
      case CK_PointerToIntegral:
      case CK_IntegralCast:
      case CK_IntegralToBoolean:
      case CK_IntegralToFloating:
      case CK_FloatingToFixedPoint:
      case CK_FixedPointToFloating:
      case CK_FixedPointCast:
      case CK_FixedPointToIntegral:
      case CK_IntegralToFixedPoint:
      case CK_FixedPointToBoolean:
      case CK_FloatingToIntegral:
      case CK_FloatingToBoolean:
      case CK_BooleanToSignedIntegral:
      case CK_FloatingCast:
      case CK_FloatingRealToComplex:
      case CK_FloatingComplexToReal:
      case CK_FloatingComplexToBoolean:
      case CK_FloatingComplexCast:
      case CK_FloatingComplexToIntegralComplex:
      case CK_IntegralRealToComplex:
      case CK_IntegralComplexToReal:
      case CK_IntegralComplexToBoolean:
      case CK_IntegralComplexCast:
      case CK_IntegralComplexToFloatingComplex:
        return {};

      // This can definitely be null!
      case CK_NullToPointer: {
        TypeNullability Nullability;
        // Explicit casts get the inner of the written type.
        if (const auto* ECE = dyn_cast<ExplicitCastExpr>(CE))
          Nullability =
              getTypeNullability(ECE->getTypeInfoAsWritten()->getTypeLoc(),
                                 State.Lattice.defaults());
        else
          Nullability = unspecifiedNullability(CE);
        // Despite the name `NullToPointer`, the destination type of the cast
        // may be `nullptr_t` (which is, itself, not a pointer type).
        if (!CE->getType()->isNullPtrType())
          Nullability.front() = NullabilityKind::Nullable;
        return Nullability;
      }

      // Pointers out of thin air, who knows?
      case CK_IntegralToPointer:
        return unspecifiedNullability(CE);

      // Decayed objects are never null.
      case CK_ArrayToPointerDecay:
      case CK_FunctionToPointerDecay:
        return prepend(NullabilityKind::NonNull,
                       getNullabilityForChild(CE->getSubExpr(), State));

      // Despite its name, the result type of `BuiltinFnToFnPtr` is a function,
      // not a function pointer, so nullability doesn't change.
      case CK_BuiltinFnToFnPtr:
        return getNullabilityForChild(CE->getSubExpr(), State);

      // TODO: what is our model of member pointers?
      case CK_BaseToDerivedMemberPointer:
      case CK_DerivedToBaseMemberPointer:
      case CK_NullToMemberPointer:
      case CK_ReinterpretMemberPointer:
      case CK_ToUnion:  // and unions?
        return unspecifiedNullability(CE);

      // TODO: Non-C/C++ constructs, do we care about these?
      case CK_CPointerToObjCPointerCast:
      case CK_ObjCObjectLValueCast:
      case CK_MatrixCast:
      case CK_VectorSplat:
      case CK_BlockPointerToObjCPointerCast:
      case CK_AnyPointerToBlockPointerCast:
      case CK_ARCProduceObject:
      case CK_ARCConsumeObject:
      case CK_ARCReclaimReturnedObject:
      case CK_ARCExtendBlockObject:
      case CK_CopyAndAutoreleaseBlockObject:
      case CK_ZeroToOCLOpaqueType:
      case CK_IntToOCLSampler:
      case CK_HLSLVectorTruncation:
      case CK_HLSLArrayRValue:
      case CK_HLSLAggregateSplatCast:
      case CK_HLSLElementwiseCast:
      case CK_HLSLMatrixTruncation:
        return unspecifiedNullability(CE);

      case CK_Dependent:
        CHECK(false) << "Shouldn't see dependent casts here?";
    }
  });
}

static void transferMaterializeTemporaryExpr(
    const MaterializeTemporaryExpr* absl_nonnull MTE,
    const MatchFinder::MatchResult& MR,
    TransferState<PointerNullabilityLattice>& State) {
  computeNullability(MTE, State, [&]() {
    return getNullabilityForChild(MTE->getSubExpr(), State);
  });
}

static void transferCXXBindTemporaryExpr(
    const CXXBindTemporaryExpr* BTE, const MatchFinder::MatchResult& MR,
    TransferState<PointerNullabilityLattice>& State) {
  computeNullability(BTE, State, [&]() {
    return getNullabilityForChild(BTE->getSubExpr(), State);
  });
}

static void transferCopyOrMoveConstruct(
    const CXXConstructExpr* CCE, const MatchFinder::MatchResult& MR,
    TransferState<PointerNullabilityLattice>& State) {
  computeNullability(CCE, State, [&]() {
    return getNullabilityForChild(CCE->getArg(0), State);
  });
}

static TypeNullability computeTypeNullabilityForCallExpr(
    const CallExpr* absl_nonnull CE,
    TransferState<PointerNullabilityLattice>& State) {
  TypeNullability CalleeNullability =
      getNullabilityForChild(CE->getCallee(), State);
  ArrayRef ResultNullability = CalleeNullability;
  if (CE->getCallee()->getType()->isPointerType())  // Callee is usually fptr.
    ResultNullability = ResultNullability.drop_front();
  // Return value nullability is at the front of the function type.
  ResultNullability =
      ResultNullability.take_front(countPointersInType(CE->getType()));
  return ResultNullability.vec();
}

static void transferCallExpr(const CallExpr* absl_nonnull CE,
                             const MatchFinder::MatchResult& MR,
                             TransferState<PointerNullabilityLattice>& State) {
  computeNullability(CE, State, [&]() {
    if (auto ID = CE->getBuiltinCallee();
        (ID == Builtin::BIforward || ID == Builtin::BImove) &&
        CE->getNumArgs() == 1) {
      return getNullabilityForChild(CE->getArg(0), State);
    }

    return computeTypeNullabilityForCallExpr(CE, State);
  });
}

static void transferCXXOperatorCallExpr(
    const CXXOperatorCallExpr* absl_nonnull CE,
    const MatchFinder::MatchResult& MR,
    TransferState<PointerNullabilityLattice>& State) {
  computeNullability(CE, State, [&]() {
    // If this is a method call, see if it is a template specialization
    // and whether resugaring with the Base (arg 0)'s nullability helps
    // refine the return type nullability, similar to transferMemberExpr.
    // This only helps refine the return type nullability, not callee's
    // nullability including the params. TODO(b/405355053): see if we can refine
    // the params too.
    if (auto* Callee = dyn_cast<CXXMethodDecl>(CE->getCalleeDecl())) {
      const auto* Base = CE->getArg(0);
      TypeNullability BaseNullability = getNullabilityForChild(Base, State);
      Resugarer Resugar(State.Lattice.defaults());
      if (const auto* RT = Base->getType()->getAs<RecordType>()) {
        if (auto* CTSpec =
                dyn_cast<ClassTemplateSpecializationDecl>(RT->getDecl())) {
          Resugar.Enclosing.push_back({CTSpec, BaseNullability});
        }
      }

      TypeNullability Nullability =
          State.Lattice.getTypeNullabilityWithOverrides(*Callee, Resugar);
      ArrayRef ResultNullability = Nullability;
      // Return value nullability is at the front of the function type.
      ResultNullability =
          ResultNullability.take_front(countPointersInType(CE->getType()));
      return ResultNullability.vec();
    }

    // Not a member operator call.
    return computeTypeNullabilityForCallExpr(CE, State);
  });
}

static void transferUnaryOperator(
    const UnaryOperator* absl_nonnull UO, const MatchFinder::MatchResult& MR,
    TransferState<PointerNullabilityLattice>& State) {
  computeNullability(UO, State, [&]() -> TypeNullability {
    switch (UO->getOpcode()) {
      case UO_AddrOf:
        return prepend(NullabilityKind::NonNull,
                       getNullabilityForChild(UO->getSubExpr(), State));
      case UO_Deref:
        return ArrayRef(getNullabilityForChild(UO->getSubExpr(), State))
            .drop_front()
            .vec();

      case UO_PreInc:
      case UO_PreDec: {
        TypeNullability SubNullability =
            getNullabilityForChild(UO->getSubExpr(), State);
        if (!isSupportedRawPointerType(UO->getSubExpr()->getType()))
          return SubNullability;
        assert(!SubNullability.empty());
        SubNullability[0] = NullabilityKind::NonNull;
        return SubNullability;
      }

      case UO_PostInc:
      case UO_PostDec:
      case UO_Plus:
      case UO_Minus:
      case UO_Not:
      case UO_LNot:
      case UO_Real:
      case UO_Imag:
      case UO_Extension:
        return getNullabilityForChild(UO->getSubExpr(), State);

      case UO_Coawait:
        // TODO: work out what to do here!
        return unspecifiedNullability(UO);
    }
  });
}

static void transferBinaryOperator(
    const BinaryOperator* absl_nonnull BO, const MatchFinder::MatchResult& MR,
    TransferState<PointerNullabilityLattice>& State) {
  computeNullability(BO, State, [&]() -> TypeNullability {
    switch (BO->getOpcode()) {
      case BO_PtrMemD:
      case BO_PtrMemI:
        // TODO: pointers-to-member should really have nullability vectors
        return unspecifiedNullability(BO);
      case BO_Assign:
      case BO_Comma:
        return getNullabilityForChild(BO->getRHS(), State);
      case BO_Add:
      case BO_Sub:
      // The `+=` and `-=` operators will always take the "LHS" branch below but
      // can otherwise be handled using the same code as `+` and `-`, so we do.
      case BO_AddAssign:
      case BO_SubAssign: {
        bool LhsIsPointer = isSupportedRawPointerType(BO->getLHS()->getType());
        bool RhsIsPointer = isSupportedRawPointerType(BO->getRHS()->getType());
        // Pointer difference.
        if (LhsIsPointer && RhsIsPointer) {
          assert(BO->getOpcode() == BO_Sub);
          assert(BO->getType()->isIntegerType());
          return {};
        }
        TypeNullability PtrNullability;
        if (LhsIsPointer)
          PtrNullability = getNullabilityForChild(BO->getLHS(), State);
        else if (RhsIsPointer)
          PtrNullability = getNullabilityForChild(BO->getRHS(), State);
        else
          return unspecifiedNullability(BO);
        assert(!PtrNullability.empty());
        PtrNullability[0] = NullabilityKind::NonNull;
        return PtrNullability;
      }
      default:
        // No other built-in binary operators can be pointer-valued
        return unspecifiedNullability(BO);
    }
  });
}

static void transferNewExpr(const CXXNewExpr* absl_nonnull NE,
                            const MatchFinder::MatchResult& MR,
                            TransferState<PointerNullabilityLattice>& State) {
  computeNullability(NE, State, [&]() {
    TypeNullability ObjectNullability =
        getTypeNullability(NE->getAllocatedTypeSourceInfo()->getTypeLoc(),
                           State.Lattice.defaults());
    return prepend(NE->shouldNullCheckAllocation() ? NullabilityKind::Nullable
                                                   : NullabilityKind::NonNull,
                   ObjectNullability);
  });
}

static void transferArraySubscriptExpr(
    const ArraySubscriptExpr* absl_nonnull ASE,
    const MatchFinder::MatchResult& MR,
    TransferState<PointerNullabilityLattice>& State) {
  computeNullability(ASE, State, [&]() {
    auto& BaseNullability = getNullabilityForChild(ASE->getBase(), State);
    QualType BaseType = ASE->getBase()->getType();
    CHECK(isSupportedRawPointerType(BaseType) || BaseType->isVectorType());
    return isSupportedRawPointerType(BaseType)
               ? ArrayRef(BaseNullability).slice(1).vec()
               : BaseNullability;
  });
}

static void transferThisExpr(const CXXThisExpr* absl_nonnull TE,
                             const MatchFinder::MatchResult& MR,
                             TransferState<PointerNullabilityLattice>& State) {
  computeNullability(TE, State, [&]() {
    // If the current class is an instantiation, we can't assume any particular
    // nullability of its arguments.
    TypeNullability Result = unspecifiedNullability(TE);
    Result.front() = NullabilityKind::NonNull;
    return Result;
  });
}

dataflow::CFGMatchSwitch<dataflow::TransferState<PointerNullabilityLattice>>
buildTypeTransferer() {
  return dataflow::CFGMatchSwitchBuilder<
             TransferState<PointerNullabilityLattice>>()
      .CaseOfCFGStmt<DeclRefExpr>(ast_matchers::declRefExpr(),
                                  transferDeclRefExpr)
      .CaseOfCFGStmt<MemberExpr>(ast_matchers::memberExpr(), transferMemberExpr)
      .CaseOfCFGStmt<CastExpr>(ast_matchers::castExpr(), transferCastExpr)
      .CaseOfCFGStmt<MaterializeTemporaryExpr>(
          ast_matchers::materializeTemporaryExpr(),
          transferMaterializeTemporaryExpr)
      .CaseOfCFGStmt<CXXBindTemporaryExpr>(ast_matchers::cxxBindTemporaryExpr(),
                                           transferCXXBindTemporaryExpr)
      .CaseOfCFGStmt<CXXOperatorCallExpr>(ast_matchers::cxxOperatorCallExpr(),
                                          transferCXXOperatorCallExpr)
      .CaseOfCFGStmt<CallExpr>(ast_matchers::callExpr(), transferCallExpr)
      .CaseOfCFGStmt<UnaryOperator>(ast_matchers::unaryOperator(),
                                    transferUnaryOperator)
      .CaseOfCFGStmt<BinaryOperator>(ast_matchers::binaryOperator(),
                                     transferBinaryOperator)
      .CaseOfCFGStmt<CXXNewExpr>(ast_matchers::cxxNewExpr(), transferNewExpr)
      .CaseOfCFGStmt<ArraySubscriptExpr>(ast_matchers::arraySubscriptExpr(),
                                         transferArraySubscriptExpr)
      .CaseOfCFGStmt<CXXThisExpr>(ast_matchers::cxxThisExpr(), transferThisExpr)
      .CaseOfCFGStmt<CXXConstructExpr>(
          ast_matchers::cxxConstructExpr(
              ast_matchers::argumentCountIs(1),
              ast_matchers::hasDeclaration(ast_matchers::cxxConstructorDecl(
                  anyOf(ast_matchers::isCopyConstructor(),
                        ast_matchers::isMoveConstructor())))),
          transferCopyOrMoveConstruct)
      .Build();
}
}  // namespace clang::tidy::nullability
