// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "lifetime_annotations/lifetime_annotations.h"

#include <optional>
#include <string>
#include <utility>

#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "absl/strings/str_cat.h"
#include "absl/strings/str_format.h"
#include "absl/strings/string_view.h"
#include "common/status_test_matchers.h"
#include "lifetime_annotations/lifetime_error.h"
#include "lifetime_annotations/lifetime_symbol_table.h"
#include "lifetime_annotations/test/named_func_lifetimes.h"
#include "lifetime_annotations/test/run_on_code.h"
#include "lifetime_annotations/type_lifetimes.h"
#include "clang/AST/Decl.h"
#include "clang/ASTMatchers/ASTMatchFinder.h"
#include "clang/ASTMatchers/ASTMatchers.h"
#include "clang/Tooling/Tooling.h"
#include "llvm/ADT/StringRef.h"
#include "llvm/Support/Error.h"
#include "llvm/Support/FormatVariadic.h"
#include "llvm/Support/raw_ostream.h"

// This file contains tests both for the "legacy" lifetime annotations
// (`[[clang::annotate("lifetimes", ...)]]` placed on a function declaration)
// and the newer annotations (`[[clang::annotate_type("lifetime", ...")]]`
// placed on a type). This is because we expect we may continue to use the
// "legacy" style of annotations in sidecar files.
//
// Some tests only test one style of annotation where testing the other style
// does not make sense for the particular test.
//
// In addition, this file contains tests for lifetime elision (which can be
// considered an implicit form of lifetime annotation). Lifetime elision tests
// fail if the lifetimes do not have exactly the expected names, even if the
// lifetimes are equivalent up to a renaming. The intent of this is to catch
// cases where additional lifetimes are produced but not used.

