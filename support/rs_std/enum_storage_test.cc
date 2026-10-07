// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <cstdint>
#include <memory>
#include <string>
#include <type_traits>
#include <utility>

#include "gtest/gtest.h"
#include "support/rs_std/enum.h"

namespace {

using crubit::internal::VariadicUnion;
using crubit::internal::VariadicUnionStorage;
using crubit::internal::VariantKey;
using crubit::internal::tag_encoding::Direct;
using crubit::internal::tag_encoding::Niche;

// Like generated variants, these take a `VariantKey` first: the storage's
// in-place constructor is the only way to activate a variant, and it supplies
// one.
struct VariantA {
  static constexpr uint32_t kDiscriminant = 0;
  uint32_t tag = 0;
  int x = 0;
  VariantA(VariantKey, int x) : x(x) {}
};

struct VariantB {
  static constexpr uint32_t kDiscriminant = 1;
  uint32_t tag = 1;
  std::string s;
  VariantB(VariantKey, std::string s) : s(std::move(s)) {}
};

struct VariantC {
  static constexpr uint32_t kDiscriminant = 2;
  uint32_t tag = 2;
  double d = 0.0;
  VariantC(VariantKey, double d) : d(d) {}
};

struct NotAVariant {
  static constexpr uint32_t kDiscriminant = 99;
};

struct CustomDtorOnly {
  static constexpr uint32_t kDiscriminant = 3;
  uint32_t tag = 3;
  int val = 0;
  explicit CustomDtorOnly(VariantKey) {}
  ~CustomDtorOnly() {}
  CustomDtorOnly(const CustomDtorOnly&) = default;
  CustomDtorOnly(CustomDtorOnly&&) = default;
  CustomDtorOnly& operator=(const CustomDtorOnly&) = default;
  CustomDtorOnly& operator=(CustomDtorOnly&&) = default;
};

struct TrivialMoveOnlyVariant {
  static constexpr uint32_t kDiscriminant = 4;
  uint32_t tag = 4;
  int val = 0;
  constexpr TrivialMoveOnlyVariant(VariantKey, int val) : val(val) {}
  TrivialMoveOnlyVariant(const TrivialMoveOnlyVariant&) = delete;
  TrivialMoveOnlyVariant& operator=(const TrivialMoveOnlyVariant&) = delete;
  constexpr TrivialMoveOnlyVariant(TrivialMoveOnlyVariant&&) = default;
  constexpr TrivialMoveOnlyVariant& operator=(TrivialMoveOnlyVariant&&) =
      default;
  constexpr ~TrivialMoveOnlyVariant() = default;
};

struct MoveOnlyVariant {
  static constexpr uint32_t kDiscriminant = 5;
  uint32_t tag = 5;
  std::unique_ptr<int> ptr;
};

// Destruction tally for `CountedVariant`. A test owns one of these and hands
// out a pointer, so tests share no state and none has to reset a counter.
struct Counters {
  int live = 0;
  int dtors = 0;
};

struct CountedVariant {
  static constexpr uint32_t kDiscriminant = 6;
  uint32_t tag = 6;
  Counters* counters;
  int id = 0;

