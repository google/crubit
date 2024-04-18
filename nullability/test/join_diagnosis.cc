// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests for joining different nullability types.

#include <memory>
#include <string>

#include "nullability/pointer_nullability_analysis.h"
#include "nullability/test/check_diagnostics.h"
#include "clang/AST/ASTContext.h"
#include "clang/ASTMatchers/ASTMatchers.h"
#include "clang/Analysis/FlowSensitive/DataflowEnvironment.h"
#include "clang/Analysis/FlowSensitive/WatchedLiteralsSolver.h"
#include "third_party/llvm/llvm-project/clang/unittests/Analysis/FlowSensitive/TestingSupport.h"
#include "llvm/ADT/DenseMap.h"
#include "llvm/ADT/StringRef.h"
#include "llvm/Testing/Support/Error.h"
#include "third_party/llvm/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

// TODO: The tests in this file test two different things that should be tested
// separately:
// a) That the analysis correctly joins pointers with different nullability.
//    These tests should be preserved, but they should be converted to
//    `nullability_test` tests (as they test the behavior of the analysis, not
//    diagnosis).
// b) That the nullability state of a pointer is correctly "entangled" with a
//    boolean condition. These checks should be replaced with equivalent tests
//    in path_sensitive.cc (to the extent that those don't exist already).

namespace clang::tidy::nullability {
namespace {

TEST(PointerNullabilityTest, JoinNullAndNonNull) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *_Nonnull y, bool b) {
      int *x = nullptr;
      *x;  // [[unsafe]]
      if (b) {
        *x;  // [[unsafe]]
        x = y;
        *x;
      }
      *x;  // [[unsafe]]
      if (b) {
        *x;
      } else {
        *x;  // [[unsafe]]
      }
    }
  )cc"));
}

TEST(PointerNullabilityTest, JoinNullAndNullable) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *_Nullable y, bool b) {
      int *x = nullptr;
      *x;  // [[unsafe]]
      if (b) {
        *x;  // [[unsafe]]
        x = y;
        *x;  // [[unsafe]]
      }
      *x;  // [[unsafe]]
      if (b) {
        *x;  // [[unsafe]]
      } else {
        *x;  // [[unsafe]]
      }
    }
  )cc"));
}

TEST(PointerNullabilityTest, JoinNullAndUnknown) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *y, bool b) {
      int *x = nullptr;
      *x;  // [[unsafe]]
      if (b) {
        *x;  // [[unsafe]]
        x = y;
        *x;
      }
      *x;  // [[unsafe]]
      if (b) {
        *x;
      } else {
        *x;  // [[unsafe]]
      }
    }
  )cc"));
}

TEST(PointerNullabilityTest, JoinNonNullAndNull) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *_Nonnull y, bool b) {
      int *x = y;
      *x;
      if (b) {
        *x;
        x = nullptr;
        *x;  // [[unsafe]]
      }
      *x;  // [[unsafe]]
      if (b) {
        *x;  // [[unsafe]]
      } else {
        *x;
      }
    }
  )cc"));
}

TEST(PointerNullabilityTest, JoinNonNullAndNonNull) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *_Nonnull y, int *_Nonnull z, bool b) {
      int *x = y;
      *x;
      if (b) {
        *x;
        x = z;
        *x;
      }
      *x;
      if (b) {
        *x;
      } else {
        *x;
      }
    }
  )cc"));
}

TEST(PointerNullabilityTest, JoinNonNullAndNullable) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *_Nonnull y, int *_Nullable z, bool b) {
      int *x = y;
      *x;
      if (b) {
        *x;
        x = z;
        *x;  // [[unsafe]]
      }
      *x;  // [[unsafe]]
      if (b) {
        *x;  // [[unsafe]]
      } else {
        *x;
      }
    }
  )cc"));
}

TEST(PointerNullabilityTest, JoinNonNullAndUnknown) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *_Nonnull y, int *z, bool b) {
      int *x = y;
      *x;
      if (b) {
        *x;
        x = z;
        *x;
      }
      *x;
      if (b) {
        *x;
      } else {
        *x;
      }
    }
  )cc"));
}

TEST(PointerNullabilityTest, JoinNullableAndNull) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *_Nullable y, bool b) {
      int *x = y;
      *x;  // [[unsafe]]
      if (b) {
        *x;  // [[unsafe]]
        x = nullptr;
        *x;  // [[unsafe]]
      }
      *x;  // [[unsafe]]
      if (b) {
        *x;  // [[unsafe]]
      } else {
        *x;  // [[unsafe]]
      }
    }
  )cc"));
}

