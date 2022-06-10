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
            [](ASTContext &ASTCtx, Environment &) {
              return PointerNullabilityAnalysis(ASTCtx);
            },
            [&Expectations](
                llvm::ArrayRef<
                    std::pair<std::string,
                              DataflowAnalysisState<PointerNullabilityLattice>>>
                    Results,
                ASTContext &) { EXPECT_THAT(Results, Expectations); },
            {"-fsyntax-only", "-std=c++17", "-Wno-unused-value"}),
        llvm::Succeeded());
  }
};

TEST_F(PointerNullabilityTest, NoPointerOperations) {
  std::string Code = R"(
    void target() {
      1 + 2;
      /*[[safe]]*/
    }
  )";
  expectDataflow(Code, UnorderedElementsAre(Pair("safe", IsSafe())));
}

TEST_F(PointerNullabilityTest, DereferenceWithoutACheck) {
  std::string Code = R"(
    void target(int* maybeNull) {
      *maybeNull;
      /*[[unsafe]]*/
    }
  )";
  expectDataflow(Code, UnorderedElementsAre(Pair("unsafe", IsUnsafe())));
}

TEST_F(PointerNullabilityTest, InitializedWithNullPtrLiteral) {
  std::string NullPtr = R"(
    void target() {
      int *null = nullptr;
      *null;
      /*[[unsafe]]*/
    }
  )";
  expectDataflow(NullPtr, UnorderedElementsAre(Pair("unsafe", IsUnsafe())));

  std::string ZeroAsNull = R"(
    void target() {
      int *null = 0;
      *null;
      /*[[unsafe]]*/
    }
  )";
  expectDataflow(ZeroAsNull, UnorderedElementsAre(Pair("unsafe", IsUnsafe())));
}

TEST_F(PointerNullabilityTest, InitializedWithAddressOf) {
  std::string Code = R"(
    void target(int x) {
      int *nonNull = &x;
      *nonNull;
      /*[[safe]]*/
    }
  )";
  expectDataflow(Code, UnorderedElementsAre(Pair("safe", IsSafe())));
}

TEST_F(PointerNullabilityTest, InitializedWithOtherPointer) {
  std::string DerefCopyOfNonNull = R"(
    void target(int x) {
      int *nonNull = &x;
      int *nonNullCopy = nonNull;
      *nonNullCopy;
      /*[[safe]]*/
    }
  )";
  expectDataflow(DerefCopyOfNonNull,
                 UnorderedElementsAre(Pair("safe", IsSafe())));

  std::string DerefCopyOfNullable = R"(
    void target(int* nullable) {
      int *nullableCopy = nullable;
      *nullableCopy;
      /*[[unsafe]]*/
    }
  )";
  expectDataflow(DerefCopyOfNullable,
                 UnorderedElementsAre(Pair("unsafe", IsUnsafe())));

  std::string DerefCopyOfNullableCheckOriginal = R"(
    void target(int* nullable) {
      int *nullableCopy = nullable;
      if (nullable) {
        *nullableCopy;
        /*[[safe]]*/
      } else {
        *nullableCopy;
        /*[[unsafe-1]]*/
      }
      *nullableCopy;
      /*[[unsafe-2]]*/
    }
  )";
  expectDataflow(
      DerefCopyOfNullableCheckOriginal,
      UnorderedElementsAre(Pair("safe", IsSafe()), Pair("unsafe-1", IsUnsafe()),
                           Pair("unsafe-2", IsUnsafe())));

  std::string DerefNullableCheckCopy = R"(
    void target(int* nullable) {
      int *nullableCopy = nullable;
      if (nullableCopy) {
        *nullable;
        /*[[safe]]*/
      } else {
        *nullable;
        /*[[unsafe-1]]*/
      }
      *nullable;
      /*[[unsafe-2]]*/
    }
  )";
  expectDataflow(
      DerefNullableCheckCopy,
      UnorderedElementsAre(Pair("safe", IsSafe()), Pair("unsafe-1", IsUnsafe()),
                           Pair("unsafe-2", IsUnsafe())));
}