namespace clang {
namespace tidy {
namespace lifetimes {
namespace {

using crubit::IsOkAndHolds;
using crubit::StatusIs;
using testing::StartsWith;

bool IsOverloaded(const clang::FunctionDecl* func) {
  return !func->getDeclContext()->lookup(func->getDeclName()).isSingleResult();
}

std::string QualifiedName(const clang::FunctionDecl* func) {
  std::string str;
  llvm::raw_string_ostream ostream(str);
  func->printQualifiedName(ostream);
  if (IsOverloaded(func)) {
    ostream << "[" << StripAttributes(func->getType()).getAsString() << "]";
  }
  ostream.flush();
  return str;
}

// Prepends definitions for lifetime annotation macros to the code.
std::string WithLifetimeMacros(absl::string_view code) {
  std::string result = R"(
    // TODO(mboehme): We would prefer `$(...)` to be a variadic macro that
    // stringizes each of its macro arguments individually. This is possible but
    // requires some contortions: https://stackoverflow.com/a/5958315
    #define $(l) [[clang::annotate_type("lifetime", #l)]]
    #define $2(l1, l2) [[clang::annotate_type("lifetime", #l1, #l2)]]
    #define $3(l1, l2, l3) [[clang::annotate_type("lifetime", #l1, #l2, #l3)]]
  )";
  for (char l = 'a'; l <= 'z'; ++l) {
    absl::StrAppendFormat(&result, "#define $%c $(%c)\n", l, l);
  }
  absl::StrAppend(&result, "#define $static $(static)\n");
  absl::StrAppend(&result, code);
  return result;
}

std::string FormatErrorString(llvm::Error err) {
  std::string result;
  err = llvm::handleErrors(
      std::move(err), [&result](const LifetimeError& lifetime_err) {
        switch (lifetime_err.type()) {
          case LifetimeError::Type::ElisionNotEnabled:
            result = "ERROR(ElisionNotEnabled): ";
            break;
          case LifetimeError::Type::CannotElideOutputLifetimes:
            result = "ERROR(CannotElideOutputLifetimes): ";
            break;
          case LifetimeError::Type::Other:
            result = "ERROR(Other): ";
            break;
        }
        absl::StrAppend(&result, lifetime_err.message());
      });
  if (err) {
    result = absl::StrCat("ERROR: ", llvm::toString(std::move(err)));
  }
  return result;
}

class LifetimeAnnotationsTest : public testing::Test {
 protected:
  absl::StatusOr<NamedFuncLifetimes> GetNamedLifetimeAnnotations(
      absl::string_view code,
      const clang::tooling::FileContentMappings& file_contents =
          clang::tooling::FileContentMappings(),
      bool skip_templates = true) {
    absl::StatusOr<NamedFuncLifetimes> result;
    bool success = runOnCodeWithLifetimeHandlers(
        llvm::StringRef(code.data(), code.size()),
        [&result, skip_templates](
            clang::ASTContext& ast_context,
            const LifetimeAnnotationContext& lifetime_context) {
          using clang::ast_matchers::findAll;
          using clang::ast_matchers::functionDecl;
          using clang::ast_matchers::match;

          NamedFuncLifetimes named_func_lifetimes;
          for (const auto& node :
               match(findAll(functionDecl().bind("func")), ast_context)) {
            if (const auto* func =
                    node.getNodeAs<clang::FunctionDecl>("func")) {
              // Skip various categories of function, unless explicitly
              // requested:
              // - Template instantiation don't contain any annotations that
              //   aren't present in the template itself, but they may contain
              //   reference-like types (which will obviously be unannotated),
              //   which will generate nuisance "lifetime elision not enabled"
              //   errors.
              // - Implicitly defaulted functions obviously cannot contain
              //   lifetime annotations. They will need to be handled through
              //   `AnalyzeDefaultedFunction()` in analyze.cc.
              if ((func->isTemplateInstantiation() && skip_templates) ||
                  (func->isDefaulted() && !func->isExplicitlyDefaulted())) {
                continue;
              }

              LifetimeSymbolTable symbol_table;
              llvm::Expected<FunctionLifetimes> func_lifetimes =
                  GetLifetimeAnnotations(func, lifetime_context, &symbol_table);

              std::string new_entry;
              if (func_lifetimes) {
                new_entry = NameLifetimes(*func_lifetimes, symbol_table);
              } else {
                new_entry = FormatErrorString(func_lifetimes.takeError());
              }

              std::string func_name = QualifiedName(func);
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
              IsOkAndHolds(LifetimesAre({{"f",
                                          "ERROR(ElisionNotEnabled): Lifetime "
                                          "elision not enabled for 'f'"}})));
}

TEST_F(LifetimeAnnotationsTest, Failure_NoOutputAnnotationNoLifetimeElision) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(R"(
        int* f();
  )"),
              // We specifically want to see this error message rather than
              // "Cannot elide output lifetimes".
              IsOkAndHolds(LifetimesAre({{"f",
                                          "ERROR(ElisionNotEnabled): Lifetime "
                                          "elision not enabled for 'f'"}})));
}

TEST_F(LifetimeAnnotationsTest, Failure_NoAnnotationsElisionPragmaInWrongFile) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(R"(
        #pragma clang lifetime_elision
        #include "header.h"
  )",
                                          {std::make_pair("header.h", R"(
        int** f(int*);
  )")}),
              IsOkAndHolds(LifetimesAre({{"f",
                                          "ERROR(ElisionNotEnabled): Lifetime "
                                          "elision not enabled for 'f'"}})));
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

TEST_F(LifetimeAnnotationsTest, LifetimeElision_TemplatesWithConstant) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(R"(
        #pragma clang lifetime_elision
        template <class T, bool B> class vector {};
        int* f(vector<int *, true>);
        vector<int*, false> g(int *);
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

TEST_F(LifetimeAnnotationsTest, LifetimeElision_Destructor) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(R"cc(
                // Note: this works even without #pragma clang lifetime_elision
                struct S {
                  ~S();
                };
              )cc"),
              IsOkAndHolds(LifetimesAre({{"S::~S", "a:"}})));
}

TEST_F(LifetimeAnnotationsTest, LifetimeElision_ExplicitlyDefaultedCtor) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(R"(
          #pragma clang lifetime_elision
          struct S {
            S() = default;
          };)"),
              IsOkAndHolds(LifetimesAre({{"S::S", "a:"}})));
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

TEST_F(LifetimeAnnotationsTest,
       LifetimeElision_LifetimesInsideFunctionPointerParameter) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(R"(
        #pragma clang lifetime_elision
        void f(void (*)(int *, int *));
  )"),
              IsOkAndHolds(LifetimesAre({{"f", "(a, b)"}})));
}

TEST_F(
    LifetimeAnnotationsTest,
    LifetimeElision_FunctionPointerParameterDoesNotCountTowardsInputLifetimes) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(R"(
        #pragma clang lifetime_elision
        int* f(int *, void (*)());
  )"),
              IsOkAndHolds(LifetimesAre({{"f", "a, () -> a"}})));
}

