// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_SUPPORT_CC_STD_ALLOCATOR_H_
#define THIRD_PARTY_CRUBIT_SUPPORT_CC_STD_ALLOCATOR_H_

#include <__memory/shared_count.h>
#include <stdio.h>

#include <cstddef>
#include <new>

namespace crubit_cc_std_internal::std_allocator {

// Performs `new x` without running the constructor. Instead, this directly
// calls the correct `operator new` overload.
inline void* cpp_new(size_t n, size_t align) {
  if (align <= __STDCPP_DEFAULT_NEW_ALIGNMENT__) {
    return operator new(n);
  } else {
    return operator new(n, static_cast<std::align_val_t>(align));
  }
}

// Performs `delete x` without running the destructor. Instead, this directly
// calls the correct `operator delete` overload.
inline void cpp_delete(void* ptr, size_t n, size_t align) {
#ifdef __cpp_sized_deallocation
  if (align <= __STDCPP_DEFAULT_NEW_ALIGNMENT__) {
    operator delete(ptr, n);
  } else {
    operator delete(ptr, n, static_cast<std::align_val_t>(align));
  }
#else
  if (align <= __STDCPP_DEFAULT_NEW_ALIGNMENT__) {
    operator delete(ptr);
  } else {
    operator delete(ptr, static_cast<std::align_val_t>(align));
  }
#endif
}

// Newtype wrapper around `std::__shared_weak_count` because Crubit doesn't
// bind to standard library internals nor aliases to them.
//
// This wrapper is a `private` subclass of `std::__shared_weak_count` to ensure
// that Crubit doesn't generate bindings to any of its methods.
class shared_weak_count : private std::__shared_weak_count {
  friend void shared_ptr_ref(shared_weak_count* cntrl);
  friend void shared_ptr_unref(shared_weak_count* cntrl);
  friend size_t shared_ptr_use_count(const shared_weak_count* cntrl);
};

inline void shared_ptr_ref(shared_weak_count* cntrl) {
  if (cntrl != nullptr) {
    cntrl->__add_shared();
  }
}

inline void shared_ptr_unref(shared_weak_count* cntrl) {
  if (cntrl != nullptr) {
    cntrl->__release_shared();
  }
}

inline size_t shared_ptr_use_count(const shared_weak_count* cntrl) {
  if (cntrl == nullptr) {
    return 0;
  }
  return static_cast<size_t>(cntrl->use_count());
}

// Specifies which lifecycle hook of `std::shared_ptr` is being invoked.
enum class FunctionToCall : bool {
  // Dispatched when the strong reference count hits 0 (`__on_zero_shared`).
  // The deleter should destroy the managed payload value.
  kDestroyValue,
  // Dispatched when both strong and weak reference counts hit 0
  // (`__on_zero_shared_weak`). The deleter should destruct the control block
  // and deallocate any containing storage.
  kDeleteControlBlock,
};

struct DynControlBlock;

// TODO(b/526962187): Remove this thunk once CFI works properly and call the
// deleter directly from `__on_zero_shared` and `__on_zero_shared_weak`.
extern "C" void __crubit_invoke_dyn_control_block_deleter(
    FunctionToCall function_to_call, DynControlBlock* cntrl);

// A primitive building block for custom `std::shared_ptr` control blocks.
//
// `DynControlBlock` provides a flexible interface allowing callers (such as
// Rust) to easily plug in custom destruction and deallocation logic. It
// forwards `__on_zero_shared` and `__on_zero_shared_weak` hooks to a provided
// `deleter` callback.
struct DynControlBlock : private shared_weak_count {
 public:
  using DeleterFn = void (*)(FunctionToCall, DynControlBlock*);

  // Emplaces the DynControlBlock at the given pointer.
  static shared_weak_count* Emplace(DynControlBlock* ptr, DeleterFn deleter) {
    return new (ptr) DynControlBlock(deleter);
  }

  DeleterFn deleter;

 private:
  explicit DynControlBlock(DeleterFn deleter) : deleter(deleter) {}

  void __on_zero_shared() noexcept override {
    __crubit_invoke_dyn_control_block_deleter(FunctionToCall::kDestroyValue,
                                              this);
  }

  void __on_zero_shared_weak() noexcept override {
    __crubit_invoke_dyn_control_block_deleter(
        FunctionToCall::kDeleteControlBlock, this);
  }
};

}  // namespace crubit_cc_std_internal::std_allocator

#endif  // THIRD_PARTY_CRUBIT_SUPPORT_CC_STD_ALLOCATOR_H_
