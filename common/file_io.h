// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_COMMON_FILE_IO_H_
#define CRUBIT_COMMON_FILE_IO_H_

#include <string>

#include "llvm/ADT/StringRef.h"
#include "llvm/Support/Error.h"

namespace crubit {

llvm::Expected<std::string> GetFileContents(llvm::StringRef path);

llvm::Error SetFileContents(llvm::StringRef path, llvm::StringRef contents);

}  // namespace crubit

#endif  // CRUBIT_COMMON_FILE_IO_H_
