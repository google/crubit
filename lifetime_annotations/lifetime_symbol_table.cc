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
    Lifetime lifetime = Lifetime::CreateVariable();
    iter->second = lifetime;
    assert(!lifetime_to_name_.count(lifetime));
    lifetime_to_name_[lifetime] = name;
  }
  return iter->second;
}

std::optional<llvm::StringRef> LifetimeSymbolTable::LookupLifetime(
    Lifetime lifetime) const {
  if (lifetime == Lifetime::Static()) {
    return "static";
  }

  auto iter = lifetime_to_name_.find(lifetime);
  if (iter == lifetime_to_name_.end()) {
    return std::nullopt;
  }
  return iter->second;
}

}  // namespace devtools_rust
