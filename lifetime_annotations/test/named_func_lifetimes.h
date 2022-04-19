// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_LIFETIME_ANNOTATIONS_TEST_NAMED_FUNC_LIFETIMES_H_
#define CRUBIT_LIFETIME_ANNOTATIONS_TEST_NAMED_FUNC_LIFETIMES_H_

#include <initializer_list>
#include <iostream>
#include <optional>
#include <string>
#include <utility>
#include <vector>

#include "testing/base/public/gunit.h"
#include "lifetime_annotations/function_lifetimes.h"
#include "lifetime_annotations/lifetime_symbol_table.h"
#include "third_party/llvm/llvm-project/llvm/include/llvm/ADT/StringMap.h"
#include "third_party/llvm/llvm-project/llvm/include/llvm/Support/ErrorHandling.h"
#include "third_party/llvm/llvm-project/llvm/include/llvm/Support/FormatVariadic.h"
#include "third_party/llvm/llvm-project/llvm/include/llvm/Support/raw_ostream.h"

namespace clang {
namespace tidy {
namespace lifetimes {

// Returns a human-readable representation of `func_lifetimes` that uses
// alphabetic names for lifetimes.
std::string NameLifetimes(const FunctionLifetimes& func_lifetimes);

// Returns a human-readable representation of `func_lifetimes` that uses
// the names from the given symbol table. The symbol table needs to contain
// names for all lifetimes in `func_lifetimes` except local lifetimes and the
// static lifetime.
std::string NameLifetimes(const FunctionLifetimes& func_lifetimes,
                          const LifetimeSymbolTable& symbol_table);

// Associates functions (identified by their name) with function lifetimes in
// the format returned by NameLifetimes().
class NamedFuncLifetimes {
 public:
  NamedFuncLifetimes() = default;

  NamedFuncLifetimes(const NamedFuncLifetimes&) = default;
  NamedFuncLifetimes(NamedFuncLifetimes&&) = default;
  NamedFuncLifetimes& operator=(const NamedFuncLifetimes&) = default;
  NamedFuncLifetimes& operator=(NamedFuncLifetimes&&) = default;

  NamedFuncLifetimes(
      std::initializer_list<std::pair<llvm::StringRef, std::string>> values) {
    for (const auto& pair : values) Add(pair.first, pair.second);
  }

  // Associates the function called `func` with the named lifetimes `lifetimes`.
  void Add(llvm::StringRef func, llvm::StringRef lifetimes) {
    bool did_insert = false;
    std::tie(std::ignore, did_insert) = lifetimes_.try_emplace(func, lifetimes);
    if (!did_insert) {
      llvm::report_fatal_error(llvm::formatv(
          "Calling `Add('{0}', ...)` clobbered an existing lifetimes entry",
          func));
    }
  }

  // Returns the named lifetimes for the function called `func`.
  std::optional<llvm::StringRef> Get(llvm::StringRef func) const;

  // Returns the "function name, lifetimes" entries in the mapping, sorted
  // alphabetically.
  std::vector<std::pair<llvm::StringRef, llvm::StringRef>> Entries() const;

  bool operator==(const NamedFuncLifetimes& other) const {
    return lifetimes_ == other.lifetimes_;
  }
  bool operator!=(const NamedFuncLifetimes& other) const {
    return !(*this == other);
  }

 private:
  friend std::ostream& operator<<(std::ostream& os,
                                  const NamedFuncLifetimes& lifetimes);

  llvm::StringMap<std::string> lifetimes_;
};

// Returns a matcher that matches a NamedFuncLifetimes equal to `expected`.
testing::Matcher<NamedFuncLifetimes> LifetimesAre(NamedFuncLifetimes expected);

// Returns a matcher that matches a NamedFuncLifetimes containing all oentries
// from `expected`, and possibly more.
testing::Matcher<NamedFuncLifetimes> LifetimesContain(
    NamedFuncLifetimes expected);

}  // namespace lifetimes
}  // namespace tidy
}  // namespace clang

#endif  // CRUBIT_LIFETIME_ANNOTATIONS_TEST_NAMED_FUNC_LIFETIMES_H_
