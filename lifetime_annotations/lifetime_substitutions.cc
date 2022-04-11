// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#include "lifetime_annotations/lifetime_substitutions.h"

#include <algorithm>
#include <string>

#include "third_party/absl/strings/str_join.h"
#include "third_party/llvm/llvm-project/llvm/include/llvm/Support/raw_ostream.h"

namespace devtools_rust {

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
    parts.push_back(
        absl::StrCat(from.DebugString(), " -> ", Substitute(to).DebugString()));
  }
  std::sort(parts.begin(), parts.end());
  llvm::errs() << absl::StrJoin(parts, ", ") << "\n";
}

}  // namespace devtools_rust
