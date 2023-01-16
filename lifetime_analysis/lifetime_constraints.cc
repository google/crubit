// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "lifetime_analysis/lifetime_constraints.h"

#include <llvm/ADT/DenseSet.h>

#include <algorithm>

#include "lifetime_annotations/lifetime_substitutions.h"

namespace clang {
namespace tidy {
namespace lifetimes {

clang::dataflow::LatticeJoinEffect LifetimeConstraints::join(
    const LifetimeConstraints &other) {
  bool changed = false;
  for (auto p : other.outlives_constraints_) {
    changed |= outlives_constraints_.insert(p).second;
  }
  return changed ? clang::dataflow::LatticeJoinEffect::Changed
                 : clang::dataflow::LatticeJoinEffect::Unchanged;
}

namespace {

// Simple Disjoint-Set-Union with path compression (but no union-by-rank). This
// guarantees O(log n) time per operation.
class LifetimeDSU {
 public:
  void MakeSet(Lifetime l) { parent_[l] = l; }
  Lifetime Find(Lifetime l) {
    if (l == parent_[l]) return l;
    return parent_[l] = Find(parent_[l]);
  }
  void Union(Lifetime a, Lifetime b) {
    a = Find(a);
    b = Find(b);
    if (a != b) {
      parent_[a] = b;
    }
  }

 private:
  llvm::DenseMap<Lifetime, Lifetime> parent_;
};

}  // namespace

llvm::Error LifetimeConstraints::ApplyToFunctionLifetimes(
    FunctionLifetimes &function_lifetimes) {
  // We want to make output-only lifetimes as long as possible; thus, we collect
  // those separately.
  llvm::DenseSet<Lifetime> output_lifetimes;
  function_lifetimes.GetReturnLifetimes().Traverse(
      [&output_lifetimes](Lifetime l, Variance) {
        output_lifetimes.insert(l);
      });

  // Collect all "interesting" lifetimes, i.e. all lifetimes that appear in the
  // function call.
  llvm::DenseSet<Lifetime> all_lifetimes;
  function_lifetimes.Traverse(
      [&all_lifetimes](Lifetime l, Variance) { all_lifetimes.insert(l); });

  // Compute the set of static, input or local lifetimes that must outlive the
  // given lifetime (excluding the lifetime itself).
  // This function ignores constraints of the form 'a <= 'static, as "outlived
  // by 'static" is not a meaningful constraint.
  // TODO(veluca): here we could certainly reduce complexity, for example by
  // constructing the constraint graph instead of iterating over all constraints
  // each time.
  auto get_outliving_lifetimes = [&](Lifetime l) {
    std::vector<Lifetime> stack{l};
    llvm::DenseSet<Lifetime> visited;
    while (!stack.empty()) {
      Lifetime v = stack.back();
      stack.pop_back();
      if (visited.contains(v)) continue;
      visited.insert(v);
      for (auto [shorter, longer] : outlives_constraints_) {
        if (shorter == v && longer != Lifetime::Static()) {
          stack.push_back(longer);
        }
      }
    }
    visited.erase(l);
    return visited;
  };

  LifetimeSubstitutions substitutions;

  // Keep track of which lifetimes already have their final substitutions
  // computed.
  llvm::DenseSet<Lifetime> already_have_substitutions;

  // First of all, substitute everything that outlives 'static with 'static.
  for (Lifetime outlives_static : get_outliving_lifetimes(Lifetime::Static())) {
    if (outlives_static.IsLocal()) {
      return llvm::createStringError(llvm::inconvertibleErrorCode(),
                                     "Function assigns local to static");
    }
    already_have_substitutions.insert(outlives_static);
    substitutions.Add(outlives_static, Lifetime::Static());
  }

  LifetimeDSU dsu;
  dsu.MakeSet(Lifetime::Static());
  for (Lifetime lifetime : all_lifetimes) {
    dsu.MakeSet(lifetime);
  }

  for (Lifetime lifetime : all_lifetimes) {
    llvm::DenseSet<Lifetime> longer_lifetimes =
        get_outliving_lifetimes(lifetime);
    assert(!longer_lifetimes.contains(Lifetime::Static()));


    // If constrained to be outlived by 'local, replace the lifetime with
    // 'local, or error out if 'static.
    auto local_it =
        std::find_if(longer_lifetimes.begin(), longer_lifetimes.end(),
                     [](Lifetime l) { return l.IsLocal(); });

    if (local_it != longer_lifetimes.end()) {
      substitutions.Add(lifetime, *local_it);
      already_have_substitutions.insert(lifetime);
      continue;
    }

    // Now all the longer lifetimes must be variable lifetimes. As we do not
    // support inequalities, we simply state that they must be equivalent.
    for (Lifetime longer : longer_lifetimes) {
      if (already_have_substitutions.contains(longer)) continue;
      dsu.Union(longer, lifetime);
    }
  }

  // Everything that is equivalent to 'static must be replaced by 'static, not
  // by an arbitrary lifetime in the equivalence set.
  Lifetime cc_of_static = dsu.Find(Lifetime::Static());

  for (Lifetime lifetime : all_lifetimes) {
    if (already_have_substitutions.contains(lifetime) ||
        !lifetime.IsVariable()) {
      continue;
    }

    Lifetime cc = dsu.Find(lifetime);

    if (cc == cc_of_static) {
      substitutions.Add(lifetime, Lifetime::Static());
    } else {
      substitutions.Add(lifetime, cc);
    }
  }

  function_lifetimes.SubstituteLifetimes(substitutions);

  return llvm::Error::success();
}

}  // namespace lifetimes
}  // namespace tidy
}  // namespace clang
