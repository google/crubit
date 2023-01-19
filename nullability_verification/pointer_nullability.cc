// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability_verification/pointer_nullability.h"

#include "absl/log/check.h"
#include "clang/AST/TypeVisitor.h"
#include "clang/Analysis/FlowSensitive/Value.h"
#include "llvm/ADT/StringRef.h"

namespace clang {
namespace tidy {
namespace nullability {

using dataflow::AtomicBoolValue;
using dataflow::BoolValue;
using dataflow::Environment;
using dataflow::PointerValue;
using dataflow::SkipPast;

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

class CountPointersInTypeVisitor
    : public TypeVisitor<CountPointersInTypeVisitor> {
  unsigned count = 0;

 public:
  CountPointersInTypeVisitor() {}

  unsigned getCount() { return count; }

  void Visit(QualType T) {
    CHECK(T.isCanonical());
    TypeVisitor::Visit(T.getTypePtrOrNull());
  }

  void VisitPointerType(const PointerType* PT) {
    count += 1;
    Visit(PT->getPointeeType());
  }

  void VisitFunctionProtoType(const FunctionProtoType* FPT) {
    Visit(FPT->getReturnType());
  }

  void Visit(TemplateArgument TA) {
    if (TA.getKind() == TemplateArgument::Type) {
      Visit(TA.getAsType());
    }
  }

  void VisitRecordType(const RecordType* RT) {
    if (auto* CTSD = dyn_cast<ClassTemplateSpecializationDecl>(RT->getDecl())) {
      for (auto& TA : CTSD->getTemplateArgs().asArray()) {
        Visit(TA);
      }
    }
  }
};

unsigned countPointersInType(QualType T) {
  CountPointersInTypeVisitor PointerCountVisitor;
  PointerCountVisitor.Visit(T.getCanonicalType());
  return PointerCountVisitor.getCount();
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
}  // namespace nullability
}  // namespace tidy
}  // namespace clang
