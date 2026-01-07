// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_DYN_CALLABLE_H_
#define THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_DYN_CALLABLE_H_

#include <cstddef>
#include <cstdint>
#include <cstring>
#include <exception>
#include <type_traits>
#include <utility>

#include "absl/base/nullability.h"
#include "support/bridge.h"

namespace rs_std {

template <class Sig>
class DynCallable;

namespace internal_dyn_callable {

template <class Sig>
struct DynCallableAbi;

// Storage for holding the `ZeroableCallable<dyn Trait>`.
//
// This type is only intended for passing around the in-memory representation,
// and does not provide usable constructors / copy / move operations, etc.
struct UnmanagedZeroableCallable {
  void SetZero() {
    repr[0] = 0;
    repr[1] = 0;
  }

  uintptr_t repr[2] = {0, 0};
};

// A discriminator when calling the "manager" function that describes operation
// type-erased operation should be invoked.
//
// This type is intended to be an ABI compatible duplicate of
// `absl::internal_any_invocable::FunctionToCall`.
enum class FunctionToCall : unsigned char {
  dispose = 0,
  relocate_from_to = 1,
  relocate_from_to_and_query_rust = 2,
};

// The type for functions issuing lifetime-related operations: move and dispose.
// When `do_dispose` is true, `from` is disposed and `to` is ignored.
// When `do_dispose` is false, `from` is moved to `to`. When this happens,
// `to` should already be empty.
//
// Note that while this type is useful by itself for Rust and Crubit purposes,
// it serves the dual purpose that it can be reused verbatim within the
// implementation details of absl::AnyInvocable.
using ManagerType = void(FunctionToCall,
                         UnmanagedZeroableCallable* absl_nonnull /*from*/,
                         UnmanagedZeroableCallable* absl_nonnull /*to*/);

// Storage type for managing relocation and destruction of the
// `ZeroableCallable<dyn Trait>` received from Rust.
class ManagedZeroableCallable : protected UnmanagedZeroableCallable {
 protected:
  explicit ManagedZeroableCallable(
      UnmanagedZeroableCallable unmanaged_zeroable_callable,
      ManagerType manager)
      : UnmanagedZeroableCallable(unmanaged_zeroable_callable),
        manager_(manager) {}

  explicit ManagedZeroableCallable()
      : manager_([](FunctionToCall, UnmanagedZeroableCallable*,
                    UnmanagedZeroableCallable*) {}) {}

  ~ManagedZeroableCallable() { manager_(FunctionToCall::dispose, this, this); }

  ManagedZeroableCallable(ManagedZeroableCallable&) = delete;
  ManagedZeroableCallable& operator=(ManagedZeroableCallable&) = delete;

  ManagedZeroableCallable(ManagedZeroableCallable&& other) {
    *this = std::forward<ManagedZeroableCallable>(other);
  }

  ManagedZeroableCallable& operator=(ManagedZeroableCallable&& other) {
    // Dispose this.
    manager_(FunctionToCall::dispose, this, this);

    // Move other into this.
    other.manager_(FunctionToCall::relocate_from_to, /*from=*/&other,
                   /*to=*/this);
    manager_ = other.manager_;

    // Put other in the moved-from state. Since the moved-from state is valid
    // for moving and disposing, it may keep its manager.
    other.SetZero();

    return *this;
  }

