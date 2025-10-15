// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/inference/inferable.h"

#include <cassert>
#include <optional>

#include "nullability/inference/inference.proto.h"
#include "nullability/type_nullability.h"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclBase.h"
#include "clang/AST/DeclTemplate.h"  // IWYU pragma: keep, to work around forward decl usage in clang
#include "clang/AST/NestedNameSpecifierBase.h"
#include "clang/AST/TypeBase.h"
#include "clang/AST/TypeVisitor.h"
#include "clang/Basic/LLVM.h"
#include "llvm/ADT/DenseSet.h"
#include "llvm/ADT/STLFunctionalExtras.h"
#include "llvm/ADT/SmallVector.h"
#include "llvm/Support/ErrorHandling.h"
#include "llvm/Support/raw_ostream.h"

namespace clang::tidy::nullability {

static void forEachTypeNestedNameSpecifier(
    const Type& T, llvm::function_ref<void(const Type&)> ForType) {
  for (NestedNameSpecifier NNS = T.getPrefix(); NNS;) {
    if (NNS.getKind() == NestedNameSpecifier::Kind::Type) {
      assert(NNS.getAsType() != nullptr);
      ForType(*NNS.getAsType());
    }

    switch (NNS.getKind()) {
      case NestedNameSpecifier::Kind::Null:
      case NestedNameSpecifier::Kind::Global:
      case NestedNameSpecifier::Kind::MicrosoftSuper:
        NNS = std::nullopt;
        break;
      case NestedNameSpecifier::Kind::Namespace:
        NNS = NNS.getAsNamespaceAndPrefix().Prefix;
        break;
      case NestedNameSpecifier::Kind::Type:
        NNS = NNS.getAsType()->getPrefix();
        break;
      default:
        NNS = std::nullopt;
        llvm_unreachable("unexpected NestedNameSpecifier kind");
        break;
    }
  }
}

static bool isSupportedPointerTypeOutsideOfSubstitutedTemplateParam(
    QualType T) {
  class Walker : public TypeVisitor<Walker, bool> {
    void addTemplateDeclsSeenInQualifiers(const Type& T) {
      forEachTypeNestedNameSpecifier(T, [this](const Type& NNSType) {
        if (const auto* TST =
                dyn_cast_or_null<TemplateSpecializationType>(&NNSType)) {
          TemplateDeclsSeen.insert(TST->getTemplateName().getAsTemplateDecl());
        }
      });
    }

    bool hasNestedNameSpecifierThatIsSubstitutedParamOfUnseenTemplate(
        const Type& T) {
      bool AnyIsParamOfUnseenTemplate = false;
      forEachTypeNestedNameSpecifier(T, [&AnyIsParamOfUnseenTemplate,
                                         this](const Type& NNSType) {
        if (const auto* Substituted =
                dyn_cast_or_null<SubstTemplateTypeParmType>(&NNSType)) {
          AnyIsParamOfUnseenTemplate = AnyIsParamOfUnseenTemplate ||
                                       isParamOfUnseenTemplate(*Substituted);
        }
      });
      return AnyIsParamOfUnseenTemplate;
    }

    bool isParamOfUnseenTemplate(const SubstTemplateTypeParmType& T) {
      // If the replaced template parameter is not a parameter of a template
      // that we've seen while walking the type, then it is a template parameter
      // of the current context and the type is only a pointer by way of the
      // replacement type. We no longer consider it to be a supported pointer
      // type.
      //
      // This avoids inferring the nullability for declarations in a template
      // that may only sometimes be pointers, e.g. a field holding the
      // underlying data in a generic wrapper type, for which we will never
      // write an annotation in the template and for which the nullability
      // should instead be specified as part of the template argument.
      if (TemplateDeclsSeen.contains(T.getAssociatedDecl())) return false;
      const auto* CTSD =
          dyn_cast<ClassTemplateSpecializationDecl>(T.getAssociatedDecl());
      return !CTSD ||
             !TemplateDeclsSeen.contains(CTSD->getSpecializedTemplate());
    }

   public:
    bool VisitType(const Type* T) {
      addTemplateDeclsSeenInQualifiers(*T);

      // If this is a type nested within a substituted template type parameter
      // of the current context, then treat the whole type as unsupported, as
      // we would the substituted template type parameter.
      if (hasNestedNameSpecifierThatIsSubstitutedParamOfUnseenTemplate(*T))
        return false;

      // Walk through sugar other than the types handled specifically below.
      if (const Type* Next =
              T->getLocallyUnqualifiedSingleStepDesugaredType().getTypePtr();
          Next != T) {
        return Visit(Next);
      }
      // But if there's no more sugar, we're done and this is not a supported
      // pointer type. We don't generally expect this to happen if
      // `isSupportedPointerType` already returned true for the starting type.
      llvm::errs()
          << "If `isSupportedPointerType` returned true for the starting type, "
             "we should have seen a pointer type and never reached this point. "
             "It is a waste to use this walker if the starting type is not a "
             "supported pointer type.\n";
      assert(false);
      return false;
    }
    bool VisitPointerType(const PointerType* T) { return true; }
    bool VisitRecordType(const RecordType* T) {
      addTemplateDeclsSeenInQualifiers(*T);
      bool IsSupported = isSupportedSmartPointerType(QualType(T, 0));
      if (!IsSupported) {
        llvm::errs() << "If `isSupportedPointerType` returned true for the "
                        "starting type, then `IsSupported` should also be "
                        "true. It is a waste to use this walker if the "
                        "starting type is not a supported pointer type.\n";
        assert(false);
      }
      return IsSupported;
    }
    bool VisitSubstTemplateTypeParmType(const SubstTemplateTypeParmType* T) {
      addTemplateDeclsSeenInQualifiers(*T);
      if (isParamOfUnseenTemplate(*T)) {
        return false;
      }

      // If the template parameter being replaced is a parameter of a template
      // decl referenced in the type being walked, we allow for the pointer type
      // to be inside the replacement type.
      // This allows for the template parameters of type aliases and name
      // specifiers to make the type a pointer and still consider the type
      // inferable.
      return Visit(T->getReplacementType().getTypePtr());
    }

    bool VisitTemplateSpecializationType(const TemplateSpecializationType* T) {
      addTemplateDeclsSeenInQualifiers(*T);
      if (T->isTypeAlias()) {
        TemplateDeclsSeen.insert(T->getTemplateName().getAsTemplateDecl());
        return Visit(T->getAliasedType().getTypePtr());
      }
      bool IsSupported = isSupportedSmartPointerType(QualType(T, 0));
      if (!IsSupported) {
        llvm::errs() << "If `isSupportedPointerType` returned true for the "
                        "starting type, then `IsSupported` should also be "
                        "true. It is a waste to use this walker if the "
                        "starting type is not a supported pointer type.\n";
        assert(false);
      }
      return IsSupported;
    }

   private:
    llvm::DenseSet<const Decl*> TemplateDeclsSeen;
  };

  return Walker().Visit(T.getTypePtr());
}

bool hasInferable(QualType T) {
  QualType NonReferenceType = T.getNonReferenceType();
  return isSupportedPointerType(NonReferenceType) &&
         isSupportedPointerTypeOutsideOfSubstitutedTemplateParam(
             NonReferenceType);
}

int countInferableSlots(const Decl& D) {
  if (const auto* Func = dyn_cast<FunctionDecl>(&D)) {
    int Slots = 0;
    if (hasInferable(Func->getReturnType())) ++Slots;
    for (auto* P : Func->parameters())
      if (hasInferable(P->getType())) ++Slots;
    return Slots;
  }
  if (const auto* Field = dyn_cast<FieldDecl>(&D)) {
    return hasInferable(Field->getType()) ? 1 : 0;
  }
  if (const auto* Var = dyn_cast<VarDecl>(&D)) {
    return hasInferable(Var->getType()) ? 1 : 0;
  }
  return 0;
}

llvm::SmallVector<int> getInferableSlotIndices(const Decl& D) {
  if (const auto* Func = dyn_cast<FunctionDecl>(&D)) {
    llvm::SmallVector<int> Slots;
    if (hasInferable(Func->getReturnType())) Slots.push_back(SLOT_RETURN_TYPE);
    // Intentionally match the iteration pattern evidence collection uses over
    // function parameters when gathering inferable slots, to avoid any subtle
    // differences for complex function contexts.
    auto Parameters = Func->parameters();
    for (auto I = 0; I < Parameters.size(); ++I) {
      const ParmVarDecl* Param = Parameters[I];
      if (hasInferable(Param->getType())) Slots.push_back(I + SLOT_PARAM);
    }
    return Slots;
  }
  if (const auto* Field = dyn_cast<FieldDecl>(&D)) {
    if (hasInferable(Field->getType())) return {0};
    return {};
  }
  if (const auto* Var = dyn_cast<VarDecl>(&D)) {
    if (hasInferable(Var->getType())) return {0};
    return {};
  }
  return {};
}

bool isInferenceTarget(const Decl& D) {
  if (const auto* Func = dyn_cast<FunctionDecl>(&D)) {
    return
        // Function templates are in principle inferable.
        // However since we don't analyze their bodies, and other
        // implementations cannot interact with them directly, we can't
        // perform any nontrivial inference, just propagate annotations
        // across redecls. For now, we don't do this as some infra
        // (NullabilityWalker) doesn't work on dependent code. Instead, we infer
        // for the instantiations and the decls inside the instantiations, and
        // can use matching source ranges (the ranges inside the template) to
        // merge evidence and make an inference when all instantiations are
        // consistent enough.
        !Func->isDependentContext() &&
        // Same treatment for functions in templates as for function templates.
        !Func->getDeclContext()->isDependentContext() &&
        // builtins can't be annotated and are irregular in their type checking
        // and in other ways, leading to violations of otherwise sound
        // assumptions.
        // If we find that their nullability is unexpectedly leaking into
        // programs under analysis in significant ways, we can hardcode this
        // small set of functions.
        Func->getBuiltinID() == 0 &&
        // Implicit functions cannot be annotated.
        !Func->isImplicit() &&
        // Do the most expensive check last.
        countInferableSlots(*Func) > 0;
  }
  if (const auto* Field = dyn_cast<FieldDecl>(&D)) {
    return
        // See comments above regarding templates.
        !Field->getDeclContext()->isDependentContext() &&
        // Do the most expensive check last.
        countInferableSlots(*Field) > 0;
  }
  if (const auto* Var = dyn_cast<VarDecl>(&D)) {
    // Include static member variables, global variables, and local variables,
    // including static variables defined in a function.

    // Exclude parameters, which are handled as part of their enclosing function
    // declaration.
    if (isa<ParmVarDecl>(Var)) return false;

    return
        // Exclude variables inside templates as well as variable templates. See
        // comments above regarding similar restrictions on functions. As with
        // functions, the analogous variables inside instantiations and variable
        // template instantiations are not excluded.
        !Var->getDeclContext()->isDependentContext() && !Var->isTemplated() &&
        // Do the most expensive check last.
        countInferableSlots(*Var) > 0;
  }
  return false;
}

}  // namespace clang::tidy::nullability
