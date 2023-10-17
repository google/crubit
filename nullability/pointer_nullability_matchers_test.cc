// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/pointer_nullability_matchers.h"

#include "clang/ASTMatchers/ASTMatchFinder.h"
#include "clang/Testing/TestAST.h"
#include "llvm/ADT/StringRef.h"
#include "third_party/llvm/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang::tidy::nullability {
namespace {

using ast_matchers::match;

template <typename MatcherT>
bool matches(llvm::StringRef base_input, llvm::StringRef test_input,
             MatcherT Matcher) {
  TestAST InputAST(base_input.str() + test_input.str());
  return !match(Matcher, InputAST.context()).empty();
}

TEST(PointerNullabilityTest, MatchMemberFunctions) {
  llvm::StringRef Input(R"cc(
    struct DummyStruct {
      int *p;
    };
    class C {
     public:
      int *_Nullable get() const { return x; }
      int *_Nullable get_member_in_parens() const { return (x); }
      int *_Nullable get_this_in_parens() const { return (this)->x; }
      int *_Nullable get(int i) const { return x; }
      int *_Nullable get_nonconst() { return x; }
      int *_Nullable get_external() { return ds.p; }
      void mutate(){};

     private:
      int *x;
      DummyStruct ds;
    };
  )cc");

  EXPECT_TRUE(matches(Input, "void target(){ C().get(); }",
                      isSupportedPointerAccessorCall()));
  EXPECT_TRUE(matches(Input, "void target(){ C().get_member_in_parens(); }",
                      isSupportedPointerAccessorCall()));
  EXPECT_TRUE(matches(Input, "void target(){ C().get_this_in_parens(); }",
                      isSupportedPointerAccessorCall()));
  EXPECT_TRUE(matches(Input, "void target(){ C().get(0); }",
                      isSupportedPointerAccessorCall()));
  EXPECT_TRUE(matches(Input, "void target(){ C().get_nonconst(); }",
                      isSupportedPointerAccessorCall()));

  EXPECT_FALSE(matches(Input, "void target(){ C().mutate(); }",
                       isSupportedPointerAccessorCall()));
  EXPECT_FALSE(matches(Input, "void target(){ C().get_external(); }",
                       isSupportedPointerAccessorCall()));
}

}  // namespace
}  // namespace clang::tidy::nullability
