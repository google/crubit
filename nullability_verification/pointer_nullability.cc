// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability_verification/pointer_nullability.h"

#include "absl/log/check.h"
#include "nullability_verification/pointer_nullability_lattice.h"
#include "clang/AST/ASTDumper.h"
#include "clang/AST/TypeVisitor.h"
#include "clang/Analysis/FlowSensitive/CFGMatchSwitch.h"
#include "clang/Analysis/FlowSensitive/DataflowEnvironment.h"
#include "clang/Analysis/FlowSensitive/Value.h"
#include "llvm/ADT/StringRef.h"
#include "llvm/Support/SaveAndRestore.h"

namespace clang {
namespace tidy {
namespace nullability {

using dataflow::AtomicBoolValue;
using dataflow::BoolValue;
using dataflow::Environment;
using dataflow::PointerValue;
using dataflow::SkipPast;
using dataflow::TransferState;

/// The nullness information of a pointer is represented by two properties
/// which indicate if a pointer's nullability (i.e., if the pointer can hold
/// null) is `Known` and if the pointer's value is `Null`.
constexpr llvm::StringLiteral kKnown = "is_known";
constexpr llvm::StringLiteral kNull = "is_null";

NullabilityKind getNullabilityKind(QualType Type, ASTContext& Ctx) {
  return Type->getNullability().value_or(NullabilityKind::Unspecified);
}

PointerValue* getPointerValueFromExpr(const Expr* PointerExpr,
                                      const Environment& Env) {
  return cast_or_null<PointerValue>(
      Env.getValue(*PointerExpr, SkipPast::Reference));
}

std::pair<AtomicBoolValue&, AtomicBoolValue&> getPointerNullState(
    const PointerValue& PointerVal, const Environment& Env) {
  auto& PointerKnown = *cast<AtomicBoolValue>(PointerVal.getProperty(kKnown));
  auto& PointerNull = *cast<AtomicBoolValue>(PointerVal.getProperty(kNull));
  return {PointerKnown, PointerNull};
}

void initPointerBoolProperty(PointerValue& PointerVal, llvm::StringRef Name,
                             BoolValue* BoolVal, Environment& Env) {
  if (PointerVal.getProperty(Name) == nullptr) {
    PointerVal.setProperty(Name,
                           BoolVal ? *BoolVal : Env.makeAtomicBoolValue());
  }
}

void initPointerNullState(PointerValue& PointerVal, Environment& Env,
                          BoolValue* KnownConstraint,
                          BoolValue* NullConstraint) {
  initPointerBoolProperty(PointerVal, kKnown, KnownConstraint, Env);
  initPointerBoolProperty(PointerVal, kNull, NullConstraint, Env);
}

bool isNullable(const PointerValue& PointerVal, const Environment& Env) {
  auto [PointerKnown, PointerNull] = getPointerNullState(PointerVal, Env);
  auto& PointerNotKnownNull =
      Env.makeNot(Env.makeAnd(PointerKnown, PointerNull));
  return !Env.flowConditionImplies(PointerNotKnownNull);
}

std::string nullabilityToString(ArrayRef<NullabilityKind> Nullability) {
  std::string Result = "[";
  llvm::interleave(
      Nullability,
      [&](const NullabilityKind n) {
        Result += getNullabilitySpelling(n).str();
      },
      [&] { Result += ", "; });
  Result += "]";
  return Result;
}

namespace {
// Traverses a Type to find the points where it might be nullable.
// This will visit the contained PointerType in the correct order to produce
// the TypeNullability vector.
//
// Subclasses must provide `void report(const PointerType*, NullabilityKind)`,
// and may override TypeVisitor Visit*Type methods to customize the traversal.
//
// Canonically-equivalent Types produce equivalent sequences of report() calls:
//  - corresponding PointerTypes are canonically-equivalent
//  - the NullabilityKind may be different, as it derives from type sugar
template <class Impl>
class NullabilityWalker : public TypeVisitor<Impl> {
  using Base = TypeVisitor<Impl>;
  Impl& derived() { return *static_cast<Impl*>(this); }

