// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/inference/collect_evidence.h"

#include <string>
#include <utility>
#include <vector>

#include "nullability/inference/inference.proto.h"
#include "nullability/inference/slot_fingerprint.h"
#include "nullability/test/test_headers.h"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclBase.h"
#include "clang/ASTMatchers/ASTMatchFinder.h"
#include "clang/ASTMatchers/ASTMatchers.h"
#include "clang/Basic/LLVM.h"
#include "clang/Testing/TestAST.h"
#include "third_party/llvm/llvm-project/clang/unittests/Analysis/FlowSensitive/TestingSupport.h"
#include "llvm/ADT/DenseMap.h"
#include "llvm/ADT/DenseSet.h"
#include "llvm/ADT/StringRef.h"
#include "llvm/Support/Error.h"
#include "llvm/Support/raw_ostream.h"
#include "third_party/llvm/llvm-project/third-party/unittest/googlemock/include/gmock/gmock.h"  // IWYU pragma: keep
#include "third_party/llvm/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang::tidy::nullability {
namespace {
using ::clang::ast_matchers::functionDecl;
using ::clang::ast_matchers::hasName;
using ::clang::ast_matchers::isTemplateInstantiation;
using ::clang::ast_matchers::match;
using ::clang::ast_matchers::parameterCountIs;
using ::clang::ast_matchers::selectFirst;
using ::testing::_;
using ::testing::AllOf;
using ::testing::Contains;
using ::testing::ElementsAre;
using ::testing::IsEmpty;
using ::testing::IsSupersetOf;
using ::testing::Not;
using ::testing::ResultOf;
using ::testing::SizeIs;
using ::testing::UnorderedElementsAre;

MATCHER_P3(isEvidenceMatcher, SlotMatcher, KindMatcher, SymbolMatcher, "") {
  return SlotMatcher.Matches(static_cast<Slot>(arg.slot())) &&
         KindMatcher.Matches(arg.kind()) && SymbolMatcher.Matches(arg.symbol());
}

MATCHER_P(functionNamed, Name, "") {
  return llvm::StringRef(arg.usr()).contains(
      ("@" + llvm::StringRef(Name) + "#").str());
}

testing::Matcher<const Evidence&> evidence(
    testing::Matcher<Slot> S, testing::Matcher<Evidence::Kind> Kind,
    testing::Matcher<const Symbol&> SymbolMatcher = functionNamed("target")) {
  return isEvidenceMatcher(S, Kind, SymbolMatcher);
}

clang::TestInputs getInputsWithAnnotationDefinitions(llvm::StringRef Source) {
  clang::TestInputs Inputs = Source;
  for (const auto& Entry :
       llvm::ArrayRef(test_headers_create(), test_headers_size()))
    Inputs.ExtraFiles.try_emplace(Entry.name, Entry.data);
  Inputs.ExtraArgs.push_back("-include");
  Inputs.ExtraArgs.push_back("nullability_annotations.h");
  Inputs.ExtraArgs.push_back("-I.");
  return Inputs;
}

std::vector<Evidence> collectEvidenceFromTargetFunction(
    llvm::StringRef Source, PreviousInferences PreviousInferences = {}) {
  std::vector<Evidence> Results;
  clang::TestAST AST(getInputsWithAnnotationDefinitions(Source));
  USRCache UsrCache;
  auto Err = collectEvidenceFromImplementation(
      cast<FunctionDecl>(
          *dataflow::test::findValueDecl(AST.context(), "target")),
      evidenceEmitter([&](const Evidence& E) { Results.push_back(E); },
                      UsrCache),
      UsrCache, PreviousInferences);
  if (Err) ADD_FAILURE() << toString(std::move(Err));
  return Results;
}

std::vector<Evidence> collectEvidenceFromTargetDecl(llvm::StringRef Source) {
  std::vector<Evidence> Results;
  clang::TestAST AST(getInputsWithAnnotationDefinitions(Source));
  USRCache USRCache;
  collectEvidenceFromTargetDeclaration(
      *dataflow::test::findValueDecl(AST.context(), "target"),
      evidenceEmitter([&](const Evidence& E) { Results.push_back(E); },
                      USRCache));
  return Results;
}

TEST(CollectEvidenceFromImplementationTest, NoParams) {
  static constexpr llvm::StringRef Src = R"cc(
    void target() {}
  )cc";
  EXPECT_THAT(collectEvidenceFromTargetFunction(Src), IsEmpty());
}

TEST(CollectEvidenceFromImplementationTest, OneParamUnused) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int *p0) {}
  )cc";
  EXPECT_THAT(collectEvidenceFromTargetFunction(Src), IsEmpty());
}

TEST(CollectEvidenceFromImplementationTest, OneParamUsedWithoutRestriction) {
  static constexpr llvm::StringRef Src = R"cc(
    void takesUnknown(int *unknown) {}

    void target(int *p0) { takesUnknown(p0); }
  )cc";
  EXPECT_THAT(collectEvidenceFromTargetFunction(Src),
              Not(Contains(evidence(_, _, functionNamed("target")))));
}

TEST(CollectEvidenceFromImplementationTest, Deref) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int *p0, int *p1) {
      int a = *p0;
      if (p1 != nullptr) {
        int b = *p1;
      }
    }
  )cc";
  EXPECT_THAT(collectEvidenceFromTargetFunction(Src),
              UnorderedElementsAre(
                  evidence(paramSlot(0), Evidence::UNCHECKED_DEREFERENCE)));
}

TEST(CollectEvidenceFromImplementationTest, DerefArrow) {
  static constexpr llvm::StringRef Src = R"cc(
    struct S {
      int x;
      int y();
    };
    void target(S *a, S *b) {
      a->x;
      b->y();
    }
  )cc";
  EXPECT_THAT(collectEvidenceFromTargetFunction(Src),
              UnorderedElementsAre(
                  evidence(paramSlot(0), Evidence::UNCHECKED_DEREFERENCE),
                  evidence(paramSlot(1), Evidence::UNCHECKED_DEREFERENCE)));
}

TEST(CollectEvidenceFromImplementationTest, DerefOfNonnull) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(Nonnull<int *> p) {
      *p;
    }
  )cc";
  EXPECT_THAT(collectEvidenceFromTargetFunction(Src), IsEmpty());
}

TEST(CollectEvidenceFromImplementationTest, Location) {
  llvm::StringRef Code = "void target(int *p) { *p; }";
  //                      12345678901234567890123456
  //                      0        1         2

  auto Evidence = collectEvidenceFromTargetFunction(Code);
  ASSERT_THAT(Evidence, ElementsAre(evidence(paramSlot(0),
                                             Evidence::UNCHECKED_DEREFERENCE)));
  EXPECT_EQ("input.mm:1:23", Evidence.front().location());
}

TEST(CollectEvidenceFromImplementationTest, DereferenceBeforeAssignment) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int *p) {
      *p;
      int i = 1;
      p = &i;
    }
  )cc";
  EXPECT_THAT(collectEvidenceFromTargetFunction(Src),
              UnorderedElementsAre(
                  evidence(paramSlot(0), Evidence::UNCHECKED_DEREFERENCE)));
}

TEST(CollectEvidenceFromImplementationTest, DereferenceAfterAssignment) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int *p) {
      int i = 1;
      p = &i;
      *p;
    }
  )cc";
  EXPECT_THAT(collectEvidenceFromTargetFunction(Src), IsEmpty());
}

TEST(CollectEvidenceFromImplementationTest,
     DereferenceAfterAssignmentFromReturn) {
  static constexpr llvm::StringRef Src = R"cc(
    int& getIntRef();
    int* getIntPtr();
    void target(int* p) {
      p = &getIntRef();
      *p;
      p = getIntPtr();
      *p;
    }
  )cc";
  EXPECT_THAT(collectEvidenceFromTargetFunction(Src),
              Not(Contains(evidence(_, _, functionNamed("target")))));
}

TEST(CollectEvidenceFromImplementationTest, DerefOfPtrRef) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int *&p0, int *&p1) {
      int a = *p0;
      if (p1 != nullptr) {
        int b = *p1;
      }
    }
  )cc";
  EXPECT_THAT(collectEvidenceFromTargetFunction(Src),
              UnorderedElementsAre(
                  evidence(paramSlot(0), Evidence::UNCHECKED_DEREFERENCE)));
}

