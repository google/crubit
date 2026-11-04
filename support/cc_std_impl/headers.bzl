# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""Which headers to include from the C++ standard library."""

LIBCXX_HEADERS = [
    "algorithm",
    "any",
    "array",
    "atomic",
    "bitset",
    "cassert",
    "cctype",
    "cerrno",
    "cfenv",
    "cfloat",
    "charconv",
    "chrono",
    "cinttypes",
    "climits",
    "cmath",
    # TODO(b/437061002): Re-enable once the bug is fixed.
    # "condition_variable",
    "coroutine",
    "csetjmp",
    "csignal",
    "cstdarg",
    "cstddef",
    "cstdint",
    "cstdio",
    "cstdlib",
    "cstring",
    "ctime",
    "cuchar",
    "cwchar",
    "cwctype",
    "deque",
    "exception",
    "forward_list",
    "functional",
    "initializer_list",
    "ios",
    "limits",
    "list",
    "map",
    "memory",
    "mutex",
    "new",
    "numeric",
    "optional",
    "queue",
    "ratio",
    "set",
    "scoped_allocator",
    "stack",
    "stdexcept",
    "streambuf",
    "string",
    "string_view",
    "system_error",
    "tuple",
    "type_traits",
    "typeindex",
    "typeinfo",
    "unordered_map",
    "unordered_set",
    "utility",
    "variant",
    "vector",
]

LIBC_HEADERS = [
    "netinet/tcp.h",
    "netinet/udp.h",
    "sys/socket.h",
]

# Libc headers that are not available on Apple platforms (iOS, macOS).
NON_APPLE_LIBC_HEADERS = [
    "netinet/ether.h",
    # TODO(b/433699129): Re-enable on Apple once Crubit or the Apple SDK
    # handles the RFC 3678 socket filter functions (`setipv4sourcefilter`,
    # `getipv4sourcefilter`, `setsourcefilter`, `getsourcefilter`) declared
    # outside `__BEGIN_DECLS` / `__END_DECLS` (`extern "C"`) in `netinet/in.h`.
    "netinet/in.h",
]
