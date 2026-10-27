# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""Per-feature denylists for features in `SUPPORTED_FEATURES`."""

visibility("private")

# NOTE: Only the BUILD file in this package may load this file (enforced by `visibility("private")`).
# Rules and aspects read the denylists through the `:supported_feature_denylists` target instead.
# If a widely loaded .bzl file loaded it, every edit here would affect every package that uses
# Crubit, and BTR Guardian (<internal link>) would throttle even one-line denylist changes.

# Per-feature denylists for features in `SUPPORTED_FEATURES`: maps a feature name to a list of
# target patterns which do not get that feature by default.
#
# Accepted pattern formats:
# - "//..." (matches all targets in all packages)
# - "//foo/bar/...": matches all targets in package //foo/bar and its subpackages.
# - "//foo/bar:baz": matches the specific target baz in package //foo/bar.
# - "//foo/bar": shorthand for //foo/bar:bar.
# Note: Wildcard target patterns like "//foo/bar:*" or "//foo/bar:all" are not supported.
#
# WARNING: Only add entries here as part of turning on a feature by default (i.e. adding it to
# `SUPPORTED_FEATURES`). Do not add targets here for any other reason.
SUPPORTED_FEATURE_DENYLISTS = {
    "assume_lifetimes": [
        # buildifier: keep sorted <internal link>
        "//rs_bindings_from_cc/test/allowlist_template_instantiation/...",
        "//rs_bindings_from_cc/test/annotations/...",
        "//rs_bindings_from_cc/test/assume_lifetimes/release/blocklisted_subpackage/...",
        "//rs_bindings_from_cc/test/assume_lifetimes/release/subpackage",
        "//rs_bindings_from_cc/test/assume_lifetimes/release/subpackage:experimental",
        "//rs_bindings_from_cc/test/assume_lifetimes/release/subpackage:experimental_opt_out",
        "//rs_bindings_from_cc/test/assume_lifetimes/release/subpackage:opt_in",
        "//rs_bindings_from_cc/test/assume_lifetimes/release/subpackage:opt_out",
        "//rs_bindings_from_cc/test/assume_lifetimes/release/subpackage:supported",
        "//rs_bindings_from_cc/test/consume_absl/...",
        "//rs_bindings_from_cc/test/display/...",
        "//rs_bindings_from_cc/test/function/non_extern_c/...",
        "//rs_bindings_from_cc/test/function/simple/...",
        "//rs_bindings_from_cc/test/manual_bridge_vocabulary_types/...",
        "//rs_bindings_from_cc/test/proto/...",
        "//rs_bindings_from_cc/test/roundtrip/...",
        "//rs_bindings_from_cc/test/struct/methods/...",
        "//rs_bindings_from_cc/test/struct/operator_and/...",
        "//rs_bindings_from_cc/test/templates/allowlist/...",
        "//support/cc_std_impl/test/string_view/...",
    ],
    "always_specialize_generics_in_cpp_api_from_rust": [
        "//cc_bindings_from_rs/test/bridging/composable:composable_bridging",
    ],
    "infer_send_sync": [
        # Tests the pre-`infer_send_sync` default (unannotated records are `!Send + !Sync`), which
        # denylisted targets still get.
        "//rs_bindings_from_cc/test/types/thread_safety/...",
    ],
    # Some denylists (e.g. `nonnull_smart_pointers`) are stored as one file per entry instead, in
    # `internal/<feature>/`. See `denylist_dir.bzl`.
}
