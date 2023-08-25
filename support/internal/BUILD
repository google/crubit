load("//common:crubit_wrapper_macros_oss.bzl", "crubit_cc_test")

package(default_applicable_licenses = ["//:license"])

cc_library(
    name = "bindings_support",
    hdrs = [
        "attribute_macros.h",
        "cxx20_backports.h",
        "memswap.h",
        "offsetof.h",
        "return_value_slot.h",
        "sizeof.h",
    ],
    visibility = [
        "//visibility:public",
    ],
    # It is important to be thoughtful when adding new dependencies for
    # `bindings_support` (and possibly other targets in this BUILD file).
    # Using mature Abseil APIs seems okay - we should be able to assume that
    # Crubit users have a version of Abseil that is relatively recent (although
    # we can't rely on an exact version and/or exact absl/base/options.h).
    deps = [
        "@absl//absl/base:core_headers",
    ],
)

crubit_cc_test(
    name = "memswap_test",
    srcs = ["memswap_test.cc"],
    deps = [
        ":bindings_support",
        "@com_google_googletest//:gtest_main",
    ],
)

crubit_cc_test(
    name = "offsetof_test",
    srcs = ["offsetof_test.cc"],
    deps = [
        ":bindings_support",
        "@com_google_googletest//:gtest_main",
    ],
)

crubit_cc_test(
    name = "return_value_slot_test",
    srcs = ["return_value_slot_test.cc"],
    deps = [
        ":bindings_support",
        "@absl//absl/log:check",
        "@com_google_googletest//:gtest_main",
    ],
)

crubit_cc_test(
    name = "sizeof_test",
    srcs = ["sizeof_test.cc"],
    deps = [
        ":bindings_support",
        "@com_google_googletest//:gtest_main",
    ],
)
