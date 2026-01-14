load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
)
load(
    "//cc_bindings_from_rs/bazel_support:cc_bindings_from_rust_rule.bzl",
    "cc_bindings_from_rust",
)
load(
    "//cc_bindings_from_rs/test/golden:golden_test.bzl",
    "golden_test",
)
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_cc_test")

package(default_applicable_licenses = ["//:license"])

rust_library(
    name = "trait_definition",
    testonly = 1,
    srcs = ["trait_definition.rs"],
)

cc_bindings_from_rust(
    name = "trait_definition_cc_api",
    testonly = 1,
    crate = ":trait_definition",
)

rust_library(
    name = "trait_impl",
    testonly = 1,
    srcs = ["trait_impl.rs"],
    deps = [":trait_definition"],
)

cc_bindings_from_rust(
    name = "traits_impl_cc_api",
    testonly = 1,
    crate = ":trait_impl",
)

golden_test(
    name = "trait_definition_golden_test",
    basename = "trait_definition",
    golden_h = "trait_definition_cc_api.h",
    golden_rs = "trait_definition_cc_api_impl.rs",
    rust_library = "trait_definition",
)

golden_test(
    name = "trait_impl_golden_test",
    basename = "trait_impl",
    golden_h = "trait_impl_cc_api.h",
    golden_rs = "trait_impl_cc_api_impl.rs",
    rust_library = "trait_impl",
)

crubit_cc_test(
    name = "trait_impl_test",
    srcs = ["trait_impl_test.cc"],
    deps = [
        ":traits_impl_cc_api",
        "//testing/base/public:gunit_main",
    ],
)
