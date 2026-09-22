"""End-to-end tests of `cc_bindings_from_rs`, focusing on `std::os::fd` types."""

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
    name = "fd_reexporter",
    testonly = 1,
    srcs = ["fd_reexporter.rs"],
)

rust_library(
    name = "std_os_fd",
    testonly = 1,
    srcs = ["std_os_fd.rs"],
    deps = [":fd_reexporter"],
)

cc_bindings_from_rust(
    name = "std_os_fd_cc_api",
    testonly = 1,
    crate = ":std_os_fd",
)

golden_test(
    name = "std_os_fd_golden_test",
    basename = "std_os_fd",
    golden_h = "std_os_fd_cc_api.h",
    golden_rs = "std_os_fd_cc_api_impl.rs",
    rust_library = "std_os_fd",
)
