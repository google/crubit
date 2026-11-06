# Opt-in Safe Bindings for Reference-Like C Pointer Parameters

## Problem

`rs_bindings_from_cc` binds every C/C++ pointer parameter as a raw pointer
(`*mut T` or `*const T`), so functions that accept them become `unsafe fn`. This
is the correct default, but it means Rust callers must wrap every call in
`unsafe`, even when the contract is simple: the pointer is non-null, is not
retained past the call, and does not have surprising aliasing rules.

For example, many C libraries use pointers simply to pass by reference:

```c
void ngl_tsc_global_dev_init(ngl_tsc_global_dev_t* dev,
                             const ngl_tsc_global_init_params_t* params);
```

Callers currently write `unsafe { ngl_tsc_global_dev_init(...) }`. We want to
allow C/C++ libraries to opt into treating such pointers as Rust references,
automatically yielding safe `&T` or `&mut T` bindings.

## Design

### The Rule

In an opted-in target's own headers, parameters with type `T* _Nonnull` and the
`[[clang::noescape]]` attribute (or `__attribute__((noescape))`) bind exactly as
`T&` would under `assume_lifetimes`.

Specifically:

-   `const T* _Nonnull [[clang::noescape]]` maps to `&T`.
-   `T* _Nonnull [[clang::noescape]]` maps to `&mut T`.
-   The parameter participates in normal lifetime elision.
-   If it has `[[clang::lifetimebound]]`, that is handled identically to a
    reference.
-   If no other parameters are unsafe, the function becomes a safe `fn`.

**Safety note:** `&mut T` mapped from a C `T*` out-parameter requires the Rust
caller to pass an initialized `&mut T`, meaning they cannot pass uninitialized
memory without using wrappers like `MaybeUninit`. This provides a safe, standard
contract.

**Edge cases:**

-   `void*` maps to `*mut c_void` (cannot map to a Rust reference because
    `c_void` is an incomplete type in Rust).
-   Incomplete pointees map to raw pointers, just as incomplete references do.
-   Pointer-to-pointer maps to a reference to a pointer (e.g.
    `int** _Nonnull [[clang::noescape]]` maps to `&mut *mut i32`).
-   Function pointers are already handled directly by Crubit, but if passed by
    pointer, they map to a reference to a function pointer.

### Opt-in Mechanism

The feature is opt-in per target via a Crubit feature flag in
`//features:BUILD`, `noescape_pointer_params_as_references`,
applied to the `rust_api_from_cpp` (or via aspect hint).

This feature only affects declarations originating from the target that
specifies the feature (i.e., its own headers), not transitive headers. This
ensures that a library cannot alter the bindings of its dependencies.

### Annotation Sources

Headers can be annotated in two ways:

1.  **Direct Attributes:** Modifying the header to add `_Nonnull` (or
    `absl_nonnull`) and `__attribute__((noescape))`.
2.  **Clang API Notes:** For unmodifiable third-party C headers, Clang API notes
    allow adding attributes externally (`Nullability: N` and `NoEscape: true`).

### Slices

A pointer parameter whose length is dictated by another parameter (e.g.,
annotated with `counted_by`) should bind as a single `&[T]` or `&mut [T]`
parameter.

Because Clang's `BoundsSafety` is parsed but not applied in `Sema`, Crubit must
read the `BoundsSafety` attribute directly from API notes.

-   A counted pointer must *never* take the single-reference path, even when
    annotated as non-null and noescape.
-   `counted_by_or_null` / `sized_by_or_null` map to `Option<&[T]>`.
-   `sized_by` (in bytes) is supported only for byte-like pointees (`u8` or
    `c_void` -\> `&[u8]`).
-   The generated C++ thunk receives a Rust slice (pointer + length), and calls
    the C function, passing the pointer and the length separately.
-   The conversion handles length overflow checks (panicking or returning an
    error if the length does not fit in the target type, e.g., `uint32_t`).

*Out of scope for v1:* `counted_by(*len)` (counts passed by pointer), computed
sizes, pointer return values, and nullable pointers without lengths mapping to
`Option<&T>` (though possible in the future).

### Safety Argument and Failure Modes

If a library opts into `noescape_pointer_params_as_references`, it asserts that
its annotated pointers obey Rust's aliasing and lifetime rules.

If the library author incorrectly annotates a pointer that *does* escape or is
aliased in violating ways, it results in undefined behavior (UB) in safe Rust.
The responsibility for this correctness lies with the library author who applied
the annotations and opted into the feature.

