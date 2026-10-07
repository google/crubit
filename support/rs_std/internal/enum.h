// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_INTERNAL_ENUM_H_
#define THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_INTERNAL_ENUM_H_

#include <concepts>
#include <cstddef>
#include <cstdint>
#include <cstring>
#include <memory>
#include <type_traits>
#include <utility>

// Implementation of the C++ API for Rust enums. Callers should include
// support/rs_std/enum.h, which exports this header, and use
// the API in `namespace rs` there.
//
// Everything here is for the generated bindings, not for callers. A generated
// enum is a class declaring its discriminant type as `Discriminant`, with one
// nested struct per variant deriving from
// `::crubit::internal::Variant<Self, Enum, d>`, and a single
// `::crubit::internal::VariadicUnionStorage` member holding the payload. See
// support/rs_std/enum_test_option_i32.h for what that looks
// like.

namespace crubit::internal {

// The internal sections below are in dependency order: tag encodings know
// nothing about the union, the union knows nothing about tags, and the storage
// combines them.

// =============================================================================
// Tag Encodings
// =============================================================================

// Helper concept matching one of a set of allowed types.
template <typename T, typename... Allowed>
concept OneOf = (std::same_as<T, Allowed> || ...);

// Concept for types permitted as discriminants in rustc_abi (8, 16, 32, 64, and
// 128-bit integers).
//
// The discriminant type must be derived from the width of the tag `Scalar`
// reported by rustc_abi, *not* from the number of variants. rustc performs all
// tag arithmetic at the width of `tag_layout`, which it computes as
// `tag_scalar_layout.primitive().to_int_ty()`. Pointer-primitive tags (as in
// `Option<&T>`) are converted to a pointer-sized integer, so e.g. `Option<&T>`
// has a 64-bit discriminant on 64-bit targets.
template <typename T>
concept RustcDiscriminant = OneOf<T, int8_t, uint8_t, int16_t, uint16_t,
                                  int32_t, uint32_t, int64_t, uint64_t
#ifdef __SIZEOF_INT128__
                                  ,
                                  __int128_t, __uint128_t
#endif
                                  >;

// Concept that checks whether a type `T` implements the tag encoding policy
// interface: a stateless policy that decodes a discriminant from the object
// representation of an enum's storage.
//
// `kTagFootprint` is the number of bytes past the start of that storage the
// policy reads, i.e. one past the last byte index `DiscriminantFromBytes`
// touches. It exists so `VariadicUnionStorage` (below) can check statically
// that every variant is wide enough to contain the bytes being read.
//
// Storage reads its discriminant through
// `VariadicUnionStorage::discriminant()`, which calls `DiscriminantFromBytes`
// at runtime.
template <typename T>
concept RustcTagEncoding = requires {
  typename T::Discriminant;
  requires RustcDiscriminant<typename T::Discriminant>;
  requires std::same_as<decltype(T::kTagFootprint), const size_t>;
} && requires(const unsigned char* storage) {
  {
    T::DiscriminantFromBytes(storage)
  } -> std::same_as<typename T::Discriminant>;
};

namespace tag_encoding {

// Tag encodings are stateless: they occupy no storage of their own. The tag
// bytes are written by each variant's own tag member, which is what allows enum
// construction to remain a constant expression.
//
// Each policy's `DiscriminantFromBytes(bytes)` decodes a discriminant from raw
// bytes. It is the only encoding-specific part of reading a discriminant, and
// it is runtime-only, like `VariadicUnionStorage::discriminant()`.

// 1. For rustc_abi::Variants::Single (untagged or single-variant enums).
template <typename D, D ActiveVariant>
  requires RustcDiscriminant<D>
struct Single {
  using Discriminant = D;

  // Reads no storage at all, so it imposes no minimum variant width.
  static constexpr size_t kTagFootprint = 0;

  static D DiscriminantFromBytes(const unsigned char*) noexcept {
    return ActiveVariant;
  }
};

// 2. For rustc_abi::TagEncoding::Direct (dedicated integer tag in storage).
template <typename D, size_t TagOffset>
  requires RustcDiscriminant<D>
struct Direct {
  using Discriminant = D;
  static constexpr size_t kTagFootprint = TagOffset + sizeof(D);

  static_assert(TagOffset % alignof(D) == 0,
                "crubit.rs-bug: the tag offset should be aligned to the "
                "discriminant type.");

