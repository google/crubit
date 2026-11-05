# Bitwise relocatability and Rust-movability

## 1\. Problem

Crubit decides `Unpin` (Rust-movable) with `IsRustMovable` (in
`rs_bindings_from_cc/importers/cxx_record.cc`), which is "trivial for calls" in
Clang. That has these defects:

| Defect                                                                                                                                                                                                                                                | Example                                                                 | Consequence                                                                 |
| :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------- | :-------------------------------------------------------------------------- |
| **Unsound**: `[[clang::trivial_abi]]` is trusted without looking at members, and Clang does not drop it when a member is non-trivial for calls (llvm#222294).                                                                                        | `struct ABSL_ATTRIBUTE_TRIVIAL_ABI Bar { std::string s; };`             | `Bar` is `Unpin`; Rust `memcpy`s a libstdc++ string → UB.                   |
| **Over-coupled to ABI**: the only opt-in changes the calling convention of the type, with measurable C++ performance side effects and a measurement burden that is itself prohibitive for exactly the low-level, widely used types that benefit most. | `rodos::ResourceNode`                                                   | Owners must take an ABI change to get Rust-movability; many can't or won't. |
| **Platform-dependent**: "trivial for calls" is an ABI notion; the MS ABI uses a different (and, for our purposes, wrong) algorithm, and ptrauth/PFP targets add address-dependence Clang's ABI logic does not model as relocatability.                | same type, different target                                             | Bindings shape differs per platform.                                        |
| **Rejects valid types**: a type whose copy and move constructors are both deleted can never be trivial for calls, even if `memcpy` + forget is perfectly fine for it.                                                                                 | scoped guards, handle types that forbid C++ copies/moves on API grounds | Pinned with no way out.                                                     |
| **No deduction for templates**: libc++ doesn't put `trivial_abi` on `std::optional`, `std::vector`, …, and nothing lets a template inherit the property from its args.                                                                                | `struct S { std::optional<Foo> f; };`                                   | Pinned forever, even when `Foo` is trivially copyable.                      |

Goal: a **platform-independent, verified** predicate `IsBitwiseRelocatable(T)`
driven by `CRUBIT_` annotations, which (a) never widens the set of movable types
without an explicit, local, auditable opt-in, and (b) narrows it where the
compiler *knows* bitwise relocation is wrong (vptrs, address-discriminated
ptrauth, `std::string`).

Guiding principle :
a user should be able to **look at a C++ type and predict whether its binding is
Rust-movable**, and code written against that binding must not depend on the
standard library, sanitizer, pointer-signing or PFP configuration of a given
build. That is unachievable in full generality, so the design (i) makes the
answer a function of the source text plus a closed std table, (ii) turns every
known configuration dependence into either a hard "never" or a hard generation
failure rather than a per-configuration answer, and (iii) writes the reason for
every pinned type into the generated bindings.

## 2\. Definitions

A record `T` is **bitwise-relocatable** (BR) iff, for every object `x` of type
`T`, copying its bytes and then forgetting the original:

```c++
memcpy(dst, &x, sizeof(T));  // and never touch `x` again
```

is observably equivalent to move-constructing a new object and destroying the
original:

```c++
T y(std::move(x));
x.~T();
```

on every platform and build configuration Crubit supports (or, where an explicit
opt-in such as `_STD_STRING_ONLY_ON_LIBCXX` narrows the promise, every
configuration the annotating codebase builds). `Unpin` ⇔ BR.

Three notions are kept distinct and *none* is read from `trivial_abi`:

-   **Implicitly BR**: deducible from the class definition alone (§3.1).
-   **Asserted BR**: the author vouches for the type's own user-provided special
    member functions (move/copy constructor, destructor); Crubit still verifies
    that every field and base class is BR (§3.2–3.3).
-   **Never BR**: hard denylist that no annotation can override (§3.4).

## 3\. The predicate

Computed per `CXXRecordDecl`, memoized in `ImportContext`, result is one of
`Yes`, `No(reason)`, `Error(reason)`.

### 3.1 Implicit rule (no annotation)

`T` is BR iff all of:

1.  `T` is not in the never-set (§3.4).
2.  **Fields and bases**: every base class subobject and every non-static data
    member (field) is BR (§3.5). Union members are checked like struct fields.
3.  **Own special member functions** (move/copy constructor, destructor; P2786
    "default-movable", minus assignment):
    -   the destructor is not user-provided (defaulted-on-first-declaration is
        OK) and not deleted;
    -   the constructor selected for `T(std::declval<T&&>())` is not
        user-provided and not deleted;
    -   at least one copy or move constructor is non-deleted (keeps today's
        "neither copyable nor movable ⇒ address-sensitive by design" rule).

Assignment operators are ignored: Rust never calls them (Rust assignment is drop
\+ `memcpy`, i.e. dtor + move-ctor, which (3) already covers).

This is strictly a superset of today's *platform-independent* results: every
trivially-copyable / trivial type passes (1)–(3), so nothing currently inferred
as `Unpin` without an attribute regresses.

### 3.2 `CRUBIT_UNSAFE_BITWISE_RELOCATABLE` (type-level, asserted + verified)

```c++
class CRUBIT_UNSAFE_BITWISE_RELOCATABLE ResourceNode {
 public:
  ResourceNode(ResourceNode&&);   // allowed
  ~ResourceNode();                // allowed
 private:
  std::unique_ptr<Impl> impl_;    // verified BR
  int* parent_;                   // scalar: BR
};
```

Semantics: rule (3) is waived **in full** — the author asserts that the
user-provided move/copy constructors and destructor are equivalent to `memcpy` +
forget. This includes the "at least one non-deleted copy or move constructor"
clause: a type that deletes both copy and move in C++ (to forbid C++-side
relocation on API grounds, b/461469788#comment3) can still be
bitwise-relocatable, because Rust never calls those constructors — Rust moves
are `memcpy`, and the annotation is the author's statement that this is fine.
Rules (1) and (2) are **still checked**, and a violation is a **hard error** in
`rs_bindings_from_cc` (same mechanism as `CRUBIT_MUST_BIND` failures: bindings
generation for the target fails with a message naming the offending member/base
and why it is not BR). The annotation is a correctness claim, so a false claim
must not be silently downgraded to "pinned".

The predicate never consults `isTrivialForCall()` / `canPassInRegisters`, so the
result is identical on Itanium and MS ABIs and unaffected by whether the type is
actually passed in registers.

Hard errors are raised only when the annotated type is owned by the target whose
bindings are being generated; for types owned by other targets the result is
"not Rust-movable" plus a reason in the generated doc comment (it appears in a
header processed by Crubit); see §7 for the compile-time companion.

### 3.3 `CRUBIT_UNSAFE_BITWISE_RELOCATABLE_IF_ELIGIBLE` (type-level, conditional)

```c++
template <typename T>
class CRUBIT_UNSAFE_BITWISE_RELOCATABLE_IF_ELIGIBLE MyOptional {
  union { char none_; T value_; };
  bool engaged_;
  ~MyOptional() { if (engaged_) value_.~T(); }
  MyOptional(MyOptional&&);
};
```

Same waiver of rule (3) as §3.2, but a violation of (1)/(2) yields `No`, not
`Error`. This is the `trivially_relocatable_if_eligible` analogue: an
instantiation is BR exactly when its (instantiated) members are. It is the tool
for generic containers/wrappers and is what Crubit's own std table (§4) is
expressed in terms of.

### 3.4 Never-set (not overridable)

A type is never BR if:

-   it is polymorphic or has virtual bases (`isPolymorphic() || getNumVBases()`,
    including inherited vptrs). Rationale: vptr may be address-signed (arm64e);
    the answer must not depend on target.
-   Clang marks it `CanNeverPassInRegs` (`__weak`, address-discriminated
    `__ptrauth` members) — already handled today.
-   it contains an address-tagged Pointer-Field-Protection field (§3.8) (this
    only arises under `-fexperimental-pointer-field-protection-tagged` which v1
    rejects outright; untagged `-abi` mode is unaffected).
-   it is annotated with `CRUBIT_NEVER_BITWISE_RELOCATABLE`. Rationale: lets
    authors pin types that look trivially relocatable to Crubit but must not be
    `memcpy`-moved (e.g. their address is registered elsewhere).
-   it is `std::basic_string<...>` (any specialization). Rationale: libc++'s is
    BR but libstdc++'s SSO pointer is self-referential; a platform-independent
    answer must be "no".
-   it has a member/base in the never-set, unless that member is covered by §3.6
    (never-set types that are *compiler*-known unsound — polymorphic, ptrauth,
    `__weak` — cannot be laundered by any override; `std::string` can only be
    laundered by the libc++-only override in §3.6.1).

Applying `CRUBIT_UNSAFE_BITWISE_RELOCATABLE` to a never-set type is a hard
error. `_IF_ELIGIBLE` yields `No`.

### 3.5 Member types

| Member type                                            | BR?                                   |
| :----------------------------------------------------- | :------------------------------------ |
| scalars, enums, pointers, member pointers, `nullptr_t` | yes                                   |
| references (`T&`, `T&&`)                               | yes (pointer semantics; P2786 agrees) |
| arrays `U[N]`                                          | iff `U`                               |
| record `U`                                             | recursive `IsBitwiseRelocatable(U)`   |
| bit-fields, `[[no_unique_address]]`                    | as their type                         |
| anything incomplete / unsupported                      | no                                    |

### 3.6 `CRUBIT_UNSAFE_BITWISE_RELOCATABLE_FIELD` (field-level, trusted)

```c++
struct CRUBIT_UNSAFE_BITWISE_RELOCATABLE Node {
  // SAFETY: offset_ptr stores an integer offset, not an address.
  CRUBIT_UNSAFE_BITWISE_RELOCATABLE_FIELD offset_ptr<Node> next_;
};
```

Treats this one member as BR regardless of its type's status (the type itself is
*not* reclassified elsewhere). `UNSAFE` in the name because Crubit cannot verify
it; it is the escape hatch for `offset_ptr`-style semantics and for third-party
types the author cannot annotate. Cannot launder compiler-known unsound members
(polymorphic, ptrauth, `__weak`) or `std::string` (§3.6.1). Applying it to a
member whose type is already BR is allowed but should produce a (non-fatal) note
so stale overrides get cleaned up.

#### 3.6.1 `CRUBIT_BITWISE_RELOCATABLE_STD_STRING_ONLY_ON_LIBCXX` (policy opt-in)

```c++
class CRUBIT_UNSAFE_BITWISE_RELOCATABLE GURL {
  // This program is only ever built against libc++.
  CRUBIT_BITWISE_RELOCATABLE_STD_STRING_ONLY_ON_LIBCXX std::string spec_;
  ...
};
```

The `GURL` / `url::Origin` case: `std::string` is BR under libc++ but not under
libstdc++ (SSO pointer is self-referential) or MSVC STL. The
platform-independent answer is therefore "no", but a codebase that *knows* it
only ever builds against libc++ (Chromium) should be able to say so, loudly and
locally. This variant:

-   is the only way to treat a `std::basic_string` member as BR, and applies to
    nothing else: `rs_bindings_from_cc` errors if the annotated field's type
    (through arrays) is not a `std::basic_string` specialization. The name
    deliberately spells out all three facts: relocatability, `std::string`, only
    on libc++;
-   is **verified**, not trusted, which is why it carries no `UNSAFE_` prefix:
    the only thing the author asserts is the build policy ("this program is only
    built against libc++"); the facts about libc++ itself (SSO layout, ASan
    behaviour) are Crubit's to validate and are covered by Crubit's tests. A
    call site therefore needs no `SAFETY:` justification;
-   is **enforced**: `rs_bindings_from_cc` errors if the TU it is analysing is
    not compiled against libc++ (`_LIBCPP_VERSION` not defined in the
    preprocessor state). So a libstdc++ build fails at bindings generation
    instead of producing UB at runtime;
-   still cannot launder polymorphic/ptrauth members, and is moot under tagged
    PFP (the whole generation fails, §3.8.1) — libc++'s `std::string` is *not*
    BR there either, since its pointer fields are address-tagged;
-   carries the ASan caveat from §3.8.2: it is sound only while libc++ keeps
    short-string ASan annotations off (`if (__is_long())`); the §6.5 sanitizer
    matrix includes a `std::string` fixture so a libc++ roll that re-enables
    them fails Crubit's tests rather than users' binaries;
-   does not cover `std::string` nested inside another template
    (`std::optional<std::string>`); that is what the codebase-level opt-in below
    is for.

This is the resolution of the "yes, I acknowledge this won't work with
libstdc++/GCC" policy choice: the acknowledgment is spelled in the attribute
name *and* checked by the generator.

**Why an opt-in at all, if `std::string`'s own bindings already differ per
stdlib?** (asked in the thread.) Under this design they don't, for the property
that matters here: `std::string` is in the never-set, so it is pinned on libc++
*and* libstdc++, and so is every type that contains one. The
`_STD_STRING_ONLY_ON_LIBCXX` override is the single construct that makes a
type's Rust-movability depend on build configuration, so it is the single place
that asks for an acknowledgment. What is being acknowledged is not "the shape is
unpredictable" — it stays predictable: either the libc++ shape, or a generation
failure — but the soundness promise: "this field is bitwise- relocatable on
libc++ (and, per §3.8, not under tagged PFP, and only while libc++ keeps
short-string ASan annotations off)". Reading a bare
`CRUBIT_UNSAFE_BITWISE_RELOCATABLE_FIELD` on a `std::string` member would *look*
like the generic offset-ptr-style promise and hide that it is really a promise
about somebody else's library.

**Virality.** The promise is enforced at the program level by the generator (it
fails on non-libc++ TUs whenever a `_STD_STRING_ONLY_ON_LIBCXX` override
actually participates in the BR result of an imported type — not merely when the
macro appears in some included header). So an outer `Foo { Bar bar_; }` where
`Bar` uses the override needs no marker of its own: `Bar` is BR, hence `Foo` is
BR by the implicit rule or a plain `CRUBIT_UNSAFE_BITWISE_RELOCATABLE`. For
discoverability, the generated binding of *every* type whose BR result depended
on a `_STD_STRING_ONLY_ON_LIBCXX` override gets a doc comment
(`/// Rust-movable only when built against libc++ (via field Bar::spec_)`), so
the dependence is visible on `Foo` without being viral in source. A viral
type-level marker (a libc++-only marker required on `Foo` too) was considered:
it buys no soundness over the generator check and would spread across every
transitive user of `GURL`.

#### 3.6.2 Why not an unchecked `CRUBIT_UNSAFE_RUST_MOVABLE` on the type?

Proposed in the :
"like `trivial_abi` but without the ABI effect and without checking fields". It
gives the same power as §3.2 + §3.6 but loses what makes those safer:

-   a type-level blanket waiver silences *every* member, including ones added
    later by someone who never sees the annotation; the field override forces a
    per-member SAFETY comment and keeps the rest of the type verified;
-   it reintroduces the exact failure mode
    of
    (unchecked member types) with no compiler check at all;
-   it hides the libc++-only policy inside a generic name.

`CRUBIT_UNSAFE_BITWISE_RELOCATABLE` +
`CRUBIT_BITWISE_RELOCATABLE_STD_STRING_ONLY_ON_LIBCXX` on `spec_` covers the
`GURL` motivation with two lines and makes the platform assumption explicit.

### 3.7 `[[clang::trivial_abi]]` transition

Decision: `[[clang::trivial_abi]]` on a class is treated as exactly
`CRUBIT_UNSAFE_BITWISE_RELOCATABLE_IF_ELIGIBLE`, and this stays indefinitely to
keep supporting existing APIs (`absl::Status`, `absl::Cord`, and every existing
type that relies on it today).

-   It is a pure *narrowing* of today's behavior, so it fixes the bug class
    immediately without a feature flag: `Bar{std::string}` becomes pinned,
    `absl::Status` / `absl::Cord` keep working because their members are BR.
-   It is sound on its own terms: when the attribute is honored, Clang itself
    relocates the object bitwise on by-value calls, so the author has already
    made the §3.2 claim.
-   The never-set still wins: `std::basic_string` stays pinned even if a
    standard library configuration marks it `[[clang::trivial_abi]]`; conversely
    libc++'s `_LIBCPP_ABI_ENABLE_UNIQUE_PTR_TRIVIAL_ABI` `unique_ptr` becomes BR
    iff its deleter is, consistent with the §4 table.
-   **Docs must say** (in `classes_and_structs.md#rust_movable`, the annotation
    header comment, and the pinned-type doc comment from §6.3):
    `[[clang::trivial_abi]]` / `ABSL_ATTRIBUTE_TRIVIAL_ABI` *may not* make a
    type Rust-movable — it only waives the check on the type's own special
    members; every base and non-static member must still be bitwise-relocatable,
    and Crubit silently leaves the type pinned (with a reason in the generated
    doc comment) if one is not. New code should use
    `CRUBIT_UNSAFE_BITWISE_RELOCATABLE`, which errors instead of silently
    pinning and does not change the C++ ABI.

### 3.8 Build modes that break bitwise relocation (PFP, sanitizers, ptrauth)

Bitwise relocatability is a property of *layout + semantics*, but several build
modes add hidden state keyed on the object's address or kept in shadow memory.
The analysis must account for them; the headline rule is that the **shape of the
bindings must not depend on the platform or build mode** (the Chromium/Crubit
sync-up
goal, ),
so anything that would make a type BR on one configuration and not on another is
resolved to either "never" or "unsupported configuration", not to a per-platform
answer.

#### 3.8.1 Pointer Field Protection

-   **Untagged PFP** (`-fexperimental-pointer-field-protection-abi`, what the
    `pointer_field_protection` toolchain feature used internally passes):
    pointer fields of non-standard-layout types (and of types with
    `[[clang::pointer_field_protection]]`) are encoded with a key derived from
    the *field identity* (type RTTI name + field name), not from the object
    address. `memcpy` of the whole object preserves field identity, so bitwise
    relocation is sound and the BR analysis is unchanged. (Reading such fields
    directly from Rust is a separate problem — deactivation symbols, b/577514207
    — orthogonal to relocation.)

-   **Tagged PFP** (`-fexperimental-pointer-field-protection-tagged`, AArch64
    only): for a PFP field whose parent record is not trivially destructible,
    the encoding additionally mixes in the **object address**
    (`ASTContext::arePFPFieldsTriviallyCopyable`,
    [`ASTContext.cpp`](https://github.com/llvm/llvm-project/blob/main/clang/lib/AST/ASTContext.cpp)).
    A `memcpy` move leaves the pointer encoded for the old address and the next
    C++ read traps — exactly the `TrivialAbiPointer` repro. This affects
    precisely the types this design exists to unlock (user-provided dtor +
    pointer fields): libc++ `std::string`, `std::vector`, `std::unique_ptr`,
    anything annotated `CRUBIT_UNSAFE_BITWISE_RELOCATABLE`. There is no
    annotation an author could write that makes such a type BR under tagged PFP.

    **Decision for v1: `rs_bindings_from_cc` hard-fails when
    `LangOpts.PointerFieldProtectionTagged` is set** ("Crubit does not support
    -fexperimental-pointer-field-protection-tagged"). None of our builds use it
    today, it keeps bindings shape platform-agnostic, and it is a one-line
    check. If tagged-PFP support is ever wanted, the precise rule is already
    computable from the AST: add to the never-set any record for which some
    `PFPField` in `ASTContext::findPFPFields(T)` has
    `!arePFPFieldsTriviallyCopyable(field.FD->getParent())` — accepting that
    bindings then differ between tagged-PFP and other builds.

-   The existing `CanNeverPassInRegs` check already covers address-discriminated
    `__ptrauth` fields and ObjC `__weak`; vptr signing is covered by the
    polymorphic rule (§3.4).

#### 3.8.2 Sanitizers

Crubit sees only the AST; sanitizer state is created at runtime by member
functions, so it cannot be verified — it has to be part of the contract that
annotation authors (and Crubit's own std table) uphold. The concrete hazards:

-   **ASan, in-object poisoning.** Container-overflow annotations
    (`__sanitizer_annotate_contiguous_container`, `ASAN_POISON_MEMORY_REGION`)
    poison bytes *inside the object* in some types: inline/SOO storage of
    small-buffer containers and hash tables, redzones around inline buffers
    (e.g. `absl::FixedArray`, `third_party/absl/container/fixed_array.h:530`). A
    `memcpy` out of a partially-poisoned object is itself an ASan report, and
    the destination's shadow is clean while the source stays poisoned, so later
    reuse of the source bytes trips false positives. Such a type is **not BR**
    under ASan even though its layout and move constructor say it is. Poisoning
    of *heap* buffers owned by the object (libc++ `std::vector`; libc++
    `std::string` only annotates long strings — `if (__is_long())`, [libc++
    `<string>`](https://github.com/llvm/llvm-project/blob/main/libcxx/include/string))
    is keyed by the buffer address and is fine.

    Rule: a type whose implementation poisons any byte of `*this` (not just
    owned heap memory) must not be annotated BR, and the std table must only
    contain types verified not to do so on libc++ *and* libstdc++. Because
    upstream libc++ has flipped short-string annotations on and off before, this
    is another reason `std::basic_string` stays in the never-set rather than
    being whitelisted per standard library.

-   **MSan.** Shadow is copied by the (intercepted) `memcpy`, so relocation is
    neutral. The general Crubit/MSan caveat (uninstrumented Rust writing C++
    memory leaves stale shadow) is not relocation-specific.

-   **TSan.** Shadow is per-address happens-before history; a relocated object
    has no history at the new address, which can only *miss* races, not
    fabricate them. Neutral.

-   **HWASan / MTE.** Tags live in the pointer and in the granule, not in the
    object; a type that stores a tagged pointer to its own storage is already
    self-referential and non-BR. Neutral otherwise.

-   **UBSan `-fsanitize=vptr`.** Only polymorphic types, already never-BR.

Practical consequence for the trusted parts of the design (std table, field
overrides): verification by test, not by inspection. §6.5 adds an asan/msan/tsan
matrix that Rust-moves every table entry (and the override test fixtures) and
then exercises them from C++.

## 4\. Standard library table

libc++/libstdc++ headers can't carry our annotations, and their internal layouts
differ, so `rs_bindings_from_cc` ships a small table keyed by the
fully-qualified primary template name, consulted *before* the structural rules:

| Template                                                                                        | Rule                                                                 |
| :---------------------------------------------------------------------------------------------- | :------------------------------------------------------------------- |
| `std::optional<T>`                                                                              | `IF_ELIGIBLE` ⇒ iff `T`                                              |
| `std::variant<Ts...>`                                                                           | iff all `Ts`                                                         |
| `std::expected<T,E>`                                                                            | iff `T` and `E`                                                      |
| `std::vector<T,A>`                                                                              | iff `A` (default allocator: yes; `T` irrelevant — heap)              |
| `std::unique_ptr<T,D>`                                                                          | iff `D`                                                              |
| `std::shared_ptr<T>`, `std::weak_ptr<T>`                                                        | yes                                                                  |
| `std::basic_string<...>`                                                                        | **never** (§3.4)                                                     |
| `std::function`, `std::move_only_function`, `std::any`                                          | no (inline small-buffer with self-pointer in at least one impl)      |
| `std::map/set/multimap/multiset/unordered_*`, `std::list`                                       | no (sentinel/end-node self-pointers)                                 |
| `std::pair`, `std::tuple`, `std::array`, `std::string_view`, `std::span`, `std::atomic<scalar>` | no entry needed — implicit rule (3.1) already gives the right answer |
| anything else in `std::`                                                                        | fall through to the implicit rule                                    |

Rules are expressed as "IF_ELIGIBLE restricted to these template args" so the
table and the annotation share one code path. `std::vector<bool>` is handled by
the generic `vector` rule. The table is the one place where Crubit *trusts*
without verifying; it is closed and owned by Crubit, every entry must hold on
libc++ and libstdc++ and under the sanitizers (§3.8.2), and each entry is backed
by a fixture in the §6.5 sanitizer matrix.

Abseil: `absl::Status`/`absl::Cord` are covered via §3.7. Other absl types can
spell `[[clang::annotate("crubit_bitwise_relocatable_if_eligible")]]` directly
(no Crubit dependency needed — `absl/log/log_rust_ffi.h` already does this
pattern), or be added to the table if we don't want annotations in absl.

Note: The std table ships in a follow-up CL after the predicate and annotations;
until then `std::optional<T>`/`std::vector<T>` members leave a type pinned
exactly as today.

## 5\. Worked examples

| C++                                                                                                                             | Today                  | Proposed                                                                 |
| :------------------------------------------------------------------------------------------------------------------------------ | :--------------------- | :----------------------------------------------------------------------- |
| `struct A { std::string s; };`                                                                                                  | pinned                 | pinned (`No`: member `s` is never-BR)                                    |
| `struct ABSL_ATTRIBUTE_TRIVIAL_ABI B { std::string s; };`                                                                       | **Unpin (bug)**        | pinned                                                                   |
| `struct CRUBIT_UNSAFE_BITWISE_RELOCATABLE C { std::string s; };`                                                                | n/a                    | **error**: "`s`: `std::basic_string` is never bitwise-relocatable"       |
| `struct D { std::optional<int> o; };`                                                                                           | Unpin                  | Unpin                                                                    |
| `struct E { std::optional<std::unique_ptr<int>> o; };`                                                                          | pinned                 | Unpin                                                                    |
| `struct F { std::optional<std::string> o; };`                                                                                   | pinned                 | pinned                                                                   |
| `struct G { std::vector<std::string> v; };`                                                                                     | pinned                 | Unpin                                                                    |
| `struct CRUBIT_UNSAFE_BITWISE_RELOCATABLE H : Base { virtual ~H(); };`                                                          | pinned                 | **error**: polymorphic                                                   |
| `class CRUBIT_UNSAFE_BITWISE_RELOCATABLE GURL { std::string spec_; ... };`                                                      | pinned                 | **error**: `spec_` is `std::basic_string`                                |
| `class CRUBIT_UNSAFE_BITWISE_RELOCATABLE GURL { CRUBIT_BITWISE_RELOCATABLE_STD_STRING_ONLY_ON_LIBCXX std::string spec_; ... };` | pinned                 | Unpin on libc++; **error** if the TU uses libstdc++                      |
| `struct I { I(I&&); ~I(); int x; };` (no annotation)                                                                            | pinned                 | pinned (user-provided special members)                                   |
| `struct CRUBIT_UNSAFE_BITWISE_RELOCATABLE I2 { I2(I2&&); ~I2(); int x; };`                                                      | pinned                 | Unpin                                                                    |
| `struct J { J(const J&) = default; J(J&&) = delete; ~J() = default; int x; };`                                                  | pinned                 | pinned (xvalue selects deleted move)                                     |
| `struct K { K(const K&) = delete; K(K&&) = delete; ~K(); int fd_; };`                                                           | pinned                 | pinned (no non-deleted copy/move; user-provided dtor)                    |
| `struct CRUBIT_UNSAFE_BITWISE_RELOCATABLE K2 { K2(const K2&) = delete; K2(K2&&) = delete; ~K2(); int fd_; };`                   | pinned, **no way out** | Unpin                                                                    |
| `struct CRUBIT_NEVER_BITWISE_RELOCATABLE L { int x; };`                                                                         | Unpin                  | pinned (never-set)                                                       |
| `struct ABSL_ATTRIBUTE_TRIVIAL_ABI CRUBIT_NEVER_BITWISE_RELOCATABLE L2 { int x; };`                                             | Unpin                  | pinned (never-set)                                                       |
| `struct CRUBIT_UNSAFE_BITWISE_RELOCATABLE CRUBIT_NEVER_BITWISE_RELOCATABLE L3 { int x; };`                                      | n/a                    | **error**: `CRUBIT_UNSAFE_BITWISE_RELOCATABLE` applied to never-set type |

## 6\. Implementation

1.  **Annotations**: add to `support/annotations.h` (+ `annotations.swig`,
    `wald/.../cpp.tm` per the LINT.ThenChange):
    `CRUBIT_UNSAFE_BITWISE_RELOCATABLE` → `crubit_unsafe_bitwise_relocatable`,
    `CRUBIT_UNSAFE_BITWISE_RELOCATABLE_IF_ELIGIBLE` →
    `crubit_unsafe_bitwise_relocatable_if_eligible`,
    `CRUBIT_UNSAFE_BITWISE_RELOCATABLE_FIELD` →
    `crubit_unsafe_bitwise_relocatable_field`,
    `CRUBIT_BITWISE_RELOCATABLE_STD_STRING_ONLY_ON_LIBCXX` →
    `crubit_bitwise_relocatable_std_string_only_on_libcxx` (generator checks
    that the field is a `std::basic_string` and that `_LIBCPP_VERSION` is
    defined in the TU's preprocessor state). Read with the existing
    `GetAnnotateAttrArgs` (`common/annotation_reader.h`) on `CXXRecordDecl` /
    `FieldDecl`.
2.  **Predicate**: new
    `rs_bindings_from_cc/importers/bitwise_relocatable.{h,cc}` with
    `RelocatabilityAnalysis` (memo map `const CXXRecordDecl* → Result`),
    replacing `IsRustMovable`. Mirror Clang's `IsDefaultMovable` /
    `LookupSpecialMemberFromXValue` for rule (3) (we have `ictx.sema_`); keep
    the `GetAllConstructors`/canonical-dtor care from the current code for
    modules. Std table lives here.
3.  **IR**: keep `Record.is_rust_movable` (`rs_bindings_from_cc/ir_schema.rs`);
    add `string not_rust_movable_reason` so generated bindings can carry a
    doc-comment explaining *why* a type is pinned (big UX win
    for -style
    confusion). `Error` results become hard import errors
    (`CRUBIT_MUST_BIND`-style).
4.  **cc_bindings_from_rs**: additionally emit
    `CRUBIT_UNSAFE_BITWISE_RELOCATABLE` (via raw `clang::annotate`) on generated
    C++ structs so Rust→C++→Rust round-trips don't depend on `trivial_abi` once
    Alternative B is ever taken. No behavior change otherwise.
5.  **Tests**: golden + `rs_bindings_from_cc/test/struct/unpin/`-style
    end-to-end tests for every row of §5, each on x86 Linux + arm64 via
    `multiplatform_rust_test`; negative tests asserting the hard errors,
    including the tagged-PFP hard failure (extend
    `rs_bindings_from_cc/test/pointer_field_protection/`, which already builds
    with the PFP crosstool feature on arm). **Sanitizer matrix (§3.8.2)**: one
    fixture per std-table entry and per override kind that constructs the object
    in C++, Rust-moves it (`mem::replace`, `Vec::push`, return by value), then
    mutates/reads/destroys it in C++; run under `--config=asan`,
    `--config=msan`, `--config=tsan` so in-object poisoning or shadow mismatches
    show up as test failures. 5a. **Mode checks in the generator**: fail fast on
    `LangOpts.PointerFieldProtectionTagged`; the `_STD_STRING_ONLY_ON_LIBCXX`
    override checks `_LIBCPP_VERSION`. Both are cheap preprocessor/LangOpts
    lookups in `ImportContext` setup.
6.  **Docs**: rewrite `docs/cpp/classes_and_structs.md#rust_movable`,
    `docs/design/unpin.md` (which currently defines Rust-movable *as* trivial
    for calls),
    and
    (which currently *recommends* adding `ABSL_ATTRIBUTE_TRIVIAL_ABI` —
    b/461469788 asks to retire that guidance; it becomes "use
    `CRUBIT_UNSAFE_BITWISE_RELOCATABLE`; if the type already has `trivial_abi`,
    see §3.7"), plus the `.agents/skills/using_crubit` guidance; add
    `RELNOTES=1`. The `CRUBIT_UNSAFE_BITWISE_RELOCATABLE` header comment carries
    the author checklist for what Crubit cannot verify: (a) move ctor + dtor are
    equivalent to `memcpy` + forget (no self-pointers, no registration of `this`
    in side tables, no address-keyed caches); (b) nothing poisons bytes of
    `*this` under ASan; (c) no address-dependent pointer encoding (`__ptrauth`,
    tagged PFP); (d) holds on every standard library the type is built against
    (or use `_STD_STRING_ONLY_ON_LIBCXX`).
7.  **Rollout**: this changes generated code → Mondo (`rust_api_from_cpp` and
    `cpp_api_from_rust`). Expected breakages are exactly the unsound cases
    (types that were `Unpin` only because of `trivial_abi` + non-BR member); fix
    forward with the owners (add the annotation to the member type, or the field
    override with a SAFETY comment), not via a denylist. If the count is large,
    gate step 2's *narrowing* half behind a feature flag.

## 7\. Follow-ups / alternatives considered

-   **C++ compile-time enforcement.** Attributes can't `static_assert` on the
    class's own members. Options: a ClangTidy check that re-implements §3 (so
    non-Crubit builds catch false claims), or a
    `CRUBIT_ASSERT_BITWISE_RELOCATABLE(T)` macro using
    `__builtin_is_cpp_trivially_relocatable(T)` as a *partial* approximation.
    Not required for v1.
-   **A C++ trait (`crubit::is_bitwise_relocatable<T>`) evaluated via Sema
    instead of a hard-coded std table.** Nice because users could
    `static_assert` on it, but it's trust-only (no member verification),
    requires injecting a support header into every TU Crubit analyses, and makes
    the answer depend on which specializations are visible. Rejected for v1; the
    table can later be *generated from* such a trait if wanted.
-   **`CRUBIT_UNSAFE_BITWISE_RELOCATABLE_IF(expr)`** (annotate args are
    instantiated constant expressions; `GetExprAsBool` already exists). Only
    useful as an additional *narrowing* condition on top of `_IF_ELIGIBLE`;
    deferred until a concrete need appears.
-   **Reusing `__builtin_is_cpp_trivially_relocatable` wholesale.** It has the
    right member-recursion shape but (a) doesn't exclude polymorphic types on
    non-ptrauth targets, (b) has no opt-in (the keyword isn't implemented in our
    Clang), (c) is target-dependent via ptrauth. We mirror its structure
    instead.

## 8\. Decisions

1.  **`std::string` laundering** → allowed only via the explicitly named,
    generator-verified `CRUBIT_BITWISE_RELOCATABLE_STD_STRING_ONLY_ON_LIBCXX`
    (§3.6.1), which only applies to `std::basic_string` fields.
2.  **`trivial_abi`** → kept forever as `_IF_ELIGIBLE`; docs warn that it may
    not yield a Rust-movable type (§3.7).
3.  **Who sees the error for a wrong `CRUBIT_UNSAFE_BITWISE_RELOCATABLE`?** → A
    false claim is always a hard error, regardless of which target owns the
    annotated type. (Note: Only hard-erroring for the owning target was
    considered and can be adopted later if a widely included header bites.)
4.  **Naming** → All four annotations carry the `CRUBIT_UNSAFE_` prefix and use
    the term `BITWISE_RELOCATABLE`.
5.  **Tagged PFP** → `rs_bindings_from_cc` hard-fails when
    `LangOpts.PointerFieldProtectionTagged` is set.
6.  **No feature gate** → the new annotations ship enabled. Using an annotation
    is itself the opt-in (no existing code carries them), and the narrowing
    parts (`trivial_abi` ⇒ `_IF_ELIGIBLE`, the never-set) are soundness fixes
    validated by Mondo rather than opt-in behavior.

### Open questions

1.  **Rule (3) and copy-only types**: a type with a user-provided copy ctor but
    defaulted move ctor and dtor is BR under the xvalue-selection rule even
    though its *copy* is non-trivial. That matches P2786 and is correct for Rust
    moves; confirm we're comfortable that `Clone` codegen (which calls the C++
    copy ctor) is unaffected.
