"""Parse lifetime annotations in C++ code."""

licenses(["notice"])

package(default_visibility = ["/visibility:public"])

cc_library(
    name = "lifetime",
    srcs = ["lifetime.cc"],
    hdrs = ["lifetime.h"],
    deps = [
        "//third_party/absl/strings",
        "@llvm///llvm:Support",
    ],
)

cc_test(
    name = "lifetime_test",
    srcs = ["lifetime_test.cc"],
    deps = [
        ":lifetime",
        "//testing/base/public:gunit_main",
    ],
)

cc_library(
    name = "type_lifetimes",
    srcs = ["type_lifetimes.cc"],
    hdrs = ["type_lifetimes.h"],
    deps = [
        ":lifetime",
        ":lifetime_symbol_table",
        ":pointee_type",
        "//third_party/absl/strings",
        "//third_party/absl/strings:str_format",
        "@llvm///clang:ast",
        "@llvm///clang:basic",
        "@llvm///llvm:Support",
    ],
)

cc_library(
    name = "function_lifetimes",
    srcs = ["function_lifetimes.cc"],
    hdrs = ["function_lifetimes.h"],
    deps = [
        ":lifetime",
        ":type_lifetimes",
        "//third_party/absl/strings",
        "@llvm///clang:ast",
        "@llvm///clang:basic",
        "@llvm///llvm:Support",
    ],
)

cc_library(
    name = "lifetime_annotations",
    srcs = ["lifetime_annotations.cc"],
    hdrs = ["lifetime_annotations.h"],
    deps = [
        ":function_lifetimes",
        ":lifetime_symbol_table",
        ":pointee_type",
        ":type_lifetimes",
        "//third_party/absl/strings",
        "@llvm///clang:ast",
        "@llvm///clang:basic",
        "@llvm///clang:frontend",
        "@llvm///clang:lex",
        "@llvm///llvm:Support",
    ],
)

cc_test(
    name = "lifetime_annotations_test",
    srcs = ["lifetime_annotations_test.cc"],
    deps = [
        ":lifetime_annotations",
        "//testing/base/public:gunit_main",
        "//lifetime_annotations/test:named_func_lifetimes",
        "//lifetime_annotations/test:run_on_code",
        "@llvm///clang:ast_matchers",
        "@llvm///llvm:Support",
    ],
)

cc_library(
    name = "lifetime_symbol_table",
    srcs = ["lifetime_symbol_table.cc"],
    hdrs = ["lifetime_symbol_table.h"],
    deps = [
        ":lifetime",
        "@llvm///llvm:Support",
    ],
)

cc_test(
    name = "lifetime_symbol_table_test",
    srcs = ["lifetime_symbol_table_test.cc"],
    deps = [
        ":lifetime_symbol_table",
        "//testing/base/public:gunit_main",
    ],
)

cc_library(
    name = "pointee_type",
    srcs = ["pointee_type.cc"],
    hdrs = ["pointee_type.h"],
    deps = [
        "@llvm///clang:ast",
    ],
)
