// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "lifetime_annotations/function_lifetimes.h"

#include <string>

#include "absl/strings/str_cat.h"
#include "absl/strings/str_join.h"
#include "lifetime_annotations/lifetime.h"
#include "lifetime_annotations/type_lifetimes.h"
#include "clang/AST/DeclCXX.h"
#include "clang/AST/Type.h"
#include "clang/Basic/LLVM.h"
#include "llvm/ADT/SmallVector.h"
#include "llvm/Support/Error.h"

namespace clang {
namespace tidy {
namespace lifetimes {

llvm::Expected<FunctionLifetimes> FunctionLifetimes::CreateForDecl(
    const clang::FunctionDecl* func,
    const FunctionLifetimeFactory& lifetime_factory) {
  clang::QualType this_type;
  if (auto method = clang::dyn_cast<clang::CXXMethodDecl>(func);
      method && !method->isStatic()) {
    this_type = method->getThisType();
  }
  clang::TypeLoc type_loc;
  if (func->getTypeSourceInfo()) {
    type_loc = func->getTypeSourceInfo()->getTypeLoc();
  }
  return Create(func->getType()->getAs<clang::FunctionProtoType>(), type_loc,
                this_type, lifetime_factory);
}

llvm::Expected<FunctionLifetimes> FunctionLifetimes::CreateForFunctionType(
    const clang::FunctionProtoType* func, clang::TypeLoc func_type_loc,
    const FunctionLifetimeFactory& lifetime_factory) {
  return Create(func, func_type_loc, clang::QualType(), lifetime_factory);
}

llvm::Expected<FunctionLifetimes> FunctionLifetimes::CreateForFunctionType(
    const clang::FunctionProtoType* func,
    const FunctionLifetimeFactory& lifetime_factory) {
  return CreateForFunctionType(func, clang::TypeLoc(), lifetime_factory);
}

llvm::Expected<FunctionLifetimes> FunctionLifetimes::CreateCopy(
    const LifetimeFactory& factory) const {
  FunctionLifetimes ret;

  if (this_lifetimes_.has_value()) {
    ValueLifetimes tmp;
    if (llvm::Error err =
            ValueLifetimes::Create(this_lifetimes_->Type(), factory)
                .moveInto(tmp)) {
      return std::move(err);
    }
    ret.this_lifetimes_ = std::move(tmp);
  }

  ret.param_lifetimes_.reserve(param_lifetimes_.size());
  for (size_t i = 0; i < param_lifetimes_.size(); i++) {
    ValueLifetimes tmp;
    if (llvm::Error err =
            ValueLifetimes::Create(param_lifetimes_[i].Type(), factory)
                .moveInto(tmp)) {
      return std::move(err);
    }
    ret.param_lifetimes_.push_back(std::move(tmp));
  }

  if (llvm::Error err =
          ValueLifetimes::Create(return_lifetimes_.Type(), factory)
              .moveInto(ret.return_lifetimes_)) {
    return std::move(err);
  }

  return ret;
}

FunctionLifetimes FunctionLifetimes::ForOverriddenMethod(
    const clang::CXXMethodDecl* method) {
  FunctionLifetimes ret = *this;
  assert(this_lifetimes_.has_value());
  ret.this_lifetimes_ = ValueLifetimes::ForPointerLikeType(
      method->getThisType(),
      this_lifetimes_.value().GetPointeeLifetimes().GetFieldOrBaseLifetimes(
          method->getThisObjectType(), {}));
  assert(ret.IsValidForDecl(method));
  return ret;
}

bool FunctionLifetimes::IsValidForDecl(const clang::FunctionDecl* function) {
  // TODO(veluca): also validate the types of the arguments, and/or the type of
  // the function itself.
  if (auto method = clang::dyn_cast<clang::CXXMethodDecl>(function);
      method && !method->isStatic()) {
    if (!this_lifetimes_.has_value()) return false;
  }
  return param_lifetimes_.size() == function->param_size();
}

llvm::Expected<FunctionLifetimes> FunctionLifetimes::Create(
    const clang::FunctionProtoType* type, clang::TypeLoc type_loc,
    const clang::QualType this_type,
    const FunctionLifetimeFactory& lifetime_factory) {
  FunctionLifetimes ret;

  llvm::SmallVector<const clang::Attr*> attrs;
  if (type_loc) {
    StripAttributes(type_loc, attrs);
  }
  llvm::SmallVector<const clang::Expr*> lifetime_names =
      GetAttributeLifetimes(attrs);

  if (this_type.isNull() && !lifetime_names.empty()) {
    return llvm::createStringError(
        llvm::inconvertibleErrorCode(),
        absl::StrCat("Encountered a `this` lifetime on a function with no "
                     "`this` parameter"));
  }

  if (!this_type.isNull()) {
    ValueLifetimes tmp;
    const clang::Expr* lifetime_name = nullptr;
    if (!lifetime_names.empty()) {
      if (lifetime_names.size() != 1) {
        return llvm::createStringError(
            llvm::inconvertibleErrorCode(),
            absl::StrCat("Expected a single lifetime but ",
                         lifetime_names.size(), " were given"));
      }
      lifetime_name = lifetime_names.front();
    }
    if (llvm::Error err =
            lifetime_factory.CreateThisLifetimes(this_type, lifetime_name)
                .moveInto(tmp)) {
      return std::move(err);
    }
    ret.this_lifetimes_ = std::move(tmp);
  }

  clang::FunctionTypeLoc func_type_loc;
  if (type_loc) {
    func_type_loc = type_loc.getAsAdjusted<clang::FunctionTypeLoc>();
  }
  ret.param_lifetimes_.reserve(type->getNumParams());
  for (size_t i = 0; i < type->getNumParams(); i++) {
    clang::TypeLoc param_type_loc;
    if (type_loc) {
      const clang::ParmVarDecl* param = func_type_loc.getParam(i);
      if (param && param->getTypeSourceInfo()) {
        param_type_loc = param->getTypeSourceInfo()->getTypeLoc();
      }
    }
    ValueLifetimes tmp;
    if (llvm::Error err =
            lifetime_factory
                .CreateParamLifetimes(type->getParamType(i), param_type_loc)
                .moveInto(tmp)) {
      return std::move(err);
    }
    ret.param_lifetimes_.push_back(std::move(tmp));
  }

  clang::TypeLoc return_type_loc;
  if (func_type_loc) {
    return_type_loc = func_type_loc.getReturnLoc();
  }
  if (llvm::Error err =
          lifetime_factory
              .CreateReturnLifetimes(type->getReturnType(), return_type_loc,
                                     ret.param_lifetimes_, ret.this_lifetimes_)
              .moveInto(ret.return_lifetimes_)) {
    return std::move(err);
  }

  return ret;
}

bool FunctionLifetimes::HasAny(
    const std::function<bool(Lifetime)>& predicate) const {
  return std::any_of(param_lifetimes_.begin(), param_lifetimes_.end(),
                     [&predicate](const ValueLifetimes& v) {
                       return v.HasAny(predicate);
                     }) ||
         return_lifetimes_.HasAny(predicate) ||
         (this_lifetimes_.has_value() && this_lifetimes_->HasAny(predicate));
}

llvm::DenseSet<Lifetime> FunctionLifetimes::AllFreeLifetimes() const {
  // TODO(veluca): this is incorrect in the presence of HRTBs.
  llvm::DenseSet<Lifetime> all_lifetimes;
  Traverse([&all_lifetimes](Lifetime l, Variance) {
    if (l != Lifetime::Static()) {
      all_lifetimes.insert(l);
    }
  });
  return all_lifetimes;
}

void FunctionLifetimes::SubstituteLifetimes(
    const LifetimeSubstitutions& subst) {
  // TODO(veluca): this is incorrect in the presence of HRTBs.
  std::for_each(param_lifetimes_.begin(), param_lifetimes_.end(),
                [&subst](ValueLifetimes& v) { v.SubstituteLifetimes(subst); });
  return_lifetimes_.SubstituteLifetimes(subst);
  if (this_lifetimes_.has_value()) {
    this_lifetimes_->SubstituteLifetimes(subst);
  }
}

void FunctionLifetimes::Traverse(
    std::function<void(Lifetime&, Variance)> visitor) {
  for (auto& param : param_lifetimes_) {
    param.Traverse(visitor);
  }
  return_lifetimes_.Traverse(visitor);
  if (this_lifetimes_.has_value()) {
    this_lifetimes_->Traverse(visitor);
  }
}

void FunctionLifetimes::Traverse(
    std::function<void(const Lifetime&, Variance)> visitor) const {
  const_cast<FunctionLifetimes*>(this)->Traverse(
      [visitor](Lifetime& l, Variance v) { visitor(l, v); });
}

std::string FunctionLifetimes::DebugString(LifetimeFormatter formatter) const {
  std::vector<std::string> formatted_param_lifetimes;

  // Add parenteses to non-trivial nested lifetimes, i.e. fn parameters with >1
  // lifetimes, as their DebugString does not contain parentheses.
  auto maybe_add_parentheses = [&](std::string s) {
    if (s.find_first_of(",()") != std::string::npos || s.empty()) {
      return absl::StrCat("(", s, ")");
    }
    return s;
  };

  for (const auto& param : param_lifetimes_) {
    formatted_param_lifetimes.push_back(
        maybe_add_parentheses(param.DebugString(formatter)));
  }

  std::string result;
  if (this_lifetimes_.has_value()) {
    result = absl::StrCat(
        maybe_add_parentheses(this_lifetimes_->DebugString(formatter)), ":");
  }
  if (!result.empty() && !formatted_param_lifetimes.empty()) {
    absl::StrAppend(&result, " ");
  }
  absl::StrAppend(&result, absl::StrJoin(formatted_param_lifetimes, ", "));

  if (return_lifetimes_.HasLifetimes()) {
    if (!result.empty()) {
      absl::StrAppend(&result, " ");
    }
    absl::StrAppend(
        &result, "-> ",
        maybe_add_parentheses(return_lifetimes_.DebugString(formatter)));
  }

  return result;
}

std::ostream& operator<<(std::ostream& os,
                         const FunctionLifetimes& func_lifetimes) {
  return os << func_lifetimes.DebugString();
}

}  // namespace lifetimes
}  // namespace tidy
}  // namespace clang
