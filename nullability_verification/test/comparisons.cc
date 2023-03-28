// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests for comparisons of types containing nullability annotations.

#include "nullability_verification/test/check_diagnostics.h"
#include "third_party/llvm/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang {
namespace tidy {
namespace nullability {
namespace {

TEST(PointerNullabilityTest, CompareNonNullPtrAndNonNullPtr) {
  // nonnull == nonnull
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int* _Nonnull x, int* _Nonnull y) {
      *x;
      *y;
      if (x == y) {
        *x;
        *y;
      } else {
        *x;
        *y;
      }
      *x;
      *y;
    }
  )cc"));

  // nonnull != nonnull
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int* _Nonnull x, int* _Nonnull y) {
      *x;
      *y;
      if (x != y) {
        *x;
        *y;
      } else {
        *x;
        *y;
      }
      *x;
      *y;
    }
  )cc"));
}

TEST(PointerNullabilityTest, CompareNullablePtrAndNullablePtr) {
  // nullable == nullable
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int* _Nullable x, int* _Nullable y) {
      *x;  // [[unsafe]]
      *y;  // [[unsafe]]
      if (x == y) {
        *x;  // [[unsafe]]
        *y;  // [[unsafe]]
      } else {
        *x;  // [[unsafe]]
        *y;  // [[unsafe]]
      }
      *x;  // [[unsafe]]
      *y;  // [[unsafe]]
    }
  )cc"));

  // nullable != nullable
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int* _Nullable x, int* _Nullable y) {
      *x;  // [[unsafe]]
      *y;  // [[unsafe]]
      if (x != y) {
        *x;  // [[unsafe]]
        *y;  // [[unsafe]]
      } else {
        *x;  // [[unsafe]]
        *y;  // [[unsafe]]
      }
      *x;  // [[unsafe]]
      *y;  // [[unsafe]]
    }
  )cc"));
}

TEST(PointerNullabilityTest, CompareUnknownPtrAndUnknownPtr) {
  // unknown == unknown
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *x, int *y) {
      *x;
      *y;
      if (x == y) {
        *x;
        *y;
      } else {
        *x;
        *y;
      }
      *x;
      *y;
    }
  )cc"));

  // unknown != unknown
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *x, int *y) {
      *x;
      *y;
      if (x != y) {
        *x;
        *y;
      } else {
        *x;
        *y;
      }
      *x;
      *y;
    }
  )cc"));
}

// TODO(b/233582219): Implement diagnosis of unreachable program points
TEST(PointerNullabilityTest, CompareNonNullPtrAndNullPtr) {
  // nonnull == nullptr
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int* _Nonnull x) {
      *x;
      if (x == nullptr) {
        *x;  // unreachable
      } else {
        *x;
      }
      *x;
    }
  )cc"));

  // nullptr == nonnull
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int* _Nonnull x) {
      *x;
      if (nullptr == x) {
        *x;  // unreachable
      } else {
        *x;
      }
      *x;
    }
  )cc"));

  // nonnull != nullptr
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int* _Nonnull x) {
      *x;
      if (x != nullptr) {
        *x;
      } else {
        *x;  // unreachable
      }
      *x;
    }
  )cc"));

  // nullptr != nonnull
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int* _Nonnull x) {
      *x;
      if (nullptr != x) {
        *x;
      } else {
        *x;  // unreachable
      }
      *x;
    }
  )cc"));
}

TEST(PointerNullabilityTest, CompareNullablePtrAndNullPtr) {
  // nullable == nullptr
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int* _Nullable x) {
      *x;  // [[unsafe]]
      if (x == nullptr) {
        *x;  // [[unsafe]]
      } else {
        *x;
      }
      *x;  // [[unsafe]]
    }
  )cc"));

  // nullptr == nullable
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int* _Nullable x) {
      *x;  // [[unsafe]]
      if (nullptr == x) {
        *x;  // [[unsafe]]
      } else {
        *x;
      }
      *x;  // [[unsafe]]
    }
  )cc"));

  // nullable != nullptr
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int* _Nullable x) {
      *x;  // [[unsafe]]
      if (x != nullptr) {
        *x;
      } else {
        *x;  // [[unsafe]]
      }
      *x;  // [[unsafe]]
    }
  )cc"));

  // nullptr != nullable
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int* _Nullable x) {
      *x;  // [[unsafe]]
      if (nullptr != x) {
        *x;
      } else {
        *x;  // [[unsafe]]
      }
      *x;  // [[unsafe]]
    }
  )cc"));
}