TEST_F(
    LifetimeAnnotationsTest,
    LifetimeElision_FunctionReferenceParameterDoesNotCountTowardsInputLifetimes) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(R"(
        #pragma clang lifetime_elision
        int* f(int *, void (&)());
  )"),
              IsOkAndHolds(LifetimesAre({{"f", "a, () -> a"}})));
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

TEST_F(LifetimeAnnotationsTest, LifetimeElision_PointerAlias) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(R"_(
    #pragma clang lifetime_elision
    using Alias = int *;
    void f(Alias* alias);
  )_"),
              // TODO(b/290574080): Do we want elision to "see into" the type
              // alias? See bug for details.
              IsOkAndHolds(LifetimesAre({{"f", "(a, b)"}})));
}

TEST_F(LifetimeAnnotationsTest, LifetimeElision_AliasTemplate) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(R"(
        #pragma clang lifetime_elision
        template <class T>
        using Alias = T;
        void f(Alias<int *>);
  )"),
              // TODO(b/357835254): Should be "a" rather than "b".
              // The issue is that `ValueLifetimes::Create()` creates a lifetime
              // for the `int *` twice: Once for its occurrence as the template
              // argument (adding it to `template_argument_lifetimes_`), and
              // once for its occurrence as the type alias's canonical type
              // (adding it to `pointee_lifetimes_`). This violates the
              // `ValueLifetimes` invariant that only one of
              // `template_argument_lifetimes_` or `pointee_lifetimes_` should
              // be populated.
              // We still end up with only one lifetime in the result because
              // `ValueLifetimes::DebugString()` bails out after the
              // `!PointeeType(Type()).isNull()` case and therefore ignores the
              // lifetimes in `template_argument_lifetimes_`. Because the
              // lifetime in `pointee_lifetimes_` is the second one to be
              // produced, we end up with "b" rather than "a".
              IsOkAndHolds(LifetimesAre({{"f", "b"}})));
}

TEST_F(LifetimeAnnotationsTest, LifetimeElision_AliasTemplate2) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(R"(
        #pragma clang lifetime_elision
        template <class T>
        using PtrToT = T*;
        void f(PtrToT<int *>);
  )"),
              // TODO(b/357835254): Should be "(a, b)" rather than "(b, c)".
              // TODO(b/290574080): Also, there's a question of whether we
              // want elision to "see into" the type alias? If not, this should
              // simply be "a" rather than "(a, b)".See bug for details.
              IsOkAndHolds(LifetimesAre({{"f", "(b, c)"}})));
}

TEST_F(LifetimeAnnotationsTest, LifetimeElision_NullabilityAliasTemplate) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(R"(
        #pragma clang lifetime_elision
        template <class T>
        using Nullable = T _Nullable;
        void f(Nullable<int *>);
  )"),
              // TODO(b/357835254): Should be "a" rather than "b".
              IsOkAndHolds(LifetimesAre({{"f", "b"}})));
}

TEST_F(LifetimeAnnotationsTest, LifetimeElision_FunctionAlias) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(R"_(
    #pragma clang lifetime_elision
    using Alias = void(int*);
    void f(Alias* alias);
  )_"),
              // TODO(b/290574080): Do we want elision to "see into" the type
              // alias? See bug for details.
              IsOkAndHolds(LifetimesAre({{"f", "(a)"}})));
}

