// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "lifetime_annotations/lifetime_substitutions.h"

#include <algorithm>
#include <cassert>
#include <string>
#include <vector>

#include "lifetime_annotations/lifetime.h"
#include "llvm/ADT/StringExtras.h"
#include "llvm/ADT/Twine.h"
#include "llvm/Support/raw_ostream.h"

namespace clang {
namespace tidy {
namespace lifetimes {

void LifetimeSubstitutions::Add(Lifetime variable, Lifetime substitution) {
  assert(variable.IsVariable());

  // Resolve any existing substitutions now so Substitute() has less work to do
  // later.
  substitution = Substitute(substitution);

  // Don't substitute a variable by itself. This prevents redundancy and, more
  // importantly, prevents Substitute() from going into an infinite loop.
  if (variable == substitution) {
    return;
  }

  substitutions_[variable] = substitution;
}

Lifetime LifetimeSubstitutions::Substitute(Lifetime l) const {
  while (true) {
    auto iter = substitutions_.find(l);
    if (iter == substitutions_.end()) {
      return l;
    }
    l = iter->second;
  }
}

void LifetimeSubstitutions::Dump() const {
  std::vector<std::string> parts;
  for (auto [from, to] : substitutions_) {
    parts.push_back((llvm::Twine(from.DebugString()) + llvm::Twine(" -> ") +
                     llvm::Twine(Substitute(to).DebugString()))
                        .str());
  }
  std::sort(parts.begin(), parts.end());
  llvm::errs() << llvm::join(parts, ", ") << "\n";
}

}  // namespace lifetimes
}  // namespace tidy
}  // namespace clang