TEST(CollectEvidenceFromImplementationTest, UnrelatedCondition) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int *p0, int *p1, int *p2, bool b) {
      if (b) {
        int a = *p0;
        int b = *p1;
      } else {
        int a = *p0;
        int c = *p2;
      }
    }
  )cc";
  EXPECT_THAT(collectEvidenceFromTargetFunction(Src),
              UnorderedElementsAre(
                  evidence(paramSlot(0), Evidence::UNCHECKED_DEREFERENCE),
                  evidence(paramSlot(1), Evidence::UNCHECKED_DEREFERENCE),
                  // We collect two Evidence values for two dereferences of p0
                  evidence(paramSlot(0), Evidence::UNCHECKED_DEREFERENCE),
                  evidence(paramSlot(2), Evidence::UNCHECKED_DEREFERENCE)));
}

TEST(CollectEvidenceFromImplementationTest, LaterDeref) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int *p0) {
      if (p0 == nullptr) {
        (void)0;
      } else {
        (void)0;
      }
      int a = *p0;
    }
  )cc";
  EXPECT_THAT(collectEvidenceFromTargetFunction(Src),
              UnorderedElementsAre(
                  evidence(paramSlot(0), Evidence::UNCHECKED_DEREFERENCE)));
}

TEST(CollectEvidenceFromImplementationTest, DerefBeforeGuardedDeref) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int *p0) {
      int a = *p0;
      if (p0 != nullptr) {
        int b = *p0;
      }
    }
  )cc";
  EXPECT_THAT(collectEvidenceFromTargetFunction(Src),
              UnorderedElementsAre(
                  evidence(paramSlot(0), Evidence::UNCHECKED_DEREFERENCE)));
}

TEST(CollectEvidenceFromImplementationTest, DerefAndOrCheckOfCopiedPtr) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int* p, int* q) {
      int* a = p;
      *a;
      int* b = q;
      if (q) {
        *b;
      }
      if (b) {
        *q;
      }
    }
  )cc";
  EXPECT_THAT(collectEvidenceFromTargetFunction(Src),
              UnorderedElementsAre(
                  evidence(paramSlot(0), Evidence::UNCHECKED_DEREFERENCE)));
}

TEST(CollectEvidenceFromImplementationTest, FirstSufficientSlotOnly) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int* p, int* q) {
      // Marking either of p or q Nonnull is sufficient to avoid dereferencing
      // without a check. We choose to record evidence only for the first
      // sufficient slot which can be Nonnull without the dereference becoming
      // dead code.
      int* a;
      if (p) {
        a = p;
      } else {
        a = q;
      }
      *a;
    }
  )cc";
  EXPECT_THAT(collectEvidenceFromTargetFunction(Src),
              UnorderedElementsAre(
                  evidence(paramSlot(0), Evidence::UNCHECKED_DEREFERENCE)));
}

TEST(CollectEvidenceFromImplementationTest,
     FirstSufficientSlotNotContradictingFlowConditions) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int* p, int* q) {
      // Marking p Nonnull would make the dereference dead, so we collect
      // evidence for q being Nonnull instead, since it is also sufficient.
      if (!p) {
        *q;
      }
    }
  )cc";
  EXPECT_THAT(collectEvidenceFromTargetFunction(Src),
              UnorderedElementsAre(
                  evidence(paramSlot(1), Evidence::UNCHECKED_DEREFERENCE)));
}

TEST(CollectEvidenceFromImplementationTest, EarlyReturn) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int *p0) {
      if (!p0) {
        return;
      }
      int a = *p0;
    }
  )cc";
  EXPECT_THAT(collectEvidenceFromTargetFunction(Src), IsEmpty());
}

TEST(CollectEvidenceFromImplementationTest, UnreachableCode) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int *p0, int *p1, int *p2, int *p3) {
      if (true) {
        int a = *p0;
      } else {
        int a = *p1;
      }

      if (false) {
        int a = *p2;
      }

      return;
      int a = *p3;
    }
  )cc";
  EXPECT_THAT(collectEvidenceFromTargetFunction(Src),
              UnorderedElementsAre(
                  evidence(paramSlot(0), Evidence::UNCHECKED_DEREFERENCE)));
}

TEST(CollectEvidenceFromImplementationTest, PointerToMemberField) {
  static constexpr llvm::StringRef Src = R"cc(
    struct S {};

    void target(int S::*p) {
      S s;
      s.*p;

      S s2;
      (&s2)->*p;
    }
  )cc";
  // Pointers to members are not supported pointer types, so no evidence is
  // collected. If they become a supported pointer type, this test should start
  // failing.
  EXPECT_THAT(collectEvidenceFromTargetFunction(Src), IsEmpty());
}

TEST(CollectEvidenceFromImplementationTest, PointerToMemberMethod) {
  static constexpr llvm::StringRef Src = R"cc(
    struct S {};

    void target(void (S::*p)()) {
      S s;
      (s.*p)();

      S s2;
      ((&s2)->*p)();
    }
  )cc";

  // Pointers to members are not supported pointer types, so no evidence is
  // collected. If they become a supported pointer type, this test should start
  // failing.
  EXPECT_THAT(collectEvidenceFromTargetFunction(Src), IsEmpty());
}

TEST(CollectEvidenceFromImplementationTest, NullableArgPassed) {
  static constexpr llvm::StringRef Src = R"cc(
    void callee(int *q);
    void target(Nullable<int *> p) { callee(p); }
  )cc";
  EXPECT_THAT(collectEvidenceFromTargetFunction(Src),
              Contains(evidence(paramSlot(0), Evidence::NULLABLE_ARGUMENT,
                                functionNamed("callee"))));
}

TEST(CollectEvidenceFromImplementationTest, NonnullArgPassed) {
  static constexpr llvm::StringRef Src = R"cc(
    void callee(int *q);
    void target(Nonnull<int *> p) { callee(p); }
  )cc";
  EXPECT_THAT(collectEvidenceFromTargetFunction(Src),
              Contains(evidence(paramSlot(0), Evidence::NONNULL_ARGUMENT,
                                functionNamed("callee"))));
}

TEST(CollectEvidenceFromImplementationTest, UnknownArgPassed) {
  static constexpr llvm::StringRef Src = R"cc(
    void callee(int *q);
    void target(int *p) { callee(p); }
  )cc";
  EXPECT_THAT(collectEvidenceFromTargetFunction(Src),
              Contains(evidence(paramSlot(0), Evidence::UNKNOWN_ARGUMENT,
                                functionNamed("callee"))));
}

TEST(CollectEvidenceFromImplementationTest, CheckedArgPassed) {
  static constexpr llvm::StringRef Src = R"cc(
    void callee(int *q);
    void target(int *p) {
      if (p) callee(p);
    }
  )cc";
  EXPECT_THAT(collectEvidenceFromTargetFunction(Src),
              Contains(evidence(paramSlot(0), Evidence::NONNULL_ARGUMENT,
                                functionNamed("callee"))));
}

TEST(CollectEvidenceFromImplementationTest, NullptrPassed) {
  static constexpr llvm::StringRef Src = R"cc(
    void callee(int* q);
    void target() {
      callee(nullptr);
      int* p = nullptr;
      callee(nullptr);
    }
  )cc";
  EXPECT_THAT(
      collectEvidenceFromTargetFunction(Src),
      UnorderedElementsAre(evidence(paramSlot(0), Evidence::NULLABLE_ARGUMENT,
                                    functionNamed("callee")),
                           evidence(paramSlot(0), Evidence::NULLABLE_ARGUMENT,
                                    functionNamed("callee"))));
}

TEST(CollectEvidenceFromImplementationTest, NonPtrArgPassed) {
  static constexpr llvm::StringRef Src = R"cc(
    void callee(int q);
    void target(int p) { callee(p); }
  )cc";
  EXPECT_THAT(collectEvidenceFromTargetFunction(Src), IsEmpty());
}

TEST(CollectEvidenceFromImplementationTest,
     DefaultArgumentsProduceNoEvidenceFromImplementation) {
  static constexpr llvm::StringRef Src = R"cc(
    int* getDefault();
    void hasDefaultUnannotatedFunc(int* = getDefault());
    int* q = nullptr;
    void hasDefaultUnannotatedVariable(int* = getDefault());
    int i = 1;
    void hasDefaultExpressionOfVariable(int* = &i);
    void target() {
      hasDefaultUnannotatedFunc();
      hasDefaultUnannotatedVariable();
      hasDefaultExpressionOfVariable();
    }
  )cc";
  EXPECT_THAT(collectEvidenceFromTargetFunction(Src), IsEmpty());
}

