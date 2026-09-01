// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_SUPPORT_PROTOBUF_RUST_H_
#define THIRD_PARTY_CRUBIT_SUPPORT_PROTOBUF_RUST_H_

#include <cstddef>
#include <type_traits>
#include <utility>

namespace proto2 {

class MessageLite;

// `proto2::Rust<T>` (aliased as `proto::Rust<T>`) wraps a Rust-owned Protobuf
// message of C++ type `T`.
//
// In Rust, a Protobuf message (using the C++ kernel) is an owned handle to a
// heap-allocated C++ message object. In C++, `proto::Rust<T>` represents this
// owned Rust message when embedded in a struct field or other layout-compatible
// positions across the FFI boundary.
//
// Characteristics and Invariants:
// - Layout compatibility: Consists of a single pointer `T* ptr_`, matching the
//   memory layout of Rust's Protobuf message handle.
// - Non-nullable: A `proto::Rust<T>` always points to a valid instance of `T`.
// - Ownership: Destroys the owned C++ message via `delete` upon destruction.
// - Movable in C++: Transfers ownership via `Rust(Rust&&)` and sets the
//   moved-from pointer to `nullptr`.
template <typename T>
  requires std::is_base_of_v<::google::protobuf::MessageLite, T>
class Rust final {
 public:
  using element_type = T;
  using pointer = T*;

  // Constructs an instance of `T` on the heap forwarding the provided arguments
  // (or default-constructs if no arguments are provided).
  template <typename... Args,
            typename = std::enable_if_t<std::is_constructible_v<T, Args...>>>
  explicit(sizeof...(Args) > 0) Rust(Args&&... args)
      : ptr_(new T(std::forward<Args>(args)...)) {}

  Rust(std::nullptr_t) = delete;

  ~Rust() { delete ptr_; }

  Rust(const Rust&) = delete;
  Rust& operator=(const Rust&) = delete;
  Rust(Rust&& other) noexcept : ptr_(std::exchange(other.ptr_, nullptr)) {}
  Rust& operator=(Rust&& other) noexcept {
    if (this != &other) {
      delete ptr_;
      ptr_ = std::exchange(other.ptr_, nullptr);
    }
    return *this;
  }

  T* _Nonnull get() noexcept { return ptr_; }
  const T* _Nonnull get() const noexcept { return ptr_; }

  T* _Nonnull operator->() noexcept { return ptr_; }
  const T* _Nonnull operator->() const noexcept { return ptr_; }

  T& operator*() noexcept { return *ptr_; }
  const T& operator*() const noexcept { return *ptr_; }

 private:
  T* _Nonnull ptr_;
};

}  // namespace proto2

namespace proto {
using ::proto2::Rust;
}  // namespace proto

#endif  // THIRD_PARTY_CRUBIT_SUPPORT_PROTOBUF_RUST_H_
