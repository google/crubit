# Test utilities and tests for lifetime_analysis.

package(default_visibility = ["//visibility:public"])

cc_library(
    name = "lifetime_analysis_test",
    testonly = 1,
    srcs = ["lifetime_analysis_test.cc"],
    hdrs = ["lifetime_analysis_test.h"],
    deps = [
        "//lifetime_analysis:analyze",
        "//lifetime_annotations/test:named_func_lifetimes",
        "//lifetime_annotations/test:run_on_code",
        "@absl//absl/container:flat_hash_map",
        "@com_google_googletest//:gtest",
    ],
)

cc_test(
    name = "builtin",
    srcs = ["builtin.cc"],
    deps = [
        ":lifetime_analysis_test",
        "@com_google_googletest//:gtest_main",
    ],
)

cc_test(
    name = "lifetime_params",
    srcs = ["lifetime_params.cc"],
    deps = [
        ":lifetime_analysis_test",
        "@com_google_googletest//:gtest_main",
    ],
)

cc_test(
    name = "virtual_functions",
    srcs = ["virtual_functions.cc"],
    deps = [
        ":lifetime_analysis_test",
        "@com_google_googletest//:gtest_main",
    ],
)

cc_test(
    name = "casts",
    srcs = ["casts.cc"],
    deps = [
        ":lifetime_analysis_test",
        "@com_google_googletest//:gtest_main",
    ],
)

cc_test(
    name = "callbacks",
    srcs = ["callbacks.cc"],
    deps = [
        ":lifetime_analysis_test",
        "@com_google_googletest//:gtest_main",
    ],
)

cc_test(
    name = "initializers",
    srcs = ["initializers.cc"],
    deps = [
        ":lifetime_analysis_test",
        "@com_google_googletest//:gtest_main",
    ],
)

cc_test(
    name = "recursion",
    srcs = ["recursion.cc"],
    deps = [
        ":lifetime_analysis_test",
        "@com_google_googletest//:gtest_main",
    ],
)

cc_test(
    name = "function_templates",
    srcs = ["function_templates.cc"],
    deps = [
        ":lifetime_analysis_test",
        "@com_google_googletest//:gtest_main",
    ],
)

cc_test(
    name = "function_calls",
    srcs = ["function_calls.cc"],
    deps = [
        ":lifetime_analysis_test",
        "@com_google_googletest//:gtest_main",
    ],
)

cc_test(
    name = "execution_order",
    srcs = ["execution_order.cc"],
    deps = [
        ":lifetime_analysis_test",
        "@com_google_googletest//:gtest_main",
    ],
)

cc_test(
    name = "control_flow",
    srcs = ["control_flow.cc"],
    deps = [
        ":lifetime_analysis_test",
        "@com_google_googletest//:gtest_main",
    ],
)

cc_test(
    name = "basic",
    srcs = ["basic.cc"],
    deps = [
        ":lifetime_analysis_test",
        "@com_google_googletest//:gtest_main",
    ],
)

cc_test(
    name = "static_lifetime",
    srcs = ["static_lifetime.cc"],
    deps = [
        ":lifetime_analysis_test",
        "@com_google_googletest//:gtest_main",
    ],
)

cc_test(
    name = "arrays",
    srcs = ["arrays.cc"],
    deps = [
        ":lifetime_analysis_test",
        "@com_google_googletest//:gtest_main",
    ],
)

cc_test(
    name = "records",
    srcs = ["records.cc"],
    deps = [
        ":lifetime_analysis_test",
        "@com_google_googletest//:gtest_main",
    ],
)

cc_test(
    name = "inheritance",
    srcs = ["inheritance.cc"],
    deps = [
        ":lifetime_analysis_test",
        "@com_google_googletest//:gtest_main",
    ],
)

# TODO(b/253992761): fix and re-enable.
cc_test(
    name = "class_templates",
    srcs = ["class_templates.cc"],
    tags = [
        "manual",
        "notap",
    ],
    deps = [
        ":lifetime_analysis_test",
        "@com_google_googletest//:gtest_main",
    ],
)

cc_test(
    name = "initialization",
    srcs = ["initialization.cc"],
    deps = [
        ":lifetime_analysis_test",
        "@com_google_googletest//:gtest_main",
    ],
)

cc_test(
    name = "expr",
    srcs = ["expr.cc"],
    deps = [
        ":lifetime_analysis_test",
        "@com_google_googletest//:gtest_main",
    ],
)

cc_test(
    name = "defaulted_functions",
    srcs = ["defaulted_functions.cc"],
    deps = [
        ":lifetime_analysis_test",
        "@com_google_googletest//:gtest_main",
    ],
)