TEST_F(LifetimeAnnotationsTest, LifetimeElision_FailureTooFewInputLifetimes) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(R"(
        #pragma clang lifetime_elision
        int* f();
  )"),
              IsOkAndHolds(LifetimesAre(
                  {{"f",
                    "ERROR(CannotElideOutputLifetimes): Cannot elide output "
                    "lifetimes for 'f' because it is a non-member function "
                    "that does not have exactly one input lifetime"}})));
}

TEST_F(LifetimeAnnotationsTest, LifetimeElision_FailureTooManyInputLifetimes) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(R"(
        #pragma clang lifetime_elision
        int* f(int**);
  )"),
              IsOkAndHolds(LifetimesAre(
                  {{"f",
                    "ERROR(CannotElideOutputLifetimes): Cannot elide output "
                    "lifetimes for 'f' because it is a non-member function "
                    "that does not have exactly one input lifetime"}})));
}

TEST_F(LifetimeAnnotationsTest, LifetimeAnnotation_NoLifetimes) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(R"_(
        [[clang::annotate("lifetimes", "()")]]
        void f(int);
  )_"),
              IsOkAndHolds(LifetimesAre({{"f", "()"}})));
}

TEST_F(LifetimeAnnotationsTest,
       LifetimeAnnotation_FunctionPointerHasNoLifetimes) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(R"_(
        void f(void (*)());
  )_"),
              IsOkAndHolds(LifetimesAre({{"f", "()"}})));
}

TEST_F(LifetimeAnnotationsTest,
       LifetimeAnnotation_FunctionPointerReturnTypeHasNoLifetimes) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(R"_(
        int (* f())(float, double);
  )_"),
              IsOkAndHolds(LifetimesAre({{"f", "-> ((), ())"}})));
}

TEST_F(LifetimeAnnotationsTest,
       LifetimeAnnotation_FunctionReferenceHasNoLifetimes) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(R"_(
        void f(void (&)());
  )_"),
              IsOkAndHolds(LifetimesAre({{"f", "()"}})));
}

TEST_F(LifetimeAnnotationsTest, LifetimeAnnotation_BadAttributeArgument) {
  EXPECT_THAT(
      GetNamedLifetimeAnnotations(WithLifetimeMacros(R"(
        void f(int* [[clang::annotate_type("lifetime", 1)]]);
  )")),
      IsOkAndHolds(LifetimesAre(
          {{"f",
            "ERROR(Other): cannot evaluate argument as a string literal"}})));
}

TEST_F(LifetimeAnnotationsTest, LifetimeAnnotation_Simple) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(WithLifetimeMacros(R"(
        [[clang::annotate("lifetimes", "a -> a")]]
        int* f1(int*);
        int* $a f2(int* $a);
  )")),
              IsOkAndHolds(LifetimesAre({{"f1", "a -> a"}, {"f2", "a -> a"}})));
}

TEST_F(LifetimeAnnotationsTest, LifetimeAnnotation_SimpleRef) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(WithLifetimeMacros(R"(
        [[clang::annotate("lifetimes", "a -> a")]]
        int& f1(int&);
        int& $a f2(int& $a);
  )")),
              IsOkAndHolds(LifetimesAre({{"f1", "a -> a"}, {"f2", "a -> a"}})));
}

