// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_DYN_CALLABLE_H_
#define THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_DYN_CALLABLE_H_

#include <cstddef>
#include <cstring>
#include <exception>

#include "support/annotations.h"
#include "support/bridge.h"

namespace rs_std {

template <class Sig>
class DynCallable;

namespace internal_dyn_callable {

template <class Sig>
struct DynCallableAbi;

// Storage for holding the `Option<Box<dyn Trait>>`.
struct OptionBoxDynTrait {
  alignas(8) unsigned char context[16] = {0};
};

// The type for functions issuing the actual invocation of the object.
// Note that while this type is useful by itself for Rust/Crubit purposes, it
// serves the dual purpose that it can be reused verbatim by absl::AnyInvocable.
using ManagerType = void (*)(bool /*do_dispose*/, void* /*from*/, void* /*to*/);

// Storage type for managing relocation and destruction of the
// `Option<Box<dyn Trait>>` received from Rust.
class Storage {
 protected:
  explicit Storage(OptionBoxDynTrait state, ManagerType manager)
      : state_(state), manager_(manager) {}

 public:
  ~Storage() { manager_(/*do_dispose=*/true, &state_, &state_); }

  Storage(Storage&& other) {
    manager_(/*do_dispose=*/false, &other.state_, &state_);
  }

  Storage(Storage&) = delete;

  Storage& operator=(Storage&& other) {
    manager_(/*do_dispose=*/false, &other.state_, &state_);
    return *this;
  }

  OptionBoxDynTrait state_;
  ManagerType manager_ = [](bool, void*, void*) {};
};

// Partially specialized class that wraps Storage and handles operator().
template <class Sig>
class Impl {};

template <class ReturnType, class... P>
inline ReturnType InvokedAfterMove(void*, P...) {
  std::terminate();
}

#define CRUBIT_INTERNAL_RUST_ANY_CALLABLE_IMPL(qual)            \
  template <class ReturnType, class... P>                       \
  class Impl<ReturnType(P...) qual> : public Storage {          \
   protected:                                                   \
    friend struct DynCallableAbi<ReturnType(P...) qual>;        \
    using InvokerType = ReturnType (*)(void*, P...);            \
                                                                \
    explicit Impl(OptionBoxDynTrait state, ManagerType manager, \
                  InvokerType invoker)                          \
        : Storage(state, manager), invoker_(invoker) {}         \
                                                                \
   public:                                                      \
    Impl() = default;                                           \
    ReturnType operator()(P... args) qual {                     \
      return invoker_(&state_, args...);                        \
    }                                                           \
                                                                \
   private:                                                     \
    InvokerType invoker_ = InvokedAfterMove<ReturnType, P...>;  \
  };

CRUBIT_INTERNAL_RUST_ANY_CALLABLE_IMPL(const)
CRUBIT_INTERNAL_RUST_ANY_CALLABLE_IMPL()
CRUBIT_INTERNAL_RUST_ANY_CALLABLE_IMPL(&&)

#undef CRUBIT_INTERNAL_RUST_ANY_CALLABLE_IMPL

template <class Sig>
struct DynCallableAbi {
  using Value = DynCallable<Sig>;

  static constexpr size_t kSize = sizeof(Value);

  void Encode(Value value, crubit::Encoder& encoder) && {
    crubit::TransmuteAbi<OptionBoxDynTrait>().Encode(value.state, encoder);
  }

  Value Decode(crubit::Decoder& decoder) && {
    auto state = crubit::TransmuteAbi<OptionBoxDynTrait>().Decode(decoder);
    auto manager = crubit::TransmuteAbi<ManagerType>().Decode(decoder);
    return DynCallable<Sig>(state, manager, invoker);
  }

  explicit DynCallableAbi(Value::Impl::InvokerType invoker)
      : invoker(invoker) {}
  Value::Impl::InvokerType invoker;
};

}  // namespace internal_dyn_callable

// rs_std::DynCallable
//
// `rs_std::DynCallable` is a functional wrapper type, like
// `absl::AnyInvocable`, that assumes ownership of a Rust callable object of the
// form `Box<dyn Trait>`, where `Trait` is one of `Fn`, `FnMut`, or `FnOnce`.
//
// TODO(okabayashi): other notes to include
// - move only
// - default constructor cannot be used
// - can only be constructed by bridging from Rust
// - will eventually be freely convertible to absl::AnyInvocable
template <class Sig>
class CRUBIT_BRIDGE("DynCallable", "", "") DynCallable
    : private internal_dyn_callable::Impl<Sig> {
  using Impl = internal_dyn_callable::Impl<Sig>;

  friend struct internal_dyn_callable::DynCallableAbi<Sig>;

  explicit DynCallable(internal_dyn_callable::OptionBoxDynTrait state,
                       internal_dyn_callable::ManagerType manager,
                       Impl::InvokerType invoker)
      : Impl(state, manager, invoker) {}

 public:
  DynCallable() = default;
  ~DynCallable() = default;
  DynCallable(DynCallable& other) = delete;
  DynCallable(DynCallable&& other) = default;
  DynCallable& operator=(DynCallable&& other) = default;
  using Impl::operator();
};

}  // namespace rs_std

#endif  // THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_DYN_CALLABLE_H_
