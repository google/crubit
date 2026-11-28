"""End-to-end test of `cc_bindings_from_rs` for a re-exported type that shares its name with a function."""

load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
)
load(
    "//cc_bindings_from_rs/test/golden:golden_test.bzl",
    "golden_test",
)

package(default_applicable_licenses = ["//:license"])

rust_library(
    name = "defining_crate",
    testonly = 1,
    srcs = ["defining_crate.rs"],
    edition = "2024",
)

rust_library(
    name = "middle_crate",
    testonly = 1,
    srcs = ["middle_crate.rs"],
    edition = "2024",
    deps = [":defining_crate"],
)

rust_library(
    name = "reexport_beside_function",
    testonly = 1,
    srcs = ["reexport_beside_function.rs"],
    edition = "2024",
    deps = [":middle_crate"],
)

golden_test(
    name = "reexport_beside_function_golden_test",
    basename = "reexport_beside_function",
    golden_h = "reexport_beside_function_cc_api.h",
    golden_rs = "reexport_beside_function_cc_api_impl.rs",
    rust_library = "reexport_beside_function",
)
