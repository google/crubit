"""Internal C++ libraries that help work with Rust types."""

load("@rules_cc//cc:cc_library.bzl", "cc_library")

package(default_applicable_licenses = ["//:license"])

cc_library(
    name = "enum",
    hdrs = ["enum.h"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = [
        "//visibility:public",
    ],
    deps = ["//support/rs_std/internal:enum"],
)

cc_library(
    name = "is_utf8",
    hdrs = ["is_utf8.h"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = [
        "//visibility:public",
    ],
    deps = ["//support/rs_std/internal:is_utf8"],
)
