"""End-to-end example of using `includes` attribute in `cc_library`."""

load("//common:crubit_wrapper_macros_oss.bzl", crubit_rust_test = "crubit_on_demand_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "using_includes",
    # srcs = ["includes_dir/myheader.h"],
    hdrs = [
        "includes_dir/myheader.h",
        "using_includes.h",
    ],
    includes = ["includes_dir/"],
)

crubit_rust_test(
    name = "includes_test",
    srcs = ["includes_test.rs"],
    cc_deps = [":using_includes"],
)
