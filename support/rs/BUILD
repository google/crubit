"""C++ libraries that help work with Rust types."""

load("@rules_cc//cc:cc_library.bzl", "cc_library")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_cc_test")

package(default_applicable_licenses = ["//:license"])

cc_library(
    name = "char",
    hdrs = ["char.h"],
    aspect_hints = ["//features:experimental"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = ["//visibility:public"],
    deps = ["//support/rs_std:char"],
)

cc_library(
    name = "int",
    hdrs = ["int.h"],
    aspect_hints = ["//features:experimental"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = ["//visibility:public"],
    deps = ["//support/rs_std:int"],
)

cc_library(
    name = "waker",
    hdrs = ["waker.h"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = [
        "//:__subpackages__",
    ],
    deps = ["//support/rs_std:waker"],
)

cc_library(
    name = "dyn_erased_future_cc",
    hdrs = ["dyn_erased_future.h"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = [
        "//:__subpackages__",
    ],
    deps = ["//support/rs_std:dyn_erased_future_cc"],
)

cc_library(
    name = "slice_ref",
    hdrs = ["slice_ref.h"],
    aspect_hints = ["//features:experimental"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = ["//visibility:public"],
    deps = ["//support/rs_std:slice_ref"],
)

cc_library(
    name = "str_ref",
    hdrs = ["str_ref.h"],
    aspect_hints = ["//features:experimental"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = ["//visibility:public"],
    deps = ["//support/rs_std:str_ref"],
)

cc_library(
    name = "dyn_callable",
    hdrs = ["dyn_callable.h"],
    aspect_hints = ["//features:experimental"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = ["//visibility:public"],
    deps = ["//support/rs_std:dyn_callable"],
)

cc_library(
    name = "fn_ref",
    hdrs = ["fn_ref.h"],
    aspect_hints = ["//features:experimental"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = ["//visibility:public"],
    deps = ["//support/rs_std:fn_ref"],
)

cc_library(
    name = "fn",
    hdrs = ["fn.h"],
    aspect_hints = ["//features:experimental"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = ["//visibility:public"],
    deps = ["//support/rs_std:fn"],
)

cc_library(
    name = "traits",
    hdrs = ["traits.h"],
    aspect_hints = ["//features:experimental"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = ["//visibility:public"],
    deps = ["//support/rs_std:traits"],
)

cc_library(
    name = "iterator_adapter",
    hdrs = ["iterator_adapter.h"],
    aspect_hints = ["//features:experimental"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = ["//visibility:public"],
    deps = ["//support/rs_std:iterator_adapter"],
)

cc_library(
    name = "option",
    hdrs = ["option.h"],
    aspect_hints = ["//features:experimental"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = ["//visibility:public"],
    deps = ["//support/rs_std:option"],
)

cc_library(
    name = "result",
    hdrs = ["result.h"],
    aspect_hints = ["//features:experimental"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = ["//visibility:public"],
    deps = ["//support/rs_std:result"],
)

cc_library(
    name = "vec",
    hdrs = ["vec.h"],
    aspect_hints = ["//features:experimental"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = ["//visibility:public"],
    deps = ["//support/rs_std:vec"],
)

cc_library(
    name = "tuple",
    hdrs = ["tuple.h"],
    aspect_hints = ["//features:experimental"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = ["//visibility:public"],
    deps = ["//support/rs_std:tuple"],
)

cc_library(
    name = "unit",
    hdrs = ["unit.h"],
    aspect_hints = ["//features:experimental"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = ["//visibility:public"],
    deps = ["//support/rs_std:unit"],
)

cc_library(
    name = "enum",
    hdrs = ["enum.h"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = ["//visibility:public"],
    deps = ["//support/rs_std:enum"],
)

cc_library(
    name = "lossy_formatter_for_bindings",
    hdrs = ["lossy_formatter_for_bindings.h"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = ["//:__subpackages__"],
    deps = ["//support/rs_std:lossy_formatter_for_bindings"],
)

cc_library(
    name = "core",
    hdrs = ["core.h"],
    visibility = ["//visibility:public"],
    deps = ["//support/rs_std:rs_core"],
)

cc_library(
    name = "alloc",
    hdrs = ["alloc.h"],
    visibility = ["//visibility:public"],
    deps = ["//support/rs_std:rs_alloc"],
)

cc_library(
    name = "std",
    hdrs = ["std.h"],
    visibility = ["//visibility:public"],
    deps = ["//support/rs_std"],
)

crubit_cc_test(
    name = "headers_test",
    srcs = ["headers_test.cc"],
    deps = [
        ":alloc",
        ":char",
        ":core",
        ":dyn_callable",
        ":dyn_erased_future_cc",
        ":enum",
        ":fn",
        ":fn_ref",
        ":int",
        ":iterator_adapter",
        ":lossy_formatter_for_bindings",
        ":option",
        ":result",
        ":slice_ref",
        ":std",
        ":str_ref",
        ":traits",
        ":tuple",
        ":unit",
        ":vec",
        ":waker",
        "//support/rs/internal:enum",
        "//support/rs/internal:is_utf8",
    ],
)