  static D DiscriminantFromBytes(const unsigned char* storage) noexcept {
    D tag;
    std::memcpy(&tag, storage + TagOffset, sizeof(D));
    return tag;
  }
};

// 3. For rustc_abi::TagEncoding::Niche (reuses invalid values in a variant
// field).
//
// The niche variants `NicheStartVariant..=NicheEndVariant` are encoded as
// consecutive tag values starting at `NicheStartValue`; every other tag value
// belongs to the untagged variant.
//
// rustc assigns those tag values modulo the width of the tag, so the run may
// wrap past the top of the range: e.g. three niche variants starting at 254 in
// a `uint8_t` tag are encoded as 254, 255, 0. The last niche value can thus be
// *smaller* than `NicheStartValue`, and a plain `start <= tag <= end` range
// check doesn't work. Instead, like rustc's own decoder
// (rustc_const_eval/src/interpret/discriminant.rs), we do all arithmetic
// modulo the tag width, in the unsigned type of the same width as `D`.
template <typename D, size_t NicheOffset, D UntaggedVariant,
          D NicheStartVariant, D NicheEndVariant, D NicheStartValue>
  requires RustcDiscriminant<D>
struct Niche {
  using Discriminant = D;

  // The niche read is delegated to `Direct` below, so the footprint is
  // whatever that read covers. Note this applies to the untagged variant too:
  // it must be wide enough for the field the niche is carved out of.
  static constexpr size_t kTagFootprint = Direct<D, NicheOffset>::kTagFootprint;

  static_assert(NicheStartVariant <= NicheEndVariant,
                "crubit.rs-bug: the niche variant range should be non-empty "
                "(it is inclusive on both ends).");

  static D DiscriminantFromBytes(const unsigned char* storage) noexcept {
    // We follow the exact same logic as rustc's decoder at
    // https://github.com/rust-lang/rust/blob/b57eb9a5fd94f30697f51fc88fbe898e67c7aae6/compiler/rustc_const_eval/src/interpret/discriminant.rs#L164-L202

    // Unsigned, so that wrapping is well-defined. The `static_cast<U>`s on the
    // arithmetic below undo integer promotion, which would otherwise turn
    // e.g. a `uint8_t` subtraction into a (possibly negative) `int`.
    using U = std::make_unsigned_t<D>;

    U tag =
        static_cast<U>(Direct<D, NicheOffset>::DiscriminantFromBytes(storage));

    // The tag's position within the run of niche values. Because the
    // subtraction wraps, this is correct even when the run itself wraps (in
    // the example above, tag 0 gives 0 - 254 = 2). Tags outside the run land
    // on a position past its end, including those just below
    // `NicheStartValue`, which wrap around to large values.
    U relative = static_cast<U>(tag - static_cast<U>(NicheStartValue));

    // The position of the last niche value. `NicheStartVariant <=
    // NicheEndVariant` (asserted above), so this doesn't wrap.
    U last = static_cast<U>(NicheEndVariant - NicheStartVariant);

    if (relative <= last) {
      return static_cast<D>(static_cast<U>(NicheStartVariant) + relative);
    }
    return UntaggedVariant;
  }
};

}  // namespace tag_encoding

// =============================================================================
// Variadic Union
// =============================================================================

// Recursive union storage for variant types. VariadicUnion does not manage the
// active variant's lifetime; non-trivial copy, move, and destruction are
// handled on the enclosing enum class by calling into Rust.
//
// Because the variants are real union members rather than bytes, construction
// and `get` are constant expressions, and the compiler tracks which member is
// active during constant evaluation. Once `std::is_within_lifetime` (C++26) is
// available on every supported compiler, that tracking is what will let
// `VariadicUnionStorage::discriminant()` become a constant expression too.
//
// C++ has no way to declare one union member per element of a pack (members
// need names, and a member declaration is not a pack expansion context), so
// this is a recursive coproduct: `VariadicUnion<A, B, C>` holds either an `A`
// or a `VariadicUnion<B, C>`, and so on down to the empty union.
//
// The in-place constructor and `get` recurse through `vs` until they reach
// `Target`. Both require `Target` to be one of the members, so a call with any
// other type is rejected up front, and the recursion never reaches the empty
// union below (which has neither).
//
// This primary template is the base case: the empty union that ends the
// recursion.
template <typename... Vs>
union VariadicUnion {
  constexpr VariadicUnion() noexcept {}
  constexpr ~VariadicUnion() = default;
};

// Recursive case: either the first variant `V`, or a union of the rest.
template <typename V, typename... Vs>
union VariadicUnion<V, Vs...> {
  V v;
  VariadicUnion<Vs...> vs;

