// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests that analysis converges (but not prematurely).
// Many of these tests involve pointer variables that are mutated in a loop
// because this is a common source of non-convergence.

#include "nullability/test/check_diagnostics.h"
#include "external/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang::tidy::nullability {
namespace {

// The following tests are variations on a theme: We loop over pointers where
// the first and subsequent pointers are returned by different functions with
// potentially different nullability. If either of the functions returns a
// nullable pointer, we should warn about the dereference.

TEST(PointerNullabilityTest, PointerLoop_Nullable_Nullable) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    Nullable<int*> GetFirst();
    Nullable<int*> GetNext();
    void target() {
      for (int* p = GetFirst();; p = GetNext()) {
        *p;  // [[unsafe]]
      }
    }
  )cc"));
}

TEST(PointerNullabilityTest, PointerLoop_Nonnull_Nullable) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    Nonnull<int*> GetFirst();
    Nullable<int*> GetNext();
    void target() {
      for (int* p = GetFirst();; p = GetNext()) {
        *p;  // [[unsafe]]
      }
    }
  )cc"));
}

TEST(PointerNullabilityTest, PointerLoop_Nullable_Nonnull) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    Nullable<int*> GetFirst();
    Nonnull<int*> GetNext();
    void target() {
      for (int* p = GetFirst();; p = GetNext()) {
        *p;  // [[unsafe]]
      }
    }
  )cc"));
}

TEST(PointerNullabilityTest, PointerLoop_Nonnull_Nonnull) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    Nonnull<int*> GetFirst();
    Nonnull<int*> GetNext();
    void target() {
      for (int* p = GetFirst();; p = GetNext()) {
        *p;
      }
    }
  )cc"));
}

TEST(PointerNullabilityTest, PointerLoop_Unknown_Nullable) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int* GetFirst();
    Nullable<int*> GetNext();
    void target() {
      for (int* p = GetFirst();; p = GetNext()) {
        *p;  // [[unsafe]]
      }
    }
  )cc"));
}

TEST(PointerNullabilityTest, PointerLoop_Unknown_Nonnull) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int* GetFirst();
    Nonnull<int*> GetNext();
    void target() {
      for (int* p = GetFirst();; p = GetNext()) {
        *p;
      }
    }
  )cc"));
}

TEST(PointerNullabilityTest, PointerLoop_Nullable_Unknown) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    Nullable<int*> GetFirst();
    int* GetNext();
    void target() {
      for (int* p = GetFirst();; p = GetNext()) {
        *p;  // [[unsafe]]
      }
    }
  )cc"));
}

TEST(PointerNullabilityTest, PointerLoop_Nonnull_Unknown) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    Nonnull<int*> GetFirst();
    int* GetNext();
    void target() {
      for (int* p = GetFirst();; p = GetNext()) {
        *p;
      }
    }
  )cc"));
}

TEST(PointerNullabilityTest, PointerLoop_Unknown_Unknown) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int* GetFirst();
    int* GetNext();
    void target() {
      for (int* p = GetFirst();; p = GetNext()) {
        *p;
      }
    }
  )cc"));
}

// If we check that the pointer is non-null, don't warn.
TEST(PointerNullabilityTest, PointerLoop_Checked) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    Nullable<int*> GetFirst();
    Nullable<int*> GetNext();
    void target() {
      for (int* p = GetFirst(); p != nullptr; p = GetNext()) {
        *p;
      }
    }
  )cc"));
}

// If there is a loop condition but it is unrelated to the pointer value, warn.
// Only `GetNext()` returns `_Nullable` to test that the check analyzes more
// than just the first iteration.
TEST(PointerNullabilityTest, PointerLoop_UnrelatedCondition) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    Nonnull<int*> GetFirst();
    Nullable<int*> GetNext();
    bool cond();
    void target() {
      for (int* p = GetFirst(); cond(); p = GetNext()) {
        *p;  // [[unsafe]]
      }
    }
  )cc"));
}

// Similar to `PointerLoop_UnrelatedCondition`, but we use a counted loop.
TEST(PointerNullabilityTest, PointerLoop_Counted) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    Nonnull<int*> GetFirst();
    Nullable<int*> GetNext();
    void target() {
      int* p = GetFirst();
      for (int i = 0; i < 10; ++i, p = GetNext()) {
        *p;  // [[unsafe]]
      }
    }
  )cc"));
}

// Various tests for convergence of range-based for loops.

TEST(PointerNullabilityTest, RangeFor_Array_ByValue) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target() {
      int array[10];
      for (int i : array)
        ;
    }
  )cc"));
}

TEST(PointerNullabilityTest, RangeFor_Array_ByReference) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target() {
      int array[10];
      for (const int& i : array)
        ;
    }
  )cc"));
}

TEST(PointerNullabilityTest, RangeFor_CustomContainer) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct Container {
      int* begin();
      int* end();
    };

    void target() {
      Container container;
      for (const int& i : container)
        ;
    }
  )cc"));
}

