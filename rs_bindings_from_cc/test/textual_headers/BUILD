load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "has_textual_hdrs",
    textual_hdrs = ["textual.h"],
)

crubit_rust_test(
    name = "consumes_has_textual_hdrs",
    srcs = ["empty.rs"],
    cc_deps = [":has_textual_hdrs"],
)

crubit_test_cc_library(
    name = "has_inc_header",
    hdrs = ["textual.inc"],
)

crubit_rust_test(
    name = "consumes_has_inc_header",
    srcs = ["empty.rs"],
    cc_deps = [":has_inc_header"],
)

crubit_test_cc_library(
    name = "defines_struct_in_textual_hdr",
    textual_hdrs = ["struct_in_textual_header.inc"],
)

crubit_test_cc_library(
    name = "uses_struct_from_textual_hdr_in_textual_hdr",
    hdrs = ["includes_struct_through_layers_of_textual_headers.h"],
    textual_hdrs = ["includes_textual_header.inc"],
    deps = [":defines_struct_in_textual_hdr"],
)

crubit_test_cc_library(
    name = "uses_struct_from_textual_hdr",
    hdrs = ["includes_textual_header.h"],
    deps = ["defines_struct_in_textual_hdr"],
)

crubit_rust_test(
    name = "struct_from_textual_hdr_test",
    srcs = ["uses_struct_from_textual_header.rs"],
    cc_deps = [":uses_struct_from_textual_hdr"],
    deps = [
        "@crates//:googletest",
    ],
)

crubit_rust_test(
    name = "struct_travels_through_textual_hdrs_test",
    srcs = ["uses_struct_from_layers_of_textual_headers.rs"],
    cc_deps = [":uses_struct_from_textual_hdr_in_textual_hdr"],
    deps = [
        "@crates//:googletest",
    ],
)
