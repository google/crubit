// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "lifetime_annotations/type_lifetimes.h"

#include <algorithm>
#include <memory>
#include <optional>
#include <string>
#include <utility>

#include "lifetime_annotations/lifetime.h"
#include "lifetime_annotations/lifetime_symbol_table.h"
#include "lifetime_annotations/pointee_type.h"
#include "third_party/absl/strings/str_cat.h"
#include "third_party/absl/strings/str_format.h"
#include "third_party/absl/strings/str_join.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/Attr.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/Attrs.inc"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/DeclCXX.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/DeclTemplate.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/TemplateBase.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/Type.h"
#include "third_party/llvm/llvm-project/clang/include/clang/Basic/SourceLocation.h"
#include "third_party/llvm/llvm-project/llvm/include/llvm/ADT/ArrayRef.h"
#include "third_party/llvm/llvm-project/llvm/include/llvm/ADT/DenseMapInfo.h"
#include "third_party/llvm/llvm-project/llvm/include/llvm/ADT/Hashing.h"
#include "third_party/llvm/llvm-project/llvm/include/llvm/ADT/SmallVector.h"
#include "third_party/llvm/llvm-project/llvm/include/llvm/ADT/StringRef.h"
#include "third_party/llvm/llvm-project/llvm/include/llvm/Support/ErrorHandling.h"