TEST_F(LifetimeAnnotationsTest,
       LifetimeAnnotation_Invalid_LifetimeOnNonReferenceLikeType) {
  EXPECT_THAT(
      GetNamedLifetimeAnnotations(WithLifetimeMacros(R"(
        void f(int $a);
  )")),
      IsOkAndHolds(LifetimesAre(
          {{"f", "ERROR(Other): Type may not be annotated with lifetimes"}})));
}

TEST_F(LifetimeAnnotationsTest,
       LifetimeAnnotation_Invalid_LifetimeOnFunctionPointer) {
  EXPECT_THAT(
      GetNamedLifetimeAnnotations(WithLifetimeMacros(R"(
        void f(void (* $a)());
  )")),
      IsOkAndHolds(LifetimesAre(
          {{"f", "ERROR(Other): Type may not be annotated with lifetimes"}})));
}

TEST_F(LifetimeAnnotationsTest,
       LifetimeAnnotation_Invalid_LifetimeOnFunctionPointerReturnType) {
  EXPECT_THAT(
      GetNamedLifetimeAnnotations(WithLifetimeMacros(R"(
        int (* $a f())(float, double);
  )")),
      IsOkAndHolds(LifetimesAre(
          {{"f", "ERROR(Other): Type may not be annotated with lifetimes"}})));
}

TEST_F(LifetimeAnnotationsTest,
       LifetimeAnnotation_Invalid_LifetimeOnFunctionReference) {
  EXPECT_THAT(
      GetNamedLifetimeAnnotations(WithLifetimeMacros(R"(
        void f(void (& $a)());
  )")),
      IsOkAndHolds(LifetimesAre(
          {{"f", "ERROR(Other): Type may not be annotated with lifetimes"}})));
}

TEST_F(LifetimeAnnotationsTest,
       LifetimeAnnotation_Invalid_MultipleLifetimesOnPointer) {
  EXPECT_THAT(
      GetNamedLifetimeAnnotations(WithLifetimeMacros(R"(
        void f(int* $2(a, b));
  )")),
      IsOkAndHolds(LifetimesAre(
          {{"f",
            "ERROR(Other): Expected a single lifetime but 2 were given"}})));
}

TEST_F(LifetimeAnnotationsTest, LifetimeAnnotation_Static) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(WithLifetimeMacros(R"(
        [[clang::annotate("lifetimes", "static -> static")]]
        int* f1(int*);
        int* $static f2(int* $static);
  )")),
              IsOkAndHolds(LifetimesAre(
                  {{"f1", "static -> static"}, {"f2", "static -> static"}})));
}

TEST_F(LifetimeAnnotationsTest, LifetimeAnnotation_PartialElision) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(WithLifetimeMacros(R"(
        #pragma clang lifetime_elision
        int* $a f(int* $a, int*, int* $a);
  )")),
              IsOkAndHolds(LifetimesAre({{"f", "a, b, a -> a"}})));
}

TEST_F(LifetimeAnnotationsTest, LifetimeAnnotation_MultiplePtr) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(WithLifetimeMacros(R"(
        [[clang::annotate("lifetimes", "(a, b) -> a")]]
        int* f1(int**);
        int* $a f2(int* $a * $b);
  )")),
              IsOkAndHolds(LifetimesAre(
                  {{"f1", "(a, b) -> a"}, {"f2", "(a, b) -> a"}})));
}

TEST_F(LifetimeAnnotationsTest, LifetimeAnnotation_MultipleArguments) {
  EXPECT_THAT(
      GetNamedLifetimeAnnotations(WithLifetimeMacros(R"(
        [[clang::annotate("lifetimes", "a, b -> a")]]
        int* f1(int*, int*);
        int* $a f2(int* $a, int* $b);
  )")),
      IsOkAndHolds(LifetimesAre({{"f1", "a, b -> a"}, {"f2", "a, b -> a"}})));
}

TEST_F(LifetimeAnnotationsTest, LifetimeAnnotation_NoReturn) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(WithLifetimeMacros(R"(
        [[clang::annotate("lifetimes", "a, b")]]
        void f1(int*, int*);
        void f2(int* $a, int* $b);
  )")),
              IsOkAndHolds(LifetimesAre({{"f1", "a, b"}, {"f2", "a, b"}})));
}

TEST_F(LifetimeAnnotationsTest, LifetimeAnnotation_ParamWithoutLifetime) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(WithLifetimeMacros(R"(
        [[clang::annotate("lifetimes", "a, (), a -> a")]]
        int* f1(int*, int, int*);
        int* $a f2(int* $a, int, int* $a);
  )")),
              IsOkAndHolds(LifetimesAre(
                  {{"f1", "a, (), a -> a"}, {"f2", "a, (), a -> a"}})));
}

TEST_F(LifetimeAnnotationsTest, LifetimeAnnotation_LifetimeParameterizedType) {
  // Use a custom delimiter so that the `")` in the `clang::annotate` attribute
  // below doesn't prematurely terminate the string.
  EXPECT_THAT(GetNamedLifetimeAnnotations(WithLifetimeMacros(R"code(
    struct [[clang::annotate("lifetime_params", "a", "b")]] S_param {};

    [[clang::annotate("lifetimes", "([a, b]) -> ([a, b])")]]
    S_param f1(S_param s);

    S_param $2(a, b) f2(S_param $2(a, b) s);
  )code")),
              IsOkAndHolds(LifetimesAre({{"f1", "([a, b]) -> ([a, b])"},
                                         {"f2", "([a, b]) -> ([a, b])"}})));
}

TEST_F(
    LifetimeAnnotationsTest,
    LifetimeAnnotation_LifetimeParameterizedType_Invalid_WrongNumberOfLifetimes) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(WithLifetimeMacros(R"(
    struct [[clang::annotate("lifetime_params", "a", "b")]] S_param {};

    void f(S_param $3(a, b, c) s);
  )")),
              IsOkAndHolds(LifetimesAre(
                  {{"f",
                    "ERROR(Other): Type has 2 lifetime parameters but 3 "
                    "lifetime arguments were given"}})));
}

TEST_F(
    LifetimeAnnotationsTest,
    LifetimeAnnotation_LifetimeParameterizedType_Invalid_MultipleAnnotateAttributes) {
  EXPECT_THAT(
      GetNamedLifetimeAnnotations(WithLifetimeMacros(R"(
    struct [[clang::annotate("lifetime_params", "a", "b")]] S_param {};

    void f(S_param $a $b s);
  )")),
      IsOkAndHolds(LifetimesAre(
          {{"f",
            "ERROR(Other): Only one `[[annotate_type(\"lifetime\", ...)]]` "
            "attribute may be placed on a type"}})));
}

TEST_F(LifetimeAnnotationsTest, LifetimeAnnotation_Template) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(WithLifetimeMacros(R"(
    template <class T> class vector {};

    [[clang::annotate("lifetimes", "(a, b) -> a")]]
    int* f1(const vector<int *> &);
    int* $a f2(const vector<int * $a> & $b);
  )")),
              IsOkAndHolds(LifetimesAre(
                  {{"f1", "(a, b) -> a"}, {"f2", "(a, b) -> a"}})));
}

TEST_F(LifetimeAnnotationsTest, LifetimeAnnotationTemplateUniversalReference) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(WithLifetimeMacros(R"(
    #pragma clang lifetime_elision
    template <typename T>
    struct S {
      static void f(T&&) {}
    };

    void g() { int a; S<int&>::f(a); }
  )"),
                                          clang::tooling::FileContentMappings(),
                                          /*skip_templates=*/false),
              IsOkAndHolds(LifetimesContain({{"S<int &>::f", "a"}})));
}

