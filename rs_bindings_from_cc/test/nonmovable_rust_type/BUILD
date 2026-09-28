load("@rules_cc//cc:cc_library.bzl", "cc_library")
load("@rules_rust//rust:defs.bzl", "rust_library")
load(
    "//cc_bindings_from_rs/bazel_support:cc_bindings_from_rust_rule.bzl",
    "cc_bindings_from_rust",
)
load(
    "//common:crubit_wrapper_macros_oss.bzl",
    "crubit_rust_test",
)

package(default_applicable_licenses = ["//:license"])

licenses(["notice"])

rust_library(
    name = "nonmovable",
    srcs = ["nonmovable.rs"],
    aspect_hints = ["//features:supported"],
)

cc_bindings_from_rust(
    name = "nonmovable_cc",
    crate = ":nonmovable",
)

cc_library(
    name = "takes_nonmovable",
    hdrs = ["takes_nonmovable.h"],
    aspect_hints = ["//features:supported"],
    deps = [":nonmovable_cc"],
)

crubit_rust_test(
    name = "takes_nonmovable_test",
    srcs = ["takes_nonmovable_test.rs"],
    cc_deps = [":takes_nonmovable"],
    deps = [
        ":nonmovable",
        "@crate_index//:googletest",
    ],
)

cc_library(
    name = "ctor_takes_nonmovable",
    hdrs = ["ctor_takes_nonmovable.h"],
    aspect_hints = ["//features:supported"],
    deps = [":nonmovable_cc"],
)

crubit_rust_test(
    name = "ctor_takes_nonmovable_test",
    srcs = ["ctor_takes_nonmovable_test.rs"],
    cc_deps = [":ctor_takes_nonmovable"],
    deps = [
        ":nonmovable",
        "//support:ctor",
        "@crate_index//:googletest",
        "@crate_index//:static_assertions",  # v1
    ],
)
