// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "support/rs_std/enum.h"

#include <concepts>
#include <cstddef>
#include <cstdint>
#include <cstring>
#include <type_traits>
#include <utility>

#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "absl/base/attributes.h"
#include "support/rs_std/enum_test_option_i32.h"

namespace crubit {
namespace {

using Option = ::crubit::enum_test::Option<int32_t>;
using crubit::internal::VariantKey;
using ::testing::AllOf;
using ::testing::Field;
using ::testing::Pointee;

// The raw values behind the opaque `rs::Discriminant`. Callers can't see these;
// the tests use them to check that the generated tag values are written and
// decoded correctly.
template <typename E>
auto RawDiscriminant(const E& e) {
  return crubit::internal::EnumAccess::unwrap_discriminant(rs::discriminant(e));
}
template <typename V>
constexpr auto RawDiscriminantOf() {
  return crubit::internal::EnumAccess::variant_discriminant<V>();
}

// The block every generated variant carries (spelled out in
// enum_test_option_i32.h): copy and move defaulted but private, usable only by
// the union that holds the variant. Leaves the variant's access at `public`.
#define ENUM_API_TEST_PRIVATE_COPY_AND_MOVE(V)    \
 private:                                         \
  template <typename...>                          \
  friend union ::crubit::internal::VariadicUnion; \
  V(const V&) = default;                          \
  V(V&&) = default;                               \
  V& operator=(const V&) = default;               \
  V& operator=(V&&) = default;                    \
                                                  \
 public:

struct RandomClass {};

static_assert(crubit::internal::VariantOfEnum<Option::Some, Option>);
static_assert(crubit::internal::VariantOfEnum<Option::None, Option>);
static_assert(!crubit::internal::VariantOfEnum<int, Option>);
static_assert(!crubit::internal::VariantOfEnum<RandomClass, Option>);

static_assert(
    std::derived_from<Option::Some, crubit::internal::VariantOf<Option>>);
static_assert(
    std::derived_from<Option::None, crubit::internal::VariantOf<Option>>);
static_assert(
    std::derived_from<Option::Some,
                      crubit::internal::Variant<Option::Some, Option, 1>>);
static_assert(
    std::derived_from<Option::None,
                      crubit::internal::Variant<Option::None, Option, 0>>);
static_assert(!std::derived_from<int, crubit::internal::VariantOf<Option>>);
static_assert(
    !std::derived_from<RandomClass, crubit::internal::VariantOf<Option>>);
// A variant of some *other* enum: structurally a variant, wrong enum. This is
// the realistic caller mistake, and `RandomClass` alone would not catch it.
struct OtherEnum {
  using Discriminant = uint32_t;
};
struct OtherVariant : crubit::internal::Variant<OtherVariant, OtherEnum, 0> {};
static_assert(crubit::internal::VariantOfEnum<OtherVariant, OtherEnum>);

// The public API is constrained with `requires`, so "is this call valid?" has
// an honest answer. With an in-body `static_assert` every one of the negative
// cases below would report `true` and only fail once the call was written.
//
// These must be templates: a requires-expression only turns an invalid
// expression into `false` when the invalidity comes from substituting template
// arguments ([expr.prim.req]/6).
template <typename V, typename E>
constexpr bool GetIfIsCallable = requires(E* p) { rs::get_if<V>(p); };
template <typename V, typename E>
constexpr bool GetUncheckedIsCallable =
    requires(E* p) { rs::get_unchecked<V>(p); };
template <typename V, typename E>
constexpr bool HoldsAlternativeIsCallable =
    requires(const E& e) { rs::holds_alternative<V>(e); };

static_assert(GetIfIsCallable<Option::Some, Option>);
static_assert(GetIfIsCallable<Option::None, Option>);
static_assert(GetUncheckedIsCallable<Option::Some, Option>);
static_assert(HoldsAlternativeIsCallable<Option::Some, Option>);

static_assert(!GetIfIsCallable<OtherVariant, Option>);
static_assert(!GetUncheckedIsCallable<OtherVariant, Option>);
static_assert(!HoldsAlternativeIsCallable<OtherVariant, Option>);
static_assert(!GetIfIsCallable<RandomClass, Option>);
static_assert(!GetIfIsCallable<int, Option>);
// The enum itself is not one of its own variants.
static_assert(!GetIfIsCallable<Option, Option>);

// Constness still selects the right member of the constrained overload pair.
static_assert(
    std::is_same_v<decltype(rs::get_if<Option::Some>(std::declval<Option*>())),
                   Option::Some*>);
static_assert(std::is_same_v<
              decltype(rs::get_if<Option::Some>(std::declval<const Option*>())),
              const Option::Some*>);
static_assert(!std::is_default_constructible_v<Option>);
static_assert(std::is_destructible_v<Option>);

// Variants can only be constructed in place inside their enum.
// `Option::Some(7)` would produce a variant while reading exactly like the Rust
// expression that produces an `Option`, so it must not compile.
//
// `constructible_from` is the right probe for this because the variant
// constructors are public; what makes them uncallable is that nothing outside
// the storage can produce the `VariantKey` they take. `{}` cannot either,
// since `VariantKey` is not an aggregate and its default constructor is
// private.
static_assert(!std::constructible_from<Option::Some, int32_t>);
static_assert(!std::default_initializable<Option::None>);
static_assert(!std::default_initializable<crubit::internal::VariantKey>);

// With no way to build a variant, a constructor taking one would only serve to
// re-wrap a copy taken out of another enum. There are none, so the in-place
// constructor is the only one a generated enum has.
static_assert(!std::constructible_from<Option, Option::Some>);
static_assert(!std::constructible_from<Option, const Option::Some&>);

// Nor can a variant be copied or moved out of its enum: variants are reached
// through `get_if` / `get_unchecked` and stay inside the enum. The enum itself
// remains trivially copyable, since this Rust type is `Copy`.
static_assert(!std::is_copy_constructible_v<Option::Some>);
static_assert(!std::is_move_constructible_v<Option::Some>);
static_assert(!std::is_copy_assignable_v<Option::Some>);
static_assert(!std::is_move_assignable_v<Option::Some>);
static_assert(!std::is_copy_constructible_v<Option::None>);
static_assert(std::is_trivially_copyable_v<Option>);
static_assert(std::is_copy_constructible_v<Option>);
static_assert(std::is_copy_assignable_v<Option>);

TEST(EnumApiTest, CopyableEnumCopiesItsActiveVariant) {
  Option a = Option::Some::Make(5);
  Option b = a;
  EXPECT_EQ(rs::get_if<Option::Some>(&b)->value, 5);
  b = Option::None::Make();
  EXPECT_TRUE(rs::holds_alternative<Option::None>(b));
  b = a;
  EXPECT_EQ(rs::get_if<Option::Some>(&b)->value, 5);
}

// Enums can be built at compile time, and `rs::get_unchecked` works there when
// the active variant is already known. Reading the discriminant
// (`discriminant`, `holds_alternative`, `get_if`) is runtime-only.
constexpr Option kSeven = Option::Some::Make(7);
constexpr Option kNothing = Option::None::Make();
static_assert(rs::get_unchecked<Option::Some>(&kSeven)->value == 7);

TEST(EnumApiTest, CompileTimeEnumsInspectAtRuntime) {
  EXPECT_EQ(RawDiscriminant(kSeven), 1);
  EXPECT_EQ(RawDiscriminant(kNothing), 0);
  EXPECT_TRUE(rs::holds_alternative<Option::Some>(kSeven));
  EXPECT_FALSE(rs::holds_alternative<Option::None>(kSeven));
  EXPECT_THAT(rs::get_if<Option::Some>(&kSeven),
              Pointee(Field(&Option::Some::value, 7)));
  EXPECT_EQ(rs::get_if<Option::Some>(&kNothing), nullptr);
  EXPECT_EQ(rs::get_if<Option::Some>(static_cast<const Option*>(nullptr)),
            nullptr);
}

TEST(EnumApiTest, InPlaceConstructionSelectsThatVariant) {
  Option some(std::in_place_type<Option::Some>, 42);
  Option none(std::in_place_type<Option::None>);

  EXPECT_TRUE(rs::holds_alternative<Option::Some>(some));
  EXPECT_THAT(rs::get_if<Option::Some>(&some),
              Pointee(Field(&Option::Some::value, 42)));

  EXPECT_TRUE(rs::holds_alternative<Option::None>(none));
  EXPECT_EQ(rs::get_if<Option::Some>(&none), nullptr);
}

// `rs::Discriminant` is opaque, like Rust's `std::mem::Discriminant`: it can be
// copied and compared, but it is not an integer and does not convert to or
// from one, so callers can't depend on rustc's choice of tag type.
using OptionDiscriminant = rs::Discriminant<Option>;
static_assert(std::is_same_v<decltype(rs::discriminant(std::declval<Option>())),
                             OptionDiscriminant>);
static_assert(std::is_trivially_copyable_v<OptionDiscriminant>);
static_assert(std::equality_comparable<OptionDiscriminant>);
static_assert(!std::totally_ordered<OptionDiscriminant>);
static_assert(!std::is_convertible_v<OptionDiscriminant, uint32_t>);
static_assert(!std::is_constructible_v<uint32_t, OptionDiscriminant>);
static_assert(!std::is_constructible_v<OptionDiscriminant, uint32_t>);
static_assert(!std::is_default_constructible_v<OptionDiscriminant>);

// The raw value is not reachable from a variant either.
template <typename V>
concept HasPublicKDiscriminant = requires { V::kDiscriminant; };
static_assert(!HasPublicKDiscriminant<Option::Some>);

TEST(EnumApiTest, DiscriminantIsOpaqueButComparable) {
  Option a = Option::Some::Make(42);
  Option b = Option::None::Make();
  Option c = Option::Some::Make(7);

  // Like Rust's, it identifies the variant only, not the payload.
  EXPECT_EQ(rs::discriminant(a), rs::discriminant(c));
  EXPECT_NE(rs::discriminant(a), rs::discriminant(b));
}

TEST(EnumApiTest, HoldsAlternative) {
  Option a = Option::Some::Make(42);
  Option b = Option::None::Make();

  EXPECT_TRUE(rs::holds_alternative<Option::Some>(a));
  EXPECT_FALSE(rs::holds_alternative<Option::None>(a));
  EXPECT_TRUE(rs::holds_alternative<Option::None>(b));
  EXPECT_FALSE(rs::holds_alternative<Option::Some>(b));
}

TEST(EnumApiTest, AssignFromTemporaries) {
  Option a = Option::None::Make();

  a = Option::Some::Make(88);
  EXPECT_TRUE(rs::holds_alternative<Option::Some>(a));
  EXPECT_EQ(rs::get_if<Option::Some>(&a)->value, 88);

  a = Option::None::Make();
  EXPECT_TRUE(rs::holds_alternative<Option::None>(a));
  EXPECT_FALSE(rs::holds_alternative<Option::Some>(a));
}

TEST(EnumApiTest, GetIfMutableAndConst) {
  Option a = Option::Some::Make(42);
  const Option b = Option::Some::Make(100);

  auto* a_ptr = rs::get_if<Option::Some>(&a);
  ASSERT_THAT(a_ptr, Pointee(Field(&Option::Some::value, 42)));

  // Mutate through pointer
  a_ptr->value = 99;
  EXPECT_THAT(rs::get_if<Option::Some>(&a),
              Pointee(Field(&Option::Some::value, 99)));

  // Const pointer
  const auto* b_ptr = rs::get_if<Option::Some>(&b);
  EXPECT_THAT(b_ptr, Pointee(Field(&Option::Some::value, 100)));

  // Type verification: b_ptr must be const Option::Some*
  static_assert(std::is_same_v<decltype(b_ptr), const Option::Some*>);
}

// -----------------------------------------------------------------------------
// Multi-field Variant Test: Point2D { Origin, Cartesian(i32, i32) }
// -----------------------------------------------------------------------------
class Point2D {
 public:
  using Discriminant = uint32_t;

