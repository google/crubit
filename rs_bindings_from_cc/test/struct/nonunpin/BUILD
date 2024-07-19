# End-to-end test of !Unpin classes.

load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "nonunpin",
    hdrs = ["nonunpin.h"],
    deps = ["@abseil-cpp//absl/log:check"],
)

crubit_rust_test(
    name = "nonunpin_test",
    srcs = ["nonunpin_test.rs"],
    cc_deps = [":nonunpin"],
    rustc_flags = ["-Zallow-features=negative_impls"],
    deps = ["//support:ctor"],
)
