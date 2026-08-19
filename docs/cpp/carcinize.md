# Carcinize

**Carcinize** (`carcinize`) is a migration tool that automatically converts
existing C++ libraries to Rust libraries containing embedded C++ code via
[`inline_cpp!`](inline_cpp.md).

## Overview {#overview}

Carcinize makes it possible to migrate C++ libraries to Rust incrementally.
Carcinize generates a Rust crate where:

1.  **C++ Structs** are mapped to layout-compatible Rust structs.
2.  **Rust Functions** are generated to wrap the original C++ functions using
    [`inline_cpp!`](inline_cpp.md) blocks.
3.  **Downstream C++ Callers** continue using the library through generated
    `cc_bindings_from_rust` targets.

The generated Rust target will result in the same API and use the same
implementation as the original C++ target, but will be entirely within a Rust
source file. You can then refactor functions one at a time from embedded C++
into idiomatic Rust while verifying behavior with tests at every step.

## How to use Carcinize {#usage}

Carcinize can be run directly on a `cc_library` target in your workspace (the
standard workflow for Bazel users) or in standalone mode on source files (for
non-Bazel environments, automated developer tooling, and AI agents).

### Target mode (Bazel) {#target_mode}

To migrate a `cc_library` in your workspace, run:

```sh
bazel run //google_internal/carcinize:carcinize -- //path/to/pkg:my_target
```

Carcinize will:

1.  Generate the Rust source file `//path/to/pkg/my_target_rs.rs`.
2.  Update `//path/to/pkg/BUILD` to replace the `cc_library` with a
    `rust_library_with_embedded_cpp` and re-export the original target name via
    `cc_bindings_from_rust`.

After the command completes, verify the generated target with your unit tests:

```sh
bazel test //path/to/pkg:...
```

### Standalone mode {#standalone_mode}

For developers in non-Bazel environments (such as Cargo or CMake projects), as
well as automated developer tooling, AI agents, and custom scripts, Carcinize
provides a standalone CLI mode that operates directly on C++ headers and source
files without evaluating Bazel queries or mutating `BUILD` files:

```sh
carcinize \
  --headers path/to/header.h \
  --srcs path/to/source.cc \
  --out path/to/output_rs.rs
```

## Handling partially supported libraries (Incomplete Migration) {#incomplete_migration}

Some C++ libraries contain features that Crubit does not yet bind automatically,
such as C++ function templates or unbindable types.

By default, Carcinize allows incomplete migrations by partitioning unsupported
declarations into companion `global_cpp!` blocks in the generated Rust crate so
the migrated crate compiles out-of-the-box.

If you instead want to strictly require complete Crubit binding generation and
fail if any unsupported C++ declarations are present, pass
`--require_complete_migration` (or `--require-complete-migration`):

```sh
bazel run //google_internal/carcinize:carcinize -- \
  --require_complete_migration //path/to/pkg:my_target
```

In standalone mode:

```sh
carcinize \
  --require_complete_migration \
  --headers path/to/header.h \
  --srcs path/to/source.cc \
  --out path/to/output_rs.rs
```

When `--require_complete_migration` is enabled and unsupported C++ declarations
are present, Carcinize halts with an error:

```
Error: Target //math:math_utils contains unsupported C++ declarations:
  - class template `math::UnsupportedTemplate` (Class templates are not yet supported)
  - function template `math::Clamp` (Function templates are not yet supported)
Migration aborted because --require_complete_migration was specified.
```

### How Incomplete Migration works {#how_incomplete_works}

By default, Carcinize partitions declarations:

1.  **Supported structs** are generated as layout-compatible Rust `struct`
    definitions.
2.  **Supported functions** are generated as Rust functions whose bodies wrap
    the C++ implementation using `inline_cpp!` blocks.
3.  **Unsupported declarations** (such as class or function templates) are
    emitted inside companion `global_cpp!` blocks in the generated Rust file.

### Incomplete Migration example {#incomplete_example}

Consider a C++ header containing both a standard function and a C++ template:

```
{{ #include ../../examples/cpp/carcinize/math_utils.h }}
```
<!--  content:^namespace\s+math\s*\{[\s\S]*?^\}\s*//\s*namespace\s+math -->