  struct Origin : public crubit::internal::Variant<Origin, Point2D, 0> {
    ENUM_API_TEST_PRIVATE_COPY_AND_MOVE(Origin)
    constexpr explicit Origin(VariantKey) {}

   private:
    uint32_t tag = 0;
  };

  struct Cartesian : public crubit::internal::Variant<Cartesian, Point2D, 1> {
    ENUM_API_TEST_PRIVATE_COPY_AND_MOVE(Cartesian)
   private:
    uint32_t tag = 1;

   public:
    int32_t x;
    int32_t y;

    constexpr Cartesian(VariantKey, int32_t x, int32_t y) : x(x), y(y) {}
  };

  template <typename V, typename... Args>
    requires crubit::internal::ConstructibleVariantOf<V, Point2D, Args...>
  constexpr explicit Point2D(std::in_place_type_t<V> v, Args&&... args)
      : storage_(v, std::forward<Args>(args)...) {}

 private:
  friend struct crubit::internal::EnumAccess;

  crubit::internal::VariadicUnionStorage<
      crubit::internal::tag_encoding::Direct<uint32_t, 0>, Origin, Cartesian>
      storage_;
};

//   type `Point2D`: 12 bytes, alignment: 4 bytes
//       discriminant: 4 bytes
//       variant `Cartesian`: 8 bytes
//           field `.0`: 4 bytes
//           field `.1`: 4 bytes
//       variant `Origin`: 0 bytes
static_assert(sizeof(Point2D) == 12);
static_assert(alignof(Point2D) == 4);
static_assert(offsetof(Point2D::Cartesian, x) == 4);
static_assert(offsetof(Point2D::Cartesian, y) == 8);

TEST(EnumApiTest, MultiFieldVariant) {
  static_assert(!std::is_default_constructible_v<Point2D>);
  static_assert(crubit::internal::VariantOfEnum<Point2D::Cartesian, Point2D>);
  static_assert(crubit::internal::VariantOfEnum<Point2D::Origin, Point2D>);

  Point2D pt = Point2D::Cartesian::Make(10, 20);
  EXPECT_TRUE(rs::holds_alternative<Point2D::Cartesian>(pt));
  EXPECT_FALSE(rs::holds_alternative<Point2D::Origin>(pt));

  auto* cartesian = rs::get_if<Point2D::Cartesian>(&pt);
  ASSERT_THAT(cartesian, Pointee(AllOf(Field(&Point2D::Cartesian::x, 10),
                                       Field(&Point2D::Cartesian::y, 20))));

  cartesian->x = 30;
  cartesian->y = 40;
  EXPECT_THAT(rs::get_if<Point2D::Cartesian>(&pt),
              Pointee(AllOf(Field(&Point2D::Cartesian::x, 30),
                            Field(&Point2D::Cartesian::y, 40))));

  const Point2D const_pt = pt;
  EXPECT_THAT(rs::get_if<Point2D::Cartesian>(&const_pt),
              Pointee(AllOf(Field(&Point2D::Cartesian::x, 30),
                            Field(&Point2D::Cartesian::y, 40))));
}

// -----------------------------------------------------------------------------
// Explicit, Non-Contiguous Discriminants
//
//   #[repr(u8)]
//   enum Status { Ok = 10, Error(u32) = 20 }
//
// Every other example here has discriminants 0, 1, 2..., which makes the
// discriminant value and the variant index indistinguishable. This one
// separates them: `kDiscriminant` must be 10 and 20, *not* 0 and 1.
//
// Layout taken from rustc, not derived by hand
// (`rustc -Zprint-type-sizes`, beta toolchain):
//
//   type `Status`: 8 bytes, alignment: 4 bytes
//       discriminant: 1 bytes
//       variant `Error`: 7 bytes
//           padding: 3 bytes
//           field `.0`: 4 bytes, alignment: 4 bytes
//       variant `Ok`: 0 bytes
//
// Note the 1-byte tag. `repr(int)` opts out of tag widening; the same shape
// written as `enum { A, B(u32) }` under repr(Rust) gets a *4-byte* tag, because
// rustc widens the tag to the alignment of the payload. That is exactly why
// `Direct` is parameterized by the discriminant type rather than deducing one.
// -----------------------------------------------------------------------------
class Status {
 public:
  using Discriminant = uint8_t;

