#!/bin/bash
# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception


# A script to test that we are passing the system headers include directives
# in the correct order.

path_to_test_executable="$1"
shift

has_stl_isystem=0
has_cc_stdlib_isystem=0
has_clang_builtin_isystem=0
has_grte_isystem=0

stl_isystem="third_party/stl/cxx17"
# TODO(b/324159705): Remove old include/c++/v1 paths once libc++ runtimes on
# demand is on by default.
# TODO(mboehme): If we remove the `crubit_flavor_transition` from the
# `cc_std_test` rule, we need to add the "stable" versions of these paths so
# that the test passes in the stable configuration.
cc_std_lib_unstable_isystem_old="nowhere/llvm/toolchain/include/c++/v1"
cc_std_lib_unstable_isystem="nowhere/llvm/src/libcxx/include/*"
clang_builtin_isystem="third_party/llvm/llvm-project/clang/lib/Headers"
grte_isystem="third_party/grte/v5_.*/release/usr/grte/v5/include"

function fail () {
    cat >> "${path_to_test_executable}" <<EOF
echo "$1"
exit 1
EOF
    exit 0
}

echo "echo 'Relevant arguments:'" > "${path_to_test_executable}"
is_isystem=false
for p in "$@"; do
    if $is_isystem; then
        echo "echo ' -isystem $p'" >> "${path_to_test_executable}"
        is_isystem=false
    fi
    if [[ "$p" = "-isystem" ]]; then is_isystem=true; fi
done

while [[ $# -gt 0 ]]; do
    if [[ "$1" = "-isystem" ]]; then
        shift;
        if [[ "$1" = "$stl_isystem" ]]; then
            has_stl_isystem=1
        elif [[ "$1" = "$cc_std_lib_unstable_isystem_old" ]] || \
             [[ "$1" =~ ${cc_std_lib_unstable_isystem} ]] ; then
            if [[  "$has_stl_isystem" = 0 ]]; then
                fail "C++ standard library headers appear before STL headers"
            fi
            has_cc_stdlib_isystem=1
        elif [[ "$1" = "$clang_builtin_isystem" ]]; then
            if [[  "$has_cc_stdlib_isystem" = 0 ]]; then
                fail "Clang builtin headers appear before C++ standard library headers."
            fi
            has_clang_builtin_isystem=1
        elif [[ "$1" =~ ${grte_isystem} ]]; then
            if [[  "$has_clang_builtin_isystem" = 0 ]]; then
                fail "GRTE headers appear before Clang builtin headers."
            fi
            has_grte_isystem=1
        fi
    fi
    shift;
done

if [[ "$has_stl_isystem" = 0 ]]; then
    fail "Failed to send STL -isystem directives to the command line"
elif [[ "$has_cc_stdlib_isystem" = 0 ]]; then
    fail "Failed to send C++ Standard Library -isystem directives to the command line"
elif [[ "$has_clang_builtin_isystem" = 0 ]]; then
    fail "Failed to send Clang builtin -isystem directives to the command line"
elif [[ "$has_grte_isystem" = 0 ]]; then
    fail "Failed to send GRTE -isystem directives to the command line"
fi

cat >> "${path_to_test_executable}" <<EOF
echo "Success!"
exit 0
EOF


