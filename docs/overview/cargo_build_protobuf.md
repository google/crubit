<!-- <internal link> -->

# Protobuf Code Generation in Crubit's Cargo Build

crubit.rs/overview/cargo_build_protobuf

<!--*
# Document freshness: For more information, see <internal link>.
freshness: { owner: 'lukasza' reviewed: '2026-09-03' }
*-->

In Crubit's Cargo build for `rs_bindings_from_cc`, `protoc` is invoked in two
distinct places:

1.  **Externally, before Cargo runs**: `cargo/build/generate_proto_headers.py`
    (invoked by `cargo/build/setup_bazel_env.py` or external orchestrators like
    Chromium's `build_crubit.py`) generates **C++** Protobuf headers and source
    files (`.pb.h` and `.pb.cc`).
2.  **Internally, during `cargo build`**: The `build.rs` scripts of Rust
    Protobuf crates (`cargo/rs_bindings_from_cc/ir_rust_proto/build.rs` and
    `cargo/rs_bindings_from_cc/generate_bindings/generate_bindings_rust_proto/build.rs`)
    invoke `protoc` via `protobuf-codegen` to generate **Rust** bindings.

At first glance, invoking `protoc` in two separate mechanisms might seem
redundant or fragmented. This document explains why this separation exists, the
technical constraints that necessitate it, and the alternative designs that were
evaluated.

-----

## Core Reasons for the Separation

### 1\. C++ Protobufs: Preventing Race Conditions in Parallel Cargo Builds

`rs_bindings_from_cc` is composed of many modular C++ support crates
(`ast_consumer_sys`, `cc_ir_sys`, `decl_importer_sys`, `importer_sys`,
`ir_from_cc_sys`, `src_code_gen_sys`, etc.). Many of these crates compile C++
code that directly or transitively `#include "rs_bindings_from_cc/ir.pb.h"`.

When Cargo builds these crates in parallel:

*   If each crate's `build.rs` attempted to generate `.pb.h` headers into a
    shared directory, parallel `protoc` invocations would race against each
    other, resulting in file locking contention, truncated files, and flaky
    build failures.
*   If each crate instead generated `.pb.h` headers into its own isolated
    `$OUT_DIR`, `protoc` would execute redundantly dozens of times. Furthermore,
    crates cannot easily reference headers generated in sibling crates' private
    `$OUT_DIR` paths.

Generating all C++ Protobuf headers into a shared directory **upfront** before
Cargo starts ensures that all C++ compilation units find stable, fully-generated
headers without any parallel file-system races.

### 2\. Rust Protobufs: Relying on `protobuf-codegen` and Native Cargo Serialization

Unlike C++ headers, Rust Protobuf code generation does not suffer from parallel
build races:

*   Rust Protobuf code is generated inside dedicated, isolated crates
    (`cargo/rs_bindings_from_cc/ir_rust_proto` and
    `cargo/rs_bindings_from_cc/generate_bindings/generate_bindings_rust_proto`).
*   Cargo's dependency graph guarantees that `ir_rust_proto` builds sequentially
    before any dependent Rust crates.
*   The generation is driven by `protobuf-codegen` (the official Protobuf crate
    code generator), which handles:
    *   Emitting the required `crate_mapping` configuration so imported types
        resolve across crate boundaries.
    *   Passing toolchain-specific flags (e.g.
        `--rust_opt=experimental-codegen=enabled,kernel=upb`).
    *   Post-processing generated code (e.g. inner/outer attribute adjustments).

Keeping Rust code generation in `build.rs` allows Crubit to rely on standard
Cargo idioms and upstream tooling rather than reverse-engineering
`protobuf-codegen` in custom scripts.

### 3\. External Preparation is Already Required

`rs_bindings_from_cc` depends on C++ libraries (LLVM/Clang LibTooling, Abseil,
and Protobuf runtime libraries) that cannot be built from `crates.io`. An
external build system (Bazel in Crubit's GitHub CI/CQ, CMake in Chromium
*Toolchain* Builds) must build these C++ dependencies first, and a Python script
must discover them and configure environment variables (`CLANG_...`, `ABSL_...`,
`PROTOBUF_...`, `PROTOC`) before `cargo build` can succeed.

Because an external preparation phase is already mandatory, running
`generate_proto_headers.py` as part of that step adds no additional phases or
overhead to the build.

-----

## Alternative Approaches Evaluated

### Alternative A: Move Everything to Python (Pre-generate Rust Code Upfront)

*   **Concept**: Have `generate_proto_headers.py` (or a unified script) generate
    both C++ headers and Rust source files before invoking Cargo.
*   **Why it was rejected**:
    1.  **Fragility & upstream coupling**: Replicating `protobuf-codegen` in
        Python requires manually constructing crate mappings, passing
        experimental `protoc` flags, and applying post-processing text
        replacements. Any upstream update to the `protobuf` crate or its code
        generator would risk silently breaking our custom Python generator.
    2.  **Cargo `$OUT_DIR` isolation**: `ir_rust_proto/lib.rs` includes
        generated code via `include!(concat!(env!("OUT_DIR"), "/ir.rs"));`.
        Because Cargo assigns unpredictable, hash-suffixed `$OUT_DIR` paths per
        build, Python cannot write directly to `$OUT_DIR`. A `build.rs` script
        would still be needed to copy pre-generated files into `$OUT_DIR`,
        offering little benefit over generating them in place with
        `protobuf-codegen`.
    3.  **Generator consistency**: The automated Cargo package generator
        constructs Cargo packages for `rust_proto_library` targets using
        standard `protobuf-codegen` in `build.rs`. Diverging from this model
        would require special-casing proto crates in the generator.

### Alternative B: Move Everything to Cargo via `links` in `Cargo.toml`

*   **Concept**: Declare `links = "ir_cc_proto"` in
    `rs_bindings_from_cc_ir_cc_proto_sys/Cargo.toml` and have its `build.rs`
    export `cargo:include` metadata to dependent crates.
*   **Why it was rejected**:
    *   **Cargo's non-transitive metadata limitation**: Cargo only exposes
        `DEP_<NAME>_<KEY>` metadata to **immediate, direct dependencies**. In
        Crubit, dozens of crates `#include "rs_bindings_from_cc/ir.pb.h"`
        transitively. Every intermediate crate's `build.rs` would have to
        manually forward these environment variables along the entire dependency
        tree. This is fragile, verbose, and difficult for the Cargo package
        generator to maintain.

### Alternative C: Move Everything to Cargo via Ambient File Locking

*   **Concept**: Have C++ `_sys` crates share an output directory, using an
    ambient inter-process file lock (e.g. `flock` on Unix, `LockFileEx` on
    Windows) in `cargo/build` so the first crate generates the C++ headers and
    subsequent crates wait and reuse them.
*   **Why it was rejected**:
    1.  **Violation of Cargo build invariants**: Cargo's documentation
        explicitly advises that `build.rs` should only write inside its own
        `$OUT_DIR`. Writing to shared directories outside `$OUT_DIR` can
        interfere with Cargo's caching, fingerprinting, rebuild detection,
        `cargo clean`, etc.
    2.  **Cross-platform locking complexity**: Robust cross-process file locking
        across Linux, macOS, and Windows (accounting for stale locks after
        aborted builds, permission nuances, and Windows sharing modes) adds
        non-trivial complexity to `cargo/build`.
    3.  **`PROTOC` is still needed**: Because `protoc` is typically hermetic and
        not installed on system `$PATH`, Cargo still requires the `PROTOC`
        environment variable. Ambient locking would not eliminate the need for
        `setup_bazel_env.py` to discover and export `PROTOC`.
