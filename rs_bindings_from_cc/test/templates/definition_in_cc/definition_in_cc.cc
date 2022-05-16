// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/test/templates/definition_in_cc/definition_in_cc.h"

// static
template <typename T>
MyTemplate<T> MyTemplate<T>::Create(T value) {
  MyTemplate<T> result;
  result.value_ = value;
  return result;
}

template <typename T>
const T& MyTemplate<T>::value() const {
  return value_;
}

template class MyTemplate<int>;
