// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "absl/base/nullability.h"
#include "absl/strings/str_cat.h"
#include "absl/strings/string_view.h"
#include "benchmark/benchmark.h"
#include "nullability/pointer_nullability_diagnosis.h"
#include "nullability/pragma.h"
#include "clang/AST/ASTContext.h"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclBase.h"
#include "clang/Basic/LLVM.h"
#include "clang/Testing/CommandLineArgs.h"
#include "clang/Testing/TestAST.h"
#include "llvm/Support/ErrorHandling.h"

namespace clang::tidy::nullability {
namespace {

NamedDecl *absl_nonnull lookup(absl::string_view Name, const DeclContext &DC) {
  auto Result = DC.lookup(&DC.getParentASTContext().Idents.get(Name));
  if (!Result.isSingleResult())
    llvm::reportFatalInternalError(Twine("lookup failed for ") + Name);
  return Result.front();
}

void benchmarkAnalysisOnCode(benchmark::State& State, llvm::StringRef Code) {
  clang::TestInputs Inputs(Code);
  Inputs.Language = TestLanguage::Lang_CXX20;
  Inputs.ExtraArgs = {
      "-fsyntax-only",
      "-Wno-unused-value",
      "-Wno-nonnull",
  };
  TestAST AST(Inputs);
  auto* Target = cast<FunctionDecl>(
      lookup("Target", *AST.context().getTranslationUnitDecl()));
  NullabilityPragmas NoPragmas;
  LambdaCaptureNullabilityMap CaptureMap;

  for (auto _ : State)
    (void)diagnosePointerNullability(Target, NoPragmas, CaptureMap);
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

// Benchmark with lots of short-circuiting conditions and early returns.
// Most of the conditions are irrelevant to the pointers in the function:
// - pointers: `elt_ty`, `target`, the struct field `type`, the return value
//   of `lookup()` and `insert()`.
// - conditions: `elt_ty->isInteger()`, `elt_ty->isBoolean()`, etc.
void BM_PointerAnalysisShortCircuitingAndEarlyReturnsOnIrrelevantConditions(
    benchmark::State& State) {
  benchmarkAnalysisOnCode(State, R"cpp(
    struct Type {
      bool isInteger() const;
      bool isBoolean() const;
      bool isSigned() const;
      bool isFloat() const;
      bool isSpecial() const;
    };

    struct TargetInfo {
      bool hasFeatureA() const;
      bool hasFeatureB() const;
    };

    struct Key {
      const Type* type;
      unsigned num_elements;
      unsigned num_fields;
    };

    int* lookup(const Key& k);
    int* insert(const Key& k, int val);
    unsigned getTypeSize(const Type* t);

    int* Target(const Type* elt_ty, unsigned num_elts, unsigned num_fields,
                const TargetInfo* target) {
      Key k{elt_ty, num_elts, num_fields};
      if (int* cached = lookup(k)) return cached;

      if (target->hasFeatureA()) {
        unsigned size = getTypeSize(elt_ty);

#define CASE_A(ELS, BITS, NF, SIGNED, VAL)            \
      if (elt_ty->isInteger() && !elt_ty->isBoolean() &&  \
          elt_ty->isSigned() == SIGNED && size == BITS && \
          num_elts == (ELS * NF) && num_fields == NF) {   \
        return insert(k, VAL);                            \
      }
        CASE_A(16, 8, 1, true, 1)
        CASE_A(8, 16, 1, true, 2)
        CASE_A(4, 32, 1, true, 3)
        CASE_A(2, 64, 1, true, 4)
        CASE_A(16, 8, 1, false, 5)
        CASE_A(8, 16, 1, false, 6)
        CASE_A(4, 32, 1, false, 7)
        CASE_A(2, 64, 1, false, 8)
        CASE_A(16, 8, 2, true, 9)
        CASE_A(8, 16, 2, true, 10)
        CASE_A(4, 32, 2, true, 11)
        CASE_A(2, 64, 2, true, 12)
        CASE_A(16, 8, 2, false, 13)
        CASE_A(8, 16, 2, false, 14)
        CASE_A(4, 32, 2, false, 15)
        CASE_A(2, 64, 2, false, 16)
#undef CASE_A
      } else if (target->hasFeatureB()) {
        unsigned size = getTypeSize(elt_ty);

#define CASE_B(ELS, BITS, NF, SIGNED, FP, VAL)                  \
      if (!elt_ty->isBoolean() &&                                   \
          ((elt_ty->isInteger() && elt_ty->isSigned() == SIGNED) || \
           (elt_ty->isFloat() && !elt_ty->isSpecial() && FP)) &&    \
          size == BITS && num_elts == ELS && num_fields == NF) {    \
        return insert(k, VAL);                                      \
      }
        CASE_B(1, 8, 1, true, false, 101)
        CASE_B(2, 8, 1, true, false, 102)
        CASE_B(4, 8, 1, true, false, 103)
        CASE_B(8, 8, 1, true, false, 104)
        CASE_B(16, 8, 1, true, false, 105)
        CASE_B(32, 8, 1, true, false, 106)
        CASE_B(64, 8, 1, true, false, 107)
        CASE_B(1, 8, 1, false, false, 108)
        CASE_B(2, 8, 1, false, false, 109)
        CASE_B(4, 8, 1, false, false, 110)
        CASE_B(8, 8, 1, false, false, 111)
        CASE_B(16, 8, 1, false, false, 112)
        CASE_B(32, 8, 1, false, false, 113)
        CASE_B(64, 8, 1, false, false, 114)
#undef CASE_B
      }
      return nullptr;
    }
  )cpp");
}
BENCHMARK(
    BM_PointerAnalysisShortCircuitingAndEarlyReturnsOnIrrelevantConditions);

// Benchmark with batch member pointer initialization in a factory function
// (could be a constructor too).
void BM_PointerAnalysisBatchMemberPointerInit(benchmark::State& State) {
  benchmarkAnalysisOnCode(State, R"cpp(
    struct Metric {
      void increment();
      void add(long long v);
    };

    struct Context {
      Metric* getMetric(const char* name);
    };

    struct PipelineService {
      Metric* m1_;
      Metric* m2_;
      Metric* m3_;
      Metric* m4_;
      Metric* m5_;
      Metric* m6_;
      Metric* m7_;
      Metric* m8_;
      Metric* m9_;
      Metric* m10_;
      Metric* m11_;
      Metric* m12_;
      Metric* m13_;
      Metric* m14_;
      Metric* m15_;
      Metric* m16_;
      Metric* m17_;
      Metric* m18_;
      Metric* m19_;
      Metric* m20_;
      Metric* m21_;
      Metric* m22_;
      Metric* m23_;
      Metric* m24_;
      Metric* m25_;
      Metric* m26_;
      Metric* m27_;
      Metric* m28_;
      Metric* m29_;
      Metric* m30_;
      Metric* m31_;
      Metric* m32_;
      Metric* m33_;
      Metric* m34_;
      Metric* m35_;
      Metric* m36_;
      Metric* m37_;
      Metric* m38_;
      Metric* m39_;
      Metric* m40_;
      Metric* m41_;
      Metric* m42_;
      Metric* m43_;
      Metric* m44_;
      Metric* m45_;
      Metric* m46_;
      Metric* m47_;
      Metric* m48_;
      Metric* m49_;
      Metric* m50_;
    };

    void Target(PipelineService* _Nullable self, Context* _Nullable ctx) {
      if (!self || !ctx) return;
      self->m1_ = ctx->getMetric("m1");
      self->m2_ = ctx->getMetric("m2");
      self->m3_ = ctx->getMetric("m3");
      self->m4_ = ctx->getMetric("m4");
      self->m5_ = ctx->getMetric("m5");
      self->m6_ = ctx->getMetric("m6");
      self->m7_ = ctx->getMetric("m7");
      self->m8_ = ctx->getMetric("m8");
      self->m9_ = ctx->getMetric("m9");
      self->m10_ = ctx->getMetric("m10");
      self->m11_ = ctx->getMetric("m11");
      self->m12_ = ctx->getMetric("m12");
      self->m13_ = ctx->getMetric("m13");
      self->m14_ = ctx->getMetric("m14");
      self->m15_ = ctx->getMetric("m15");
      self->m16_ = ctx->getMetric("m16");
      self->m17_ = ctx->getMetric("m17");
      self->m18_ = ctx->getMetric("m18");
      self->m19_ = ctx->getMetric("m19");
      self->m20_ = ctx->getMetric("m20");
      self->m21_ = ctx->getMetric("m21");
      self->m22_ = ctx->getMetric("m22");
      self->m23_ = ctx->getMetric("m23");
      self->m24_ = ctx->getMetric("m24");
      self->m25_ = ctx->getMetric("m25");
      self->m26_ = ctx->getMetric("m26");
      self->m27_ = ctx->getMetric("m27");
      self->m28_ = ctx->getMetric("m28");
      self->m29_ = ctx->getMetric("m29");
      self->m30_ = ctx->getMetric("m30");
      self->m31_ = ctx->getMetric("m31");
      self->m32_ = ctx->getMetric("m32");
      self->m33_ = ctx->getMetric("m33");
      self->m34_ = ctx->getMetric("m34");
      self->m35_ = ctx->getMetric("m35");
      self->m36_ = ctx->getMetric("m36");
      self->m37_ = ctx->getMetric("m37");
      self->m38_ = ctx->getMetric("m38");
      self->m39_ = ctx->getMetric("m39");
      self->m40_ = ctx->getMetric("m40");
      self->m41_ = ctx->getMetric("m41");
      self->m42_ = ctx->getMetric("m42");
      self->m43_ = ctx->getMetric("m43");
      self->m44_ = ctx->getMetric("m44");
      self->m45_ = ctx->getMetric("m45");
      self->m46_ = ctx->getMetric("m46");
      self->m47_ = ctx->getMetric("m47");
      self->m48_ = ctx->getMetric("m48");
      self->m49_ = ctx->getMetric("m49");
      self->m50_ = ctx->getMetric("m50");
    }
  )cpp");
}
BENCHMARK(BM_PointerAnalysisBatchMemberPointerInit);

// Benchmark with a large static array initializer.
void BM_PointerAnalysisLargeStaticArrayInitializer(benchmark::State& State) {
  benchmarkAnalysisOnCode(State, R"cpp(
    struct Span {
      const unsigned char* data;
      unsigned size;
    };

    Span Target() {
      static const unsigned char kDataArray[] = {
          0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a,
          0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15,
          0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20,
          0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b,
          0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36,
          0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41,
          0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c,
          0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57,
          0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62,
          0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d,
          0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78,
          0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83,
          0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e,
          0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99,
          0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4,
          0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf,
          0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba,
          0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5,
          0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0,
          0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb,
          0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6,
          0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1,
          0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc,
          0xfd, 0xfe, 0xff, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
          0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12,
          0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d,
          0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28,
          0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33,
          0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e,
          0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49,
          0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54,
          0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f,
          0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a,
          0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75,
          0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80,
          0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b,
          0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96,
          0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1,
          0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac,
          0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7,
          0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2,
          0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd,
          0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8,
          0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3,
          0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee,
          0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9,
          0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff, 0x00, 0x01, 0x02, 0x03, 0x04,
          0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
          0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a,
          0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25,
          0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30,
          0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b,
          0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46,
          0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51,
          0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c,
          0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67,
          0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72,
          0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d,
          0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88,
          0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93,
          0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e,
          0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9,
          0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4,
          0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf,
          0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca,
          0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5,
          0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0,
          0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb,
          0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6,
          0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff};
      return Span{kDataArray, sizeof(kDataArray)};
    }
  )cpp");
}
BENCHMARK(BM_PointerAnalysisLargeStaticArrayInitializer);

}  // namespace
}  // namespace clang::tidy::nullability

int main(int argc, char **absl_nonnull argv) {
  benchmark::Initialize(&argc, argv);
  benchmark::RunSpecifiedBenchmarks();
  return 0;
}