TEST(PointerNullabilityTest, CompareNullablePtrAndNonNullPtr) {
  // nullable == nonnull
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int* _Nullable x, int* _Nonnull y) {
      *x;  // [[unsafe]]
      *y;
      if (x == y) {
        *x;
        *y;
      } else {
        *x;  // [[unsafe]]
        *y;
      }
      *x;  // [[unsafe]]
      *y;
    }
  )cc"));

  // nonnull == nullable
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int* _Nullable x, int* _Nonnull y) {
      *x;  // [[unsafe]]
      *y;
      if (y == x) {
        *x;
        *y;
      } else {
        *x;  // [[unsafe]]
        *y;
      }
      *x;  // [[unsafe]]
      *y;
    }
  )cc"));

  // nullable != nonnull
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int* _Nullable x, int* _Nonnull y) {
      *x;  // [[unsafe]]
      *y;
      if (x != y) {
        *x;  // [[unsafe]]
        *y;
      } else {
        *x;
        *y;
      }
      *x;  // [[unsafe]]
      *y;
    }
  )cc"));

  // nonnull != nullable
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int* _Nullable x, int* _Nonnull y) {
      *x;  // [[unsafe]]
      *y;
      if (y != x) {
        *x;  // [[unsafe]]
        *y;
      } else {
        *x;
        *y;
      }
      *x;  // [[unsafe]]
      *y;
    }
  )cc"));
}

TEST(PointerNullabilityTest, CompareNullablePtrAndUnknownPtr) {
  // nullable == unknown
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *_Nullable x, int *y) {
      *x;  // [[unsafe]]
      *y;
      if (x == y) {
        *x;  // [[unsafe]]
        *y;
      } else {
        *x;  // [[unsafe]]
        *y;
      }
      *x;  // [[unsafe]]
      *y;
    }
  )cc"));

  // unknown == nullable
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *_Nullable x, int *y) {
      *x;  // [[unsafe]]
      *y;
      if (y == x) {
        *x;  // [[unsafe]]
        *y;
      } else {
        *x;  // [[unsafe]]
        *y;
      }
      *x;  // [[unsafe]]
      *y;
    }
  )cc"));

  // nullable != unknown
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *_Nullable x, int *y) {
      *x;  // [[unsafe]]
      *y;
      if (x != y) {
        *x;  // [[unsafe]]
        *y;
      } else {
        *x;  // [[unsafe]]
        *y;
      }
      *x;  // [[unsafe]]
      *y;
    }
  )cc"));

  // unknown != nullable
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *_Nullable x, int *y) {
      *x;  // [[unsafe]]
      *y;
      if (y != x) {
        *x;  // [[unsafe]]
        *y;
      } else {
        *x;  // [[unsafe]]
        *y;
      }
      *x;  // [[unsafe]]
      *y;
    }
  )cc"));
}

// TODO(b/233582219): Fix false negatives. The pointer is compared to nullptr,
// hence the unnannotated pointer should be considered nullable and emit
// warnings where it fails or is not null checked.
TEST(PointerNullabilityTest, CompareUnknownPtrAndNullPtr) {
  // unknown == nullptr
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *x) {
      *x;  // false-negative
      if (x == nullptr) {
        *x;  // false-negative
      } else {
        *x;
      }
      *x;  // false-negative
    }
  )cc"));

  // nullptr == unknown
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *x) {
      *x;  // false-negative
      if (nullptr == x) {
        *x;  // false-negative
      } else {
        *x;
      }
      *x;  // false-negative
    }
  )cc"));

  // unknown != nullptr
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *x) {
      *x;  // false-negative
      if (x != nullptr) {
        *x;
      } else {
        *x;  // false-negative
      }
      *x;  // false-negative
    }
  )cc"));

  // nullptr != unknown
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *x) {
      *x;  // false-negative
      if (nullptr != x) {
        *x;
      } else {
        *x;  // false-negative
      }
      *x;  // false-negative
    }
  )cc"));
}

TEST(PointerNullabilityTest, CompareUnknownPtrAndNonNullPtr) {
  // unknown == nonnull
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *x, int *_Nonnull y) {
      *x;
      *y;
      if (x == y) {
        *x;
        *y;
      } else {
        *x;
        *y;
      }
      *x;
      *y;
    }
  )cc"));

  // nonnull == unknown
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *x, int *_Nonnull y) {
      *x;
      *y;
      if (y == x) {
        *x;
        *y;
      } else {
        *x;
        *y;
      }
      *x;
      *y;
    }
  )cc"));

  // unknown != nonnull
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *x, int *_Nonnull y) {
      *x;
      *y;
      if (x != y) {
        *x;
        *y;
      } else {
        *x;
        *y;
      }
      *x;
      *y;
    }
  )cc"));

  // nonnull != unknown
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *x, int *_Nonnull y) {
      *x;
      *y;
      if (y != x) {
        *x;
        *y;
      } else {
        *x;
        *y;
      }
      *x;
      *y;
    }
  )cc"));
}

}  // namespace
}  // namespace nullability
}  // namespace tidy
}  // namespace clang
