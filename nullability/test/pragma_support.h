// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef PRAGMA_SUPPORT_H_
#define PRAGMA_SUPPORT_H_

template <class T>
struct Vec {
  T& front();
  T* data;
};

template <class T>
T create() {
  return T{};
}

#endif
