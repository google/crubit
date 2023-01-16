// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests for basic functionality.

#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "lifetime_analysis/test/lifetime_analysis_test.h"

namespace clang {
namespace tidy {
namespace lifetimes {
namespace {

TEST_F(LifetimeAnalysisTest, CompilationError) {
  // Check that we don't analyze code that doesn't compile.
  // This is a regression test -- we actually used to produce the lifetimes
  // "a -> a" for this test.
  EXPECT_THAT(GetLifetimes(R"(
    int* target(int* a) {
      undefined(&a);
      return a;
    }
  )"),
              LifetimesAre({{"", "Compilation error -- see log for details"}}));
}

TEST_F(LifetimeAnalysisTest, CompilationErrorFallback) {
  // Allow analysis of broken code to check that our fallback for detecting
  // expressions containing errors works.
  AnalyzeBrokenCode();

  EXPECT_THAT(
      GetLifetimes(R"(
    int* target(int* a) {
      undefined(&a);
      return a;
    }
  )"),
      LifetimesAre(
          {{"target", "ERROR: encountered an expression containing errors"}}));
}

TEST_F(LifetimeAnalysisTest, CompilationErrorFromWerrorDoesNotPreventAnalysis) {
  // Warnings upgraded through -Werror should not prevent analysis.
  EXPECT_THAT(GetLifetimes(R"(
#pragma clang diagnostic push
#pragma clang diagnostic error "-Wunused-variable"
    int* target(int* a) {
      int i = 0;
      return a;
    }
#pragma clang diagnostic pop
  )"),
              LifetimesAre({{"target", "a -> a"}}));
}

TEST_F(LifetimeAnalysisTest, NoLifetimes) {
  EXPECT_THAT(GetLifetimes(R"(
    void target() {
    }
  )"),
              LifetimesAre({{"target", ""}}));
}

TEST_F(LifetimeAnalysisTest, NoLifetimesArithmetic) {
  EXPECT_THAT(GetLifetimes(R"(
    int target(int a, int b) {
      return (a + b) - (-b) * a;
    }
  )"),
              LifetimesAre({{"target", "(), ()"}}));
}

TEST_F(LifetimeAnalysisTest, PointerToMemberDoesNotGetLifetime) {
  EXPECT_THAT(GetLifetimes(R"(
    struct S {};
    void target(S* s, int S::*ptr_to_member) {}
  )"),
              LifetimesAre({{"target", "a, ()"}}));
}

TEST_F(LifetimeAnalysisTest, UnconstrainedParameter) {
  EXPECT_THAT(GetLifetimes(R"(
    void target(int* a) {
    }
  )"),
              LifetimesAre({{"target", "a"}}));
}

TEST_F(LifetimeAnalysisTest, ReturnArgumentPtr) {
  EXPECT_THAT(GetLifetimes(R"(
    int* target(int* a) {
      return a;
    }
  )"),
              LifetimesAre({{"target", "a -> a"}}));
}

TEST_F(LifetimeAnalysisTest, ReturnArgumentPtrInitList) {
  EXPECT_THAT(GetLifetimes(R"(
    int* target(int* a) {
      return { a };
    }
  )"),
              LifetimesAre({{"target", "a -> a"}}));
}

TEST_F(LifetimeAnalysisTest, ReturnArgumentRef) {
  EXPECT_THAT(GetLifetimes(R"(
    int& target(int& a) {
      return a;
    }
  )"),
              LifetimesAre({{"target", "a -> a"}}));
}

TEST_F(LifetimeAnalysisTest, ReturnFirstArgumentPtr) {
  EXPECT_THAT(GetLifetimes(R"(
    int* target(int* a, int* b) {
      return a;
    }
  )"),
              LifetimesAre({{"target", "a, b -> a"}}));
}

TEST_F(LifetimeAnalysisTest, ReturnFirstArgumentRef) {
  EXPECT_THAT(GetLifetimes(R"(
    int& target(int& a, int& b) {
      return a;
    }
  )"),
              LifetimesAre({{"target", "a, b -> a"}}));
}

TEST_F(LifetimeAnalysisTest, ReturnRefFromPtr) {
  EXPECT_THAT(GetLifetimes(R"(
    int& target(int* a) {
      return *a;
    }
  )"),
              LifetimesAre({{"target", "a -> a"}}));
}

TEST_F(LifetimeAnalysisTest, ReturnPtrFromRef) {
  EXPECT_THAT(GetLifetimes(R"(
    int* target(int& a) {
      return &a;
    }
  )"),
              LifetimesAre({{"target", "a -> a"}}));
}

TEST_F(LifetimeAnalysisTest, ReturnDereferencedArgument) {
  EXPECT_THAT(GetLifetimes(R"(
    int* target(int** a) {
      return *a;
    }
  )"),
              LifetimesAre({{"target", "(a, b) -> a"}}));
}

TEST_F(LifetimeAnalysisTest, ReturnLocalViaPtr) {
  EXPECT_THAT(GetLifetimes(R"(
    int* target() {
      int a = 42;
      return &a;
    }
  )"),
              LifetimesAre({{"target",
                             "ERROR: function returns reference to a local"}}));
}

TEST_F(LifetimeAnalysisTest, ReturnLocalViaRef) {
  EXPECT_THAT(GetLifetimes(R"(
    int& target() {
      int a = 42;
      return a;
    }
  )"),
              LifetimesAre({{"target",
                             "ERROR: function returns reference to a local"}}));
}

TEST_F(LifetimeAnalysisTest, ReturnStaticViaPtr) {
  EXPECT_THAT(GetLifetimes(R"(
    int* target() {
      static int a = 42;
      return &a;
    }
  )"),
              LifetimesAre({{"target", "-> a"}}));
}

TEST_F(LifetimeAnalysisTest, StringLiteral) {
  EXPECT_THAT(GetLifetimes(R"(
    const char* target() {
      return "this is a string literal";
    }
  )"),
              LifetimesAre({{"target", "-> a"}}));
}

TEST_F(LifetimeAnalysisTest, OutParameter) {
  EXPECT_THAT(GetLifetimes(R"(
    void target(int& a) {
      a = 42;
    }
  )"),
              LifetimesAre({{"target", "a"}}));
}

TEST_F(LifetimeAnalysisTest, AssigningToPtrParamDoesNotChangeLifetime) {
  EXPECT_THAT(GetLifetimes(R"(
    void target(int* p) {
      int a = 42;
      p = &a;
    }
  )"),
              LifetimesAre({{"target", "a"}}));
}

TEST_F(LifetimeAnalysisTest, PtrInitializationTransfersLifetimes) {
  EXPECT_THAT(GetLifetimes(R"(
    int* target(int* p) {
      int* p2 = p;
      return p2;
    }
  )"),
              LifetimesAre({{"target", "a -> a"}}));
}

TEST_F(LifetimeAnalysisTest, PtrAssignmentTransfersLifetimes) {
  EXPECT_THAT(GetLifetimes(R"(
    int* target(int* p) {
      int* p2;
      p2 = p;
      return p2;
    }
  )"),
              LifetimesAre({{"target", "a -> a"}}));
}

TEST_F(LifetimeAnalysisTest, RefInitializationTransfersLifetimes) {
  EXPECT_THAT(GetLifetimes(R"(
    int& target(int& r) {
      int& r2 = r;
      return r2;
    }
  )"),
              LifetimesAre({{"target", "a -> a"}}));
}

TEST_F(LifetimeAnalysisTest, RefAssignmentDoesNotTransferLifetimes) {
  EXPECT_THAT(GetLifetimes(R"(
    int& target(int& r) {
      int a = 42;
      int& r2 = a;
      r2 = r;
      return r2;
    }
  )"),
              LifetimesAre({{"target",
                             "ERROR: function returns reference to a local"}}));
}

TEST_F(LifetimeAnalysisTest, ReturnLocalSneaky_Initialization) {
  EXPECT_THAT(GetLifetimes(R"(
    int* target(int* arg1) {
      // Initialization should be aware that outer pointer is invariant in its
      // type.
      int** pp = &arg1;
      int local = 42;
      *pp = &local;
      return arg1;
    }
  )"),
              LifetimesAre({{"target",
                             "ERROR: function returns reference to a local"}}));
}

TEST_F(LifetimeAnalysisTest, ReturnLocalSneaky_Assignment) {
  EXPECT_THAT(GetLifetimes(R"(
    int* target(int* arg1) {
      // Assignment should be aware that outer pointer is invariant in its type.
      int** pp;
      pp = &arg1;
      int local = 42;
      *pp = &local;
      return arg1;
    }
  )"),
              LifetimesAre({{"target",
                             "ERROR: function returns reference to a local"}}));
}

TEST_F(LifetimeAnalysisTest, ReturnLocalSneaky2_Initialization) {
  EXPECT_THAT(GetLifetimes(R"(
    int* target(int* arg1) {
      // Initialization should be aware that outer pointer is invariant in its
      // type.
      int** pp = &arg1;
      int local = 42;
      arg1 = &local;
      return *pp;
    }
  )"),
              LifetimesAre({{"target",
                             "ERROR: function returns reference to a local"}}));
}

TEST_F(LifetimeAnalysisTest, ReturnLocalSneaky2_Assignment) {
  EXPECT_THAT(GetLifetimes(R"(
    int* target(int* arg1) {
      // Assignment should be aware that outer pointer is invariant in its type.
      int** pp;
      pp = &arg1;
      int local = 42;
      arg1 = &local;
      return *pp;
    }
  )"),
              LifetimesAre({{"target",
                             "ERROR: function returns reference to a local"}}));
}

TEST_F(LifetimeAnalysisTest, ReturnLocalSneaky3_Initialization) {
  EXPECT_THAT(GetLifetimes(R"(
    int* target(int* arg1) {
      int*& pp = arg1;
      int local = 42;
      arg1 = &local;
      return pp;
    }
  )"),
              LifetimesAre({{"target",
                             "ERROR: function returns reference to a local"}}));
}

TEST_F(LifetimeAnalysisTest, SwapPointers) {
  EXPECT_THAT(GetLifetimes(R"(
    void swap_ptr(int** pp1, int** pp2) {
      int* tmp = *pp2;
      *pp2 = *pp1;
      *pp1 = tmp;
    }
  )"),
              LifetimesAre({{"swap_ptr", "(a, b), (a, c)"}}));
}

TEST_F(LifetimeAnalysisTest, DuplicatePointer) {
  EXPECT_THAT(GetLifetimes(R"(
    void duplicate_ptr(int* from, int** to1, int** to2) {
      *to1 = from;
      *to2 = from;
    }
  )"),
              LifetimesAre({{"duplicate_ptr", "a, (a, b), (a, c)"}}));
}

TEST_F(LifetimeAnalysisTest, Aliasing) {
  EXPECT_THAT(GetLifetimes(R"(
    int* target(int** a, int** b, int* c) {
      *a = c;
      return *b;
    }
  )"),
              LifetimesAre({{"target", "(a, b), (c, d), a -> c"}}));
}

TEST_F(LifetimeAnalysisTest, IncompleteType) {
  // Test that we can handle pointers to incomplete types.
  EXPECT_THAT(GetLifetimes(R"(
    struct S;
    S* target(S* s) {
      return s;
    }
  )"),
              LifetimesAre({{"target", "a -> a"}}));
}

TEST_F(LifetimeAnalysisTest, DISABLED_IncompleteTypeTemplate) {
  // TODO(mboehme): Disabled because it returns the wrong lifetimes.
  // S<int*> is never instantiated because we only deal with pointers to it,
  // so it's an incomplete type.
  //
  // We can handle incomplete types in principle, but in this case,  because
  // we don't create any pointees for the fields of `S<int*>`, we will produce
  // these incorrect lifetimes:
  //   (a, b) -> (c, b)
  // Even more strangely, the lifetimes we infer change (to the correct ones)
  // once we happen to instantiate S<int*> somewhere else in the same
  // translation unit.
  //
  // I'm not sure how best to solve this. We could simply force instantiation
  // of all uninstantiated templates we see, but I believe this might change the
  // semantics of the program in subtle ways.
  //
  // The better alternative seems to be: If we're unifying lifetimes of an
  // object that is of an instantiated class template type, unify the lifetimes
  // of its template arguments too. This can be overly restrictive -- think of a
  // class template that doesn't actually use its template arguments in any of
  // its fields, e.g. `template <class T> struct S {};`. However, it seems to be
  // the only option that produces consistent results without requiring us to
  // instantiate class templates that could otherwise be used as incomplete
  // types.
  EXPECT_THAT(GetLifetimes(R"(
    template <class T>
    struct S {
      T t;
    };

    S<int*>* target(S<int*>* s) {
      return s;
    }
  )"),
              LifetimesAre({{"target", "(a, b) -> (a, b)"}}));
}

TEST_F(LifetimeAnalysisTest, UndefinedFunction_NoLifetimeElision) {
  EXPECT_THAT(
      GetLifetimes(R"(
    int* f(int* a);
    int* target(int* a) {
      return f(a);
    }
  )"),
      LifetimesAre({{"f", "ERROR: Lifetime elision not enabled for 'f'"},
                    {"target",
                     "ERROR: No lifetimes for callee 'f': Lifetime elision not "
                     "enabled for 'f'"}}));
}

TEST_F(LifetimeAnalysisTest, UndefinedFunction_LifetimeElision) {
  EXPECT_THAT(GetLifetimes(R"(
    #pragma clang lifetime_elision
    int* f(int* a);
    int* target(int* a) {
      return f(a);
    }
  )"),
              LifetimesAre({{"f", "a -> a"}, {"target", "a -> a"}}));
}

TEST_F(LifetimeAnalysisTest, ForwardDeclaration) {
  EXPECT_THAT(GetLifetimes(R"(
    int* f(int* a);
    int* target(int* a) {
      return f(a);
    }
    int* f(int* a) {
      return a;
    }
  )"),
              LifetimesAre({{"f", "a -> a"}, {"target", "a -> a"}}));
}

TEST_F(LifetimeAnalysisTest, Overwrite_SingleDestination) {
  EXPECT_THAT(GetLifetimes(R"(
    int* target(int* a, int* b) {
      int** pp = &b;
      // There is only one thing that `pp` can be pointing at, so the analysis
      // should conclude that `b` is being overwritten with `a`.
      *pp = a;
      return b;
    }
  )"),
              LifetimesAre({{"target", "a, b -> a"}}));
}

TEST_F(LifetimeAnalysisTest, Overwrite_SingleDestinationVariant) {
  EXPECT_THAT(GetLifetimes(R"(
    // Similar to above, but potentially leave `pp` uninitialized.
    int* target(int* a, int* b) {
      int** pp;
      if (*a > 0) {
        pp = &b;
      }
      // If `pp` is uninitialized, the following is UB, so the analysis can
      // assume that `pp` was initialized to point to `b`.
      // This particular test function is pretty terrible style, but it seems
      // plausible that similar situations can come up in more reasonable code.
      *pp = a;
      return b;
    }
  )"),
              LifetimesAre({{"target", "a, b -> a"}}));
}

TEST_F(LifetimeAnalysisTest, Overwrite_MultipleDestinations) {
  EXPECT_THAT(GetLifetimes(R"(
    // This is a regression test. The analysis used to conclude falsely that `b`
    // was unconditionally being overwritten with `a` in the assignment and was
    // therefore producing the wrong lifetimes "a, b -> a".
    int* target(int* a, int* b) {
      int** pp = *a > 0? &a : &b;
      // The analysis should understand that the following assignment _might_
      // overwrite `b` with `a` but does not necessarily do so.
      *pp = a;
      return b;
    }
  )"),
              LifetimesAre({{"target", "a, a -> a"}}));
}

}  // namespace
}  // namespace lifetimes
}  // namespace tidy
}  // namespace clang