namespace devtools_rust {

std::string DebugString(const TypeLifetimes& lifetimes,
                        const LifetimeFormatter& formatter) {
  std::vector<std::string> parts;
  parts.reserve(lifetimes.size());

  for (Lifetime l : lifetimes) {
    parts.push_back(formatter(l));
  }

  if (parts.size() == 1) {
    return parts[0];
  }
  return absl::StrFormat("(%s)", absl::StrJoin(parts, ", "));
}

llvm::SmallVector<std::string> GetLifetimeParameters(clang::QualType type) {
  // TODO(mboehme):
  // - Add support for type aliases with lifetime parameters
  // - Report errors as Clang diagnostics, not through
  //   llvm::report_fatal_error().

  auto record = type->getAs<clang::RecordType>();
  if (!record) {
    return {};
  }

  auto cxx_record = record->getAsCXXRecordDecl();
  if (!cxx_record) {
    return {};
  }

  const clang::AnnotateAttr* lifetime_params_attr = nullptr;
  for (auto annotate : cxx_record->specific_attrs<clang::AnnotateAttr>()) {
    if (annotate->getAnnotation() == "lifetime_params") {
      if (lifetime_params_attr) {
        llvm::report_fatal_error("repeated lifetime annotation");
      }
      lifetime_params_attr = annotate;
    }
  }
  if (!lifetime_params_attr) {
    return {};
  }

  llvm::SmallVector<std::string> ret;
  for (const auto& arg : lifetime_params_attr->args()) {
    llvm::StringRef lifetime;
    if (llvm::Error err =
            EvaluateAsStringLiteral(arg, cxx_record->getASTContext())
                .moveInto(lifetime)) {
      llvm::report_fatal_error(llvm::StringRef(toString(std::move(err))));
    }
    ret.push_back(lifetime.str());
  }

  return ret;
}

TypeLifetimes CreateLifetimesForType(
    clang::QualType type, std::function<Lifetime()> lifetime_factory) {
  assert(!type.isNull());
  TypeLifetimes ret;

  for (const auto& lftm_param : GetLifetimeParameters(type)) {
    (void)lftm_param;
    ret.push_back(lifetime_factory());
  }

  // Add implicit lifetime parameters for type template parameters.
  llvm::ArrayRef<clang::TemplateArgument> template_args = GetTemplateArgs(type);
  if (!template_args.empty()) {
    for (const clang::TemplateArgument& arg : template_args) {
      if (arg.getKind() == clang::TemplateArgument::Type) {
        ret.append(CreateLifetimesForType(arg.getAsType(), lifetime_factory));
      } else if (arg.getKind() == clang::TemplateArgument::Pack) {
        for (const clang::TemplateArgument& inner_arg : arg.getPackAsArray()) {
          if (inner_arg.getKind() == clang::TemplateArgument::Type) {
            ret.append(CreateLifetimesForType(inner_arg.getAsType(),
                                              lifetime_factory));
          }
        }
      }
    }
    return ret;
  }
  clang::QualType pointee = PointeeType(type);
  if (pointee.isNull()) return ret;
  ret = CreateLifetimesForType(pointee, lifetime_factory);
  ret.push_back(lifetime_factory());
  return ret;
}

ValueLifetimes& ValueLifetimes::operator=(const ValueLifetimes& other) {
  type_ = other.type_;
  template_argument_lifetimes_ = other.template_argument_lifetimes_;
  lifetime_parameters_by_name_ = other.lifetime_parameters_by_name_;
  pointee_lifetimes_ =
      other.pointee_lifetimes_
          ? std::make_unique<ObjectLifetimes>(*other.pointee_lifetimes_)
          : nullptr;
  return *this;
}

std::string ValueLifetimes::DebugString() const {
  std::string ret;
  if (!lifetime_parameters_by_name_.GetMapping().empty()) {
    std::vector<std::string> lftm_args_strings;
    for (const auto& lftm_arg : lifetime_parameters_by_name_.GetMapping()) {
      lftm_args_strings.push_back(lftm_arg.second.DebugString());
    }
    absl::StrAppend(&ret, "(", absl::StrJoin(lftm_args_strings, ", "), ")");
  }
  if (!template_argument_lifetimes_.empty()) {
    std::vector<std::string> tmpl_arg_strings;
    for (const std::optional<ValueLifetimes>& tmpl_arg :
         template_argument_lifetimes_) {
      if (tmpl_arg) {
        tmpl_arg_strings.push_back(tmpl_arg->DebugString());
      } else {
        tmpl_arg_strings.push_back("");
      }
    }
    absl::StrAppend(&ret, "<", absl::StrJoin(tmpl_arg_strings, ", "), ">");
  }
  if (pointee_lifetimes_) {
    absl::StrAppend(&ret, " -> ", pointee_lifetimes_->DebugString());
  }
  return ret;
}

void ValueLifetimes::ReverseVisitTemplateArgs(
    llvm::ArrayRef<clang::TemplateArgument> template_args,
    TypeLifetimesRef& type_lifetimes, ValueLifetimes& out) {
  for (size_t i = template_args.size(); i-- > 0;) {
    const clang::TemplateArgument& arg = template_args[i];
    if (arg.getKind() == clang::TemplateArgument::Type) {
      out.template_argument_lifetimes_.push_back(
          FromTypeLifetimes(type_lifetimes, arg.getAsType()));
    } else if (arg.getKind() == clang::TemplateArgument::Pack) {
      ReverseVisitTemplateArgs(arg.getPackAsArray(), type_lifetimes, out);
    } else {
      out.template_argument_lifetimes_.push_back(std::nullopt);
    }
  }
}

// Here, `type_lifetimes` are the lifetimes of a prvalue of the given `type`,
// unlike ObjectLifetimes::FromTypeLifetimes, which assumes a glvalue.
ValueLifetimes ValueLifetimes::FromTypeLifetimes(
    TypeLifetimesRef& type_lifetimes, clang::QualType type) {
  assert(!type.isNull());

  ValueLifetimes ret(type);

  llvm::SmallVector<std::string> params = GetLifetimeParameters(type);
  // Visit in reverse order, as we are doing a post-order traversal.
  for (size_t i = params.size(); i-- > 0;) {
    if (ret.lifetime_parameters_by_name_.LookupName(params[i])) {
      llvm::report_fatal_error("duplicate lifetime parameter name");
    }
    ret.lifetime_parameters_by_name_.Add(params[i], type_lifetimes.back());
    type_lifetimes = type_lifetimes.drop_back();
  }

  llvm::ArrayRef<clang::TemplateArgument> template_args = GetTemplateArgs(type);
  if (!template_args.empty()) {
    // Since we are simulating reversing a post-order visit, we need to
    // extract template arguments in reverse order.
    ReverseVisitTemplateArgs(template_args, type_lifetimes, ret);
    std::reverse(ret.template_argument_lifetimes_.begin(),
                 ret.template_argument_lifetimes_.end());
    return ret;
  }

  clang::QualType pointee_type = PointeeType(type);
  if (!pointee_type.isNull()) {
    ret.pointee_lifetimes_ = std::make_unique<ObjectLifetimes>(
        ObjectLifetimes::FromTypeLifetimes(type_lifetimes, pointee_type));
  }
  return ret;
}

ValueLifetimes ValueLifetimes::ForLifetimeLessType(clang::QualType type) {
  assert(PointeeType(type).isNull() && !type->isRecordType());
  return ValueLifetimes(type);
}

ValueLifetimes ValueLifetimes::ForPointerLikeType(
    clang::QualType type, const ObjectLifetimes& object_lifetimes) {
  assert(!PointeeType(type).isNull());
  ValueLifetimes result(type);
  result.pointee_lifetimes_ =
      std::make_unique<ObjectLifetimes>(object_lifetimes);
  return result;
}

ValueLifetimes ValueLifetimes::ForRecord(
    clang::QualType type,
    std::vector<std::optional<ValueLifetimes>> template_argument_lifetimes,
    LifetimeSymbolTable lifetime_parameters) {
  assert(type->isRecordType());
  assert(GetTemplateArgs(type).size() == template_argument_lifetimes.size());
  ValueLifetimes result(type);
  result.template_argument_lifetimes_ = std::move(template_argument_lifetimes);
  result.lifetime_parameters_by_name_ = lifetime_parameters;
  return result;
}

const ObjectLifetimes& ValueLifetimes::GetPointeeLifetimes() const {
  assert(!PointeeType(type_).isNull());
  return *pointee_lifetimes_;
}

ObjectLifetimes ObjectLifetimes::FromTypeLifetimes(
    TypeLifetimesRef& type_lifetimes, clang::QualType type) {
  assert(!type_lifetimes.empty());
  assert(!type.isNull());
  Lifetime self_lifetime = type_lifetimes.back();
  type_lifetimes = type_lifetimes.drop_back();
  return ObjectLifetimes(
      self_lifetime, ValueLifetimes::FromTypeLifetimes(type_lifetimes, type));
}

std::string ObjectLifetimes::DebugString() const {
  return absl::StrCat(lifetime_.DebugString(), value_lifetimes_.DebugString());
}

const llvm::ArrayRef<clang::TemplateArgument> GetTemplateArgs(
    clang::QualType type) {
  if (auto specialization = type->getAs<clang::TemplateSpecializationType>()) {
    return specialization->template_arguments();
  }
  if (auto record = type->getAs<clang::RecordType>()) {
    if (auto specialization_decl =
            clang::dyn_cast<clang::ClassTemplateSpecializationDecl>(
                record->getDecl())) {
      return specialization_decl->getTemplateArgs().asArray();
    }
  }
  return {};
}

ObjectLifetimes ObjectLifetimes::GetObjectLifetimesForTypeInContext(
    clang::QualType type, llvm::SmallVector<std::string> type_lifetime_args,
    llvm::StringRef object_lifetime_parameter) const {
  assert(value_lifetimes_.Type()->isRecordType());

  // The object of the `type` (i.e. field or a base class) basically has the
  // same lifetime as the struct.
  Lifetime ret_lifetime = lifetime_;
  // ... unless the field has lifetime parameters.
  if (!object_lifetime_parameter.empty()) {
    ret_lifetime =
        value_lifetimes_.GetLifetimeParameter(object_lifetime_parameter);
  }

  // `type` is one of a template argument, a struct, a pointer, or a type
  // with no lifetimes (other than its own).

  // First case: template argument. We just attach the
  // template argument's lifetimes to the leaf ObjectLifetimes.
  if (auto targ = type->getAs<clang::SubstTemplateTypeParmType>()) {
    const std::optional<ValueLifetimes>& arg_lifetimes =
        value_lifetimes_.GetTemplateArgumentLifetimes(
            targ->getReplacedParameter()->getIndex());
    if (!arg_lifetimes) {
      assert(false);
      return llvm::DenseMapInfo<ObjectLifetimes>::getEmptyKey();
    }
    return ObjectLifetimes(ret_lifetime, *arg_lifetimes);
  } else if (type->isStructureOrClassType()) {
    // Second case: struct.
    // Resolve lifetime parameters for the struct, if it has any.
    LifetimeSymbolTable lifetime_params;
    llvm::SmallVector<std::string> params = GetLifetimeParameters(type);
    for (size_t i = params.size(); i-- > 0;) {
      assert(!type_lifetime_args.empty());
      auto lftm_arg = type_lifetime_args.back();
      type_lifetime_args.pop_back();
      lifetime_params.Add(params[i],
                          value_lifetimes_.GetLifetimeParameter(lftm_arg));
    }

    // We need to construct potentally reshuffled
    // template arguments, if the struct is a template.

    // TODO(veluca): mixing lifetimes and template parameters is not supported
    // yet.
    std::vector<std::optional<ValueLifetimes>> template_argument_lifetimes;
    size_t parameter_pack_expansion_index = 0;
    for (const clang::TemplateArgument& arg : GetTemplateArgs(type)) {
      if (arg.getKind() == clang::TemplateArgument::Type) {
        if (auto templ_arg = clang::dyn_cast<clang::SubstTemplateTypeParmType>(
                arg.getAsType())) {
          // Template parameter packs get the index of the *pack*, not the index
          // of the type inside the pack itself. As they must appear last, we
          // can just increase the counter at every occurrence and wraparound
          // when we run out of template arguments.
          size_t index = templ_arg->getReplacedParameter()->getIndex();
          if (templ_arg->getReplacedParameter()->isParameterPack()) {
            index += parameter_pack_expansion_index++;
            if (index + 1 >= value_lifetimes_.GetNumTemplateArguments()) {
              parameter_pack_expansion_index = 0;
            }
          }
          template_argument_lifetimes.push_back(
              value_lifetimes_.GetTemplateArgumentLifetimes(index));
        } else {
          // Create a new ValueLifetimes of the type of the template parameter,
          // with lifetime `lifetime_`.
          // TODO(veluca): we need to propagate lifetime parameters here.
          TypeLifetimes type_lifetimes = CreateLifetimesForType(
              arg.getAsType(), [this]() { return this->lifetime_; });
          TypeLifetimesRef type_lifetimes_ref(type_lifetimes);
          template_argument_lifetimes.push_back(
              ValueLifetimes::FromTypeLifetimes(type_lifetimes_ref,
                                                arg.getAsType()));
        }
      } else {
        template_argument_lifetimes.push_back(std::nullopt);
      }
    }

    return ObjectLifetimes(
        ret_lifetime,
        ValueLifetimes::ForRecord(type, std::move(template_argument_lifetimes),
                                  std::move(lifetime_params)));
  } else if (clang::QualType pointee_type = PointeeType(type);
             !pointee_type.isNull()) {
    std::string object_lifetime_parameter;
    if (!type_lifetime_args.empty()) {
      object_lifetime_parameter = std::move(type_lifetime_args.back());
      type_lifetime_args.pop_back();
    }
    // Third case: pointer.
    return ObjectLifetimes(
        ret_lifetime, ValueLifetimes::ForPointerLikeType(
                          type, GetObjectLifetimesForTypeInContext(
                                    pointee_type, std::move(type_lifetime_args),
                                    object_lifetime_parameter)));
  }

  return ObjectLifetimes(ret_lifetime,
                         ValueLifetimes::ForLifetimeLessType(type));
}

ObjectLifetimes ObjectLifetimes::GetFieldOrBaseLifetimes(
    clang::QualType type,
    llvm::SmallVector<std::string> type_lifetime_args) const {
  return GetObjectLifetimesForTypeInContext(type, std::move(type_lifetime_args),
                                            "");
}

llvm::Expected<llvm::StringRef> EvaluateAsStringLiteral(
    const clang::Expr* expr, const clang::ASTContext& ast_context) {
  auto error = []() {
    return llvm::createStringError(
        llvm::inconvertibleErrorCode(),
        "cannot evaluate argument as a string literal");
  };

  clang::Expr::EvalResult eval_result;
  if (!expr->EvaluateAsConstantExpr(eval_result, ast_context) ||
      !eval_result.Val.isLValue()) {
    return error();
  }

  const auto* eval_result_expr =
      eval_result.Val.getLValueBase().dyn_cast<const clang::Expr*>();
  if (!eval_result_expr) {
    return error();
  }

  const auto* strlit = clang::dyn_cast<clang::StringLiteral>(eval_result_expr);
  if (!strlit) {
    return error();
  }

  return strlit->getString();
}

}  // namespace devtools_rust