  struct Ok : public crubit::internal::Variant<Ok, Status, 10> {
    ENUM_API_TEST_PRIVATE_COPY_AND_MOVE(Ok)
    constexpr explicit Ok(VariantKey) {}

   private:
    uint8_t tag = 10;
  };

  struct Error : public crubit::internal::Variant<Error, Status, 20> {
    ENUM_API_TEST_PRIVATE_COPY_AND_MOVE(Error)
   private:
    uint8_t tag = 20;

   public:
    // rustc puts this at offset 4: one tag byte, then 3 bytes of padding.
    uint32_t code;

    constexpr explicit Error(VariantKey, uint32_t code) : code(code) {}
  };

  template <typename V, typename... Args>
    requires crubit::internal::ConstructibleVariantOf<V, Status, Args...>
  constexpr explicit Status(std::in_place_type_t<V> v, Args&&... args)
      : storage_(v, std::forward<Args>(args)...) {}

 private:
  friend struct crubit::internal::EnumAccess;

  crubit::internal::VariadicUnionStorage<
      crubit::internal::tag_encoding::Direct<uint8_t, 0>, Ok, Error>
      storage_;
};

static_assert(sizeof(Status) == 8);
static_assert(alignof(Status) == 4);
static_assert(sizeof(Status::Ok) == 1);
static_assert(sizeof(Status::Error) == 8);
static_assert(alignof(Status::Error) == 4);
static_assert(offsetof(Status::Error, code) == 4);

//   #[repr(u8)]
//   enum Code { Lo = 10, Hi = 200 }
//
// Fieldless, so each variant is exactly the tag -- the narrowest layout the
// storage's tag-coverage check permits. `Hi = 200` also has the high bit set,
// which would read back as -56 if anything in the decode path used a signed
// type.
//
//   type `Code`: 1 bytes, alignment: 1 bytes
class Code {
 public:
  using Discriminant = uint8_t;

  struct Lo : public crubit::internal::Variant<Lo, Code, 10> {
    ENUM_API_TEST_PRIVATE_COPY_AND_MOVE(Lo)
    constexpr explicit Lo(VariantKey) {}

   private:
    uint8_t tag = 10;
  };

  struct Hi : public crubit::internal::Variant<Hi, Code, 200> {
    ENUM_API_TEST_PRIVATE_COPY_AND_MOVE(Hi)
    constexpr explicit Hi(VariantKey) {}

   private:
    uint8_t tag = 200;
  };

  template <typename V, typename... Args>
    requires crubit::internal::ConstructibleVariantOf<V, Code, Args...>
  constexpr explicit Code(std::in_place_type_t<V> v, Args&&... args)
      : storage_(v, std::forward<Args>(args)...) {}

 private:
  friend struct crubit::internal::EnumAccess;

