package(default_applicable_licenses = ["//third_party/crubit:license"])

cc_library(
    name = "rs_api_impl_support",
    hdrs = [
        "cxx20_backports.h",
        "offsetof.h",
    ],
    visibility = ["//:__subpackages__"],
    # It is important to be thoughtful when adding new dependencies for
    # `rs_api_impl_support` (and possibly other targets in this BUILD file).
    # Using mature Abseil APIs seems okay - we should be able to assume that
    # Crubit users have a version of Abseil that is relatively recent (although
    # we can't rely on an exact version and/or exact absl/base/options.h).
    deps = [],
)

cc_test(
    name = "offsetof_test",
    srcs = ["offsetof_test.cc"],
    deps = [
        ":rs_api_impl_support",
        "@com_google_googletest//:gtest_main",
    ],
)
