// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "lifetime_annotations/lifetime_symbol_table.h"

#include <iostream>
#include <optional>
#include <string>

#include "third_party/llvm/llvm-project/llvm/include/llvm/Support/ErrorHandling.h"

namespace clang {
namespace tidy {
namespace lifetimes {

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

static std::string NameFromIndex(int index) {
  int num_chars = 1;
  // Number of combinations that are possible with `num_chars` characters.
  int num_combinations = 26;
  while (index >= num_combinations) {
    index -= num_combinations;
    ++num_chars;
    num_combinations *= 26;
  }
  std::string name;
  name.reserve(num_chars);
  for (int i = 0; i < num_chars; ++i) {
    name.insert(0, static_cast<size_t>(1), 'a' + index % 26);
    index /= 26;
  }
  return name;
}

llvm::StringRef LifetimeSymbolTable::LookupLifetimeAndMaybeDeclare(
    Lifetime lifetime) {
  if (lifetime == Lifetime::Static()) {
    return "static";
  }

  auto lifetime_to_name_iter = lifetime_to_name_.find(lifetime);
  if (lifetime_to_name_iter != lifetime_to_name_.end()) {
    return lifetime_to_name_iter->second;
  }

  while (true) {
    std::string name = NameFromIndex(next_name_index_++);
    auto [name_to_lifetime_iter, inserted] =
        name_to_lifetime_.try_emplace(name, lifetime);
    if (inserted) {
      lifetime_to_name_[lifetime] = name;
      return name_to_lifetime_iter->first();
    }
  }
}

void LifetimeSymbolTable::Add(llvm::StringRef name, Lifetime lifetime) {
  auto [_, inserted] = name_to_lifetime_.try_emplace(name, lifetime);
  if (!inserted) {
    llvm::report_fatal_error("duplicate lifetime parameter");
  }
  lifetime_to_name_[lifetime] = name;
}

void LifetimeSymbolTable::Rebind(llvm::StringRef name, Lifetime lifetime) {
  auto iter = name_to_lifetime_.find(name);
  if (iter == name_to_lifetime_.end()) {
    llvm::report_fatal_error("invalid call to rebind");
  }
  lifetime_to_name_.erase(iter->second);
  lifetime_to_name_[lifetime] = name;
  iter->second = lifetime;
}

}  // namespace lifetimes
}  // namespace tidy
}  // namespace clang