  constexpr VariadicUnion() noexcept {}
  constexpr VariadicUnion(const VariadicUnion&) = default;
  constexpr VariadicUnion& operator=(const VariadicUnion&) = default;
  constexpr VariadicUnion(VariadicUnion&&) = default;
  constexpr VariadicUnion& operator=(VariadicUnion&&) = default;

  template <typename Target, typename... Args>
    requires OneOf<Target, V, Vs...>
  constexpr explicit VariadicUnion(std::in_place_type_t<Target>,
                                   Args&&... args) {
    if constexpr (std::is_same_v<Target, V>) {
      std::construct_at(&v, std::forward<Args>(args)...);
    } else {
      std::construct_at(&vs, std::in_place_type<Target>,
                        std::forward<Args>(args)...);
    }
  }

  constexpr ~VariadicUnion()
    requires(std::is_trivially_destructible_v<V> &&
             (std::is_trivially_destructible_v<Vs> && ...))
  = default;

  constexpr ~VariadicUnion() noexcept {}

  template <typename Target>
    requires OneOf<Target, V, Vs...>
  constexpr const Target* get() const noexcept {
    if constexpr (std::is_same_v<Target, V>) {
      return &v;
    } else {
      return vs.template get<Target>();
    }
  }

  template <typename Target>
    requires OneOf<Target, V, Vs...>
  constexpr Target* get() noexcept {
    if constexpr (std::is_same_v<Target, V>) {
      return &v;
    } else {
      return vs.template get<Target>();
    }
  }
};

// =============================================================================
// Enum Machinery
// =============================================================================

// The two variant mixins below are a deliberate split, not one class:
//
//   VariantOf<E>            -- a variant knows its enum, and nothing else.
//   Variant<Self, E, d>     -- a variant knows its enum and its own
//                              discriminant, and can `Make` an `E`.
//
// `VariantOf` is load-bearing; do not fold it into `Variant`. Because it is
// parameterized only on the enum, `std::derived_from<V, VariantOf<E>>` can ask
// "is `V` a variant of `E`?" without naming `V`'s discriminant. That is the
// question the public API has to answer, since an enum type is all a caller
// names. Folding the two together would make the predicate demand the very
// discriminant the caller is trying to discover.
//
// The enum itself needs no base: it declares `using Discriminant = D;`
// directly, which is all `RustEnum` looks for.

// Base mixin associating a variant with its enclosing enum type.
template <typename EnumType>
struct VariantOf {
  using Enum = EnumType;
};

template <typename EnumType>
concept RustEnum = requires { typename EnumType::Discriminant; };

// Concept that checks whether `V` is a variant declared inside `Enum`.
//
// This is the predicate the public API is constrained on, because an enum type
// is all a caller names. The storage layer asks a stricter question -- whether
// `V` is exactly one of the variants in its own pack -- which is not the same
// thing: a variant could belong to the enum and still be missing from the
// storage it was generated alongside. Keeping the two distinct is what leaves
// that seam checked.
template <typename V, typename Enum>
concept VariantOfEnum =
    std::derived_from<V, VariantOf<std::remove_cvref_t<Enum>>>;

template <typename TagEncoding, typename... Vs>
  requires RustcTagEncoding<TagEncoding>
class VariadicUnionStorage;

// Passkey for variant constructors. Every variant constructor takes one of
// these first, and only `VariadicUnionStorage` can create one, so a variant
// can only be constructed in place inside its enum:
//
//   Option o = Option::Some::Make(7);               // OK
//   Option p(std::in_place_type<Option::Some>, 7);  // OK
//   Option::Some s(7);                              // does not compile
//
// Without this, `Option::Some(7)` would compile and read exactly like the Rust
// expression that produces an `Option` -- but it would produce a variant, not
// an enum.
//
// This is a passkey rather than private constructors because a private
// constructor is inaccessible to `std::construct_at`, which is what keeps
// in-place construction a constant expression, and it makes every
// `std::constructible_from` constraint on a variant answer false. Here the
// constructors stay public, so both keep working; what is withheld is the
// argument.
//
// Together with variants' private copy and move (see `Variant`), this keeps
// variants inside their enum: they cannot be built, copied, or moved on their
// own, only reached through `rs::get_if` / `rs::get`. It guards against
// accidents, not determined misuse.
class VariantKey {
 private:
  // User-declared, so `VariantKey` is not an aggregate and `{}` cannot build
  // one either.
  constexpr VariantKey() = default;