TEST(CollectEvidenceFromImplementationTest, NullableReturn) {
  static constexpr llvm::StringRef Src = R"cc(
    int* target() { return nullptr; }
  )cc";
  EXPECT_THAT(collectEvidenceFromTargetFunction(Src),
              Contains(evidence(SLOT_RETURN_TYPE, Evidence::NULLABLE_RETURN,
                                functionNamed("target"))));
}

TEST(CollectEvidenceFromImplementationTest, NonnullReturn) {
  static constexpr llvm::StringRef Src = R"cc(
    int* target(Nonnull<int*> p) {
      return p;
    }
  )cc";
  EXPECT_THAT(collectEvidenceFromTargetFunction(Src),
              Contains(evidence(SLOT_RETURN_TYPE, Evidence::NONNULL_RETURN,
                                functionNamed("target"))));
}

TEST(CollectEvidenceFromImplementationTest, UnknownReturn) {
  static constexpr llvm::StringRef Src = R"cc(
    int* target(int* p) { return p; }
  )cc";
  EXPECT_THAT(collectEvidenceFromTargetFunction(Src),
              Contains(evidence(SLOT_RETURN_TYPE, Evidence::UNKNOWN_RETURN,
                                functionNamed("target"))));
}

TEST(CollectEvidenceFromImplementationTest, MultipleReturns) {
  static constexpr llvm::StringRef Src = R"cc(
    int* target(Nonnull<int*> p, Nullable<int*> q, bool b, bool c) {
      if (b) return q;
      if (c) return nullptr;
      return p;
    }
  )cc";
  EXPECT_THAT(
      collectEvidenceFromTargetFunction(Src),
      UnorderedElementsAre(evidence(SLOT_RETURN_TYPE, Evidence::NULLABLE_RETURN,
                                    functionNamed("target")),
                           evidence(SLOT_RETURN_TYPE, Evidence::NULLABLE_RETURN,
                                    functionNamed("target")),
                           evidence(SLOT_RETURN_TYPE, Evidence::NONNULL_RETURN,
                                    functionNamed("target"))));
}

TEST(CollectEvidenceFromImplemetationTest, FunctionCallDereferenced) {
  static constexpr llvm::StringRef Src = R"cc(
    int* makePtr();
    void target() { *makePtr(); }
  )cc";
  EXPECT_THAT(
      collectEvidenceFromTargetFunction(Src),
      Contains(evidence(SLOT_RETURN_TYPE, Evidence::UNCHECKED_DEREFERENCE,
                        functionNamed("makePtr"))));
}

TEST(CollectEvidenceFromImplementationTest,
     FunctionCallResultDereferencedAfterAssignedLocally) {
  static constexpr llvm::StringRef Src = R"cc(
    int* makePtr();
    void target() {
      auto p = makePtr();
      *p;
    }
  )cc";
  EXPECT_THAT(
      collectEvidenceFromTargetFunction(Src),
      Contains(evidence(SLOT_RETURN_TYPE, Evidence::UNCHECKED_DEREFERENCE,
                        functionNamed("makePtr"))));
}

TEST(CollectEvidenceFromImplementationTest,
     FunctionCallResultDereferencedAfterAssignedLocallyAndChecked) {
  static constexpr llvm::StringRef Src = R"cc(
    int* makePtr();
    void target() {
      auto p = makePtr();
      if (p) *p;
    }
  )cc";
  EXPECT_THAT(collectEvidenceFromTargetFunction(Src), IsEmpty());
}

TEST(CollectEvidenceFromImplementationTest,
     FunctionCallResultDereferencedAfterUnrelatedConditionChecked) {
  static constexpr llvm::StringRef Src = R"cc(
    int* makePtr();
    void target(bool cond) {
      auto p = makePtr();
      if (cond) *p;
    }
  )cc";
  EXPECT_THAT(
      collectEvidenceFromTargetFunction(Src),
      Contains(evidence(SLOT_RETURN_TYPE, Evidence::UNCHECKED_DEREFERENCE,
                        functionNamed("makePtr"))));
}

TEST(CollectEvidenceFromImplementationTest, FunctionCallDereferencedWithArrow) {
  static constexpr llvm::StringRef Src = R"cc(
    struct S {
      void member();
    };

    S* makePtr();
    void target() { makePtr()->member(); }
  )cc";
  EXPECT_THAT(
      collectEvidenceFromTargetFunction(Src),
      Contains(evidence(SLOT_RETURN_TYPE, Evidence::UNCHECKED_DEREFERENCE,
                        functionNamed("makePtr"))));
}

TEST(CollectEvidenceFromImplemetationTest,
     AlreadyNonnullFunctionCallDereferenced) {
  static constexpr llvm::StringRef Src = R"cc(
    Nonnull<int*> makeNonnullPtr();
    void target() { *makeNonnullPtr(); }
  )cc";
  EXPECT_THAT(collectEvidenceFromTargetFunction(Src), IsEmpty());
}

TEST(CollectEvidenceFromImplementationTest, FunctionPointerCall) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(void (*f)()) { f(); }
  )cc";
  EXPECT_THAT(collectEvidenceFromTargetFunction(Src),
              UnorderedElementsAre(evidence(paramSlot(0),
                                            Evidence::UNCHECKED_DEREFERENCE,
                                            functionNamed("target"))));
}

TEST(CollectEvidenceFromImplemetationTest,
     ConstAccessorDereferencedAfterCheck) {
  static constexpr llvm::StringRef Src = R"cc(
    struct S {
      int* accessor() const { return i; }
      int* i = nullptr;
    };
    void target() {
      S s;
      if (s.accessor() != nullptr) {
        *s.accessor();
      }
    }
  )cc";
  EXPECT_THAT(collectEvidenceFromTargetFunction(Src), IsEmpty());
}

// Special modeling of accessors is not implemented for accessors returning
// references.
TEST(CollectEvidenceFromImplementationTest,
     ReferenceConstAccessorDereferencedAfterCheck) {
  static constexpr llvm::StringRef Src = R"cc(
    struct S {
      int* const& accessor() const { return i; }
      int* i = nullptr;
    };
    void target() {
      S s;
      if (s.accessor() != nullptr) {
        *s.accessor();
      }
    }
  )cc";
  EXPECT_THAT(collectEvidenceFromTargetFunction(Src),
              UnorderedElementsAre(evidence(SLOT_RETURN_TYPE,
                                            Evidence::UNCHECKED_DEREFERENCE,
                                            functionNamed("accessor"))));
}

TEST(CollectEvidenceFromImplementationTest,
     ConstAccessorOnTwoDifferentObjectsDereferencedAfterCheck) {
  static constexpr llvm::StringRef Src = R"cc(
    struct S {
      int* const& accessor() const { return i; }
      int* i = nullptr;
    };

    S makeS();

    void target() {
      if (makeS().accessor() != nullptr) {
        *makeS().accessor();
      }
    }
  )cc";
  EXPECT_THAT(collectEvidenceFromTargetFunction(Src),
              UnorderedElementsAre(evidence(SLOT_RETURN_TYPE,
                                            Evidence::UNCHECKED_DEREFERENCE,
                                            functionNamed("accessor"))));
}

TEST(CollectEvidenceFromImplementationTest,
     MemberCallOperatorReturnDereferenced) {
  static constexpr llvm::StringRef Src = R"cc(
    struct S {
      int* operator()();
    };
    void target() {
      S s;
      *s();
    }
  )cc";
  EXPECT_THAT(
      collectEvidenceFromTargetFunction(Src),
      Contains(evidence(SLOT_RETURN_TYPE, Evidence::UNCHECKED_DEREFERENCE,
                        functionNamed("operator()"))));
}

TEST(CollectEvidenceFromImplementationTest, MemberOperatorCall) {
  static constexpr llvm::StringRef Src = R"cc(
    struct S {
      bool operator+(int*);
    };
    void target() { S{} + nullptr; }
  )cc";
  EXPECT_THAT(collectEvidenceFromTargetFunction(Src),
              Contains(evidence(paramSlot(0), Evidence::NULLABLE_ARGUMENT,
                                functionNamed("operator+"))));
}

