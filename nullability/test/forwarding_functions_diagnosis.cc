// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests for diagnosis seeing through forwarding functions to the underlying
// constructor.

#include "nullability/test/check_diagnostics.h"
#include "clang/Testing/CommandLineArgs.h"
#include "third_party/llvm/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang::tidy::nullability {
namespace {

TEST(PointerNullabilityTest, ConstructorThroughMakeUnique) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
#include <memory>

    struct S {
      // Check that we find the right constructor overload amongst several.
      S() = default;
      explicit S(int *_Nonnull p0);
      S(int x, int *_Nullable p0);
      S(int x, int *_Nullable p0, int *_Nonnull p1);
    };

    void target(int x, int *_Nullable p, int *_Nonnull q,
                int *_Nullable const cp, int *_Nonnull const cq) {
      if (x == 0) {
        std::make_unique<S>(p);  // [[unsafe]]
      } else if (x == 1) {
        std::make_unique<S>(q);
      } else if (x == 2) {
        std::make_unique<S>(x, p);
      } else if (x == 3) {
        std::make_unique<S>(x, p, q);
      } else if (x == 4) {
        std::make_unique<S>(x, q, p);  // [[unsafe]]
      } else if (x == 5) {
        std::make_unique<S>(x, cq, cp);  // [[unsafe]]
        std::make_unique<S>(x, cq, cp);  // [[unsafe]]
      } else if (x == 6) {
        // Test with nullptr literal, which results in a make_unique
        // instantiation with a parameter of type nullptr_t (which isn't
        // considered a PointerValue)
        // TODO(b/378501394): This looks like it is caught in tests, but it
        // falls into the "untracked" category, so is suppressed in production.
        std::make_unique<S>(x, cq, nullptr);  // [[unsafe]]
      } else if (x == 7) {
        // Also test uninteresting constructors (e.g., the 0-arg one)
        std::make_unique<S[]>(10);
      } else {
        std::make_unique<double>(1.0);
      }
    }
  )cc"));
}

TEST(PointerNullabilityTest, TemplateConstructorThroughMakeUnique) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
#include <memory>

    template <typename T, typename U, typename V>
    struct S {
      S() = default;
      explicit S(T *_Nonnull p0);
      S(T x, U *_Nullable p0, V *_Nonnull p1);
    };

    void target(int x, double *_Nullable p, double *_Nonnull q) {
      if (x == 0) {
        std::make_unique<S<double, char, int>>(p);  // [[unsafe]]
        std::make_unique<S<double, char, int>>(q);
      } else if (x == 1) {
        std::make_unique<S<int, double, double>>(x, p, q);
      } else if (x == 2) {
        std::make_unique<S<int, double, double>>(x, q, p);  // [[unsafe]]
      }
    }
  )cc"));
}

TEST(PointerNullabilityTest, ConstructorWithSafeDefaultArg) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
#include <memory>

    struct S {
      S(int *_Nonnull p0, int *_Nullable p1 = kDefault);

      static int *_Nullable kDefault;
    };
    int *_Nullable S::kDefault = nullptr;

    // Case where the non-default arg for p0 is unsafe, but the default arg for
    // p1 is safe.
    void target(int x, int *_Nullable p) {
      if (x == 0) {
        new S(p, p);  // [[unsafe]]
      } else if (x == 1) {
        new S(p);  // [[unsafe]]
      } else if (x == 2) {
        std::make_unique<S>(p, p);  // [[unsafe]]
      } else {
        std::make_unique<S>(p);  // [[unsafe]]
      }
    }
  )cc"));
}

TEST(PointerNullabilityTest, InitListInsteadOfConstructor) {
  // Test when a struct doesn't have a constructor, so make_unique's `new
  // S2(...)` will be an `CXXParenListInitExpr` instead.
  // This is a C++20 (and onward) feature.
  EXPECT_TRUE(checkDiagnosticsWithMin(
      // force indenting
      R"cc(
#include <memory>
        // A struct to test implicit conversion from int.
        struct S1 {
          S1() = default;
          S1(int x) : x(x) {}
          int x = 0;
        };

        struct S2 {
          S1 s1a;
          int *_Nullable p;
          int *_Nonnull q;
          int x;
          int y = -1;
          int z = {};
          bool b = z;
          S1 s1b;
        };

        void target(int x, int *_Nullable p) {
          if (x == 0) {
            new S2{x, new int, p, x, 0, 1, false, S1{x}};  // [[unsafe]]
          } else if (x == 1) {
            // Try `x` vs `S1{x}` to test implicit conversion.
            // That will make an unrelated CXXConstructExpr within the body of
            // make_unique<S2> to set up the argument for `s1a`.
            std::make_unique<S2>(x, new int, p, x, 0, 1, false, S1{x});  // [[unsafe]]
          } else if (x == 2) {
            // Test fewer arguments than fields.
            std::make_unique<S2>(x, p, p);  // [[unsafe]]
          } else if (x == 3) {
            // Test nullptr literal.
            std::make_unique<S2>(x, p, nullptr);  // [[unsafe]]
          }
        }
      )cc",
      TestLanguage::Lang_CXX20));
}

TEST(PointerNullabilityTest, InitListInsteadOfConstructorWithBaseClass) {
  EXPECT_TRUE(checkDiagnosticsWithMin(
      R"cc(
#include <memory>
        struct Base {
          int *_Nonnull a;
          int *_Nullable b;
        };

        struct S : public Base {
          int *_Nullable p;
          int *_Nonnull q;
        };

        void target(int x, int *_Nullable null, int *_Nonnull nonnull) {
          if (x == 0) {
            // Brace aggregation initialization
            S{nonnull, null, nonnull, null};  // [[unsafe]]
          } else if (x == 1) {
            // make_unique cases
            std::make_unique<S>(Base(nonnull, null), nonnull, null);  // [[unsafe]]
          } else if (x == 2) {
            // Test nullptr literal
            std::make_unique<S>(Base(nonnull, null), nonnull, nullptr);  // [[unsafe]]
          }
        }
      )cc",
      TestLanguage::Lang_CXX20));
}

}  // namespace
}  // namespace clang::tidy::nullability
