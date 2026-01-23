load("@rules_rust//rust:defs.bzl", "rust_library", "rust_test")

package(default_applicable_licenses = ["//:license"])

# The more-stable version of forward_declare, which does not use unsized_const_params.
# This is tested separately, so that we know the fallback is always available.
rust_library(
    name = "forward_declare",
    testonly = True,
    srcs = ["//support:forward_declare.rs"],
    compatible_with = ["//buildenv/target:non_prod"],
    crate_features = [],
    proc_macro_deps = ["//support:forward_declare_proc_macros"],
    # LINT.IfChange
    rustc_flags = ["-Zallow-features=extern_types,negative_impls"],
    # LINT.ThenChange(//docs/overview/unstable_features.md)
)

rust_test(
    name = "forward_declare_macros_stable_test",
    srcs = ["//support:forward_declare_macros_test.rs"],
    deps = [
        ":forward_declare",
        "@crate_index//:googletest",
    ],
)

rust_test(
    name = "forward_declare_stable_test",
    srcs = ["forward_declare_stable_test.rs"],
    deps = [
        ":forward_declare",
    ],
)
