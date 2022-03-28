# Support libraries that the generated Rust/C++ bindings depend upon.
load("//third_party/bazel_rules/rules_rust/rust:defs.bzl", "rust_library", "rust_proc_macro", "rust_test")

licenses(["notice"])

cc_library(
    name = "rs_api_impl_support",
    hdrs = [
        "cxx20_backports.h",
    ],
    visibility = [
        "//devtools/rust/cc_interop:__subpackages__",
        "//third_party/crubit:__subpackages__",
    ],
)

rust_library(
    name = "ctor",
    srcs = ["ctor.rs"],
    proc_macro_deps = [":ctor_proc_macros"],
    visibility = [
        "//devtools/rust/cc_interop:__subpackages__",
        "//third_party/crubit:__subpackages__",
    ],
)

rust_proc_macro(
    name = "ctor_proc_macros",
    srcs = ["ctor_proc_macros.rs"],
    deps = [
        "//third_party/rust/proc_macro2/v1:proc_macro2",
        "//third_party/rust/quote/v1:quote",
        "//third_party/rust/syn/v1:syn",
    ],
)

rust_test(
    name = "ctor_test",
    srcs = ["ctor.rs"],
    proc_macro_deps = [":ctor_proc_macros"],
    deps = ["//third_party/rust/pin_project/v1:pin_project"],
)

rust_test(
    name = "ctor_proc_macros_test",
    srcs = ["ctor_proc_macros_test.rs"],
    deps = [
        ":ctor",
        "//third_party/rust/pin_project/v1:pin_project",
    ],
)

rust_test(
    name = "ctor_macro_test",
    srcs = ["ctor_macro_test.rs"],
    deps = [
        ":ctor",
    ],
)
