"""Support libraries that the generated Rust/C++ bindings depend upon."""

load("@rules_cc//cc:cc_test.bzl", "cc_test")
load("@rules_rust//rust:defs.bzl", "rust_library", "rust_proc_macro", "rust_test")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")

package(default_applicable_licenses = ["//:license"])

exports_files(
    srcs = glob(["*"]),
    visibility = [
        ":__subpackages__",
    ],
)

rust_library(
    name = "ctor",
    srcs = ["ctor.rs"],
    compatible_with = ["//buildenv/target:non_prod"],
    edition = "2021",
    proc_macro_deps = [":ctor_proc_macros"],
    # LINT.IfChange
    rustc_flags = ["-Zallow-features=allow_internal_unstable,negative_impls,super_let"],
    # LINT.ThenChange(//docs/overview/unstable_features.md)
    visibility = [
        "//visibility:public",
    ],
)

rust_proc_macro(
    name = "ctor_proc_macros",
    srcs = ["ctor_proc_macros.rs"],
    compatible_with = ["//buildenv/target:non_prod"],
    deps = [
        "@crate_index//:proc-macro2",
        "@crate_index//:quote",  # v1
        "@crate_index//:syn",  # v2
    ],
)

cc_library(
    name = "annotations",
    hdrs = ["annotations.h"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = [
        "//visibility:public",
    ],
    deps = [
        "//support:annotations_internal",
    ],
)

cc_library(
    name = "annotations_internal",
    hdrs = ["annotations_internal.h"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = [
        "//visibility:public",
    ],
    deps = [
        "@abseil-cpp//absl/base:core_headers",
        "@abseil-cpp//absl/base:nullability",
    ],
)

cc_library(
    name = "lifetime_annotations",
    hdrs = ["lifetime_annotations.h"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = [
        "//visibility:public",
    ],
)

rust_proc_macro(
    name = "crubit_annotate",
    srcs = ["crubit_annotate.rs"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = [
        "//visibility:public",
    ],
    deps = [
        "@crate_index//:proc-macro2",
        "@crate_index//:quote",  # v1
        "@crate_index//:syn",  # v2
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

crubit_rust_test(
    name = "ctor_rename_test",
    srcs = ["ctor_rename_test.rs"],
    # Despite the name "aliases", the name `ctor` is actually not available in the test body.
    # This is important for the purpose of the test.
    aliases = {
        "ctor": "renamed_ctor",
    },
    # LINT.IfChange
    rustc_flags = ["-Zallow-features=negative_impls"],
    # LINT.ThenChange(//docs/overview/unstable_features.md)
    deps = [
        ":ctor",
        "@crate_index//:googletest",
    ],
)

rust_test(
    name = "ctor_test",
    srcs = ["ctor_test.rs"],
    rustc_flags = ["-Zallow-features=negative_impls"],
    deps = [
        ":ctor",
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
    compatible_with = ["//buildenv/target:non_prod"],
    crate_features = ["unstable"],
    proc_macro_deps = [":forward_declare_proc_macros"],
    # LINT.IfChange
    rustc_flags = ["-Zallow-features=extern_types,negative_impls,vec_into_raw_parts,unsized_const_params,adt_const_params"],
    # LINT.ThenChange(//docs/overview/unstable_features.md)
    visibility = [
        "//visibility:public",
    ],
)

rust_proc_macro(
    name = "forward_declare_proc_macros",
    srcs = ["forward_declare_proc_macros.rs"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = [
        ":__subpackages__",
    ],
    deps = [
        "@crate_index//:proc-macro2",
        "@crate_index//:quote",  # v1
        "@crate_index//:syn",  # v2
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
    compatible_with = ["//buildenv/target:non_prod"],
    # LINT.IfChange
    rustc_flags = ["-Zallow-features=negative_impls"],
    # LINT.ThenChange(//docs/overview/unstable_features.md)
    visibility = [
        "//visibility:public",
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
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = [
        "//visibility:public",
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
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = [
        "//visibility:public",
    ],
)

cc_test(
    name = "bridge_cpp_test",
    srcs = ["bridge_test.cc"],
    deps = [
        ":bridge_cpp",
        "//testing/base/public:gunit_main",
    ],
)

# Used to fix SWIG syntax errors when you see Crubit annotations.
filegroup(
    name = "annotations_swig",
    srcs = ["annotations.swig"],
    visibility = [
        "//visibility:public",
    ],
)
