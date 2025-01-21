load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
)

package(
    default_applicable_licenses = ["//:license"],
    default_visibility = ["//:__subpackages__"],
)

rust_library(
    name = "database",
    srcs = [
        "adt_core_bindings.rs",
        "code_snippet.rs",
        "db.rs",
        "fine_grained_feature.rs",
        "fully_qualified_name.rs",
        "include_guard.rs",
        "lib.rs",
        "sugared_ty.rs",
        "type_location.rs",
    ],
    # LINT.IfChange
    rustc_flags = ["-Zallow-features=rustc_private,rustc_attr"],
    # LINT.ThenChange(//docs/overview/unstable_features.md)
    deps = [
        "//cc_bindings_from_rs:crubit_attr",
        "//common:arc_anyhow",
        "//common:code_gen_utils",
        "//common:crubit_feature",
        "//common:error_report",
        "//common:memoized",
        "@crate_index//:flagset",
        "@crate_index//:itertools",
        "@crate_index//:proc-macro2",
        "@crate_index//:quote",
    ],
)
