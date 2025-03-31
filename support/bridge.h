// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_SUPPORT_BRIDGE_H_
#define THIRD_PARTY_CRUBIT_SUPPORT_BRIDGE_H_

#include <cstddef>
#include <cstring>
#include <optional>
#include <tuple>
#include <type_traits>
#include <utility>

namespace crubit {

// TODO(b/403623652): Document these schemas better.

// Schema for encoding a value by value.
struct ByTransmute;

// Schema for encoding a value by bridging it.
template <typename... T>
struct ByBridge;

// TODO(b/403626070): Write a CRTP enforcing class for CrubitAbiTrait.

template <typename T>
constexpr bool dependent_false = false;

// A type trait that describes how to encode and decode a value of type `T`
// using a schema of type `S`.
//
// Crubit allows for types to have several ways of being encoded and decoded,
// where each way is described by a marker type `S`, called a "schema".
// This essentially allows for type trait impls that are parameterized by the
// schema, allowing for a single type to be encoded and decoded in multiple
// ways, with the impl decided by how Crubit specifies the schema at the call
// site. The two schemas currently understood by Crubit are `ByTransmute` and
// `ByBridge`.
//
// `ByTransmute` is a schema that describes literally copying the memory of the
// value using `memcpy`. The `CrubitAbiTrait` already implements all movable
// types with this schema.
//
// `ByBridge` is a schema that describes bridging the value across the Rust-C++
// boundary, and allows for bridging templated types in a composable manner.
//
// To explain how to bridge custom types, let's walk through the `std::optional`
// bridging implementation:
//
// ```
// template <typename T, typename S>
// struct CrubitAbiTrait<std::optional<T>, ByBridge<S>> {
//   static constexpr size_t kSize = sizeof(bool) + CrubitAbiTrait<T, S>::kSize;
//   static void Encode(std::optional<T> value, Encoder& encoder) {
//     if (value.has_value()) {
//       encoder.EncodeTransmute(true);
//       encoder.Encode<T, S>(*std::move(value));
//     } else {
//       encoder.EncodeTransmute(false);
//     }
//   }
//   static std::optional<T> Decode(Decoder& decoder) {
//     if (!decoder.DecodeTransmute<bool>()) {
//       return std::nullopt;
//     }
//     return decoder.Decode<T, S>();
//   }
// };
// ```
//
// The most critical part of bridging is that it's composable, meaning that
// the way we bridge the inner type `T` is not determined by `std::optional`,
// but instead by some schema `S` that we pass in. The marker type `ByBridge<S>`
// says "when ByBridge<S> is used, this is the specialization of
// `CrubitAbiTrait` that should be used." There are three things the user must
// provide, each corresponding to an item in the Rust `CrubitAbi` trait:
//
// 1. kSize: The size of the encoded value. This is used to allocate the buffer.
// 2. Encode: A function that encodes the value into the buffer.
// 3. Decode: A function that decodes the value from the buffer.
//
// The implementation requires three invariants:
//
// 1. Encoding and decoding must be inverses of each other.
// 2. Encoding and decoding must not read or write more bytes than the size
//    declared in kSize.
// 3. The type on the Rust side must implement the `CrubitAbi` trait with the
//    same schema (as understood by Crubit), the same size, and encoding and
//    decoding functions that are semantically identical.
//
// If any of these invariants are violated, the behavior is unspecified. It may
// work, or it may cause undefined behavior depending on your implementation.
template <typename T, typename S>
struct CrubitAbiTrait {
  static_assert(dependent_false<T>,
                "CrubitAbiTrait not implemented for type T; are you using the "
                "correct schema?");
};

template <typename T, typename S>
constexpr size_t CrubitAbiSize() {
  return CrubitAbiTrait<T, S>::kSize;
}

namespace internal {

template <typename T, typename S>
void Encode(unsigned char* buf, T value);

template <typename T, typename S>
T Decode(const unsigned char* buf);

}  // namespace internal

// A wrapper around a buffer that tracks which parts of a buffer have already
// been written to.
class Encoder {
  explicit Encoder(size_t remaining_bytes, unsigned char* buf)
      : remaining_bytes_(remaining_bytes), buf_(buf) {}

 public:
  // Encodes a value by a provided schema.
  template <typename T, typename S>
  void Encode(T value) & {
    CrubitAbiTrait<T, S>::Encode(std::move(value), *this);
  }

  // Encodes a value via `memcpy`.
  template <typename T>
  void EncodeTransmute(T value) & {
    CrubitAbiTrait<T, ByTransmute>::Encode(std::move(value), *this);
  }

  void* Next(size_t size) & {
    remaining_bytes_ -= size;
    return buf_ + remaining_bytes_;
  }