  ManagerType* absl_nonnull manager_;
};

// The type for functions issuing the actual invocation of the object.
// A pointer to such a function is contained in each DynCallable instance.
template <class ReturnType, class... P>
using InvokerType = ReturnType(UnmanagedZeroableCallable*, P...);

// Partially specialized class that wraps ManagedZeroableCallable and handles
// operator().
template <class Sig>
class Impl {};

// Raises a fatal error when the DynCallable is invoked after a move.
template <class ReturnType, class... P>
inline ReturnType InvokedAfterMove(UnmanagedZeroableCallable*, P...) {
  std::terminate();
}

#define CRUBIT_INTERNAL_RUST_ANY_CALLABLE_IMPL(qual)                          \
  template <class ReturnType, class... P>                                     \
  class Impl<ReturnType(P...) qual> : public ManagedZeroableCallable {        \
   protected:                                                                 \
    friend struct DynCallableAbi<ReturnType(P...) qual>;                      \
    using InvokerType = InvokerType<ReturnType, P...>;                        \
                                                                              \
    explicit Impl(UnmanagedZeroableCallable unmanaged_zeroable_callable,      \
                  ManagerType manager, InvokerType* invoker)                  \
        : ManagedZeroableCallable(unmanaged_zeroable_callable, manager),      \
          invoker_(invoker) {}                                                \
                                                                              \
   public:                                                                    \
    Impl() = default;                                                         \
    Impl(const Impl& other) = delete;                                         \
    Impl& operator=(const Impl& other) = delete;                              \
    Impl(Impl&& other) { *this = std::forward<Impl>(other); }                 \
    Impl& operator=(Impl&& other) {                                           \
      ManagedZeroableCallable::operator=(std::move(other));                   \
      /*other may keep its invoker_, but invoking will safely panic in Rust*/ \
      invoker_ = other.invoker_;                                              \
      return *this;                                                           \
    }                                                                         \
                                                                              \
    ReturnType operator()(P... args) qual {                                   \
      using Self = std::remove_pointer_t<decltype(this)>;                     \
                                                                              \
      InvokerType* invoker_copy = invoker_;                                   \
      if constexpr (std::is_rvalue_reference_v<Self>) {                       \
        invoker_ = InvokedAfterMove<ReturnType, P...>;                        \
      }                                                                       \
      if constexpr (std::is_const_v<Self>) {                                  \
        return invoker_copy(const_cast<Impl*>(this), args...);                \
      } else {                                                                \
        return invoker_copy(this, args...);                                   \
      }                                                                       \
    }                                                                         \
                                                                              \
    bool HasValue() const { return invoker_ != nullptr; }                     \
                                                                              \
   protected:                                                                 \
    InvokerType* invoker_ = nullptr;                                          \
  };

CRUBIT_INTERNAL_RUST_ANY_CALLABLE_IMPL(const)
CRUBIT_INTERNAL_RUST_ANY_CALLABLE_IMPL()
CRUBIT_INTERNAL_RUST_ANY_CALLABLE_IMPL(&&)

#undef CRUBIT_INTERNAL_RUST_ANY_CALLABLE_IMPL

template <class Sig>
struct DynCallableAbi {
  using Value = DynCallable<Sig>;

  static constexpr size_t kSize = 24;

  void Encode(Value value, crubit::Encoder& encoder) && {
    crubit::TransmuteAbi<UnmanagedZeroableCallable>().Encode(value.state,
                                                             encoder);
  }

  Value Decode(crubit::Decoder& decoder) && {
    auto unmanaged_zero_callable =
        crubit::TransmuteAbi<UnmanagedZeroableCallable>().Decode(decoder);
    auto manager = crubit::TransmuteAbi<ManagerType*>().Decode(decoder);
    return DynCallable<Sig>(unmanaged_zero_callable, manager, invoker);
  }

  explicit DynCallableAbi(Value::Impl::InvokerType* invoker)
      : invoker(invoker) {}

