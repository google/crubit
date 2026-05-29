"""End-to-end tests of `cc_bindings_from_rs`, focusing on type bridging."""

load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
)
load("//cc_bindings_from_rs/bazel_support:cc_bindings_from_rust_library_config_aspect_hint.bzl", "cc_bindings_from_rust_library_config")
load(
    "//cc_bindings_from_rs/bazel_support:cc_bindings_from_rust_rule.bzl",
    "cc_bindings_from_rust",
)
load(
    "//cc_bindings_from_rs/test/golden:golden_test.bzl",
    "golden_test",
)
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_cc_test")

package(default_applicable_licenses = ["//:license"])

rust_library(
    name = "rust_type",
    srcs = [
        "rust_type.rs",
    ],
    aspect_hints = [
        ":cc_type_hint",
    ],
    proc_macro_deps = [
        "//support:crubit_annotate",
    ],
)

cc_bindings_from_rust_library_config(
    name = "cc_type_hint",
    extra_cc_hdrs = ["cc_type.h"],
)

cc_bindings_from_rust(
    name = "rust_type_cc_api",
    crate = ":rust_type",
)

rust_library(
    name = "rust_pointer_types",
    srcs = ["rust_pointer_types.rs"],
    aspect_hints = [
        ":cc_type_hint",
    ],
    proc_macro_deps = [
        "//support:crubit_annotate",
    ],
)

cc_bindings_from_rust(
    name = "rust_pointer_types_cc_api",
    crate = ":rust_pointer_types",
)

rust_library(
    name = "layout_equivalent_generics",
    srcs = ["layout_equivalent_generics.rs"],
    aspect_hints = [
        ":layout_equivalent_generics_hint",
    ],
    proc_macro_deps = [
        "//support:crubit_annotate",
    ],
)

cc_bindings_from_rust_library_config(
    name = "layout_equivalent_generics_hint",
    extra_cc_hdrs = ["cc_generics.h"],
)

cc_bindings_from_rust(
    name = "layout_equivalent_generics_cc_api",
    crate = ":layout_equivalent_generics",
)

golden_test(
    name = "layout_equivalent_generics_golden_test",
    basename = "layout_equivalent_generics",
    golden_h = "layout_equivalent_generics_cc_api.h",
    golden_rs = "layout_equivalent_generics_cc_api_impl.rs",
    rust_library = "layout_equivalent_generics",
)

crubit_cc_test(
    name = "bridging_test",
    srcs = ["bridging_test.cc"],
    deps = [
        ":layout_equivalent_generics_cc_api",
        ":rust_pointer_types_cc_api",
        ":rust_type_cc_api",
        "//testing/base/public:gunit_main",
    ],
)

rust_library(
    name = "bridged_types",
    srcs = ["bridged_types.rs"],
    aspect_hints = [
        "//features:experimental",
    ],
    deps = [
        "//support:bridge_rust",
    ],
)

golden_test(
    name = "bridged_types_golden_test",
    basename = "bridged_types",
    golden_h = "bridged_types_cc_api.h",
    golden_rs = "bridged_types_cc_api_impl.rs",
    rust_library = "bridged_types",
)

rust_library(
    name = "layout_equivalent_pointers",
    srcs = ["layout_equivalent_pointers.rs"],
    aspect_hints = [
        "//features:experimental",
    ],
    deps = [
        "//support:bridge_rust",
    ],
)

golden_test(
    name = "layout_equivalent_pointers_golden_test",
    basename = "layout_equivalent_pointers",
    golden_h = "layout_equivalent_pointers_cc_api.h",
    golden_rs = "layout_equivalent_pointers_cc_api_impl.rs",
    rust_library = "layout_equivalent_pointers",
)