When migrated, Carcinize generates `math_utils_rs.rs`:

```
{{ #include ../../examples/cpp/carcinize/math_utils_rs.rs }}
```
<!--  content:^\s*global_cpp!\s*\{[\s\S]*?^\}\s*pub\s+mod\s+math\s*\{[\s\S]*?^\} -->


### Caller access {#incomplete_downstream}

`rust_library_with_embedded_cpp` extracts the companion C++ code from
`global_cpp!` and compiles it into a companion C++ header.
`cc_bindings_from_rust` re-exports both the Rust bindings and the companion
header back to C++ callers.

*   **C++ callers** retain access to all declarations, including templates
    (`Clamp<T>`).
*   **Rust code in the crate** can invoke templates from `global_cpp!` with
    concrete types inside `inline_cpp!` blocks (for example, `math::Clamp(val,
    min, max)`).
*   **Rust callers outside the crate** use the generated Rust structs and
    functions (`Vector2`, `DotProduct`).

### Migrating fallback C++ declarations to pure Rust {#resolving_fallbacks}

The generated target compiles immediately without manual intervention.

Migrating fallback declarations to pure Rust is optional. If you choose to
remove C++ dependencies entirely, you can replace the fallback `global_cpp!`
declarations with Rust implementations over time using one of these strategies:

#### Strategy 1: Rewrite in Rust (for Rust callers)

```
{{ #include ../../examples/cpp/carcinize/math_utils_rs.rs }}
```
<!--  function:clamp -->


> NOTE: `cc_bindings_from_rust` does not currently generate C++ function
> templates from Rust generic functions. If existing C++ callers still require
> the C++ template, keep the declaration in the `global_cpp!` block (or provide
> concrete C++ wrappers as shown below) until downstream C++ callers are
> migrated.

#### Strategy 2: Wrap specific instantiations with `inline_cpp!`

```
{{ #include ../../examples/cpp/carcinize/math_utils_rs.rs }}
```
<!--  function:clamp_i32 -->


#### Clean up fallback blocks

Once all declarations in `global_cpp!` have been ported to Rust or are no longer
needed, delete the `global_cpp!` block from the generated file and remove the
corresponding C++ dependencies from `deps_of_cc_library` in your `BUILD` file.

### Governance guards and incremental migration {#governance}

When performing an **incremental migration** across multiple CLs, fallback
`global_cpp!` blocks can be safely checked in and maintained while downstream
callers or unsupported features are migrated over time.

However, if your team wants to enforce complete migration in a single change (or
prevent unmigrated C++ declarations from being accidentally committed during
local experimentation), you can pass `--macro_name DO_NOT_SUBMIT_CPP_DECL!`:

```sh
bazel run //google_internal/carcinize:carcinize -- \
  --macro_name DO_NOT_SUBMIT_CPP_DECL! \
  //path/to/pkg:my_target
```

Carcinize generates the fallback block wrapped in `DO_NOT_SUBMIT_CPP_DECL!`:

```rust
// Generated with --macro_name DO_NOT_SUBMIT_CPP_DECL!
DO_NOT_SUBMIT_CPP_DECL! {
    #include "math/math_utils.h"

    namespace math {
    template <typename T>
    T Clamp(T val, T min, T max) {
      return val < min ? min : (val > max ? max : val);
    }
    }  // namespace math
}
```

Piper presubmit checks will block submitting changes containing `DO_NOT_SUBMIT`
blocks while still allowing local compilation and testing until all fallbacks
are resolved. The build rules also emit compile-time warnings if declarations
inside fallback blocks become natively supported by Crubit.

## BUILD file transformations {#build_transformations}

### Before migration

Consider a C++ library target:

```
{{ #include ../../examples/cpp/carcinize/BUILD }}
```
<!--  symbol:point_before -->


### After migration

Carcinize updates the `BUILD` file to define a `rust_library_with_embedded_cpp`
target for the migrated Rust library:

```
{{ #include ../../examples/cpp/carcinize/BUILD }}
```
<!--  symbol:point_rs -->


And re-exports the library back to C++ callers under the original target name
using `cc_bindings_from_rust`:

```
{{ #include ../../examples/cpp/carcinize/BUILD }}
```
<!--  symbol:\bpoint\b -->