  template <typename TagEncoding, typename... Vs>
    requires RustcTagEncoding<TagEncoding>
  friend class VariadicUnionStorage;
};

// Concept that checks whether `V` is a variant of `Enum` constructible from
// `args`. This is what a generated enum's in-place constructor is constrained
// on, so that generated code never has to name `VariantKey` outside a variant
// constructor's parameter list.
template <typename V, typename Enum, typename... Args>
concept ConstructibleVariantOf =
    VariantOfEnum<V, Enum> && std::constructible_from<V, VariantKey, Args...>;

// Base struct associating a variant with its enclosing enum type and
// discriminant, and giving it a `Make` factory.
//
// `Self` is the deriving variant (CRTP), so that `Make` knows which variant to
// build without any per-variant generated code.
//
// Variants are not copyable or movable on their own; only the enum is (when
// the Rust type permits). Each generated variant therefore declares its copy
// and move operations defaulted but private, befriending the union that holds
// it:
//
//   struct Some : ::crubit::internal::Variant<Some, Option, 1> {
//     ...
//    private:
//     template <typename...>
//     friend union ::crubit::internal::VariadicUnion;
//     Some(const Some&) = default;
//     Some(Some&&) = default;
//     Some& operator=(const Some&) = default;
//     Some& operator=(Some&&) = default;
//   };
//
// Private-but-defaulted, rather than deleted, is what keeps a `Copy` enum
// trivially copyable: a deleted member would delete the union's copy too, and
// with it the enum's. Defaulted members stay trivial whatever their access, so
// for a `Copy` enum the union -- the one class allowed to use them -- copies
// trivially, and so does the enum. For a non-`Copy` enum the union's copy is
// unavailable regardless, and the enum's copy and move go through Rust.
//
// This cannot live in this base class: a private base member would make the
// derived variant's own implicit copy deleted, a protected one would make it
// public, and the base cannot befriend the union on the variant's behalf.
template <typename Self, typename EnumType,
          typename EnumType::Discriminant Discriminant>
  requires RustEnum<EnumType>
struct Variant : VariantOf<EnumType> {
  // Builds an `EnumType` holding a `Self` constructed from `args`:
  //
  //   constexpr Option kSeven = Option::Some::Make(7);
  //
  // This is the enum's in-place constructor spelled from the variant's side,
  // so unlike a variant constructor it returns the enum, not the variant. The
  // variant is still built directly in the enum's storage: the returned
  // prvalue initializes the caller's object without a copy or move.
  //
  // Constrained rather than checked in the body, so that "can I `Make` this
  // variant from these arguments?" has an honest `requires` answer.
  template <typename... Args>
    requires ConstructibleVariantOf<Self, EnumType, Args...>
  static constexpr EnumType Make(Args&&... args) {
    static_assert(
        std::derived_from<Self, Variant>,
        "crubit.rs-bug: a variant should pass itself as the first "
        "template argument of the ::crubit::internal::Variant it derives from");
    static_assert(!std::is_copy_constructible_v<Self> &&
                      !std::is_move_constructible_v<Self> &&
                      !std::is_copy_assignable_v<Self> &&
                      !std::is_move_assignable_v<Self>,
                  "crubit.rs-bug: a variant should declare its copy and move "
                  "operations private (see ::crubit::internal::Variant)");
    return EnumType(std::in_place_type<Self>, std::forward<Args>(args)...);
  }

 private:
  // The raw discriminant value. Private so that callers can't depend on its
  // type, which is an implementation detail of the enum's layout; they ask
  // `rs::holds_alternative<V>(e)` instead.
  static constexpr typename EnumType::Discriminant kDiscriminant = Discriminant;

