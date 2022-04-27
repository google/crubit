// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "lifetime_annotations/lifetime_annotations.h"

#include <string>
#include <utility>

#include "testing/base/public/gmock.h"
#include "testing/base/public/gunit.h"
#include "lifetime_annotations/test/named_func_lifetimes.h"
#include "lifetime_annotations/test/run_on_code.h"
#include "clang/ASTMatchers/ASTMatchFinder.h"
#include "clang/ASTMatchers/ASTMatchers.h"
#include "llvm/Support/FormatVariadic.h"

namespace clang {
namespace tidy {
namespace lifetimes {
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
    bool success = runOnCodeWithLifetimeHandlers(
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

    if (!success) {
      return absl::UnknownError(
          "Error extracting lifetimes. (Compilation error?)");
    }

    return result;
  }
};

TEST_F(LifetimeAnnotationsTest, NoLifetimes) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(R"(
        int f(int);
  )"),
              IsOkAndHolds(LifetimesAre({{"f", "()"}})));
}

TEST_F(LifetimeAnnotationsTest, Failure_CompileError) {
  EXPECT_THAT(
      GetNamedLifetimeAnnotations(R"(
        undefined f(undefined);
  )"),
      StatusIs(absl::StatusCode::kUnknown,
               StartsWith("Error extracting lifetimes. (Compilation error?)")));
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

TEST_F(LifetimeAnnotationsTest, LifetimeElision_NestedTemplates) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(R"(
        #pragma clang lifetime_elision
        template <class T>
        struct Outer {
          template <class U>
          struct Inner {
          };
        };
        void f(Outer<int *>::Inner<int *> &);
        Outer<int *>::Inner<int *> g(int *);
  )"),
              IsOkAndHolds(LifetimesAre(
                  {{"f", "(<a>::<b>, c)"}, {"g", "a -> <a>::<a>"}})));
}

TEST_F(LifetimeAnnotationsTest, LifetimeElision_LifetimeParameterizedType) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(R"(
        #pragma clang lifetime_elision
        struct [[clang::annotate("lifetime_params", "s")]] string_view{};
        string_view f(string_view);
  )"),
              IsOkAndHolds(LifetimesAre({{"f", "a -> a"}})));
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

TEST_F(LifetimeAnnotationsTest, LifetimeElision_ArrayParamLifetimes) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(R"(
        #pragma clang lifetime_elision
        void f(int pair[2]);
  )"),
              IsOkAndHolds(LifetimesAre({{"f", "a"}})));
}

TEST_F(LifetimeAnnotationsTest, LifetimeElision_ArrayParamAsTypedefLifetimes) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(R"(
        #pragma clang lifetime_elision
        typedef int Arr[2];
        void f(Arr);
  )"),
              IsOkAndHolds(LifetimesAre({{"f", "a"}})));
}

TEST_F(LifetimeAnnotationsTest, LifetimeElision_FunctionPointerLifetimes) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(R"(
        #pragma clang lifetime_elision
        void f(void (*)());
  )"),
              IsOkAndHolds(LifetimesAre({{"f", "a"}})));
}

TEST_F(LifetimeAnnotationsTest,
       LifetimeElision_FunctionPointerAsTypedefLifetimes) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(R"(
        #pragma clang lifetime_elision
        typedef void (*FunctionPointer)();
        void f(FunctionPointer hook);
  )"),
              IsOkAndHolds(LifetimesAre({{"f", "a"}})));
}

TEST_F(LifetimeAnnotationsTest, LifetimeElision_FunctionReferenceLifetimes) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(R"(
        #pragma clang lifetime_elision
        typedef void (&FunctionReference)();
        void f(FunctionReference hook);
  )"),
              IsOkAndHolds(LifetimesAre({{"f", "a"}})));
}

TEST_F(LifetimeAnnotationsTest,
       LifetimeElision_FunctionReferenceAsTypedefLifetimes) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(R"(
        #pragma clang lifetime_elision
        void f(void (&)());
  )"),
              IsOkAndHolds(LifetimesAre({{"f", "a"}})));
}

TEST_F(LifetimeAnnotationsTest,
       LifetimeElision_PointerToMemberDoesNotGetLifetime) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(R"(
        #pragma clang lifetime_elision
        struct S {};
        void f(int S::*ptr_to_member);
  )"),
              IsOkAndHolds(LifetimesAre({{"f", "()"}})));
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

TEST_F(LifetimeAnnotationsTest, LifetimeAnnotation_NoLifetimes) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(R"_(
        [[clang::annotate("lifetimes", "()")]]
        void f(int);
  )_"),
              IsOkAndHolds(LifetimesAre({{"f", "()"}})));
}

TEST_F(LifetimeAnnotationsTest, LifetimeAnnotation_Simple) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(R"(
        [[clang::annotate("lifetimes", "a -> a")]]
        int* f(int*);
  )"),
              IsOkAndHolds(LifetimesAre({{"f", "a -> a"}})));
}

TEST_F(LifetimeAnnotationsTest, LifetimeAnnotation_MultiplePtr) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(R"(
        [[clang::annotate("lifetimes", "(a, b) -> a")]]
        int* f(int**);
  )"),
              IsOkAndHolds(LifetimesAre({{"f", "(a, b) -> a"}})));
}

