// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// example_crate_golden

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_EXAMPLES_RUST_STD_EXAMPLE_CRATE_GOLDEN
#define THIRD_PARTY_CRUBIT_EXAMPLES_RUST_STD_EXAMPLE_CRATE_GOLDEN

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#pragma clang diagnostic ignored "-Wunused-private-field"
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
#pragma clang diagnostic ignored "-Wignored-attributes"
#include "support/internal/slot.h"
#include "support/rs_std/int.h"
#include "support/rs_std/option.h"
#include "support/rs_std/result.h"
#include "support/rs_std/str_ref.h"

#include <bit>
#include <cstdint>
#include <cstring>
#include <optional>
#include <type_traits>
#include <utility>

#include "support/rs_std/rs_alloc.h"

namespace example_crate {

rs_std::Result<rs_std::Option<rs_std::StrRef>, ::rs::alloc::string::String>
returns_result(bool is_ok);

}

#ifndef _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020StrRef_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020StrRef_x00000020_x0000003e
template <>
struct alignas(8) CRUBIT_INTERNAL_RUST_TYPE(
    "std :: option :: Option < & 'static str >") rs_std::Option<rs_std::StrRef>
    : public rs_std::OptionBase<rs_std::Option<rs_std::StrRef>,
                                rs_std::StrRef> {
 public:
  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  Option(const Option&) = default;
  Option& operator=(const Option&) = default;
  Option(Option&&) = default;
  Option& operator=(Option&&) = default;

  Option(::crubit::UnsafeRelocateTag, Option&& value);
  using base_type =
      rs_std::OptionBase<rs_std::Option<rs_std::StrRef>, rs_std::StrRef>;
  constexpr Option() = default;
  constexpr Option(::std::nullopt_t) noexcept;
  constexpr Option& operator=(::std::nullopt_t) noexcept;
  template <typename U>
    requires(rs_std::OptionForwardConstructible<Option, rs_std::StrRef, U>)
  Option(U&& value) noexcept;
  template <typename U>
    requires(rs_std::OptionForwardConstructible<Option, rs_std::StrRef, U>)
  Option& operator=(U&& value) noexcept;
  template <typename Opt>
    requires(rs_std::OptionFromStdOptional<rs_std::StrRef, Opt>)
  Option(Opt&& value) noexcept;
  template <typename Opt>
    requires(rs_std::OptionFromStdOptional<rs_std::StrRef, Opt>)
  Option& operator=(Opt&& value) noexcept;
  template <typename... Args>
  explicit Option(::std::in_place_t ip, Args&&... args) noexcept;
  ~Option() noexcept = default;

 private:
  friend base_type;
  using tag_type = ::std::uint64_t;
  static constexpr tag_type kNoneVal = 0;
  rs_std::StrRef* some_ptr() noexcept {
    return reinterpret_cast<rs_std::StrRef*>(storage_);
  }
  rs_std::StrRef const* some_const_ptr() const noexcept {
    return reinterpret_cast<rs_std::StrRef const*>(storage_);
  }
  void set_some_tag() noexcept {}
  constexpr void set_none_tag() noexcept { set_tag(kNoneVal); }
  constexpr bool is_none() const noexcept { return tag() == kNoneVal; }
  constexpr ::std::uint64_t tag() const& noexcept;
  constexpr void set_tag(::std::uint64_t tag) noexcept;

 private:
  unsigned char storage_[16];
};
#endif

