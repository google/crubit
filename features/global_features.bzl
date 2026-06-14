# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""A shared list of supported features, including fine grained additions to `:supported`."""

visibility(["//..."])

SUPPORTED_FEATURES = [
    "supported",
    "assume_lifetimes",
]

# A list of targets that should not be granted the `assume_lifetimes` feature by default.
# Accepted pattern formats:
# - "//..." (matches all targets in all packages)
# - "//foo/bar/...": matches all targets in package //foo/bar and its subpackages.
# - "//foo/bar:baz": matches the specific target baz in package //foo/bar.
# - "//foo/bar": shorthand for //foo/bar:bar.
# Note: Wildcard target patterns like "//foo/bar:*" or "//foo/bar:all" are not supported.

# buildifier: keep sorted <internal link>
NO_ASSUME_LIFETIMES_TARGETS = [
    "//...",
    "//rs_bindings_from_cc/test/assume_lifetimes/release/blocklisted_subpackage/...",
    "//rs_bindings_from_cc/test/assume_lifetimes/release/subpackage",
    "//rs_bindings_from_cc/test/assume_lifetimes/release/subpackage:experimental",
    "//rs_bindings_from_cc/test/assume_lifetimes/release/subpackage:experimental_opt_out",
    "//rs_bindings_from_cc/test/assume_lifetimes/release/subpackage:opt_in",
    "//rs_bindings_from_cc/test/assume_lifetimes/release/subpackage:opt_out",
    "//rs_bindings_from_cc/test/assume_lifetimes/release/subpackage:supported",
]
