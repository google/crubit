// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <bit>
#include <cstdint>

#include "gtest/gtest.h"
#include "support/rs_std/enum.h"

namespace crubit {
namespace {

namespace te = crubit::internal::tag_encoding;

// -----------------------------------------------------------------------------
// Decoding
//
// `DiscriminantFromBytes` is runtime-only, so the decode tables are checked
// by tests. (`VariadicUnionStorage::discriminant()`, which applies it to a real
// enum's storage, is exercised in enum_storage_test.cc.)
// -----------------------------------------------------------------------------

// The decoders use native byte order, matching how each variant's tag member
// was written, so they are correct on any endianness. The byte patterns spelled
// out below are not -- they assume little endian.
static_assert(std::endian::native == std::endian::little,
              "The byte patterns in these tests assume little endian.");

inline constexpr unsigned char kTagBytes[8] = {0x2A, 0x00, 0x00, 0x00,
                                               0x07, 0x00, 0x00, 0x00};

// Niche: sentinel bytes map to niche variants, everything else is untagged.
// Mirrors the `Command` enum: niche variants 1..=3 encoded as bytes 2..=4.
using CommandNiche = te::Niche<uint8_t, /*Offset=*/0, /*UntaggedVariant=*/0,
                               /*NicheStartVariant=*/1, /*NicheEndVariant=*/3,
                               /*NicheStartValue=*/2>;

// A niche whose encoded values wrap past the top of the tag's range:
// variants 1..=3 encode as bytes 254, 255, 0.
using WrappingNiche = te::Niche<uint8_t, /*Offset=*/0, /*UntaggedVariant=*/0,
                                /*NicheStartVariant=*/1, /*NicheEndVariant=*/3,
                                /*NicheStartValue=*/254>;

// Decodes a single tag byte.
template <typename Encoding>
typename Encoding::Discriminant DecodeByte(unsigned char byte) {
  const unsigned char storage[1] = {byte};
  return Encoding::DiscriminantFromBytes(storage);
}

TEST(TagEncodingTest, SingleReadsNothing) {
  // The pointer is irrelevant.
  EXPECT_EQ((te::Single<uint32_t, 0>::DiscriminantFromBytes(nullptr)), 0u);
}

TEST(TagEncodingTest, DirectHonorsWidthAndOffset) {
  EXPECT_EQ((te::Direct<uint32_t, 0>::DiscriminantFromBytes(kTagBytes)), 42u);
  EXPECT_EQ((te::Direct<uint32_t, 4>::DiscriminantFromBytes(kTagBytes)), 7u);
  EXPECT_EQ((te::Direct<uint8_t, 0>::DiscriminantFromBytes(kTagBytes)), 42u);
  EXPECT_EQ((te::Direct<uint64_t, 0>::DiscriminantFromBytes(kTagBytes)),
            0x000000070000002Aull);
}

TEST(TagEncodingTest, DirectSignedDiscriminant) {
  EXPECT_EQ((DecodeByte<te::Direct<int8_t, 0>>(0xFF)), -1);
}

TEST(TagEncodingTest, NicheDecodeTable) {
  EXPECT_EQ(DecodeByte<CommandNiche>(2), 1);
  EXPECT_EQ(DecodeByte<CommandNiche>(4), 3);
  // Below the niche range: the wrapping subtraction must not alias into it.
  EXPECT_EQ(DecodeByte<CommandNiche>(0), 0);
  EXPECT_EQ(DecodeByte<CommandNiche>(1), 0);
  // Above the niche range.
  EXPECT_EQ(DecodeByte<CommandNiche>(5), 0);
  EXPECT_EQ(DecodeByte<CommandNiche>(255), 0);
}

TEST(TagEncodingTest, WrappingNicheDecodeTable) {
  EXPECT_EQ(DecodeByte<WrappingNiche>(255), 2);
  EXPECT_EQ(DecodeByte<WrappingNiche>(0), 3);
  EXPECT_EQ(DecodeByte<WrappingNiche>(1), 0);
}

// `kTagFootprint` is one past the last byte index the encoding reads.
// `VariadicUnionStorage` requires every variant to be at least this wide.
static_assert(te::Single<uint32_t, 0>::kTagFootprint == 0);
static_assert(te::Direct<uint8_t, 0>::kTagFootprint == 1);
static_assert(te::Direct<uint32_t, 0>::kTagFootprint == 4);
static_assert(te::Direct<uint32_t, 4>::kTagFootprint == 8);
static_assert(te::Direct<uint64_t, 8>::kTagFootprint == 16);
// A niche reads exactly where its field is, so it inherits Direct's footprint.
static_assert(CommandNiche::kTagFootprint == 1);

// Every encoding satisfies the concept `VariadicUnionStorage` is constrained
// on.
static_assert(crubit::internal::RustcTagEncoding<te::Single<uint32_t, 0>>);
static_assert(crubit::internal::RustcTagEncoding<te::Direct<int8_t, 4>>);
static_assert(crubit::internal::RustcTagEncoding<CommandNiche>);

// `kTagFootprint` must be a `size_t`, not merely something convertible to one.
struct IntFootprint {
  using Discriminant = uint8_t;
  static constexpr int kTagFootprint = 1;
  static uint8_t DiscriminantFromBytes(const unsigned char*) { return 0; }
};
struct SizeTFootprint {
  using Discriminant = uint8_t;
  static constexpr size_t kTagFootprint = 1;
  static uint8_t DiscriminantFromBytes(const unsigned char*) { return 0; }
};
static_assert(!crubit::internal::RustcTagEncoding<IntFootprint>);
static_assert(crubit::internal::RustcTagEncoding<SizeTFootprint>);

// -----------------------------------------------------------------------------
// Runtime differential against an independent reference
// -----------------------------------------------------------------------------

// Reference implementation of niche decoding, deliberately written in the
// "absolute index" form -- walk each niche variant and ask whether the tag is
// the value that variant would encode as -- rather than the range check the
// policy uses. Agreeing on every input is then evidence about the decoding
// rule, not a restatement of one implementation.
uint8_t ReferenceNicheDecode(uint8_t tag, unsigned untagged,
                             unsigned start_variant, unsigned end_variant,
                             unsigned start_value) {
  for (unsigned v = start_variant; v <= end_variant; ++v) {
    unsigned encoded = (start_value + (v - start_variant)) & 0xFF;
    if (tag == encoded) return static_cast<uint8_t>(v);
  }
  return static_cast<uint8_t>(untagged);
}

TEST(TagEncodingTest, NicheDecodeMatchesReferenceOnEveryTagByte) {
  for (int i = 0; i <= 255; ++i) {
    const unsigned char storage[1] = {static_cast<unsigned char>(i)};
    EXPECT_EQ(CommandNiche::DiscriminantFromBytes(storage),
              ReferenceNicheDecode(static_cast<uint8_t>(i), 0, 1, 3, 2))
        << "tag byte " << i;
  }
}

TEST(TagEncodingTest, WrappingNicheDecodeMatchesReferenceOnEveryTagByte) {
  for (int i = 0; i <= 255; ++i) {
    const unsigned char storage[1] = {static_cast<unsigned char>(i)};
    EXPECT_EQ(WrappingNiche::DiscriminantFromBytes(storage),
              ReferenceNicheDecode(static_cast<uint8_t>(i), 0, 1, 3, 254))
        << "tag byte " << i;
  }
}

// A niche whose last value is exactly the top of the tag's range (253, 254,
// 255): the edge between the wrapping and non-wrapping cases.
TEST(TagEncodingTest, NicheEndingAtMaxMatchesReferenceOnEveryTagByte) {
  using EndsAtMax = te::Niche<uint8_t, /*Offset=*/0, /*UntaggedVariant=*/0,
                              /*NicheStartVariant=*/1, /*NicheEndVariant=*/3,
                              /*NicheStartValue=*/253>;
  for (int i = 0; i <= 255; ++i) {
    const unsigned char storage[1] = {static_cast<unsigned char>(i)};
    EXPECT_EQ(EndsAtMax::DiscriminantFromBytes(storage),
              ReferenceNicheDecode(static_cast<uint8_t>(i), 0, 1, 3, 253))
        << "tag byte " << i;
  }
}

TEST(TagEncodingTest, SingleIgnoresStorage) {
  unsigned char storage[4] = {0xAA, 0xBB, 0xCC, 0xDD};
  EXPECT_EQ((te::Single<uint32_t, 7>::DiscriminantFromBytes(storage)), 7u);
}

}  // namespace
}  // namespace crubit