TEST(PointerNullabilityTest, JoinNullableAndNonNull) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *_Nullable y, int *_Nonnull z, bool b) {
      int *x = y;
      *x;  // [[unsafe]]
      if (b) {
        *x;  // [[unsafe]]
        x = z;
        *x;
      }
      *x;  // [[unsafe]]
      if (b) {
        *x;
      } else {
        *x;  // [[unsafe]]
      }
    }
  )cc"));
}

TEST(PointerNullabilityTest, JoinNullableAndNullable) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *_Nullable y, int *_Nullable z, bool b) {
      int *x = y;
      *x;  // [[unsafe]]
      if (b) {
        *x;  // [[unsafe]]
        x = z;
        *x;  // [[unsafe]]
      }
      *x;  // [[unsafe]]
      if (b) {
        *x;  // [[unsafe]]
      } else {
        *x;  // [[unsafe]]
      }
    }
  )cc"));
}

TEST(PointerNullabilityTest, JoinNullableAndUnknown) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *_Nullable y, int *z, bool b) {
      int *x = y;
      *x;  // [[unsafe]]
      if (b) {
        *x;  // [[unsafe]]
        x = z;
        *x;
      }
      *x;  // [[unsafe]]
      if (b) {
        *x;
      } else {
        *x;  // [[unsafe]]
      }
    }
  )cc"));
}

TEST(PointerNullabilityTest, JoinUnknownAndNull) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *y, bool b) {
      int *x = y;
      *x;
      if (b) {
        *x;
        x = nullptr;
        *x;  // [[unsafe]]
      }
      *x;  // [[unsafe]]
      if (b) {
        *x;  // [[unsafe]]
      } else {
        *x;
      }
    }
  )cc"));
}

TEST(PointerNullabilityTest, JoinUnknownAndNonNull) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *y, int *_Nonnull z, bool b) {
      int *x = y;
      *x;
      if (b) {
        *x;
        x = z;
        *x;
      }
      *x;
      if (b) {
        *x;
      } else {
        *x;
      }
    }
  )cc"));
}

TEST(PointerNullabilityTest, JoinUnknownAndNullable) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *y, int *_Nullable z, bool b) {
      int *x = y;
      *x;
      if (b) {
        *x;
        x = z;
        *x;  // [[unsafe]]
      }
      *x;  // [[unsafe]]
      if (b) {
        *x;  // [[unsafe]]
      } else {
        *x;
      }
    }
  )cc"));
}

TEST(PointerNullabilityTest, JoinUnknownAndUnknown) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *y, int *z, bool b) {
      int *x = y;
      if (b) {
        *x;
        x = z;
        *x;
      }
      *x;
      if (b) {
        *x;
      } else {
        *x;
      }
    }
  )cc"));
}

TEST(PointerNullabilityTest, JoinPointerLValues) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct Node {
      Node* next;
    };
    bool b();
    void target(Node* first) {
      for (Node* cur = first; cur; cur = cur->next) {
        // We used to crash here: `PointerNullabilityAnalysis::join()` assumed
        // `Value`s of pointer type were always `PointerValue`.
        //
        // Here the `MemberExpr` is a glvalue and produces a `ReferenceValue`
        // of type `Node *`.
        // When we join the first and second loop iteration,
        // `Environment::join()` calls `PointerNullabilityAnalysis::join()` to
        // combine the two `ReferenceValue`s.
        cur->next;

        // The rest of this function exists to actually trigger a situation
        // where we perform a join and the two `ReferenceValue`s to be joined
        // are actually different. (Otherwise, we will never call through to
        // `PointerNullabilityAnalysis::join()` in the first place.)
        // This code is unfortunately pretty arbitrary, because it relies on the
        // specific order in which the framework processes blocks in the CFG.
        // This is unsatisfactory, but will be moot when `ReferenceValue` is
        // eliminated (see https://discourse.llvm.org/t/70086 for details).
        // At that point, this test should be converted into a test that checks
        // that the analysis converges (which is also not the case, see below).
        if (b())
          ;
        else {
          // The join that used to trigger the crash happens at the top of this
          // loop where the edge that comes from outside the loop joins the edge
          // that comes from the bottom of the loop.
          for (int i = 0; i < 10; ++i) {
          }
        }
      }
    }
  )cc"));
}

}  // namespace
}  // namespace clang::tidy::nullability