TEST(CollectEvidenceFromImplementationTest, NonMemberOperatorCall) {
  static constexpr llvm::StringRef Src = R"cc(
    struct S {};
    bool operator+(const S&, int*);
    void target() { S{} + nullptr; }
  )cc";
  EXPECT_THAT(collectEvidenceFromTargetFunction(Src),
              Contains(evidence(paramSlot(1), Evidence::NULLABLE_ARGUMENT,
                                functionNamed("operator+"))));
}

TEST(CollectEvidenceFromImplementationTest, VarArgs) {
  static constexpr llvm::StringRef Src = R"cc(
    void callee(int*...);
    void target() { callee(nullptr, nullptr); }
  )cc";
  EXPECT_THAT(
      collectEvidenceFromTargetFunction(Src),
      UnorderedElementsAre(evidence(paramSlot(0), Evidence::NULLABLE_ARGUMENT,
                                    functionNamed("callee"))));
}

TEST(CollectEvidenceFromImplementationTest, MemberOperatorCallVarArgs) {
  static constexpr llvm::StringRef Src = R"cc(
    struct S {
      bool operator()(int*...);
    };
    void target() { S{}(nullptr, nullptr); }
  )cc";
  EXPECT_THAT(
      collectEvidenceFromTargetFunction(Src),
      UnorderedElementsAre(evidence(paramSlot(0), Evidence::NULLABLE_ARGUMENT,
                                    functionNamed("operator()"))));
}

TEST(CollectEvidenceFromImplementationTest, ConstructorCall) {
  static constexpr llvm::StringRef Src = R"cc(
    struct S {
      S(Nonnull<int*> a);
    };
    void target(int* p) { S s(p); }
  )cc";
  EXPECT_THAT(
      collectEvidenceFromTargetFunction(Src),
      UnorderedElementsAre(evidence(paramSlot(0), Evidence::BOUND_TO_NONNULL,
                                    functionNamed("target")),
                           evidence(paramSlot(0), Evidence::UNKNOWN_ARGUMENT,
                                    functionNamed("S"))));
}

TEST(CollectEvidenceFromImplementationTest, NonTargetConstructorCall) {
  static constexpr llvm::StringRef Src = R"cc(
    template <typename T>
    struct S {
      // Not a target due to templating, but the annotation here can still
      // provide evidence for `p` from the call in `target`'s body.
      S(Nonnull<T*> a);
    };
    void target(int* p) { S s(p); }
  )cc";
  EXPECT_THAT(
      collectEvidenceFromTargetFunction(Src),
      UnorderedElementsAre(evidence(paramSlot(0), Evidence::BOUND_TO_NONNULL,
                                    functionNamed("target"))));
}

TEST(CollectEvidenceFromImplementationTest,
     ConstructorCallWithBaseInitializer) {
  static constexpr llvm::StringRef Src = R"cc(
    struct TakeNonnull {
      explicit TakeNonnull(Nonnull<int *>);
    };
    struct target : TakeNonnull {
      target(int *i) : TakeNonnull(i) {}
    };
  )cc";
  EXPECT_THAT(collectEvidenceFromTargetFunction(Src),
              Contains(evidence(paramSlot(0), Evidence::BOUND_TO_NONNULL,
                                functionNamed("target"))));
}

TEST(CollectEvidenceFromImplementationTest,
     ConstructorCallWithDelegatingConstructor) {
  static constexpr llvm::StringRef Src = R"cc(
    struct target {
      target(int* i);
      target() : target(nullptr){};
    };
  )cc";

  std::vector<Evidence> Results;
  clang::TestAST AST(Src);
  USRCache UsrCache;

  const auto& Delegator = *selectFirst<FunctionDecl>(
      "d", match(functionDecl(hasName("target"), parameterCountIs(0)).bind("d"),
                 AST.context()));

  auto Err = collectEvidenceFromImplementation(
      Delegator,
      evidenceEmitter([&](const Evidence& E) { Results.push_back(E); },
                      UsrCache),
      UsrCache);
  if (Err) ADD_FAILURE() << toString(std::move(Err));

  EXPECT_THAT(Results,
              Contains(evidence(paramSlot(0), Evidence::NULLABLE_ARGUMENT,
                                functionNamed("target"))));
}

TEST(CollectEvidenceFromImplementationTest, VariadicConstructorCall) {
  static constexpr llvm::StringRef Src = R"cc(
    struct S {
      S(Nonnull<int*> i, ...);
    };
    void target(int* p, int* q) { S s(p, q); }
  )cc";
  EXPECT_THAT(
      collectEvidenceFromTargetFunction(Src),
      UnorderedElementsAre(evidence(paramSlot(0), Evidence::BOUND_TO_NONNULL,
                                    functionNamed("target")),
                           evidence(paramSlot(0), Evidence::UNKNOWN_ARGUMENT,
                                    functionNamed("S"))));
}

// Not yet supported: Needs special handling for CXXCtorInitializer, but is
// not likely to produce many inference results as long as we are not
// inferring annotations for fields.
TEST(DISABLED_CollectEvidenceFromImplementationTest,
     ConstructorCallWithFieldInitializer) {
  static constexpr llvm::StringRef Src = R"cc(
    struct target {
      target(int *i) : i_(i) {}
      Nonnull<int *> i_;
    };
  )cc";
  EXPECT_THAT(collectEvidenceFromTargetFunction(Src),
              Contains(evidence(paramSlot(0), Evidence::BOUND_TO_NONNULL,
                                functionNamed("target"))));
}

TEST(CollectEvidenceFromImplementationTest, PassedToNonnull) {
  static constexpr llvm::StringRef Src = R"cc(
    void callee(Nonnull<int*> i);

    void target(int* p) { callee(p); }
  )cc";
  EXPECT_THAT(
      collectEvidenceFromTargetFunction(Src),
      UnorderedElementsAre(evidence(paramSlot(0), Evidence::BOUND_TO_NONNULL,
                                    functionNamed("target")),
                           evidence(paramSlot(0), Evidence::UNKNOWN_ARGUMENT,
                                    functionNamed("callee"))));
}

TEST(CollectEvidenceFromImplementationTest, PassedToNonnullRef) {
  static constexpr llvm::StringRef Src = R"cc(
    void callee(Nonnull<int*>& i);

    void target(int* p) { callee(p); }
  )cc";
  EXPECT_THAT(
      collectEvidenceFromTargetFunction(Src),
      UnorderedElementsAre(evidence(paramSlot(0), Evidence::BOUND_TO_NONNULL,
                                    functionNamed("target")),
                           evidence(paramSlot(0), Evidence::UNKNOWN_ARGUMENT,
                                    functionNamed("callee"))));
}

TEST(CollectEvidenceFromImplementationTest, PassedToNonnullInMemberFunction) {
  static constexpr llvm::StringRef Src = R"cc(
    struct S {
      void callee(Nonnull<int*> i);
    };

    void target(int* p) {
      S s;
      s.callee(p);
    }
  )cc";
  EXPECT_THAT(
      collectEvidenceFromTargetFunction(Src),
      UnorderedElementsAre(evidence(paramSlot(0), Evidence::BOUND_TO_NONNULL,
                                    functionNamed("target")),
                           evidence(paramSlot(0), Evidence::UNKNOWN_ARGUMENT,
                                    functionNamed("callee"))));
}

TEST(CollectEvidenceFromImplementationTest,
     PassedToNonnullInFunctionPointerParam) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int* p, void (*callee)(Nonnull<int*> i)) {
      callee(p);
    }
  )cc";
  EXPECT_THAT(collectEvidenceFromTargetFunction(Src),
              UnorderedElementsAre(
                  evidence(paramSlot(0), Evidence::BOUND_TO_NONNULL,
                           functionNamed("target")),
                  evidence(paramSlot(1), Evidence::UNCHECKED_DEREFERENCE,
                           functionNamed("target"))));
}

TEST(CollectEvidenceFromImplementationTest,
     PassedToNonnullInFunctionPointerField) {
  static constexpr llvm::StringRef Src = R"cc(
    struct S {
      void (*callee)(Nonnull<int*> i);
    };

    void target(int* p) { S().callee(p); }
  )cc";
  EXPECT_THAT(
      collectEvidenceFromTargetFunction(Src),
      UnorderedElementsAre(evidence(paramSlot(0), Evidence::BOUND_TO_NONNULL,
                                    functionNamed("target"))));
}

