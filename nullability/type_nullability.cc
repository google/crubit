// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "absl/log/check.h"
#include "nullability/pointer_nullability.h"
#include "clang/AST/ASTContext.h"
#include "clang/AST/DeclTemplate.h"
#include "clang/AST/Type.h"
#include "clang/AST/TypeVisitor.h"
#include "clang/Basic/LLVM.h"
#include "clang/Basic/Specifiers.h"

namespace clang::tidy::nullability {
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
struct Visitor : public TypeVisitor<Visitor, QualType> {
  Visitor(ArrayRef<NullabilityKind> Nullability, ASTContext& Ctx)
      : Nullability(Nullability), Ctx(Ctx) {}

  bool done() const { return Nullability.empty(); }

  using Base = TypeVisitor<Visitor, QualType>;
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

 private:
  ArrayRef<NullabilityKind> Nullability;
  ASTContext& Ctx;
};

}  // namespace

QualType rebuildWithNullability(QualType T,
                                ArrayRef<NullabilityKind> Nullability,
                                ASTContext& Ctx) {
  Visitor V(Nullability, Ctx);
  QualType Result = V.Visit(T.getCanonicalType());
  CHECK(V.done()) << "Nullability vector[" << Nullability.size()
                  << "] too long for " << T.getAsString();
  return Result;
}

std::string printWithNullability(QualType T,
                                 ArrayRef<NullabilityKind> Nullability,
                                 ASTContext& Ctx) {
  return rebuildWithNullability(T, Nullability, Ctx)
      .getAsString(Ctx.getPrintingPolicy());
}

}  // namespace clang::tidy::nullability
