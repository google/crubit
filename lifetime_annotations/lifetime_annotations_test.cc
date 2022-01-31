// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "lifetime_annotations/lifetime_annotations.h"

#include <string>
#include <utility>

#include "lifetime_annotations/test/named_func_lifetimes.h"
#include "lifetime_annotations/test/run_on_code.h"
#include "testing/base/public/gmock.h"
#include "testing/base/public/gunit.h"
#include "third_party/llvm/llvm-project/clang/include/clang/ASTMatchers/ASTMatchFinder.h"
#include "third_party/llvm/llvm-project/clang/include/clang/ASTMatchers/ASTMatchers.h"
#include "third_party/llvm/llvm-project/llvm/include/llvm/Support/FormatVariadic.h"

namespace devtools_rust {
namespace {

using testing::StartsWith;
using testing::status::IsOkAndHolds;
using testing::status::StatusIs;

bool IsOverloaded(const clang::FunctionDecl* func) {
  return !func->getDeclContext()->lookup(func->getDeclName()).isSingleResult();
}

std::string QualifiedName(const clang::FunctionDecl* func) {
  std::string str;
  llvm::raw_string_ostream ostream(str);
  func->printQualifiedName(ostream);
  if (IsOverloaded(func)) {
    ostream << "[" << func->getType().getAsString() << "]";
  }
  ostream.flush();
  return str;
}

class LifetimeAnnotationsTest : public testing::Test {
 protected:
  absl::StatusOr<NamedFuncLifetimes> GetNamedLifetimeAnnotations(
      absl::string_view code,
      const clang::tooling::FileContentMappings& file_contents =
          clang::tooling::FileContentMappings()) {
    absl::StatusOr<NamedFuncLifetimes> result;
    runOnCodeWithLifetimeHandlers(
        llvm::StringRef(code.data(), code.size()),
        [&result](clang::ASTContext& ast_context,
                  const LifetimeAnnotationContext& lifetime_context) {
          using clang::ast_matchers::findAll;
          using clang::ast_matchers::functionDecl;
          using clang::ast_matchers::match;

          NamedFuncLifetimes named_func_lifetimes;
          for (const auto& node :
               match(findAll(functionDecl().bind("func")), ast_context)) {
            if (const auto* func =
                    node.getNodeAs<clang::FunctionDecl>("func")) {
              LifetimeSymbolTable symbol_table;
              llvm::Expected<FunctionLifetimes> func_lifetimes =
                  GetLifetimeAnnotations(func, lifetime_context, &symbol_table);

              if (!func_lifetimes) {
                result = absl::UnknownError(
                    llvm::toString(func_lifetimes.takeError()));
                return;
              }
              std::string func_name = QualifiedName(func);
              std::string new_entry =
                  NameLifetimes(*func_lifetimes, symbol_table);
              std::optional<llvm::StringRef> old_entry =
                  named_func_lifetimes.Get(func_name);
              if (old_entry.has_value()) {
                if (new_entry != old_entry.value()) {
                  result = absl::UnknownError(
                      llvm::formatv(
                          "Unexpectedly different lifetimes for function '{0}'."
                          "Old: '{1}'. New: '{2}'.",
                          func_name, old_entry.value(), new_entry)
                          .str());
                  return;
                }
              } else {
                named_func_lifetimes.Add(std::move(func_name),
                                         std::move(new_entry));
              }
            }
          }

          result = std::move(named_func_lifetimes);
        },
        {}, file_contents);

    return result;
  }
};

TEST_F(LifetimeAnnotationsTest, NoLifetimes) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(R"(
        int f(int);
  )"),
              IsOkAndHolds(LifetimesAre({{"f", "()"}})));
}

TEST_F(LifetimeAnnotationsTest, Failure_NoAnnotationsNoLifetimeElision) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(R"(
        int** f(int*);
  )"),
              StatusIs(absl::StatusCode::kUnknown,
                       StartsWith("Lifetime elision not enabled")));
}

TEST_F(LifetimeAnnotationsTest, Failure_NoOutputAnnotationNoLifetimeElision) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(R"(
        int* f();
  )"),
              StatusIs(absl::StatusCode::kUnknown,
                       // We specifically want to see this error message rather
                       // than "Cannot elide output lifetimes".
                       StartsWith("Lifetime elision not enabled")));
}