TEST(CollectEvidenceFromImplementationTest,
     PassedToNonnullInFunctionPointerFromAddressOfFunctionDecl) {
  static constexpr llvm::StringRef Src = R"cc(
    void callee(Nonnull<int*> i);

    void target(int* p) { (&callee)(p); }
  )cc";
  EXPECT_THAT(
      collectEvidenceFromTargetFunction(Src),
      UnorderedElementsAre(evidence(paramSlot(0), Evidence::BOUND_TO_NONNULL,
                                    functionNamed("target")),
                           evidence(paramSlot(0), Evidence::UNKNOWN_ARGUMENT,
                                    functionNamed("callee"))));
}

TEST(CollectEvidenceFromImplementationTest, FunctionCallPassedToNonnull) {
  static constexpr llvm::StringRef Src = R"cc(
    void callee(Nonnull<int*> i);
    int* makeIntPtr();

    void target() { callee(makeIntPtr()); }
  )cc";
  EXPECT_THAT(collectEvidenceFromTargetFunction(Src),
              UnorderedElementsAre(
                  evidence(SLOT_RETURN_TYPE, Evidence::BOUND_TO_NONNULL,
                           functionNamed("makeIntPtr")),
                  evidence(paramSlot(0), Evidence::UNKNOWN_ARGUMENT,
                           functionNamed("callee"))));
}

TEST(CollectEvidenceFromImplementationTest,
     FunctionCallPassedToNonnullFunctionPointer) {
  static constexpr llvm::StringRef Src = R"cc(
    int* makeIntPtr();

    void target(void (*callee)(Nonnull<int*> i)) { callee(makeIntPtr()); }
  )cc";
  EXPECT_THAT(collectEvidenceFromTargetFunction(Src),
              UnorderedElementsAre(
                  evidence(SLOT_RETURN_TYPE, Evidence::BOUND_TO_NONNULL,
                           functionNamed("makeIntPtr")),
                  evidence(paramSlot(0), Evidence::UNCHECKED_DEREFERENCE,
                           functionNamed("target"))));
}

TEST(CollectEvidenceFromImplementationTest,
     FunctionCallPassedToNonnullFunctionPointerTargetNotAnInferenceTarget) {
  static constexpr llvm::StringRef Src = R"cc(
    int* makeIntPtr();

    template <typename T>
    void target(void (*callee)(Nonnull<T*> i), int* a) {
      callee(makeIntPtr());
      *a;
    }

    void instantiate() {
      target<int>([](Nonnull<int*> i) {}, nullptr);
    }
  )cc";

  clang::TestAST AST(getInputsWithAnnotationDefinitions(Src));
  auto TargetInstantiationNodes = match(
      functionDecl(hasName("target"), isTemplateInstantiation()).bind("target"),
      AST.context());
  ASSERT_THAT(TargetInstantiationNodes, SizeIs(1));
  auto* const InstantiationDecl = ast_matchers::selectFirst<FunctionDecl>(
      "target", TargetInstantiationNodes);
  ASSERT_NE(InstantiationDecl, nullptr);

  USRCache USRCache;
  std::vector<Evidence> Results;
  auto Err = collectEvidenceFromImplementation(
      *InstantiationDecl,
      evidenceEmitter([&](const Evidence& E) { Results.push_back(E); },
                      USRCache),
      USRCache);
  if (Err) ADD_FAILURE() << toString(std::move(Err));
  // Doesn't collect any evidence for target from target's body, only collects
  // some for makeIntPtr.
  EXPECT_THAT(Results, UnorderedElementsAre(evidence(
                           SLOT_RETURN_TYPE, Evidence::BOUND_TO_NONNULL,
                           functionNamed("makeIntPtr"))));
}

TEST(CollectEvidenceFromImplementationTest, PassedToNullable) {
  static constexpr llvm::StringRef Src = R"cc(
    void callee(Nullable<int*> i);

    void target(int* p) { callee(p); }
  )cc";
  EXPECT_THAT(collectEvidenceFromTargetFunction(Src),
              Not(Contains(evidence(_, _, functionNamed("target")))));
}

TEST(CollectEvidenceFromImplementationTest, PassedToNullableRef) {
  static constexpr llvm::StringRef Src = R"cc(
    void callee(Nullable<int*>& i);

    void target(int* p) { callee(p); }
  )cc";
  EXPECT_THAT(collectEvidenceFromTargetFunction(Src),
              UnorderedElementsAre(
                  evidence(paramSlot(0), Evidence::BOUND_TO_MUTABLE_NULLABLE,
                           functionNamed("target")),
                  evidence(paramSlot(0), Evidence::UNKNOWN_ARGUMENT,
                           functionNamed("callee"))));
}

TEST(CollectEvidenceFromImplementationTest,
     PassedToNullableRefFromStoredFunctionCall) {
  static constexpr llvm::StringRef Src = R"cc(
    void callee(Nullable<int*>& i);
    int* producer();

    void target() {
      auto p = producer();
      callee(p);
    }
  )cc";
  EXPECT_THAT(
      collectEvidenceFromTargetFunction(Src),
      UnorderedElementsAre(
          evidence(SLOT_RETURN_TYPE, Evidence::BOUND_TO_MUTABLE_NULLABLE,
                   functionNamed("producer")),
          evidence(paramSlot(0), Evidence::UNKNOWN_ARGUMENT,
                   functionNamed("callee"))));
}

TEST(CollectEvidenceFromImplementationTest,
     PassedToNullableRefFromFunctionCall) {
  static constexpr llvm::StringRef Src = R"cc(
    void callee(Nullable<int*>& i);
    int*& producer();

    void target() { callee(producer()); }
  )cc";
  EXPECT_THAT(
      collectEvidenceFromTargetFunction(Src),
      UnorderedElementsAre(
          evidence(SLOT_RETURN_TYPE, Evidence::BOUND_TO_MUTABLE_NULLABLE,
                   functionNamed("producer")),
          evidence(paramSlot(0), Evidence::UNKNOWN_ARGUMENT,
                   functionNamed("callee"))));
}

TEST(CollectEvidenceFromImplementationTest, AssignedToNonnull) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int* p, int* q, int* r) {
      Nonnull<int*> a = p, b = q;
      a = r;
    }
  )cc";
  EXPECT_THAT(
      collectEvidenceFromTargetFunction(Src),
      UnorderedElementsAre(evidence(paramSlot(0), Evidence::BOUND_TO_NONNULL,
                                    functionNamed("target")),
                           evidence(paramSlot(1), Evidence::BOUND_TO_NONNULL,
                                    functionNamed("target")),
                           evidence(paramSlot(2), Evidence::BOUND_TO_NONNULL,
                                    functionNamed("target"))));
}

TEST(CollectEvidenceFromImplementationTest, RefAssignedToNonnullRef) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int*& p) {
      Nonnull<int*>& a = p;
    }
  )cc";
  EXPECT_THAT(
      collectEvidenceFromTargetFunction(Src),
      UnorderedElementsAre(evidence(paramSlot(0), Evidence::BOUND_TO_NONNULL,
                                    functionNamed("target"))));
}

TEST(CollectEvidenceFromImplementationTest,
     FunctionCallAssignedToNonnullTargetNotAnInferenceTarget) {
  static constexpr llvm::StringRef Src = R"cc(
    int* makeIntPtr();

    template <typename T>
    void target() {
      Nonnull<T*> p = makeIntPtr();
    }

    void instantiate() { target<int>(); }
  )cc";

  clang::TestAST AST(getInputsWithAnnotationDefinitions(Src));
  auto TargetInstantiationNodes = match(
      functionDecl(hasName("target"), isTemplateInstantiation()).bind("target"),
      AST.context());
  ASSERT_THAT(TargetInstantiationNodes, SizeIs(1));
  auto* const InstantiationDecl = ast_matchers::selectFirst<FunctionDecl>(
      "target", TargetInstantiationNodes);
  ASSERT_NE(InstantiationDecl, nullptr);

  USRCache USRCache;
  std::vector<Evidence> Results;
  auto Err = collectEvidenceFromImplementation(
      *InstantiationDecl,
      evidenceEmitter([&](const Evidence& E) { Results.push_back(E); },
                      USRCache),
      USRCache);
  if (Err) ADD_FAILURE() << toString(std::move(Err));
  // Doesn't collect any evidence for target from target's body, only collects
  // some for makeIntPtr.
  EXPECT_THAT(Results, UnorderedElementsAre(evidence(
                           SLOT_RETURN_TYPE, Evidence::BOUND_TO_NONNULL,
                           functionNamed("makeIntPtr"))));
}