  crubit::internal::VariadicUnionStorage<
      crubit::internal::tag_encoding::Direct<uint8_t, 0>, Lo, Hi>
      storage_;
};

static_assert(sizeof(Code) == 1);
static_assert(alignof(Code) == 1);

TEST(EnumApiTest, ExplicitDiscriminantsAreValuesNotVariantIndices) {
  // The whole point of this enum: if the generator had emitted variant indices
  // these would be 0 and 1.
  static_assert(RawDiscriminantOf<Status::Ok>() == 10);
  static_assert(RawDiscriminantOf<Status::Error>() == 20);
  static_assert(
      std::is_same_v<decltype(RawDiscriminant(std::declval<Status>())),
                     uint8_t>);

  Status ok = Status::Ok::Make();
  Status err = Status::Error::Make(404);

  EXPECT_EQ(RawDiscriminant(ok), 10);
  EXPECT_EQ(RawDiscriminant(err), 20);

  EXPECT_TRUE(rs::holds_alternative<Status::Ok>(ok));
  EXPECT_FALSE(rs::holds_alternative<Status::Error>(ok));
  EXPECT_TRUE(rs::holds_alternative<Status::Error>(err));

  EXPECT_THAT(rs::get_if<Status::Error>(&err),
              Pointee(Field(&Status::Error::code, 404)));
  EXPECT_EQ(rs::get_if<Status::Error>(&ok), nullptr);
}

TEST(EnumApiTest, HighBitDiscriminantIsUnsigned) {
  static_assert(RawDiscriminantOf<Code::Hi>() == 200);

  Code hi = Code::Hi::Make();
  EXPECT_EQ(RawDiscriminant(hi), 200);
  EXPECT_TRUE(rs::holds_alternative<Code::Hi>(hi));
  EXPECT_FALSE(rs::holds_alternative<Code::Lo>(hi));

  Code lo = Code::Lo::Make();
  EXPECT_EQ(RawDiscriminant(lo), 10);
  EXPECT_TRUE(rs::holds_alternative<Code::Lo>(lo));
}

TEST(EnumApiTest, TagMatchingNoVariantIsInertRatherThanUb) {
  // Non-contiguous discriminants leave gaps, so a corrupt or truncated value
  // can name no variant at all. rustc treats that as UB on the Rust side; this
  // API degrades to "matches nothing" instead, which leaks rather than crashes.
  // Pinned here so the behaviour is a decision rather than an accident.
  //
  // `Status` is trivially copyable, so writing its object representation is
  // well-defined; 15 lies in the gap between `Ok` (10) and `Error` (20).
  Status s = Status::Ok::Make();
  const uint8_t gap_tag = 15;
  std::memcpy(&s, &gap_tag, sizeof(gap_tag));

  EXPECT_EQ(RawDiscriminant(s), 15);
  EXPECT_FALSE(rs::holds_alternative<Status::Ok>(s));
  EXPECT_FALSE(rs::holds_alternative<Status::Error>(s));
  EXPECT_EQ(rs::get_if<Status::Ok>(&s), nullptr);
  EXPECT_EQ(rs::get_if<Status::Error>(&s), nullptr);
}

// -----------------------------------------------------------------------------
// Non-trivial Drop and Move Test
// -----------------------------------------------------------------------------
struct TrackedDrop {
  int* drop_count = nullptr;
  int value = 0;

  explicit TrackedDrop(int val, int* counter = nullptr)
      : drop_count(counter), value(val) {}
  ~TrackedDrop() {
    if (drop_count != nullptr) {
      ++(*drop_count);
    }
  }
  TrackedDrop(TrackedDrop&& o) noexcept
      : drop_count(std::exchange(o.drop_count, nullptr)), value(o.value) {}
  TrackedDrop& operator=(TrackedDrop&& o) noexcept {
    if (this != &o) {
      if (drop_count != nullptr) ++(*drop_count);
      drop_count = std::exchange(o.drop_count, nullptr);
      value = o.value;
    }
    return *this;
  }
  TrackedDrop(const TrackedDrop&) = delete;
  TrackedDrop& operator=(const TrackedDrop&) = delete;
};

// Stands in for the Rust drop thunk that generated code calls from `~E()`.
// Rust's drop glue picks the active variant itself, so the destructor below is
// a single call with no per-variant dispatch -- the shape codegen emits.
class DropOption;
void FakeRustDrop(DropOption* e);

class DropOption {
 public:
  using Discriminant = uint32_t;

  struct None : public crubit::internal::Variant<None, DropOption, 0> {
    ENUM_API_TEST_PRIVATE_COPY_AND_MOVE(None)
    constexpr explicit None(VariantKey) {}

   private:
    uint32_t tag = 0;
  };

  struct Some : public crubit::internal::Variant<Some, DropOption, 1> {
    ENUM_API_TEST_PRIVATE_COPY_AND_MOVE(Some)
   private:
    uint32_t tag = 1;

   public:
    TrackedDrop value;
    explicit Some(VariantKey, int val, int* counter = nullptr)
        : value(val, counter) {}
  };

  // Simulates `Default::default` (putting `this` in `None`) so Crubit's
  // generated move constructor `DropOption(DropOption&& other) : DropOption()
  // { *this = std::move(other); }` leaves `other` as `None`.
  DropOption() : DropOption(std::in_place_type<None>) {}

  template <typename V, typename... Args>
    requires crubit::internal::ConstructibleVariantOf<V, DropOption, Args...>
  constexpr explicit DropOption(std::in_place_type_t<V> v, Args&&... args)
      : storage_(v, std::forward<Args>(args)...) {}

  // Simulates Crubit's generated `~E()` calling the Rust drop thunk.
  // Afterward `storage_.~VariadicUnionStorage()` runs automatically and must
  // be a no-op so the payload is not dropped a second time.
  //
  // The drop is skipped during constant evaluation, as generated code would
  // skip its call into Rust. That is what lets `None` be a constexpr variable;
  // the storage refuses to build `Some` there, so nothing is ever skipped.
  constexpr ~DropOption() {
    if (!std::is_constant_evaluated()) {
      FakeRustDrop(this);
    }
  }

  // Simulates Crubit's generated move constructor and `MemSwap` move
  // assignment.
  DropOption(DropOption&& other) noexcept : DropOption() {
    *this = std::move(other);
  }
  DropOption& operator=(DropOption&& other) noexcept {
    if (this == &other) return *this;
    char tmp[sizeof(DropOption)];
    std::memcpy(tmp, this, sizeof(*this));
    std::memcpy(this, &other, sizeof(*this));
    std::memcpy(&other, tmp, sizeof(*this));
    return *this;
  }

  DropOption(const DropOption&) = delete;
  DropOption& operator=(const DropOption&) = delete;

 private:
  friend struct crubit::internal::EnumAccess;

