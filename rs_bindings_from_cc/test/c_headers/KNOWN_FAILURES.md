# Known Failures in C Header Bindings

## C vs C++ Parsing

Crubit's `rs_bindings_from_cc` generator (`CppHeaderAnalysis` action) parses
headers using Clang in C++ mode. Constructs that are valid in C but invalid in
C++ fail parsing before bindings can be generated.

Note: The `cc_library` compilation action for `.c` files uses `clang` directly
without C++ forcing (as verified by `bazel aquery`, which shows no `-x c++` for
the `.c` files), so the companion C files are correctly compiled as C. The
failure strictly resides in Crubit's header analysis tool itself.

### Fail to Parse (Syntax Errors)

The following C constructs fail (or previously failed) with Clang C++
compilation errors, and thus their test cases have been moved into
`*_known_failure.h` headers and tagged with `manual` in `BUILD` (reproduce with
`--config=crubit-unstable` so Crubit-at-HEAD is tested):

-   **`_Bool`**: `error: unknown type name '_Bool'` (C++ uses `bool`).
-   **C11 Keywords (`_Alignas`, `_Static_assert`, `_Atomic`)**: Placement
    matters in Clang C++ mode: `struct _Alignas(16)` fails parsing
    (`error: declaration of anonymous struct must be a definition`), whereas
    `_Alignas` before `struct` (`alignment_known_failure.h`) currently emits
    `-Wignored-attributes` and ignores the alignment, and `_Static_assert`
    (`static_assert_known_failure.h`) is accepted as a Clang C11 extension (so
    both `alignment_known_failure_test` and `static_assert_known_failure_test`
    currently pass when run directly). `_Atomic` is unsupported during binding
    generation (`types.h`).
-   **C99 Keywords (`restrict`, `_Complex`)**: Result in parse errors like
    `error: expected ')'` (C++ uses `__restrict` extensions) and `_Complex` is
    not standard C++.
-   **C++ Keywords (`class`)**: When used as identifiers, they result in
    `error: declaration of anonymous class must be a definition`. (Rust keywords
    like `type`, `fn`, `match` bound correctly).
-   **`__int128`**: Emits `error: __int128 is not supported on this target` in
    some target configurations used by the analyzer (e.g. Android 32-bit
    compiled by `multiplatform_rust_test`).

### Linkage Mismatches

-   **Headers without `extern "C"`**: Because Crubit parses headers as C++, any
    C functions declared without `extern "C"` are assumed to be C++ functions
    and their generated Rust bindings use C++ mangled names
    (`_Z25no_extern_c_func_unmangledv`). However, the companion `.c` file
    compiles them as C functions (unmangled). This results in a linker error
    (`undefined symbol: no_extern_c_func_unmangled()`) when the generated Rust
    test tries to call them. (See `no_extern_c_known_failure.h`).

### Missing Bindings (Coverage Gaps)

These items successfully parse in C++ mode but Crubit legitimately reports them
as `could not be bound` or `Unsupported type`, or skips them silently. They act
as coverage gaps compared to `rust-bindgen`:

-   **Macros (`#define`) with Floats or Strings**: Crubit requires the
    `macro_constants` aspect hint
    (`//features:macro_constants`) to bind macros. However,
    `macro_constants.cc` only evaluates integer expressions, so float constants
    (`#define FLOAT_CONST 3.14f`) and string literals
    (`#define STRING_CONST "hello"`) are silently skipped without errors in the
    API file.
-   **Anonymous Structs/Unions** (`nested_anonymous.h`): `Unsupported type
    'struct Nested::(unnamed...)': No generated bindings found for ''`
-   **Enums with explicit underlying types** (`enums.h`):
    `error: enum 'ExplicitUnderlying' could not be bound` (Reason:
    `unknown attribute(s): enum_extensibility, mode` because Crubit's AST
    visitor rejects `__attribute__((enum_extensibility(closed), mode(QI)))`
    during unknown attribute validation).
-   **Flexible / Incomplete Arrays** (`flexible_arrays.h`):
    `Unsupported type 'int[]': Unsupported clang::Type class 'IncompleteArray'`
-   **Zero-Length Arrays** (`zero_length_arrays.h`):
    `Unsupported type 'int[0]': Zero-length C arrays are not supported`
-   **Variadic Functions** (`functions.h`):
    `error: function 'variadic_func' could not be bound`
-   **C Types / Extensions** (`types.h`): `Unsupported builtin type` for
    `wchar_t`, `long double`. `Unsupported clang::Type class 'Vector'` for
    vector types (`__attribute__((vector_size(...)))`).
    `Unsupported clang::Type class 'Complex'` for `_Complex float/double`.
    `Unsupported clang::Type class 'Atomic'` for `_Atomic(int)`.
-   **Unavailable Attributes** (`attributes.h`):
    `error: function 'unavailable_func' could not be bound`
-   **Nested Scopes Passed By-Value** (`nested_scope.h`):
    `InnerScope cannot be passed by-value because it is an incomplete type`.

-   **Volatile Globals** (`qualifiers.h`): `volatile` globals are silently
    skipped or fail to bind.
- **Bitfields**:
  Crubit correctly binds the struct and preserves size and alignment, but it does not generate accessor methods (getters/setters) for bitfield members.

-   **Rust Illegal Raw Identifiers** (`keywords.h`): C identifiers like `self`,
    `Self`, and `crate` cannot be represented as raw identifiers in Rust
    (`r#self` is invalid). Crubit generates them as `__field_X`.
