load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")

package(default_applicable_licenses = ["//:license"])

crubit_rust_test(
    name = "cc_template_integration_test",
    srcs = [
        "__cc_template_instantiations_rs_api.rs",
        "cc_template_integration_test.rs",
    ],
    data = ["__cc_template_instantiations.json"],
    proc_macro_deps = ["//support/cc_template:cc_template"],
    rustc_env = {
        "CRUBIT_INSTANTIATIONS_FILE": "$(location __cc_template_instantiations.json)",
    },
    deps = [
        "@crate_index//:googletest",
    ],
)
