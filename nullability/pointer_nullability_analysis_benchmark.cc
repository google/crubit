// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <cstdint>

#include "absl/log/check.h"
#include "absl/strings/str_cat.h"
#include "absl/strings/string_view.h"
#include "third_party/benchmark/include/benchmark/benchmark.h"
#include "nullability/pointer_nullability_analysis.h"
#include "nullability/pointer_nullability_diagnosis.h"
#include "clang/AST/ASTContext.h"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclBase.h"
#include "clang/Analysis/FlowSensitive/DataflowAnalysis.h"
#include "clang/Basic/LLVM.h"
#include "clang/Testing/TestAST.h"

namespace clang::tidy::nullability {
namespace {

NamedDecl *lookup(absl::string_view Name, const DeclContext &DC) {
  auto Result = DC.lookup(&DC.getParentASTContext().Idents.get(Name));
  CHECK(Result.isSingleResult()) << Name;
  return Result.front();
}

void benchmarkAnalysisOnCode(benchmark::State &State, llvm::StringRef Code) {
  TestAST AST(Code);
  auto *Target = cast<FunctionDecl>(
      lookup("Target", *AST.context().getTranslationUnitDecl()));

  auto Diagnoser = pointerNullabilityDiagnoser();
  constexpr std::int64_t MaxSATIterations = 1'000'000;
  for (auto _ : State) {
    (void)dataflow::diagnoseFunction<PointerNullabilityAnalysis,
                                     PointerNullabilityDiagnostic>(
        *Target, AST.context(), Diagnoser, MaxSATIterations);
  }
}

void BM_PointerAnalysisCopyPointer(benchmark::State &State) {
  benchmarkAnalysisOnCode(State, R"cpp(
    int *Target(int *p) {
      int *q = p;
      return q;
    }
  )cpp");
}
BENCHMARK(BM_PointerAnalysisCopyPointer);

void BM_PointerAnalysisIntLoop(benchmark::State &State) {
  benchmarkAnalysisOnCode(State, R"cpp(
    int Target(int *p) {
      for (int i = 0; i < 10; ++i) *p += i;
      return *p;
    }
  )cpp");
}
BENCHMARK(BM_PointerAnalysisIntLoop);

void BM_PointerAnalysisPointerLoop(benchmark::State &State) {
  benchmarkAnalysisOnCode(State, R"cpp(
    int *_Nullable next();
    void Target(int i) {
      for (int *p = next(); p != nullptr; p = next()) *p += i;
    }
  )cpp");
}
BENCHMARK(BM_PointerAnalysisPointerLoop);

void BM_PointerAnalysisBranch(benchmark::State &State) {
  benchmarkAnalysisOnCode(State, R"cpp(
    int Target(int *p, bool b) {
      int i = 0;
      if (b)
        i = *p;
      else
        p = nullptr;
      return *p;
    }
  )cpp");
}
BENCHMARK(BM_PointerAnalysisBranch);

void BM_PointerAnalysisLoopAndBranch(benchmark::State &State) {
  benchmarkAnalysisOnCode(State, R"cpp(
    int *_Nullable next();
    bool cond();
    void Target(int *p, bool b) {
      int x = 0;
      for (int *p = next(); p != nullptr; p = next()) {
        if (cond())
          x = *p;  // arbitrary code with `*p`.
        else
          *p = x;  // different code with `*p`.
      }
    }
  )cpp");
}
BENCHMARK(BM_PointerAnalysisLoopAndBranch);

void BM_PointerAnalysisTwoLoops(benchmark::State &State) {
  benchmarkAnalysisOnCode(State, R"cpp(
    int Target(int *p, bool b) {
      int x = 0;
      for (int i = 0; i < 10; ++i) {
        x += *p;
      }
      x = 7;
      for (int i = 0; i < 10; ++i) {
        x += *p;
      }
      return *p;
    }
  )cpp");
}
BENCHMARK(BM_PointerAnalysisTwoLoops);

constexpr inline char preamble[] = R"cpp(
  namespace std {
  using size_t = unsigned;

  template <typename T>
  class vector {
   public:
    using iterator = T*;
    size_t size() const;
    iterator begin();
    iterator end();
  };

  class string_view {
   public:
    bool empty();
    char front();
    char* data();
    size_t size() const;
    void remove_prefix(size_t);
  };

  class string {
   public:
    struct iterator {
      char& operator*();
      iterator& operator++();
      iterator operator++(int);
      iterator& operator+=(unsigned);
      friend size_t operator-(const iterator&, const iterator&);
      friend bool operator!=(const iterator&, const iterator&);
    };
    void resize(size_t);
    void erase(size_t);
    iterator begin();
    iterator end();
  };
  }  // namespace std
)cpp";

// This benchmark is a simplified version of a function that joins two file-path
// strings.
void BM_PointerAnalysisJoinFilePath(benchmark::State &State) {
  absl::string_view code = R"cpp(
    std::string Target(std::vector<std::string_view> paths) {
      std::string result;

      if (paths.size() == 0) return result;

      std::size_t total_size = paths.size() - 1;
      for (const std::string_view path : paths) {
        total_size += path.size();
      }
      result.resize(total_size);

      auto begin = result.begin();
      auto out = begin;
      for (std::string_view path : paths) {
        if (path.empty()) continue;
        if (path.front() != '/' && out != begin) {
          *out++ = '/';
        }
        const std::size_t this_size = path.size();
        out += this_size;
      }
      result.erase(out - begin);

      return result;
    }
  )cpp";
  benchmarkAnalysisOnCode(State, absl::StrCat(preamble, code));
}
BENCHMARK(BM_PointerAnalysisJoinFilePath);

// In practice, the call to `memcpy` inside the loop demonstrated a substantial
// impact on microbenchmark performance. It is unclear why, and probably worth
// further reducing this benchmark. For now, it seems interesting enough to
// include in the suite.
void BM_PointerAnalysisCallInLoop(benchmark::State &State) {
  absl::string_view code = R"cpp(
    void* memcpy(void* dest, const void* src, std::size_t count);

    void Target(char* out, std::vector<std::string_view> paths) {
      if (paths.size() != 0) {
        std::size_t total_size = paths.size() - 1;
        for (const std::string_view path : paths) {
          total_size += path.size();
        }
        for (std::string_view path : paths) {
          if (path.empty()) continue;
          const std::size_t this_size = path.size();
          memcpy(out, path.data(), this_size);
          out += this_size;
        }
      }
    }
  )cpp";
  benchmarkAnalysisOnCode(State, absl::StrCat(preamble, code));
}
BENCHMARK(BM_PointerAnalysisCallInLoop);

}  // namespace
}  // namespace clang::tidy::nullability

int main(int argc, char **argv) {
  benchmark::Initialize(&argc, argv);
  benchmark::RunSpecifiedBenchmarks();
  return 0;
}
