# Unstable Features

## Unstable features used by Crubit {#accepted}

For each feature we use, we document the following:

*   **Crubit feature:** Under which Crubit feature flags do we use this? If it
    is under `supported`, then the Rust feature breaking can break Crubit users
    in general. If it is under `experimental`, then it only breaks tests and
    close partner teams.
*   **Use case:** What do we use the feature for?
*   **Exit strategy:** What would happen if the feature went away?

### `custom_inner_attributes`

*   **Crubit feature:** `supported`
*   **Use case:** Used to suppress automatic formatting, to make our golden
    tests stable even in the face of e.g. non-idempotency bugs in `rustfmt`.
*   **Exit strategy:** Disable `rustfmt` on the golden tests some other way.

### `negative_impls`

*   **Crubit feature:** `supported`
*   **Use case:** Used to implement `ctor` / nontrivial intialization, so that
    we can dispatch on the existence of the `Unpin` trait (which is not possible
    with `PhantomPinned`). Also used so that we can pin/unpin a type without
    adding a field (which is not possible with `PhantomPinned`).
*   **Exit strategy:** For dispatch, we can define a new auto trait (also an
    unstable feature) or use specialization (also an unstable feature). For the
    fields, this is a compatibility break, and we'd need to add a PhantomData
    field to all C++ types to mark them as `!Send`, `!Unpin`, etc.

### `vec_into_raw_parts`

*   **Crubit feature:** `experimental`
*   **Use case:** Used for conversions of vectors of forward-declared objects,
    which is not yet released.
*   **Exit strategy:** We could delete this if we had to. Hopefully low-risk,
    people love raw parts.

### `extern_types`

*   **Crubit feature:** `experimental`
*   **Use case:** Used for forward declarations, which have an unknown size.
*   **Exit strategy:** Hard to avoid: without this, we get UB for forward
    declarations if you use references, due to provenance rules. Don't need the
    `forward_declare` crate except for migration from existing C++ code to Rust,
    where it relies on forward declarations for build performance or
    cycle-breaking reasons.

### `sized_hierarchy`

*   **Crubit feature:** `experimental`

*   **Use case:** Used for forward declarations, which have an unknown size.

*   **Exit strategy:** Same as `extern_types`.

### `arbitrary_self_types`

*   **Crubit feature:** `experimental`
*   **Use case:** We need this or an equivalent feature in order to make methods
    callable on rvalue references but not regular const references, so as to
    support C++ move semantics. Also likely need it for aliasing-safe
    references.
*   **Exit strategy:** Likely can't live without this feature. Lowish risk, in
    that Rust needs user-defined types to be usable as self types (e.g. it
    already has this for Rc), although the exact mechanism is as-yet undecided.

### `never_type`

*   **Crubit feature:** `experimental`
*   **Use case:** Used in a test to demonstrate we don't support it.
*   **Exit strategy:** Delete test.

### `c_variadic`

*   **Crubit feature:** `experimental`
*   **Use case:** Used in a test to demonstrate we don't support it.
*   **Exit strategy:** Delete test.

### `abi_vectorcall`

*   **Crubit feature:** `experimental`
*   **Use case:** Used in a test to test non-C calling conventions.
*   **Exit strategy:** Delete test at worst, replace with different non-C
    calling convention at best.

### `impl_trait_in_assoc_type`