Reviewers of an `.apinotes` file or an opt-in PR should treat it with the same
scrutiny as an `unsafe` block in Rust. The annotations are trusted/unsafe
claims, identical in category to `CRUBIT_UNSAFE_MARK_SAFE`.

### Worked Example

**Before (C header):**

```c
void ngl_tsc_global_dev_init(ngl_tsc_global_dev_t* dev,
                             const ngl_tsc_global_init_params_t* params);

int ngl_tsc_set_queues_ingress_suspend(
    const ngl_tsc_indirect_dev_t* dev,
    uint32_t start_group_id, uint32_t len,
    const uint32_t* bitmaps);
```

**After (API Notes file `ngl.apinotes`):**

```yaml
Name: ngl
Functions:
  - Name: ngl_tsc_global_dev_init
    Params:
      - Position: 0
        Nullability: N
        NoEscape: true
      - Position: 1
        Nullability: N
        NoEscape: true
  - Name: ngl_tsc_set_queues_ingress_suspend
    Params:
      - Position: 0
        Nullability: N
        NoEscape: true
      - Position: 3
        Nullability: N
        NoEscape: true
        BoundsSafety:
          Kind: CountedBy
          BoundedBy: "len"
```

**Proposed BUILD API:**

```python
rust_api_from_cpp(
    name = "ngl_rust_api",
    cpp_target = ":ngl",
    crubit_features = ["//features:noescape_pointer_params_as_references"],
    apinotes = ["ngl.apinotes"],
)
```

**After (Rust):**

```rust
pub fn ngl_tsc_global_dev_init(
    dev: &mut ngl_tsc_global_dev_t,
    params: &ngl_tsc_global_init_params_t,
); // Fully safe fn!

pub unsafe fn ngl_tsc_set_queues_ingress_suspend(
    dev: &ngl_tsc_indirect_dev_t,
    start_group_id: u32,
    len: u32,
    bitmaps: *const u32,
) -> i32;
```

*(Once slice support lands, `bitmaps` and `len` would collapse into
`bitmaps: &[u32]`, making `ngl_tsc_set_queues_ingress_suspend` completely
safe).*

Of the 28 `unsafe` blocks in `//net/nighthawk/nightagent/hal/nhk/ngl`, v1
(references only) is estimated to remove about half, while v1 + slices will
remove the remainder.

### Alternatives Considered

1.  **Wrapper Header:** Write a C++ wrapper inline that takes `T&` and calls the
    underlying C function. This is tedious, requires maintaining a separate
    wrapper API, and doesn't scale to many functions.
2.  **Treating all `_Nonnull` pointers as references:** `_Nonnull` only claims
    that the pointer is not null; it makes no claims about aliasing or whether
    the pointer escapes. Treating all `_Nonnull` pointers as references would be
    unsound for functions that retain the pointer.
3.  **Per-parameter explicit API-notes key (e.g. `Reference: true`):** Instead
    of a per-target opt-in, we could require a specific key on each parameter.
    However:
    -   The per-target opt-in ensures a specific library makes the decision, so
        existing `_Nonnull` code elsewhere is never affected.
    -   `noescape` is already an explicit per-parameter claim, so accidental
        conversion needs both the target opt-in and a deliberate `noescape`.
    -   A new Crubit-specific API-notes key isn't possible without changing
        Clang's API-notes YAML schema, because unknown keys fail parsing.
        Reusing standard keys keeps the files valid for Clang and Swift tooling.

## Open Questions

-   **Resolved: How do users provide API notes?** Via an
    `apinotes = ["foo.apinotes"]` attribute on `rust_api_from_cpp`.
    -   *Investigation:* Attempting to load API notes via standard Bazel `copts`
        (`-fapinotes`, `-fmodule-name=...`, `-iapinotes-modules=...`) and
        placing the file in `hdrs` fails to load annotations without module maps
        / full modules. The aspect must instead pass `-iapinotes-modules=<dir>`
        and `-fmodule-name=<target_module>` matched to a generated `.cppmap`, or
        Crubit must apply API notes itself via
        `clang::api_notes::APINotesReader`.
-   **Resolved: What is the trust model?** It is identical to
    `CRUBIT_UNSAFE_MARK_SAFE`. Wrong annotations lead to UB.
-   **Resolved: Should the opt-in be per-target or per-parameter?** Decided on
    per-target opt-in combined with standard per-parameter `noescape`
    annotations, as detailed in Alternatives Considered.