TEST_F(PointerNullabilityTest, CheckByComparisonToNullPtr) {
  std::string NENullRight = R"(
    void target(int *maybeNull) {
      if (maybeNull != nullptr) {
        *maybeNull;
        /*[[safe]]*/
      } else {
        *maybeNull;
        /*[[unsafe-1]]*/
      }
      *maybeNull;
      /*[[unsafe-2]]*/
    }
  )";
  expectDataflow(
      NENullRight,
      UnorderedElementsAre(Pair("safe", IsSafe()), Pair("unsafe-1", IsUnsafe()),
                           Pair("unsafe-2", IsUnsafe())));

  std::string NENullLeft = R"(
    void target(int *maybeNull) {
      if (nullptr != maybeNull) {
        *maybeNull;
        /*[[safe]]*/
      } else {
        *maybeNull;
        /*[[unsafe-1]]*/
      }
      *maybeNull;
      /*[[unsafe-2]]*/
    }
  )";
  expectDataflow(
      NENullLeft,
      UnorderedElementsAre(Pair("safe", IsSafe()), Pair("unsafe-1", IsUnsafe()),
                           Pair("unsafe-2", IsUnsafe())));

  std::string EQNullRight = R"(
    void target(int* maybeNull) {
      if (maybeNull == nullptr) {
        *maybeNull;
        /*[[unsafe-1]]*/
      } else {
        *maybeNull;
        /*[[safe]]*/
      }
      *maybeNull;
      /*[[unsafe-2]]*/
    }
  )";
  expectDataflow(
      EQNullRight,
      UnorderedElementsAre(Pair("safe", IsSafe()), Pair("unsafe-1", IsUnsafe()),
                           Pair("unsafe-2", IsUnsafe())));

  std::string EQNullLeft = R"(
    void target(int* maybeNull) {
      if (nullptr == maybeNull) {
        *maybeNull;
        /*[[unsafe-1]]*/
      } else {
        *maybeNull;
        /*[[safe]]*/
      }
      *maybeNull;
      /*[[unsafe-2]]*/
    }
  )";
  expectDataflow(
      EQNullLeft,
      UnorderedElementsAre(Pair("safe", IsSafe()), Pair("unsafe-1", IsUnsafe()),
                           Pair("unsafe-2", IsUnsafe())));
}

TEST_F(PointerNullabilityTest, CheckByImplicitCastToBool) {
  std::string PointerAsBool = R"(
    void target(int* maybeNull) {
      if (maybeNull) {
        *maybeNull;
        /*[[safe]]*/
      } else {
        *maybeNull;
        /*[[unsafe-1]]*/
      }
      *maybeNull;
      /*[[unsafe-2]]*/
    }
  )";
  expectDataflow(
      PointerAsBool,
      UnorderedElementsAre(Pair("safe", IsSafe()), Pair("unsafe-1", IsUnsafe()),
                           Pair("unsafe-2", IsUnsafe())));

  std::string PointerAsBoolNegated = R"(
    void target(int* maybeNull) {
      if (!maybeNull) {
        *maybeNull;
        /*[[unsafe-1]]*/
      } else {
        *maybeNull;
        /*[[safe]]*/
      }
      *maybeNull;
      /*[[unsafe-2]]*/
    }
  )";
  expectDataflow(
      PointerAsBoolNegated,
      UnorderedElementsAre(Pair("safe", IsSafe()), Pair("unsafe-1", IsUnsafe()),
                           Pair("unsafe-2", IsUnsafe())));
}

TEST_F(PointerNullabilityTest, CheckByComparisonToOtherNullPtr) {
  std::string NEOtherNullPtr = R"(
    void target(int* maybeNull) {
      int *null = nullptr;
      if (maybeNull != null) {
        *maybeNull;
        /*[[safe]]*/
      } else {
        *maybeNull;
        /*[[unsafe-1]]*/
      }
      *maybeNull;
      /*[[unsafe-2]]*/
    }
  )";
  expectDataflow(
      NEOtherNullPtr,
      UnorderedElementsAre(Pair("safe", IsSafe()), Pair("unsafe-1", IsUnsafe()),
                           Pair("unsafe-2", IsUnsafe())));

  std::string EQOtherNullPtr = R"(
    void target(int* maybeNull) {
      int *null = nullptr;
      if (maybeNull == null) {
        *maybeNull;
        /*[[unsafe-1]]*/
      } else {
        *maybeNull;
        /*[[safe]]*/
      }
      *maybeNull;
      /*[[unsafe-2]]*/
    }
  )";
  expectDataflow(
      EQOtherNullPtr,
      UnorderedElementsAre(Pair("safe", IsSafe()), Pair("unsafe-1", IsUnsafe()),
                           Pair("unsafe-2", IsUnsafe())));
}

