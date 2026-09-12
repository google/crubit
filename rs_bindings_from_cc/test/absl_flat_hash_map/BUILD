load("@rules_cc//cc:cc_library.bzl", "cc_library")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load(
    "//rs_bindings_from_cc/bazel_support:additional_rust_srcs_for_crubit_bindings_aspect_hint.bzl",
    "additional_rust_srcs_for_crubit_bindings",
)
load("//rs_bindings_from_cc/bazel_support:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")
load(
    "//rs_bindings_from_cc/test/golden:golden_test.bzl",
    "golden_test",
)

package(default_applicable_licenses = ["//:license"])

# A mostly-empty `flat_hash_map` target that re-exports the support types the same way the real one
# does. This is needed only for tests that don't use the real implementation.
cc_library(
    name = "flat_hash_map",
    aspect_hints = [
        ":flat_hash_map_rust.hint",
    ],
)

rust_api_from_cpp(
    name = "flat_hash_map_rust",
    srcs = ["reexport_support_types.rs"],
    cpp_target = ":flat_hash_map",
    deps = ["//support:container"],
)

cc_library(
    name = "isolated",
    hdrs = ["isolated.h"],
    aspect_hints = [
        "//features:experimental",
    ],
    deps = [
        ":flat_hash_map",
        "//support:annotations",
    ],
)

golden_test(
    name = "isolated_golden_test",
    basename = "isolated",
    cc_library = "isolated",
    golden_cc = "isolated_api_impl.cc",
    golden_rs = "isolated_rs_api.rs",
)

crubit_rust_test(
    name = "isolated_test",
    srcs = ["isolated_test.rs"],
    cc_deps = [":isolated"],
    deps = ["@crate_index//:googletest"],
)

crubit_test_cc_library(
    name = "fake",
    testonly = 1,
    hdrs = ["fake.h"],
    aspect_hints = [
        "//features:experimental",
    ],
    deps = [
        ":flat_hash_map",
    ],
)

crubit_rust_test(
    name = "fake_test",
    srcs = ["fake_test.rs"],
    cc_deps = [
        ":fake",
    ],
    deps = [
        "//support:container",
        "//support:ctor",
        "@crate_index//:googletest",
    ],
)
