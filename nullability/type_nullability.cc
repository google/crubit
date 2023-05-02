// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/type_nullability.h"

#include "absl/log/check.h"
#include "clang/AST/ASTContext.h"
#include "clang/AST/DeclTemplate.h"
#include "clang/AST/Type.h"
#include "clang/AST/TypeVisitor.h"
#include "clang/Basic/LLVM.h"
#include "clang/Basic/Specifiers.h"
#include "llvm/Support/SaveAndRestore.h"

namespace clang::tidy::nullability {

std::string nullabilityToString(const TypeNullability& Nullability) {
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

  void ignoreUnexpectedNullability() {
    // TODO: Can we upgrade this to an assert?
    // clang is pretty thorough about ensuring we can't put _Nullable on
    // non-pointers, even failing template instantiation on this basis.
    PendingNullability.reset();
  }

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
    if (TA.getKind() == TemplateArgument::Pack)
      for (const auto& PackElt : TA.getPackAsArray()) Visit(PackElt);
  }
  void Visit(const DeclContext* DC) {
    // For now, only consider enclosing classes.
    // TODO: The nullability of template functions can affect local classes too,
    // this can be relevant e.g. when instantiating templates with such types.
    if (auto* CRD = llvm::dyn_cast<CXXRecordDecl>(DC))
      Visit(DC->getParentASTContext().getRecordType(CRD));
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

    auto* CRD = TST->getAsCXXRecordDecl();
    CHECK(CRD) << "Expected an alias or class specialization in concrete code";
    ignoreUnexpectedNullability();
    Visit(CRD->getDeclContext());
    for (auto TA : TST->template_arguments()) Visit(TA);
  }

  void VisitSubstTemplateTypeParmType(const SubstTemplateTypeParmType* T) {
    if (isa<TypeAliasTemplateDecl>(T->getAssociatedDecl())) {
      if (CurrentAliasTemplate != nullptr) {
        CHECK(T->getAssociatedDecl() == CurrentAliasTemplate->AssociatedDecl);
        unsigned Index = T->getIndex();
        // Valid because pack must be the last param in alias templates.
        if (auto PackIndex = T->getPackIndex())
          Index = CurrentAliasTemplate->Args.size() - 1 - *PackIndex;
        const TemplateArgument& Arg = CurrentAliasTemplate->Args[Index];

        llvm::SaveAndRestore OriginalContext(CurrentAliasTemplate,
                                             CurrentAliasTemplate->Parent);
        return Visit(Arg);
      } else {
        // Our top-level type references an unbound type alias param.
        // Presumably our original input was the underlying type of an alias
        // instantiation, we now lack the context needed to resugar it.
        // TODO: maybe this could be an assert? We would need to trust all
        // callers are obtaining types appropriately, and that clang never
        // partially-desugars in a problematic way.
      }
    }
    VisitType(T);
  }

