"""End-to-end example of using `includes` attribute in `cc_library`."""


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

rust_test(
    name = "includes_test",
    srcs = ["includes_test.rs"],
    cc_deps = [":using_includes"],
)