TEST_F(PointerNullabilityTest, CheckByComparisonToOtherNonNullPtr) {
  std::string NEOtherNonNullPtr = R"(
    void target(int* maybeNull, int x) {
      int* nonNull = &x;
      if (maybeNull != nonNull) {
        *maybeNull;
        /*[[unsafe-1]]*/
      } else {
        *maybeNull;
        /*[[safe]]*/
      }
      *maybeNull;
      /*[[unsafe-2]]*/
    }
  )";
  expectDataflow(NEOtherNonNullPtr,
                 UnorderedElementsAre(Pair("unsafe-1", IsUnsafe()),
                                      Pair("unsafe-2", IsUnsafe()),
                                      Pair("safe", IsSafe())));

  std::string EQOtherNonNullPtr = R"(
    void target(int* maybeNull, int x) {
      int* nonNull = &x;
      if (maybeNull == nonNull) {
        *maybeNull;
        /*[[safe]]*/
      } else {
        *maybeNull;
        /*[[unsafe-1]]*/
      }
      *maybeNull;
      /*[[unsafe-2]]*/
    }
  )";
  expectDataflow(
      EQOtherNonNullPtr,
      UnorderedElementsAre(Pair("safe", IsSafe()), Pair("unsafe-1", IsUnsafe()),
                           Pair("unsafe-2", IsUnsafe())));
}

TEST_F(PointerNullabilityTest, CheckByComparisonToOtherUnknownPtr) {
  std::string NEOtherUnknownPtr = R"(
    void target(int* x, int* y) {
      if (x != y) {
        *x;
        /*[[unsafe-1]]*/
      } else {
        *x;
        /*[[unsafe-2]]*/
      }
      *x;
      /*[[unsafe-3]]*/
    }
  )";
  expectDataflow(NEOtherUnknownPtr,
                 UnorderedElementsAre(Pair("unsafe-1", IsUnsafe()),
                                      Pair("unsafe-2", IsUnsafe()),
                                      Pair("unsafe-3", IsUnsafe())));

  std::string EQOtherUnknownPtr = R"(
    void target(int* x, int* y) {
      if (x == y) {
        *x;
        /*[[unsafe-1]]*/
      } else {
        *x;
        /*[[unsafe-2]]*/
      }
      *x;
      /*[[unsafe-3]]*/
    }
  )";
  expectDataflow(EQOtherUnknownPtr,
                 UnorderedElementsAre(Pair("unsafe-1", IsUnsafe()),
                                      Pair("unsafe-2", IsUnsafe()),
                                      Pair("unsafe-3", IsUnsafe())));
}

TEST_F(PointerNullabilityTest, BinaryExpressions) {
  std::string And = R"(
    void target(int* x, int* y) {
      if (x && y) {
        *x;
        /*[[safe]]*/
      } else {
        *x;
        /*[[unsafe-1]]*/
      }
      *x;
      /*[[unsafe-2]]*/
    }
  )";
  expectDataflow(And, UnorderedElementsAre(Pair("safe", IsSafe()),
                                           Pair("unsafe-1", IsUnsafe()),
                                           Pair("unsafe-2", IsUnsafe())));

  std::string Or = R"(
    void target(int* x, int* y) {
      if (x || y) {
        *x;
        /*[[unsafe-1]]*/
      } else {
        *x;
        /*[[unsafe-2]]*/
      }
      *x;
      /*[[unsafe-3]]*/
    }
  )";
  expectDataflow(Or, UnorderedElementsAre(Pair("unsafe-1", IsUnsafe()),
                                          Pair("unsafe-2", IsUnsafe()),
                                          Pair("unsafe-3", IsUnsafe())));

  std::string AndNegatedBoth = R"(
    void target(int* x, int* y) {
      if (!x && !y) {
        *x;
        /*[[unsafe-1]]*/
      } else {
        *x;
        /*[[unsafe-2]]*/
      }
      *x;
      /*[[unsafe-3]]*/
    }
  )";
  expectDataflow(AndNegatedBoth,
                 UnorderedElementsAre(Pair("unsafe-1", IsUnsafe()),
                                      Pair("unsafe-2", IsUnsafe()),
                                      Pair("unsafe-3", IsUnsafe())));

  std::string OrNegatedBoth = R"(
    void target(int* x, int* y) {
      if (!x || !y) {
        *x;
        /*[[unsafe-1]]*/
      } else {
        *x;
        /*[[safe]]*/
      }
      *x;
      /*[[unsafe-2]]*/
    }
  )";
  expectDataflow(
      OrNegatedBoth,
      UnorderedElementsAre(Pair("safe", IsSafe()), Pair("unsafe-1", IsUnsafe()),
                           Pair("unsafe-2", IsUnsafe())));
}

