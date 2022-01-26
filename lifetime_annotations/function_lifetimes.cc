// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "lifetime_annotations/function_lifetimes.h"

#include <string>

#include "lifetime_annotations/lifetime.h"
#include "lifetime_annotations/type_lifetimes.h"
#include "third_party/absl/strings/str_cat.h"
#include "third_party/absl/strings/str_join.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/DeclCXX.h"
#include "third_party/llvm/llvm-project/clang/include/clang/Basic/LLVM.h"

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

bool FunctionLifetimes::ContainsLifetimes() {
  for (const auto& param : param_lifetimes) {
    if (!param.empty()) {
      return true;
    }
  }

  return !return_lifetimes.empty() || !this_lifetimes.empty();
}

bool FunctionLifetimes::IsIsomorphic(const FunctionLifetimes& other) const {
  // We expect this function to only be called for 2 FunctionLifetime objects
  // that are for the same function, thus the number of parameters, and the
  // number of Lifetimes for each type, should always match.
  assert(param_lifetimes.size() == other.param_lifetimes.size());
  assert(this_lifetimes.size() == other.this_lifetimes.size());
  assert(return_lifetimes.size() == other.return_lifetimes.size());
  for (size_t i = 0; i < param_lifetimes.size(); ++i) {
    assert(param_lifetimes[i].size() == other.param_lifetimes[i].size());
  }

  // Map of equivalent lifetimes between `*this` and `other`.
  LifetimeBijection bijection;

  auto compare_type_lifetimes = [&bijection](const TypeLifetimes& a,
                                             const TypeLifetimes& b) {
    assert(a.size() == b.size());
    for (size_t i = 0; i < a.size(); ++i) {
      if (!bijection.Add(a[i], b[i])) {
        return false;
      }
    }
    return true;
  };

  if (!compare_type_lifetimes(return_lifetimes, other.return_lifetimes)) {
    return false;
  }
  if (!compare_type_lifetimes(this_lifetimes, other.this_lifetimes)) {
    return false;
  }
  for (size_t i = 0; i < param_lifetimes.size(); ++i) {
    if (!compare_type_lifetimes(param_lifetimes[i], other.param_lifetimes[i]))
      return false;
  }
  return true;
}

std::string FunctionLifetimes::DebugString(LifetimeFormatter formatter) const {
  std::vector<std::string> formatted_param_lifetimes;

  for (const auto& param : param_lifetimes) {
    formatted_param_lifetimes.push_back(
        ::devtools_rust::DebugString(param, formatter));
  }

  std::string result;
  if (!this_lifetimes.empty()) {
    result = absl::StrCat(
        ::devtools_rust::DebugString(this_lifetimes, formatter), ":");
  }
  if (!result.empty() && !formatted_param_lifetimes.empty()) {
    absl::StrAppend(&result, " ");
  }
  absl::StrAppend(&result, absl::StrJoin(formatted_param_lifetimes, ", "));

  if (!return_lifetimes.empty()) {
    if (!result.empty()) {
      absl::StrAppend(&result, " ");
    }
    absl::StrAppend(&result, "-> ",
                    ::devtools_rust::DebugString(return_lifetimes, formatter));
  }

  return result;
}

bool FunctionLifetimes::Validate(const clang::FunctionDecl* func) const {
  if (auto method = clang::dyn_cast<clang::CXXMethodDecl>(func)) {
    if (CreateLifetimesForType(method->getThisType(), Lifetime::CreateLocal)
            .size() != this_lifetimes.size()) {
      return false;
    }
  } else if (!this_lifetimes.empty()) {
    return false;
  }
  if (CreateLifetimesForType(func->getReturnType(), Lifetime::CreateLocal)
          .size() != return_lifetimes.size()) {
    return false;
  }
  if (param_lifetimes.size() != func->getNumParams()) {
    return false;
  }
  for (size_t i = 0; i < param_lifetimes.size(); i++) {
    if (CreateLifetimesForType(func->getParamDecl(i)->getType(),
                               Lifetime::CreateLocal)
            .size() != param_lifetimes[i].size()) {
      return false;
    }
  }
  return true;
}

std::ostream& operator<<(std::ostream& os,
                         const FunctionLifetimes& func_lifetimes) {
  return os << func_lifetimes.DebugString();
}

}  // namespace devtools_rust