#ifndef _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020StrRef_x00000020_x0000003e_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020StrRef_x00000020_x0000003e_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e
template <>
struct alignas(8) CRUBIT_INTERNAL_RUST_TYPE(
    "std :: result :: Result < :: core :: option :: Option < & 'static str > , "
    ":: alloc :: string :: String >")
    rs_std::Result<rs_std::Option<rs_std::StrRef>, ::rs::alloc::string::String>
    : public rs_std::ResultBase<rs_std::Result<rs_std::Option<rs_std::StrRef>,
                                               ::rs::alloc::string::String>,
                                rs_std::Option<rs_std::StrRef>,
                                ::rs::alloc::string::String> {
 public:
  // Clone::clone
  Result(const Result&);

  // Clone::clone_from
  rs_std::Result<rs_std::Option<rs_std::StrRef>, ::rs::alloc::string::String>&
  operator=(const Result&);

  Result(::crubit::UnsafeRelocateTag, Result&& value);

 public:
  using base_type =
      rs_std::ResultBase<rs_std::Result<rs_std::Option<rs_std::StrRef>,
                                        ::rs::alloc::string::String>,
                         rs_std::Option<rs_std::StrRef>,
                         ::rs::alloc::string::String>;
  template <typename U>
    requires(rs_std::ResultForwardConstructible<
             Result, rs_std::Option<rs_std::StrRef>, U>)
  explicit constexpr Result(U&& ok) noexcept;
  template <typename U>
    requires(rs_std::ResultForwardConstructible<
             Result, rs_std::Option<rs_std::StrRef>, U>)
  constexpr Result& operator=(U&& ok) noexcept;
  template <typename F>
    requires(
        rs_std::ResultUnexpectedConstructible<::rs::alloc::string::String, F>)
  explicit constexpr Result(rs_std::unexpected<F>&& err) noexcept;
  template <typename F>
    requires(
        rs_std::ResultUnexpectedConstructible<::rs::alloc::string::String, F>)
  constexpr Result& operator=(rs_std::unexpected<F>&& err) noexcept;
  template <typename... Args>
  explicit constexpr Result(::std::in_place_t ip, Args&&... args) noexcept;
  template <typename... Args>
  explicit constexpr Result(rs_std::unexpect_t u, Args&&... args) noexcept;
  ~Result() noexcept;

 private:
  friend base_type;
  bool has_value_impl() const noexcept {
    return tag() == UINT64_C(18446744073709551615);
  }
  rs_std::Option<rs_std::StrRef>* ok_ptr() noexcept {
    return reinterpret_cast<rs_std::Option<rs_std::StrRef>*>(__storage + 8);
  }
  rs_std::Option<rs_std::StrRef> const* ok_const_ptr() const noexcept {
    return reinterpret_cast<rs_std::Option<rs_std::StrRef> const*>(__storage +
                                                                   8);
  }
  ::rs::alloc::string::String* err_ptr() noexcept {
    return reinterpret_cast<::rs::alloc::string::String*>(__storage);
  }
  ::rs::alloc::string::String const* err_const_ptr() const noexcept {
    return reinterpret_cast<::rs::alloc::string::String const*>(__storage);
  }
  void set_ok_tag() noexcept { set_tag(UINT64_C(18446744073709551615)); }
  void set_err_tag() noexcept {}
  constexpr ::std::uint64_t tag() const& noexcept;
  constexpr void set_tag(::std::uint64_t tag) noexcept;

 private:
  unsigned char __storage[24];
};
#endif