TEST(CollectEvidenceFromImplementationTest, AssignedToNullableOrUnknown) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int* p, int* q, int* r) {
      Nullable<int*> a = p;
      int* b = q;
      NullabilityUnknown<int*> c = r;
      q = r;
    }
  )cc";
  EXPECT_THAT(collectEvidenceFromTargetFunction(Src), IsEmpty());
}

TEST(CollectEvidenceFromImplementationTest, AssignedToNullableRef) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int* p) {
      Nullable<int*>& a = p;
    }
  )cc";
  EXPECT_THAT(collectEvidenceFromTargetFunction(Src),
              UnorderedElementsAre(evidence(paramSlot(0),
                                            Evidence::BOUND_TO_MUTABLE_NULLABLE,
                                            functionNamed("target"))));
}

TEST(CollectEvidenceFromImplementationTest, RefAssignedToNullableRef) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int*& p) {
      Nullable<int*>& a = p;
    }
  )cc";
  EXPECT_THAT(collectEvidenceFromTargetFunction(Src),
              UnorderedElementsAre(evidence(paramSlot(0),
                                            Evidence::BOUND_TO_MUTABLE_NULLABLE,
                                            functionNamed("target"))));
}

// Ternary expressions are not currently modeled correctly by the analysis, but
// are necessary to test the case of multiple connected decls.
//
// DISABLED until ternary expressions are handle.
TEST(CollectEvidenceFromImplementationTest,
     DISABLED_AssignedToNullableRefAllConnectedDecls) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int* p, int* q, bool b) {
      Nullable<int*>& x = b ? p : q;
    }
  )cc";
  EXPECT_THAT(collectEvidenceFromTargetFunction(Src),
              UnorderedElementsAre(
                  evidence(paramSlot(0), Evidence::BOUND_TO_MUTABLE_NULLABLE,
                           functionNamed("target")),
                  evidence(paramSlot(1), Evidence::BOUND_TO_MUTABLE_NULLABLE,
                           functionNamed("target"))));
}

TEST(CollectEvidenceFromImplementationTest, IrrelevantAssignments) {
  static constexpr llvm::StringRef Src = R"cc(
    struct S {
      S(int* i);
    };

    void target(int* p) {
      int* a = p;  // No useful information.

      // We don't collect if types on either side are not a supported pointer
      // type.
      int* b = 0;
      int c = 4;
      bool d = false;
      S e = a;

      // We don't collect from compound assignments.
      b += 8;
    }
  )cc";
  EXPECT_THAT(
      collectEvidenceFromTargetFunction(Src),
      // From the constructor call constructing an S; no evidence from
      // assignments.
      UnorderedElementsAre(evidence(paramSlot(0), Evidence::UNKNOWN_ARGUMENT,
                                    functionNamed("S"))));
}

TEST(CollectEvidenceFromImplementationTest, FunctionCallInLoop) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int* p) {
      for (int i = 0; i < 3; ++i) {
        target(nullptr);
      }
      for (int i = 0; i < 3; ++i) {
        target(&i);
      }
      for (int i = 0; i < 3; ++i) {
        target(p);
      }
    }
  )cc";
  EXPECT_THAT(
      collectEvidenceFromTargetFunction(Src),
      UnorderedElementsAre(evidence(paramSlot(0), Evidence::NULLABLE_ARGUMENT,
                                    functionNamed("target")),
                           evidence(paramSlot(0), Evidence::UNKNOWN_ARGUMENT,
                                    functionNamed("target")),
                           evidence(paramSlot(0), Evidence::NONNULL_ARGUMENT,
                                    functionNamed("target"))));
}

TEST(CollectEvidenceFromImplementationTest, OutputParameterPointerToPointer) {
  static constexpr llvm::StringRef Src = R"cc(
    void maybeModifyPtr(int** a);
    void target(int* p) {
      maybeModifyPtr(&p);
      *p;
    }
  )cc";
  EXPECT_THAT(collectEvidenceFromTargetFunction(Src),
              Not(Contains(evidence(_, _, functionNamed("target")))));
}

TEST(CollectEvidenceFromImplementationTest, OutputParameterReferenceToPointer) {
  static constexpr llvm::StringRef Src = R"cc(
    void maybeModifyPtr(int*& a);
    void target(int* p) {
      maybeModifyPtr(p);
      *p;
    }
  )cc";
  EXPECT_THAT(collectEvidenceFromTargetFunction(Src),
              Not(Contains(evidence(_, _, functionNamed("target")))));
}

TEST(CollectEvidenceFromImplementationTest,
     OutputParameterReferenceToConstPointer) {
  static constexpr llvm::StringRef Src = R"cc(
    void dontModifyPtr(int* const& a);
    void target(int* p) {
      dontModifyPtr(p);
      *p;
    }
  )cc";
  EXPECT_THAT(collectEvidenceFromTargetFunction(Src),
              Contains(evidence(paramSlot(0), Evidence::UNCHECKED_DEREFERENCE,
                                functionNamed("target"))));
}

TEST(CollectEvidenceFromImplementationTest,
     OutputParameterReferenceToPointerToPointer) {
  static constexpr llvm::StringRef Src = R"cc(
    void maybeModifyPtr(int**& a);
    void target(int** p) {
      maybeModifyPtr(p);
      *p;
      **p;
    }
  )cc";
  EXPECT_THAT(collectEvidenceFromTargetFunction(Src),
              Not(Contains(evidence(_, _, functionNamed("target")))));
}

TEST(CollectEvidenceFromImplementationTest,
     OutputParameterPointerToConstPointer) {
  static constexpr llvm::StringRef Src = R"cc(
    void dontModifyPtr(int* const* a);
    void target(int* p) {
      dontModifyPtr(&p);
      *p;
    }
  )cc";
  EXPECT_THAT(collectEvidenceFromTargetFunction(Src),
              Contains(evidence(paramSlot(0), Evidence::UNCHECKED_DEREFERENCE,
                                functionNamed("target"))));
}

TEST(CollectEvidenceFromImplementationTest,
     OutputParameterConstPointerToPointerToConst) {
  static constexpr llvm::StringRef Src = R"cc(
    // Outer pointer and int are const, but inner pointer can still be modified.
    void maybeModifyPtr(const int** const a);
    void target(const int* p) {
      maybeModifyPtr(&p);
      *p;
    }
  )cc";
  EXPECT_THAT(collectEvidenceFromTargetFunction(Src),
              Not(Contains(evidence(_, _, functionNamed("target")))));
}

TEST(CollectEvidenceFromImplementationTest,
     PassAsOutputParameterOrDereference) {
  static constexpr llvm::StringRef Src = R"cc(
    void maybeModifyPtr(int** a);
    void target(int* p, bool b) {
      if (b) {
        maybeModifyPtr(&p);
      } else {
        *p;
      }
    }
  )cc";
  EXPECT_THAT(collectEvidenceFromTargetFunction(Src),
              Contains(evidence(paramSlot(0), Evidence::UNCHECKED_DEREFERENCE,
                                functionNamed("target"))));
}

TEST(CollectEvidenceFromImplementationTest,
     ConditionallyPassAsOutputParameterAlwaysDereference) {
  static constexpr llvm::StringRef Src = R"cc(
    void maybeModifyPtr(int** a);
    void target(int* p, bool b) {
      if (b) maybeModifyPtr(&p);
      *p;  // Because we model p as Unknown post-output-parameter-use, adding an
           // annotation would not be considered sufficient to make this
           // dereference safe, so we do not collect evidence for p.
    }
  )cc";
  EXPECT_THAT(collectEvidenceFromTargetFunction(Src),
              Not(Contains(evidence(_, _, functionNamed("target")))));
}

