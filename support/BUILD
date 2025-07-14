"""Support libraries that the generated Rust/C++ bindings depend upon."""

load("@rules_rust//rust:defs.bzl", "rust_library", "rust_proc_macro", "rust_test")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")

package(default_applicable_licenses = ["//:license"])

exports_files(
    srcs = glob(["*"]),
    visibility = [
        ":__subpackages__",
        "//google_internal/release:crubit_support_lib_visibility",
    ],
)

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
    compatible_with = ["//buildenv/target:non_prod"],
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
        # INTERNAL VISIBILITY ONLY.
        # FOR USAGE ADD TO //support/public:annotations
        "//:__subpackages__",
    ],
    deps = [
        "//support/internal:bindings_support",
    ],
)

rust_proc_macro(
    name = "crubit_annotate",
    srcs = ["crubit_annotate.rs"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = [
        "//:__subpackages__",
    ],
    deps = [
        "@crate_index//:proc-macro2",
        "@crate_index//:quote",
        "@crate_index//:syn",
    ],
)

rust_test(
    name = "crubit_annotate_test",
    srcs = ["crubit_annotate_test.rs"],
    proc_macro_deps = [
        ":crubit_annotate",
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

rust_test(
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

rust_test(
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

rust_test(
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
    crate_features = ["unstable"],
    proc_macro_deps = [":forward_declare_proc_macros"],
    # LINT.IfChange
    rustc_flags = ["-Zallow-features=extern_types,negative_impls,vec_into_raw_parts,unsized_const_params"],
    # LINT.ThenChange(//docs/overview/unstable_features.md)
    visibility = [
        "//:__subpackages__",
        "//google_internal/release:crubit_support_lib_visibility",
    ],
)

rust_proc_macro(
    name = "forward_declare_proc_macros",
    srcs = ["forward_declare_proc_macros.rs"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = [
        ":__subpackages__",
        "//google_internal/release:crubit_support_lib_visibility",
    ],
    deps = [
        "@crate_index//:proc-macro2",
        "@crate_index//:quote",
        "@crate_index//:syn",
    ],
)

rust_test(
    name = "forward_declare_macros_test",
    srcs = ["forward_declare_macros_test.rs"],
    deps = [
        ":forward_declare",
        "@crate_index//:googletest",
    ],
)

rust_test(
    name = "forward_declare_unstable_test",
    srcs = ["forward_declare_unstable_test.rs"],
    deps = [
        ":forward_declare",
    ],
)

rust_library(
    name = "oops",
    srcs = ["oops.rs"],
    # LINT.IfChange
    rustc_flags = ["-Zallow-features=negative_impls"],
    # LINT.ThenChange(//docs/overview/unstable_features.md)
    visibility = [
        # INTERNAL VISIBILITY ONLY.
        # FOR USAGE ADD TO //support:oops
        "//:__subpackages__",
    ],
)

rust_test(
    name = "oops_test",
    srcs = ["oops.rs"],
    # LINT.IfChange
    rustc_flags = ["-Zallow-features=negative_impls"],
    # LINT.ThenChange(//docs/overview/unstable_features.md)
    deps = [
        "@crate_index//:googletest",
    ],
)

rust_library(
    name = "bridge_rust",
    srcs = ["bridge.rs"],
    visibility = [
        # INTERNAL VISIBILITY ONLY.
        # FOR USAGE ADD TO //support/public:bridge_rust
        "//:__subpackages__",
    ],
)

rust_test(
    name = "bridge_rust_test",
    srcs = ["bridge.rs"],
    deps = [
        "@crate_index//:googletest",
    ],
)

cc_library(
    name = "bridge_cpp",
    hdrs = ["bridge.h"],
    visibility = [
        # INTERNAL VISIBILITY ONLY.
        # FOR USAGE ADD TO //support/public:bridge_cpp
        "//:__subpackages__",
        "//google_internal/release:crubit_support_lib_visibility",
    ],
)

cc_test(
    name = "bridge_cpp_test",
    srcs = ["bridge_test.cc"],
    deps = [
        ":bridge_cpp",
        "@googletest//:gtest_main",
    ],
)

cc_library(
    name = "status_bridge_cpp",
    srcs = ["status_bridge.cc"],
    hdrs = ["status_bridge.h"],
    visibility = [
        # INTERNAL VISIBILITY ONLY.
        # FOR USAGE ADD TO //support/public:status_bridge_cpp
        "//:__subpackages__",
    ],
    deps = [
        ":bridge_cpp",
        "@abseil-cpp//absl/status",
        "@abseil-cpp//absl/status:statusor",
        "@abseil-cpp//absl/strings:string_view",
    ],
)

cc_test(
    name = "status_bridge_cpp_test",
    srcs = ["status_bridge_test.cc"],
    deps = [
        ":bridge_cpp",
        ":status_bridge_cpp",
        "@abseil-cpp//absl/status",
        "@abseil-cpp//absl/status:statusor",
        "@googletest//:gtest_main",
    ],
)