  // A nullability attribute we've seen, waiting to attach to a pointer type.
  // There may be sugar in between: Attributed -> Typedef -> Typedef -> Pointer.
  // All non-sugar types must consume nullability, most will ignore it.
  std::optional<NullabilityKind> PendingNullability;

  void ignoreUnexpectedNullability() { PendingNullability.reset(); }

  // While walking the underlying type of alias TemplateSpecializationTypes,
  // we see SubstTemplateTypeParmTypes where type parameters were referenced.
  // The directly-available underlying types lack sugar, but we can retrieve the
  // sugar from the arguments of the original TemplateSpecializationType.
  //
  // It is only possible to reference params of the immediately enclosing alias,
  // so we keep details of the alias specialization we're currently processing.
  struct AliasArgs {
    const Decl* AssociatedDecl;
    ArrayRef<TemplateArgument> Args;
    // The alias context in which the alias specialization itself appeared.
    // (The alias's args may reference params from this context.)
    const AliasArgs* Parent;
  };
  const AliasArgs* CurrentAliasTemplate = nullptr;

 public:
  void Visit(QualType T) { Base::Visit(T.getTypePtr()); }
  void Visit(const TemplateArgument& TA) {
    if (TA.getKind() == TemplateArgument::Type) Visit(TA.getAsType());
  }

  void VisitType(const Type* T) {
    // For sugar not explicitly handled below, desugar and continue.
    // (We need to walk the full structure of the canonical type.)
    if (auto* Desugar =
            T->getLocallyUnqualifiedSingleStepDesugaredType().getTypePtr();
        Desugar != T)
      return Base::Visit(Desugar);

    // We don't expect to see any nullable non-sugar types except PointerType.
    ignoreUnexpectedNullability();
    Base::VisitType(T);
  }

  void VisitFunctionProtoType(const FunctionProtoType* FPT) {
    ignoreUnexpectedNullability();
    Visit(FPT->getReturnType());
    for (auto ParamType : FPT->getParamTypes()) Visit(ParamType);
  }

  void VisitTemplateSpecializationType(const TemplateSpecializationType* TST) {
    if (TST->isTypeAlias()) {
      // Aliases are sugar, visit the underlying type.
      // Record template args so we can resugar substituted params.
      const AliasArgs Args{TST->getTemplateName().getAsTemplateDecl(),
                           TST->template_arguments(), CurrentAliasTemplate};
      llvm::SaveAndRestore UseAlias(CurrentAliasTemplate, &Args);
      VisitType(TST);
      return;
    }

    ignoreUnexpectedNullability();
    for (auto TA : TST->template_arguments()) Visit(TA);
  }

  void VisitSubstTemplateTypeParmType(const SubstTemplateTypeParmType* T) {
    if (isa<TypeAliasTemplateDecl>(T->getAssociatedDecl())) {
      if (CurrentAliasTemplate != nullptr) {
        CHECK(T->getAssociatedDecl() == CurrentAliasTemplate->AssociatedDecl);
        unsigned Index = T->getIndex();
        // Valid because pack must be the last param in alias templates.
        if (auto PackIndex = T->getPackIndex()) Index += *PackIndex;
        const TemplateArgument& Arg = CurrentAliasTemplate->Args[Index];

        llvm::SaveAndRestore OriginalContext(CurrentAliasTemplate,
                                             CurrentAliasTemplate->Parent);
        return Visit(Arg);
      } else {
        // Our top-level type references an unbound type alias param.
        // Presumably our original input was the underlying type of an alias
        // instantiation, we now lack the context needed to resugar it.
      }
    }
    VisitType(T);
  }