*   **Crubit feature:** `experimental`
*   **Use case:** Used for returning `impl Ctor` for nontrivial construction.
*   **Exit strategy:** Replace with
    [RPITIT](https://github.com/rust-lang/rfcs/pull/3425), which is stable now.

### `allocator_api`

*   **Crubit feature:** unreleased
*   **Use case:** Used for an ABI-compatible implementation of `std::vector`
    that can reuse the C++ allocator.
*   **Exit strategy:** Do not release, or release but require that the Rust
    global allocator *is* the C++ allocator.

### `cfg_sanitize`

*   **Crubit feature:** unreleased
*   **Use case:** Used for an ABI-compatible implementation of `std::vector`
    that poisons memory in the same way as libc++.
*   **Exit strategy:** Do not release, or release but with degraded
    AddressSanitizer results that hide bugs.

### `cfg_accessible`, `stmt_expr_attributes`, `proc_macro_hygiene`

*   **Crubit feature:** N/A, used internally by Crubit
*   **Use case:** Crubit uses rustc as a library, which is subject to
    backwards-incompatible changes. `cfg_accessible` makes it much easier to
    write code which is compatible with two versions of rustc at the same time.
    `stmt_expr_attributes`, `proc_macro_hygiene` lets us handle
    backwards-incompatible changes affecting individual expressions.
*   **Exit strategy:** If `cfg_accessible` changes during the very same update
    we are trying to be compatible with, we can't use it in the first place. If
    it changes in a future version, then the update has already completed
    successfully, so we can delete the usage (keeping the `cfg_accessible` usage
    that is currently used, and deleting the other one). Same for the rest.

### `unsized_const_params`, `adt_const_params`

*   **Crubit feature:** `supported`
*   **Use case:** Better error messages when you get a type error on forward
    declarations. `Symbol<(C<'F'>, C<'o'>, C<'o'>)>` is so substantially worse
    than `Symbol<"Foo">` that Rust will even hide the type entirely, forcing you
    to read the name from a separate file (which may not be available to you if
    you were doing a remote build).
*   **Exit strategy:** The `Symbol<(...)>` approach works and is tested, and we
    can switch back with a single flag flip. The only impact is a regression on
    error messages.

### `super_let`

*   **Crubit feature:** `wrapper`
*   **Use case:** Less painfully awkward syntax for in-place initialization.
*   **Exit strategy:** Migrate all calls like `let x = emplace!(y)` to
    `emplace!{let x = y;}`.

Without this feature, Crubit's `ctor.rs` crate must use a syntax like
`emplace!{let x = ...}` to in-place initialize a pinned value using a C++
constructor. It cannot use the same trick that `pin!()` used to use, because it
needs to actually call a function to initialize the memory, and so lifetime
extension would not apply: the memory, being passed to a function, would have
its lifetime end at the end of the statement.

This doesn't work great with rustfmt, which gives up inside the macro call. It
is also confusing, and even *alarming*, and makes the `ctor.rs` crate look less
like a set of ecosystem and vocabulary types/traits, and more like a language
dialect.

`super_let` allows us to instead write `let x = emplace!(...)`, with the macro
expanding to a use of `super_let` to lifetime-extend the storage subexpression.

### `allow_internal_unstable`

*   **Crubit feature:** `wrapper`
*   **Use case:** Allow callers of the super_let version of `emplace` not to
    require the feature.
*   **Exit strategy:** Add `feature(super_let)` to all callers.

## Unstable features **not** used by Crubit {#rejected}

The following features are ones we'd hypothetically like to use, but do not.

### `try_trait_v2`

Crubit does aim to support the Abseil error type, `absl::Status`, which ideally
would be usable everywhere a `Result` is for ergonomics. For example, we'd
expect you to be able to write code like the following:

```rust
pub fn read() -> Status {
  foo()?;
}
```

Where `foo` returns a `Status`, or even an `io::Result`.

For this to work with `Status` being *exactly the same type* as it is in C++,
with the same layout, we need to use `try_trait_v2`.

Until it is stabilized, or the cost/benefit becomes worthwhile, we can work
around it by using `Result`, and converting when a `Result` is passed or
returned by value.

### `fn_traits`

Much like `try_traits_v2`, we'd like to support C++ "function objects" which
implement the call operator. In Rust, without `fn_traits`, the API is awkward:
given a `foo` of type `std::function`, `absl::AnyInvocable`, or
`absl::FunctionRef`, you cannot just write `foo()` to call it, because it cannot
implement the `Fn*` family of traits. This makes these types less convenient to
use than they are in C++, requiring something like `foo.as_closure()()` to
create a closure out of a `FunctionRef` or similar.

(Another use case we've examined in the past: in addition to implementing
function objects, `fn_traits` could also be used to implement overloading,
including overloaded function objects. For example, an overloaded function
could, instead, be a constant with many different implementations of `Fn`, for
different parameter types. Though, this ends up looking a bit odd:
`(mystruct.overloaded)()`)

### `min_specialization`

We have many use cases for `min_specialization`, including bindings for C++
templates that have explicit template specialization or partial template
specialization. However, they are not yet pressing, and specialization related
Rust features are especially risky.

### `inherent_associated_types`

Inherent associated types give a natural Rust spelling of the following C++ type
definition:

```c++
struct Foo {
    using MyAlias = int;
};
```

### `disjoint_associated_types`

[`disjoint_associated_types`](https://github.com/rust-lang/rfcs/pull/1672) was
closed, but not necessarily permanently. See also
https://github.com/rust-lang/rust/issues/20400.

Not every C++ type supports the Rust move operation. For more on this, see
[Classes and Structs](../cpp/classes_and_structs.md#rust_movable).

Crubit (in `experimental` mode) supports passing and returning these
non-Rust-movable C++ objects by value. But since they are not Rust-movable, they
cannot literally be returned in Rust by value: `pub fn foo() -> X` performs a
Rust move of its return type by definition.

Instead, these objects support lazy construction, in the same style as
[`moveit`](https://mcyoung.xyz/2021/04/26/move-ctors/). See
[`ctor.rs`](https://github.com/google/crubit/blob/main/support/ctor.rs). This
results in, for example, the following API differences:

`X` is rust-movable  | `X` is not rust-movable
-------------------- | ---------------------------------------
`pub fn foo() -> X`  | `pub fn foo() -> impl Ctor<Output=X>`
`impl Add<X> for &C` | `impl<T: Ctor<Output=X>> Add<T> for &C`

The problem comes in with operator overloading: the following is valid:

```rust
impl Add<X> for &C {...}
impl Add<Y> for &C {...}
```

But the equivalent using this lazy-construction trait is not:

```rust
impl<T: Ctor<Output=X>> Add<T> for &C {...}
impl<T: Ctor<Output=Y>> Add<T> for &C {...}
```

Rust doesn't know that these two are disjoint, meaning that we cannot use the
`Ctor` trait approach for traits.

An alternative fix would be language support for in-place pinned construction.
That would render the `Ctor` trait obsolete, and reduce Crubit's needs around
trait coherence (as well as `negative_impls`, above).

### `register_tool`

We previously used this attribute to allow Crubit to read annotations on
types/functions. However, the need for end-users to enable a feature gate and
re-declare the tool caused us to migrate away to using `#[doc]` attributes.

We use custom attributes so that Crubit can round-trip a type correctly, or to
implement automated bridging so that a C++ `Status` becomes a Rust `Result<(),
StatusError>`, or what have you.

