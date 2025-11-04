# Debugging Crubit

<internal link>/overview/debugging

<!--*
# Document freshness: For more information, see <internal link>.
freshness: { owner: 'mvanbem' reviewed: '2025-11-04' }
*-->

Crubit is trying to solve a hard problem and is under rapid development. Things
go wrong, and it's nice to have tools and methods to diagnose those problems.

[TOC]

## Verbose Logging

NOTE: This feature is under development and is planned to be complete by the end
of 2025. There might not be much in the logs yet. See b/449749452 for status,
and the design doc <internal link>.

Crubit tool invocations only log at the error level or higher to keep spam out
of build logs, but the stderr logging verbosity level is configurable.

We have a shell script checked in that automates the process of invalidating
your local Bazel cache and rerunning Crubit on a specific target while emitting
info logs:

```
common/bazel_support/run_with_verbose_logging.sh
```

This script accepts zero or more Bazel flags (each starting with `--`) followed
by one Bazel target label to be built.

### Example usage with cc_bindings_from_rs

```bash
$ common/bazel_support/run_with_verbose_logging.sh \
    --config=llvm-unstable \
    //third_party/rust/bytes/v1:bytes
```

### Example usage with rs_bindings_from_cc:

```bash
$ common/bazel_support/run_with_verbose_logging.sh \
    --config=llvm-unstable \
    //file/base
```

### Implementation details

The stderr logging level is configured through the Bazel custom build setting
`//common/bazel_support:verbose_log_targets`, which accepts a
list of strings that are each interpreted as a target label. It's probably
easier to just run the script.