TEST(CollectEvidenceFromImplementationTest, NotInferenceTarget) {
  static constexpr llvm::StringRef Src = R"cc(
    void isATarget(Nonnull<int*> a);
    template <typename T>
    T* target(T* p) {
      *p;
      Nonnull<int*> a = p;
      isATarget(p);
      target<T>(nullptr);
      target<int>(nullptr);
      return nullptr;
    }

    void instantiate() { target<int>(nullptr); }
  )cc";

  clang::TestAST AST(getInputsWithAnnotationDefinitions(Src));
  auto TargetInstantiationNodes = match(
      functionDecl(hasName("target"), isTemplateInstantiation()).bind("target"),
      AST.context());
  ASSERT_THAT(TargetInstantiationNodes, SizeIs(1));
  auto* const InstantiationDecl = ast_matchers::selectFirst<FunctionDecl>(
      "target", TargetInstantiationNodes);
  ASSERT_NE(InstantiationDecl, nullptr);

  USRCache USRCache;
  std::vector<Evidence> Results;
  auto Err = collectEvidenceFromImplementation(
      *InstantiationDecl,
      evidenceEmitter([&](const Evidence& E) { Results.push_back(E); },
                      USRCache),
      USRCache);
  if (Err) ADD_FAILURE() << toString(std::move(Err));
  // Doesn't collect any evidence for target from target's body, only collects
  // some for isATarget.
  EXPECT_THAT(Results, UnorderedElementsAre(
                           evidence(paramSlot(0), Evidence::UNKNOWN_ARGUMENT,
                                    functionNamed("isATarget"))));
}

TEST(CollectEvidenceFromImplementationTest, PropagatesPreviousInferences) {
  static constexpr llvm::StringRef Src = R"cc(
    void calledWithToBeNullable(int* x);
    void calledWithToBeNonnull(int* a);
    void target(int* p, int* q) {
      target(nullptr, q);
      calledWithToBeNullable(p);
      *q;
      calledWithToBeNonnull(q);
    }
  )cc";
  std::string TargetUsr = "c:@F@target#*I#S0_#";
  std::vector ExpectedBothRoundResults = {
      evidence(paramSlot(0), Evidence::NULLABLE_ARGUMENT,
               AllOf(functionNamed("target"),
                     // Double-check that target's usr is as expected before we
                     // use it to create SlotFingerprints.
                     ResultOf([](Symbol S) { return S.usr(); }, TargetUsr))),
      evidence(paramSlot(1), Evidence::UNCHECKED_DEREFERENCE,
               functionNamed("target")),
  };
  std::vector ExpectedSecondRoundResults = {
      evidence(paramSlot(0), Evidence::NULLABLE_ARGUMENT,
               functionNamed("calledWithToBeNullable")),
      evidence(paramSlot(0), Evidence::NONNULL_ARGUMENT,
               functionNamed("calledWithToBeNonnull"))};

  // Only proceed if we have the correct USR for target and the first round
  // results contain the evidence needed to produce our expected inferences and
  // do not contain the evidence only found from propagating inferences from the
  // first round.
  auto FirstRoundResults = collectEvidenceFromTargetFunction(Src);
  ASSERT_THAT(FirstRoundResults, IsSupersetOf(ExpectedBothRoundResults));
  for (const auto& E : ExpectedSecondRoundResults) {
    ASSERT_THAT(FirstRoundResults, Not(Contains(E)));
  }

  EXPECT_THAT(collectEvidenceFromTargetFunction(
                  Src, {/*Nullable=*/{fingerprint(TargetUsr, paramSlot(0))},
                        /*Nonnull=*/{fingerprint(TargetUsr, paramSlot(1))}}),
              AllOf(IsSupersetOf(ExpectedBothRoundResults),
                    IsSupersetOf(ExpectedSecondRoundResults)));
}

TEST(CollectEvidenceFromImplementationTest,
     AnalysisUsesPreviousInferencesForSlotsOutsideTargetImplementation) {
  static constexpr llvm::StringRef Src = R"cc(
    int* returnsToBeNonnull(int* a) {
      return a;
    }
    int* target(int* q) {
      *q;
      return returnsToBeNonnull(q);
    }
  )cc";
  std::string TargetUsr = "c:@F@target#*I#";
  std::string ReturnsToBeNonnullUsr = "c:@F@returnsToBeNonnull#*I#";
  const llvm::DenseMap<int, std::vector<testing::Matcher<const Evidence&>>>
      ExpectedNewResultsPerRound = {
          {0,
           {evidence(
               paramSlot(0), Evidence::UNCHECKED_DEREFERENCE,
               AllOf(functionNamed("target"),
                     // Double-check that target's usr is as expected before
                     // we use it to create SlotFingerprints.
                     ResultOf([](Symbol S) { return S.usr(); }, TargetUsr)))}},
          {1,
           {evidence(
               paramSlot(0), Evidence::NONNULL_ARGUMENT,
               AllOf(functionNamed("returnsToBeNonnull"),
                     // Double-check that returnsToBeNonnull's usr is as
                     // expected before we use it to create SlotFingerprints.
                     ResultOf([](Symbol S) { return S.usr(); },
                              ReturnsToBeNonnullUsr)))}},
          {2,
           {
               // No new evidence from target's implementation in this round,
               // but in a full-TU analysis, this would be the round where we
               // decide returnsToBeNonnull returns Nonnull, based on the
               // now-Nonnull argument that is the only return value.
           }},
          {3,
           {evidence(SLOT_RETURN_TYPE, Evidence::NONNULL_RETURN,
                     functionNamed("target"))}}};

  // Assert first round results because they don't rely on previous inference
  // propagation at all and in this case are test setup and preconditions.
  auto FirstRoundResults = collectEvidenceFromTargetFunction(Src);
  ASSERT_THAT(FirstRoundResults,
              IsSupersetOf(ExpectedNewResultsPerRound.at(0)));
  for (const auto& E : ExpectedNewResultsPerRound.at(1)) {
    ASSERT_THAT(FirstRoundResults, Not(Contains(E)));
  }

  auto SecondRoundResults = collectEvidenceFromTargetFunction(
      Src, {.Nonnull = {fingerprint(TargetUsr, paramSlot(0))}});
  EXPECT_THAT(SecondRoundResults,
              AllOf(IsSupersetOf(ExpectedNewResultsPerRound.at(0)),
                    IsSupersetOf(ExpectedNewResultsPerRound.at(1))));
  for (const auto& E : ExpectedNewResultsPerRound.at(2)) {
    ASSERT_THAT(SecondRoundResults, Not(Contains(E)));
  }

  auto ThirdRoundResults = collectEvidenceFromTargetFunction(
      Src, {.Nonnull = {fingerprint(TargetUsr, paramSlot(0)),
                        fingerprint(ReturnsToBeNonnullUsr, paramSlot(0))}});
  EXPECT_THAT(ThirdRoundResults,
              AllOf(IsSupersetOf(ExpectedNewResultsPerRound.at(0)),
                    IsSupersetOf(ExpectedNewResultsPerRound.at(1)),
                    IsSupersetOf(ExpectedNewResultsPerRound.at(2))));
  for (const auto& E : ExpectedNewResultsPerRound.at(3)) {
    ASSERT_THAT(ThirdRoundResults, Not(Contains(E)));
  }

  auto FourthRoundResults = collectEvidenceFromTargetFunction(
      Src,
      {.Nonnull = {
           fingerprint(TargetUsr, paramSlot(0)),
           fingerprint(ReturnsToBeNonnullUsr, paramSlot(0)),
           // As noted in the Evidence matcher list above, we don't infer the
           // return type of returnsToBeNonnull from only collecting evidence
           // from target's implementation, but for the sake of this test, let's
           // pretend we collected evidence from the entire TU.
           fingerprint(ReturnsToBeNonnullUsr, SLOT_RETURN_TYPE)}});
  EXPECT_THAT(FourthRoundResults,
              AllOf(IsSupersetOf(ExpectedNewResultsPerRound.at(0)),
                    IsSupersetOf(ExpectedNewResultsPerRound.at(1)),
                    IsSupersetOf(ExpectedNewResultsPerRound.at(2)),
                    IsSupersetOf(ExpectedNewResultsPerRound.at(3))));
}

