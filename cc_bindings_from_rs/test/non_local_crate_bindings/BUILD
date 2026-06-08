load("//common:crubit_wrapper_macros_oss.bzl", "crubit_cc_test")

package(default_applicable_licenses = ["//:license"])

crubit_cc_test(
    name = "core_bindings_test",
    srcs = ["core_bindings_test.cc"],
    deps = [
        "//support/rs_std:rs_core",
        "//testing/base/public:gunit_main",
    ],
)
