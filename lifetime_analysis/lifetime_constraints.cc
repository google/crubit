// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "lifetime_analysis/lifetime_constraints.h"

#include <llvm/ADT/DenseSet.h>

#include <algorithm>

#include "lifetime_annotations/lifetime.h"
#include "lifetime_annotations/lifetime_substitutions.h"
#include "lifetime_annotations/pointee_type.h"

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

llvm::DenseSet<Lifetime> LifetimeConstraints::GetOutlivingLifetimes(
    const Lifetime l) const {
  // TODO(veluca): here we could certainly reduce complexity, for example by
  // constructing the constraint graph instead of iterating over all constraints
  // each time.
  std::vector<Lifetime> stack{l};
  llvm::DenseSet<Lifetime> visited;
  while (!stack.empty()) {
    Lifetime v = stack.back();
    stack.pop_back();
    if (visited.contains(v)) continue;
    visited.insert(v);
    for (auto [shorter, longer] : outlives_constraints_) {
      if (shorter == v) {
        stack.push_back(longer);
      }
    }
  }
  visited.erase(l);
  return visited;
}

llvm::Error LifetimeConstraints::ApplyToFunctionLifetimes(
    FunctionLifetimes& function_lifetimes) {
  // We want to make output-only lifetimes as long as possible; thus, we collect
  // those separately.
  llvm::DenseSet<Lifetime> output_lifetimes;
  function_lifetimes.GetReturnLifetimes().Traverse(
      [&output_lifetimes](Lifetime l, Variance) {
        output_lifetimes.insert(l);
      });

  // Collect all "interesting" lifetimes, i.e. all lifetimes that appear in the
  // function call.
  llvm::DenseSet<Lifetime> all_interesting_lifetimes;
  function_lifetimes.Traverse(
      [&all_interesting_lifetimes](Lifetime l, Variance) {
        all_interesting_lifetimes.insert(l);
      });

  LifetimeSubstitutions substitutions;

  // Keep track of which lifetimes already have their final substitutions
  // computed.
  llvm::DenseSet<Lifetime> already_have_substitutions;

  // First of all, substitute everything that outlives 'static with 'static.
  for (Lifetime outlives_static : GetOutlivingLifetimes(Lifetime::Static())) {
    if (outlives_static.IsLocal()) {
      return llvm::createStringError(llvm::inconvertibleErrorCode(),
                                     "Function assigns local to static");
    }
    already_have_substitutions.insert(outlives_static);
    substitutions.Add(outlives_static, Lifetime::Static());
  }

  LifetimeDSU dsu;
  dsu.MakeSet(Lifetime::Static());
  for (Lifetime lifetime : all_interesting_lifetimes) {
    dsu.MakeSet(lifetime);
  }

  for (Lifetime lifetime : all_interesting_lifetimes) {
    llvm::DenseSet<Lifetime> longer_lifetimes = GetOutlivingLifetimes(lifetime);
    longer_lifetimes.erase(Lifetime::Static());

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
      if (!all_interesting_lifetimes.contains(longer)) continue;
      dsu.Union(longer, lifetime);
    }
  }

  // Everything that is equivalent to 'static must be replaced by 'static, not
  // by an arbitrary lifetime in the equivalence set.
  Lifetime cc_of_static = dsu.Find(Lifetime::Static());

  for (Lifetime lifetime : all_interesting_lifetimes) {
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

namespace {

enum LifetimeRequirement {
  kReplacementIsGe = 0x1,
  kReplacementIsLe = 0x2,
  kReplacementIsEq = 0x3,
};

// Computes the requirement corresponding to *composing* the two requirements
// together; for example, using a type containing a contravariant lifetime in
// contravariant position would result in a covariant lifetime.
// In general, this function behaves like multiplication where Ge = -1, Le = 1,
// Eq = 0.
LifetimeRequirement Compose(LifetimeRequirement a, LifetimeRequirement b) {
  if (a == LifetimeRequirement::kReplacementIsEq ||
      b == LifetimeRequirement::kReplacementIsEq) {
    return LifetimeRequirement::kReplacementIsEq;
  }
  if (a != b) {
    return LifetimeRequirement::kReplacementIsGe;
  }
  return LifetimeRequirement::kReplacementIsLe;
}

void AddConstraint(LifetimeRequirement req, Lifetime obj, Lifetime replacement,
                   LifetimeConstraints& constraints) {
  if (req & LifetimeRequirement::kReplacementIsLe) {
    constraints.AddOutlivesConstraint(replacement, obj);
  }
  if (req & LifetimeRequirement::kReplacementIsGe) {
    constraints.AddOutlivesConstraint(obj, replacement);
  }
}

void CollectLifetimeConstraints(const ValueLifetimes&, const ValueLifetimes&,
                                LifetimeRequirement, LifetimeConstraints&);

void CollectLifetimeConstraints(const FunctionLifetimes&,
                                const FunctionLifetimes&, LifetimeRequirement,
                                LifetimeConstraints&);

// Collects all the constraints that are required to use `replacement` as a
// replacement for `obj`, taking into account the requirements due to their
// positions (i.e. covariant/contravariant/invariant).
void CollectLifetimeConstraints(const ObjectLifetimes& obj,
                                const ObjectLifetimes& replacement,
                                LifetimeRequirement object_requirement,
                                LifetimeRequirement descendants_requirement,
                                LifetimeConstraints& constraints) {
  AddConstraint(object_requirement, obj.GetLifetime(),
                replacement.GetLifetime(), constraints);
  CollectLifetimeConstraints(obj.GetValueLifetimes(),
                             replacement.GetValueLifetimes(),
                             descendants_requirement, constraints);
}

void CollectLifetimeConstraints(const ValueLifetimes& obj,
                                const ValueLifetimes& replacement,
                                LifetimeRequirement requirement,
                                LifetimeConstraints& constraints) {
  assert(obj.Type().getCanonicalType() ==
         replacement.Type().getCanonicalType());
  if (!PointeeType(obj.Type()).isNull()) {
    LifetimeRequirement pointee_req =
        PointeeType(obj.Type()).isConstQualified()
            ? LifetimeRequirement::kReplacementIsLe
            : LifetimeRequirement::kReplacementIsEq;
    CollectLifetimeConstraints(obj.GetPointeeLifetimes(),
                               replacement.GetPointeeLifetimes(), requirement,
                               Compose(pointee_req, requirement), constraints);
  }
  if (obj.Type()->isRecordType()) {
    assert(obj.GetNumTemplateNestingLevels() ==
           replacement.GetNumTemplateNestingLevels());
    for (size_t depth = 0; depth < obj.GetNumTemplateNestingLevels(); depth++) {
      assert(obj.GetNumTemplateArgumentsAtDepth(depth) ==
             replacement.GetNumTemplateArgumentsAtDepth(depth));
      for (size_t idx = 0; idx < obj.GetNumTemplateArgumentsAtDepth(depth);
           idx++) {
        std::optional<ValueLifetimes> obj_arg =
            obj.GetTemplateArgumentLifetimes(depth, idx);
        std::optional<ValueLifetimes> replacement_arg =
            replacement.GetTemplateArgumentLifetimes(depth, idx);
        assert(obj_arg.has_value() == replacement_arg.has_value());
        if (obj_arg.has_value() && replacement_arg.has_value()) {
          CollectLifetimeConstraints(*obj_arg, *replacement_arg,
                                     LifetimeRequirement::kReplacementIsEq,
                                     constraints);
        }
      }
    }
    for (const auto& lftm_param : GetLifetimeParameters(obj.Type())) {
      // TODO(veluca): should lifetime parameters be invariant like template
      // parameters?
      AddConstraint(requirement, obj.GetLifetimeParameter(lftm_param),
                    replacement.GetLifetimeParameter(lftm_param), constraints);
    }
  }
  if (clang::isa<clang::FunctionProtoType>(obj.Type())) {
    CollectLifetimeConstraints(obj.GetFuncLifetimes(),
                               replacement.GetFuncLifetimes(), requirement,
                               constraints);
  }
}

void CollectLifetimeConstraints(const FunctionLifetimes& callable,
                                const FunctionLifetimes& replacement_callable,
                                LifetimeRequirement requirement,
                                LifetimeConstraints& constraints) {
  for (size_t i = 0; i < callable.GetNumParams(); i++) {
    CollectLifetimeConstraints(
        callable.GetParamLifetimes(i),
        replacement_callable.GetParamLifetimes(i),
        Compose(LifetimeRequirement::kReplacementIsGe, requirement),
        constraints);
  }
  CollectLifetimeConstraints(
      callable.GetReturnLifetimes(), replacement_callable.GetReturnLifetimes(),
      Compose(LifetimeRequirement::kReplacementIsLe, requirement), constraints);
  if (callable.IsNonStaticMethod()) {
    CollectLifetimeConstraints(
        callable.GetThisLifetimes(), replacement_callable.GetThisLifetimes(),
        Compose(LifetimeRequirement::kReplacementIsGe, requirement),
        constraints);
  }
}

}  // namespace

LifetimeConstraints LifetimeConstraints::ForCallableSubstitution(
    const FunctionLifetimes& callable,
    const FunctionLifetimes& replacement_callable) {
  LifetimeConstraints constraints =
      LifetimeConstraints::ForCallableSubstitutionFull(callable,
                                                       replacement_callable);

  llvm::DenseSet<Lifetime> all_lifetimes;
  callable.Traverse(
      [&all_lifetimes](Lifetime l, Variance) { all_lifetimes.insert(l); });

  LifetimeConstraints ret;
  for (auto l : all_lifetimes) {
    for (auto outliving : constraints.GetOutlivingLifetimes(l)) {
      if (all_lifetimes.contains(outliving)) {
        ret.AddOutlivesConstraint(l, outliving);
      }
    }
  }

  return ret;
}

LifetimeConstraints LifetimeConstraints::ForCallableSubstitutionFull(
    const FunctionLifetimes& callable,
    const FunctionLifetimes& replacement_callable) {
  LifetimeConstraints constraints;
  CollectLifetimeConstraints(callable, replacement_callable,
                             LifetimeRequirement::kReplacementIsLe,
                             constraints);
  return constraints;
}

}  // namespace lifetimes
}  // namespace tidy
}  // namespace clang
