# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
"""Shared constants for locating run time environment libraries.

This is included, transitively, by the top level MODULE.bazel. Do not add anything except the shared
constants between the toolchain definitions and registrations for cc_std / GRTE.

If this file grows significantly (e.g. to ~100 entries), this will need to be redesigned to avoid
placing too much strain on users.
"""

# Mapping of GRTE CPU name to the config_setting for that CPU.
CPU = {
    "arm": "//support/cc_std:cpu_arm",
    "x86": "//support/cc_std:cpu_x86",
    "diorite_acc": "//buildenv/platforms/settings:diorite_acc",
    "diorite_imc": "//buildenv/platforms/settings:diorite_imc",
}

# Mapping of GRTE version to target.
GRTE = {
    "v4": "//third_party/grte:grte_v4",
    "v5": "//third_party/grte:grte_v5",
}

_CPU_GRTE_VERSIONS_OVERRIDE = {
    # diorite is v5-only.
    "diorite_acc": ["v5"],
    "diorite_imc": ["v5"],
}

# Mapping of GRTE CPU name to the list of GRTE versions for that CPU.
CPU_GRTE_VERSIONS = {
    cpu: _CPU_GRTE_VERSIONS_OVERRIDE.get(cpu, sorted(GRTE))
    for cpu in CPU
}
