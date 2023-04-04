// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests for merging different nullability types.

#include "nullability_verification/test/check_diagnostics.h"
#include "third_party/llvm/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang {
namespace tidy {
namespace nullability {
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

}  // namespace
}  // namespace nullability
}  // namespace tidy
}  // namespace clang