  CountedVariant(VariantKey, Counters* counters, int id)
      : counters(counters), id(id) {
    ++counters->live;
  }
  ~CountedVariant() {
    --counters->live;
    ++counters->dtors;
  }
};

using TrivialTU = VariadicUnionStorage<Direct<uint32_t, 0>, VariantA, VariantC>;
using TrivialMoveOnlyTU =
    VariadicUnionStorage<Direct<uint32_t, 0>, VariantA, TrivialMoveOnlyVariant>;
using NonTrivialTU =
    VariadicUnionStorage<Direct<uint32_t, 0>, VariantA, VariantB, VariantC>;
using CustomDtorTU = VariadicUnionStorage<Direct<uint32_t, 0>, CustomDtorOnly>;
using MoveOnlyTU = VariadicUnionStorage<Direct<uint32_t, 0>, MoveOnlyVariant>;
using CountedTU = VariadicUnionStorage<Direct<uint32_t, 0>, CountedVariant>;

// Default constructible so the enclosing enum's thunk-backed constructors
// (`Default`, `Clone`, `UnsafeRelocateTag`) can default-initialize `storage_`
// before populating it via Rust or `memcpy`.
static_assert(std::is_nothrow_default_constructible_v<TrivialTU>);
static_assert(std::is_nothrow_default_constructible_v<NonTrivialTU>);
static_assert(std::is_nothrow_default_constructible_v<CustomDtorTU>);
static_assert(std::is_nothrow_default_constructible_v<MoveOnlyTU>);

static_assert(std::is_trivially_destructible_v<TrivialTU>);
static_assert(std::is_trivially_destructible_v<TrivialMoveOnlyTU>);
static_assert(!std::is_trivially_destructible_v<NonTrivialTU>);
static_assert(!std::is_trivially_destructible_v<CustomDtorTU>);
static_assert(!std::is_trivially_destructible_v<MoveOnlyTU>);

// TrivialTU special members are defaulted and trivial.
static_assert(std::is_trivially_copy_constructible_v<TrivialTU>);
static_assert(std::is_trivially_move_constructible_v<TrivialTU>);
static_assert(std::is_trivially_copy_assignable_v<TrivialTU>);
static_assert(std::is_trivially_move_assignable_v<TrivialTU>);

// TrivialMoveOnlyTU preserves trivial move while deleting copy.
static_assert(!std::is_copy_constructible_v<TrivialMoveOnlyTU>);
static_assert(!std::is_copy_assignable_v<TrivialMoveOnlyTU>);
static_assert(std::is_trivially_move_constructible_v<TrivialMoveOnlyTU>);
static_assert(std::is_trivially_move_assignable_v<TrivialMoveOnlyTU>);

// Non-trivial copy and move are delegated to Rust on the enclosing enum class,
// so VariadicUnionStorage itself is neither copyable nor movable when any
// variant is non-trivial.
static_assert(!std::is_copy_constructible_v<NonTrivialTU>);
static_assert(!std::is_move_constructible_v<NonTrivialTU>);
static_assert(!std::is_copy_assignable_v<NonTrivialTU>);
static_assert(!std::is_move_assignable_v<NonTrivialTU>);

static_assert(!std::is_copy_constructible_v<MoveOnlyTU>);
static_assert(!std::is_move_constructible_v<MoveOnlyTU>);
static_assert(!std::is_copy_assignable_v<MoveOnlyTU>);
static_assert(!std::is_move_assignable_v<MoveOnlyTU>);

// Even though `CustomDtorOnly` has defaulted copy/move assignment, its
// non-trivial destructor disqualifies trivial assignment on the storage (which
// would overwrite the active variant's bytes without destroying it).
static_assert(!std::is_copy_constructible_v<CustomDtorTU>);
static_assert(!std::is_move_constructible_v<CustomDtorTU>);
static_assert(!std::is_copy_assignable_v<CustomDtorTU>);
static_assert(!std::is_move_assignable_v<CustomDtorTU>);

// The storage accessors are constrained with `requires`, not asserted in the
// body, so asking whether a call is well-formed gives an honest answer. An
// in-body `static_assert` would report every one of these as callable -- a
// requires-expression does not instantiate the body that holds the assert --
// and only fail once the call was actually written.
//
// These must be templates: a requires-expression only turns an invalid
// expression into `false` when the invalidity comes from substituting template
// arguments ([expr.prim.req]/6).
template <typename Target, typename TU>
constexpr bool GetIsCallable = requires(TU& tu) { tu.template get<Target>(); };
template <typename Target, typename TU>
constexpr bool GetIfIsCallable =
    requires(TU& tu) { tu.template get_if<Target>(); };

static_assert(GetIsCallable<VariantA, NonTrivialTU>);
static_assert(GetIfIsCallable<VariantA, NonTrivialTU>);
static_assert(!GetIsCallable<NotAVariant, NonTrivialTU>);
static_assert(!GetIfIsCallable<NotAVariant, NonTrivialTU>);

// Membership is in *this* storage's pack, not "is a variant somewhere":
// `TrivialTU` does not carry `VariantB`.
static_assert(!GetIsCallable<VariantB, TrivialTU>);
static_assert(!GetIfIsCallable<VariantB, TrivialTU>);

// `VariadicUnion` is constrained the same way, so a non-member `Target` is
// rejected at the outermost call instead of recursing down to the empty union.
using ABUnion = VariadicUnion<VariantA, VariantB>;
static_assert(GetIsCallable<VariantA, ABUnion>);
static_assert(GetIsCallable<VariantB, ABUnion>);
static_assert(GetIsCallable<VariantB, const ABUnion>);
static_assert(!GetIsCallable<VariantC, ABUnion>);
static_assert(!GetIsCallable<VariantC, const ABUnion>);
static_assert(std::is_constructible_v<ABUnion, std::in_place_type_t<VariantB>,
                                      VariantKey, std::string>);
static_assert(!std::is_constructible_v<ABUnion, std::in_place_type_t<VariantC>,
                                       VariantKey, double>);

// True iff calling `F()` is a constant expression. Lets the negative cases
// below be passing assertions rather than build failures.
template <auto F>
concept ConstantEvaluable =
    requires { typename std::integral_constant<bool, (F(), true)>; };

// -----------------------------------------------------------------------------
// Construction in constant expressions
//
// During constant evaluation only trivially destructible variants may be
// constructed, so that an enum with drop glue can skip its Rust drop there.
// -----------------------------------------------------------------------------

struct TrivialVariant {
  static constexpr uint32_t kDiscriminant = 0;
  uint32_t tag = 0;
  constexpr explicit TrivialVariant(VariantKey) {}
};

// A literal type whose destructor is constexpr but non-trivial. This is the
// case the guard exists for: it would otherwise be constructible at compile
// time, and the enum's skipped drop would silently skip this destructor.
struct ConstexprDtorVariant {
  static constexpr uint32_t kDiscriminant = 1;
  uint32_t tag = 1;
  int x = 0;
  constexpr explicit ConstexprDtorVariant(VariantKey) {}
  constexpr ~ConstexprDtorVariant() { x = 0; }
};

using GuardTU = VariadicUnionStorage<Direct<uint32_t, 0>, TrivialVariant,
                                     ConstexprDtorVariant>;

static_assert(ConstantEvaluable<[] {
  GuardTU tu(std::in_place_type<TrivialVariant>);
  return 0;
}>);
static_assert(!ConstantEvaluable<[] {
  GuardTU tu(std::in_place_type<ConstexprDtorVariant>);
  return 0;
}>);

TEST(VariadicUnionStorageTest, NonTriviallyDestructibleVariantAtRuntime) {
  // The guard only applies during constant evaluation.
  GuardTU tu(std::in_place_type<ConstexprDtorVariant>);
  EXPECT_EQ(tu.discriminant(), ConstexprDtorVariant::kDiscriminant);
  std::destroy_at(tu.get<ConstexprDtorVariant>());
}

// -----------------------------------------------------------------------------
// Objects built at compile time
//
// Construction and `get` are constant expressions; reading the discriminant is
// runtime-only, and must decode the tag that constant evaluation wrote.
// -----------------------------------------------------------------------------

// Three variants, so construction has to recurse past the head of the union;
// the last one is the deepest.
struct ConstexprA {
  static constexpr uint8_t kDiscriminant = 0;
  uint8_t tag = 0;
  int8_t x;
  constexpr ConstexprA(VariantKey, int8_t x) : x(x) {}
};
struct ConstexprB {
  static constexpr uint8_t kDiscriminant = 1;
  uint8_t tag = 1;
  int8_t y;
  constexpr ConstexprB(VariantKey, int8_t y) : y(y) {}
};
struct ConstexprC {
  static constexpr uint8_t kDiscriminant = 2;
  uint8_t tag = 2;
  constexpr explicit ConstexprC(VariantKey) {}
};
using ConstexprTU = VariadicUnionStorage<Direct<uint8_t, 0>, ConstexprA,
                                         ConstexprB, ConstexprC>;

constexpr ConstexprTU kA(std::in_place_type<ConstexprA>, int8_t{1});
constexpr ConstexprTU kB(std::in_place_type<ConstexprB>, int8_t{2});
constexpr ConstexprTU kC(std::in_place_type<ConstexprC>);
static_assert(kA.get<ConstexprA>()->x == 1);
static_assert(kB.get<ConstexprB>()->y == 2);

// A pointer niche, as in `Option<&i32>`: `None` is the all-zero pointer and
// `Some` is the untagged variant. Building `Some` at compile time needs the
// pointer to be a real member -- it could not be `bit_cast` into bytes.
constexpr int32_t kPointee = 5;
struct NoneRef {
  static constexpr uint64_t kDiscriminant = 0;
  uint64_t null = 0;
  constexpr explicit NoneRef(VariantKey) {}
};
struct SomeRef {
  static constexpr uint64_t kDiscriminant = 1;
  const int32_t* p;
  constexpr SomeRef(VariantKey, const int32_t* p) : p(p) {}
};
using OptionRefTU = VariadicUnionStorage<
    Niche<uint64_t, /*NicheOffset=*/0, /*UntaggedVariant=*/1,
          /*NicheStartVariant=*/0, /*NicheEndVariant=*/0,
          /*NicheStartValue=*/0>,
    NoneRef, SomeRef>;

constexpr OptionRefTU kNoneRef(std::in_place_type<NoneRef>);
constexpr OptionRefTU kSomeRef(std::in_place_type<SomeRef>, &kPointee);
static_assert(*kSomeRef.get<SomeRef>()->p == 5);

TEST(VariadicUnionStorageTest, RuntimeDecodeOfCompileTimeObjects) {
  EXPECT_EQ(kA.discriminant(), 0u);
  EXPECT_EQ(kB.discriminant(), 1u);
  EXPECT_EQ(kC.discriminant(), 2u);
  ASSERT_NE(kB.get_if<ConstexprB>(), nullptr);
  EXPECT_EQ(kB.get_if<ConstexprB>()->y, 2);
  EXPECT_EQ(kB.get_if<ConstexprA>(), nullptr);
  EXPECT_EQ(kB.get_if<ConstexprC>(), nullptr);

  EXPECT_EQ(kNoneRef.discriminant(), 0u);
  EXPECT_EQ(kSomeRef.discriminant(), 1u);
  ASSERT_NE(kSomeRef.get_if<SomeRef>(), nullptr);
  EXPECT_EQ(*kSomeRef.get_if<SomeRef>()->p, 5);
  EXPECT_EQ(kSomeRef.get_if<NoneRef>(), nullptr);

  int32_t local = 9;
  OptionRefTU some(std::in_place_type<SomeRef>, &local);
  EXPECT_EQ(some.discriminant(), 1u);
  ASSERT_NE(some.get_if<SomeRef>(), nullptr);
  EXPECT_EQ(*some.get_if<SomeRef>()->p, 9);
}

TEST(VariadicUnionTest, RecursiveGetConstAndMutable) {
  // `VariadicUnion` knows nothing about tags or `VariantKey`, so plain structs
  // are enough to exercise it.
  struct PlainA {
    int x;
  };
  struct PlainB {
    std::string s;
  };
  struct PlainC {
    double d;
  };
  using UnionType = VariadicUnion<PlainA, PlainB, PlainC>;
  UnionType u;

  new (&u.vs.v) PlainB{.s = "hello"};

  // Non-const get
  PlainB* b_ptr = u.get<PlainB>();
  ASSERT_NE(b_ptr, nullptr);
  EXPECT_EQ(b_ptr->s, "hello");

  b_ptr->s = "world";

  // Const get
  const UnionType& const_u = u;
  const PlainB* const_b_ptr = const_u.get<PlainB>();
  ASSERT_NE(const_b_ptr, nullptr);
  EXPECT_EQ(const_b_ptr->s, "world");

  // Clean up
  b_ptr->~PlainB();
}

TEST(VariadicUnionStorageTest, ApproachBGetIfMatchesActiveTag) {
  NonTrivialTU tu(std::in_place_type<VariantB>, "crubit");

  // When querying active variant, returns non-null pointer
  VariantB* b_ptr = tu.get_if<VariantB>();
  ASSERT_NE(b_ptr, nullptr);
  EXPECT_EQ(b_ptr->s, "crubit");

  // Inactive variants return nullptr
  EXPECT_EQ(tu.get_if<VariantA>(), nullptr);
  EXPECT_EQ(tu.get_if<VariantC>(), nullptr);

  // Const get_if
  const NonTrivialTU& const_tu = tu;
  const VariantB* const_b = const_tu.get_if<VariantB>();
  ASSERT_NE(const_b, nullptr);
  EXPECT_EQ(const_b->s, "crubit");
  EXPECT_EQ(const_tu.get_if<VariantA>(), nullptr);
  EXPECT_EQ(const_tu.get_if<VariantC>(), nullptr);

  // `~VariadicUnionStorage()` is a no-op for non-trivially destructible
  // variants (in real bindings, the enclosing enum's `~E()` calls the Rust
  // drop thunk), so destroy the active `VariantB` explicitly here.
  std::destroy_at(b_ptr);
}

TEST(VariadicUnionStorageTest, FirstAndLastVariants) {
  TrivialTU tu_a(std::in_place_type<VariantA>, 123);
  EXPECT_NE(tu_a.get_if<VariantA>(), nullptr);
  EXPECT_EQ(tu_a.get_if<VariantA>()->x, 123);
  EXPECT_EQ(tu_a.get_if<VariantC>(), nullptr);

  TrivialTU tu_c(std::in_place_type<VariantC>, 3.14);
  EXPECT_EQ(tu_c.get_if<VariantA>(), nullptr);
  EXPECT_NE(tu_c.get_if<VariantC>(), nullptr);
  EXPECT_DOUBLE_EQ(tu_c.get_if<VariantC>()->d, 3.14);
}

TEST(VariadicUnionStorageTest, TrivialCopyAndMove) {
  TrivialTU tu1(std::in_place_type<VariantC>, 2.5);
  TrivialTU tu2 = tu1;
  ASSERT_NE(tu2.get_if<VariantC>(), nullptr);
  EXPECT_DOUBLE_EQ(tu2.get_if<VariantC>()->d, 2.5);

  TrivialTU tu3(std::in_place_type<VariantA>, 42);
  tu3 = tu2;
  EXPECT_EQ(tu3.get_if<VariantA>(), nullptr);
  ASSERT_NE(tu3.get_if<VariantC>(), nullptr);
  EXPECT_DOUBLE_EQ(tu3.get_if<VariantC>()->d, 2.5);

  TrivialMoveOnlyTU m1(std::in_place_type<TrivialMoveOnlyVariant>, 99);
  TrivialMoveOnlyTU m2 = std::move(m1);
  ASSERT_NE(m2.get_if<TrivialMoveOnlyVariant>(), nullptr);
  EXPECT_EQ(m2.get_if<TrivialMoveOnlyVariant>()->val, 99);

  TrivialMoveOnlyTU m3(std::in_place_type<VariantA>, 7);
  m3 = std::move(m2);
  EXPECT_EQ(m3.get_if<VariantA>(), nullptr);
  ASSERT_NE(m3.get_if<TrivialMoveOnlyVariant>(), nullptr);
  EXPECT_EQ(m3.get_if<TrivialMoveOnlyVariant>()->val, 99);
}

TEST(VariadicUnionStorageTest,
     NoOpDestructorLeavesNonTrivialCleanupToOuterEnum) {
  Counters counters;
  {
    CountedTU tu(std::in_place_type<CountedVariant>, &counters, 1);
    EXPECT_EQ(counters.live, 1);
    EXPECT_EQ(counters.dtors, 0);

    // Simulate the enclosing enum's `~E()` running its Rust drop thunk.
    std::destroy_at(tu.get<CountedVariant>());
    EXPECT_EQ(counters.live, 0);
    EXPECT_EQ(counters.dtors, 1);
  }
  // `~VariadicUnionStorage()` ran at scope exit and must not have destroyed
  // `CountedVariant` a second time.
  EXPECT_EQ(counters.live, 0);
  EXPECT_EQ(counters.dtors, 1);
}

}  // namespace
