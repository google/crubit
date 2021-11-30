// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "lifetime_annotations/lifetime_symbol_table.h"

#include <optional>

namespace devtools_rust {

std::optional<Lifetime> LifetimeSymbolTable::LookupName(
    llvm::StringRef name) const {
  if (name == "static") {
    return Lifetime::Static();
  }

  auto iter = name_to_lifetime_.find(name);
  if (iter == name_to_lifetime_.end()) {
    return std::nullopt;
  }
  return iter->second;
}

Lifetime LifetimeSymbolTable::LookupNameAndMaybeDeclare(llvm::StringRef name) {
  if (name == "static") {
    return Lifetime::Static();
  }

  auto [iter, inserted] =
      name_to_lifetime_.try_emplace(name, Lifetime::Static());
  if (inserted) {
    iter->second = Lifetime::CreateVariable();
  }
  return iter->second;
}

}  // namespace devtools_rust