*   **`rust_library_with_embedded_cpp` (`:point_rs`)**: Compiles the generated
    Rust crate and formats/extracts embedded C++ blocks.
*   **`deps_of_cc_library`**: Lists C++ dependencies required by C++ headers
    used in `inline_cpp!` and `global_cpp!` blocks.
*   **`cc_bindings_from_rust` (`:point`)**: Exposes the Rust library back to C++
    callers under the original target name, so downstream dependencies require
    no changes.

## Generated Rust code {#generated_code}

For a C++ library like:

```
{{ #include ../../examples/cpp/carcinize/point.h }}
```
<!--  content:^namespace\s+geometry\s*\{[\s\S]*?^\}\s*//\s*namespace\s+geometry -->


Carcinize generates `point_rs.rs`:

```
{{ #include ../../examples/cpp/carcinize/point_rs.rs }}
```
<!--  content:^\s*global_cpp!\s*\{[\s\S]*?^\}\s*pub\s+mod\s+geometry\s*\{[\s\S]*?^\} -->


## Refactoring workflow {#workflow}

Once Carcinize generates the initial Rust scaffolding, you can incrementally
refactor the crate into idiomatic Rust:

1.  **Verify Baseline**: Run `bazel test //path/to/pkg:...` to confirm the
    generated target compiles and passes existing unit tests.
2.  **Rewrite Functions in Rust**: Replace `inline_cpp!` blocks one function at
    a time with pure Rust implementations.

    For example, converting embedded C++:

    ```live-snippet
    cs/file:examples/cpp/carcinize/point_rs.rs function:GetX
    ```

    Into pure, idiomatic Rust:

    ```live-snippet
    cs/file:examples/cpp/carcinize/point_refactored.rs function:GetX
    ```

3.  **Idiomatic Rust Types and Traits**: Replace FFI types (such as
    `::ffi_11::c_int`) with standard Rust types (such as `i32`), and derive
    standard traits (`Debug`, `PartialEq`, `Default`).

4.  **Remove C++ Dependencies**: Once all embedded C++ is replaced with pure
    Rust, remove the `global_cpp!` blocks and `deps_of_cc_library` from your
    `BUILD` file.

## Common errors {#errors}

Carcinize and `inline_cpp!` use Crubit under the hood, so any Crubit error can
occur during migration. See crubit.rs/errors and
[Inline C++ in Crubit](inline_cpp.md) for related diagnostic guidance.

### Target collision: `<target>_rs already exists` {#error_collision}

If a target named `<target>_rs` already exists in the `BUILD` file, Carcinize
aborts to avoid overwriting existing code:

```
Error: Target //geometry:point_rs already exists. Aborting to prevent collisions.
```

Rename or remove the existing target before running Carcinize.

### Unsupported C++ declarations {#error_unsupported}

When `--require_complete_migration` is enabled, Carcinize halts with an error if
any unbindable declarations are encountered:

```
Error: Target //math:math_utils contains unsupported C++ declarations:
  - class template `math::UnsupportedTemplate` (Class templates are not yet supported)
  - function template `math::Clamp` (Function templates are not yet supported)
Migration aborted because --require_complete_migration was specified.
```

To proceed with migrating partially supported libraries without halting, omit
`--require_complete_migration` to allow Carcinize to route those declarations
into companion `global_cpp!` fallback blocks automatically.

### Missing C++ dependencies {#error_missing_headers}

If Clang fails to find `#include` headers during compilation:

```
error: 'third_party/absl/strings/str_cat.h' file not found
 #include "third_party/absl/strings/str_cat.h"
          ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
```

Ensure the target exporting those headers is listed in `deps_of_cc_library` in
your `rust_library_with_embedded_cpp` rule (or in `deps` of the original
`cc_library`).

### Unmatched braces in embedded C++ {#error_unmatched_braces}

If C++ code inside `inline_cpp!` or `global_cpp!` has unbalanced braces (for
example, in macros or raw strings), the Rust compiler reports an error when
parsing token trees:

```
error: unexpected closing delimiter: `}`
 --> math_utils_rs.rs:25:5
  |
25|     }
  |     ^ unexpected closing delimiter
```

Ensure all opening and closing braces within the macro block are properly
balanced.