TEST_F(LifetimeAnnotationsTest, Failure_NoAnnotationsElisionPragmaInWrongFile) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(R"(
        #pragma clang lifetime_elision
        #include "header.h"
  )",
                                          {std::make_pair("header.h", R"(
        int** f(int*);
  )")}),
              StatusIs(absl::StatusCode::kUnknown,
                       StartsWith("Lifetime elision not enabled")));
}

TEST_F(LifetimeAnnotationsTest, LifetimeElision_OneInputLifetime) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(R"(
        #pragma clang lifetime_elision
        int** f(int*);
  )"),
              IsOkAndHolds(LifetimesAre({{"f", "a -> (a, a)"}})));
}

TEST_F(LifetimeAnnotationsTest, FunctionPointerLifetimes) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(R"(
        #pragma clang lifetime_elision
        void f(void (*)());
  )"),
              IsOkAndHolds(LifetimesAre({{"f", "a"}})));
}

TEST_F(LifetimeAnnotationsTest, FunctionPointerAsTypedefLifetimes) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(R"(
        #pragma clang lifetime_elision
        typedef void (*FunctionPointer)();
        void f(FunctionPointer hook);
  )"),
              IsOkAndHolds(LifetimesAre({{"f", "a"}})));
}

TEST_F(LifetimeAnnotationsTest, FunctionReferenceLifetimes) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(R"(
        #pragma clang lifetime_elision
        typedef void (&FunctionReference)();
        void f(FunctionReference hook);
  )"),
              IsOkAndHolds(LifetimesAre({{"f", "a"}})));
}

TEST_F(LifetimeAnnotationsTest, FunctionReferenceAsTypedefLifetimes) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(R"(
        #pragma clang lifetime_elision
        void f(void (&)());
  )"),
              IsOkAndHolds(LifetimesAre({{"f", "a"}})));
}

TEST_F(LifetimeAnnotationsTest, ArrayParamLifetimes) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(R"(
        #pragma clang lifetime_elision
        void f(int pair[2]);
  )"),
              IsOkAndHolds(LifetimesAre({{"f", "a"}})));
}

TEST_F(LifetimeAnnotationsTest, ArrayParamAsTypedefLifetimes) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(R"(
        #pragma clang lifetime_elision
        typesef int Arr[2];
        void f(Arr);
  )"),
              IsOkAndHolds(LifetimesAre({{"f", "a"}})));
}

TEST_F(LifetimeAnnotationsTest, LifetimeElision_NoOutputLifetimes) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(R"(
        #pragma clang lifetime_elision
        void f(int**, int *);
  )"),
              IsOkAndHolds(LifetimesAre({{"f", "(a, b), c"}})));
}

TEST_F(LifetimeAnnotationsTest, LifetimeElision_Templates) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(R"(
        #pragma clang lifetime_elision
        template <class T> class vector {};
        int* f(vector<int *>);
        vector<int*> g(int *);
  )"),
              IsOkAndHolds(LifetimesAre({{"f", "a -> a"}, {"g", "a -> a"}})));
}

TEST_F(LifetimeAnnotationsTest, LifetimeElision_Method) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(R"(
        #pragma clang lifetime_elision
        struct S {
          int** method(int *, int *);
        };
  )"),
              IsOkAndHolds(LifetimesAre({{"S::method", "a: b, c -> (a, a)"}})));
}

TEST_F(LifetimeAnnotationsTest, LifetimeElision_FailureTooFewInputLifetimes) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(R"(
        #pragma clang lifetime_elision
        int* f();
  )"),
              StatusIs(absl::StatusCode::kUnknown,
                       StartsWith("Cannot elide output lifetimes")));
}

TEST_F(LifetimeAnnotationsTest, LifetimeElision_FailureTooManyInputLifetimes) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(R"(
        #pragma clang lifetime_elision
        int* f(int**);
  )"),
              StatusIs(absl::StatusCode::kUnknown,
                       StartsWith("Cannot elide output lifetimes")));
}

TEST_F(LifetimeAnnotationsTest, LifetimeAnnotationsNoLifetimes) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(R"_(
        [[clang::annotate("lifetimes = ()")]]
        void f(int);
  )_"),
              IsOkAndHolds(LifetimesAre({{"f", "()"}})));
}

