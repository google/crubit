# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

# Script mode: cmake -DIN=<file> -DOUT=<file> -P CrubitQuoteParams.cmake
#
# Converts IN, a list of arguments with one argument per line, into OUT, a
# param file for rs_bindings_from_cc, which reads param files with GNU-style
# quoting (llvm::cl::TokenizeGNUCommandLine): whitespace separates arguments,
# and quotes and backslashes are special. Each special character is escaped
# with a backslash, which is how Bazel writes param files too.
#
# _crubit_generate writes IN with file(GENERATE), so that it can contain
# generator expressions. Their values can't be escaped at generate time (no
# generator expression does that), hence this build step. Empty lines are
# dropped, as GNU-style quoting would drop them anyway, and an argument can't
# contain a newline.
cmake_minimum_required(VERSION 3.22)

file(READ "${IN}" content)
string(ASCII 11 vt)
string(ASCII 12 ff)
# The backslash first, so that the backslashes added below aren't escaped.
foreach(c "\\" "\"" "'" " " "\t" "\r" "${vt}" "${ff}")
  string(REPLACE "${c}" "\\${c}" content "${content}")
endforeach()
file(WRITE "${OUT}" "${content}")
