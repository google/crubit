load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "forward_declaration",
    srcs = [
        "forward_declaration.cc",
    ],
    hdrs = ["forward_declaration.h"],
)

# Basic but odd uses of forward declaration. Actually, these usages aren't _that_ odd: Crubit
# constructs a 'virtual' header that `#include` headers in a target, so forward_declaration.h
# can happen after concatenating a few headers.
crubit_rust_test(
    name = "test",
    srcs = ["test.rs"],
    cc_deps = [
        ":forward_declaration",
    ],
    deps = [
        "//third_party/gtest_rust/googletest",
    ],
)
