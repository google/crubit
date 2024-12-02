// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/inference/merge.h"

#include <array>

#include "absl/log/check.h"
#include "nullability/inference/inference.proto.h"
#include "nullability/proto_matchers.h"
#include "llvm/ADT/StringRef.h"
#include "third_party/llvm/llvm-project/third-party/unittest/googlemock/include/gmock/gmock.h"
#include "third_party/llvm/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"
#include "third_party/protobuf/text_format.h"

namespace clang::tidy::nullability {
namespace {

template <typename T>
T proto(llvm::StringRef Text) {
  T Result;
  CHECK(proto2::TextFormat::ParseFromString(Text, &Result));
  return Result;
}

TEST(PartialFromEvidenceTest, ContainsEvidenceInfo) {
  EXPECT_THAT(partialFromEvidence(proto<Evidence>(R"pb(
                symbol { usr: "func" }
                slot: 1
                kind: UNCHECKED_DEREFERENCE
                location: "foo.cc:42"
              )pb")),
              EqualsProto(R"pb(
                kind_count { key: 3 value: 1 }
                kind_samples {
                  key: 3
                  value { location: "foo.cc:42" }
                }
              )pb"));
}

TEST(MergePartialsTest, BothContainEvidence) {
  auto L = proto<SlotPartial>(R"pb(
    kind_count { key: 1 value: 1 }
    kind_count { key: 0 value: 2 }
    kind_samples {
      key: 0
      value { location: "a" location: "b" }
    }
  )pb");
  auto R = proto<SlotPartial>(
      R"pb(kind_count { key: 2 value: 1 }
           kind_count { key: 0 value: 2 }
           kind_samples {
             key: 0
             value { location: "c" location: "a" location: "d" }
           })pb");

  mergePartials(L, R);
  EXPECT_THAT(L, EqualsProto(R"pb(
                kind_count { key: 0 value: 4 }
                kind_count { key: 2 value: 1 }
                kind_count { key: 1 value: 1 }
                kind_samples {
                  key: 0
                  value { location: "a" location: "b" location: "c" }
                }
              )pb"));
}

TEST(MergePartialsTest, RightEmpty) {
  auto L = proto<SlotPartial>(R"pb(kind_count { key: 0 value: 1 })pb");
  SlotPartial R;
  mergePartials(L, R);
  EXPECT_THAT(L, EqualsProto(L));
}

TEST(MergePartialsTest, LeftEmpty) {
  SlotPartial L;
  auto R = proto<SlotPartial>(R"pb(kind_count { key: 3 value: 1 })pb");
  mergePartials(L, R);
  EXPECT_THAT(L, EqualsProto(R));
}

TEST(FinalizeTest, ConflictingAnnotations) {
  EXPECT_THAT(finalize(proto<SlotPartial>(R"pb(
                kind_count { key: 1 value: 1 }  # ANNOTATED_NULLABLE
                kind_count { key: 2 value: 1 }  # ANNOTATED_NONNULL
                kind_samples {
                  key: 1
                  value { location: "decl" }
                }
                kind_samples {
                  key: 2
                  value { location: "def" }
                }
              )pb")),
              EqualsProto(R"pb(
                nullability: UNKNOWN
                conflict: true
                sample_evidence { kind: ANNOTATED_NULLABLE location: "decl" }
                sample_evidence { kind: ANNOTATED_NONNULL location: "def" }
              )pb"));
}

TEST(FinalizeTest, Empty) {
  EXPECT_THAT(finalize(SlotPartial()), EqualsProto(SlotInference()));
}

TEST(FinalizeTest, TypicalEvidence) {
  EXPECT_THAT(finalize(proto<SlotPartial>(R"pb(
                kind_count { key: 3 value: 1 }  # UNCHECKED_DEREFERENCE
              )pb")),
              EqualsProto(R"pb(nullability: NONNULL)pb"));
}

TEST(FinalizeTest, AnnotatedUnknown) {
  EXPECT_THAT(finalize(proto<SlotPartial>(R"pb(
                kind_count { key: 0 value: 1 }  # ANNOTATED_UNKNOWN
              )pb")),
              EqualsProto(R"pb(nullability: UNKNOWN)pb"));
}

TEST(FinalizeTest, AnnotatedNonUnknown) {
  EXPECT_THAT(finalize(proto<SlotPartial>(R"pb(
                kind_count { key: 1 value: 1 }  # ANNOTATED_NULLABLE
              )pb")),
              EqualsProto(R"pb(nullability: NULLABLE trivial: true)pb"));
}

TEST(MergeEvidenceTest, TreeShapedMerge) {
  SlotPartial P1 = partialFromEvidence(
      proto<Evidence>(R"pb(kind: ASSIGNED_FROM_UNKNOWN)pb"));
  SlotPartial P2 = partialFromEvidence(
      proto<Evidence>(R"pb(kind: ASSIGNED_FROM_NONNULL)pb"));
  SlotPartial P3 = partialFromEvidence(
      proto<Evidence>(R"pb(kind: GCC_NONNULL_ATTRIBUTE)pb"));
  SlotPartial P4 = partialFromEvidence(
      proto<Evidence>(R"pb(kind: ASSIGNED_FROM_NULLABLE)pb"));

  auto Expected = EqualsProto(R"pb(nullability: NULLABLE)pb");

  EXPECT_THAT(
      [&] {
        SlotPartial P = P1;
        mergePartials(P, P2);
        mergePartials(P, P3);
        mergePartials(P, P4);
        return finalize(P);
      }(),
      Expected);

  EXPECT_THAT(
      [&] {
        SlotPartial P = P4;
        mergePartials(P, P3);
        mergePartials(P, P2);
        mergePartials(P, P1);
        return finalize(P);
      }(),
      Expected);

  EXPECT_THAT(
      [&] {
        SlotPartial PA = P1;
        mergePartials(PA, P2);
        SlotPartial PB = P3;
        mergePartials(PB, P4);
        mergePartials(PA, PB);
        return finalize(PA);
      }(),
      Expected);
}

class InferTest : public ::testing::Test {
  std::array<unsigned, Evidence::Kind_MAX + 1> Counts = {};

 protected:
  void add(Evidence::Kind E, int N = 1) { Counts[E] += N; }

  Nullability infer(bool ExpectConflict = false, bool ExpectTrivial = false) {
    auto Result = nullability::infer(Counts);
    EXPECT_EQ(ExpectConflict, Result.Conflict);
    EXPECT_EQ(ExpectTrivial, Result.Trivial);
    return Result.Nullability;
  }
};

TEST_F(InferTest, NoEvidence) { EXPECT_EQ(Nullability::UNKNOWN, infer()); }

TEST_F(InferTest, Annotated) {
  add(Evidence::ANNOTATED_NULLABLE);
  EXPECT_EQ(Nullability::NULLABLE,
            infer(/*ExpectConflict=*/false, /*ExpectTrivial=*/true));
  add(Evidence::UNCHECKED_DEREFERENCE);  // No conflict, annotation wins.
  EXPECT_EQ(Nullability::NULLABLE,
            infer(/*ExpectConflict=*/false, /*ExpectTrivial=*/true));
  add(Evidence::ANNOTATED_NONNULL);  // Conflicting annotations!
  EXPECT_EQ(Nullability::UNKNOWN, infer(/*ExpectConflict=*/true));
}

TEST_F(InferTest, Deref) {
  add(Evidence::UNCHECKED_DEREFERENCE);
  EXPECT_EQ(Nullability::NONNULL, infer());
}

TEST_F(InferTest, NullableArgumentPassed) {
  add(Evidence::NULLABLE_ARGUMENT);
  EXPECT_EQ(Nullability::NULLABLE, infer());
  add(Evidence::NONNULL_ARGUMENT);
  EXPECT_EQ(Nullability::NULLABLE, infer());
  add(Evidence::UNKNOWN_ARGUMENT);
  EXPECT_EQ(Nullability::NULLABLE, infer());
  add(Evidence::UNCHECKED_DEREFERENCE);
  EXPECT_EQ(Nullability::NONNULL, infer(/*ExpectConflict=*/true));
}

TEST_F(InferTest, OnlyNonnullArgumentsPassed) {
  add(Evidence::NONNULL_ARGUMENT);
  EXPECT_EQ(Nullability::NONNULL, infer());
}

TEST_F(InferTest, NonnullAndUnknownArgumentsPassed) {
  add(Evidence::NONNULL_ARGUMENT);
  add(Evidence::UNKNOWN_ARGUMENT);
  EXPECT_EQ(Nullability::UNKNOWN, infer());
}

TEST_F(InferTest, AssignedFromNullable) {
  add(Evidence::ASSIGNED_FROM_NULLABLE);
  EXPECT_EQ(Nullability::NULLABLE, infer());
  add(Evidence::UNCHECKED_DEREFERENCE);
  EXPECT_EQ(Nullability::NONNULL, infer(/*ExpectConflict=*/true));
}

TEST_F(InferTest, ReturnValues) {
  add(Evidence::NONNULL_RETURN);
  EXPECT_EQ(Nullability::NONNULL, infer());
  add(Evidence::UNKNOWN_RETURN);
  EXPECT_EQ(Nullability::UNKNOWN, infer());
  add(Evidence::NULLABLE_RETURN);
  EXPECT_EQ(Nullability::NULLABLE, infer());
}

TEST_F(InferTest, PassedToNonnull) {
  add(Evidence::ASSIGNED_TO_NONNULL);
  EXPECT_EQ(Nullability::NONNULL, infer());
}

TEST_F(InferTest, PassedToMutableNullable) {
  add(Evidence::ASSIGNED_TO_MUTABLE_NULLABLE);
  EXPECT_EQ(Nullability::NULLABLE, infer());
  add(Evidence::ASSIGNED_TO_NONNULL);
  EXPECT_EQ(Nullability::NONNULL, infer(/*ExpectConflict=*/true));
}

TEST_F(InferTest, AbortIfNull) {
  add(Evidence::ABORT_IF_NULL);
  EXPECT_EQ(Nullability::NONNULL, infer());
  add(Evidence::NULLABLE_ARGUMENT);
  EXPECT_EQ(Nullability::NULLABLE, infer(/*ExpectConflict=*/true));
}

TEST_F(InferTest, Arithmetic) {
  add(Evidence::ARITHMETIC);
  EXPECT_EQ(Nullability::NONNULL, infer());
  add(Evidence::NULLABLE_ARGUMENT);
  EXPECT_EQ(Nullability::NULLABLE, infer(/*ExpectConflict=*/true));
}

TEST_F(InferTest, NullableDefaultMemberInitializer) {
  add(Evidence::NULLPTR_DEFAULT_MEMBER_INITIALIZER);
  EXPECT_EQ(Nullability::NULLABLE, infer());
  add(Evidence::UNCHECKED_DEREFERENCE);
  EXPECT_EQ(Nullability::NONNULL, infer());
}

TEST_F(InferTest, MixedAssignments) {
  add(Evidence::ASSIGNED_FROM_NONNULL);
  EXPECT_EQ(Nullability::NONNULL, infer());
  add(Evidence::ASSIGNED_FROM_UNKNOWN);
  EXPECT_EQ(Nullability::UNKNOWN, infer());
  add(Evidence::ASSIGNED_FROM_NULLABLE);
  EXPECT_EQ(Nullability::NULLABLE, infer());
}

TEST_F(InferTest, MixedConstReferenceReturns) {
  add(Evidence::NONNULL_REFERENCE_RETURN_AS_CONST);
  EXPECT_EQ(Nullability::NONNULL, infer());
  add(Evidence::UNKNOWN_REFERENCE_RETURN);
  EXPECT_EQ(Nullability::UNKNOWN, infer());
  add(Evidence::NULLABLE_REFERENCE_RETURN);
  EXPECT_EQ(Nullability::NULLABLE, infer());
}

TEST_F(InferTest, MixedReferenceReturns) {
  add(Evidence::UNKNOWN_REFERENCE_RETURN);
  EXPECT_EQ(Nullability::UNKNOWN, infer());
  add(Evidence::NONNULL_REFERENCE_RETURN);
  EXPECT_EQ(Nullability::NONNULL, infer());
  add(Evidence::NULLABLE_REFERENCE_RETURN);
  EXPECT_EQ(Nullability::NULLABLE, infer(/*ExpectConflict=*/true));
}

TEST_F(InferTest, MixedConstReferenceArguments) {
  add(Evidence::NONNULL_REFERENCE_ARGUMENT_AS_CONST);
  EXPECT_EQ(Nullability::NONNULL, infer());
  add(Evidence::UNKNOWN_REFERENCE_ARGUMENT);
  EXPECT_EQ(Nullability::UNKNOWN, infer());
  add(Evidence::NULLABLE_REFERENCE_ARGUMENT);
  EXPECT_EQ(Nullability::NULLABLE, infer());
}

TEST_F(InferTest, MixedReferenceArguments) {
  add(Evidence::UNKNOWN_REFERENCE_ARGUMENT);
  EXPECT_EQ(Nullability::UNKNOWN, infer());
  add(Evidence::NONNULL_REFERENCE_ARGUMENT);
  EXPECT_EQ(Nullability::NONNULL, infer());
  add(Evidence::NULLABLE_REFERENCE_ARGUMENT);
  EXPECT_EQ(Nullability::NULLABLE, infer(/*ExpectConflict=*/true));
}

}  // namespace
}  // namespace clang::tidy::nullability
