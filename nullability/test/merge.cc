// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests for merging different nullability types.

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

namespace clang::tidy::nullability {
namespace {

TEST(PointerNullabilityTest, MergeNullAndNonNull) {
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

TEST(PointerNullabilityTest, MergeNullAndNullable) {
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

TEST(PointerNullabilityTest, MergeNullAndUnknown) {
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

TEST(PointerNullabilityTest, MergeNonNullAndNull) {
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

TEST(PointerNullabilityTest, MergeNonNullAndNonNull) {
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

TEST(PointerNullabilityTest, MergeNonNullAndNullable) {
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

TEST(PointerNullabilityTest, MergeNonNullAndUnknown) {
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

TEST(PointerNullabilityTest, MergeNullableAndNull) {
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

TEST(PointerNullabilityTest, MergeNullableAndNonNull) {
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

TEST(PointerNullabilityTest, MergeNullableAndNullable) {
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

TEST(PointerNullabilityTest, MergeNullableAndUnknown) {
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

TEST(PointerNullabilityTest, MergeUnknownAndNull) {
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

TEST(PointerNullabilityTest, MergeUnknownAndNonNull) {
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

TEST(PointerNullabilityTest, MergeUnknownAndNullable) {
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

TEST(PointerNullabilityTest, MergeUnknownAndUnknown) {
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

TEST(PointerNullabilityTest, MergePointerLValues) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct Node {
      Node* next;
    };
    bool b();
    void target(Node* first) {
      for (Node* cur = first; cur; cur = cur->next) {
        // We used to crash here: `PointerAnalysis::merge()` assumed `Value`s
        // of pointer type were always `PointerValue`.
        //
        // Here the `MemberExpr` is a glvalue and produces a `ReferenceValue`
        // of type `Node *`.
        // When we merge the first and second loop iteration,
        // `Environment::join()` calls `PointerAnalysis::merge()` to combine
        // the two `ReferenceValue`s.
        cur->next;

        // The rest of this function exists to actually trigger a situation
        // where we perform a merge and the two `ReferenceValue`s to be merged
        // are actually different. (Otherwise, we will never call through to
        // `PointerNullabilityAnalysis::merge()` in the first place.)
        // This code is unfortunately pretty arbitrary, because it relies on the
        // specific order in which the framework processes blocks in the CFG.
        // This is unsatisfactory, but will be moot when `ReferenceValue` is
        // eliminated (see https://discourse.llvm.org/t/70086 for details).
        // At that point, this test should be converted into a test that checks
        // that the analysis converges (which is also not the case, see below).
        if (b())
          ;
        else {
          // The merge that used to trigger the crash happens at the top of this
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