  crubit::internal::VariadicUnionStorage<
      crubit::internal::tag_encoding::Direct<uint32_t, 0>, None, Some>
      storage_;
};

// What Rust's drop glue does for `Option<TrackedDrop>`: drop the payload if
// there is one.
void FakeRustDrop(DropOption* e) {
  if (auto* some = rs::get_if<DropOption::Some>(e)) {
    std::destroy_at(some);
  }
}

// Not `Copy`, but `Default`: the enum moves (by swapping with a fresh
// `Default` from Rust), while its variants still cannot be moved or copied out
// on their own -- even though `TrackedDrop` itself is movable.
static_assert(std::is_move_constructible_v<DropOption>);
static_assert(std::is_move_assignable_v<DropOption>);
static_assert(!std::is_copy_constructible_v<DropOption>);
static_assert(!std::is_trivially_copyable_v<DropOption>);
static_assert(std::is_move_constructible_v<TrackedDrop>);
static_assert(!std::is_move_constructible_v<DropOption::Some>);
static_assert(!std::is_move_assignable_v<DropOption::Some>);
static_assert(!std::is_copy_constructible_v<DropOption::Some>);

TEST(EnumApiTest, NonTrivialDestructorOnScopeExit) {
  int drop_count = 0;
  {
    DropOption opt = DropOption::Some::Make(42, &drop_count);
    EXPECT_EQ(drop_count, 0);
  }
  EXPECT_EQ(drop_count, 1);
}

TEST(EnumApiTest, NonTrivialNoneHasZeroDrops) {
  int drop_count = 0;
  {
    DropOption opt = DropOption::None::Make();
    EXPECT_EQ(drop_count, 0);
  }
  EXPECT_EQ(drop_count, 0);
}

// `None` of an enum with drop glue can be a constexpr variable: it is
// trivially destructible, so skipping the drop at compile time loses nothing.
// (`Some` cannot be; see `VariadicUnionStorage`'s in-place constructor, and
// the storage-level tests in enum_storage_test.cc.)
constexpr DropOption kDropNone = DropOption::None::Make();

TEST(EnumApiTest, NonTrivialConstexprNone) {
  EXPECT_TRUE(rs::holds_alternative<DropOption::None>(kDropNone));
}

TEST(EnumApiTest, MoveAssignmentDropsPreviousVariant) {
  int drop1 = 0;
  int drop2 = 0;
  {
    DropOption opt = DropOption::Some::Make(1, &drop1);
    EXPECT_EQ(drop1, 0);

    opt = DropOption::Some::Make(2, &drop2);
    EXPECT_EQ(drop1, 1);
    EXPECT_EQ(drop2, 0);

    opt = DropOption::None::Make();
    EXPECT_EQ(drop1, 1);
    EXPECT_EQ(drop2, 1);
  }
  EXPECT_EQ(drop1, 1);
  EXPECT_EQ(drop2, 1);
}

TEST(EnumApiTest, NonTrivialMoveConstructor) {
  int drop_count = 0;
  {
    DropOption opt1 = DropOption::Some::Make(99, &drop_count);
    EXPECT_EQ(drop_count, 0);

    DropOption opt2 = std::move(opt1);
    EXPECT_TRUE(rs::holds_alternative<DropOption::Some>(opt2));
    EXPECT_EQ(drop_count, 0);

    EXPECT_THAT(rs::get_if<DropOption::Some>(&opt2),
                Pointee(Field(&DropOption::Some::value,
                              Field(&TrackedDrop::value, 99))));
  }
  EXPECT_EQ(drop_count, 1);
}

// -----------------------------------------------------------------------------
// In-place Construction Test: MoveCounted { Empty, Payload(i32) }
//
// The `std::in_place_type` constructor forwards its arguments to the variant's
// constructor, which runs on the storage bytes directly. Variants cannot be
// moved at all (their move is private), so "no move" is guaranteed at compile
// time; this enum counts constructions to show the variant is built exactly
// once.
// -----------------------------------------------------------------------------
struct BuildCounters {
  int built = 0;
};

class MoveCounted {
 public:
  using Discriminant = uint32_t;

  struct Empty : public crubit::internal::Variant<Empty, MoveCounted, 0> {
    ENUM_API_TEST_PRIVATE_COPY_AND_MOVE(Empty)
    constexpr explicit Empty(VariantKey) {}

   private:
    uint32_t tag = 0;
  };

  struct Payload : public crubit::internal::Variant<Payload, MoveCounted, 1> {
    ENUM_API_TEST_PRIVATE_COPY_AND_MOVE(Payload)
   private:
    uint32_t tag = 1;

   public:
    BuildCounters* counters;
    int value;

    Payload(VariantKey, BuildCounters* counters, int value)
        : counters(counters), value(value) {
      ++counters->built;
    }
  };

  template <typename V, typename... Args>
    requires crubit::internal::ConstructibleVariantOf<V, MoveCounted, Args...>
  constexpr explicit MoveCounted(std::in_place_type_t<V> v, Args&&... args)
      : storage_(v, std::forward<Args>(args)...) {}

 private:
  friend struct crubit::internal::EnumAccess;

  crubit::internal::VariadicUnionStorage<
      crubit::internal::tag_encoding::Direct<uint32_t, 0>, Empty, Payload>
      storage_;
};

// The in-place constructor is constrained on its signature rather than failing
// inside its mem-init list, so it can be asked whether a call would work
// instead of hard-erroring on the ones that would not. The `typename V::Enum`
// requirement comes first so that a non-variant answers false rather than
// failing to substitute.
template <typename V, typename... Args>
constexpr bool InPlaceConstructible = requires {
  typename V::Enum;
  requires std::constructible_from<typename V::Enum, std::in_place_type_t<V>,
                                   Args...>;
};
static_assert(InPlaceConstructible<MoveCounted::Payload, BuildCounters*, int>);
static_assert(!InPlaceConstructible<MoveCounted::Payload>);
static_assert(!InPlaceConstructible<MoveCounted::Payload, const char*>);
static_assert(!InPlaceConstructible<RandomClass>);
// The key is supplied by the storage, never by the caller.
static_assert(!InPlaceConstructible<MoveCounted::Payload, VariantKey,
                                    BuildCounters*, int>);

TEST(EnumApiTest, InPlaceConstructionBuildsTheVariantWithoutMovingIt) {
  BuildCounters counters;
  MoveCounted e(std::in_place_type<MoveCounted::Payload>, &counters, 7);

  EXPECT_EQ(counters.built, 1);
  EXPECT_EQ(rs::get_if<MoveCounted::Payload>(&e)->value, 7);
}

// `Variant::Make` is the in-place constructor spelled from the variant's side.
// It returns the enum rather than the variant, and accepts exactly the
// arguments the in-place constructor does.
template <typename V, typename... Args>
constexpr bool Makeable =
    requires(Args&&... args) { V::Make(std::forward<Args>(args)...); };
static_assert(std::same_as<decltype(Option::Some::Make(7)), Option>);
static_assert(std::same_as<decltype(Option::None::Make()), Option>);
static_assert(Makeable<Option::Some, int32_t>);
static_assert(Makeable<Option::None>);
static_assert(!Makeable<Option::Some>);
static_assert(!Makeable<Option::Some, const char*>);
static_assert(!Makeable<Option::None, int32_t>);
static_assert(Makeable<MoveCounted::Payload, BuildCounters*, int>);
static_assert(!Makeable<MoveCounted::Payload, VariantKey, BuildCounters*, int>);

TEST(EnumApiTest, MakeBuildsTheVariantWithoutMovingIt) {
  BuildCounters counters;
  // The returned prvalue initializes `e` directly, so the variant is built
  // once, in `e`'s storage, and never moved.
  MoveCounted e = MoveCounted::Payload::Make(&counters, 7);

  EXPECT_EQ(counters.built, 1);
  EXPECT_EQ(rs::get_if<MoveCounted::Payload>(&e)->value, 7);
}

// -----------------------------------------------------------------------------
// Record / Struct Variant Enum Test: WebEvent { PageLoad, Click { x: i32, y:
// i32 } }
// -----------------------------------------------------------------------------
class WebEvent {
 public:
  using Discriminant = uint32_t;