TEST_F(LifetimeAnnotationsTest, LifetimeAnnotationsSimple) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(R"(
        [[clang::annotate("lifetimes = a -> a")]]
        int* f(int*);
  )"),
              IsOkAndHolds(LifetimesAre({{"f", "a -> a"}})));
}

TEST_F(LifetimeAnnotationsTest, LifetimeAnnotationsMultiplePtr) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(R"(
        [[clang::annotate("lifetimes = (a, b) -> a")]]
        int* f(int**);
  )"),
              IsOkAndHolds(LifetimesAre({{"f", "(a, b) -> a"}})));
}

TEST_F(LifetimeAnnotationsTest, LifetimeAnnotationsMultipleArguments) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(R"(
        [[clang::annotate("lifetimes = a, b -> a")]]
        int* f(int*, int*);
  )"),
              IsOkAndHolds(LifetimesAre({{"f", "a, b -> a"}})));
}

TEST_F(LifetimeAnnotationsTest, LifetimeAnnotationsNoReturn) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(R"(
        [[clang::annotate("lifetimes = a, b")]]
        void f(int*, int*);
  )"),
              IsOkAndHolds(LifetimesAre({{"f", "a, b"}})));
}

TEST_F(LifetimeAnnotationsTest, LifetimeAnnotationsNoLifetimeParam) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(R"(
        [[clang::annotate("lifetimes = a, (), a -> a")]]
        int* f(int*, int, int*);
  )"),
              IsOkAndHolds(LifetimesAre({{"f", "a, (), a -> a"}})));
}

TEST_F(LifetimeAnnotationsTest, LifetimeAnnotationsMethod) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(R"(
        struct S {
          [[clang::annotate("lifetimes = a: -> a")]]
          int* f();
        };
  )"),
              IsOkAndHolds(LifetimesAre({{"S::f", "a: -> a"}})));
}

TEST_F(LifetimeAnnotationsTest, LifetimeAnnotationsMethodWithParam) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(R"(
        struct S {
          [[clang::annotate("lifetimes = a: b -> a")]]
          int* f(int*);
        };
  )"),
              IsOkAndHolds(LifetimesAre({{"S::f", "a: b -> a"}})));
}

TEST_F(LifetimeAnnotationsTest, LifetimeAnnotationsInvalid_MissingThis) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(R"(
        struct S {
          [[clang::annotate("lifetimes = -> a")]]
          int* f();
        };
  )"),
              StatusIs(absl::StatusCode::kUnknown,
                       StartsWith("Invalid lifetime annotation")));
}

TEST_F(LifetimeAnnotationsTest, LifetimeAnnotationsInvalid_ThisOnFreeFunction) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(R"(
        [[clang::annotate("lifetimes = a: a -> a")]]
        int* f(int*);
  )"),
              StatusIs(absl::StatusCode::kUnknown,
                       StartsWith("Invalid lifetime annotation")));
}

TEST_F(LifetimeAnnotationsTest, LifetimeAnnotationsInvalid_WrongNumber) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(R"(
        [[clang::annotate("lifetimes = a -> a")]]
        int* f(int**);
  )"),
              StatusIs(absl::StatusCode::kUnknown,
                       StartsWith("Invalid lifetime annotation")));
}

TEST_F(LifetimeAnnotationsTest, LifetimeElision_ExplicitlyDefaultedCtor) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(R"(
          #pragma clang lifetime_elision
          struct S {
            S() = default;
          };)"),
              IsOkAndHolds(LifetimesAre({{"S::S", "a:"}})));
}

TEST_F(LifetimeAnnotationsTest, LifetimeElision_ImplicitlyDefaultedCtor) {
  // Implicitly-defaulted constructors don't have associated `TypeSourceInfo`.
  EXPECT_THAT(
      GetNamedLifetimeAnnotations(R"(
          #pragma clang lifetime_elision
          struct S {};
          // We need to use the implicitly-defaulted constructors to make
          // them appear in the AST so that we can process them.
          void foo() { S s; }
          )"),
      IsOkAndHolds(LifetimesContain({{"S::S[void (void) noexcept]", "a:"}})));
}

}  // namespace
}  // namespace devtools_rust
