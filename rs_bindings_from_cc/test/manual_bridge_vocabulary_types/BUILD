load("@rules_cc//cc:cc_library.bzl", "cc_library")
load(
    "//common:crubit_wrapper_macros_oss.bzl",
    "crubit_rust_test",
)

cc_library(
    name = "common",
    hdrs = ["common.h"],
    aspect_hints = [
        "//features:experimental",
    ],
)

cc_library(
    name = "crubit_disabled",
    hdrs = ["crubit_disabled.h"],
)

crubit_rust_test(
    name = "span_test",
    srcs = ["span_test.rs"],
    cc_deps = [
        ":span_lib",
        "@abseil-cpp//absl/types:span",
    ],
    deps = [
        "//support:ctor",
        "@crate_index//:googletest",
    ],
)

cc_library(
    name = "span_lib",
    srcs = ["span_lib.cc"],
    hdrs = ["span_lib.h"],
    aspect_hints = [
        "//features:supported",
    ],
    deps = [
        "//support/public:annotations",
        "@abseil-cpp//absl/types:span",
    ],
)

cc_library(
    name = "unique_ptr_lib",
    hdrs = ["unique_ptr_lib.h"],
    aspect_hints = [
        "//features:supported",
    ],
    deps = [
        ":common",
        ":crubit_disabled",
        "//support:annotations",
    ],
)

crubit_rust_test(
    name = "unique_ptr_test",
    srcs = ["unique_ptr_test.rs"],
    cc_deps = [
        ":common",
        ":unique_ptr_lib",
        "//support/public:cc_std",
    ],
    proc_macro_deps = [
        "//common:item_exists",
    ],
    deps = [
        "@crate_index//:googletest",
    ],
)

cc_library(
    name = "unique_ptr_experimental_lib",
    hdrs = ["unique_ptr_lib.h"],
    aspect_hints = [
        "//features:experimental",
    ],
    deps = [
        ":common",
        ":crubit_disabled",
        "//support:annotations",
    ],
)

cc_library(
    name = "vector_lib",
    hdrs = ["vector_lib.h"],
    aspect_hints = [
        "//features:supported",
    ],
    deps = [
        ":common",
        ":crubit_disabled",
        "//support:annotations",
    ],
)

crubit_rust_test(
    name = "vector_test",
    srcs = ["vector_test.rs"],
    cc_deps = [
        ":common",
        ":vector_lib",
        "//support/public:cc_std",
    ],
    proc_macro_deps = [
        "//common:item_exists",
    ],
    deps = [
        "@crate_index//:googletest",
    ],
)

cc_library(
    name = "vector_experimental_lib",
    hdrs = ["vector_lib.h"],
    aspect_hints = [
        "//features:experimental",
    ],
    deps = [
        ":common",
        ":crubit_disabled",
        "//support:annotations",
    ],
)

crubit_rust_test(
    name = "vector_experimental_test",
    srcs = ["vector_experimental_test.rs"],
    cc_deps = [
        ":common",
        ":vector_experimental_lib",
        "//support/public:cc_std",
    ],
    proc_macro_deps = [
        "//common:item_exists",
    ],
    deps = [
        "@crate_index//:googletest",
    ],
)
