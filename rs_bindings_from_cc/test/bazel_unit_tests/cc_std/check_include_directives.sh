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

# Expand any multiline @paramfile entries if Crubit arguments were spilled to parameter files.
expanded_args=()
for arg in "$@"; do
    if [[ "${arg}" == @* ]] && [[ -f "${arg#@}" ]]; then
        mapfile -t _param_args < "${arg#@}"
        expanded_args+=("${_param_args[@]}")
    else
        expanded_args+=("${arg}")
    fi
done
set -- "${expanded_args[@]}"

# Remove all arguments before the `--` from the command line (these are intended
# for Crubit, not Clang).
while (( $# > 0 )) && [[ "$1" != "--" ]]; do
    shift
done
if (( $# == 0 )); then
    cat > "${path_to_test_executable}" <<EOF
#!/bin/bash
echo "Error: Did not find '--' separator in Crubit arguments."
exit 1
EOF
    exit 0
fi
# Also remove the "--" itself
shift

# Run `clang` with the given command line arguments. Add `-v` so that it
# prints the system include paths, and extract this list of paths from the
# output.
sys_includes=$("${CC}" "$@" -v -E -o /dev/null -x c++ /dev/null 2>&1 \
    | awk '/#include <\.\.\.> search starts here/,/End of search list/')

pattern="#include <\.\.\.> search starts here: \+"
pattern+="\(.*cxx17 \+[^ ]*itanium_abi \+.*libcxx/include\|[^ ]*llvm_toolchain[^ ]*/include/c++/v1\) \+"
pattern+=".*clang/[^ ]*/include \+"
pattern+=".*End of search list\."

# We don't want to assume that the installed grep supports PCRE mode, so we
# replace newlines with spaces.
echo "${sys_includes}" | tr '\n' ' ' | grep "${pattern}"
grep_error=$?

if [[ $grep_error == 0 ]]; then
    cat > "${path_to_test_executable}" <<EOF
#!/bin/bash
echo "Success!"
exit 0
EOF
else
    cat > "${path_to_test_executable}" <<EOF
#!/bin/bash
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