TEST_F(LifetimeAnnotationsTest, LifetimeAnnotation_MultipleArguments) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(R"(
        [[clang::annotate("lifetimes", "a, b -> a")]]
        int* f(int*, int*);
  )"),
              IsOkAndHolds(LifetimesAre({{"f", "a, b -> a"}})));
}

TEST_F(LifetimeAnnotationsTest, LifetimeAnnotation_NoReturn) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(R"(
        [[clang::annotate("lifetimes", "a, b")]]
        void f(int*, int*);
  )"),
              IsOkAndHolds(LifetimesAre({{"f", "a, b"}})));
}

TEST_F(LifetimeAnnotationsTest, LifetimeAnnotation_NoLifetimeParam) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(R"(
        [[clang::annotate("lifetimes", "a, (), a -> a")]]
        int* f(int*, int, int*);
  )"),
              IsOkAndHolds(LifetimesAre({{"f", "a, (), a -> a"}})));
}

TEST_F(LifetimeAnnotationsTest, LifetimeAnnotation_Method) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(R"(
        struct S {
          [[clang::annotate("lifetimes", "a: -> a")]]
          int* f();
        };
  )"),
              IsOkAndHolds(LifetimesAre({{"S::f", "a: -> a"}})));
}

TEST_F(LifetimeAnnotationsTest, LifetimeAnnotation_MethodWithParam) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(R"(
        struct S {
          [[clang::annotate("lifetimes", "a: b -> a")]]
          int* f(int*);
        };
  )"),
              IsOkAndHolds(LifetimesAre({{"S::f", "a: b -> a"}})));
}

TEST_F(LifetimeAnnotationsTest, LifetimeAnnotation_MethodWithLifetimeParams) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(R"(
        struct [[clang::annotate("lifetime_params", "x", "y")]] S {
          [[clang::annotate("lifetimes", "([x, y], a): -> x")]]
          int* f();
        };
  )"),
              IsOkAndHolds(LifetimesAre({{"S::f", "([x, y], a): -> x"}})));
}

TEST_F(LifetimeAnnotationsTest, LifetimeAnnotation_Invalid_MissingThis) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(R"(
        struct S {
          [[clang::annotate("lifetimes", "-> a")]]
          int* f();
        };
  )"),
              StatusIs(absl::StatusCode::kUnknown,
                       StartsWith("Invalid lifetime annotation")));
}

TEST_F(LifetimeAnnotationsTest, LifetimeAnnotation_Invalid_ThisOnFreeFunction) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(R"(
        [[clang::annotate("lifetimes", "a: a -> a")]]
        int* f(int*);
  )"),
              StatusIs(absl::StatusCode::kUnknown,
                       StartsWith("Invalid lifetime annotation")));
}

TEST_F(LifetimeAnnotationsTest, LifetimeAnnotation_Invalid_WrongNumber) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(R"(
        [[clang::annotate("lifetimes", "a -> a")]]
        int* f(int**);
  )"),
              StatusIs(absl::StatusCode::kUnknown,
                       StartsWith("Invalid lifetime annotation")));
}

TEST_F(LifetimeAnnotationsTest, LifetimeAnnotation_Callback) {
  EXPECT_THAT(
      GetNamedLifetimeAnnotations(R"(
        [[clang::annotate("lifetimes", "b, ((a -> a), static) -> b")]]
        int* f(int*, int* (*)(int*));
  )"),
      IsOkAndHolds(LifetimesAre({{"f", "b, ((a -> a), static) -> b"}})));
}

TEST_F(LifetimeAnnotationsTest, LifetimeAnnotation_CallbackMultipleParams) {
  EXPECT_THAT(
      GetNamedLifetimeAnnotations(R"(
        [[clang::annotate("lifetimes", "c, ((a, b -> a), static) -> c")]]
        int* f(int*, int* (*)(int*, int*));
  )"),
      IsOkAndHolds(LifetimesAre({{"f", "c, ((a, b -> a), static) -> c"}})));
}

TEST_F(LifetimeAnnotationsTest, LifetimeAnnotation_CallbackTmplFunc) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(R"(
        template <typename Func>
        struct function;
        [[clang::annotate("lifetimes", "a, ((b -> b)) -> a")]]
        int* f(int*, function<int*(int*)>);
  )"),
              IsOkAndHolds(LifetimesAre({{"f", "a, ((b -> b)) -> a"}})));
}

TEST_F(LifetimeAnnotationsTest, LifetimeAnnotation_MultipleCallbacks) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(R"(
        [[clang::annotate("lifetimes", "a, ((b -> b), static), ((c -> c), static) -> a")]]
        int* f(int*, int* (*)(int*), int* (*)(int*));
  )"),
              IsOkAndHolds(LifetimesAre(
                  {{"f", "a, ((b -> b), static), ((c -> c), static) -> a"}})));
}

TEST_F(LifetimeAnnotationsTest, LifetimeAnnotation_ReturnFunctionPtr) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(R"_(
        typedef int* (*FP)(int*);
        [[clang::annotate("lifetimes", "a -> ((b -> b), static)")]]
        FP f(int*);
  )_"),
              IsOkAndHolds(LifetimesAre({{"f", "a -> ((b -> b), static)"}})));
}

}  // namespace
}  // namespace lifetimes
}  // namespace tidy
}  // namespace clang