  template <typename TagEncoding, typename... Vs>
    requires RustcTagEncoding<TagEncoding>
  friend class VariadicUnionStorage;
  friend struct EnumAccess;
};

// --- Variadic Union Storage --------------------------------------------------

// Deliberately not constexpr: reaching it during constant evaluation is a
// compile error. Compilers quote the call's source line in the diagnostic, so
// pass the explanation as a string literal on that same line.
inline void crubit_compile_time_error(const char* /*message*/) {}

// Storage for a Rust enum: a union of variant structs, where each variant
// carries its own tag field at the offset reported by rustc_abi.
//
// The tag is deliberately *not* a member of the union. Making it one would mean
// every discriminant read was a read of an inactive union member, which is
// undefined behavior (the variants are not layout-compatible with a tag type,
// so the common initial sequence rule does not apply). Instead, at runtime
// `TagEncoding` reads the tag from this object's representation via `unsigned
// char`, which is permitted by [basic.lval]/11.3.
//
// Consequences:
//   * Construction is a constant expression: activating a union member is
//     allowed in constant expressions, and each variant writes its own tag
//     through an ordinary default member initializer. So is `get`.
//   * Reading the discriminant is runtime-only (see `discriminant()`), and so
//     are `get_if` and everything built on it.
//   * Types whose variants are all trivially copyable and destructible keep
//     defaulted (and hence constexpr) special member functions, so they remain
//     literal types and may be used as constexpr variables.
//   * Non-trivial copy, move, and destruction are handled on the enclosing enum
//     class by calling into Rust (`Clone::clone`, `MemSwap`, `Drop::drop`), so
//     this storage class does not dispatch on the tag to copy, move, or destroy
//     variants in C++.
//   * An enum with drop glue (but no `Drop` impl of its own) gets a constexpr
//     destructor that skips the Rust drop during constant evaluation:
//
//       constexpr ~E() {
//         if (!std::is_constant_evaluated()) { /* call the Rust drop */ }
//       }
//
//     The in-place constructor only lets trivially destructible variants be
//     created during constant evaluation, so the skip never skips anything.
template <typename TagEncoding, typename... Vs>
  requires RustcTagEncoding<TagEncoding>
class VariadicUnionStorage {
 public:
  using Discriminant = typename TagEncoding::Discriminant;

  // Every variant lies at offset 0 of `vs_`, so a variant narrower than the
  // tag read would take its discriminant from the union's trailing padding,
  // which holds unspecified bytes -- a wrong discriminant, silently. Nothing
  // else in the design catches this. (Only coverage is checked: that those
  // bytes hold a tag member of the right type and value is the generator's
  // responsibility, which C++ cannot verify without reflection.)
  //
  // This sits at class scope rather than in `discriminant()` so that it fires
  // when the enum is declared. Inside `discriminant()` it only fired once
  // something instantiated that function, which let a broken layout through
  // for any enum whose tag is never read.
  static_assert(((sizeof(Vs) >= TagEncoding::kTagFootprint) && ...),
                "crubit.rs-bug: every variant should be at least "
                "TagEncoding::kTagFootprint bytes wide; a narrower one reads "
                "padding instead of its tag.");

  // Creates an uninitialized union so the enclosing enum's thunk-backed
  // constructors (e.g. `Default`, `Clone`, `UnsafeRelocateTag`) can populate
  // the object representation via Rust or `memcpy`.
  constexpr VariadicUnionStorage() noexcept = default;

  // Activates the variant `Target` by constructing it in place from `args`.
  // `Target` is named explicitly, via `std::in_place_type<V>`.
  //
  // This is the only place a `VariantKey` is created, and it goes straight to
  // `Target`'s constructor, which is what confines variant construction to
  // the inside of an enum.
  //
  // At runtime any variant may be constructed. During constant evaluation only
  // trivially destructible ones may: an enum with drop glue skips its Rust drop
  // during constant evaluation (the drop is a call into Rust), and this is what
  // makes skipping it sound -- whatever is skipped had nothing to drop. So
  // `static constexpr` `None` works for an `Option<T>` with drop glue, while
  // `Some` is a compile error. Note that while the variant may be trivially
  // destructible, the parent enum is not, meaning that `Some(None)` of
  // `Option<Option<T>>` is not consteval friendly because the inner `Option<T>`
  // is not trivially destructible.
  //
  // This is checked in the body rather than in a `requires` clause or a
  // `static_assert`: one instantiation serves both runtime and compile-time
  // calls, and inside either of those `std::is_constant_evaluated()` is always
  // true, so they would forbid runtime construction too.
  template <typename Target, typename... Args>
    requires(OneOf<Target, Vs...> &&
             std::constructible_from<Target, VariantKey, Args...>)
  constexpr explicit VariadicUnionStorage(std::in_place_type_t<Target> tag,
                                          Args&&... args)
      : vs_(tag, VariantKey(), std::forward<Args>(args)...) {
    if (std::is_constant_evaluated() &&
        !std::is_trivially_destructible_v<Target>) {
      crubit_compile_time_error("variants with drop glue are runtime-only");
    }
  }

