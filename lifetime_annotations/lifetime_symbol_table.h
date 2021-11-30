// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_LIFETIME_ANNOTATIONS_LIFETIME_SYMBOL_TABLE_H_
#define CRUBIT_LIFETIME_ANNOTATIONS_LIFETIME_SYMBOL_TABLE_H_

#include <optional>

#include "lifetime_annotations/lifetime.h"
#include "third_party/llvm/llvm-project/llvm/include/llvm/ADT/DenseMap.h"
#include "third_party/llvm/llvm-project/llvm/include/llvm/ADT/StringMap.h"
#include "third_party/llvm/llvm-project/llvm/include/llvm/ADT/StringRef.h"

namespace devtools_rust {

// One-to-one mapping between lifetime names and the corresponding lifetimes.
class LifetimeSymbolTable {
 public:
  // Looks up a lifetime name in the symbol table.
  // Returns the corresponding lifetime if the name was present in the symbol
  // table, or nullopt if the lifetime name wasn't found.
  // If `name` is "static", returns `Lifetime::Static()`.
  std::optional<Lifetime> LookupName(llvm::StringRef name) const;

  // Looks up a lifetime name in the symbol table and inserts a new variable
  // lifetime in the table if the name was not found.
  // If `name` is "static", returns `Lifetime::Static()`.
  Lifetime LookupNameAndMaybeDeclare(llvm::StringRef name);

 private:
  llvm::StringMap<Lifetime> name_to_lifetime_;
};

}  // namespace devtools_rust

#endif  // CRUBIT_LIFETIME_ANNOTATIONS_LIFETIME_SYMBOL_TABLE_H_
