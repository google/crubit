// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <stddef.h>
#include <stdlib.h>

// Our temporary, local solution to
//     https://github.com/rust-lang/rust/issues/73632.

void *__rdl_alloc(size_t, size_t);
void __rdl_dealloc(void *);
void *__rdl_realloc(void *, size_t, size_t, size_t);
void *__rdl_alloc_zeroed(size_t, size_t);

// https://stdrs.dev/nightly/x86_64-unknown-linux-gnu/alloc/alloc/index.html

__attribute__((weak))
void *__rust_alloc(size_t size, size_t align) {
  return __rdl_alloc(size, align);
}

__attribute__((weak))
void __rust_dealloc(void *ptr) {
  __rdl_dealloc(ptr);
}

__attribute__((weak))
void *__rust_realloc(void *ptr, size_t old_size, size_t align,
    size_t new_size) {
  return __rdl_realloc(ptr, old_size, align, new_size);
}

__attribute__((weak))
void *__rust_alloc_zeroed(size_t size, size_t align) {
  return __rdl_alloc_zeroed(size, align);
}

__attribute__((weak))
void *__rust_alloc_error_handler(size_t size, size_t align) {
  (void)size;
  (void)align;

  abort();
}
