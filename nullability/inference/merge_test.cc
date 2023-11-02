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

TEST(MergeEvidenceTest, PartialFromEvidence) {
  EXPECT_THAT(partialFromEvidence(proto<Evidence>(R"pb(
                symbol { usr: "func" }
                slot: 1
                kind: UNCHECKED_DEREFERENCE
                location: "foo.cc:42"
              )pb")),
              EqualsProto(R"pb(
                symbol { usr: "func" }
                slot {}
                slot {
                  kind_count { key: 3 value: 1 }
                  kind_samples {
                    key: 3
                    value { location: "foo.cc:42" }
                  }
                }
              )pb"));
}

TEST(MergeEvidenceTest, MergePartials) {
  auto L = proto<Partial>(R"pb(
    symbol { usr: "func" }
    slot {
      kind_count { key: 1 value: 1 }
      kind_count { key: 0 value: 2 }
      kind_samples {
        key: 0
        value { location: "a" location: "b" }
      }
    }
    slot { kind_count { key: 0 value: 1 } }
  )pb");

  auto R = proto<Partial>(R"pb(
    symbol { usr: "func" }
    slot {
      kind_count { key: 2 value: 1 }
      kind_count { key: 0 value: 2 }
      kind_samples {
        key: 0
        value { location: "c" location: "a" location: "d" }
      }
    }
    slot {}
    slot { kind_count { key: 3 value: 1 } }
  )pb");

  mergePartials(L, R);
  EXPECT_THAT(L, EqualsProto(R"pb(
                symbol { usr: "func" }
                slot {
                  kind_count { key: 0 value: 4 }
                  kind_count { key: 2 value: 1 }
                  kind_count { key: 1 value: 1 }
                  kind_samples {
                    key: 0
                    value { location: "a" location: "b" location: "c" }
                  }
                }
                slot { kind_count { key: 0 value: 1 } }
                slot { kind_count { key: 3 value: 1 } }
              )pb"));
}

TEST(MergeEvidenceTest, Finalize) {
  EXPECT_THAT(finalize(proto<Partial>(R"pb(
                symbol { usr: "func" }
                slot {
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
                }
                slot {}
                slot {
                  kind_count { key: 3 value: 1 }  # UNCHECKED_DEREFERENCE
                }
                slot { kind_count { key: 0 value: 1 } }  # ANNOTATED_UNKNOWN
                slot { kind_count { key: 1 value: 1 } }  # ANNOTATED_NULLABLE
              )pb")),
              EqualsProto(R"pb(
                symbol { usr: "func" }
                slot_inference {
                  slot: 0
                  nullability: UNKNOWN
                  conflict: true
                  sample_evidence { kind: ANNOTATED_NULLABLE location: "decl" }
                  sample_evidence { kind: ANNOTATED_NONNULL location: "def" }
                }
                slot_inference { slot: 2 nullability: NONNULL }
                slot_inference { slot: 3 nullability: UNKNOWN }
                slot_inference { slot: 4 nullability: NULLABLE trivial: true }
              )pb"));
}

TEST(MergeEvidenceTest, TreeShapedMerge) {
  Partial P1 =
      partialFromEvidence(proto<Evidence>("slot: 0 "
                                          "kind: ANNOTATED_NULLABLE"));
  Partial P2 =
      partialFromEvidence(proto<Evidence>("slot: 2 "
                                          "kind: UNCHECKED_DEREFERENCE"));
  Partial P3 =
      partialFromEvidence(proto<Evidence>("slot: 4 "
                                          "kind: ANNOTATED_NULLABLE"));
  Partial P4 =
      partialFromEvidence(proto<Evidence>("slot: 0 "
                                          "kind: ANNOTATED_NONNULL"));

  auto IsExpected = EqualsProto(R"pb(
    symbol {}
    slot_inference { slot: 0 nullability: UNKNOWN conflict: true }
    slot_inference { slot: 2 nullability: NONNULL }
    slot_inference { slot: 4 nullability: NULLABLE trivial: true }
  )pb");

  EXPECT_THAT(
      [&] {
        Partial P = P1;
        mergePartials(P, P2);
        mergePartials(P, P3);
        mergePartials(P, P4);
        return finalize(P);
      }(),
      IsExpected);

  EXPECT_THAT(
      [&] {
        Partial P = P4;
        mergePartials(P, P3);
        mergePartials(P, P2);
        mergePartials(P, P1);
        return finalize(P);
      }(),
      IsExpected);

  EXPECT_THAT(
      [&] {
        Partial PA = P1;
        mergePartials(PA, P2);
        Partial PB = P3;
        mergePartials(PB, P4);
        mergePartials(PA, PB);
        return finalize(PA);
      }(),
      IsExpected);
}

class InferTest : public ::testing::Test {
  std::array<unsigned, Evidence::Kind_MAX + 1> Counts = {};

 protected:
  void add(Evidence::Kind E, int N = 1) { Counts[E] += N; }

  Inference::Nullability infer(bool ExpectConflict = false,
                               bool ExpectTrivial = false) {
    auto Result = nullability::infer(Counts);
    EXPECT_EQ(ExpectConflict, Result.Conflict);
    EXPECT_EQ(ExpectTrivial, Result.Trivial);
    return Result.Nullability;
  }
};

TEST_F(InferTest, NoEvidence) { EXPECT_EQ(Inference::UNKNOWN, infer()); }

TEST_F(InferTest, Annotated) {
  add(Evidence::ANNOTATED_NULLABLE);
  EXPECT_EQ(Inference::NULLABLE,
            infer(/*ExpectConflict=*/false, /*ExpectTrivial=*/true));
  add(Evidence::UNCHECKED_DEREFERENCE);  // No conflict, annotation wins.
  EXPECT_EQ(Inference::NULLABLE,
            infer(/*ExpectConflict=*/false, /*ExpectTrivial=*/true));
  add(Evidence::ANNOTATED_NONNULL);  // Conflicting annotations!
  EXPECT_EQ(Inference::UNKNOWN, infer(/*ExpectConflict=*/true));
}

TEST_F(InferTest, Deref) {
  add(Evidence::UNCHECKED_DEREFERENCE);
  EXPECT_EQ(Inference::NONNULL, infer());
}

TEST_F(InferTest, NullableArgumentPassed) {
  add(Evidence::NULLABLE_ARGUMENT);
  EXPECT_EQ(Inference::NULLABLE, infer());
  add(Evidence::NONNULL_ARGUMENT);
  EXPECT_EQ(Inference::NULLABLE, infer());
  add(Evidence::UNKNOWN_ARGUMENT);
  EXPECT_EQ(Inference::NULLABLE, infer());
  add(Evidence::UNCHECKED_DEREFERENCE);
  EXPECT_EQ(Inference::NONNULL, infer(/*ExpectConflict=*/true));
}

TEST_F(InferTest, OnlyNonnullArgumentsPassed) {
  add(Evidence::NONNULL_ARGUMENT);
  EXPECT_EQ(Inference::NONNULL, infer());
}

TEST_F(InferTest, NonnullAndUnknownArgumentsPassed) {
  add(Evidence::NONNULL_ARGUMENT);
  add(Evidence::UNKNOWN_ARGUMENT);
  EXPECT_EQ(Inference::UNKNOWN, infer());
}

TEST_F(InferTest, ReturnValues) {
  add(Evidence::NONNULL_RETURN);
  EXPECT_EQ(Inference::NONNULL, infer());
  add(Evidence::UNKNOWN_RETURN);
  EXPECT_EQ(Inference::UNKNOWN, infer());
  add(Evidence::NULLABLE_RETURN);
  EXPECT_EQ(Inference::NULLABLE, infer());
}

TEST_F(InferTest, PassedToNonnull) {
  add(Evidence::BOUND_TO_NONNULL);
  EXPECT_EQ(Inference::NONNULL, infer());
}

}  // namespace
}  // namespace clang::tidy::nullability
