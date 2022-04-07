load("@rules_rust//rust:defs.bzl", "rust_test")

licenses(["notice"])

cc_library(
    name = "has_textual_hdrs",
    textual_hdrs = ["textual.h"],
)

rust_test(
    name = "consumes_has_textual_hdrs",
    srcs = ["empty.rs"],
    cc_deps = [":has_textual_hdrs"],
)

cc_library(
    name = "has_inc_header",
    hdrs = ["textual.inc"],
)

rust_test(
    name = "consumes_has_inc_header",
    srcs = ["empty.rs"],
    cc_deps = [":has_inc_header"],
)

cc_library(
    name = "defines_struct_in_textual_hdr",
    textual_hdrs = ["struct_in_textual_header.inc"],
)

cc_library(
    name = "uses_struct_from_textual_hdr_in_textual_hdr",
    hdrs = ["includes_struct_through_layers_of_textual_headers.h"],
    textual_hdrs = ["includes_textual_header.inc"],
    deps = [":defines_struct_in_textual_hdr"],
)

cc_library(
    name = "uses_struct_from_textual_hdr",
    hdrs = ["includes_textual_header.h"],
    deps = ["defines_struct_in_textual_hdr"],
)

rust_test(
    name = "struct_from_textual_hdr_test",
    srcs = ["uses_struct_from_textual_header.rs"],
    cc_deps = [":uses_struct_from_textual_hdr"],
)

rust_test(
    name = "struct_travels_through_textual_hdrs_test",
    srcs = ["uses_struct_from_layers_of_textual_headers.rs"],
    cc_deps = [":uses_struct_from_textual_hdr_in_textual_hdr"],
)
