// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Parses C++ headers and generates:
// * a Rust source file with bindings for the C++ API
// * a C++ source file with the implementation of the bindings

#include <cstdlib>
#include <string>
#include <utility>
#include <vector>

#include "common/check.h"
#include "common/file_io.h"
#include "common/status.h"
#include "common/status_macros.h"
#include "rs_bindings_from_cc/cmdline.h"
#include "rs_bindings_from_cc/collect_namespaces.h"
#include "rs_bindings_from_cc/generate_bindings_and_metadata.h"
#include "rs_bindings_from_cc/ir.h"
#include "rs_bindings_from_cc/profiling.h"
#include "rs_bindings_from_cc/src_code_gen_ffi.h"
#include "llvm/ADT/ScopeExit.h"
#include "llvm/ADT/Twine.h"
#include "llvm/Support/Error.h"
#include "llvm/Support/FormatVariadic.h"
#include "llvm/Support/InitLLVM.h"
#include "llvm/Support/JSON.h"
#include "llvm/Support/raw_ostream.h"

namespace crubit {

std::string InstantiationsAsJson(
    const BindingsAndMetadata& bindings_and_metadata) {
  llvm::json::Object obj;
  for (const auto& entry : bindings_and_metadata.instantiations) {
    obj[llvm::StringRef(entry.first.Ident())] =
        std::string(entry.second.Ident());
  }
  return std::string(llvm::formatv("{0:2}", llvm::json::Value(std::move(obj))));
}

llvm::Error Main(int argc, const char* const* argv) {
  CRUBIT_ASSIGN_OR_RETURN(Cmdline cmdline, Cmdline::FromArgv(argc, argv));
  const CmdlineArgs& args = cmdline.args();

  if (args.do_nothing) {
    CRUBIT_RETURN_IF_ERROR(SetFileContents(
        args.rs_out,
        "// intentionally left empty because --do_nothing was passed."));
    CRUBIT_RETURN_IF_ERROR(SetFileContents(
        args.cc_out,
        "// intentionally left empty because --do_nothing was passed."));
    if (!args.instantiations_out.empty()) {
      CRUBIT_RETURN_IF_ERROR(SetFileContents(args.instantiations_out, "[]"));
    }
    if (!args.namespaces_out.empty()) {
      CRUBIT_RETURN_IF_ERROR(SetFileContents(args.namespaces_out, "[]"));
    }
    return llvm::Error::success();
  }

  CRUBIT_ASSIGN_OR_RETURN(
      BindingsAndMetadata bindings_and_metadata,
      GenerateBindingsAndMetadata(cmdline, args.clang_args));

  if (!args.ir_out.empty()) {
    CRUBIT_RETURN_IF_ERROR(
        SetFileContents(args.ir_out, FormatIrDebug(bindings_and_metadata.ir)));
  }

  std::string extra_cpp_srcs_content;
  for (const std::string& cc_src : args.extra_cpp_srcs) {
    CRUBIT_ASSIGN_OR_RETURN(std::string cc_src_content,
                            GetFileContents(cc_src));
    extra_cpp_srcs_content += "\n#line 1 \"";
    extra_cpp_srcs_content += cc_src;
    extra_cpp_srcs_content += "\"\n";
    extra_cpp_srcs_content += cc_src_content;
    extra_cpp_srcs_content += "\n";
  }
  bindings_and_metadata.rs_api_impl =
      extra_cpp_srcs_content + bindings_and_metadata.rs_api_impl;

  CRUBIT_RETURN_IF_ERROR(
      SetFileContents(args.rs_out, bindings_and_metadata.rs_api));
  CRUBIT_RETURN_IF_ERROR(
      SetFileContents(args.cc_out, bindings_and_metadata.rs_api_impl));

  if (!args.instantiations_out.empty()) {
    CRUBIT_RETURN_IF_ERROR(SetFileContents(
        args.instantiations_out, InstantiationsAsJson(bindings_and_metadata)));
  }

  if (!args.namespaces_out.empty()) {
    CRUBIT_RETURN_IF_ERROR(SetFileContents(
        args.namespaces_out,
        crubit::NamespacesAsJson(bindings_and_metadata.namespaces)));
  }

  if (!args.error_report_out.empty()) {
    CRUBIT_RETURN_IF_ERROR(SetFileContents(args.error_report_out,
                                           bindings_and_metadata.error_report));
  }

  return llvm::Error::success();
}

}  // namespace crubit

extern "C" int crubit_rs_bindings_from_cc_main(int argc, char* argv[]) {
  llvm::InitLLVM init(argc, argv);
  if (const char* prefix = getenv("RS_BINDINGS_ALLOC_RECORDER_PREFIX")) {
    crubit::StartHeapProfiling(prefix);
  }
  auto cleanup = llvm::scope_exit([] { crubit::StopHeapProfiling(); });
  llvm::Error status = crubit::Main(argc, argv);
  if (status) {
    llvm::errs() << llvm::toString(std::move(status)) << "\n";
    return -1;
  }
  return 0;
}
