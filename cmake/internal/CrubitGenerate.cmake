# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

include_guard(GLOBAL)

# Returns the Cargo dependency lines every bindings crate needs: the Crubit
# runtime crates the generated code refers to (see `deps_for_bindings` in
# rs_bindings_from_cc/BUILD).
function(_crubit_runtime_cargo_deps out_var)
  _crubit_source_dir(crubit_dir)
  set(support "${crubit_dir}/support")
  set(deps "")
  # Crates whose Bazel targets share a directory get their Cargo manifests
  # under cargo/support, at the path of the crate's sources in support.
  foreach(crate IN ITEMS ctor operator forward_declare oops cref
                         rs_std/dyn_callable_rs rs_std/lossy_formatter
                         extract_cpp_from_rust/inline_cpp_macro)
    cmake_path(GET crate FILENAME name)
    string(APPEND deps
      "${name} = { path = \"${crubit_dir}/cargo/support/${crate}\" }\n")
  endforeach()
  foreach(crate IN ITEMS ffi_11 crubit_support crubit_annotate)
    string(APPEND deps "${crate} = { path = \"${support}/${crate}\" }\n")
  endforeach()
  string(APPEND deps
    "bridge_rust = { package = \"crubit_bridge_rust\", path = \"${support}/crubit_bridge_rust\" }\n"
    "static_assertions = \"1\"\n")
  set(${out_var} "${deps}" PARENT_SCOPE)
endfunction()

# _crubit_generate(
#   CRATE <name>                   Rust crate name of the bindings.
#   LABEL <label>                  Synthetic Bazel-style label of the C++ target.
#   CLANG_ARGS <arg>...            Clang args (may contain generator expressions,
#                                  each element one or more lines).
#   PUBLIC_HEADERS <header>...     Absolute paths (may be generator expressions).
#   TARGET_ARGS_JSON <json>...     One `--target_to_arg` entry per target.
#   [EXTRA_RS_SRCS <file[=ns]>...] Extra Rust sources for the crate.
#   [LINK <target>...]             C++ targets the thunks link against.
#   [DEP_CRATES <crate>...])       Other bindings crates this crate uses.
#
# Generates bindings with rs_bindings_from_cc and creates:
#   - target `crubit_generate_<name>`, which runs the generator;
#   - static library `crubit::<name>`, the C++ thunks;
#   - `${CRUBIT_CRATES_DIR}/<name>/Cargo.toml`, the Rust crate, patched into
#     `${CRUBIT_CARGO_CONFIG}` as `<name> = "0.0.0"`.
function(_crubit_generate)
  cmake_parse_arguments(PARSE_ARGV 0 ARG ""
    "CRATE;LABEL"
    "CLANG_ARGS;PUBLIC_HEADERS;TARGET_ARGS_JSON;EXTRA_RS_SRCS;LINK;DEP_CRATES")
  set(crate "${ARG_CRATE}")
  set(dir "${CRUBIT_CRATES_DIR}/${crate}")
  set(rs_out "${dir}/${crate}_rust_api.rs")
  set(cc_out "${dir}/${crate}_rust_api_impl.cc")
  set(ns_out "${dir}/${crate}_namespaces.json")
  # The arguments, one per line. CrubitQuoteParams.cmake turns them into the
  # param file `params` at build time.
  set(rsp "${dir}/${crate}.rsp")
  set(params "${dir}/${crate}.params")
  set(lines
    "--target=${ARG_LABEL}"
    --rs_out "${rs_out}"
    --cc_out "${cc_out}"
    --namespaces_out "${ns_out}"
    --crubit_support_path_format "\"support/{header}\"")
  list(JOIN ARG_EXTRA_RS_SRCS "," extra_rs_srcs)
  if(extra_rs_srcs)
    list(APPEND lines "--extra_rs_srcs=${extra_rs_srcs}")
  endif()
  foreach(json IN LISTS ARG_TARGET_ARGS_JSON)
    list(APPEND lines --target_to_arg "${json}")
  endforeach()
  _crubit_builtin_headers_target_arg(builtin_json)
  list(APPEND lines --target_to_arg "${builtin_json}")
  list(APPEND lines --)
  list(JOIN lines "\n" content)
  string(APPEND content "\n")
  string(PREPEND content
    "--public_headers=$<JOIN:${ARG_PUBLIC_HEADERS},$<COMMA>>\n")
  _crubit_source_dir(crubit_dir)
  string(APPEND content
    "$<JOIN:${ARG_CLANG_ARGS},\n>\n"
    "-resource-dir\n${CRUBIT_CLANG_RESOURCE_DIR}\n"
    "-I${crubit_dir}\n"
    "-include\n$<JOIN:${ARG_PUBLIC_HEADERS},\n-include\n>\n")
  file(GENERATE OUTPUT "${rsp}" CONTENT "${content}")

  _crubit_rs_bindings_from_cc(tool)
  add_custom_command(
    OUTPUT "${rs_out}" "${cc_out}" "${ns_out}"
    COMMAND "${CMAKE_COMMAND}" "-DIN=${rsp}" "-DOUT=${params}"
      -P "${crubit_dir}/cmake/internal/CrubitQuoteParams.cmake"
    COMMAND "${tool}" --stderrthreshold=2 "@${params}"
    DEPENDS "${tool}" "${rsp}"
      "${crubit_dir}/cmake/internal/CrubitQuoteParams.cmake"
      ${ARG_PUBLIC_HEADERS}
    COMMENT "Generating Rust bindings for C++: ${crate}"
    VERBATIM)
  add_custom_target(crubit_generate_${crate} DEPENDS "${rs_out}" "${cc_out}")

  add_library(crubit_impl_${crate} STATIC "${cc_out}")
  add_library(crubit::${crate} ALIAS crubit_impl_${crate})
  add_dependencies(crubit_impl_${crate} crubit_generate_${crate})
  target_link_libraries(crubit_impl_${crate} PUBLIC crubit_support ${ARG_LINK})
  target_compile_features(crubit_impl_${crate} PUBLIC cxx_std_20)

  _crubit_runtime_cargo_deps(deps)
  list(TRANSFORM ARG_DEP_CRATES APPEND " = \"0.0.0\"\n" OUTPUT_VARIABLE dep_lines)
  list(JOIN dep_lines "" dep_lines)
  string(APPEND deps "${dep_lines}")
  file(CONFIGURE OUTPUT "${dir}/Cargo.toml" @ONLY CONTENT [[
# Automatically @generated by Crubit. Do not edit.

[package]
name = "@crate@"
version = "0.0.0"
edition = "2024"

[lib]
path = "@rs_out@"

[dependencies]
@deps@
# Crubit's sources use `cfg(feature = ...)` values that Cargo does not know.
[lints.rust]
unexpected_cfgs = "allow"

[workspace]
]])
  # Redirect the `${crate} = "0.0.0"` dependency to this crate (see
  # CRUBIT_CARGO_CONFIG in _crubit_init_toolchain).
  file(APPEND "${CRUBIT_CARGO_CONFIG}" "${crate} = { path = \"${dir}\" }\n")
endfunction()