namespace llvm {

bool DenseMapInfo<devtools_rust::ValueLifetimes>::isEqual(
    const devtools_rust::ValueLifetimes& lhs,
    const devtools_rust::ValueLifetimes& rhs) {
  if (lhs.type_ != rhs.type_) {
    return false;
  }
  if ((lhs.pointee_lifetimes_ == nullptr) !=
      (rhs.pointee_lifetimes_ == nullptr)) {
    return false;
  }
  if (lhs.pointee_lifetimes_ &&
      !DenseMapInfo<devtools_rust::ObjectLifetimes>::isEqual(
          *lhs.pointee_lifetimes_, *rhs.pointee_lifetimes_)) {
    return false;
  }
  if (lhs.template_argument_lifetimes_.size() !=
      rhs.template_argument_lifetimes_.size()) {
    return false;
  }
  for (size_t i = 0; i < lhs.template_argument_lifetimes_.size(); i++) {
    const auto& alhs = lhs.template_argument_lifetimes_[i];
    const auto& arhs = rhs.template_argument_lifetimes_[i];
    if (alhs.has_value() != arhs.has_value()) {
      return false;
    }
    if (alhs.has_value() && !isEqual(*alhs, *arhs)) {
      return false;
    }
  }
  if (lhs.lifetime_parameters_by_name_.GetMapping() !=
      rhs.lifetime_parameters_by_name_.GetMapping()) {
    return false;
  }
  return true;
}

unsigned DenseMapInfo<devtools_rust::ValueLifetimes>::getHashValue(
    const devtools_rust::ValueLifetimes& value_lifetimes) {
  llvm::hash_code hash = 0;
  if (value_lifetimes.pointee_lifetimes_) {
    hash = DenseMapInfo<devtools_rust::ObjectLifetimes>::getHashValue(
        *value_lifetimes.pointee_lifetimes_);
  }
  for (const auto& tmpl_lifetime :
       value_lifetimes.template_argument_lifetimes_) {
    if (tmpl_lifetime) {
      hash = hash_combine(hash, getHashValue(*tmpl_lifetime));
    }
  }
  for (const auto& lifetime_arg :
       value_lifetimes.lifetime_parameters_by_name_.GetMapping()) {
    hash = hash_combine(hash, DenseMapInfo<llvm::StringRef>::getHashValue(
                                  lifetime_arg.first()));
    hash =
        hash_combine(hash, DenseMapInfo<devtools_rust::Lifetime>::getHashValue(
                               lifetime_arg.second));
  }
  return hash_combine(
      hash, DenseMapInfo<clang::QualType>::getHashValue(value_lifetimes.type_));
}

}  // namespace llvm
