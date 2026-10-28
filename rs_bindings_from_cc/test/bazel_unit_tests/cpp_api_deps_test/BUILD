load("@rules_cc//cc:cc_library.bzl", "cc_library")
load("@rules_rust//rust:defs.bzl", "rust_library")
load("//bazel:cpp_api_from_rust.bzl", "cpp_api_from_rust")
load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load(
    "//common:crubit_wrapper_macros_oss.bzl",
    "crubit_cc_test",
)

package(default_applicable_licenses = ["//:license"])

cc_library(
    name = "base_cc",
    hdrs = ["base.h"],
    aspect_hints = [":base_rust.hint"],
)

# Depends on `base_cc`, so `base_rust.hint` cannot depend on `derived_cc`
# without creating a dependency cycle (`base_cc -> base_rust.hint -> derived_cc -> base_cc`).
cc_library(
    name = "derived_cc",
    hdrs = ["derived.h"],
    deps = [":base_cc"],
)

rust_api_from_cpp(
    name = "base_rust",
    srcs = ["additional_src.rs"],
    cpp_api_deps = [":derived_cc"],
    cpp_target = ":base_cc",
)

rust_library(
    name = "rust_api",
    srcs = ["rust_api.rs"],
    edition = "2024",
    deps = [":base_rust"],
)

cpp_api_from_rust(
    name = "rust_api_cc",
    crate = ":rust_api",
)

rust_library(
    name = "reexport_consumer_rust_api",
    srcs = ["reexport_consumer_rust_api.rs"],
    edition = "2024",
    deps = [":rust_api"],
)

cpp_api_from_rust(
    name = "reexport_consumer_rust_api_cc",
    crate = ":reexport_consumer_rust_api",
)

# Intentionally does not depend on `:derived_cc` (nor on `gunit_main`): the
# generated `rust_api.h` includes `derived.h`, so `rust_api_cc` must supply
# `derived_cc`'s `CcInfo` itself via `base_rust`'s `cpp_api_deps`.
cc_library(
    name = "cc_consumer",
    srcs = ["cc_consumer.cc"],
    hdrs = ["cc_consumer.h"],
    deps = [
        ":reexport_consumer_rust_api_cc",
        ":rust_api_cc",
    ],
)

crubit_cc_test(
    name = "cpp_api_deps_test",
    srcs = ["cpp_api_deps_test.cc"],
    platforms = ["android"],
    # TODO(b/522829368): remove `no_windows` to enable.
    tags = ["no_windows"],
    deps = [
        ":cc_consumer",
        "@googletest//:gtest_main",
    ],
)
