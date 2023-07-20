# C++ libraries that help work with Rust types.

package(default_applicable_licenses = ["//:license"])

cc_library(
    name = "rs_char",
    hdrs = ["rs_char.h"],
    # Enable bidirectional bindings (via crubit_internal_rust_type).
    aspect_hints = ["//:experimental"],
    visibility = [
        "//visibility:public",
    ],

    # It is important to be thoughtful when adding new dependencies for `char`
    # (and possibly other targets in this BUILD file).  Using mature Abseil APIs
    # seems okay - we should be able to assume that Crubit users have a version
    # of Abseil that is relatively recent (although we can't rely on an
    # exact version and/or exact absl/base/options.h).
    deps = [
        "//support/internal:bindings_support",
        "@absl//absl/base:core_headers",
    ],
)

cc_test(
    name = "rs_char_test",
    srcs = ["rs_char_test.cc"],
    deps = [
        ":rs_char",
        "@com_google_googletest//:gtest_main",
    ],
)