TEST_F(PointerNullabilityTest, MemberPointers) {
  std::string DerefStructMember = R"(
    struct Foo {
      Foo* ptr;
    };
    void target(Foo foo) {
      if (foo.ptr) {
        *foo.ptr;
        /*[[safe]]*/
      } else {
        *foo.ptr;
        /*[[unsafe-1]]*/
      }
      *foo.ptr;
      /*[[unsafe-2]]*/
    }
  )";
  expectDataflow(
      DerefStructMember,
      UnorderedElementsAre(Pair("safe", IsSafe()), Pair("unsafe-1", IsUnsafe()),
                           Pair("unsafe-2", IsUnsafe())));

  std::string DerefClassMember = R"(
    class Foo {
     public:
      Foo* ptr;
    };
    void target(Foo foo) {
      if (foo.ptr) {
        *foo.ptr;
        /*[[safe]]*/
      } else {
        *foo.ptr;
        /*[[unsafe-1]]*/
      }
      *foo.ptr;
      /*[[unsafe-2]]*/
    }
  )";
  expectDataflow(
      DerefClassMember,
      UnorderedElementsAre(Pair("safe", IsSafe()), Pair("unsafe-1", IsUnsafe()),
                           Pair("unsafe-2", IsUnsafe())));
}

TEST_F(PointerNullabilityTest, MemberAccessOnPointer) {
  std::string MemberAccess = R"(
    struct Foo {
      void foo();
    };
    void target(Foo* foo) {
      if (foo) {
        foo->foo();
        /*[[safe]]*/
      } else {
        foo->foo();
        /*[[unsafe-1]]*/
      }
      foo->foo();
      /*[[unsafe-2]]*/
    }
  )";
  expectDataflow(
      MemberAccess,
      UnorderedElementsAre(Pair("safe", IsSafe()), Pair("unsafe-1", IsUnsafe()),
                           Pair("unsafe-2", IsUnsafe())));

  std::string AccessChainOnlyCheckOnFirst = R"(
    struct Foo {
      Foo* foo;
    };
    void target(Foo* foo) {
      if (foo) {
        foo->foo->foo;
        /*[[unsafe-1]]*/
      } else {
        foo->foo->foo;
        /*[[unsafe-2]]*/
      }
      foo->foo->foo;
      /*[[unsafe-3]]*/
    }
  )";
  expectDataflow(AccessChainOnlyCheckOnFirst,
                 UnorderedElementsAre(Pair("unsafe-1", IsUnsafe()),
                                      Pair("unsafe-2", IsUnsafe()),
                                      Pair("unsafe-3", IsUnsafe())));

  std::string AccessChainCheckOnAll = R"(
    struct Foo {
      Foo* foo;
    };
    void target(Foo* foo) {
      if (foo && foo->foo) {
        foo->foo->foo;
        /*[[safe]]*/
      } else {
        foo->foo;
        /*[[unsafe-1]]*/
      }
      foo->foo;
      /*[[unsafe-2]]*/
    }
  )";
  expectDataflow(
      AccessChainCheckOnAll,
      UnorderedElementsAre(Pair("safe", IsSafe()), Pair("unsafe-1", IsUnsafe()),
                           Pair("unsafe-2", IsUnsafe())));
}

}  // namespace
}  // namespace nullability
}  // namespace tidy
}  // namespace clang