  void VisitRecordType(const RecordType* RT) {
    ignoreUnexpectedNullability();
    Visit(RT->getDecl()->getDeclContext());
    if (auto* CTSD = dyn_cast<ClassTemplateSpecializationDecl>(RT->getDecl())) {
      // TODO: if this is an instantiation, these args lack sugar.
      // We can try to retrieve it from the current template context.
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

  void VisitReferenceType(const ReferenceType* RT) {
    ignoreUnexpectedNullability();
    Visit(RT->getPointeeTypeAsWritten());
  }

  void VisitArrayType(const ArrayType* AT) {
    ignoreUnexpectedNullability();
    Visit(AT->getElementType());
  }
};

template <typename T>
unsigned countPointers(const T& Object) {
  struct Walker : public NullabilityWalker<Walker> {
    unsigned Count = 0;
    void report(const PointerType*, NullabilityKind) { ++Count; }
  } PointerCountWalker;
  PointerCountWalker.Visit(Object);
  return PointerCountWalker.Count;
}

}  // namespace

unsigned countPointersInType(QualType T) { return countPointers(T); }

unsigned countPointersInType(const DeclContext* DC) {
  return countPointers(DC);
}
unsigned countPointersInType(TemplateArgument TA) { return countPointers(TA); }

QualType exprType(const Expr* E) {
  if (E->hasPlaceholderType(BuiltinType::BoundMember))
    return Expr::findBoundMemberType(E);
  return E->getType();
}

unsigned countPointersInType(const Expr* E) {
  return countPointersInType(exprType(E));
}

TypeNullability getNullabilityAnnotationsFromType(
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

TypeNullability unspecifiedNullability(const Expr* E) {
  return TypeNullability(countPointersInType(E), NullabilityKind::Unspecified);
}

namespace {

// Visitor to rebuild a QualType with explicit nullability.
// Extra AttributedType nodes are added wrapping interior PointerTypes, and
// other sugar is added as needed to allow this (e.g. TypeSpecializationType).
//
// We only have to handle types that have nontrivial nullability vectors, i.e.
// those handled by NullabilityWalker.
// Additionally, we only operate on canonical types (otherwise the sugar we're
// adding could conflict with existing sugar).
//
// This needs to stay in sync with the other algorithms that manipulate
// nullability data structures for particular types: the non-flow-sensitive
// transfer and NullabilityWalker.
struct Rebuilder : public TypeVisitor<Rebuilder, QualType> {
  Rebuilder(const TypeNullability& Nullability, ASTContext& Ctx)
      : Nullability(Nullability), Ctx(Ctx) {}

  bool done() const { return Nullability.empty(); }

  using Base = TypeVisitor<Rebuilder, QualType>;
  using Base::Visit;
  QualType Visit(QualType T) {
    if (T.isNull()) return T;
    return Ctx.getQualifiedType(Visit(T.getTypePtr()), T.getLocalQualifiers());
  }
  TemplateArgument Visit(TemplateArgument TA) {
    if (TA.getKind() == TemplateArgument::Type)
      return TemplateArgument(Visit(TA.getAsType()));
    return TA;
  }

  // Default behavior for unhandled types: do not transform.
  QualType VisitType(const Type* T) { return QualType(T, 0); }

  QualType VisitPointerType(const PointerType* PT) {
    CHECK(!Nullability.empty())
        << "Nullability vector too short at " << QualType(PT, 0).getAsString();
    NullabilityKind NK = Nullability.front();
    Nullability = Nullability.drop_front();

    QualType Rebuilt = Ctx.getPointerType(Visit(PT->getPointeeType()));
    if (NK == NullabilityKind::Unspecified) return Rebuilt;
    return Ctx.getAttributedType(AttributedType::getNullabilityAttrKind(NK),
                                 Rebuilt, Rebuilt);
  }

  QualType VisitRecordType(const RecordType* RT) {
    if (const auto* CTSD =
            dyn_cast<ClassTemplateSpecializationDecl>(RT->getDecl())) {
      std::vector<TemplateArgument> TransformedArgs;
      for (const auto& Arg : CTSD->getTemplateArgs().asArray())
        TransformedArgs.push_back(Visit(Arg));
      return Ctx.getTemplateSpecializationType(
          TemplateName(CTSD->getSpecializedTemplate()), TransformedArgs,
          QualType(RT, 0));
    }
    return QualType(RT, 0);
  }

  QualType VisitFunctionProtoType(const FunctionProtoType* T) {
    QualType Ret = Visit(T->getReturnType());
    std::vector<QualType> Params;
    for (const auto& Param : T->getParamTypes()) Params.push_back(Visit(Param));
    return Ctx.getFunctionType(Ret, Params, T->getExtProtoInfo());
  }

  QualType VisitLValueReferenceType(const LValueReferenceType* T) {
    return Ctx.getLValueReferenceType(Visit(T->getPointeeType()));
  }
  QualType VisitRValueReferenceType(const RValueReferenceType* T) {
    return Ctx.getRValueReferenceType(Visit(T->getPointeeType()));
  }

  QualType VisitConstantArrayType(const ConstantArrayType* AT) {
    return Ctx.getConstantArrayType(Visit(AT->getElementType()), AT->getSize(),
                                    AT->getSizeExpr(), AT->getSizeModifier(),
                                    AT->getIndexTypeCVRQualifiers());
  }
  QualType VisitIncompleteArrayType(const IncompleteArrayType* AT) {
    return Ctx.getIncompleteArrayType(Visit(AT->getElementType()),
                                      AT->getSizeModifier(),
                                      AT->getIndexTypeCVRQualifiers());
  }
  QualType VisitVariableArrayType(const VariableArrayType* AT) {
    return Ctx.getVariableArrayType(
        Visit(AT->getElementType()), AT->getSizeExpr(), AT->getSizeModifier(),
        AT->getIndexTypeCVRQualifiers(), AT->getBracketsRange());
  }

 private:
  ArrayRef<NullabilityKind> Nullability;
  ASTContext& Ctx;
};

}  // namespace

QualType rebuildWithNullability(QualType T, const TypeNullability& Nullability,
                                ASTContext& Ctx) {
  Rebuilder V(Nullability, Ctx);
  QualType Result = V.Visit(T.getCanonicalType());
  CHECK(V.done()) << "Nullability vector[" << Nullability.size()
                  << "] too long for " << T.getAsString();
  return Result;
}

std::string printWithNullability(QualType T, const TypeNullability& Nullability,
                                 ASTContext& Ctx) {
  return rebuildWithNullability(T, Nullability, Ctx)
      .getAsString(Ctx.getPrintingPolicy());
}

}  // namespace clang::tidy::nullability
