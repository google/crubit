// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/pointer_nullability_matchers.h"

#include "nullability/type_nullability.h"
#include "clang/ASTMatchers/ASTMatchFinder.h"
#include "clang/Testing/TestAST.h"
#include "llvm/ADT/StringRef.h"
#include "third_party/llvm/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang::tidy::nullability {
namespace {

// Static initializer turns on support for smart pointers.
test::EnableSmartPointers Enable;

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
      void may_mutate(){};

     private:
      int *x;
      DummyStruct ds;
    };
    C foo() { return C(); }
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
  EXPECT_TRUE(matches(Input, "void target(){ foo().get(); }",
                      isSupportedPointerAccessorCall()));

  EXPECT_FALSE(matches(Input, "void target(){ C().may_mutate(); }",
                       isSupportedPointerAccessorCall()));
  EXPECT_FALSE(matches(Input, "void target(){ C().get_external(); }",
                       isSupportedPointerAccessorCall()));
}

TEST(PointerNullabilityTest, MatchConstMemberFunctions) {
  llvm::StringRef Input(R"cc(
    class C {
     public:
      int *_Nullable get() const;
      int *_Nullable get(int i) const;
      int *_Nullable get_with_default_arg(int i = 0) const;
      int *_Nullable get_nonconst();
    };
    C foo() { return C(); }
  )cc");
  EXPECT_TRUE(matches(Input, "void target(){ C().get(); }",
                      isZeroParamConstMemberCall()));
  EXPECT_TRUE(matches(Input, "void target(){ foo().get(); }",
                      isZeroParamConstMemberCall()));

  EXPECT_FALSE(matches(Input, "void target(){ C().get(0); }",
                       isZeroParamConstMemberCall()));
  EXPECT_FALSE(matches(Input, "void target(){ C().get_with_default_arg(); }",
                       isZeroParamConstMemberCall()));
  EXPECT_FALSE(matches(Input, "void target(){ C().get_nonconst(); }",
                       isZeroParamConstMemberCall()));
}

TEST(PointerNullabilityTest, MatchSmartPointerMethodCall) {
  llvm::StringRef Input(R"cc(
    namespace std {
    template <class T>
    struct unique_ptr {
      using pointer = T *;
      T *get() const;
    };
    }  // namespace std
    template <class T>
    struct MyUniquePtr {
      using pointer = T *;
      T *get() const;
    };
  )cc");
  // Call using `.method()` syntax.
  EXPECT_TRUE(matches(Input, "void target(){ std::unique_ptr<int>().get(); }",
                      isSmartPointerMethodCall("get")));
  // Call using `->method()` syntax.
  EXPECT_TRUE(matches(Input,
                      "void target(std::unique_ptr<int> *p) { p->get(); }",
                      isSmartPointerMethodCall("get")));
  // Querying for wrong method name.
  EXPECT_FALSE(matches(Input, "void target(){ std::unique_ptr<int>().get(); }",
                       isSmartPointerMethodCall("reset")));
  // Not a supported smart pointer type.
  EXPECT_FALSE(matches(Input, "void target(){ MyUniquePtr<int>().get(); }",
                       isSmartPointerMethodCall("get")));
}

TEST(PointerNullabilityTest, MatchSmartPointerBoolConversionCall) {
  llvm::StringRef Input(R"cc(
    namespace std {
    template <class T>
    struct unique_ptr {
      using pointer = T*;
      explicit operator bool() const;
    };
    }  // namespace std
    template <class T>
    struct MyUniquePtr {
      using pointer = T*;
      explicit operator bool() const;
    };
  )cc");
  // Call using `static_cast<bool>()` syntax.
  EXPECT_TRUE(matches(
      Input, "void target(){ static_cast<bool>(std::unique_ptr<int>()); }",
      isSmartPointerBoolConversionCall()));
  // Explicit call using `.method()` syntax.
  EXPECT_TRUE(
      matches(Input, "void target(){ std::unique_ptr<int>().operator bool(); }",
              isSmartPointerBoolConversionCall()));
  // Explicit call using `->method()` syntax.
  EXPECT_TRUE(matches(
      Input, "void target(std::unique_ptr<int> *p) { p->operator bool(); }",
      isSmartPointerBoolConversionCall()));
  // Not a supported smart pointer type.
  EXPECT_FALSE(
      matches(Input, "void target(){ static_cast<bool>(MyUniquePtr<int>()); }",
              isSmartPointerBoolConversionCall()));
}

}  // namespace
}  // namespace clang::tidy::nullability