TEST_F(LifetimeAnnotationsTest, LifetimeAnnotation_VariadicTemplate) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(WithLifetimeMacros(R"code(
    template <class... T> class variadic{};

    [[clang::annotate("lifetimes", "(<a, b>, c)")]]
    void f1(const variadic<int *, int *> &);
    void f2(const variadic<int * $a, int * $b> & $c);
  )code")),
              IsOkAndHolds(LifetimesAre(
                  {{"f1", "(<a, b>, c)"}, {"f2", "(<a, b>, c)"}})));
}

TEST_F(LifetimeAnnotationsTest, LifetimeAnnotation_VariadicTemplateWithCtor) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(WithLifetimeMacros(R"code(
    template <typename... Args> struct S { S() $a {} };
    template <typename T, typename... Args>
    struct S<T, Args...> {
      S(T t, Args... args) $a {}
    };

    void target(int* $a a, int* $b b) {
      S<int*, int*> s = {a, b};
    }
  )code")),
              IsOkAndHolds(LifetimesAre({{"S::S<Args...>", "a:"},
                                         {"S<type-parameter-0-0, "
                                          "type-parameter-0-1...>::"
                                          "S<type-parameter-0-0, "
                                          "type-parameter-0-1...>",
                                          "a: (), ()"},
                                         {"target", "a, b"}})));
}

