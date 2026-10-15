load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(
    # Build and use precompiled header modules (where supported).
    features = ["header_modules"],
)

licenses(["notice"])

# `lib.h` uses `hidden_ns::Holder`, which `fwd.h` forward-declares, and which is defined in
# `hidden.h`.
#
# When header modules are used, `:lib`'s header analysis loads the precompiled module of `:hidden`,
# so Clang can find the definition of `Holder` even though `lib.h` doesn't include `hidden.h`. The
# definition is not visible in `lib.h`, nor in the generated C++ code for `:lib`, so Crubit must
# treat `Holder` as incomplete, as it would without header modules.
crubit_test_cc_library(
    name = "lib",
    hdrs = ["lib.h"],
    deps = [
        ":fwd",
        ":hidden",
    ],
)

crubit_test_cc_library(
    name = "fwd",
    hdrs = ["fwd.h"],
)

crubit_test_cc_library(
    name = "hidden",
    hdrs = ["hidden.h"],
)

crubit_rust_test(
    name = "test",
    srcs = ["test.rs"],
    cc_deps = [":lib"],
    deps = [
        "@crate_index//:googletest",
    ],
)