  struct PageLoad : public crubit::internal::Variant<PageLoad, WebEvent, 0> {
    ENUM_API_TEST_PRIVATE_COPY_AND_MOVE(PageLoad)
    constexpr explicit PageLoad(VariantKey) {}

   private:
    uint32_t tag = 0;
  };

  struct Click : public crubit::internal::Variant<Click, WebEvent, 1> {
    ENUM_API_TEST_PRIVATE_COPY_AND_MOVE(Click)
   private:
    uint32_t tag = 1;

   public:
    int32_t x;
    int32_t y;
    constexpr Click(VariantKey, int32_t x, int32_t y) : x(x), y(y) {}
  };

  template <typename V, typename... Args>
    requires crubit::internal::ConstructibleVariantOf<V, WebEvent, Args...>
  constexpr explicit WebEvent(std::in_place_type_t<V> v, Args&&... args)
      : storage_(v, std::forward<Args>(args)...) {}

 private:
  friend struct crubit::internal::EnumAccess;

  crubit::internal::VariadicUnionStorage<
      crubit::internal::tag_encoding::Direct<uint32_t, 0>, PageLoad, Click>
      storage_;
};

// Named fields do not change the layout: `Click { x, y }` lays out exactly like
// `Cartesian(i32, i32)`.
static_assert(sizeof(WebEvent) == 12);
static_assert(alignof(WebEvent) == 4);
static_assert(offsetof(WebEvent::Click, x) == 4);
static_assert(offsetof(WebEvent::Click, y) == 8);

TEST(EnumApiTest, RecordVariantNamedFields) {
  WebEvent click_event = WebEvent::Click::Make(100, 200);
  EXPECT_TRUE(rs::holds_alternative<WebEvent::Click>(click_event));
  EXPECT_FALSE(rs::holds_alternative<WebEvent::PageLoad>(click_event));

  auto* click = rs::get_if<WebEvent::Click>(&click_event);
  ASSERT_THAT(click, Pointee(AllOf(Field(&WebEvent::Click::x, 100),
                                   Field(&WebEvent::Click::y, 200))));

  click->x = 300;
  click->y = 400;
  EXPECT_THAT(rs::get_if<WebEvent::Click>(&click_event),
              Pointee(AllOf(Field(&WebEvent::Click::x, 300),
                            Field(&WebEvent::Click::y, 400))));

  const WebEvent const_event = click_event;
  EXPECT_THAT(rs::get_if<WebEvent::Click>(&const_event),
              Pointee(AllOf(Field(&WebEvent::Click::x, 300),
                            Field(&WebEvent::Click::y, 400))));
}

// -----------------------------------------------------------------------------
// Single-Variant / Untagged Enum Test
// -----------------------------------------------------------------------------
class SingleVariant {
 public:
  using Discriminant = uint32_t;

  struct Value : public crubit::internal::Variant<Value, SingleVariant, 0> {
    ENUM_API_TEST_PRIVATE_COPY_AND_MOVE(Value)
    int32_t value;
    constexpr explicit Value(VariantKey, int32_t v) : value(v) {}
  };

  template <typename V, typename... Args>
    requires crubit::internal::ConstructibleVariantOf<V, SingleVariant, Args...>
  constexpr explicit SingleVariant(std::in_place_type_t<V> v, Args&&... args)
      : storage_(v, std::forward<Args>(args)...) {}

 private:
  friend struct crubit::internal::EnumAccess;

  crubit::internal::VariadicUnionStorage<
      crubit::internal::tag_encoding::Single<uint32_t, 0>, Value>
      storage_;
};

TEST(EnumApiTest, UntaggedSingleVariant) {
  static_assert(
      crubit::internal::VariantOfEnum<SingleVariant::Value, SingleVariant>);

  SingleVariant s = SingleVariant::Value::Make(42);
  EXPECT_TRUE(rs::holds_alternative<SingleVariant::Value>(s));
  EXPECT_EQ(rs::get_if<SingleVariant::Value>(&s)->value, 42);
  EXPECT_EQ(RawDiscriminant(s), 0);
}

// -----------------------------------------------------------------------------
// Niche-Optimized Pointer Enum Test: Option<&int32_t>
// -----------------------------------------------------------------------------
class RefOption {
 public:
  using Discriminant = uintptr_t;

  struct None : public crubit::internal::Variant<None, RefOption, 0> {
    ENUM_API_TEST_PRIVATE_COPY_AND_MOVE(None)
    constexpr explicit None(VariantKey) {}

   private:
    uintptr_t tag = 0;
  };

  struct Some : public crubit::internal::Variant<Some, RefOption, 1> {
    ENUM_API_TEST_PRIVATE_COPY_AND_MOVE(Some)
    const int32_t* value;
    constexpr explicit Some(VariantKey, const int32_t* ptr) : value(ptr) {}
  };

  template <typename V, typename... Args>
    requires crubit::internal::ConstructibleVariantOf<V, RefOption, Args...>
  constexpr explicit RefOption(std::in_place_type_t<V> v, Args&&... args)
      : storage_(v, std::forward<Args>(args)...) {}

 private:
  friend struct crubit::internal::EnumAccess;

  crubit::internal::VariadicUnionStorage<
      crubit::internal::tag_encoding::Niche<uintptr_t, /*Offset=*/0,
                                            /*UntaggedVariant=*/1,
                                            /*NicheStartVariant=*/0,
                                            /*NicheEndVariant=*/0,
                                            /*NicheStartValue=*/0>,
      None, Some>
      storage_;
};

TEST(EnumApiTest, NicheOptimizedPointerOption) {
  static_assert(sizeof(RefOption) == sizeof(const int32_t*));

  int32_t x = 42;
  RefOption some_opt = RefOption::Some::Make(&x);
  RefOption none_opt = RefOption::None::Make();

  EXPECT_TRUE(rs::holds_alternative<RefOption::Some>(some_opt));
  EXPECT_FALSE(rs::holds_alternative<RefOption::None>(some_opt));
  EXPECT_EQ(RawDiscriminant(some_opt), 1);
  EXPECT_EQ(rs::get_if<RefOption::Some>(&some_opt)->value, &x);

  EXPECT_TRUE(rs::holds_alternative<RefOption::None>(none_opt));
  EXPECT_FALSE(rs::holds_alternative<RefOption::Some>(none_opt));
  EXPECT_EQ(RawDiscriminant(none_opt), 0);
}

// -----------------------------------------------------------------------------
// Multi-Niche 1-Byte Enum Test
// -----------------------------------------------------------------------------
class Command {
 public:
  using Discriminant = uint8_t;