  void VisitRecordType(const RecordType* RT) {
    ignoreUnexpectedNullability();
    if (auto* CTSD = dyn_cast<ClassTemplateSpecializationDecl>(RT->getDecl())) {
      for (auto& TA : CTSD->getTemplateArgs().asArray()) Visit(TA);
    }
  }

  void VisitAttributedType(const AttributedType* AT) {
    if (auto NK = AT->getImmediateNullability()) {
      // If we see nullability applied twice, the outer one wins.
      if (!PendingNullability.has_value()) PendingNullability = *NK;
    }
    Visit(AT->getModifiedType());
    CHECK(!PendingNullability.has_value())
        << "Should have been consumed by modified type! "
        << AT->getModifiedType().getAsString();
  }

  void VisitPointerType(const PointerType* PT) {
    derived().report(PT,
                     PendingNullability.value_or(NullabilityKind::Unspecified));
    PendingNullability.reset();
    Visit(PT->getPointeeType());
  }
};
}  // namespace

unsigned countPointersInType(QualType T) {
  struct Walker : public NullabilityWalker<Walker> {
    unsigned Count = 0;
    void report(const PointerType*, NullabilityKind) { ++Count; }
  } PointerCountVisitor;
  PointerCountVisitor.Visit(T.getCanonicalType());
  return PointerCountVisitor.Count;
}

unsigned countPointersInType(TemplateArgument TA) {
  if (TA.getKind() == TemplateArgument::Type) {
    return countPointersInType(TA.getAsType().getCanonicalType());
  }
  return 0;
}

QualType exprType(const Expr* E) {
  if (E->hasPlaceholderType(BuiltinType::BoundMember))
    return Expr::findBoundMemberType(E);
  return E->getType();
}

unsigned countPointersInType(const Expr* E) {
  return countPointersInType(exprType(E));
}

std::vector<NullabilityKind> getNullabilityAnnotationsFromType(
    QualType T,
    llvm::function_ref<GetTypeParamNullability> SubstituteTypeParam) {
  struct Walker : NullabilityWalker<Walker> {
    std::vector<NullabilityKind> Annotations;
    llvm::function_ref<GetTypeParamNullability> SubstituteTypeParam;

    void report(const PointerType*, NullabilityKind NK) {
      Annotations.push_back(NK);
    }

    void VisitSubstTemplateTypeParmType(const SubstTemplateTypeParmType* ST) {
      if (SubstituteTypeParam) {
        if (auto Subst = SubstituteTypeParam(ST)) {
          DCHECK_EQ(Subst->size(),
                    countPointersInType(ST->getCanonicalTypeInternal()))
              << "Substituted nullability has the wrong structure: "
              << QualType(ST, 0).getAsString();
          llvm::append_range(Annotations, *Subst);
          return;
        }
      }
      NullabilityWalker::VisitSubstTemplateTypeParmType(ST);
    }
  } AnnotationVisitor;
  AnnotationVisitor.SubstituteTypeParam = SubstituteTypeParam;
  AnnotationVisitor.Visit(T);
  return std::move(AnnotationVisitor.Annotations);
}

std::vector<NullabilityKind> unspecifiedNullability(const Expr* E) {
  return std::vector<NullabilityKind>(countPointersInType(E),
                                      NullabilityKind::Unspecified);
}

ArrayRef<NullabilityKind> getNullabilityForChild(
    const Expr* E, TransferState<PointerNullabilityLattice>& State) {
  return State.Lattice.insertExprNullabilityIfAbsent(E, [&] {
    // Since we process child nodes before parents, we should already have
    // computed the child nullability. However, this is not true in all test
    // cases. So, we return unspecified nullability annotations.
    // TODO: fix this issue, and CHECK() instead.
    llvm::dbgs() << "=== Missing child nullability: ===\n";
    dump(E, llvm::dbgs());
    llvm::dbgs() << "==================================\n";

    return unspecifiedNullability(E);
  });
}

}  // namespace nullability
}  // namespace tidy
}  // namespace clang