 private:
  template <typename T, typename S>
  friend void internal::Encode(unsigned char* buf, T value);
  // The number of bytes remaining in the buffer.
  size_t remaining_bytes_;
  unsigned char* buf_;
};

// A wrapper around a buffer that tracks which parts of a buffer have already
// been read from.
class Decoder {
  explicit Decoder(size_t remaining_bytes, const unsigned char* buf)
      : remaining_bytes_(remaining_bytes), buf_(buf) {}

 public:
  // Decodes a value by a provided schema. The caller must ensure that the
  // buffer contains a value that was encoded with the same schema.
  template <typename T, typename S>
  T Decode() & {
    return CrubitAbiTrait<T, S>::Decode(*this);
  }

  // Decodes a value via `memcpy`. The caller must ensure that the buffer
  // contains a value that was encoded with ByTransmute.
  template <typename T>
  T DecodeTransmute() & {
    return CrubitAbiTrait<T, ByTransmute>::Decode(*this);
  }

  const void* Next(size_t size) & {
    remaining_bytes_ -= size;
    return buf_ + remaining_bytes_;
  }

 private:
  template <typename T, typename S>
  friend T internal::Decode(const unsigned char* buf);
  // The number of bytes remaining in the buffer.
  size_t remaining_bytes_;
  const unsigned char* buf_;
};

// By value impl
template <typename T>
  requires(std::is_move_constructible_v<T>)
struct CrubitAbiTrait<T, ByTransmute> {
  static constexpr size_t kSize = sizeof(T);
  static void Encode(T value, Encoder& encoder) {
    // Move-construct the value into a type erased buffer, ensuring that value
    // is in a "moved from" state. Then copy the value into the encoder buffer.
    // We use an intermediate buffer and a memcpy to avoid strict aliasing
    // violations. Furthermore, the destructor is not called- this is intended,
    // because we have semantically moved the value into the buffer.
    alignas(T) char buf[sizeof(T)];
    std::memcpy(encoder.Next(sizeof(T)), new (buf) T(std::move(value)),
                sizeof(T));
  }
  static T Decode(Decoder& decoder) {
    alignas(T) char buf[sizeof(T)];
    // Copy the value from the decoder buffer into the intermediate buffer.
    std::memcpy(buf, decoder.Next(sizeof(T)), sizeof(T));
    // Move-construct the value from the buffer.
    return std::move(*reinterpret_cast<T*>(buf));
  }
};

template <typename... Elems, typename... Impls>
  requires(sizeof...(Elems) == sizeof...(Impls))
struct CrubitAbiTrait<std::tuple<Elems...>, ByBridge<Impls...>> {
  static constexpr size_t kSize = (0 + ... + CrubitAbiSize<Elems, Impls>());
  static void Encode(std::tuple<Elems...> value, Encoder& encoder) {
    std::apply(
        [&](Elems&&... args) { (encoder.Encode<Elems, Impls>(args), ...); },
        std::move(value));
  }
  static std::tuple<Elems...> Decode(Decoder& decoder) {
    return std::make_tuple(decoder.Decode<Elems, Impls>()...);
  }
};

template <typename A, typename B, typename ImplA, typename ImplB>
struct CrubitAbiTrait<std::pair<A, B>, ByBridge<ImplA, ImplB>> {
  static constexpr size_t kSize =
      CrubitAbiSize<A, ImplA>() + CrubitAbiSize<B, ImplB>();
  static void Encode(std::pair<A, B> value, Encoder& encoder) {
    encoder.Encode<A, ImplA>(std::move(value.first));
    encoder.Encode<B, ImplB>(std::move(value.second));
  }
  static std::pair<A, B> Decode(Decoder& decoder) {
    return {
        .first = decoder.Decode<A, ImplA>(),
        .second = decoder.Decode<B, ImplB>(),
    };
  }
};

template <typename T, typename S>
struct CrubitAbiTrait<std::optional<T>, ByBridge<S>> {
  static constexpr size_t kSize = sizeof(bool) + CrubitAbiSize<T, S>();
  static void Encode(std::optional<T> value, Encoder& encoder) {
    if (value.has_value()) {
      encoder.EncodeTransmute(true);
      encoder.Encode<T, S>(*std::move(value));
    } else {
      encoder.EncodeTransmute(false);
    }
  }
  static std::optional<T> Decode(Decoder& decoder) {
    if (!decoder.DecodeTransmute<bool>()) {
      return std::nullopt;
    }
    return decoder.Decode<T, S>();
  }
};

namespace internal {

template <typename T, typename S>
void Encode(unsigned char* buf, T value) {
  Encoder encoder(CrubitAbiSize<T, S>(), buf);
  encoder.Encode<T, S>(std::move(value));
}

template <typename T, typename S>
T Decode(const unsigned char* buf) {
  Decoder decoder(CrubitAbiSize<T, S>(), buf);
  return decoder.Decode<T, S>();
}

}  // namespace internal

}  // namespace crubit

#endif  // THIRD_PARTY_CRUBIT_SUPPORT_BRIDGE_H_