  struct Run : public crubit::internal::Variant<Run, Command, 0> {
    ENUM_API_TEST_PRIVATE_COPY_AND_MOVE(Run)
    bool flag;
    constexpr explicit Run(VariantKey, bool f) : flag(f) {}
  };
  struct Pause : public crubit::internal::Variant<Pause, Command, 1> {
    ENUM_API_TEST_PRIVATE_COPY_AND_MOVE(Pause)
    constexpr explicit Pause(VariantKey) {}

   private:
    uint8_t tag = 2;
  };
  struct Stop : public crubit::internal::Variant<Stop, Command, 2> {
    ENUM_API_TEST_PRIVATE_COPY_AND_MOVE(Stop)
    constexpr explicit Stop(VariantKey) {}

   private:
    uint8_t tag = 3;
  };
  struct Restart : public crubit::internal::Variant<Restart, Command, 3> {
    ENUM_API_TEST_PRIVATE_COPY_AND_MOVE(Restart)
    constexpr explicit Restart(VariantKey) {}

   private:
    uint8_t tag = 4;
  };

  template <typename V, typename... Args>
    requires crubit::internal::ConstructibleVariantOf<V, Command, Args...>
  constexpr explicit Command(std::in_place_type_t<V> v, Args&&... args)
      : storage_(v, std::forward<Args>(args)...) {}

 private:
  friend struct crubit::internal::EnumAccess;

  crubit::internal::VariadicUnionStorage<
      crubit::internal::tag_encoding::Niche<uint8_t, /*Offset=*/0,
                                            /*UntaggedVariant=*/0,
                                            /*NicheStartVariant=*/1,
                                            /*NicheEndVariant=*/3,
                                            /*NicheStartValue=*/2>,
      Run, Pause, Stop, Restart>
      storage_;
};

TEST(EnumApiTest, MultiNicheByteCommandEnum) {
  static_assert(sizeof(Command) == 1);

  Command r_false = Command::Run::Make(false);
  Command r_true = Command::Run::Make(true);
  Command pause = Command::Pause::Make();
  Command stop = Command::Stop::Make();
  Command restart = Command::Restart::Make();

  EXPECT_EQ(RawDiscriminant(r_false), 0);
  EXPECT_EQ(RawDiscriminant(r_true), 0);
  EXPECT_EQ(RawDiscriminant(pause), 1);
  EXPECT_EQ(RawDiscriminant(stop), 2);
  EXPECT_EQ(RawDiscriminant(restart), 3);

  EXPECT_TRUE(rs::holds_alternative<Command::Run>(r_false));
  EXPECT_EQ(rs::get_if<Command::Run>(&r_false)->flag, false);

  EXPECT_TRUE(rs::holds_alternative<Command::Run>(r_true));
  EXPECT_EQ(rs::get_if<Command::Run>(&r_true)->flag, true);

  EXPECT_TRUE(rs::holds_alternative<Command::Pause>(pause));
  EXPECT_TRUE(rs::holds_alternative<Command::Stop>(stop));
  EXPECT_TRUE(rs::holds_alternative<Command::Restart>(restart));
}

// -----------------------------------------------------------------------------
// Signed Direct Discriminant Enum Test (#[repr(i8)])
// -----------------------------------------------------------------------------
class SignedEnum {
 public:
  using Discriminant = int8_t;

  struct Neg : public crubit::internal::Variant<Neg, SignedEnum, -1> {
    ENUM_API_TEST_PRIVATE_COPY_AND_MOVE(Neg)
    constexpr explicit Neg(VariantKey) {}

   private:
    int8_t tag = -1;
  };
  struct Zero : public crubit::internal::Variant<Zero, SignedEnum, 0> {
    ENUM_API_TEST_PRIVATE_COPY_AND_MOVE(Zero)
    constexpr explicit Zero(VariantKey) {}

   private:
    int8_t tag = 0;
  };
  struct Pos : public crubit::internal::Variant<Pos, SignedEnum, 1> {
    ENUM_API_TEST_PRIVATE_COPY_AND_MOVE(Pos)
    constexpr explicit Pos(VariantKey) {}

   private:
    int8_t tag = 1;
  };

  template <typename V, typename... Args>
    requires crubit::internal::ConstructibleVariantOf<V, SignedEnum, Args...>
  constexpr explicit SignedEnum(std::in_place_type_t<V> v, Args&&... args)
      : storage_(v, std::forward<Args>(args)...) {}

 private:
  friend struct crubit::internal::EnumAccess;

