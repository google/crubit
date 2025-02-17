#!/bin/bash
# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception


# A script to test that we are passing the system headers include directives
# in the correct order.

# This script (check_include_directives.sh) is not actually the test executable
# but rather _generates_ the test executable (which is another script).
# `path_to_test_exeutable` is the path to the test executable that we should
# generate.
#
# We do things this way so that this script can access the Crubit command line
# arguments, which in turn are extracted using an aspect.
#
# If the checks in the current script (check_include_directives.sh) pass, we
# generate a `path_to_test_executable` script that exits with a success code.
#
# If the checks fail, we generate a `path_to_test_executable` script that prints
# an error message and exits with a failure code.
path_to_test_executable="$1"
shift

# Remove all arguments before the `--` from the command line (these are intended
# for Crubit, not Clang).
while [[ "$1" != "--" ]]; do
    shift
done
# Also remove the "--" itself
shift

# Run `clang` with the given command line arguments. Add `-v` so that it
# prints the system include paths, and extract this list of paths from the
# output.
sys_includes=$("${CC}" "$@" -v -E -o /dev/null -x c++ /dev/null 2>&1 \
    | awk '/#include <\.\.\.> search starts here/,/End of search list/')

# TODO(mboehme): If we remove the `crubit_flavor_transition` from the
# `cc_std_test` rule, we need to add the "stable" versions of these paths so
# that the test passes in the stable configuration.
pattern="#include <\.\.\.> search starts here: \+"
pattern+="third_party/stl/cxx17 \+"
pattern+="third_party/stl/itanium_abi \+"
pattern+="nowhere/llvm/src/libcxx/include \+"
pattern+="End of search list\."

# We don't want to assume that the installed grep supports PCRE mode, so we
# replace newlines with spaces.
echo "${sys_includes}" | tr '\n' ' ' | grep "${pattern}"
grep_error=$?

if [[ $grep_error == 0 ]]; then
    cat >> "${path_to_test_executable}" <<EOF
echo "Success!"
exit 0
EOF
else
    cat >> "${path_to_test_executable}" <<EOF
echo "Error: Didn't see expected system include directories."
echo "Actual include directories:"
cat <<SYS_INCLUDES_END
EOF

    echo "${sys_includes}" >> "${path_to_test_executable}"
    cat >> "${path_to_test_executable}" <<EOF
SYS_INCLUDES_END
exit 1
EOF
fi
