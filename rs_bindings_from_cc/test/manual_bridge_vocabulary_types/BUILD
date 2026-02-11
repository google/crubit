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

cc_library(
    name = "unique_ptr_lib",
    hdrs = ["unique_ptr_lib.h"],
    aspect_hints = [
        "//features:std_unique_ptr",
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
        "//features:std_vector",
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
