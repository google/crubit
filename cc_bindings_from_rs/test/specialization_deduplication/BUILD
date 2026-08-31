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

package(default_applicable_licenses = ["//:license"])

rust_library(
    name = "dep_crate",
    testonly = 1,
    srcs = ["dep_crate.rs"],
    aspect_hints = [
        "//features:supported",
    ],
)

cc_bindings_from_rust(
    name = "dep_crate_cc_api",
    testonly = 1,
    crate = ":dep_crate",
)

golden_test(
    name = "dep_crate_golden_test",
    basename = "dep_crate",
    golden_h = "dep_crate_cc_api.h",
    golden_rs = "dep_crate_cc_api_impl.rs",
    platforms = [
        "android",
        "ios",
    ],
    rust_library = "dep_crate",
)

rust_library(
    name = "reexport_crate",
    testonly = 1,
    srcs = ["reexport_crate.rs"],
    aspect_hints = [
        "//features:supported",
    ],
    deps = [":dep_crate"],
)

cc_bindings_from_rust(
    name = "reexport_crate_cc_api",
    testonly = 1,
    crate = ":reexport_crate",
)

golden_test(
    name = "reexport_crate_golden_test",
    basename = "reexport_crate",
    golden_h = "reexport_crate_cc_api.h",
    golden_rs = "reexport_crate_cc_api_impl.rs",
    platforms = [
        "android",
        "ios",
    ],
    rust_library = "reexport_crate",
)
