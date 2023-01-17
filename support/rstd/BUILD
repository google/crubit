# C++ libraries that help work with Rust types.

cc_library(
    name = "char",
    hdrs = ["char.h"],
    visibility = ["//:__subpackages__"],
    # It is important to be thoughtful when adding new dependencies for
    # `rs_api_impl_support` (and possibly other targets in this BUILD file).
    # Even simple dependencies like `absl` may be tricky in practice, because
    # `rs_api_impl_support` may be developed and tested with one set of
    # absl/base/options.h which may be different from absl/base/options.h
    # adopted by users of Crubit.
    deps = [],
)

cc_test(
    name = "char_test",
    srcs = ["char_test.cc"],
    deps = [
        ":char",
        "@com_google_googletest//:gtest_main",
    ],
)
