// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "common/annotation_reader.h"

#include <optional>
#include <string>
#include <utility>
#include <vector>

#include "common/status.h"
#include "common/status_macros.h"
#include "clang/AST/APValue.h"
#include "clang/AST/ASTContext.h"
#include "clang/AST/Attr.h"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclBase.h"
#include "clang/AST/DeclTemplate.h"
#include "clang/AST/Expr.h"
#include "clang/AST/TypeBase.h"
#include "clang/Basic/AttrKinds.h"
#include "clang/Basic/LLVM.h"
#include "llvm/ADT/APSInt.h"
#include "llvm/ADT/ArrayRef.h"
#include "llvm/ADT/STLFunctionalExtras.h"
#include "llvm/ADT/StringRef.h"
#include "llvm/Support/Error.h"
#include "llvm/Support/FormatVariadic.h"

namespace crubit {
namespace {

// Returns the string literal value of `expr`.
//
// `ast_context` is the associated `ASTContext`. If `expr` is not a string
// literal, `error()` is returned.
//
// As `StringLiteral` expressions are always allocated on the `ASTContext (see
// `StringLiteral::Create()`), the returned `string_view` has the same lifetime
// as `ast_context`.
//
// TODO(yongheng): Merge with lifetime_annotations/type_lifetimes.cc.
static llvm::Expected<llvm::StringRef> GetExprAsStringLiteral(
    const clang::Expr& expr,
    const clang::ASTContext& ast_context [[clang::lifetimebound]],
    llvm::function_ref<llvm::Error()> error) {
  clang::Expr::EvalResult eval_result;
  if (!expr.EvaluateAsConstantExpr(eval_result, ast_context) ||
      !eval_result.Val.isLValue()) {
    return error();
  }

  const auto* eval_result_expr =
      eval_result.Val.getLValueBase().dyn_cast<const clang::Expr*>();
  if (!eval_result_expr) {
    return error();
  }

  const auto* string_literal =
      clang::dyn_cast<clang::StringLiteral>(eval_result_expr);
  if (!string_literal) {
    return error();
  }

  return string_literal->getString();
}

// Returns the `AnnotateAttr` with the given `annotation_name` if it exists. If
// there are multiple annotations with the given name, returns an error.
//
// For example, given the following C++ code:
//
// class [[clang::annotate("crubit_annotation_foo", "bar")]] MyClass {}
//   ...
// };
//
// GetAnnotateAttrSingleDecl(my_class_decl, "crubit_annotation_foo") will
// return the `AnnotateAttr` with the annotation `crubit_annotation_foo`,
// from which the "bar" argument can be extracted.
static llvm::Expected<const clang::AnnotateAttr*> GetAnnotateAttrSingleDecl(
    const clang::Decl& decl, llvm::StringRef annotation_name) {
  const clang::AnnotateAttr* found_attr = nullptr;
  for (clang::AnnotateAttr* attr : decl.specific_attrs<clang::AnnotateAttr>()) {
    if (attr->getAnnotation() != llvm::StringRef(annotation_name)) continue;

    if (found_attr != nullptr) {
      return crubit::InvalidArgumentError(
          (llvm::Twine("Only one `") + annotation_name +
           "` annotation may be placed on a declaration.")
              .str());
    }
    found_attr = attr;
  }
  return found_attr;
}

static llvm::Error InconsistentAnnotationsError(
    llvm::StringRef annotation_name) {
  return crubit::InvalidArgumentError(
      (llvm::Twine("Different declarations have inconsistent `") +
       annotation_name + "` annotations.")
          .str());
}

static llvm::Error CheckExpressionsAreSameConstant(
    const clang::Expr& expr1, const clang::Expr& expr2,
    llvm::StringRef annotation_name, const clang::ASTContext& ast_context) {
  if (!ast_context.hasSameType(expr1.getType(), expr2.getType())) {
    return InconsistentAnnotationsError(annotation_name);
  }

  clang::Expr::EvalResult eval_result_1, eval_result_2;
  if (!expr1.EvaluateAsConstantExpr(eval_result_1, ast_context) ||
      !expr2.EvaluateAsConstantExpr(eval_result_2, ast_context)) {
    return crubit::InvalidArgumentError(
        (llvm::Twine("Arguments of `") + annotation_name +
         "` annotation must be constant expressions.")
            .str());
  }

  const clang::APValue& value1 = eval_result_1.Val;
  const clang::APValue& value2 = eval_result_2.Val;

  if (value1.getKind() != value2.getKind()) {
    return InconsistentAnnotationsError(annotation_name);
  }

  auto must_be_int_or_string_error = [annotation_name]() {
    return crubit::InvalidArgumentError(
        (llvm::Twine("Arguments of `") + annotation_name +
         "` annotation must be of integral type, string literal, "
         "or constructor.")
            .str());
  };

  switch (value1.getKind()) {
    case clang::APValue::Int:
      if (value1.getInt() != value2.getInt()) {
        return InconsistentAnnotationsError(annotation_name);
      }
      break;
    case clang::APValue::Struct:
      // Structs only appear when we use them for their type info, they
      // are all empty though.
      break;
    case clang::APValue::LValue: {
      CRUBIT_ASSIGN_OR_RETURN(
          llvm::StringRef value1_string,
          GetExprAsStringLiteral(expr1, ast_context,
                                 must_be_int_or_string_error));
      CRUBIT_ASSIGN_OR_RETURN(
          llvm::StringRef value2_string,
          GetExprAsStringLiteral(expr2, ast_context,
                                 must_be_int_or_string_error));
      if (value1_string != value2_string) {
        return InconsistentAnnotationsError(annotation_name);
      }
      break;
    }
    default:
      return must_be_int_or_string_error();
  }

  return llvm::Error::success();
}

static llvm::Error CheckAnnotationsConsistent(
    const clang::AnnotateAttr* annotate1, const clang::AnnotateAttr* annotate2,
    const clang::ASTContext& ast_context) {
  if (annotate1->args_size() != annotate2->args_size())
    return InconsistentAnnotationsError(annotate1->getAnnotation());
  for (int i = 0; i < annotate1->args_size(); ++i) {
    if (auto err = CheckExpressionsAreSameConstant(
            *annotate1->args_begin()[i], *annotate2->args_begin()[i],
            annotate1->getAnnotation(), ast_context))
      return err;
  }
  return llvm::Error::success();
}

// Returns the `clang::Decl` that should be used for reading annotations.
//
// Template declarations technically do not have annotations-- the
// *templated* decl has annotations. So, if we're searching for annotations
// on a template decl, we should search for annotations on the templated
// decl instead.
static const clang::Decl& DeclForAnnotations(const clang::Decl& decl) {
  auto* template_decl = clang::dyn_cast<clang::TemplateDecl>(&decl);
  if (template_decl == nullptr) {
    return decl;
  }
  auto* templated_decl = template_decl->getTemplatedDecl();
  if (templated_decl == nullptr) {
    return decl;
  }
  return *templated_decl;
}

}  // namespace

llvm::Expected<bool> GetExprAsBool(const clang::Expr& expr,
                                   const clang::ASTContext& ast_context) {
  if (!expr.getType()->isBooleanType()) {
    return crubit::InvalidArgumentError(
        "annotation expression must evaluate to a bool");
  }
  clang::Expr::EvalResult eval_result;
  if (!expr.EvaluateAsConstantExpr(eval_result, ast_context)) {
    return crubit::InvalidArgumentError(
        "failed to evaluate annotation expression as a constant");
  }
  if (eval_result.Val.getKind() != clang::APValue::Int) {
    return crubit::InvalidArgumentError(
        "annotation expression must evaluate to a bool");
  }
  const llvm::APSInt& int_value = eval_result.Val.getInt();
  return !int_value.isZero();
}

llvm::Expected<llvm::StringRef> GetExprAsStringLiteral(
    const clang::Expr& expr,
    const clang::ASTContext& ast_context [[clang::lifetimebound]]) {
  return GetExprAsStringLiteral(expr, ast_context, []() {
    return crubit::InvalidArgumentError(
        "cannot evaluate argument as a string literal");
  });
}

llvm::Expected<std::optional<AnnotateArgs>> GetAnnotateAttrArgs(
    const clang::Decl& decl, llvm::StringRef annotation_name) {
  const clang::AnnotateAttr* found_attr = nullptr;

  int num_found = 0;
  for (const clang::Decl* redecl_ptr : decl.redecls()) {
    if (redecl_ptr == nullptr) continue;
    const clang::Decl& redecl = DeclForAnnotations(*redecl_ptr);

    CRUBIT_ASSIGN_OR_RETURN(const clang::AnnotateAttr* attr,
                            GetAnnotateAttrSingleDecl(redecl, annotation_name));

    if (attr != nullptr) {
      ++num_found;
      if (found_attr == nullptr) {
        found_attr = attr;
      } else {
        CRUBIT_RETURN_IF_ERROR(
            CheckAnnotationsConsistent(found_attr, attr, decl.getASTContext()));
      }
    }
  }

  // If only one redeclaration had an annotation, check the annotation against
  // itself. This checks that all arguments have the expected type.
  if (num_found == 1) {
    CRUBIT_RETURN_IF_ERROR(CheckAnnotationsConsistent(
        found_attr, found_attr, DeclForAnnotations(decl).getASTContext()));
  }

  if (found_attr == nullptr) {
    return std::nullopt;
  }

  return AnnotateArgs(found_attr->args_begin(), found_attr->args_end());
}

llvm::Expected<bool> HasAnnotationWithoutArgs(const clang::Decl& decl,
                                              llvm::StringRef annotation) {
  llvm::Expected<std::optional<AnnotateArgs>> maybe_args =
      GetAnnotateAttrArgs(decl, annotation);
  if (!maybe_args) {
    return maybe_args.takeError();
  }
  if (!maybe_args->has_value()) {
    return false;
  }
  if (!maybe_args->value().empty()) {
    return crubit::InvalidArgumentError((llvm::Twine("Annotation ") +
                                         annotation +
                                         " does not expect arguments.")
                                            .str());
  }
  return true;
}

llvm::Expected<std::optional<std::vector<std::string>>>
GetAnnotationWithStringArgs(const clang::Decl& decl,
                            llvm::StringRef annotation_name) {
  CRUBIT_ASSIGN_OR_RETURN(std::optional<AnnotateArgs> maybe_args,
                          GetAnnotateAttrArgs(decl, annotation_name));
  if (!maybe_args.has_value()) {
    return std::nullopt;
  }
  const AnnotateArgs& args = *maybe_args;
  std::vector<std::string> result;
  result.reserve(args.size());
  for (const clang::Expr* arg_expr : args) {
    llvm::Expected<llvm::StringRef> arg =
        GetExprAsStringLiteral(*arg_expr, decl.getASTContext());
    if (!arg) {
      return crubit::InvalidArgumentError(
          (llvm::Twine("Annotation ") + annotation_name +
           " arguments must be string literals.")
              .str());
    }
    result.push_back(std::string(*arg));
  }
  return result;
}

llvm::Expected<std::optional<std::string>> GetAnnotationWithStringArg(
    const clang::Decl& decl, llvm::StringRef annotation_name) {
  CRUBIT_ASSIGN_OR_RETURN(std::optional<std::vector<std::string>> maybe_args,
                          GetAnnotationWithStringArgs(decl, annotation_name));
  if (!maybe_args.has_value()) {
    return std::nullopt;
  }
  if (maybe_args->size() != 1) {
    return crubit::InvalidArgumentError((llvm::Twine("Annotation ") +
                                         annotation_name +
                                         " must have a single string argument.")
                                            .str());
  }
  return std::move(maybe_args->front());
}

llvm::Expected<const clang::AnnotateTypeAttr* _Nullable>
GetTypeAnnotationSingleDecl(const clang::Type* _Nonnull type
                            [[clang::lifetimebound]],
                            llvm::StringRef annotation_name) {
  const clang::Type* current_type = type;
  const clang::AnnotateTypeAttr* found_attr = nullptr;
  while (const auto* attributed_type =
             current_type->getAs<clang::AttributedType>()) {
    if (attributed_type->getAttrKind() == clang::attr::AnnotateType) {
      const clang::AnnotateTypeAttr* annotateAttr =
          clang::cast<clang::AnnotateTypeAttr>(attributed_type->getAttr());
      if (annotateAttr->getAnnotation() == llvm::StringRef(annotation_name)) {
        if (found_attr != nullptr) {
          return crubit::InvalidArgumentError(
              (llvm::Twine("Only one `") + annotation_name +
               "` annotation may be placed on a type.")
                  .str());
        }
        found_attr = annotateAttr;
      }
    }
    current_type = attributed_type->getEquivalentType().getTypePtr();
  }
  return found_attr;
}

}  // namespace crubit
