// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "lifetime_analysis/lifetime_constraints.h"

#include "lifetime_annotations/lifetime_substitutions.h"

namespace clang {
namespace tidy {
namespace lifetimes {

clang::dataflow::LatticeJoinEffect LifetimeConstraints::join(
    const LifetimeConstraints& other) {
  bool changed = false;
  for (auto p : other.outlives_constraints_) {
    changed |= outlives_constraints_.insert(p).second;
  }
  return changed ? clang::dataflow::LatticeJoinEffect::Changed
                 : clang::dataflow::LatticeJoinEffect::Unchanged;
}

llvm::Error LifetimeConstraints::ApplyToFunctionLifetimes(
    FunctionLifetimes& function_lifetimes) {
  // Since we do not support "outlives" annotations, we can simply
  // find all the *connected* components on an undirected graph where there is
  // an edge between lifetime a and lifetime b iff there is a constraint in
  // either direction between a and b, ignoring `static` and `local` (which are
  // handled separately).
  // TODO(veluca): do this properly by collapsing SCCs and analyzing the
  // resulting outlives DAG if/when we support "outlives" annotations.
  llvm::DenseMap<Lifetime, llvm::SmallVector<Lifetime>> outlives_edges;
  llvm::DenseSet<Lifetime> all_lifetimes;
  // CCs that contain any of these lifetimes must be substituted with `$static`.
  llvm::DenseSet<Lifetime> outlives_static;
  llvm::DenseMap<Lifetime, Lifetime> is_outlived_by_local;
  for (auto [shorter, longer] : outlives_constraints_) {
    all_lifetimes.insert(longer);
    all_lifetimes.insert(shorter);
    if (shorter.IsVariable() && longer.IsVariable()) {
      outlives_edges[longer].push_back(shorter);
      outlives_edges[shorter].push_back(longer);
    }
    if (shorter == Lifetime::Static()) {
      outlives_static.insert(longer);
    }
    if (longer.IsLocal()) {
      is_outlived_by_local[shorter] = longer;
    }
  }

  llvm::DenseSet<Lifetime> visited;
  LifetimeSubstitutions substitutions;

  for (const auto& lifetime : all_lifetimes) {
    if (!lifetime.IsVariable() || visited.count(lifetime)) continue;

    llvm::SmallVector<Lifetime> connected_component;
    llvm::SmallVector<Lifetime> stack;
    stack.push_back(lifetime);
    bool cc_outlives_static = false;
    bool cc_is_outlived_by_local = false;
    Lifetime local_outliving_lifetime;
    while (!stack.empty()) {
      Lifetime cur = stack.back();
      stack.pop_back();
      if (visited.count(cur)) continue;
      visited.insert(cur);
      if (outlives_static.count(cur)) {
        cc_outlives_static = true;
      }
      if (is_outlived_by_local.count(cur)) {
        cc_is_outlived_by_local = true;
        local_outliving_lifetime = is_outlived_by_local[cur];
      }
      connected_component.push_back(cur);
      for (auto next : outlives_edges[cur]) {
        stack.push_back(next);
      }
    }
    if (cc_outlives_static && cc_is_outlived_by_local) {
      return llvm::createStringError(llvm::inconvertibleErrorCode(),
                                     "Function assigns local to static");
    }
    Lifetime representative = cc_outlives_static ? Lifetime::Static()
                              : cc_is_outlived_by_local
                                  ? local_outliving_lifetime
                                  : lifetime;

    for (Lifetime memb : connected_component) {
      substitutions.Add(memb, representative);
    }
  }
  function_lifetimes.SubstituteLifetimes(substitutions);

  return llvm::Error::success();
}

}  // namespace lifetimes
}  // namespace tidy
}  // namespace clang
