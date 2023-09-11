"""End-to-end example of using generated headers."""

load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

genrule(
    name = "generate_header",
    srcs = ["generated_header_content.h"],
    outs = ["generated_header.h"],
    cmd = "cat $(location generated_header_content.h) > \"$@\"",
)

crubit_test_cc_library(
    name = "generated_header",
    hdrs = ["generated_header.h"],
)

crubit_rust_test(
    name = "generated_header_test",
    srcs = ["generated_header_test.rs"],
    cc_deps = [":generated_header"],
)
