# Nullability Inference

## Purpose

This directory contains the implementation of the Crubit nullability inference
system. The system's goal is to automatically deduce the nullability of C++
pointer-typed symbols (functions, parameters, fields, and global variables) by
analyzing their usage patterns across a codebase.

The system follows a distributed "map-reduce" architecture:

1. **Collection (Map Phase):** Local static analysis (using the Clang dataflow
framework) examines individual translation units to gather "evidence" of
nullability—such as unchecked dereferences (suggesting `Nonnull`) or assignments
from `nullptr` (suggesting `Nullable`).

2. **Merging (Reduce Phase):** Evidence from across the entire codebase is
aggregated by symbol and slot to form a final "conclusion" (e.g., this parameter
is likely `Nonnull`).

3. **Application:** These conclusions can then be propagated back into the
source code as nullability annotations.

## File Descriptions

### Core Logic

*   **collect_evidence.h / .cc**: Implements the "map" phase. It analyzes ASTs
    and CFGs to gather local observations (Evidence) about how symbols are used.
*   **merge.h / .cc**: Implements the "reduce" phase. It consolidates Evidence
    from multiple sources into final nullability conclusions.
*   **inferable.h / .cc**: Defines predicates that determine which C++ symbols
    and types are eligible targets for inference.
*   **eligible_ranges.h / .cc**: Identifies source code ranges (e.g., the exact
    location of a `*` in a declaration) where nullability annotations can be
    inserted.
*   **infer_tu.h / .cc**: Provides a high-level entry point for running the
    entire inference pipeline on a single translation unit, primarily used for
    testing and debugging.
*   **infer_tu_main.cc**: A standalone tool that runs single-translation-unit
    inference on a specified source file.

### Data Models and Utilities

*   **inference.proto**: Protocol buffer definitions for core data structures:
    `Symbol`, `Evidence`, `SlotInference`, and `CFGSummary`.
*   **slot_fingerprint.h / .cc**: Computes stable 64-bit hashes (fingerprints)
    for individual nullability "slots" (e.g., a specific parameter's pointer
    type) to identify them across translation units.
*   **usr_cache.h / .cc**: Provides performance-optimizing caching for Clang
    Unified Symbol Resolution (USR) strings.
* **replace_macros.h / .cc**: Implements a preprocessor-based mechanism to intercept and wrap common assertion macros (like `CHECK`, `DCHECK`, and `CHECK_NE`) with internal "argument-capture" functions. This allows the inference engine to reliably detect these patterns in the AST and collect evidence (e.g., that a checked pointer is `Nonnull`).
* **clang_tidy_nullability_replacement_macros.h**: Contains the alternative macro definitions and capture function templates used by `replace_macros` to expose macro-hidden nullability signals to the analysis.

### Build and Infrastructure

*   **BUILD**: The Bazel/Bazel build configuration for the inference library and
    its associated tools and tests.

### Testing Utilities

*   **augmented_test_inputs.h / .cc**: Helpers for creating synthetic C++ code
    snippets and AST structures for testing the inference engine.
*   **collect_evidence_test_utilities.h / .cc**: Shared infrastructure
    specifically for unit testing the evidence collection logic.
*   **eligible_ranges_for_test.h**: Simple data structures for identifying
    pointer ranges in test code.

### Tests

*   **collect_evidence_test.cc**: Comprehensive tests for the evidence
    collection "map" phase.
*   **eligible_ranges_test.cc**: Tests for identifying annotatable source
    ranges.
*   **infer_tu_test.cc**: Tests for the single-TU inference orchestration.
*   **inferable_test.cc**: Tests for the logic determining what is inferable.
*   **merge_test.cc**: Tests for the evidence aggregation and conclusion logic.
*   **replace_macros_test.cc**: Tests for macro handling during analysis.
*   **slot_fingerprint_test.cc**: Tests for the stable fingerprinting of
    nullability slots.