TEST(CollectEvidenceFromImplementationTest,
     PreviousInferencesOfNonTargetParameterNullabilitiesPropagate) {
  static constexpr llvm::StringRef Src = R"cc(
    void takesToBeNonnull(int* a) {
      // Not read when collecting evidence only from Target, but corresponding
      // inference is explicitly input below.
      *a;
    }
    void target(int* q) { takesToBeNonnull(q); }
  )cc";
  std::string TakesToBeNonnullUsr = "c:@F@takesToBeNonnull#*I#";

  // Pretend that in a first round of inferring for all functions, we made this
  // inference about takesToBeNonnull's first parameter.
  // This test confirms that we use that information when collecting from
  // target's implementation.
  EXPECT_THAT(
      collectEvidenceFromTargetFunction(
          Src, {.Nonnull = {fingerprint(TakesToBeNonnullUsr, paramSlot(0))}}),
      Contains(evidence(paramSlot(0), Evidence::BOUND_TO_NONNULL,
                        functionNamed("target"))));
}

TEST(CollectEvidenceFromDeclarationTest, VariableDeclIgnored) {
  llvm::StringLiteral Src = R"cc(Nullable<int *> target;)cc";
  EXPECT_THAT(collectEvidenceFromTargetDecl(Src), IsEmpty());
}

TEST(CollectEvidenceFromDeclarationTest, FunctionDeclReturnType) {
  llvm::StringLiteral Src = R"cc(Nonnull<int *> target();)cc";
  EXPECT_THAT(
      collectEvidenceFromTargetDecl(Src),
      ElementsAre(evidence(SLOT_RETURN_TYPE, Evidence::ANNOTATED_NONNULL,
                           functionNamed("target"))));
}

TEST(CollectEvidenceFromDeclarationTest, FunctionDeclParams) {
  llvm::StringLiteral Src =
      R"cc(void target(Nullable<int*>, int*, Nonnull<int*>);)cc";
  EXPECT_THAT(collectEvidenceFromTargetDecl(Src),
              ElementsAre(evidence(paramSlot(0), Evidence::ANNOTATED_NULLABLE),
                          evidence(paramSlot(2), Evidence::ANNOTATED_NONNULL)));
}

TEST(CollectEvidenceFromDeclarationTest, FunctionDeclNonTopLevel) {
  llvm::StringLiteral Src = R"cc(Nonnull<int*>** target(Nullable<int*>*);)cc";
  EXPECT_THAT(collectEvidenceFromTargetDecl(Src), IsEmpty());
}

TEST(CollectEvidenceFromDeclarationTest, FunctionTemplateIgnored) {
  // We used to inspect the type of `target` and crash.
  llvm::StringLiteral Src = R"cc(
    template <class A>
    struct S {
      template <class B>
      static void target(const S<B>&) {}
    };
  )cc";
  EXPECT_THAT(collectEvidenceFromTargetDecl(Src), IsEmpty());
}

TEST(CollectEvidenceFromDeclarationTest, DefaultArgumentNullptrLiteral) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int* = nullptr);
  )cc";
  EXPECT_THAT(
      collectEvidenceFromTargetDecl(Src),
      UnorderedElementsAre(evidence(paramSlot(0), Evidence::NULLABLE_ARGUMENT,
                                    functionNamed("target"))));
}

TEST(CollectEvidenceFromDeclarationTest, DefaultArgumentZeroLiteral) {
  static constexpr llvm::StringRef Src =
      R"cc(
    void target(int* = 0);
      )cc";
  EXPECT_THAT(
      collectEvidenceFromTargetDecl(Src),
      UnorderedElementsAre(evidence(paramSlot(0), Evidence::NULLABLE_ARGUMENT,
                                    functionNamed("target"))));
}

TEST(CollectEvidenceFromDeclarationTest, DefaultArgumentAnnotatedVariable) {
  static constexpr llvm::StringRef Src = R"cc(
    Nonnull<int*> q;
    void target(int* = q);
  )cc";
  EXPECT_THAT(
      collectEvidenceFromTargetDecl(Src),
      UnorderedElementsAre(evidence(paramSlot(0), Evidence::NONNULL_ARGUMENT,
                                    functionNamed("target"))));
}

TEST(CollectEvidenceFromDeclarationTest,
     DefaultArgumentCallingAnnotatedFunction) {
  static constexpr llvm::StringRef Src = R"cc(
    Nullable<int*> getDefault();
    void target(int* = getDefault());
  )cc";
  EXPECT_THAT(
      collectEvidenceFromTargetDecl(Src),
      UnorderedElementsAre(evidence(paramSlot(0), Evidence::NULLABLE_ARGUMENT,
                                    functionNamed("target"))));
}

TEST(CollectEvidenceFromDeclarationTest,
     DefaultArgumentUnannotatedNonLiteralExpressionsUnknown) {
  static constexpr llvm::StringRef Src = R"cc(
    int* getDefault();
    int* q = nullptr;
    int i = 1;
    void target(int* = getDefault(), int* = q, int* = &i);
  )cc";
  EXPECT_THAT(
      collectEvidenceFromTargetDecl(Src),
      UnorderedElementsAre(evidence(paramSlot(0), Evidence::UNKNOWN_ARGUMENT,
                                    functionNamed("target")),
                           evidence(paramSlot(1), Evidence::UNKNOWN_ARGUMENT,
                                    functionNamed("target")),
                           evidence(paramSlot(2), Evidence::UNKNOWN_ARGUMENT,
                                    functionNamed("target"))));
}

MATCHER_P(declNamed, Name, "") {
  std::string Actual;
  llvm::raw_string_ostream OS(Actual);
  if (auto* ND = dyn_cast<NamedDecl>(arg))
    ND->getNameForDiagnostic(
        OS, arg->getDeclContext()->getParentASTContext().getPrintingPolicy(),
        /*Qualified=*/true);
  return ::testing::ExplainMatchResult(Name, Actual, result_listener);
}

TEST(EvidenceSitesTest, Functions) {
  TestAST AST(R"cc(
    void foo();
    void bar();
    void bar() {}
    void baz() {}
    auto Lambda = []() {};  // Not analyzed yet.

    struct S {
      S() {}
      void member();
    };
    void S::member() {}
  )cc");
  auto Sites = EvidenceSites::discover(AST.context());
  EXPECT_THAT(Sites.Declarations,
              ElementsAre(declNamed("foo"), declNamed("bar"), declNamed("bar"),
                          declNamed("baz"), declNamed("S::S"),
                          declNamed("S::member"), declNamed("S::member")));
  EXPECT_THAT(Sites.Implementations,
              ElementsAre(declNamed("bar"), declNamed("baz"), declNamed("S::S"),
                          declNamed("S::member")));
}

TEST(EvidenceSitesTest, Variables) {
  TestAST AST(R"cc(
    int* x = true ? nullptr : nullptr;
    struct S {
      int* s;
    };
  )cc");
  auto Sites = EvidenceSites::discover(AST.context());
  // For now, variables are not inferable.
  EXPECT_THAT(Sites.Declarations, IsEmpty());
  // For now, we don't examine variable initializers.
  EXPECT_THAT(Sites.Implementations, IsEmpty());
}

TEST(EvidenceSitesTest, Templates) {
  TestAST AST(R"cc(
    template <int I>
    int f() {
      return I;
    }
    template <>
    int f<1>() {
      return 1;
    }

    struct S {
      template <int I>
      int f() {
        return I;
      }
    };

    template <int I>
    struct T {
      int f() { return I; }
    };

    auto Unused = f<0>() + f<1>() + S{}.f<0>() + T<0>{}.f();
  )cc");
  auto Sites = EvidenceSites::discover(AST.context());

  // Relevant declarations are the written ones that are not templates.
  EXPECT_THAT(Sites.Declarations, ElementsAre(declNamed("f<1>")));
  // Instantiations are relevant inference targets.
  EXPECT_THAT(Sites.Implementations,
              ElementsAre(declNamed("f<0>"), declNamed("f<1>"),
                          declNamed("S::f<0>"), declNamed("T<0>::f")));
}

TEST(EvidenceEmitterTest, NotInferenceTarget) {
  TestAST AST(R"cc(
    template <int I>
    int target() {
      return I;
    })cc");

  const auto* TargetDecl =
      dataflow::test::findValueDecl(AST.context(), "target");
  ASSERT_NE(TargetDecl, nullptr);

  USRCache USRCache;
  EXPECT_DEATH(evidenceEmitter([](const Evidence& e) {}, USRCache)(
                   *TargetDecl, Slot{}, Evidence::ANNOTATED_UNKNOWN,
                   TargetDecl->getLocation()),
               "not an inference target");
}

}  // namespace
}  // namespace clang::tidy::nullability