TEST_F(LifetimeAnnotationsTest,
       LifetimeAnnotation_AliasTemplateForClassTemplate) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(WithLifetimeMacros(R"(
    template<class T> struct trait {
      using type = T;
    };
    template<class T> using alias_template = typename trait<T>::type;
    void target(alias_template<int>* $a p) {
    }
  )")),
              IsOkAndHolds(LifetimesAre({{"target", "a"}})));
}

TEST_F(LifetimeAnnotationsTest, LifetimeAnnotation_Method) {
  EXPECT_THAT(
      GetNamedLifetimeAnnotations(WithLifetimeMacros(R"(
        struct S {
          [[clang::annotate("lifetimes", "a: -> a")]]
          int* f1();
          int* $a f2() $a;
        };
      )")),
      IsOkAndHolds(LifetimesAre({{"S::f1", "a: -> a"}, {"S::f2", "a: -> a"}})));
}

TEST_F(LifetimeAnnotationsTest, LifetimeAnnotation_ConstMethod) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(WithLifetimeMacros(R"(
        struct S {
          int* $a f2() const $a;
        };
      )")),
              IsOkAndHolds(LifetimesAre({{"S::f2", "a: -> a"}})));
}

TEST_F(LifetimeAnnotationsTest, LifetimeAnnotation_MethodWithParam) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(WithLifetimeMacros(R"(
        struct S {
          [[clang::annotate("lifetimes", "a: b -> a")]]
          int* f1(int*);
          int* $a f2(int* $b) $a;
        };
  )")),
              IsOkAndHolds(LifetimesAre(
                  {{"S::f1", "a: b -> a"}, {"S::f2", "a: b -> a"}})));
}

TEST_F(LifetimeAnnotationsTest, LifetimeAnnotation_MethodWithLifetimeParams) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(WithLifetimeMacros(R"(
        struct [[clang::annotate("lifetime_params", "x", "y")]] S {
          [[clang::annotate("lifetimes", "([x, y], a): -> x")]]
          int* f1();
          // It's implied that the lifetime parameters of `this` are $x and $y
          // because this is a member function on struct with those lifetime
          // parameters.
          // TODO(mboehme): This doesn't work yet. We need some special handling
          // to know that in this context, the type `S` doesn't need lifetimes
          // put on it.
          // TODO(mboehme): How do we resolve this difference relative to the
          // "legacy" lifetime annotations? Does this mean that they should also
          // not include the lifetimes x and y?
          // int* $x f2() $a;
        };
  )")),
              IsOkAndHolds(LifetimesAre({{"S::f1", "([x, y], a): -> x"}})));
}

