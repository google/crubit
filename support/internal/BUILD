load("//common:crubit_wrapper_macros_oss.bzl", "crubit_cc_test")

package(default_applicable_licenses = ["//:license"])

cc_library(
    name = "bindings_support",
    hdrs = [
        "attribute_macros.h",
        "cxx20_backports.h",
        "lazy_init.h",
        "memswap.h",
        "offsetof.h",
        "sizeof.h",
        "slot.h",
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
        "@abseil-cpp//absl/base:core_headers",
    ],
)

crubit_cc_test(
    name = "memswap_test",
    srcs = ["memswap_test.cc"],
    deps = [
        ":bindings_support",
        "@googletest//:gtest_main",
    ],
)

crubit_cc_test(
    name = "offsetof_test",
    srcs = ["offsetof_test.cc"],
    deps = [
        ":bindings_support",
        "@googletest//:gtest_main",
    ],
)

crubit_cc_test(
    name = "slot_test",
    srcs = ["slot_test.cc"],
    deps = [
        ":bindings_support",
        "@abseil-cpp//absl/base:core_headers",
        "@abseil-cpp//absl/log:check",
        "@googletest//:gtest_main",
    ],
)

crubit_cc_test(
    name = "sizeof_test",
    srcs = ["sizeof_test.cc"],
    deps = [
        ":bindings_support",
        "@googletest//:gtest_main",
    ],
)
