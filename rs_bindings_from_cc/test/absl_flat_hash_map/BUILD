load("@rules_cc//cc:cc_library.bzl", "cc_library")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/bazel_support:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")
load(
    "//rs_bindings_from_cc/test/golden:golden_test.bzl",
    "golden_test",
)

package(default_applicable_licenses = ["//:license"])

# A stub of the absl target that defines `absl::flat_hash_map`, for tests that don't use the real
# implementation. Like the real target, its Rust bindings re-export the support types that the
# bindings for instantiations of `absl::flat_hash_map` refer to.
cc_library(
    name = "flat_hash_map",
    hdrs = ["flat_hash_map.h"],
    aspect_hints = [
        ":flat_hash_map_rust.hint",
        "//features:experimental",
    ],
)

rust_api_from_cpp(
    name = "flat_hash_map_rust",
    srcs = ["reexport_support_types.rs"],
    cpp_target = ":flat_hash_map",
    deps = ["//support:map"],
)

cc_library(
    name = "isolated",
    hdrs = ["isolated.h"],
    aspect_hints = [
        "//features:supported",
    ],
    deps = [
        ":flat_hash_map",
        "//support:annotations",
        "//support/rs_std:slice_ref",
        "//support/rs_std:str_ref",
        "@abseil-cpp//absl/types:span",
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
    deps = [
        "//support:map",
        "@crate_index//:googletest",
        "@crate_index//:static_assertions",  # v1
    ],
)

# Defines a fake `absl::flat_hash_map` itself, so it plays the role of both the target that defines
# the class template and the target that instantiates it.
crubit_test_cc_library(
    name = "fake",
    testonly = 1,
    hdrs = ["fake.h"],
    aspect_hints = [
        ":fake_rust.hint",
        "//features:experimental",
    ],
)

rust_api_from_cpp(
    name = "fake_rust",
    testonly = 1,
    srcs = ["reexport_support_types.rs"],
    cpp_target = ":fake",
    deps = ["//support:map"],
)

crubit_rust_test(
    name = "fake_test",
    srcs = ["fake_test.rs"],
    cc_deps = [
        ":fake",
    ],
    deps = [
        "//support:ctor",
        "//support:map",
        "@crate_index//:googletest",
    ],
)