  crubit::internal::VariadicUnionStorage<
      crubit::internal::tag_encoding::Direct<int8_t, 0>, Neg, Zero, Pos>
      storage_;
};

TEST(EnumApiTest, SignedDirectDiscriminant) {
  static_assert(
      std::is_same_v<decltype(RawDiscriminant(std::declval<SignedEnum>())),
                     int8_t>);

  SignedEnum neg = SignedEnum::Neg::Make();
  SignedEnum zero = SignedEnum::Zero::Make();
  SignedEnum pos = SignedEnum::Pos::Make();

  EXPECT_EQ(RawDiscriminant(neg), -1);
  EXPECT_EQ(RawDiscriminant(zero), 0);
  EXPECT_EQ(RawDiscriminant(pos), 1);

  EXPECT_TRUE(rs::holds_alternative<SignedEnum::Neg>(neg));
  EXPECT_TRUE(rs::holds_alternative<SignedEnum::Zero>(zero));
  EXPECT_TRUE(rs::holds_alternative<SignedEnum::Pos>(pos));
}

// -----------------------------------------------------------------------------
// Constexpr Construction
//
// Construction activates a union member, which is permitted in a constant
// expression, and each variant writes its own tag through a default member
// initializer. Reading the tag back requires inspecting the object
// representation, so `discriminant` / `get_if` are runtime-only;
// `get_unchecked` is the constexpr path for reading a variant whose identity
// is already known.
// -----------------------------------------------------------------------------

// Enums over trivially destructible payloads remain literal types.
static_assert(std::is_trivially_destructible_v<Option>);
static_assert(std::is_trivially_copyable_v<Option>);

// Direct tag encoding, constructed at compile time.
constexpr Option kConstexprSome = Option::Some::Make(42);
constexpr Option kConstexprNone = Option::None::Make();
static_assert(rs::get_unchecked<Option::Some>(&kConstexprSome)->value == 42);

// Constexpr construction and readback inside a constant-evaluated function.
constexpr int32_t SumInConstexpr(int32_t a, int32_t b) {
  Option x = Option::Some::Make(a);
  Option y = Option::Some::Make(b);
  return rs::get_unchecked<Option::Some>(&x)->value +
         rs::get_unchecked<Option::Some>(&y)->value;
}
static_assert(SumInConstexpr(20, 22) == 42);

// Mutation during constant evaluation.
constexpr int32_t MutateInConstexpr() {
  Option o = Option::Some::Make(1);
  rs::get_unchecked<Option::Some>(&o)->value = 99;
  return rs::get_unchecked<Option::Some>(&o)->value;
}
static_assert(MutateInConstexpr() == 99);

// Niche encodings are constexpr-constructible too.
constexpr int32_t kConstexprTarget = 7;
constexpr RefOption kConstexprRef = RefOption::Some::Make(&kConstexprTarget);
static_assert(*rs::get_unchecked<RefOption::Some>(&kConstexprRef)->value == 7);

constexpr Command kConstexprRun = Command::Run::Make(true);
static_assert(rs::get_unchecked<Command::Run>(&kConstexprRun)->flag);

// The encodings themselves are covered by enum_tag_encoding_test.cc. What
// belongs here is that *this* enum's variants cover the encoding's tag read
// (which `VariadicUnionStorage` also asserts). `None` is the interesting one:
// it is pure tag with no payload, so it covers the 4-byte read exactly, with
// nothing to spare.
namespace te = crubit::internal::tag_encoding;

static_assert(sizeof(Option::None) == 4);
static_assert(sizeof(Option::Some) == 8);
static_assert(te::Direct<uint32_t, 0>::kTagFootprint == 4);

TEST(EnumApiTest, ConstexprObjectsAreReadableAtRuntime) {
  // Enums built at compile time carry correct tag bytes into runtime.
  EXPECT_EQ(RawDiscriminant(kConstexprSome), RawDiscriminantOf<Option::Some>());
  EXPECT_EQ(RawDiscriminant(kConstexprNone), RawDiscriminantOf<Option::None>());
  EXPECT_TRUE(rs::holds_alternative<Option::Some>(kConstexprSome));
  EXPECT_TRUE(rs::holds_alternative<Option::None>(kConstexprNone));

  EXPECT_THAT(rs::get_if<Option::Some>(&kConstexprSome),
              Pointee(Field(&Option::Some::value, 42)));
  EXPECT_EQ(rs::get_if<Option::Some>(&kConstexprNone), nullptr);

  // Niche encodings likewise.
  EXPECT_EQ(RawDiscriminant(kConstexprRef),
            RawDiscriminantOf<RefOption::Some>());
  EXPECT_EQ(rs::get_if<RefOption::Some>(&kConstexprRef)->value,
            &kConstexprTarget);

  EXPECT_EQ(RawDiscriminant(kConstexprRun), RawDiscriminantOf<Command::Run>());
  EXPECT_TRUE(rs::get_if<Command::Run>(&kConstexprRun)->flag);
}

TEST(EnumApiTest, GetUncheckedAgreesWithGetIfOnActiveVariant) {
  Option o = Option::Some::Make(123);
  EXPECT_EQ(rs::get_unchecked<Option::Some>(&o), rs::get_if<Option::Some>(&o));
  EXPECT_EQ(rs::get_unchecked<Option::Some>(&o)->value, 123);
}

TEST(EnumApiDeathTest, GetUncheckedOnInactiveVariantFailsInDebugBuilds) {
  Option none = Option::None::Make();
  EXPECT_DEBUG_DEATH(rs::get_unchecked<Option::Some>(&none),
                     "rs::get_unchecked: V is not the active variant");
}

// -----------------------------------------------------------------------------
// Tag Round-Tripping Through Real Memory
//
// Variant tag members are private and never named after initialization; the
// only reads go through a byte-wise `reinterpret_cast`.
//
// These tests were originally written to ask whether an optimizer might drop
// those stores as dead. It cannot, and no test is needed to establish that:
// reading an object's representation through `unsigned char` is permitted by
// [basic.lval]/11.3, which makes a byte-wise load a well-defined read, and the
// as-if rule then forbids eliding the store in any way that changes its result.
// A conforming compiler cannot fail these for that reason.
//
// What they are actually worth is the *other* direction: they force the decode
// to happen at run time against real memory instead of being constant-folded at
// compile time, which is where a latent UB-driven miscompile in the cast-and-
// decode path would show up. That is what the `noinline` below is for. They
// also pin two ordinary behavioural properties -- that a trivial copy carries
// the tag and not just the payload, and that assignment updates it.
//
// (`-Wunused-private-field` fires on the tag members precisely because nothing
// names them, which is why they carry NOLINT -- the warning is about
// readability, not reachability.)
// -----------------------------------------------------------------------------

// Without a real call boundary the compiler folds the whole thing and the
// run-time path these tests exist to exercise never executes. absl degrades
// this attribute to nothing on a toolchain that lacks it, which would silently
// turn these into compile-time tautologies.
#ifndef ABSL_HAVE_ATTRIBUTE_NOINLINE
#error "These tests need a real noinline; without it they fold to a tautology."
#endif

ABSL_ATTRIBUTE_NOINLINE uint32_t ReadThroughPointer(const Option* o) {
  return RawDiscriminant(*o);
}

ABSL_ATTRIBUTE_NOINLINE Option RoundTripByValue(Option o) { return o; }

TEST(EnumApiTest, DiscriminantReadsBackThroughAnOpaqueCall) {
  Option some = Option::Some::Make(42);
  Option none = Option::None::Make();
  EXPECT_EQ(ReadThroughPointer(&some), RawDiscriminantOf<Option::Some>());
  EXPECT_EQ(ReadThroughPointer(&none), RawDiscriminantOf<Option::None>());
}

TEST(EnumApiTest, TrivialCopyCarriesTheTag) {
  // The implicit copy is a byte copy of the variant union; it must carry the
  // tag, not just the payload that later code is seen to use.
  Option copy = RoundTripByValue(Option::Some::Make(7));
  EXPECT_EQ(RawDiscriminant(copy), RawDiscriminantOf<Option::Some>());
  EXPECT_EQ(rs::get_if<Option::Some>(&copy)->value, 7);
}

TEST(EnumApiTest, AssignmentUpdatesTheTag) {
  Option o = Option::Some::Make(1);
  o = Option::None::Make();
  EXPECT_EQ(RawDiscriminant(o), RawDiscriminantOf<Option::None>());
  EXPECT_EQ(rs::get_if<Option::Some>(&o), nullptr);
}

}  // namespace

}  // namespace crubit