  Value::Impl::InvokerType* invoker;
};

}  // namespace internal_dyn_callable

// rs_std::DynCallable
//
// `rs_std::DynCallable` is a move-only functional wrapper type, like
// `absl::AnyInvocable`. It assumes ownership of a Rust `Box<dyn Trait>`,
// where `Trait` is one of `Fn`, `FnMut`, or `FnOnce`.
//
// It is similar to `absl::AnyInvocable` in the following ways:
// * It is move-only and cannot be copied, because it owns the callable.
// * It can be default constructed to an empty state for delayed initialization,
//   but invoking it results in program termination. You can check for emptiness
//   using `operator bool()`.
// * It respects `const` qualifiers and rvalue-reference qualifiers as part of
//   `Sig`, applying them to `operator()`. That is, `Fn()` maps to
//   `void() const`, and `FnOnce()` maps to `void() &&`.
// * It is cheaply convertible to `absl::AnyInvocable` because it uses the same
//   underlying representation, only with stricter invariants.
//
// Unlike `absl::AnyInvocable`, it may only store Rust callables, and so
// non-empty `DynCallable`s can only be obtained by exposing a C++ function
// with a `DynCallable` parameter to Rust via Crubit. Passing a moved-from or
// default constructed `DynCallable` back to Rust will result in a Rust
// `Box<dyn Trait>` that panics when called.
template <class Sig>
class DynCallable : private internal_dyn_callable::Impl<Sig> {
 private:
  using Impl = internal_dyn_callable::Impl<Sig>;

  // DynCallableAbi<Sig> is a friend so it can access the DynCallable
  // constructor below.
  friend struct internal_dyn_callable::DynCallableAbi<Sig>;

  // Private constructor, only intended to be called by DynCallableAbi<Sig>.
  //
  // * `unmanaged_zeroable_callable` should either be zeros, or be a verbatim
  //   memcpy of the representation of `Box<dyn Trait>` in Rust, e.g., two
  //   pointers because it is a wide pointer with a vtable pointer as the
  //   metadata.
  // * `manager` is a function pointer with the `ManagerType` signature, and
  //   knows how to move and dispose of the `unmanaged_zeroable_callable`.
  // * `invoker` is a C++ function pointer that knows how to invoke the
  //   `Box<dyn Trait>`.
  explicit DynCallable(internal_dyn_callable::UnmanagedZeroableCallable
                           unmanaged_zeroable_callable,
                       internal_dyn_callable::ManagerType manager,
                       Impl::InvokerType* invoker)
      : Impl(unmanaged_zeroable_callable, manager, invoker) {}

 public:
  // Constructors

  // Constructs the `DynCallable` in an empty state.
  // Invoking it results in undefined behavior.
  DynCallable() noexcept = default;
  DynCallable(std::nullptr_t) noexcept {}  // NOLINT

  // DynCallable is not copyable.
  DynCallable(DynCallable& other) = delete;
  DynCallable& operator=(DynCallable& other) = delete;

  DynCallable(DynCallable&& other) = default;
  DynCallable& operator=(DynCallable&& other) = default;

  // Destructor

  // If not empty, destroys the target.
  ~DynCallable() = default;

  // rs_std::DynCallable::swap()
  //
  // Exchanges the targets of `*this` and `other`.
  void swap(DynCallable& other) noexcept { std::swap(*this, other); }

  // rs_std::DynCallable::operator bool()
  //
  // Returns `true` if `*this` is not empty.
  //
  // Invoking an empty `DynCallable` results in undefined behavior.
  explicit operator bool() const noexcept { return this->HasValue(); }

  using Impl::operator();

  // Returns `true` if `f` is empty.
  friend bool operator==(const DynCallable& f, std::nullptr_t) noexcept {
    return !f.HasValue();
  }

  // Returns `true` if `f` is empty.
  friend bool operator==(std::nullptr_t, const DynCallable& f) noexcept {
    return !f.HasValue();
  }

  // Returns `false` if `f` is empty.
  friend bool operator!=(const DynCallable& f, std::nullptr_t) noexcept {
    return f.HasValue();
  }

  // Returns `false` if `f` is empty.
  friend bool operator!=(std::nullptr_t, const DynCallable& f) noexcept {
    return f.HasValue();
  }

  // swap()
  //
  // Exchanges the targets of `f1` and `f2`.
  friend void swap(DynCallable& f1, DynCallable& f2) noexcept { f1.swap(f2); }
};

}  // namespace rs_std

#endif  // THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_DYN_CALLABLE_H_
