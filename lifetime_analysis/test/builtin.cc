// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests involving builtins.

#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "lifetime_analysis/test/lifetime_analysis_test.h"

namespace clang {
namespace tidy {
namespace lifetimes {
namespace {

TEST_F(LifetimeAnalysisTest, ReturnPtrFromRefAddressOf) {
  EXPECT_THAT(GetLifetimes(R"(
    int* target(int& a) {
      return __builtin_addressof(a);
    }
  )"),
              LifetimesContain({{"target", "a -> a"}}));
}

TEST_F(LifetimeAnalysisTest, ReturnDoublePtrFromRefAddressOf) {
  EXPECT_THAT(GetLifetimes(R"(
    int** target(int*& a) {
      return __builtin_addressof(a);
    }
  )"),
              LifetimesContain({{"target", "(a, b) -> (a, b)"}}));
}

TEST_F(LifetimeAnalysisTest, BuiltinNoLifetimes) {
  EXPECT_THAT(GetLifetimes(R"(
    int target(int a) {
      return __builtin_labs(a);
    }
  )"),
              LifetimesContain({{"target", "()"}}));
}

// TODO(veluca): add tests for the strto* functions.

TEST_F(LifetimeAnalysisTest, BuiltinMemStrChr) {
  EXPECT_THAT(GetLifetimes(R"(
    void* memchr(void* a, int val, int num) {
      return __builtin_memchr(a, val, num);
    }
    const char* strchr(const char* a, int val) {
      return __builtin_strchr(a, val);
    }
    const char* strrchr(const char* a, int val) {
      return __builtin_strrchr(a, val);
    }
  )"),
              LifetimesContain({
                  {"memchr", "a, (), () -> a"},
                  {"strchr", "a, () -> a"},
                  {"strrchr", "a, () -> a"},
              }));
}

TEST_F(LifetimeAnalysisTest, BuiltinStrProcessing) {
  EXPECT_THAT(GetLifetimes(R"(
    const char* strstr(const char* a, const char* b) {
      return __builtin_strstr(a, b);
    }
    const char* strpbrk(const char* a, const char* b) {
      return __builtin_strpbrk(a, b);
    }
  )"),
              LifetimesContain({
                  {"strstr", "a, b -> a"},
                  {"strpbrk", "a, b -> a"},
              }));
}

TEST_F(LifetimeAnalysisTest, BuiltinForward) {
  EXPECT_THAT(GetLifetimes(R"(
    namespace std {
      // This is simplified from the actual definition of forward(), but it's
      // all we need for this test.
      template<class T>
      T&& forward(T& t) noexcept {
        return static_cast<T&&>(t);
      }
    }
    int* target(int* a) {
      return std::forward(a);
    }
  )"),
              LifetimesContain({{"target", "a -> a"}}));
}

TEST_F(LifetimeAnalysisTest, BuiltinMove) {
  EXPECT_THAT(
      GetLifetimes(R"(
    namespace std {
      // This is simplified from the actual definition of move(), but it's all
      // we need for this test.
      template<class T>
      T&& move(T&& t) noexcept {
        return static_cast<T&&>(t);
      }
    }
    int* move_int_ptr(int* a) {
      return std::move(a);
    }
    template <class T, class U> struct S { T t; U u; };
    S<int**, int*> move_template(S<int**, int*> s) {
      return std::move(s);
    }
  )"),
      LifetimesContain({{"move_int_ptr", "a -> a"},
                        {"move_template", "(<a, b, c>) -> (<a, b, c>)"}}));
}

}  // namespace
}  // namespace lifetimes
}  // namespace tidy
}  // namespace clang
