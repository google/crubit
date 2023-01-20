# C++ libraries that help work with Rust types.

cc_library(
    name = "char",
    hdrs = ["char.h"],
    visibility = ["//:__subpackages__"],

    # It is important to be thoughtful when adding new dependencies for `char`
    # (and possibly other targets in this BUILD file).  Using mature Abseil APIs
    # seems okay - we should be able to assume that Crubit users have a version
    # of Abseil that is relatively recent (although we can't rely on an
    # exact version and/or exact absl/base/options.h).
    deps = ["@absl//absl/base:core_headers"],
)

cc_test(
    name = "char_test",
    srcs = ["char_test.cc"],
    deps = [
        ":char",
        "@com_google_googletest//:gtest_main",
    ],
)
