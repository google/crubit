# C++ Nullability Analysis

The `nullability` directory contains a comprehensive static analysis system for
bringing robust null-safety to C++. Its primary purpose is to eliminate the
ambiguity inherent in C++ pointers by providing tools to **infer**, **verify**,
and **enforce** nullability contracts (such as `_Nullable` and `_Nonnull`).

This project is a component of **Crubit**, where it enables the generation of
safer and more ergonomic Rust bindings. By explicitly documenting nullability,
Crubit can bind non-null C++ pointers directly to Rust references (`&T`) or
smart pointers, rather than wrapping them in `Option<T>`.

The directory provides two main toolsets:

-   **Nullability inference** (`inference/`) suggests annotations to add to
    existing APIs by analyzing how they are implemented and used across the
    codebase.

-   **Nullability verification** (this directory) ensures that annotated APIs
    are used and implemented safely (e.g., checking nullable pointers before
    dereferencing). This is a local, flow-sensitive analysis suitable for use in
    `clang-tidy`.

These tools are built on Clang, its [dataflow framework][], and its
[nullability annotations][].

## File Overview

### Core Analysis

-   **pointer_nullability_analysis.h / .cc**: Implements the dataflow analysis
    for tracking pointer nullability.
-   **pointer_nullability_diagnosis.h / .cc**: Diagnoses nullability safety
    violations (e.g., dereferencing nullable pointers) based on the analysis
    results.
-   **pointer_nullability_lattice.h / .cc**: Defines the lattice (program state)
    used in the dataflow analysis.
-   **type_transferer.h / .cc**: Handles the propagation of **static,
    type-based** nullability information. It computes the nullability of each
    C++ type in the AST (e.g., the nested pointer types in `vector<int*>`) in a
    non-flow-sensitive manner, providing a baseline for the analysis.
-   **value_transferer.h / .cc**: Handles the propagation of **flow-sensitive,
    value-based** nullability properties. It models how the nullability state of
    specific pointer values changes at different program points due to control
    flow, such as becoming "known non-null" after a successful null check or
    dereference.

### Data Model

-   **type_nullability.h / .cc**: Defines the `TypeNullability` model,
    representing nullability for all pointer "slots" within a complex C++ type.
-   **pointer_nullability.h / .cc**: Extends the dataflow framework's `Value`
    model to track properties like `is_null` and `from_nullable` for pointer
    values.
-   **pragma.h / .cc**: Handles `#pragma nullability` directives for setting
    per-file nullability defaults.

### Utilities and Helpers

-   **annotations.h**: Defines string constants containing the literal text of
    supported nullability attributes (e.g., `_Nullable`, `_Nonnull`) and Abseil
    macros (e.g., `absl_nullable`).
-   **ast_helpers.h**: Provides helper classes for simplifying access to the
    Clang AST (e.g., matching parameters and arguments).
-   **forwarding_functions.h / .cc**: Detects and analyzes forwarding functions
    like `std::make_unique` to improve analysis precision.
-   **loc_filter.h / .cc**: Interface for filtering source locations (e.g.,
    restricting analysis to specific files).
-   **macro_arg_capture.h**: Constants for capturing arguments passed to
    internal macros during inference.
-   **pointer_nullability_matchers.h / .cc**: AST matchers for identifying
    nullability-relevant constructs (pointers, dereferences, smart pointers,
    etc.).
-   **proto_matchers.h / .cc**: GoogleMock matchers for comparing protocol
    buffer messages in tests.
-   **type_and_maybe_loc_visitor.h**: A specialized visitor for simultaneously
    traversing a `Type` and its corresponding `TypeLoc`.

### Subdirectories

-   **formal_methods/**: Contains formal specifications or models related to
    nullability.
-   **google/**: Google-specific regression and crash tests using real-world
    code snippets.
-   **inference/**: Implementation of the whole-codebase nullability inference
    system.
-   **test/**: Additional shared testing infrastructure and data.

## Style

This directory mostly uses [LLVM-style][] C++, rather than Google-style C++ used
in the rest of `crubit/`. The goal is to make it easy to upstream into
clang-tidy once mature.

Specifically:

-   We follow the LLVM coding standards, with the exceptions listed here:
    -   We write `// TODO` instead of `// FIXME`.
-   We otherwise avoid relying on absl, using llvm's Support libraries instead.

This list isn't set in stone: we can choose to diverge further from LLVM style,
if it's worth more cost of upstreaming later.

[dataflow framework]: <https://github.com/llvm/llvm-project/tree/main/clang/include/clang/Analysis/FlowSensitive>
[nullability annotations]: <https://clang.llvm.org/docs/AttributeReference.html#nullability-attributes>
[LLVM-style]: <https://llvm.org/docs/CodingStandards.html>