TEST_F(LifetimeAnnotationsTest, LifetimeAnnotation_Invalid_MissingThis) {
  EXPECT_THAT(
      GetNamedLifetimeAnnotations(WithLifetimeMacros(R"(
        struct S {
          [[clang::annotate("lifetimes", "-> a")]]
          int* f();
        };
  )")),
      IsOkAndHolds(LifetimesAre(
          {{"S::f",
            "ERROR(Other): Invalid lifetime annotation: too few lifetimes"}})));
  EXPECT_THAT(GetNamedLifetimeAnnotations(WithLifetimeMacros(R"(
        struct S {
          int* $a f();
        };
  )")),
              IsOkAndHolds(LifetimesAre({{"S::f",
                                          "ERROR(ElisionNotEnabled): Lifetime "
                                          "elision not enabled for 'f'"}})));
}

TEST_F(LifetimeAnnotationsTest, LifetimeAnnotation_Invalid_ThisOnFreeFunction) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(WithLifetimeMacros(R"(
        [[clang::annotate("lifetimes", "a: a -> a")]]
        int* f(int*);
  )")),
              IsOkAndHolds(LifetimesAre({{"f",
                                          "ERROR(Other): Invalid lifetime "
                                          "annotation: too many lifetimes"}})));
  EXPECT_THAT(GetNamedLifetimeAnnotations(WithLifetimeMacros(R"(
        int* $a f(int* $a) $a;
  )")),
              IsOkAndHolds(LifetimesAre(
                  {{"f",
                    "ERROR(Other): Encountered a `this` lifetime on a "
                    "function with no `this` parameter"}})));
}

TEST_F(LifetimeAnnotationsTest, LifetimeAnnotation_Invalid_WrongNumber) {
  EXPECT_THAT(
      GetNamedLifetimeAnnotations(R"(
        [[clang::annotate("lifetimes", "a -> a")]]
        int* f(int**);
  )"),
      IsOkAndHolds(LifetimesAre(
          {{"f",
            "ERROR(Other): Invalid lifetime annotation: too few lifetimes"}})));
}

TEST_F(LifetimeAnnotationsTest, LifetimeAnnotation_Callback) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(WithLifetimeMacros(R"(
        [[clang::annotate("lifetimes", "b, (a -> a) -> b")]]
        int* f1(int*, int* (*)(int*));
        int* $b f2(int* $b, int* $a (*)(int* $a));
  )")),
              IsOkAndHolds(LifetimesAre(
                  {{"f1", "b, (a -> a) -> b"}, {"f2", "b, (a -> a) -> b"}})));
}

TEST_F(LifetimeAnnotationsTest, LifetimeAnnotation_CallbackMultipleParams) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(WithLifetimeMacros(R"(
        [[clang::annotate("lifetimes", "c, ((a, b -> a)) -> c")]]
        int* f1(int*, int* (*)(int*, int*));
        int* $c f2(int* $c, int* $a (*)(int* $a, int* $b));
  )")),
              IsOkAndHolds(LifetimesAre({{"f1", "c, (a, b -> a) -> c"},
                                         {"f2", "c, (a, b -> a) -> c"}})));
}

TEST_F(LifetimeAnnotationsTest, LifetimeAnnotation_CallbackTmplFunc) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(WithLifetimeMacros(R"(
        template <typename Func>
        struct function;
        [[clang::annotate("lifetimes", "a, (b -> b) -> a")]]
        int* f1(int*, function<int*(int*)>);
        int* $a f2(int* $a, function<int* $b(int* $b)>);
  )")),
              IsOkAndHolds(LifetimesAre(
                  {{"f1", "a, (b -> b) -> a"}, {"f2", "a, (b -> b) -> a"}})));
}

TEST_F(LifetimeAnnotationsTest, LifetimeAnnotation_MultipleCallbacks) {
  EXPECT_THAT(
      GetNamedLifetimeAnnotations(WithLifetimeMacros(R"(
        [[clang::annotate("lifetimes", "a, (b -> b), (c -> c) -> a")]]
        int* f1(int*, int* (*)(int*), int* (*)(int*));
        int* $a f2(int* $a, int* $b (*)(int* $b), int* $c (*)(int* $c));
  )")),
      IsOkAndHolds(LifetimesAre({{"f1", "a, (b -> b), (c -> c) -> a"},
                                 {"f2", "a, (b -> b), (c -> c) -> a"}})));
}

TEST_F(LifetimeAnnotationsTest, LifetimeAnnotation_ReturnFunctionPtr) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(WithLifetimeMacros(R"_(
        typedef int* (*FP)(int*);
        [[clang::annotate("lifetimes", "a -> (b -> b)")]]
        FP f(int*);
        // TODO(mboehme): Need to support lifetime parameters on type aliases to
        // be able to express this in the new syntax.
  )_")),
              IsOkAndHolds(LifetimesAre({{"f", "a -> (b -> b)"}})));
}

}  // namespace
}  // namespace lifetimes
}  // namespace tidy
}  // namespace clang
