// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "lifetime_annotations/type_lifetimes.h"

#include <algorithm>
#include <string>

#include "third_party/absl/strings/str_format.h"
#include "third_party/absl/strings/str_join.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/DeclTemplate.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/TemplateBase.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/Type.h"

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

TypeLifetimes CreateLifetimesForType(
    clang::QualType type, std::function<Lifetime()> lifetime_factory) {
  assert(!type.isNull());
  TypeLifetimes ret;
  // Add implicit lifetime parameters for type template parameters.
  // TODO(veluca): we'll need to handle explicit lifetime parameters too.
  llvm::ArrayRef<clang::TemplateArgument> template_args = GetTemplateArgs(type);
  if (!template_args.empty()) {
    for (const clang::TemplateArgument& arg : template_args) {
      if (arg.getKind() == clang::TemplateArgument::Type) {
        ret.append(CreateLifetimesForType(arg.getAsType(), lifetime_factory));
      }
    }
    return ret;
  }
  clang::QualType pointee = type->getPointeeType();
  if (pointee.isNull()) return {};
  ret = CreateLifetimesForType(pointee, lifetime_factory);
  ret.push_back(lifetime_factory());
  return ret;
}

ObjectLifetimes ObjectLifetimes::FromTypeLifetimes(
    TypeLifetimesRef& type_lifetimes, clang::QualType type) {
  assert(!type_lifetimes.empty());
  assert(!type.isNull());
  ObjectLifetimes ret;
  ret.type_ = type;
  ret.lifetime_ = type_lifetimes.back();
  type_lifetimes = type_lifetimes.drop_back();
  ret.value_lifetimes_ =
      ValueLifetimes::FromTypeLifetimes(type_lifetimes, type);
  return ret;
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

ObjectLifetimes ObjectLifetimes::GetRecordObjectLifetimes(
    clang::QualType type) const {
  assert(type_->isRecordType());
  ObjectLifetimes ret;

  // The object of the `type` (i.e. field or a base class) basically has the
  // same lifetime as the struct.
  // TODO(veluca): this needs adaptation to lifetime parameters.
  ret.lifetime_ = lifetime_;
  ret.type_ = type;

  // `type` is one of a template argument, a struct, a pointer, or a type
  // with no lifetimes (other than its own).

  // First case: template argument. We just attach the
  // template argument's lifetimes to the leaf ObjectLifetimes.
  if (auto targ = type->getAs<clang::SubstTemplateTypeParmType>()) {
    const std::optional<ValueLifetimes>& arg_lifetimes =
        value_lifetimes_.GetTemplateArgumentLifetimes(targ);
    if (!arg_lifetimes) {
      assert(false);
      return {};
    }
    ret.value_lifetimes_ = *arg_lifetimes;
  } else if (type->isStructureOrClassType()) {
    // Second case: struct. We need to construct potentally reshuffled
    // template arguments, if the struct is a template.
    for (const clang::TemplateArgument& arg : GetTemplateArgs(type)) {
      if (arg.getKind() == clang::TemplateArgument::Type) {
        // TODO(veluca): handle new template arguments correctly; see also
        // the StructNoTemplateInnerTemplate test.
        // For now, clang::cast will assert-fail; we need to instead create a
        // sequence of lifetimes of the same lifetime as the struct as new
        // template arguments.
        auto templ_arg =
            clang::cast<clang::SubstTemplateTypeParmType>(arg.getAsType());
        ret.value_lifetimes_.template_argument_lifetimes_.push_back(
            value_lifetimes_.GetTemplateArgumentLifetimes(templ_arg));
      } else {
        ret.value_lifetimes_.template_argument_lifetimes_.push_back(
            std::nullopt);
      }
    }
    // TODO(veluca): handle potentially reshuffled lifetime parameters.
  } else if (!type->getPointeeType().isNull()) {
    // Third case: pointer.
    ret.value_lifetimes_.pointee_lifetimes_ = std::make_unique<ObjectLifetimes>(
        GetRecordObjectLifetimes(type->getPointeeType()));
  }

  return ret;
}

const ObjectLifetimes& ObjectLifetimes::GetPointeeLifetimes() const {
  assert(!type_->getPointeeType().isNull());
  return *value_lifetimes_.pointee_lifetimes_;
}

std::string ObjectLifetimes::ValueLifetimes::DebugString() const {
  std::string ret;
  if (!template_argument_lifetimes_.empty()) {
    std::vector<std::string> tmpl_arg_strings;
    for (const std::optional<ObjectLifetimes::ValueLifetimes>& tmpl_arg :
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

// Here, `type_lifetimes` are the lifetimes of a prvalue of the given `type`,
// unlike ObjectLifetimes::FromTypeLifetimes, which assumes a glvalue.
ObjectLifetimes::ValueLifetimes
ObjectLifetimes::ValueLifetimes::FromTypeLifetimes(
    TypeLifetimesRef& type_lifetimes, clang::QualType type) {
  assert(!type.isNull());

  ObjectLifetimes::ValueLifetimes ret;

  llvm::ArrayRef<clang::TemplateArgument> template_args = GetTemplateArgs(type);
  if (!template_args.empty()) {
    // Since we are simulating reversing a post-order visit, we need to
    // extract template arguments in reverse order.
    for (size_t i = template_args.size(); i-- > 0;) {
      const clang::TemplateArgument& arg = template_args[i];
      if (arg.getKind() == clang::TemplateArgument::Type) {
        ret.template_argument_lifetimes_.push_back(
            FromTypeLifetimes(type_lifetimes, arg.getAsType()));
      } else {
        ret.template_argument_lifetimes_.push_back(std::nullopt);
      }
    }
    std::reverse(ret.template_argument_lifetimes_.begin(),
                 ret.template_argument_lifetimes_.end());
    return ret;
  }

  clang::QualType pointee_type = type->getPointeeType();
  if (!pointee_type.isNull()) {
    ret.pointee_lifetimes_ = std::make_unique<ObjectLifetimes>(
        ObjectLifetimes::FromTypeLifetimes(type_lifetimes, pointee_type));
  }
  return ret;
}

}  // namespace devtools_rust

namespace llvm {

bool DenseMapInfo<devtools_rust::ObjectLifetimes>::isEqual(
    const devtools_rust::ObjectLifetimes::ValueLifetimes& lhs,
    const devtools_rust::ObjectLifetimes::ValueLifetimes& rhs) {
  if ((lhs.pointee_lifetimes_ == nullptr) !=
      (rhs.pointee_lifetimes_ == nullptr)) {
    return false;
  }
  if (lhs.pointee_lifetimes_ &&
      !isEqual(*lhs.pointee_lifetimes_, *rhs.pointee_lifetimes_)) {
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
  return true;
}

unsigned DenseMapInfo<devtools_rust::ObjectLifetimes>::getHashValue(
    const devtools_rust::ObjectLifetimes::ValueLifetimes& lifetime_node) {
  llvm::hash_code hash = 0;
  if (lifetime_node.pointee_lifetimes_) {
    hash = getHashValue(*lifetime_node.pointee_lifetimes_);
  }
  for (const auto& tmpl_lifetime : lifetime_node.template_argument_lifetimes_) {
    if (tmpl_lifetime) {
      hash = hash_combine(hash, getHashValue(*tmpl_lifetime));
    }
  }
  return hash;
}

}  // namespace llvm