namespace example_crate {

namespace __crubit_internal {
extern "C" void __crubit_thunk_returns_uresult(
    bool,
    rs_std::Result<rs_std::Option<rs_std::StrRef>, ::rs::alloc::string::String>*
        __ret_ptr);
}
inline rs_std::Result<rs_std::Option<rs_std::StrRef>,
                      ::rs::alloc::string::String>
returns_result(bool is_ok) {
  crubit::Slot<rs_std::Result<rs_std::Option<rs_std::StrRef>,
                              ::rs::alloc::string::String>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_returns_uresult(is_ok,
                                                    __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

}  // namespace example_crate

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020StrRef_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020StrRef_x00000020_x0000003e
static_assert(
    ::std::is_trivially_copy_constructible_v<rs_std::Option<rs_std::StrRef>>);
static_assert(
    ::std::is_trivially_copy_assignable_v<rs_std::Option<rs_std::StrRef>>);
static_assert(
    ::std::is_trivially_move_constructible_v<rs_std::Option<rs_std::StrRef>>);
static_assert(
    ::std::is_trivially_move_assignable_v<rs_std::Option<rs_std::StrRef>>);
inline rs_std::Option<rs_std::StrRef>::Option(::crubit::UnsafeRelocateTag,
                                              Option&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
static_assert(
    ::std::is_trivially_destructible_v<rs_std::Option<rs_std::StrRef>>);
inline constexpr ::std::uint64_t rs_std::Option<rs_std::StrRef>::tag()
    const& noexcept {
  ::std::array<unsigned char, sizeof(::std::uint64_t)> __bytes = {};
  for (::std::size_t i = 0; i < sizeof(::std::uint64_t); ++i) {
    __bytes[i] = storage_[0 + i];
  }
  return ::std::bit_cast<::std::uint64_t>(__bytes);
}
inline constexpr void rs_std::Option<rs_std::StrRef>::set_tag(
    ::std::uint64_t tag) noexcept {
  auto __bytes =
      ::std::bit_cast<::std::array<unsigned char, sizeof(::std::uint64_t)>>(
          tag);
  for (::std::size_t i = 0; i < sizeof(::std::uint64_t); ++i) {
    storage_[0 + i] = __bytes[i];
  }
}

inline constexpr rs_std::Option<rs_std::StrRef>::Option(
    ::std::nullopt_t) noexcept
    : base_type(::std::nullopt) {}
inline constexpr rs_std::Option<rs_std::StrRef>&
rs_std::Option<rs_std::StrRef>::operator=(::std::nullopt_t) noexcept {
  base_type::operator=(::std::nullopt);
  return *this;
}
template <typename U>
  requires(rs_std::OptionForwardConstructible<rs_std::Option<rs_std::StrRef>,
                                              rs_std::StrRef, U>)
inline rs_std::Option<rs_std::StrRef>::Option(U&& value) noexcept
    : base_type(::std::forward<U>(value)) {}
template <typename U>
  requires(rs_std::OptionForwardConstructible<rs_std::Option<rs_std::StrRef>,
                                              rs_std::StrRef, U>)
inline rs_std::Option<rs_std::StrRef>&
rs_std::Option<rs_std::StrRef>::operator=(U&& value) noexcept {
  base_type::operator=(::std::forward<U>(value));
  return *this;
}
template <typename Opt>
  requires(rs_std::OptionFromStdOptional<rs_std::StrRef, Opt>)
inline rs_std::Option<rs_std::StrRef>::Option(Opt&& value) noexcept
    : base_type(::std::forward<Opt>(value)) {}
template <typename Opt>
  requires(rs_std::OptionFromStdOptional<rs_std::StrRef, Opt>)
inline rs_std::Option<rs_std::StrRef>&
rs_std::Option<rs_std::StrRef>::operator=(Opt&& value) noexcept {
  base_type::operator=(::std::forward<Opt>(value));
  return *this;
}
template <typename... Args>
inline rs_std::Option<rs_std::StrRef>::Option(::std::in_place_t ip,
                                              Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020StrRef_x00000020_x0000003e_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020StrRef_x00000020_x0000003e_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020alloc_x00000020_x0000003a_x0000003a_x00000020string_x00000020_x0000003a_x0000003a_x00000020String_x00000020_x0000003e
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_ustd_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003cstd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003c_x00000026_x00000027static_x00000020str_x0000003e_x0000002c_x00000020std_x0000003a_x0000003astring_x0000003a_x0000003aString_x0000003e(
    rs_std::Result<rs_std::Option<rs_std::StrRef>,
                   ::rs::alloc::string::String> const&,
    rs_std::Result<rs_std::Option<rs_std::StrRef>, ::rs::alloc::string::String>*
        __ret_ptr);
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_ufrom_ustd_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003cstd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003c_x00000026_x00000027static_x00000020str_x0000003e_x0000002c_x00000020std_x0000003a_x0000003astring_x0000003a_x0000003aString_x0000003e(
    rs_std::Result<rs_std::Option<rs_std::StrRef>,
                   ::rs::alloc::string::String>&,
    rs_std::Result<rs_std::Option<rs_std::StrRef>,
                   ::rs::alloc::string::String> const&);
}
inline rs_std::Result<rs_std::Option<rs_std::StrRef>,
                      ::rs::alloc::string::String>::Result(const Result&
                                                               other) {
  __crubit_internal::
      __crubit_thunk_Clone_uclone_ustd_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003cstd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003c_x00000026_x00000027static_x00000020str_x0000003e_x0000002c_x00000020std_x0000003a_x0000003astring_x0000003a_x0000003aString_x0000003e(
          other, this);
}
inline rs_std::Result<rs_std::Option<rs_std::StrRef>,
                      ::rs::alloc::string::String>&
rs_std::Result<rs_std::Option<rs_std::StrRef>,
               ::rs::alloc::string::String>::operator=(const Result& other) {
  if (this != &other) {
    __crubit_internal::
        __crubit_thunk_Clone_uclone_ufrom_ustd_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003cstd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003c_x00000026_x00000027static_x00000020str_x0000003e_x0000002c_x00000020std_x0000003a_x0000003astring_x0000003a_x0000003aString_x0000003e(
            *this, other);
  }
  return *this;
}
inline rs_std::
    Result<rs_std::Option<rs_std::StrRef>, ::rs::alloc::string::String>::Result(
        ::crubit::UnsafeRelocateTag, Result&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
inline rs_std::Result<rs_std::Option<rs_std::StrRef>,
                      ::rs::alloc::string::String>::~Result() noexcept {
  this->Reset();
}
inline constexpr ::std::uint64_t
rs_std::Result<rs_std::Option<rs_std::StrRef>,
               ::rs::alloc::string::String>::tag() const& noexcept {
  std::array<unsigned char, sizeof(::std::uint64_t)> __bytes = {};
  for (std::size_t i = 0; i < sizeof(::std::uint64_t); ++i) {
    __bytes[i] = __storage[0 + i];
  }
  return std::bit_cast<::std::uint64_t>(__bytes);
}
inline constexpr void rs_std::Result<
    rs_std::Option<rs_std::StrRef>,
    ::rs::alloc::string::String>::set_tag(::std::uint64_t tag) noexcept {
  auto __bytes =
      std::bit_cast<std::array<unsigned char, sizeof(::std::uint64_t)>>(tag);
  for (std::size_t i = 0; i < sizeof(::std::uint64_t); ++i) {
    __storage[0 + i] = __bytes[i];
  }
}

template <typename U>
  requires(rs_std::ResultForwardConstructible<
           rs_std::Result<rs_std::Option<rs_std::StrRef>,
                          ::rs::alloc::string::String>,
           rs_std::Option<rs_std::StrRef>, U>)
inline constexpr rs_std::Result<
    rs_std::Option<rs_std::StrRef>,
    ::rs::alloc::string::String>::Result(U&& ok) noexcept
    : base_type(::std::forward<U>(ok)) {}
template <typename U>
  requires(rs_std::ResultForwardConstructible<
           rs_std::Result<rs_std::Option<rs_std::StrRef>,
                          ::rs::alloc::string::String>,
           rs_std::Option<rs_std::StrRef>, U>)
inline constexpr rs_std::Result<rs_std::Option<rs_std::StrRef>,
                                ::rs::alloc::string::String>&
rs_std::Result<rs_std::Option<rs_std::StrRef>,
               ::rs::alloc::string::String>::operator=(U&& ok) noexcept {
  base_type::operator=(::std::forward<U>(ok));
  return *this;
}
template <typename F>
  requires(
      rs_std::ResultUnexpectedConstructible<::rs::alloc::string::String, F>)
inline constexpr rs_std::Result<
    rs_std::Option<rs_std::StrRef>,
    ::rs::alloc::string::String>::Result(rs_std::unexpected<F>&& err) noexcept
    : base_type(::std::move(err)) {}
template <typename F>
  requires(
      rs_std::ResultUnexpectedConstructible<::rs::alloc::string::String, F>)
inline constexpr rs_std::Result<rs_std::Option<rs_std::StrRef>,
                                ::rs::alloc::string::String>&
rs_std::Result<rs_std::Option<rs_std::StrRef>, ::rs::alloc::string::String>::
operator=(rs_std::unexpected<F>&& err) noexcept {
  base_type::operator=(::std::move(err));
  return *this;
}
template <typename... Args>
inline constexpr rs_std::
    Result<rs_std::Option<rs_std::StrRef>, ::rs::alloc::string::String>::Result(
        ::std::in_place_t ip, Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}
template <typename... Args>
inline constexpr rs_std::
    Result<rs_std::Option<rs_std::StrRef>, ::rs::alloc::string::String>::Result(
        rs_std::unexpect_t u, Args&&... args) noexcept
    : base_type(u, ::std::forward<Args>(args)...) {}

#endif

#pragma clang diagnostic pop
#endif  // THIRD_PARTY_CRUBIT_EXAMPLES_RUST_STD_EXAMPLE_CRATE_GOLDEN