TEST(PointerNullabilityTest, RangeFor_TemplatedContainer) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    template <typename T>
    struct Container {
      T* begin();
      T* end();
    };

    void target() {
      Container<int> container;
      for (const int& i : container)
        ;
    }
  )cc"));
}

TEST(PointerNullabilityTest, RangeFor_CustomIterator) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct Iterator {
      bool operator==(const Iterator& I) const;
      bool operator!=(const Iterator& I) const;
      int& operator*() const;
      Iterator operator++();
    };
    struct Container {
      Iterator begin();
      Iterator end();
    };

    void target() {
      Container container;
      for (const int& i : container)
        ;
    }
  )cc"));
}

// This test and the one below are regression tests for false positives caused
// by a framework bug: https://github.com/llvm/llvm-project/issues/67834.
TEST(PointerNullabilityTest, WhileAssignment) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    Nullable<int*> GetNext();
    void target() {
      int* p;
      while ((p = GetNext())) {
        *p;
      }
    }
  )cc"));
}

TEST(PointerNullabilityTest, WhileAssignment2) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int* GetFirst();
    Nullable<int*> GetNext();
    void target() {
      Nullable<int*> p = GetFirst();
      while ((p = GetNext()) != nullptr) {
        *p;
      }
    }
  )cc"));
}

// Regression test for a false positive that was caused by an inconsistent
// representation of state in a loop: b/300979650.
TEST(PointerNullabilityTest, InconsistentLoopStateRepro) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    Nullable<int*> GetNullable();
    bool cond();

    void target(int* b, int* e) {
      // This loop is necessary for the false positive to occur.
      for (; b != e; ++b)
        ;

      int* ptr = GetNullable();
      if (ptr != nullptr) {
        while (cond()) {
          (void)*ptr;
        }
      }
    }
  )cc"));
}

TEST(PointerNullabilityTest, ReproForFalsePositiveTriggeredByUnrelatedLoop) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    bool cond();
    struct Node {
      const Node* _Nonnull parent() const;
    };
    void target(const Node* _Nonnull node) {
      // Redundant null check -- but if we comment this out, the analysis
      // doesn't converge.
      if (node == nullptr) return;
      // We get no false positive if the following unrelated loop is commented
      // out.
      for (bool b = cond(); cond(); b = 0 & b) {
      }
      while (cond()) {
        // This is the line where the false positive occurred (now fixed).
        node = node->parent();
      }
    }
  )cc"));
}

TEST(PointerNullabilityTest, WidenAfterContradictionVarAsCondition) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int *get();
    bool cond();

    void target() {
      bool b = true;
      while (b) {
        b = cond();
      }
      // If this code is analyzed before the loop body, then `b` is true and
      // this code has a contradictory flow condition: it contains `!b`, that
      // is, `false`. Once the loop is fully analyzed, `b` will be Top,
      // resolving the issue (i.e. the code will have a satisfiable flow
      // condition).
      int *p = get();
      while (cond()) {
        (void)*p;
      }
    }
  )cc"));
}

TEST(PointerNullabilityTest, WidenAfterContradictionArbitraryCondition) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int *get();
    bool cond();

    void target() {
      bool b = true;
      while (cond()) {
        b = false;
      }
      // If this code is analyzed before the loop body, then `b` is true and the
      // code below the return statement has a contradictory flow condition: it
      // contains `!b`, that is, `false`. Once the loop is fully analyzed, `b`
      // will be Top, resolving the issue (i.e. the code will have a satisfiable
      // flow condition).
      if (b) return;
      int *p = get();
      while (cond()) {
        (void)*p;
      }
    }
  )cc"));
}

TEST(PointerNullabilityTest, TriplyNestedForLoopSingleIteration) {
  // The test is minimized from `ABSL_LOG_INTERNAL_STATEFUL_CONDITION`, which is
  // used, for example, in the implementation of Abseil's `LOG_FIRST_N` logging
  // macro.
  // This used to cause a "maximum number of blocks processed" error.
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target() {
      for (bool b = true; b;)
        for (int x; b;)
          for (int c; b; b = false) {
            (void)0;
          }
    }
  )cc"));
}

TEST(PointerNullabilityTest, FieldWithLoopInBetweenDefinitionAndUse) {
  // TODO: b/347699477 Figure out this false positive.
  // It seems like two loops are needed to repro the issue.
  // The original code gets the `item` from a foreach loop over a vector
  // but this is simplified to a function returning the item (vs an iterator
  // returning the item) (a plain array with [] doesn't repro).
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct S {
      Nullable<int*> ptr;
    };
    S& at(int i);

    void target() {
      for (int i = 0; i < 10; ++i) {
        const S& item = at(i);
        if (item.ptr == nullptr) continue;
        (void)*item.ptr;
        for (int j = 0; j < 5; ++j) {
        }
        // NOTE: This should be safe as the loop before does nothing
        (void)*item.ptr;  // [[unsafe]]
      }
    }
  )cc"));
}

}  // namespace
}  // namespace clang::tidy::nullability
