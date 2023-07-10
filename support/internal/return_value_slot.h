// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_SUPPORT_INTERNAL_RETURN_VALUE_SLOT_H_
#define CRUBIT_SUPPORT_INTERNAL_RETURN_VALUE_SLOT_H_

#include <memory>
#include <utility>

namespace crubit {

// `ReturnValueSlot<T>` provides a slot that can store a move-only return
// value.  This class is used to return non-`#[repr(C)]` structs from Rust
// into C++ in a way that is compatible with the ABI of `extern "C"` Rust
// thunks.
//
// An example will help to illustrate the purpose of this class:
//
//     ```rs
//     pub struct SomeStruct(...);
//     pub fn foo(arg1: i32, arg2: i32) -> SomeStruct { unimplemented!() }
//     ```
//
// The generated C++ header will look like this:
//
//     ```cc
//              inline SomeStruct foo(int32_t arg1, int32_t arg2) {
//     /* 1 */    crubit::ReturnValueSlot<SomeStruct> __ret;
//     /* 2 */    __rust_thunk_for_foo(arg1, arg2, __ret.GetSlotPtr());
//     /* 3 */    return __ret.AssumeInitAndTakeValue();
//              }
//     }
// ```
//
// `ReturnValueSlot` helps to coordinate when C++ constructors and destructors
// run in the example above:
// - `SomeStruct`'s constructor should *not* run on line 1.
// - Rust thunk can populates the return slot on line 2.
//   The Rust thunk may panic without populating the return slot - in this
//   case nothing should operate on the uninitialized `SomeStruct` value
//   (this is accomplished by ReturnValueSlot having an empty/no-op destructor)
// - `SomeStruct`'s move constructor will run on line 3 (moving the return value
//   out of `ReturnValueSlot::value_`, and then destructing the moved-away
//   `ReturnValueSlot::value_`).
//
// Behavior of `ReturnValueSlot<T>` in steps 1 and 2 is identical to
// `MaybeUninit<T>` in Rust, but the behavior on line 3 is a bit different:
// there is an extra call to a move constructor in C++, but there are no move
// constructors in Rust.
template <typename T>
union ReturnValueSlot {
 public:
  // Creates `ReturnValueSlot` in an uninitialized state.
  ReturnValueSlot() {
    // Leaving `value_` uninitialized / not invoking any constructor of `T`.
  }

  // Gets a pointer to the slot where the return value may be written.
  //
  // SAFETY REQUIREMENTS:
  // - Caller should not read from the returned pointer before the value has
  //   been initialized.
  // - Caller should only write to the returned pointer while the
  //   `ReturnValueSlot` is in an uninitialized state (i.e. care should be taken
  //   to avoid writing to the slot twice, potentially overwriting a value
  //   without calling its destructor).
  T* Get() { return &value_; }

  // Takes and returns the value.  This leaves the `ReturnValueSlot` in a
  // moved-away state - afterwards the only valid operation is to destroy the
  // `ReturnValueSlot` object (or assign to it, but the assignment operators
  // have been `delete`d below).
  //
  // SAFETY REQUIREMENTS:
  // Caller should ensure the return value has been initialized before calling
  // `AssumeInitAndTakeValue()`. (e.g. by ensuring that the value has been
  // earlier written to the location pointed to by `GetPtr()`).
  T AssumeInitAndTakeValue() && {
    T return_value(std::move(value_));
    std::destroy_at(&value_);
    return return_value;
  }

  // SAFETY REQUIREMENTS: The return value in `other` must have been
  // initialized.  (It is also okay if the other value is in a moved-away state
  // after calling `AssumeInitAndTakeValue` on it).
  ReturnValueSlot(ReturnValueSlot&& other) { value_ = std::move(other.value_); }

  // Does not destroy the contained value.
  //
  // Before `~ReturnValueSlot()` is invoked, the contained value should be
  // destroyed by the user (typically, by calling `AssumeInitAndTakeValue`).  If
  // the contained value is left initialized by the time `~ReturnValueSlot()`
  // runs, the value is leaked.
  ~ReturnValueSlot() {
    // Not destroying or otherwise using `value_`.
  }

  ReturnValueSlot(const ReturnValueSlot&) = delete;
  ReturnValueSlot& operator=(const ReturnValueSlot&) = delete;
  ReturnValueSlot& operator=(ReturnValueSlot&&) = delete;

 private:
  T value_;
};

}  // namespace crubit

#endif  // CRUBIT_SUPPORT_INTERNAL_RETURN_VALUE_SLOT_H_