  // Copy and move are trivial when every variant is trivially copyable (a Rust
  // `Copy` enum), and otherwise unavailable. These must be spelled out: the
  // implicit ones would, e.g., leave a variant with only a custom destructor
  // copyable.
  //
  // The condition is `is_trivially_copyable` rather than, e.g.,
  // `is_trivially_copy_constructible`, because variants declare their copy and
  // move private (see `Variant`). The latter traits ask whether *anyone* may
  // copy, and would say no; `is_trivially_copyable` asks only whether the
  // copies that exist are trivial, which is what makes copying the bytes of the
  // whole union correct.
  static constexpr bool kTriviallyCopyable =
      (std::is_trivially_copyable_v<Vs> && ...);

  constexpr VariadicUnionStorage(const VariadicUnionStorage&)
    requires kTriviallyCopyable
  = default;
  constexpr VariadicUnionStorage& operator=(const VariadicUnionStorage&)
    requires kTriviallyCopyable
  = default;
  constexpr VariadicUnionStorage(VariadicUnionStorage&&)
    requires kTriviallyCopyable
  = default;
  constexpr VariadicUnionStorage& operator=(VariadicUnionStorage&&)
    requires kTriviallyCopyable
  = default;

  // Trivial destructor
  constexpr ~VariadicUnionStorage()
    requires(std::is_trivially_destructible_v<Vs> && ...)
  = default;

  // No-op when any variant has a non-trivial destructor: the enclosing enum's
  // `~E()` invokes the Rust drop thunk, and destroying the active variant in
  // C++ as well would double-drop it. Not `= default`, which would be deleted
  // here because `vs_` has a non-trivial destructor.
  // NOLINTNEXTLINE(modernize-use-equals-default)
  constexpr ~VariadicUnionStorage() noexcept {}

  // Recovers the active variant's discriminant by decoding the tag from the
  // object representation (`TagEncoding::DiscriminantFromBytes`): accessing an
  // object's representation through `unsigned char` is permitted by
  // [basic.lval]/11.3, and since every variant lies at offset 0 of `vs_`, the
  // tag bytes lie within it.
  //
  // Runtime-only: the `reinterpret_cast` that gets there is forbidden in
  // constant expressions. Asking the compiler which member is live instead
  // needs C++26 `std::is_within_lifetime`, which not every supported compiler
  // provides; until they all do, this is runtime-only everywhere rather than
  // constexpr on some compilers only.
  Discriminant discriminant() const noexcept {
    return TagEncoding::DiscriminantFromBytes(
        reinterpret_cast<const unsigned char*>(&vs_));
  }

  // Returns `Target` if it is the active variant, and null otherwise. `Target`
  // must be exactly one of `Vs...`, not merely a variant of the same enum,
  // because it names a member of `vs_`. Runtime-only, like `discriminant()`.
  template <typename Target>
    requires OneOf<Target, Vs...>
  const Target* get_if() const noexcept {
    return discriminant() == Target::kDiscriminant ? get<Target>() : nullptr;
  }

  template <typename Target>
    requires OneOf<Target, Vs...>
  Target* get_if() noexcept {
    return discriminant() == Target::kDiscriminant ? get<Target>() : nullptr;
  }

  // Projects to `Target` without consulting the tag.
  //
  // Precondition: `Target` is the active variant; violating it is undefined
  // behavior.
  template <typename Target>
    requires OneOf<Target, Vs...>
  constexpr const Target* get() const noexcept {
    return vs_.template get<Target>();
  }

  template <typename Target>
    requires OneOf<Target, Vs...>
  constexpr Target* get() noexcept {
    return vs_.template get<Target>();
  }

 private:
  VariadicUnion<Vs...> vs_;
};

// Friend accessor allowing public Crubit operations to inspect enum storage.
// `E` may be const-qualified, in which case so is the result.
struct EnumAccess {
  template <typename E>
  static constexpr auto& storage(E& e) noexcept {
    return e.storage_;
  }

  // The raw discriminant value of variant `V`, which `Variant` keeps private.
  template <typename V>
  static constexpr auto variant_discriminant() noexcept {
    return V::kDiscriminant;
  }

  // Wraps and unwraps `rs::Discriminant`, whose constructor and value are
  // private. Templated on the wrapper so that this can be declared first.
  template <typename WrappedDiscriminant, typename Raw>
  static constexpr WrappedDiscriminant wrap_discriminant(Raw raw) noexcept {
    return WrappedDiscriminant(raw);
  }
  template <typename WrappedDiscriminant>
  static constexpr auto unwrap_discriminant(
      const WrappedDiscriminant& d) noexcept {
    return d.value_;
  }
};

}  // namespace crubit::internal

#endif  // THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_INTERNAL_ENUM_H_
