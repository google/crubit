// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability_verification/pointer_nullability_analysis.h"

#include <string>

#include "third_party/llvm/llvm-project/clang/unittests/Analysis/FlowSensitive/TestingSupport.h"
#include "llvm/ADT/ArrayRef.h"
#include "llvm/ADT/StringRef.h"
#include "llvm/Support/Error.h"
#include "llvm/Testing/Support/Error.h"
#include "third_party/llvm/llvm-project/llvm/utils/unittest/googlemock/include/gmock/gmock.h"
#include "third_party/llvm/llvm-project/llvm/utils/unittest/googletest/include/gtest/gtest.h"

namespace clang {
namespace tidy {
namespace nullability {
namespace {

using ::testing::Pair;
using ::testing::Test;
using ::testing::UnorderedElementsAre;

using dataflow::DataflowAnalysisState;
using dataflow::Environment;
using dataflow::test::checkDataflow;

MATCHER(IsSafe, "") { return arg.Lattice.isSafe(); }
MATCHER(IsUnsafe, "") { return !arg.Lattice.isSafe(); }

class PointerNullabilityTest : public Test {
 protected:
  template <typename Matcher>
  void expectDataflow(llvm::StringRef Code, Matcher Expectations) {
    ASSERT_THAT_ERROR(
        checkDataflow<PointerNullabilityAnalysis>(
            Code, "target",
            [](ASTContext& ASTCtx, Environment&) {
              return PointerNullabilityAnalysis(ASTCtx);
            },
            [&Expectations](
                llvm::ArrayRef<
                    std::pair<std::string,
                              DataflowAnalysisState<PointerNullabilityLattice>>>
                    Results,
                ASTContext&) { EXPECT_THAT(Results, Expectations); },
            {"-fsyntax-only", "-std=c++17"}),
        llvm::Succeeded());
  }
};

TEST_F(PointerNullabilityTest, SafeNoOp) {
  std::string Code = R"(
    void target(int* maybeNull) {
      1 + 2;
      /*[[check]]*/
    }
  )";
  expectDataflow(Code, UnorderedElementsAre(Pair("check", IsSafe())));
}

TEST_F(PointerNullabilityTest, UnsafeUnchecked) {
  std::string Code = R"(
    void target(int* maybeNull) {
      *maybeNull;
      /*[[check]]*/
    }
  )";
  expectDataflow(Code, UnorderedElementsAre(Pair("check", IsUnsafe())));
}

TEST_F(PointerNullabilityTest, SafeCheckNEQNull) {
  std::string NullLiteralOnRight = R"(
    void target(int* maybeNull) {
      if (maybeNull != nullptr) {
        *maybeNull;
        /*[[check-safe]]*/
      } else {
        *maybeNull;
        /*[[check-unsafe1]]*/
      }
      *maybeNull;
      /*[[check-unsafe2]]*/
    }
  )";
  expectDataflow(NullLiteralOnRight,
                 UnorderedElementsAre(Pair("check-safe", IsSafe()),
                                      Pair("check-unsafe1", IsUnsafe()),
                                      Pair("check-unsafe2", IsUnsafe())));

  std::string NullLiteralOnLeft = R"(
    void target(int* maybeNull) {
      if (nullptr != maybeNull) {
        *maybeNull;
        /*[[check-safe]]*/
      } else {
        *maybeNull;
        /*[[check-unsafe1]]*/
      }
      *maybeNull;
      /*[[check-unsafe2]]*/
    }
  )";
  expectDataflow(NullLiteralOnLeft,
                 UnorderedElementsAre(Pair("check-safe", IsSafe()),
                                      Pair("check-unsafe1", IsUnsafe()),
                                      Pair("check-unsafe2", IsUnsafe())));
}

TEST_F(PointerNullabilityTest, SafeCheckImplicitCastToBool) {
  std::string Code = R"(
    void target(int* maybeNull) {
      if (maybeNull) {
        *maybeNull;
        /*[[check-safe]]*/
      } else {
        *maybeNull;
        /*[[check-unsafe1]]*/
      }
      *maybeNull;
      /*[[check-unsafe2]]*/
    }
  )";
  expectDataflow(Code, UnorderedElementsAre(Pair("check-safe", IsSafe()),
                                            Pair("check-unsafe1", IsUnsafe()),
                                            Pair("check-unsafe2", IsUnsafe())));
  std::string NegatedCondition = R"(
    void target(int* maybeNull) {
      if (!maybeNull) {
        *maybeNull;
        /*[[check-unsafe1]]*/
      } else {
        *maybeNull;
        /*[[check-safe]]*/
      }
      *maybeNull;
      /*[[check-unsafe2]]*/
    }
  )";
  expectDataflow(NegatedCondition,
                 UnorderedElementsAre(Pair("check-safe", IsSafe()),
                                      Pair("check-unsafe1", IsUnsafe()),
                                      Pair("check-unsafe2", IsUnsafe())));
}

}  // namespace
}  // namespace nullability
}  // namespace tidy
}  // namespace clang
