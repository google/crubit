"""Support libraries that the generated Rust/C++ bindings depend upon."""

load("@rules_rust//rust:defs.bzl", "rust_library", "rust_proc_macro")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")

package(default_applicable_licenses = ["//:license"])

rust_library(
    name = "ctor",
    srcs = ["ctor.rs"],
    proc_macro_deps = [":ctor_proc_macros"],
    # LINT.IfChange
    rustc_flags = ["-Zallow-features=negative_impls"],
    # LINT.ThenChange(//docs/overview/unstable_features.md)
    visibility = ["//:__subpackages__"],
)

rust_proc_macro(
    name = "ctor_proc_macros",
    srcs = ["ctor_proc_macros.rs"],
    deps = [
        "@crate_index//:proc-macro2",
        "@crate_index//:quote",
        "@crate_index//:syn",
    ],
)

cc_library(
    name = "annotations",
    hdrs = ["annotations.h"],
    visibility = [
        "//:__subpackages__",
    ],
    deps = [
        "//support/internal:bindings_support",
    ],
)

crubit_rust_test(
    name = "ctor_proc_macros_test",
    crate = ":ctor_proc_macros_proc_macro_internal",
    deps = [
        "//common:token_stream_matchers",
        "@crate_index//:googletest",
    ],
)

crubit_rust_test(
    name = "ctor_test",
    srcs = ["ctor.rs"],
    proc_macro_deps = [":ctor_proc_macros"],
    # LINT.IfChange
    rustc_flags = ["-Zallow-features=negative_impls"],
    # LINT.ThenChange(//docs/overview/unstable_features.md)
    deps = [
        "@crate_index//:googletest",
    ],
)

crubit_rust_test(
    name = "ctor_proc_macros_e2e_test",
    srcs = ["ctor_proc_macros_test.rs"],
    # LINT.IfChange
    rustc_flags = ["-Zallow-features=negative_impls"],
    # LINT.ThenChange(//docs/overview/unstable_features.md)
    deps = [
        ":ctor",
        "@crate_index//:googletest",
    ],
)

crubit_rust_test(
    name = "ctor_macro_test",
    srcs = ["ctor_macro_test.rs"],
    deps = [
        ":ctor",
        "@crate_index//:googletest",
    ],
)

rust_library(
    name = "forward_declare",
    srcs = ["forward_declare.rs"],
    proc_macro_deps = [":forward_declare_proc_macros"],
    # LINT.IfChange
    rustc_flags = ["-Zallow-features=extern_types,negative_impls,vec_into_raw_parts"],
    # LINT.ThenChange(//docs/overview/unstable_features.md)
    visibility = ["//:__subpackages__"],
)

rust_proc_macro(
    name = "forward_declare_proc_macros",
    srcs = ["forward_declare_proc_macros.rs"],
    deps = [
        "@crate_index//:proc-macro2",
        "@crate_index//:quote",
        "@crate_index//:syn",
    ],
)

crubit_rust_test(
    name = "forward_declare_macros_test",
    srcs = ["forward_declare_macros_test.rs"],
    deps = [
        ":forward_declare",
        "@crate_index//:googletest",
    ],
)

rust_library(
    name = "oops",
    srcs = ["oops.rs"],
    # LINT.IfChange
    rustc_flags = ["-Zallow-features=negative_impls"],
    # LINT.ThenChange(//docs/overview/unstable_features.md)
    visibility = [
        "//:__subpackages__",
    ],
)

crubit_rust_test(
    name = "oops_test",
    srcs = ["oops.rs"],
    # LINT.IfChange
    rustc_flags = ["-Zallow-features=negative_impls"],
    # LINT.ThenChange(//docs/overview/unstable_features.md)
    deps = [
        "@crate_index//:googletest",
    ],
)
