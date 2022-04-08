// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "lifetime_annotations/function_lifetimes.h"

#include <string>

#include "third_party/absl/strings/str_cat.h"
#include "third_party/absl/strings/str_join.h"
#include "lifetime_annotations/lifetime.h"
#include "lifetime_annotations/type_lifetimes.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/DeclCXX.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/Type.h"
#include "third_party/llvm/llvm-project/clang/include/clang/Basic/LLVM.h"
#include "third_party/llvm/llvm-project/llvm/include/llvm/ADT/SmallVector.h"
#include "third_party/llvm/llvm-project/llvm/include/llvm/Support/Error.h"

namespace devtools_rust {

namespace {
// Track a bijective mapping between 2 sets of Lifetimes.
class LifetimeBijection {
 public:
  // Returns true if the bidirectional mapping between lifetimes could be added
  // to the bijection, or is already present. Returns false if the lifetimes
  // conflict with other mappings already recorded in the bijection.
  bool Add(Lifetime lifetime_in_a, Lifetime lifetime_in_b) {
    auto [a_to_b_iter, a_to_b_inserted] =
        a_to_b_.try_emplace(lifetime_in_a, lifetime_in_b);
    if (!a_to_b_inserted) {
      return a_to_b_iter->second == lifetime_in_b;
    }
    auto [_, b_to_a_inserted] =
        b_to_a_.try_emplace(lifetime_in_b, lifetime_in_a);
    return b_to_a_inserted;
  }

 private:
  llvm::DenseMap<Lifetime, Lifetime> a_to_b_;
  llvm::DenseMap<Lifetime, Lifetime> b_to_a_;
};

}  // namespace

llvm::Expected<FunctionLifetimes> FunctionLifetimes::CreateForDecl(
    const clang::FunctionDecl* func,
    const FunctionLifetimeFactory& lifetime_factory) {
  clang::QualType this_type;
  if (auto method = clang::dyn_cast<clang::CXXMethodDecl>(func);
      method && !method->isStatic()) {
    this_type = method->getThisType();
  }
  return Create(func->getType()->getAs<clang::FunctionProtoType>(), this_type,
                lifetime_factory);
}

llvm::Expected<FunctionLifetimes> FunctionLifetimes::CreateForFunctionType(
    const clang::FunctionProtoType* func,
    const FunctionLifetimeFactory& lifetime_factory) {
  return Create(func, clang::QualType(), lifetime_factory);
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
    const clang::FunctionProtoType* type, const clang::QualType this_type,
    const FunctionLifetimeFactory& lifetime_factory) {
  FunctionLifetimes ret;

  if (!this_type.isNull()) {
    ValueLifetimes tmp;
    if (llvm::Error err =
            ValueLifetimes::Create(this_type, [&](clang::QualType type,
                                                  llvm::StringRef param) {
              return lifetime_factory.CreateParamLifetime(type, param);
            }).moveInto(tmp)) {
      return std::move(err);
    }
    ret.this_lifetimes_ = std::move(tmp);
  }

  ret.param_lifetimes_.reserve(type->getNumParams());
  for (size_t i = 0; i < type->getNumParams(); i++) {
    ValueLifetimes tmp;
    if (llvm::Error err = ValueLifetimes::Create(
                              type->getParamType(i),
                              [&](clang::QualType type, llvm::StringRef param) {
                                return lifetime_factory.CreateParamLifetime(
                                    type, param);
                              })
                              .moveInto(tmp)) {
      return std::move(err);
    }
    ret.param_lifetimes_.push_back(std::move(tmp));
  }

  if (llvm::Error err = ValueLifetimes::Create(
                            type->getReturnType(),
                            [&](clang::QualType type, llvm::StringRef param) {
                              return lifetime_factory.CreateReturnLifetime(
                                  type, param, ret.param_lifetimes_,
                                  ret.this_lifetimes_);
                            })
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

bool FunctionLifetimes::IsIsomorphic(const FunctionLifetimes& other) const {
  // We expect this function to only be called for 2 FunctionLifetime objects
  // that are for the same function, thus the number of parameters, and the
  // number of Lifetimes for each type, should always match.
  assert(param_lifetimes_.size() == other.param_lifetimes_.size());
  assert(this_lifetimes_.has_value() == other.this_lifetimes_.has_value());

  // Map of equivalent lifetimes between `*this` and `other`.
  LifetimeBijection bijection;

  llvm::SmallVector<Lifetime> my_lifetimes;
  Traverse(
      [&my_lifetimes](Lifetime l, Variance) { my_lifetimes.push_back(l); });
  llvm::SmallVector<Lifetime> other_lifetimes;
  other.Traverse([&other_lifetimes](Lifetime l, Variance) {
    other_lifetimes.push_back(l);
  });

  assert(my_lifetimes.size() == other_lifetimes.size());
  for (size_t i = 0; i < my_lifetimes.size(); ++i) {
    if (!bijection.Add(my_lifetimes[i], other_lifetimes[i])) {
      return false;
    }
  }
  return true;
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

}  // namespace devtools_rust
