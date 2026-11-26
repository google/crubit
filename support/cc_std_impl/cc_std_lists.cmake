# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#
# Automatically @generated from support/cc_std_impl/BUILD and headers.bzl
# by common/golden_update.sh. Do not edit.

set(_CRUBIT_STD_LIBCXX_HEADERS
  "algorithm"
  "any"
  "array"
  "atomic"
  "bitset"
  "cassert"
  "cctype"
  "cerrno"
  "cfenv"
  "cfloat"
  "charconv"
  "chrono"
  "cinttypes"
  "climits"
  "cmath"
  "coroutine"
  "csetjmp"
  "csignal"
  "cstdarg"
  "cstddef"
  "cstdint"
  "cstdio"
  "cstdlib"
  "cstring"
  "ctime"
  "cuchar"
  "cwchar"
  "cwctype"
  "deque"
  "exception"
  "forward_list"
  "functional"
  "initializer_list"
  "ios"
  "limits"
  "list"
  "map"
  "memory"
  "mutex"
  "new"
  "numeric"
  "optional"
  "queue"
  "ratio"
  "set"
  "scoped_allocator"
  "stack"
  "stdexcept"
  "streambuf"
  "string"
  "string_view"
  "system_error"
  "tuple"
  "type_traits"
  "typeindex"
  "typeinfo"
  "unordered_map"
  "unordered_set"
  "utility"
  "variant"
  "vector"
)

set(_CRUBIT_STD_LIBC_HEADERS
  "netinet/tcp.h"
  "netinet/udp.h"
  "sys/socket.h"
)

set(_CRUBIT_STD_NON_APPLE_LIBC_HEADERS
  "netinet/ether.h"
  "netinet/in.h"
)

set(_CRUBIT_STD_HELPER_HEADERS
  "allocator.h"
  "conversion_function_helpers.h"
  "macos_sdk_workarounds.h"
)

set(_CRUBIT_STD_EXTRA_RS_SRCS
  "lossy_utf8.rs"
  "slice_ptr.rs"
  "allocator.rs=std"
  "nullable.rs=std"
  "optional.rs=std"
  "shared_ptr.rs=std"
  "string.rs=std"
  "string_view.rs=std"
  "unique_ptr.rs=std"
  "variant.rs=std"
  "variant_coproduct.rs=std"
  "variant_enums.rs=std"
  "variant_internal.rs=std"
  "vector.rs=std"
  "vector_partial_eq.rs=std"
)
