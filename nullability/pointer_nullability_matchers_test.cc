// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/pointer_nullability_matchers.h"

#include "clang/ASTMatchers/ASTMatchFinder.h"
#include "clang/ASTMatchers/ASTMatchers.h"
#include "clang/Testing/TestAST.h"
#include "llvm/ADT/StringRef.h"
#include "external/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang::tidy::nullability {
namespace {

using ast_matchers::callee;
using ast_matchers::callExpr;
using ast_matchers::declRefExpr;
using ast_matchers::enumConstantDecl;
using ast_matchers::functionDecl;
using ast_matchers::hasName;
using ast_matchers::match;
using ast_matchers::to;

template <typename MatcherT>
bool matches(llvm::StringRef test_input, MatcherT Matcher) {
  TestAST InputAST(test_input.str());
  return !match(Matcher, InputAST.context()).empty();
}

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
      C *_Nullable operator->() const;
      C &operator++();
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
  EXPECT_TRUE(matches(Input, "void target(){ C()->get(); }",
                      isZeroParamConstMemberOperatorCall()));
  EXPECT_FALSE(matches(Input, "void target(){ ++C(); }",
                       isZeroParamConstMemberOperatorCall()));
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

TEST(AbslNamespaceTest, MatchesAbslFunctionCall) {
  llvm::StringRef Input(R"cc(
    namespace absl {
    void f() {}
    }  // namespace absl
  )cc");
  auto AbslFunctionCall =
      callExpr(callee(functionDecl(isInAbslNamespace(), hasName("f"))));
  EXPECT_TRUE(matches(Input, "void target(){ absl::f(); }", AbslFunctionCall));
  EXPECT_TRUE(matches(Input, "using namespace absl; void target(){ f(); }",
                      AbslFunctionCall));
  EXPECT_TRUE(
      matches(Input, "using absl::f; void target(){ f(); }", AbslFunctionCall));
}

TEST(AbslNamespaceTest, DoesNotMatchFunctionCallWithNoNamespace) {
  llvm::StringRef Input(R"cc(
    void f() {}
    void target() { f(); }
  )cc");
  EXPECT_FALSE(matches(Input, isInAbslNamespace()));
}

TEST(AbslNamespaceTest, DoesNotMatchFunctionCallWithDifferentNamespace) {
  llvm::StringRef Input(R"cc(
    namespace util {
    void f() {}
    }  // namespace util
  )cc");
  EXPECT_FALSE(
      matches(Input, "void target(){ util::f(); }", isInAbslNamespace()));
  EXPECT_FALSE(matches(Input, "using namespace util; void target(){ f(); }",
                       isInAbslNamespace()));
  EXPECT_FALSE(matches(Input, "using util::f; void target(){ f(); }",
                       isInAbslNamespace()));
}

TEST(AbslNamespaceTest, DoesNotMatchEnumWithAbslType) {
  llvm::StringRef Input(R"cc(
    enum absl {
      kValue,
    };
  )cc");
  EXPECT_FALSE(
      matches(Input, "void target() { kValue; }", isInAbslNamespace()));
  EXPECT_FALSE(
      matches(Input, "void target() { absl::kValue; }", isInAbslNamespace()));
}

TEST(AbslNamespaceTest, DoesNotMatchEnumClassWithQualifiedAbslType) {
  llvm::StringRef Input(R"cc(
    enum class absl {
      kValue,
    };
    void target() { absl::kValue; }
  )cc");
  EXPECT_FALSE(matches(Input, isInAbslNamespace()));
}

TEST(AbslNamespaceTest, MatchesEnumInAbslNamespace) {
  llvm::StringRef Input(R"cc(
    namespace absl {
    enum Type {
      kValue,
    };
    }  // namespace absl
  )cc");
  EXPECT_TRUE(matches(Input, "void target() { absl::kValue; }",
                      declRefExpr(to(enumConstantDecl(isInAbslNamespace(),
                                                      hasName("kValue"))))));
  EXPECT_TRUE(matches(Input, "void target() { absl::Type::kValue; }",
                      declRefExpr(to(enumConstantDecl(isInAbslNamespace(),
                                                      hasName("kValue"))))));
}

TEST(AbslNamespaceTest, MatchesEnumClassInAbslNamespace) {
  llvm::StringRef Input(R"cc(
    namespace absl {
    enum class Type {
      kValue,
    };
    void target() { absl::Type::kValue; }
    }  // namespace absl
  )cc");
  EXPECT_TRUE(matches(Input, declRefExpr(to(enumConstantDecl(
                                 isInAbslNamespace(), hasName("kValue"))))));
}

TEST(AbslNamespaceTest, MatchesFunctionCallWithNestedNamespace) {
  llvm::StringRef Input(R"cc(
    namespace absl {
    namespace nested {
    void f() {}
    }  // namespace nested
    }  // namespace absl
  )cc");
  auto AbslFunctionCall =
      callExpr(callee(functionDecl(isInAbslNamespace(), hasName("f"))));
  EXPECT_TRUE(
      matches(Input, "void target(){ absl::nested::f(); }", AbslFunctionCall));
  EXPECT_TRUE(matches(Input,
                      "using namespace absl; void target(){ nested::f(); }",
                      AbslFunctionCall));
  EXPECT_TRUE(matches(Input,
                      "using namespace absl::nested; void target(){ f(); }",
                      AbslFunctionCall));
  EXPECT_TRUE(matches(Input, "using absl::nested::f; void target(){ f(); }",
                      AbslFunctionCall));
}

TEST(AbslNamespaceTest, MatchesFunctionCallWithInlineNamespace) {
  llvm::StringRef Input(R"cc(
    namespace absl {
    inline namespace latest {
    void f() {}
    }  // namespace latest
    }  // namespace absl
  )cc");
  auto AbslFunctionCall =
      callExpr(callee(functionDecl(isInAbslNamespace(), hasName("f"))));
  EXPECT_TRUE(matches(Input, "void target(){ absl::f(); }", AbslFunctionCall));
  EXPECT_TRUE(
      matches(Input, "void target(){ absl::latest::f(); }", AbslFunctionCall));
  EXPECT_TRUE(matches(Input, "using namespace absl; void target(){ f(); }",
                      AbslFunctionCall));
  EXPECT_TRUE(matches(Input,
                      "using namespace absl; void target(){ latest::f(); }",
                      AbslFunctionCall));
  EXPECT_TRUE(matches(Input,
                      "using namespace absl::latest; void target(){ f(); }",
                      AbslFunctionCall));
  EXPECT_TRUE(
      matches(Input, "using absl::f; void target(){ f(); }", AbslFunctionCall));
  EXPECT_TRUE(matches(Input, "using absl::latest::f; void target(){ f(); }",
                      AbslFunctionCall));
}

TEST(AbslNamespaceTest, MatchesFunctionCallWithNestedInlineNamespace) {
  llvm::StringRef Input(R"cc(
    namespace absl {
    inline namespace latest {
    inline namespace nested {
    void f() {}
    }  // namespace nested
    }  // namespace latest
    }  // namespace absl
  )cc");
  auto AbslFunctionCall =
      callExpr(callee(functionDecl(isInAbslNamespace(), hasName("f"))));
  EXPECT_TRUE(matches(Input, "void target(){ absl::f(); }", AbslFunctionCall));
  EXPECT_TRUE(
      matches(Input, "void target(){ absl::latest::f(); }", AbslFunctionCall));
  EXPECT_TRUE(matches(Input, "void target(){ absl::latest::nested::f(); }",
                      AbslFunctionCall));
}

TEST(AbslNamespaceTest, DoesNotMatchFunctionCallWithNestedAbslNamespace) {
  llvm::StringRef Input(R"cc(
    namespace util {
    namespace absl {
    void f() {}
    }  // namespace absl
    }  // namespace util
  )cc");
  EXPECT_FALSE(
      matches(Input, "void target(){ util::absl::f(); }", isInAbslNamespace()));
  EXPECT_FALSE(matches(Input,
                       "using namespace util; void target(){ absl::f(); }",
                       isInAbslNamespace()));
  EXPECT_FALSE(matches(Input,
                       "using namespace util::absl; void target(){ f(); }",
                       isInAbslNamespace()));
}

TEST(AbslNamespaceTest, DoesNotMatchFunctionCallWithInlineAbslNamespace) {
  llvm::StringRef Input(R"cc(
    namespace util {
    inline namespace absl {
    void f() {}
    }  // namespace absl
    }  // namespace util
  )cc");
  EXPECT_FALSE(
      matches(Input, "void target(){ util::f(); }", isInAbslNamespace()));
  EXPECT_FALSE(
      matches(Input, "void target(){ util::absl::f(); }", isInAbslNamespace()));
  EXPECT_FALSE(matches(Input,
                       "using namespace util; void target(){ absl::f(); }",
                       isInAbslNamespace()));
  EXPECT_FALSE(matches(Input,
                       "using namespace util::absl; void target(){ f(); }",
                       isInAbslNamespace()));
}

}  // namespace
}  // namespace clang::tidy::nullability
