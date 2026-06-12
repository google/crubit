// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// from_golden

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_KNOWN_TRAITS_FROM_FROM_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_KNOWN_TRAITS_FROM_FROM_GOLDEN

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#pragma clang diagnostic ignored "-Wunused-private-field"
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
#pragma clang diagnostic ignored "-Wignored-attributes"
#include "support/annotations_internal.h"
#include "support/internal/slot.h"
#include "support/lifetime_annotations.h"
#include "support/rs_std/str_ref.h"

#include <array>
#include <cstddef>
#include <cstdint>
#include <cstring>
#include <type_traits>
#include <utility>

#include "support/rs_std/rs_alloc.h"

namespace from {
struct CloneAllocType;
struct CloneCopyType;
struct LoopB;
struct NoCloneCopyDropType;
struct NoCloneDefaultType;
struct OpaqueRef;
// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(":: from_golden :: CloneAllocSource") alignas(
    8) [[clang::trivial_abi]] CloneAllocSource final {
 public:
  // `from_golden::CloneAllocSource` doesn't implement the `Default` trait
  CloneAllocSource() = delete;

  // Drop::drop
  ~CloneAllocSource();

  // Clone::clone
  CloneAllocSource(const CloneAllocSource&);

  // Clone::clone_from
  ::from::CloneAllocSource& operator=(const CloneAllocSource&);

  CloneAllocSource(::crubit::UnsafeRelocateTag, CloneAllocSource&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

  // CRUBIT_ANNOTATE: must_bind=
  static ::from::CloneAllocSource create(rs::StrRef s);

  // CRUBIT_ANNOTATE: must_bind=
  rs::StrRef get_value() const& $(__anon1) CRUBIT_LIFETIME_BOUND;

  explicit operator ::from::CloneAllocType();

  union {
    ::rs::alloc::string::String value;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(":: from_golden :: CloneAllocType") alignas(8)
    [[clang::trivial_abi]] CloneAllocType final {
 public:
  // `from_golden::CloneAllocType` doesn't implement the `Default` trait
  CloneAllocType() = delete;

  // Drop::drop
  ~CloneAllocType();

  // Clone::clone
  CloneAllocType(const CloneAllocType&);

  // Clone::clone_from
  ::from::CloneAllocType& operator=(const CloneAllocType&);

  CloneAllocType(::crubit::UnsafeRelocateTag, CloneAllocType&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

  // CRUBIT_ANNOTATE: must_bind=
  rs::StrRef get_value() const& $(__anon1) CRUBIT_LIFETIME_BOUND;

  explicit CloneAllocType(::from::CloneAllocSource value);

  union {
    ::rs::alloc::string::String value;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(":: from_golden :: CloneCopySource") alignas(4)
    [[clang::trivial_abi]] CloneCopySource final {
 public:
  // `from_golden::CloneCopySource` doesn't implement the `Default` trait
  CloneCopySource() = delete;

  // Synthesized tuple constructor
  explicit CloneCopySource(::std::int32_t __field0)
      : __field0(::std::move(__field0)) {}

  // No custom `Drop` impl and no custom "drop glue" required
  ~CloneCopySource() = default;
  CloneCopySource(CloneCopySource&&) = default;
  CloneCopySource& operator=(CloneCopySource&&) = default;

  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  CloneCopySource(const CloneCopySource&) = default;
  CloneCopySource& operator=(const CloneCopySource&) = default;
  CloneCopySource(::crubit::UnsafeRelocateTag, CloneCopySource&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }
  explicit operator ::from::CloneCopyType();

  union {
    ::std::int32_t __field0;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(":: from_golden :: CloneCopyType") alignas(4)
    [[clang::trivial_abi]] CloneCopyType final {
 public:
  // Default::default
  CloneCopyType();

  // Synthesized tuple constructor
  explicit CloneCopyType(::std::int32_t __field0)
      : __field0(::std::move(__field0)) {}

  // No custom `Drop` impl and no custom "drop glue" required
  ~CloneCopyType() = default;
  CloneCopyType(CloneCopyType&&) = default;
  CloneCopyType& operator=(CloneCopyType&&) = default;

  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  CloneCopyType(const CloneCopyType&) = default;
  CloneCopyType& operator=(const CloneCopyType&) = default;
  CloneCopyType(::crubit::UnsafeRelocateTag, CloneCopyType&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }
  explicit CloneCopyType(::from::CloneCopySource value);

  union {
    ::std::int32_t __field0;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: from_golden :: CollidingConstructor") alignas(8) [[clang::trivial_abi]]
CollidingConstructor final {
 public:
  // `from_golden::CollidingConstructor` doesn't implement the `Default` trait
  CollidingConstructor() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~CollidingConstructor() = default;
  CollidingConstructor(CollidingConstructor&&) = default;
  CollidingConstructor& operator=(CollidingConstructor&&) = default;

  // `from_golden::CollidingConstructor` doesn't implement the `Clone` trait
  CollidingConstructor(const CollidingConstructor&) = delete;
  CollidingConstructor& operator=(const CollidingConstructor&) = delete;
  CollidingConstructor(::crubit::UnsafeRelocateTag,
                       CollidingConstructor&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

  // Error generating bindings for implementation
  // `<from_golden::CollidingConstructor as std::convert::From<u64>>` defined at
  // cc_bindings_from_rs/test/known_traits/from/from.rs;l=190:
  // From implementation for `u64` is not supported when `From<usize>` is
  // implemented as it may overlap.

  explicit CollidingConstructor(::std::uintptr_t value);

 private:
  union {
    ::std::uint64_t value;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(":: from_golden :: LoopA") alignas(4)
    [[clang::trivial_abi]] LoopA final {
 public:
  // `from_golden::LoopA` doesn't implement the `Default` trait
  LoopA() = delete;

  // Synthesized tuple constructor
  explicit LoopA(::std::int32_t __field0) : __field0(::std::move(__field0)) {}

  // No custom `Drop` impl and no custom "drop glue" required
  ~LoopA() = default;
  LoopA(LoopA&&) = default;
  LoopA& operator=(LoopA&&) = default;

  // `from_golden::LoopA` doesn't implement the `Clone` trait
  LoopA(const LoopA&) = delete;
  LoopA& operator=(const LoopA&) = delete;
  LoopA(::crubit::UnsafeRelocateTag, LoopA&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }
  explicit operator ::from::LoopB();

  explicit LoopA(::from::LoopB value);

  union {
    ::std::int32_t __field0;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(":: from_golden :: LoopB") alignas(4)
    [[clang::trivial_abi]] LoopB final {
 public:
  // `from_golden::LoopB` doesn't implement the `Default` trait
  LoopB() = delete;

  // Synthesized tuple constructor
  explicit LoopB(::std::int32_t __field0) : __field0(::std::move(__field0)) {}

  // No custom `Drop` impl and no custom "drop glue" required
  ~LoopB() = default;
  LoopB(LoopB&&) = default;
  LoopB& operator=(LoopB&&) = default;

  // `from_golden::LoopB` doesn't implement the `Clone` trait
  LoopB(const LoopB&) = delete;
  LoopB& operator=(const LoopB&) = delete;
  LoopB(::crubit::UnsafeRelocateTag, LoopB&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }
  explicit operator ::from::LoopA();

  explicit LoopB(::from::LoopA value);

  union {
    ::std::int32_t __field0;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: from_golden :: NoCloneCopyDropSource") alignas(4) [[clang::trivial_abi]]
NoCloneCopyDropSource final {
 public:
  // `from_golden::NoCloneCopyDropSource` doesn't implement the `Default` trait
  NoCloneCopyDropSource() = delete;

  // Synthesized tuple constructor
  explicit NoCloneCopyDropSource(::std::int32_t __field0)
      : __field0(::std::move(__field0)) {}

  // No custom `Drop` impl and no custom "drop glue" required
  ~NoCloneCopyDropSource() = default;
  NoCloneCopyDropSource(NoCloneCopyDropSource&&) = default;
  NoCloneCopyDropSource& operator=(NoCloneCopyDropSource&&) = default;

  // `from_golden::NoCloneCopyDropSource` doesn't implement the `Clone` trait
  NoCloneCopyDropSource(const NoCloneCopyDropSource&) = delete;
  NoCloneCopyDropSource& operator=(const NoCloneCopyDropSource&) = delete;
  NoCloneCopyDropSource(::crubit::UnsafeRelocateTag,
                        NoCloneCopyDropSource&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }
  explicit operator ::from::NoCloneCopyDropType();

  union {
    ::std::int32_t __field0;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: from_golden :: NoCloneCopyDropType") alignas(4) [[clang::trivial_abi]]
NoCloneCopyDropType final {
 public:
  // `from_golden::NoCloneCopyDropType` doesn't implement the `Default` trait
  NoCloneCopyDropType() = delete;

  // Synthesized tuple constructor
  explicit NoCloneCopyDropType(::std::int32_t __field0)
      : __field0(::std::move(__field0)) {}

  // No custom `Drop` impl and no custom "drop glue" required
  ~NoCloneCopyDropType() = default;
  NoCloneCopyDropType(NoCloneCopyDropType&&) = default;
  NoCloneCopyDropType& operator=(NoCloneCopyDropType&&) = default;

  // `from_golden::NoCloneCopyDropType` doesn't implement the `Clone` trait
  NoCloneCopyDropType(const NoCloneCopyDropType&) = delete;
  NoCloneCopyDropType& operator=(const NoCloneCopyDropType&) = delete;
  NoCloneCopyDropType(::crubit::UnsafeRelocateTag,
                      NoCloneCopyDropType&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }
  explicit NoCloneCopyDropType(::from::NoCloneCopyDropSource value);

  union {
    ::std::int32_t __field0;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: from_golden :: NoCloneDefaultSource") alignas(4) [[clang::trivial_abi]]
NoCloneDefaultSource final {
 public:
  // Default::default
  NoCloneDefaultSource();

  // Synthesized tuple constructor
  explicit NoCloneDefaultSource(::std::int32_t __field0)
      : __field0(::std::move(__field0)) {}

  // No custom `Drop` impl and no custom "drop glue" required
  ~NoCloneDefaultSource() = default;
  NoCloneDefaultSource(NoCloneDefaultSource&&) = default;
  NoCloneDefaultSource& operator=(NoCloneDefaultSource&&) = default;

  // `from_golden::NoCloneDefaultSource` doesn't implement the `Clone` trait
  NoCloneDefaultSource(const NoCloneDefaultSource&) = delete;
  NoCloneDefaultSource& operator=(const NoCloneDefaultSource&) = delete;
  NoCloneDefaultSource(::crubit::UnsafeRelocateTag,
                       NoCloneDefaultSource&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }
  explicit operator ::from::NoCloneDefaultType();

  union {
    ::std::int32_t __field0;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: from_golden :: NoCloneDefaultType") alignas(4) [[clang::trivial_abi]]
NoCloneDefaultType final {
 public:
  // Default::default
  NoCloneDefaultType();

  // Synthesized tuple constructor
  explicit NoCloneDefaultType(::std::int32_t __field0)
      : __field0(::std::move(__field0)) {}

  // No custom `Drop` impl and no custom "drop glue" required
  ~NoCloneDefaultType() = default;
  NoCloneDefaultType(NoCloneDefaultType&&) = default;
  NoCloneDefaultType& operator=(NoCloneDefaultType&&) = default;

  // `from_golden::NoCloneDefaultType` doesn't implement the `Clone` trait
  NoCloneDefaultType(const NoCloneDefaultType&) = delete;
  NoCloneDefaultType& operator=(const NoCloneDefaultType&) = delete;
  NoCloneDefaultType(::crubit::UnsafeRelocateTag, NoCloneDefaultType&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }
  explicit NoCloneDefaultType(::from::NoCloneDefaultSource value);

  union {
    ::std::int32_t __field0;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(":: from_golden :: NotFfiSafe") alignas(8)
    [[clang::trivial_abi]] NotFfiSafe final {
 public:
  // `from_golden::NotFfiSafe` doesn't implement the `Default` trait
  NotFfiSafe() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~NotFfiSafe() = default;
  NotFfiSafe(NotFfiSafe&&) = default;
  NotFfiSafe& operator=(NotFfiSafe&&) = default;

  // `from_golden::NotFfiSafe` doesn't implement the `Clone` trait
  NotFfiSafe(const NotFfiSafe&) = delete;
  NotFfiSafe& operator=(const NotFfiSafe&) = delete;
  NotFfiSafe(::crubit::UnsafeRelocateTag, NotFfiSafe&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

  // CRUBIT_ANNOTATE: must_bind=
  static ::from::NotFfiSafe create();

  explicit operator ::std::int32_t();

 private:
  // Field type has been replaced with a blob of bytes: Function pointers can't
  // have a thunk: Any calling convention other than `extern "C"` requires a
  // thunk
  ::std::array<unsigned char, 8> __field0;

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(":: from_golden :: Opaque") alignas(4)
    [[clang::trivial_abi]] Opaque final {
 public:
  // `from_golden::Opaque` doesn't implement the `Default` trait
  Opaque() = delete;

  // Synthesized tuple constructor
  explicit Opaque(::std::int32_t __field0) : __field0(::std::move(__field0)) {}

  // No custom `Drop` impl and no custom "drop glue" required
  ~Opaque() = default;
  Opaque(Opaque&&) = default;
  Opaque& operator=(Opaque&&) = default;

  // `from_golden::Opaque` doesn't implement the `Clone` trait
  Opaque(const Opaque&) = delete;
  Opaque& operator=(const Opaque&) = delete;
  Opaque(::crubit::UnsafeRelocateTag, Opaque&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }
  explicit operator ::std::int32_t();

  explicit operator ::std::int64_t();

  explicit operator rs::StrRef();

  explicit operator ::std::int16_t();

  explicit operator ::from::OpaqueRef();

  union {
    ::std::int32_t __field0;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(":: from_golden :: OpaqueRef") alignas(8)
    [[clang::trivial_abi]] OpaqueRef final {
 public:
  // `from_golden::OpaqueRef` doesn't implement the `Default` trait
  OpaqueRef() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~OpaqueRef() = default;
  OpaqueRef(OpaqueRef&&) = default;
  OpaqueRef& operator=(OpaqueRef&&) = default;

  // `from_golden::OpaqueRef` doesn't implement the `Clone` trait
  OpaqueRef(const OpaqueRef&) = delete;
  OpaqueRef& operator=(const OpaqueRef&) = delete;
  OpaqueRef(::crubit::UnsafeRelocateTag, OpaqueRef&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

  // CRUBIT_ANNOTATE: must_bind=
  static ::from::OpaqueRef create(rs::StrRef s);

  // CRUBIT_ANNOTATE: must_bind=
  rs::StrRef get_arg() const;

  explicit operator rs::StrRef();

  explicit OpaqueRef(::from::Opaque value);

 private:
  union {
    rs::StrRef __field0;
  };

 private:
  static void __crubit_field_offset_assertions();
};

static_assert(
    sizeof(CloneAllocSource) == 24,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(CloneAllocSource) == 8,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void __crubit_thunk_drop(::from::CloneAllocSource&);
}
inline CloneAllocSource::~CloneAllocSource() {
  __crubit_internal::__crubit_thunk_drop(*this);
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_clone(::from::CloneAllocSource const&,
                                     ::from::CloneAllocSource* __ret_ptr);
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_clone_ufrom(::from::CloneAllocSource&,
                                           ::from::CloneAllocSource const&);
}
inline ::from::CloneAllocSource::CloneAllocSource(
    const CloneAllocSource& other) {
  __crubit_internal::__crubit_thunk_clone(other, this);
}
inline ::from::CloneAllocSource& ::from::CloneAllocSource::operator=(
    const CloneAllocSource& other) {
  if (this != &other) {
    __crubit_internal::__crubit_thunk_clone_ufrom(*this, other);
  }
  return *this;
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_create(rs::StrRef,
                                      ::from::CloneAllocSource* __ret_ptr);
}
inline ::from::CloneAllocSource CloneAllocSource::create(rs::StrRef s) {
  crubit::Slot<::from::CloneAllocSource> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_create(s, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" rs::StrRef __crubit_thunk_get_uvalue(
    ::from::CloneAllocSource const&);
}
inline rs::StrRef CloneAllocSource::get_value() const& $(__anon1)
    CRUBIT_LIFETIME_BOUND {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_get_uvalue(self);
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_into_ufrom_ugolden_x0000003a_x0000003aCloneAllocType(
    ::from::CloneAllocSource*, ::from::CloneAllocType* __ret_ptr);
}
inline CloneAllocSource::operator ::from::CloneAllocType() {
  auto&& self = *this;
  crubit::Slot self_slot((::std::move(self)));
  crubit::Slot<::from::CloneAllocType> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::
      __crubit_thunk_into_ufrom_ugolden_x0000003a_x0000003aCloneAllocType(
          self_slot.Get(), __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void CloneAllocSource::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(CloneAllocSource, value));
}
static_assert(
    sizeof(CloneAllocType) == 24,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(CloneAllocType) == 8,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void __crubit_thunk_drop(::from::CloneAllocType&);
}
inline CloneAllocType::~CloneAllocType() {
  __crubit_internal::__crubit_thunk_drop(*this);
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_clone(::from::CloneAllocType const&,
                                     ::from::CloneAllocType* __ret_ptr);
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_clone_ufrom(::from::CloneAllocType&,
                                           ::from::CloneAllocType const&);
}
inline ::from::CloneAllocType::CloneAllocType(const CloneAllocType& other) {
  __crubit_internal::__crubit_thunk_clone(other, this);
}
inline ::from::CloneAllocType& ::from::CloneAllocType::operator=(
    const CloneAllocType& other) {
  if (this != &other) {
    __crubit_internal::__crubit_thunk_clone_ufrom(*this, other);
  }
  return *this;
}
namespace __crubit_internal {
extern "C" rs::StrRef __crubit_thunk_get_uvalue(::from::CloneAllocType const&);
}
inline rs::StrRef CloneAllocType::get_value() const& $(__anon1)
    CRUBIT_LIFETIME_BOUND {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_get_uvalue(self);
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_from_ufrom_ugolden_x0000003a_x0000003aCloneAllocSource(
    ::from::CloneAllocSource*, ::from::CloneAllocType* __ret_ptr);
}
inline CloneAllocType::CloneAllocType(::from::CloneAllocSource value) {
  crubit::Slot value_slot((::std::move(value)));
  __crubit_internal::
      __crubit_thunk_from_ufrom_ugolden_x0000003a_x0000003aCloneAllocSource(
          value_slot.Get(), this);
}
inline void CloneAllocType::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(CloneAllocType, value));
}
static_assert(
    sizeof(CloneCopySource) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(CloneCopySource) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<CloneCopySource>);
static_assert(
    ::std::is_trivially_move_constructible_v<::from::CloneCopySource>);
static_assert(::std::is_trivially_move_assignable_v<::from::CloneCopySource>);
static_assert(
    ::std::is_trivially_copy_constructible_v<::from::CloneCopySource>);
static_assert(::std::is_trivially_copy_assignable_v<::from::CloneCopySource>);
namespace __crubit_internal {
extern "C" void
__crubit_thunk_into_ufrom_ugolden_x0000003a_x0000003aCloneCopyType(
    ::from::CloneCopySource*, ::from::CloneCopyType* __ret_ptr);
}
inline CloneCopySource::operator ::from::CloneCopyType() {
  auto& self = const_cast<::std::remove_cvref_t<decltype(*this)>&>(*this);
  crubit::Slot<::from::CloneCopyType> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::
      __crubit_thunk_into_ufrom_ugolden_x0000003a_x0000003aCloneCopyType(
          &self, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void CloneCopySource::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(CloneCopySource, __field0));
}
static_assert(
    sizeof(CloneCopyType) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(CloneCopyType) == 4,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void __crubit_thunk_default(::from::CloneCopyType* __ret_ptr);
}
inline ::from::CloneCopyType::CloneCopyType() {
  __crubit_internal::__crubit_thunk_default(this);
}
static_assert(::std::is_trivially_destructible_v<CloneCopyType>);
static_assert(::std::is_trivially_move_constructible_v<::from::CloneCopyType>);
static_assert(::std::is_trivially_move_assignable_v<::from::CloneCopyType>);
static_assert(::std::is_trivially_copy_constructible_v<::from::CloneCopyType>);
static_assert(::std::is_trivially_copy_assignable_v<::from::CloneCopyType>);
namespace __crubit_internal {
extern "C" void
__crubit_thunk_from_ufrom_ugolden_x0000003a_x0000003aCloneCopySource(
    ::from::CloneCopySource*, ::from::CloneCopyType* __ret_ptr);
}
inline CloneCopyType::CloneCopyType(::from::CloneCopySource value) {
  __crubit_internal::
      __crubit_thunk_from_ufrom_ugolden_x0000003a_x0000003aCloneCopySource(
          &value, this);
}
inline void CloneCopyType::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(CloneCopyType, __field0));
}
static_assert(
    sizeof(CollidingConstructor) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(CollidingConstructor) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<CollidingConstructor>);
static_assert(
    ::std::is_trivially_move_constructible_v<::from::CollidingConstructor>);
static_assert(
    ::std::is_trivially_move_assignable_v<::from::CollidingConstructor>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_from_uusize(
    ::std::uintptr_t, ::from::CollidingConstructor* __ret_ptr);
}
inline CollidingConstructor::CollidingConstructor(::std::uintptr_t value) {
  __crubit_internal::__crubit_thunk_from_uusize(value, this);
}
inline void CollidingConstructor::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(CollidingConstructor, value));
}
static_assert(
    sizeof(LoopA) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(LoopA) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<LoopA>);
static_assert(::std::is_trivially_move_constructible_v<::from::LoopA>);
static_assert(::std::is_trivially_move_assignable_v<::from::LoopA>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_into_ufrom_ugolden_x0000003a_x0000003aLoopB(
    ::from::LoopA*, ::from::LoopB* __ret_ptr);
}
inline LoopA::operator ::from::LoopB() {
  auto&& self = *this;
  crubit::Slot<::from::LoopB> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_into_ufrom_ugolden_x0000003a_x0000003aLoopB(
      &self, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_from_ufrom_ugolden_x0000003a_x0000003aLoopB(
    ::from::LoopB*, ::from::LoopA* __ret_ptr);
}
inline LoopA::LoopA(::from::LoopB value) {
  __crubit_internal::__crubit_thunk_from_ufrom_ugolden_x0000003a_x0000003aLoopB(
      &value, this);
}
inline void LoopA::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(LoopA, __field0));
}
static_assert(
    sizeof(LoopB) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(LoopB) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<LoopB>);
static_assert(::std::is_trivially_move_constructible_v<::from::LoopB>);
static_assert(::std::is_trivially_move_assignable_v<::from::LoopB>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_into_ufrom_ugolden_x0000003a_x0000003aLoopA(
    ::from::LoopB*, ::from::LoopA* __ret_ptr);
}
inline LoopB::operator ::from::LoopA() {
  auto&& self = *this;
  crubit::Slot<::from::LoopA> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_into_ufrom_ugolden_x0000003a_x0000003aLoopA(
      &self, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_from_ufrom_ugolden_x0000003a_x0000003aLoopA(
    ::from::LoopA*, ::from::LoopB* __ret_ptr);
}
inline LoopB::LoopB(::from::LoopA value) {
  __crubit_internal::__crubit_thunk_from_ufrom_ugolden_x0000003a_x0000003aLoopA(
      &value, this);
}
inline void LoopB::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(LoopB, __field0));
}
static_assert(
    sizeof(NoCloneCopyDropSource) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(NoCloneCopyDropSource) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<NoCloneCopyDropSource>);
static_assert(
    ::std::is_trivially_move_constructible_v<::from::NoCloneCopyDropSource>);
static_assert(
    ::std::is_trivially_move_assignable_v<::from::NoCloneCopyDropSource>);
namespace __crubit_internal {
extern "C" void
__crubit_thunk_into_ufrom_ugolden_x0000003a_x0000003aNoCloneCopyDropType(
    ::from::NoCloneCopyDropSource*, ::from::NoCloneCopyDropType* __ret_ptr);
}
inline NoCloneCopyDropSource::operator ::from::NoCloneCopyDropType() {
  auto&& self = *this;
  crubit::Slot<::from::NoCloneCopyDropType> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::
      __crubit_thunk_into_ufrom_ugolden_x0000003a_x0000003aNoCloneCopyDropType(
          &self, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void NoCloneCopyDropSource::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(NoCloneCopyDropSource, __field0));
}
static_assert(
    sizeof(NoCloneCopyDropType) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(NoCloneCopyDropType) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<NoCloneCopyDropType>);
static_assert(
    ::std::is_trivially_move_constructible_v<::from::NoCloneCopyDropType>);
static_assert(
    ::std::is_trivially_move_assignable_v<::from::NoCloneCopyDropType>);
namespace __crubit_internal {
extern "C" void
__crubit_thunk_from_ufrom_ugolden_x0000003a_x0000003aNoCloneCopyDropSource(
    ::from::NoCloneCopyDropSource*, ::from::NoCloneCopyDropType* __ret_ptr);
}
inline NoCloneCopyDropType::NoCloneCopyDropType(
    ::from::NoCloneCopyDropSource value) {
  __crubit_internal::
      __crubit_thunk_from_ufrom_ugolden_x0000003a_x0000003aNoCloneCopyDropSource(
          &value, this);
}
inline void NoCloneCopyDropType::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(NoCloneCopyDropType, __field0));
}
static_assert(
    sizeof(NoCloneDefaultSource) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(NoCloneDefaultSource) == 4,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void __crubit_thunk_default(::from::NoCloneDefaultSource* __ret_ptr);
}
inline ::from::NoCloneDefaultSource::NoCloneDefaultSource() {
  __crubit_internal::__crubit_thunk_default(this);
}
static_assert(::std::is_trivially_destructible_v<NoCloneDefaultSource>);
static_assert(
    ::std::is_trivially_move_constructible_v<::from::NoCloneDefaultSource>);
static_assert(
    ::std::is_trivially_move_assignable_v<::from::NoCloneDefaultSource>);
namespace __crubit_internal {
extern "C" void
__crubit_thunk_into_ufrom_ugolden_x0000003a_x0000003aNoCloneDefaultType(
    ::from::NoCloneDefaultSource*, ::from::NoCloneDefaultType* __ret_ptr);
}
inline NoCloneDefaultSource::operator ::from::NoCloneDefaultType() {
  auto&& self = *this;
  crubit::Slot<::from::NoCloneDefaultType> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::
      __crubit_thunk_into_ufrom_ugolden_x0000003a_x0000003aNoCloneDefaultType(
          &self, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void NoCloneDefaultSource::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(NoCloneDefaultSource, __field0));
}
static_assert(
    sizeof(NoCloneDefaultType) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(NoCloneDefaultType) == 4,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void __crubit_thunk_default(::from::NoCloneDefaultType* __ret_ptr);
}
inline ::from::NoCloneDefaultType::NoCloneDefaultType() {
  __crubit_internal::__crubit_thunk_default(this);
}
static_assert(::std::is_trivially_destructible_v<NoCloneDefaultType>);
static_assert(
    ::std::is_trivially_move_constructible_v<::from::NoCloneDefaultType>);
static_assert(
    ::std::is_trivially_move_assignable_v<::from::NoCloneDefaultType>);
namespace __crubit_internal {
extern "C" void
__crubit_thunk_from_ufrom_ugolden_x0000003a_x0000003aNoCloneDefaultSource(
    ::from::NoCloneDefaultSource*, ::from::NoCloneDefaultType* __ret_ptr);
}
inline NoCloneDefaultType::NoCloneDefaultType(
    ::from::NoCloneDefaultSource value) {
  __crubit_internal::
      __crubit_thunk_from_ufrom_ugolden_x0000003a_x0000003aNoCloneDefaultSource(
          &value, this);
}
inline void NoCloneDefaultType::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(NoCloneDefaultType, __field0));
}
static_assert(
    sizeof(NotFfiSafe) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(NotFfiSafe) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<NotFfiSafe>);
static_assert(::std::is_trivially_move_constructible_v<::from::NotFfiSafe>);
static_assert(::std::is_trivially_move_assignable_v<::from::NotFfiSafe>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_create(::from::NotFfiSafe* __ret_ptr);
}
inline ::from::NotFfiSafe NotFfiSafe::create() {
  crubit::Slot<::from::NotFfiSafe> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_create(__return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_into_ui32(::from::NotFfiSafe*);
}
inline NotFfiSafe::operator ::std::int32_t() {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_into_ui32(&self);
}
inline void NotFfiSafe::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(NotFfiSafe, __field0));
}
static_assert(
    sizeof(Opaque) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(Opaque) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<Opaque>);
static_assert(::std::is_trivially_move_constructible_v<::from::Opaque>);
static_assert(::std::is_trivially_move_assignable_v<::from::Opaque>);
namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_into_ui32(::from::Opaque*);
}
inline Opaque::operator ::std::int32_t() {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_into_ui32(&self);
}
namespace __crubit_internal {
extern "C" ::std::int64_t __crubit_thunk_into_ui64(::from::Opaque*);
}
inline Opaque::operator ::std::int64_t() {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_into_ui64(&self);
}
namespace __crubit_internal {
extern "C" rs::StrRef
__crubit_thunk_into_u_x00000026_x00000027static_x00000020str(::from::Opaque*);
}
inline Opaque::operator rs::StrRef() {
  auto&& self = *this;
  return __crubit_internal::
      __crubit_thunk_into_u_x00000026_x00000027static_x00000020str(&self);
}
namespace __crubit_internal {
extern "C" ::std::int16_t __crubit_thunk_into_ui16(::from::Opaque*);
}
inline Opaque::operator ::std::int16_t() {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_into_ui16(&self);
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_into_ufrom_ugolden_x0000003a_x0000003aOpaqueRef_x0000003c_x00000027static_x0000003e(
    ::from::Opaque*, ::from::OpaqueRef* __ret_ptr);
}
inline Opaque::operator ::from::OpaqueRef() {
  auto&& self = *this;
  crubit::Slot<::from::OpaqueRef> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::
      __crubit_thunk_into_ufrom_ugolden_x0000003a_x0000003aOpaqueRef_x0000003c_x00000027static_x0000003e(
          &self, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void Opaque::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(Opaque, __field0));
}
static_assert(
    sizeof(OpaqueRef) == 16,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(OpaqueRef) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<OpaqueRef>);
static_assert(::std::is_trivially_move_constructible_v<::from::OpaqueRef>);
static_assert(::std::is_trivially_move_assignable_v<::from::OpaqueRef>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_create(rs::StrRef, ::from::OpaqueRef* __ret_ptr);
}
inline ::from::OpaqueRef OpaqueRef::create(rs::StrRef s) {
  crubit::Slot<::from::OpaqueRef> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_create(s, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" rs::StrRef __crubit_thunk_get_uarg(::from::OpaqueRef const&);
}
inline rs::StrRef OpaqueRef::get_arg() const {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_get_uarg(self);
}
namespace __crubit_internal {
extern "C" rs::StrRef __crubit_thunk_into_u_x00000026_x00000027a_x00000020str(
    ::from::OpaqueRef*);
}
inline OpaqueRef::operator rs::StrRef() {
  auto&& self = *this;
  return __crubit_internal::
      __crubit_thunk_into_u_x00000026_x00000027a_x00000020str(&self);
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_from_ufrom_ugolden_x0000003a_x0000003aOpaque(
    ::from::Opaque*, ::from::OpaqueRef* __ret_ptr);
}
inline OpaqueRef::OpaqueRef(::from::Opaque value) {
  __crubit_internal::
      __crubit_thunk_from_ufrom_ugolden_x0000003a_x0000003aOpaque(&value, this);
}
inline void OpaqueRef::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(OpaqueRef, __field0));
}
}  // namespace from

#pragma clang diagnostic pop
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_KNOWN_TRAITS_FROM_FROM_GOLDEN
