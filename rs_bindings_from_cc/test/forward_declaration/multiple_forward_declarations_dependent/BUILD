load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "forward_declaration1",
    hdrs = ["forward_declaration1.h"],
)

crubit_test_cc_library(
    name = "forward_declaration2",
    hdrs = ["forward_declaration2.h"],
    deps = [
        ":forward_declaration1",
    ],
)

crubit_test_cc_library(
    name = "definition",
    hdrs = ["definition.h"],
)

# Test cases to ensure we can easily convert forward declarations of the same type across crates
# may depend on each other.
crubit_rust_test(
    name = "test",
    srcs = ["test.rs"],
    cc_deps = [
        ":definition",
        ":forward_declaration1",
        ":forward_declaration2",
    ],
    deps = [
        "//support:forward_declare",
        "@crate_index//:googletest",
    ],
)
