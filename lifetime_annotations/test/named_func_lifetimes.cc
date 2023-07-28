// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "lifetime_annotations/test/named_func_lifetimes.h"

#include <algorithm>
#include <optional>
#include <string>

#include "lifetime_annotations/function_lifetimes.h"
#include "llvm/ADT/DenseMap.h"

namespace clang {
namespace tidy {
namespace lifetimes {

std::string NameLifetimes(const FunctionLifetimes& func_lifetimes) {
  LifetimeSymbolTable symbol_table;
  return func_lifetimes.DebugString([&symbol_table](Lifetime l) -> std::string {
    if (l.IsLocal()) {
      return "local";
    }
    if (l == Lifetime::Static()) {
      return "static";
    }
    return symbol_table.LookupLifetimeAndMaybeDeclare(l).str();
  });
}

std::string NameLifetimes(const FunctionLifetimes& func_lifetimes,
                          const LifetimeSymbolTable& symbol_table) {
  return func_lifetimes.DebugString([&symbol_table](Lifetime l) -> std::string {
    if (l.IsLocal()) {
      return "local";
    }
    if (l == Lifetime::Static()) {
      return "static";
    }
    std::optional<llvm::StringRef> name = symbol_table.LookupLifetime(l);
    assert(name.has_value());
    return name.value().str();
  });
}

std::optional<llvm::StringRef> NamedFuncLifetimes::Get(
    llvm::StringRef func) const {
  auto iter = lifetimes_.find(func);
  if (iter == lifetimes_.end()) {
    return std::nullopt;
  }
  return iter->second;
}

std::vector<std::pair<llvm::StringRef, llvm::StringRef>>
NamedFuncLifetimes::Entries() const {
  std::vector<std::pair<llvm::StringRef, llvm::StringRef>> entries;
  for (const auto& entry : lifetimes_) {
    entries.emplace_back(entry.getKey(), entry.getValue());
  }
  std::sort(entries.begin(), entries.end());
  return entries;
}

std::ostream& operator<<(std::ostream& os,
                         const NamedFuncLifetimes& lifetimes) {
  std::vector<std::pair<llvm::StringRef, llvm::StringRef>> entries =
      lifetimes.Entries();
  for (size_t i = 0; i < entries.size(); ++i) {
    if (i > 0) {
      os << "; ";
    }
    os << entries[i].first.str() << ": " << entries[i].second.str();
  }

  return os;
}

class LifetimesAreMatcher {
 public:
  using is_gtest_matcher = void;

  explicit LifetimesAreMatcher(NamedFuncLifetimes expected)
      : expected_(std::move(expected)) {}

  bool MatchAndExplain(const NamedFuncLifetimes& lifetimes,
                       std::ostream*) const {
    return lifetimes == expected_;
  }

  void DescribeTo(std::ostream* os) const { *os << "is " << expected_; }

  void DescribeNegationTo(std::ostream* os) const {
    *os << "is not " << expected_;
  }

 private:
  NamedFuncLifetimes expected_;
};

testing::Matcher<NamedFuncLifetimes> LifetimesAre(NamedFuncLifetimes expected) {
  return LifetimesAreMatcher(std::move(expected));
}

class LifetimesContainMatcher {
 public:
  using is_gtest_matcher = void;

  explicit LifetimesContainMatcher(NamedFuncLifetimes expected)
      : expected_(std::move(expected)) {}

  bool MatchAndExplain(const NamedFuncLifetimes& lifetimes,
                       std::ostream*) const {
    for (auto [func, expected_lifetimes] : expected_.Entries()) {
      std::optional<llvm::StringRef> actual_lifetimes = lifetimes.Get(func);
      if (!actual_lifetimes.has_value() ||
          *actual_lifetimes != expected_lifetimes) {
        return false;
      }
    }
    return true;
  }

  void DescribeTo(std::ostream* os) const { *os << "contains " << expected_; }

  void DescribeNegationTo(std::ostream* os) const {
    *os << "does not contain " << expected_;
  }

 private:
  NamedFuncLifetimes expected_;
};

testing::Matcher<NamedFuncLifetimes> LifetimesContain(
    NamedFuncLifetimes expected) {
  return LifetimesContainMatcher(std::move(expected));
}

}  // namespace lifetimes
}  // namespace tidy
}  // namespace clang
